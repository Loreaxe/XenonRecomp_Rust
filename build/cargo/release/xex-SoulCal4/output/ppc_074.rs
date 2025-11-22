pub fn sub_825135D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825135D8 size=228
    let mut pc: u32 = 0x825135D8;
    'dispatch: loop {
        match pc {
            0x825135D8 => {
    //   block [0x825135D8..0x825136BC)
	// 825135D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825135DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825135E0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825135E4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 825135E8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825135EC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 825135F0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 825135F4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825135F8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825135FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82513600: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82513604: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82513608: 4200FFF0  bdnz 0x825135f8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825135F8; continue 'dispatch;
	}
	// 8251360C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513610: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82513614: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82513618: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 8251361C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82513620: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825136C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825136C0 size=16
    let mut pc: u32 = 0x825136C0;
    'dispatch: loop {
        match pc {
            0x825136C0 => {
    //   block [0x825136C0..0x825136D0)
	// 825136C0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825136C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825136C8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825136CC: 48012424  b 0x82525af0
	sub_82525AF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825136D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825136D0 size=76
    let mut pc: u32 = 0x825136D0;
    'dispatch: loop {
        match pc {
            0x825136D0 => {
    //   block [0x825136D0..0x8251371C)
	// 825136D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825136D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825136D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825136DC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825136E0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825136E4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825136E8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825136EC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 825136F0: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 825136F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825136F8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825136FC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82513700: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82513704: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82513708: 48012111  bl 0x82525818
	ctx.lr = 0x8251370C;
	sub_82525818(ctx, base);
	// 8251370C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513720 size=80
    let mut pc: u32 = 0x82513720;
    'dispatch: loop {
        match pc {
            0x82513720 => {
    //   block [0x82513720..0x82513770)
	// 82513720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251372C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513730: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82513734: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82513738: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251373C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513740: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513744: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82513748: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251374C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513750: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82513754: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82513758: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251375C: 48011B2D  bl 0x82525288
	ctx.lr = 0x82513760;
	sub_82525288(ctx, base);
	// 82513760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513770 size=176
    let mut pc: u32 = 0x82513770;
    'dispatch: loop {
        match pc {
            0x82513770 => {
    //   block [0x82513770..0x82513820)
	// 82513770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251377C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513780: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82513784: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513788: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8251378C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513790: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513794: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513798: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251379C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825137A0: 480119C1  bl 0x82525160
	ctx.lr = 0x825137A4;
	sub_82525160(ctx, base);
	// 825137A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825137A8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825137AC: 40980044  bge cr6, 0x825137f0
	if !ctx.cr[6].lt {
	pc = 0x825137F0; continue 'dispatch;
	}
	// 825137B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825137B4: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 825137B8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513820 size=240
    let mut pc: u32 = 0x82513820;
    'dispatch: loop {
        match pc {
            0x82513820 => {
    //   block [0x82513820..0x82513910)
	// 82513820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251382C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513830: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513834: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513838: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251383C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82513840: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82513844: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82513848: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8251384C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82513850: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82513854: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82513858: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8251385C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82513860: 4200FFF0  bdnz 0x82513850
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82513850; continue 'dispatch;
	}
	// 82513864: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513868: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 8251386C: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82513870: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82513874: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82513878: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513910 size=120
    let mut pc: u32 = 0x82513910;
    'dispatch: loop {
        match pc {
            0x82513910 => {
    //   block [0x82513910..0x82513988)
	// 82513910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251391C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513920: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82513924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513928: 396B4C9C  addi r11, r11, 0x4c9c
	ctx.r[11].s64 = ctx.r[11].s64 + 19612;
	// 8251392C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513930: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82513934: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82513938: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251393C: 409A002C  bne cr6, 0x82513968
	if !ctx.cr[6].eq {
	pc = 0x82513968; continue 'dispatch;
	}
	// 82513940: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513944: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82513948: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8251394C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82513950: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82513954: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82513958: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8251395C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82513960: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82513964: 4BF50755  bl 0x824640b8
	ctx.lr = 0x82513968;
	sub_824640B8(ctx, base);
	// 82513968: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251396C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82513970: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513988 size=60
    let mut pc: u32 = 0x82513988;
    'dispatch: loop {
        match pc {
            0x82513988 => {
    //   block [0x82513988..0x825139C4)
	// 82513988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251399C: 4800073D  bl 0x825140d8
	ctx.lr = 0x825139A0;
	sub_825140D8(ctx, base);
	// 825139A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825139A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825139A8: 396B4CD8  addi r11, r11, 0x4cd8
	ctx.r[11].s64 = ctx.r[11].s64 + 19672;
	// 825139AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825139B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825139B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825139B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825139BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825139C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825139C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825139C8 size=432
    let mut pc: u32 = 0x825139C8;
    'dispatch: loop {
        match pc {
            0x825139C8 => {
    //   block [0x825139C8..0x82513B78)
	// 825139C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825139CC: 480216DD  bl 0x825350a8
	ctx.lr = 0x825139D0;
	sub_82535080(ctx, base);
	// 825139D0: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825139D4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825139D8: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 825139DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825139E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825139E4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825139E8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825139EC: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825139F0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 825139F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825139F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825139FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82513A00: 40980020  bge cr6, 0x82513a20
	if !ctx.cr[6].lt {
	pc = 0x82513A20; continue 'dispatch;
	}
	// 82513A04: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513A08: 39294D10  addi r9, r9, 0x4d10
	ctx.r[9].s64 = ctx.r[9].s64 + 19728;
	// 82513A0C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82513A10: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82513A14: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82513A18: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82513A1C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82513A20: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82513A24: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82513A28: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82513A2C: 4809572D  bl 0x825a9158
	ctx.lr = 0x82513A30;
	sub_825A9158(ctx, base);
	// 82513A30: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513A34: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82513A38: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82513A3C: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82513A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513A44: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82513A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82513A4C: 4E800421  bctrl
	ctx.lr = 0x82513A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82513A50: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82513A54: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82513A58: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82513A5C: 395F0050  addi r10, r31, 0x50
	ctx.r[10].s64 = ctx.r[31].s64 + 80;
	// 82513A60: C1BF0004  lfs f13, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82513A64: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82513A68: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82513A6C: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82513A70: C009BFFC  lfs f0, -0x4004(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82513A74: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82513A78: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82513A7C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82513A80: 39299F50  addi r9, r9, -0x60b0
	ctx.r[9].s64 = ctx.r[9].s64 + -24752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513B78 size=176
    let mut pc: u32 = 0x82513B78;
    'dispatch: loop {
        match pc {
            0x82513B78 => {
    //   block [0x82513B78..0x82513C28)
	// 82513B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513B8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513B90: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82513B94: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513B98: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82513B9C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82513BA0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82513BA4: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82513BA8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82513BAC: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82513BB0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82513BB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513BB8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82513BBC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82513BC0: 40980028  bge cr6, 0x82513be8
	if !ctx.cr[6].lt {
	pc = 0x82513BE8; continue 'dispatch;
	}
	// 82513BC4: 4BF50475  bl 0x82464038
	ctx.lr = 0x82513BC8;
	sub_82464038(ctx, base);
	// 82513BC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513BCC: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82513BD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82513BD4: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82513BD8: 48000501  bl 0x825140d8
	ctx.lr = 0x82513BDC;
	sub_825140D8(ctx, base);
	// 82513BDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82513BE0: 396B4CD8  addi r11, r11, 0x4cd8
	ctx.r[11].s64 = ctx.r[11].s64 + 19672;
	// 82513BE4: 48000024  b 0x82513c08
	pc = 0x82513C08; continue 'dispatch;
	// 82513BE8: 4BF50451  bl 0x82464038
	ctx.lr = 0x82513BEC;
	sub_82464038(ctx, base);
	// 82513BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513BF0: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82513BF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82513BF8: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82513BFC: 480004DD  bl 0x825140d8
	ctx.lr = 0x82513C00;
	sub_825140D8(ctx, base);
	// 82513C00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82513C04: 396B4D1C  addi r11, r11, 0x4d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 19740;
	// 82513C08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513C10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513C28 size=256
    let mut pc: u32 = 0x82513C28;
    'dispatch: loop {
        match pc {
            0x82513C28 => {
    //   block [0x82513C28..0x82513D28)
	// 82513C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513C2C: 48021491  bl 0x825350bc
	ctx.lr = 0x82513C30;
	sub_82535080(ctx, base);
	// 82513C30: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513C34: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82513C38: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82513C3C: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82513C40: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82513C44: 39084500  addi r8, r8, 0x4500
	ctx.r[8].s64 = ctx.r[8].s64 + 17664;
	// 82513C48: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 82513C4C: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82513C50: 396B20F0  addi r11, r11, 0x20f0
	ctx.r[11].s64 = ctx.r[11].s64 + 8432;
	// 82513C54: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82513C58: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82513C5C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82513C60: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82513C64: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82513C68: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82513C6C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82513C70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82513C74: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82513C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513C7C: 9BA10060  stb r29, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u8 ) };
	// 82513C80: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82513C84: 4BFEC105  bl 0x824ffd88
	ctx.lr = 0x82513C88;
	sub_824FFD88(ctx, base);
	// 82513C88: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82513C8C: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82513C90: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82513C94: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82513C98: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82513C9C: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82513CA0: 39084138  addi r8, r8, 0x4138
	ctx.r[8].s64 = ctx.r[8].s64 + 16696;
	// 82513CA4: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82513CA8: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82513CAC: 396B39C8  addi r11, r11, 0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14792;
	// 82513CB0: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82513CB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82513CB8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82513CBC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82513CC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82513CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513CC8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82513CCC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82513CD0: 4BFEC0B9  bl 0x824ffd88
	ctx.lr = 0x82513CD4;
	sub_824FFD88(ctx, base);
	// 82513CD4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82513CD8: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82513CDC: 9BC100A0  stb r30, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u8 ) };
	// 82513CE0: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82513CE4: 9BA100A1  stb r29, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[29].u8 ) };
	// 82513CE8: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82513CEC: 39083B78  addi r8, r8, 0x3b78
	ctx.r[8].s64 = ctx.r[8].s64 + 15224;
	// 82513CF0: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82513CF4: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82513CF8: 396B39C8  addi r11, r11, 0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14792;
	// 82513CFC: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82513D00: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82513D04: 91010090  stw r8, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82513D08: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82513D0C: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82513D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513D14: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82513D18: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82513D1C: 4BFEC06D  bl 0x824ffd88
	ctx.lr = 0x82513D20;
	sub_824FFD88(ctx, base);
	// 82513D20: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82513D24: 480213E8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513D28 size=16
    let mut pc: u32 = 0x82513D28;
    'dispatch: loop {
        match pc {
            0x82513D28 => {
    //   block [0x82513D28..0x82513D38)
	// 82513D28: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513D2C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513D30: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513D34: 480004FC  b 0x82514230
	sub_82514230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513D38 size=76
    let mut pc: u32 = 0x82513D38;
    'dispatch: loop {
        match pc {
            0x82513D38 => {
    //   block [0x82513D38..0x82513D84)
	// 82513D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513D44: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513D48: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513D4C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513D50: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513D54: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82513D58: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82513D5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82513D60: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82513D64: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82513D68: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82513D6C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82513D70: 48002919  bl 0x82516688
	ctx.lr = 0x82513D74;
	sub_82516688(ctx, base);
	// 82513D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513D88 size=80
    let mut pc: u32 = 0x82513D88;
    'dispatch: loop {
        match pc {
            0x82513D88 => {
    //   block [0x82513D88..0x82513DD8)
	// 82513D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513D94: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513D98: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82513D9C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82513DA0: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82513DA4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513DA8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513DAC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82513DB0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82513DB4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513DB8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82513DBC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82513DC0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82513DC4: 480028AD  bl 0x82516670
	ctx.lr = 0x82513DC8;
	sub_82516670(ctx, base);
	// 82513DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513DD8 size=100
    let mut pc: u32 = 0x82513DD8;
    'dispatch: loop {
        match pc {
            0x82513DD8 => {
    //   block [0x82513DD8..0x82513E3C)
	// 82513DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513DF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513DF4: 4BFFFB1D  bl 0x82513910
	ctx.lr = 0x82513DF8;
	sub_82513910(ctx, base);
	// 82513DF8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82513DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82513E00: 419A0020  beq cr6, 0x82513e20
	if ctx.cr[6].eq {
	pc = 0x82513E20; continue 'dispatch;
	}
	// 82513E04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513E08: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82513E0C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82513E10: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513E14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513E18: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82513E1C: 4BF5029D  bl 0x824640b8
	ctx.lr = 0x82513E20;
	sub_824640B8(ctx, base);
	// 82513E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513E30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513E40 size=240
    let mut pc: u32 = 0x82513E40;
    'dispatch: loop {
        match pc {
            0x82513E40 => {
    //   block [0x82513E40..0x82513F30)
	// 82513E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513E50: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513E54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513E58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513E5C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82513E60: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82513E64: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82513E68: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82513E6C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82513E70: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82513E74: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82513E78: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82513E7C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82513E80: 4200FFF0  bdnz 0x82513e70
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82513E70; continue 'dispatch;
	}
	// 82513E84: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513E88: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82513E8C: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82513E90: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82513E94: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82513E98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513F30 size=16
    let mut pc: u32 = 0x82513F30;
    'dispatch: loop {
        match pc {
            0x82513F30 => {
    //   block [0x82513F30..0x82513F40)
	// 82513F30: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82513F34: 896BD790  lbz r11, -0x2870(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-10352 as u32) ) } as u64;
	// 82513F38: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82513F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513F40 size=12
    let mut pc: u32 = 0x82513F40;
    'dispatch: loop {
        match pc {
            0x82513F40 => {
    //   block [0x82513F40..0x82513F4C)
	// 82513F40: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82513F44: 986BD790  stb r3, -0x2870(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-10352 as u32), ctx.r[3].u8 ) };
	// 82513F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513F50 size=136
    let mut pc: u32 = 0x82513F50;
    'dispatch: loop {
        match pc {
            0x82513F50 => {
    //   block [0x82513F50..0x82513FD8)
	// 82513F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513F54: 48021165  bl 0x825350b8
	ctx.lr = 0x82513F58;
	sub_82535080(ctx, base);
	// 82513F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513F5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82513F60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82513F64: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82513F68: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82513F6C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82513F70: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82513F74: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82513F78: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82513F7C: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82513F80: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82513F84: 419A0034  beq cr6, 0x82513fb8
	if ctx.cr[6].eq {
	pc = 0x82513FB8; continue 'dispatch;
	}
	// 82513F88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82513F8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82513F90: 419A001C  beq cr6, 0x82513fac
	if ctx.cr[6].eq {
	pc = 0x82513FAC; continue 'dispatch;
	}
	// 82513F94: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82513F98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82513F9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513FA0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82513FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82513FA8: 4E800421  bctrl
	ctx.lr = 0x82513FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82513FAC: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82513FB0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82513FB4: 409AFFD4  bne cr6, 0x82513f88
	if !ctx.cr[6].eq {
	pc = 0x82513F88; continue 'dispatch;
	}
	// 82513FB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513FBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82513FC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82513FC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513FC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82513FCC: 4E800421  bctrl
	ctx.lr = 0x82513FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82513FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82513FD4: 48021134  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513FD8 size=108
    let mut pc: u32 = 0x82513FD8;
    'dispatch: loop {
        match pc {
            0x82513FD8 => {
    //   block [0x82513FD8..0x82514044)
	// 82513FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513FDC: 480210E1  bl 0x825350bc
	ctx.lr = 0x82513FE0;
	sub_82535080(ctx, base);
	// 82513FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513FE4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82513FE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82513FEC: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82513FF0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82513FF4: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82513FF8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82513FFC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82514000: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82514004: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82514008: 419A0034  beq cr6, 0x8251403c
	if ctx.cr[6].eq {
	pc = 0x8251403C; continue 'dispatch;
	}
	// 8251400C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514014: 419A001C  beq cr6, 0x82514030
	if ctx.cr[6].eq {
	pc = 0x82514030; continue 'dispatch;
	}
	// 82514018: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8251401C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82514020: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514024: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82514028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251402C: 4E800421  bctrl
	ctx.lr = 0x82514030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82514030: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82514034: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82514038: 409AFFD4  bne cr6, 0x8251400c
	if !ctx.cr[6].eq {
	pc = 0x8251400C; continue 'dispatch;
	}
	// 8251403C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82514040: 480210CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514048 size=140
    let mut pc: u32 = 0x82514048;
    'dispatch: loop {
        match pc {
            0x82514048 => {
    //   block [0x82514048..0x825140D4)
	// 82514048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251404C: 48021071  bl 0x825350bc
	ctx.lr = 0x82514050;
	sub_82535080(ctx, base);
	// 82514050: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82514054: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82514058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251405C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82514060: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82514064: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514068: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8251406C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82514070: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82514074: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82514078: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251407C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82514080: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82514084: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82514088: 419A003C  beq cr6, 0x825140c4
	if ctx.cr[6].eq {
	pc = 0x825140C4; continue 'dispatch;
	}
	// 8251408C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514094: 419A0024  beq cr6, 0x825140b8
	if ctx.cr[6].eq {
	pc = 0x825140B8; continue 'dispatch;
	}
	// 82514098: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8251409C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825140A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825140A4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825140A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825140AC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825140B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825140B4: 4E800421  bctrl
	ctx.lr = 0x825140B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825140B8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 825140BC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825140C0: 409AFFCC  bne cr6, 0x8251408c
	if !ctx.cr[6].eq {
	pc = 0x8251408C; continue 'dispatch;
	}
	// 825140C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825140C8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825140CC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825140D0: 4802103C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825140D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825140D8 size=96
    let mut pc: u32 = 0x825140D8;
    'dispatch: loop {
        match pc {
            0x825140D8 => {
    //   block [0x825140D8..0x82514138)
	// 825140D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825140DC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 825140E0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825140E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825140E8: 39294C9C  addi r9, r9, 0x4c9c
	ctx.r[9].s64 = ctx.r[9].s64 + 19612;
	// 825140EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825140F0: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825140F4: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 825140F8: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 825140FC: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82514100: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82514104: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82514108: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8251410C: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82514110: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82514114: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82514138 size=164
    let mut pc: u32 = 0x82514138;
    'dispatch: loop {
        match pc {
            0x82514138 => {
    //   block [0x82514138..0x825141DC)
	// 82514138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514148: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251414C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82514150: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82514154: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82514158: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251415C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82514160: 4BF4FED9  bl 0x82464038
	ctx.lr = 0x82514164;
	sub_82464038(ctx, base);
	// 82514164: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82514168: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8251416C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82514170: 392B4C9C  addi r9, r11, 0x4c9c
	ctx.r[9].s64 = ctx.r[11].s64 + 19612;
	// 82514174: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82514178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8251417C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82514180: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82514184: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82514188: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251418C: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82514190: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82514194: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82514198: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 8251419C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 825141A0: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 825141A4: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 825141A8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825141E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825141E0 size=80
    let mut pc: u32 = 0x825141E0;
    'dispatch: loop {
        match pc {
            0x825141E0 => {
    //   block [0x825141E0..0x82514230)
	// 825141E0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825141E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825141E8: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 825141EC: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825141F0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825141F4: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825141F8: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 825141FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514200: 409A0008  bne cr6, 0x82514208
	if !ctx.cr[6].eq {
	pc = 0x82514208; continue 'dispatch;
	}
	// 82514204: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 82514208: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251420C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82514210: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82514214: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82514218: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251421C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82514220: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82514224: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82514228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251422C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514230 size=472
    let mut pc: u32 = 0x82514230;
    'dispatch: loop {
        match pc {
            0x82514230 => {
    //   block [0x82514230..0x82514408)
	// 82514230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514234: 48020E71  bl 0x825350a4
	ctx.lr = 0x82514238;
	sub_82535080(ctx, base);
	// 82514238: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251423C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82514240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514244: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82514248: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251424C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82514250: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514258: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251425C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82514260: 4E800421  bctrl
	ctx.lr = 0x82514264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82514264: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82514268: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8251426C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82514270: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514274: 4099018C  ble cr6, 0x82514400
	if !ctx.cr[6].gt {
	pc = 0x82514400; continue 'dispatch;
	}
	// 82514278: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251427C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514280: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82514284: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514288: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8251428C: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82514290: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82514294: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82514298: 4E800421  bctrl
	ctx.lr = 0x8251429C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251429C: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 825142A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825142A4: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 825142A8: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 825142AC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825142B0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825142B4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825142B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825142BC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825142C0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825142C4: 7D3E582E  lwzx r9, r30, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825142C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825142CC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825142D0: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 825142D4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825142D8: 7D3E582E  lwzx r9, r30, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825142DC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825142E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825142E4: 4E800421  bctrl
	ctx.lr = 0x825142E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825142E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825142EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825142F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825142F4: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825142F8: 419A00B4  beq cr6, 0x825143ac
	if ctx.cr[6].eq {
	pc = 0x825143AC; continue 'dispatch;
	}
	// 825142FC: 4BFEC84D  bl 0x82500b48
	ctx.lr = 0x82514300;
	sub_82500B48(ctx, base);
	// 82514300: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514304: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82514308: 409A0074  bne cr6, 0x8251437c
	if !ctx.cr[6].eq {
	pc = 0x8251437C; continue 'dispatch;
	}
	// 8251430C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82514310: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514314: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82514318: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8251431C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514320: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514324: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514328: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251432C: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82514330: 409A0008  bne cr6, 0x82514338
	if !ctx.cr[6].eq {
	pc = 0x82514338; continue 'dispatch;
	}
	// 82514334: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	// 82514338: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251433C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82514340: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82514344: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82514348: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251434C: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82514350: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82514354: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82514358: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251435C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82514360: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82514364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82514368: 4E800421  bctrl
	ctx.lr = 0x8251436C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251436C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514370: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82514374: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82514378: 48000074  b 0x825143ec
	pc = 0x825143EC; continue 'dispatch;
	// 8251437C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514380: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82514384: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82514388: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8251438C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82514390: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82514394: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251439C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 825143A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825143A4: 4E800421  bctrl
	ctx.lr = 0x825143A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825143A8: 48000044  b 0x825143ec
	pc = 0x825143EC; continue 'dispatch;
	// 825143AC: 4BFEC79D  bl 0x82500b48
	ctx.lr = 0x825143B0;
	sub_82500B48(ctx, base);
	// 825143B0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825143B4: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 825143B8: 419A0034  beq cr6, 0x825143ec
	if ctx.cr[6].eq {
	pc = 0x825143EC; continue 'dispatch;
	}
	// 825143BC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825143C0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825143C4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825143C8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825143CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825143D0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825143D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825143D8: 4E800421  bctrl
	ctx.lr = 0x825143DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825143DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825143E0: 7F7E5A14  add r27, r30, r11
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825143E4: 4BFEC765  bl 0x82500b48
	ctx.lr = 0x825143E8;
	sub_82500B48(ctx, base);
	// 825143E8: 907B0008  stw r3, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825143EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825143F0: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 825143F4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 825143F8: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825143FC: 4198FE80  blt cr6, 0x8251427c
	if ctx.cr[6].lt {
	pc = 0x8251427C; continue 'dispatch;
	}
	// 82514400: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82514404: 48020CF0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514408 size=244
    let mut pc: u32 = 0x82514408;
    'dispatch: loop {
        match pc {
            0x82514408 => {
    //   block [0x82514408..0x825144FC)
	// 82514408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251440C: 48020CAD  bl 0x825350b8
	ctx.lr = 0x82514410;
	sub_82535080(ctx, base);
	// 82514410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82514418: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251441C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514420: 388B4D60  addi r4, r11, 0x4d60
	ctx.r[4].s64 = ctx.r[11].s64 + 19808;
	// 82514424: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82514428: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251442C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82514430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514434: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251443C: 4E800421  bctrl
	ctx.lr = 0x82514440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82514440: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82514444: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82514448: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251444C: 409A0048  bne cr6, 0x82514494
	if !ctx.cr[6].eq {
	pc = 0x82514494; continue 'dispatch;
	}
	// 82514450: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514454: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82514458: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251445C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82514460: 38894D54  addi r4, r9, 0x4d54
	ctx.r[4].s64 = ctx.r[9].s64 + 19796;
	// 82514464: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82514468: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8251446C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82514470: 83A80008  lwz r29, 8(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514474: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82514478: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8251447C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82514480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514484: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82514488: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8251448C: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82514490: 4E800421  bctrl
	ctx.lr = 0x82514494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82514494: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82514498: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251449C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 825144A0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 825144A4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825144A8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825144AC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825144B0: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825144B4: 419A0040  beq cr6, 0x825144f4
	if ctx.cr[6].eq {
	pc = 0x825144F4; continue 'dispatch;
	}
	// 825144B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825144BC: 3B8B08B0  addi r28, r11, 0x8b0
	ctx.r[28].s64 = ctx.r[11].s64 + 2224;
	// 825144C0: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825144C4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 825144C8: 419A0020  beq cr6, 0x825144e8
	if ctx.cr[6].eq {
	pc = 0x825144E8; continue 'dispatch;
	}
	// 825144CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825144D0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825144D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825144D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825144DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825144E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825144E4: 4E800421  bctrl
	ctx.lr = 0x825144E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825144E8: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 825144EC: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825144F0: 409AFFD0  bne cr6, 0x825144c0
	if !ctx.cr[6].eq {
	pc = 0x825144C0; continue 'dispatch;
	}
	// 825144F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825144F8: 48020C10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82514500 size=176
    let mut pc: u32 = 0x82514500;
    'dispatch: loop {
        match pc {
            0x82514500 => {
    //   block [0x82514500..0x825145B0)
	// 82514500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251450C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514510: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514514: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82514518: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251451C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82514520: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82514524: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82514528: 4BF4FB11  bl 0x82464038
	ctx.lr = 0x8251452C;
	sub_82464038(ctx, base);
	// 8251452C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82514530: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82514534: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82514538: 396B4C9C  addi r11, r11, 0x4c9c
	ctx.r[11].s64 = ctx.r[11].s64 + 19612;
	// 8251453C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82514544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82514548: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 8251454C: B0E30004  sth r7, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82514550: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82514554: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82514558: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 8251455C: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82514560: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82514564: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82514568: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8251456C: 392A4D70  addi r9, r10, 0x4d70
	ctx.r[9].s64 = ctx.r[10].s64 + 19824;
	// 82514570: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82514574: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825145B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825145B0 size=324
    let mut pc: u32 = 0x825145B0;
    'dispatch: loop {
        match pc {
            0x825145B0 => {
    //   block [0x825145B0..0x825146F4)
	// 825145B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825145B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825145B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825145BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825145C0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825145C4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 825145C8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825145CC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825145D0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825145D4: C00B00A0  lfs f0, 0xa0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825145D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825145DC: C1AA00A0  lfs f13, 0xa0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825145E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825145E4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825145E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825145EC: 40980080  bge cr6, 0x8251466c
	if !ctx.cr[6].lt {
	pc = 0x8251466C; continue 'dispatch;
	}
	// 825145F0: 4BF4FA49  bl 0x82464038
	ctx.lr = 0x825145F4;
	sub_82464038(ctx, base);
	// 825145F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825145F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825145FC: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82514600: 390B4C9C  addi r8, r11, 0x4c9c
	ctx.r[8].s64 = ctx.r[11].s64 + 19612;
	// 82514604: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82514608: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251460C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82514610: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82514614: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82514618: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251461C: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82514620: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82514624: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82514628: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 8251462C: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82514630: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82514634: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82514638: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825146F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825146F8 size=2472
    let mut pc: u32 = 0x825146F8;
    'dispatch: loop {
        match pc {
            0x825146F8 => {
    //   block [0x825146F8..0x825150A0)
	// 825146F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825146FC: 48020991  bl 0x8253508c
	ctx.lr = 0x82514700;
	sub_82535080(ctx, base);
	// 82514700: 9421FA80  stwu r1, -0x580(r1)
	ea = ctx.r[1].u32.wrapping_add(-1408 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514704: 7C932378  mr r19, r4
	ctx.r[19].u64 = ctx.r[4].u64;
	// 82514708: 7CB12B78  mr r17, r5
	ctx.r[17].u64 = ctx.r[5].u64;
	// 8251470C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82514710: 386104C0  addi r3, r1, 0x4c0
	ctx.r[3].s64 = ctx.r[1].s64 + 1216;
	// 82514714: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82514718: 80B30008  lwz r5, 8(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251471C: 7CF23B78  mr r18, r7
	ctx.r[18].u64 = ctx.r[7].u64;
	// 82514720: 80910008  lwz r4, 8(r17)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82514724: 48094A35  bl 0x825a9158
	ctx.lr = 0x82514728;
	sub_825A9158(ctx, base);
	// 82514728: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8251472C: 80730000  lwz r3, 0(r19)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514730: C0170004  lfs f0, 4(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82514734: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82514738: 388104C0  addi r4, r1, 0x4c0
	ctx.r[4].s64 = ctx.r[1].s64 + 1216;
	// 8251473C: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82514740: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514744: EC20037A  fmadds f1, f0, f13, f0
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82514748: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8251474C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82514750: 4E800421  bctrl
	ctx.lr = 0x82514754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82514754: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82514758: 896BD790  lbz r11, -0x2870(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-10352 as u32) ) } as u64;
	// 8251475C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514760: 419A0064  beq cr6, 0x825147c4
	if ctx.cr[6].eq {
	pc = 0x825147C4; continue 'dispatch;
	}
	// 82514764: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82514768: 113F038C  vspltisw v9, -1
	for i in 0..4 {
		ctx.v[9].u32[i] = 4294967295;
	}
	// 8251476C: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 82514770: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82514774: 390000E0  li r8, 0xe0
	ctx.r[8].s64 = 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825150A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825150A0 size=3516
    let mut pc: u32 = 0x825150A0;
    'dispatch: loop {
        match pc {
            0x825150A0 => {
    //   block [0x825150A0..0x82515E5C)
	// 825150A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825150A4: 4801FFDD  bl 0x82535080
	ctx.lr = 0x825150A8;
	sub_82535080(ctx, base);
	// 825150A8: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 825150AC: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 825150B0: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515E60 size=576
    let mut pc: u32 = 0x82515E60;
    'dispatch: loop {
        match pc {
            0x82515E60 => {
    //   block [0x82515E60..0x825160A0)
	// 82515E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515E64: 4801F239  bl 0x8253509c
	ctx.lr = 0x82515E68;
	sub_82535080(ctx, base);
	// 82515E68: 9421FB00  stwu r1, -0x500(r1)
	ea = ctx.r[1].u32.wrapping_add(-1280 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515E6C: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515E70: 3AC00014  li r22, 0x14
	ctx.r[22].s64 = 20;
	// 82515E74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82515E78: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82515E7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82515E80: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82515E84: 7D55B02E  lwzx r10, r21, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82515E88: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82515E8C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515E90: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82515E94: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82515E98: 4098002C  bge cr6, 0x82515ec4
	if !ctx.cr[6].lt {
	pc = 0x82515EC4; continue 'dispatch;
	}
	// 82515E9C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82515EA0: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82515EA4: 39294DB8  addi r9, r9, 0x4db8
	ctx.r[9].s64 = ctx.r[9].s64 + 19896;
	// 82515EA8: 39084C08  addi r8, r8, 0x4c08
	ctx.r[8].s64 = ctx.r[8].s64 + 19464;
	// 82515EAC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82515EB0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82515EB4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82515EB8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82515EBC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82515EC0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82515EC4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82515EC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82515ECC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82515ED0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82515ED4: 480007CD  bl 0x825166a0
	ctx.lr = 0x82515ED8;
	sub_825166A0(ctx, base);
	// 82515ED8: 3961009C  addi r11, r1, 0x9c
	ctx.r[11].s64 = ctx.r[1].s64 + 156;
	// 82515EDC: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515EE0: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82515EE4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82515EE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82515EEC: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82515EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82515EF4: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82515EF8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82515EFC: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82515F00: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82515F04: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515F08: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82515F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82515F10: 4E800421  bctrl
	ctx.lr = 0x82515F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82515F14: 7D75B02E  lwzx r11, r21, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82515F18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515F1C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82515F20: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82515F24: 40980020  bge cr6, 0x82515f44
	if !ctx.cr[6].lt {
	pc = 0x82515F44; continue 'dispatch;
	}
	// 82515F28: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82515F2C: 39294DA8  addi r9, r9, 0x4da8
	ctx.r[9].s64 = ctx.r[9].s64 + 19880;
	// 82515F30: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82515F34: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82515F38: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82515F3C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82515F40: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82515F44: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82515F48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82515F4C: 83E10090  lwz r31, 0x90(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82515F50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82515F54: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515F58: 7EEBFA14  add r23, r11, r31
	ctx.r[23].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82515F5C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82515F60: 836A000C  lwz r27, 0xc(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82515F64: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82515F68: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82515F6C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515F70: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82515F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82515F78: 4E800421  bctrl
	ctx.lr = 0x82515F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82515F7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82515F80: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82515F84: 419A00BC  beq cr6, 0x82516040
	if ctx.cr[6].eq {
	pc = 0x82516040; continue 'dispatch;
	}
	// 82515F88: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82515F8C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82515F90: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515F94: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82515F98: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82515F9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82515FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82515FA4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82515FB0: 4E800421  bctrl
	ctx.lr = 0x82515FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82515FB4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82515FBC: 419A0078  beq cr6, 0x82516034
	if ctx.cr[6].eq {
	pc = 0x82516034; continue 'dispatch;
	}
	// 82515FC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FC4: 38A102A0  addi r5, r1, 0x2a0
	ctx.r[5].s64 = ctx.r[1].s64 + 672;
	// 82515FC8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82515FD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82515FD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82515FD8: 4E800421  bctrl
	ctx.lr = 0x82515FDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82515FDC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82515FE4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515FE8: 391B000D  addi r8, r27, 0xd
	ctx.r[8].s64 = ctx.r[27].s64 + 13;
	// 82515FEC: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82515FF0: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82515FF4: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82515FF8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82515FFC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82516000: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82516004: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516008: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251600C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82516010: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82516014: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516018: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 8251601C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82516020: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516024: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82516028: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 8251602C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516030: 4E800421  bctrl
	ctx.lr = 0x82516034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516034: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82516038: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 8251603C: 409AFF4C  bne cr6, 0x82515f88
	if !ctx.cr[6].eq {
	pc = 0x82515F88; continue 'dispatch;
	}
	// 82516040: 7D55B02E  lwzx r10, r21, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82516044: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516048: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251604C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516050: 40980020  bge cr6, 0x82516070
	if !ctx.cr[6].lt {
	pc = 0x82516070; continue 'dispatch;
	}
	// 82516054: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516058: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 8251605C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516060: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516064: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82516068: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251606C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516070: 81610098  lwz r11, 0x98(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516074: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82516078: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251607C: 409A001C  bne cr6, 0x82516098
	if !ctx.cr[6].eq {
	pc = 0x82516098; continue 'dispatch;
	}
	// 82516080: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82516084: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82516088: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8251608C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82516090: 7C75502E  lwzx r3, r21, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516094: 4BF4E025  bl 0x824640b8
	ctx.lr = 0x82516098;
	sub_824640B8(ctx, base);
	// 82516098: 38210500  addi r1, r1, 0x500
	ctx.r[1].s64 = ctx.r[1].s64 + 1280;
	// 8251609C: 4801F050  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825160A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825160A0 size=596
    let mut pc: u32 = 0x825160A0;
    'dispatch: loop {
        match pc {
            0x825160A0 => {
    //   block [0x825160A0..0x825162F4)
	// 825160A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825160A4: 4801EFFD  bl 0x825350a0
	ctx.lr = 0x825160A8;
	sub_82535080(ctx, base);
	// 825160A8: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825160AC: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825160B0: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 825160B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825160B8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825160BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825160C0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 825160C4: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825160C8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825160CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825160D0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825160D4: 4098002C  bge cr6, 0x82516100
	if !ctx.cr[6].lt {
	pc = 0x82516100; continue 'dispatch;
	}
	// 825160D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825160DC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 825160E0: 39294DB8  addi r9, r9, 0x4db8
	ctx.r[9].s64 = ctx.r[9].s64 + 19896;
	// 825160E4: 39084C08  addi r8, r8, 0x4c08
	ctx.r[8].s64 = ctx.r[8].s64 + 19464;
	// 825160E8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825160EC: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 825160F0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825160F4: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 825160F8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825160FC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516100: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82516104: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516108: 809A0008  lwz r4, 8(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251610C: 4809304D  bl 0x825a9158
	ctx.lr = 0x82516110;
	sub_825A9158(ctx, base);
	// 82516110: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516114: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82516118: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8251611C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82516120: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516124: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82516128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251612C: 4E800421  bctrl
	ctx.lr = 0x82516130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516130: 396100DC  addi r11, r1, 0xdc
	ctx.r[11].s64 = ctx.r[1].s64 + 220;
	// 82516134: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516138: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 8251613C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82516140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82516144: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82516148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251614C: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 82516150: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82516154: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82516158: 916100D8  stw r11, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 8251615C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516160: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82516164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516168: 4E800421  bctrl
	ctx.lr = 0x8251616C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251616C: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82516170: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516174: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516178: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251617C: 40980020  bge cr6, 0x8251619c
	if !ctx.cr[6].lt {
	pc = 0x8251619C; continue 'dispatch;
	}
	// 82516180: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516184: 39294DA8  addi r9, r9, 0x4da8
	ctx.r[9].s64 = ctx.r[9].s64 + 19880;
	// 82516188: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251618C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516190: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82516194: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516198: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251619C: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 825161A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825161A4: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 825161A8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825161AC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825161B0: 7F0BFA14  add r24, r11, r31
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825161B4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 825161B8: 836A000C  lwz r27, 0xc(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825161BC: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 825161C0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825161C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825161C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825161CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825161D0: 4E800421  bctrl
	ctx.lr = 0x825161D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825161D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825161D8: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 825161DC: 419A00B8  beq cr6, 0x82516294
	if ctx.cr[6].eq {
	pc = 0x82516294; continue 'dispatch;
	}
	// 825161E0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825161E4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 825161E8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825161EC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825161F0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825161F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825161F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825161FC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516200: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516208: 4E800421  bctrl
	ctx.lr = 0x8251620C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251620C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82516214: 419A0074  beq cr6, 0x82516288
	if ctx.cr[6].eq {
	pc = 0x82516288; continue 'dispatch;
	}
	// 82516218: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251621C: 38A102E0  addi r5, r1, 0x2e0
	ctx.r[5].s64 = ctx.r[1].s64 + 736;
	// 82516220: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82516228: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251622C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516230: 4E800421  bctrl
	ctx.lr = 0x82516234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516234: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516238: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251623C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516240: 391B000D  addi r8, r27, 0xd
	ctx.r[8].s64 = ctx.r[27].s64 + 13;
	// 82516244: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82516248: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8251624C: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82516250: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82516254: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82516258: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8251625C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516260: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82516264: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82516268: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251626C: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82516270: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82516274: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516278: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251627C: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82516280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516284: 4E800421  bctrl
	ctx.lr = 0x82516288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516288: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8251628C: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82516290: 409AFF50  bne cr6, 0x825161e0
	if !ctx.cr[6].eq {
	pc = 0x825161E0; continue 'dispatch;
	}
	// 82516294: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82516298: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251629C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825162A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825162A4: 40980020  bge cr6, 0x825162c4
	if !ctx.cr[6].lt {
	pc = 0x825162C4; continue 'dispatch;
	}
	// 825162A8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825162AC: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 825162B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825162B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825162B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825162BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825162C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825162C4: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 825162C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825162CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825162D0: 409A001C  bne cr6, 0x825162ec
	if !ctx.cr[6].eq {
	pc = 0x825162EC; continue 'dispatch;
	}
	// 825162D4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825162D8: 808100D0  lwz r4, 0xd0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 825162DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825162E0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825162E4: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825162E8: 4BF4DDD1  bl 0x824640b8
	ctx.lr = 0x825162EC;
	sub_824640B8(ctx, base);
	// 825162EC: 38210540  addi r1, r1, 0x540
	ctx.r[1].s64 = ctx.r[1].s64 + 1344;
	// 825162F0: 4801EE00  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825162F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825162F8 size=608
    let mut pc: u32 = 0x825162F8;
    'dispatch: loop {
        match pc {
            0x825162F8 => {
    //   block [0x825162F8..0x82516558)
	// 825162F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825162FC: 4801EDA5  bl 0x825350a0
	ctx.lr = 0x82516300;
	sub_82535080(ctx, base);
	// 82516300: 9421FAC0  stwu r1, -0x540(r1)
	ea = ctx.r[1].u32.wrapping_add(-1344 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516304: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516308: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 8251630C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82516310: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82516314: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82516318: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8251631C: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82516320: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516324: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516328: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251632C: 4098002C  bge cr6, 0x82516358
	if !ctx.cr[6].lt {
	pc = 0x82516358; continue 'dispatch;
	}
	// 82516330: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516334: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82516338: 39294DB8  addi r9, r9, 0x4db8
	ctx.r[9].s64 = ctx.r[9].s64 + 19896;
	// 8251633C: 39084C08  addi r8, r8, 0x4c08
	ctx.r[8].s64 = ctx.r[8].s64 + 19464;
	// 82516340: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516344: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82516348: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251634C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82516350: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516354: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516358: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251635C: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516360: 80990008  lwz r4, 8(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516364: 48092DF5  bl 0x825a9158
	ctx.lr = 0x82516368;
	sub_825A9158(ctx, base);
	// 82516368: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251636C: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82516370: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82516374: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82516378: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251637C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82516380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516384: 4E800421  bctrl
	ctx.lr = 0x82516388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516388: 396100DC  addi r11, r1, 0xdc
	ctx.r[11].s64 = ctx.r[1].s64 + 220;
	// 8251638C: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516390: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82516394: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82516398: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251639C: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 825163A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825163A4: 916100D4  stw r11, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 825163A8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 825163AC: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 825163B0: 916100D8  stw r11, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 825163B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825163B8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825163BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825163C0: 4E800421  bctrl
	ctx.lr = 0x825163C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825163C4: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825163C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825163CC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825163D0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825163D4: 40980020  bge cr6, 0x825163f4
	if !ctx.cr[6].lt {
	pc = 0x825163F4; continue 'dispatch;
	}
	// 825163D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825163DC: 39294DA8  addi r9, r9, 0x4da8
	ctx.r[9].s64 = ctx.r[9].s64 + 19880;
	// 825163E0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825163E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825163E8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825163EC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825163F0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825163F4: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 825163F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825163FC: 83E100D0  lwz r31, 0xd0(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82516400: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516404: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516408: 7F0BFA14  add r24, r11, r31
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8251640C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516410: 834A000C  lwz r26, 0xc(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516414: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82516418: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8251641C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516420: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82516424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516428: 4E800421  bctrl
	ctx.lr = 0x8251642C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251642C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82516430: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82516434: 419A00C4  beq cr6, 0x825164f8
	if ctx.cr[6].eq {
	pc = 0x825164F8; continue 'dispatch;
	}
	// 82516438: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251643C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82516440: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516444: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82516448: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8251644C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82516450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82516454: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516458: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251645C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516460: 4E800421  bctrl
	ctx.lr = 0x82516464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516464: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251646C: 419A0080  beq cr6, 0x825164ec
	if ctx.cr[6].eq {
	pc = 0x825164EC; continue 'dispatch;
	}
	// 82516470: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516474: 38A102E0  addi r5, r1, 0x2e0
	ctx.r[5].s64 = ctx.r[1].s64 + 736;
	// 82516478: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251647C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82516480: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82516484: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516488: 4E800421  bctrl
	ctx.lr = 0x8251648C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251648C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516490: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82516494: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516498: 391A000D  addi r8, r26, 0xd
	ctx.r[8].s64 = ctx.r[26].s64 + 13;
	// 8251649C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825164A0: 55082834  slwi r8, r8, 5
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825164A4: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 825164A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825164AC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825164B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825164B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825164B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825164BC: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 825164C0: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825164C4: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 825164C8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825164CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825164D0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825164D4: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 825164D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825164DC: 4E800421  bctrl
	ctx.lr = 0x825164E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825164E0: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825164E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825164E8: 409A0010  bne cr6, 0x825164f8
	if !ctx.cr[6].eq {
	pc = 0x825164F8; continue 'dispatch;
	}
	// 825164EC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825164F0: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 825164F4: 409AFF44  bne cr6, 0x82516438
	if !ctx.cr[6].eq {
	pc = 0x82516438; continue 'dispatch;
	}
	// 825164F8: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825164FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516500: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516504: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516508: 40980020  bge cr6, 0x82516528
	if !ctx.cr[6].lt {
	pc = 0x82516528; continue 'dispatch;
	}
	// 8251650C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516510: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82516514: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516518: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251651C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82516520: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516524: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516528: 816100D8  lwz r11, 0xd8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 8251652C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82516530: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82516534: 409A001C  bne cr6, 0x82516550
	if !ctx.cr[6].eq {
	pc = 0x82516550; continue 'dispatch;
	}
	// 82516538: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251653C: 808100D0  lwz r4, 0xd0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82516540: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82516544: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82516548: 7C76502E  lwzx r3, r22, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251654C: 4BF4DB6D  bl 0x824640b8
	ctx.lr = 0x82516550;
	sub_824640B8(ctx, base);
	// 82516550: 38210540  addi r1, r1, 0x540
	ctx.r[1].s64 = ctx.r[1].s64 + 1344;
	// 82516554: 4801EB9C  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516558 size=256
    let mut pc: u32 = 0x82516558;
    'dispatch: loop {
        match pc {
            0x82516558 => {
    //   block [0x82516558..0x82516658)
	// 82516558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251655C: 4801EB61  bl 0x825350bc
	ctx.lr = 0x82516560;
	sub_82535080(ctx, base);
	// 82516560: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516564: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82516568: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 8251656C: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82516570: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82516574: 39084500  addi r8, r8, 0x4500
	ctx.r[8].s64 = ctx.r[8].s64 + 17664;
	// 82516578: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 8251657C: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82516580: 396B32B0  addi r11, r11, 0x32b0
	ctx.r[11].s64 = ctx.r[11].s64 + 12976;
	// 82516584: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82516588: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8251658C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82516590: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82516594: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82516598: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251659C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825165A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825165A4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825165A8: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 825165AC: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825165B0: 4BFE97D9  bl 0x824ffd88
	ctx.lr = 0x825165B4;
	sub_824FFD88(ctx, base);
	// 825165B4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 825165B8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825165BC: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 825165C0: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 825165C4: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825165C8: 39084138  addi r8, r8, 0x4138
	ctx.r[8].s64 = ctx.r[8].s64 + 16696;
	// 825165CC: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 825165D0: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 825165D4: 396B5E60  addi r11, r11, 0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + 24160;
	// 825165D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825165DC: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 825165E0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825165E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825165E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825165EC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825165F0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 825165F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825165F8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825165FC: 9BA10080  stb r29, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u8 ) };
	// 82516600: 4BFE9789  bl 0x824ffd88
	ctx.lr = 0x82516604;
	sub_824FFD88(ctx, base);
	// 82516604: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82516608: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 8251660C: 9BA100A0  stb r29, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[29].u8 ) };
	// 82516610: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82516614: 9BC100A1  stb r30, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[30].u8 ) };
	// 82516618: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 8251661C: 390845B0  addi r8, r8, 0x45b0
	ctx.r[8].s64 = ctx.r[8].s64 + 17840;
	// 82516620: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82516624: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82516628: 396B5E60  addi r11, r11, 0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + 24160;
	// 8251662C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82516630: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82516634: 91010090  stw r8, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82516638: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8251663C: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82516640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516644: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82516648: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8251664C: 4BFE973D  bl 0x824ffd88
	ctx.lr = 0x82516650;
	sub_824FFD88(ctx, base);
	// 82516650: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82516654: 4801EAB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516658 size=24
    let mut pc: u32 = 0x82516658;
    'dispatch: loop {
        match pc {
            0x82516658 => {
    //   block [0x82516658..0x82516670)
	// 82516658: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251665C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82516660: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82516664: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82516668: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8251666C: 4BFFF7F4  b 0x82515e60
	sub_82515E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516670 size=20
    let mut pc: u32 = 0x82516670;
    'dispatch: loop {
        match pc {
            0x82516670 => {
    //   block [0x82516670..0x82516684)
	// 82516670: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82516674: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82516678: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251667C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82516680: 4BFFFA20  b 0x825160a0
	sub_825160A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516688 size=20
    let mut pc: u32 = 0x82516688;
    'dispatch: loop {
        match pc {
            0x82516688 => {
    //   block [0x82516688..0x8251669C)
	// 82516688: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251668C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82516690: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82516694: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82516698: 4BFFFC60  b 0x825162f8
	sub_825162F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825166A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825166A0 size=200
    let mut pc: u32 = 0x825166A0;
    'dispatch: loop {
        match pc {
            0x825166A0 => {
    //   block [0x825166A0..0x82516768)
	// 825166A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825166A4: 4801EA15  bl 0x825350b8
	ctx.lr = 0x825166A8;
	sub_82535080(ctx, base);
	// 825166A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825166AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825166B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825166B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825166B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825166BC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 825166C0: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825166C4: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825166C8: 48092A91  bl 0x825a9158
	ctx.lr = 0x825166CC;
	sub_825A9158(ctx, base);
	// 825166CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825166D0: C03C0004  lfs f1, 4(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825166D4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 825166D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825166DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825166E0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825166E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825166E8: 4E800421  bctrl
	ctx.lr = 0x825166EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825166EC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 825166F0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825166F4: 393C0050  addi r9, r28, 0x50
	ctx.r[9].s64 = ctx.r[28].s64 + 80;
	// 825166F8: 394A9F50  addi r10, r10, -0x60b0
	ctx.r[10].s64 = ctx.r[10].s64 + -24752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82516768 size=176
    let mut pc: u32 = 0x82516768;
    'dispatch: loop {
        match pc {
            0x82516768 => {
    //   block [0x82516768..0x82516818)
	// 82516768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251676C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82516770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82516774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82516778: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251677C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516780: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82516784: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82516788: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251678C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82516790: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516794: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82516798: 4BFFE909  bl 0x825150a0
	ctx.lr = 0x8251679C;
	sub_825150A0(ctx, base);
	// 8251679C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825167A0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825167A4: 40980044  bge cr6, 0x825167e8
	if !ctx.cr[6].lt {
	pc = 0x825167E8; continue 'dispatch;
	}
	// 825167A8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825167AC: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 825167B0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516818 size=68
    let mut pc: u32 = 0x82516818;
    'dispatch: loop {
        match pc {
            0x82516818 => {
    //   block [0x82516818..0x8251685C)
	// 82516818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251681C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82516820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516824: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82516828: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251682C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82516830: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 82516834: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82516838: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251683C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82516840: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82516844: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82516848: 4BFFFAB1  bl 0x825162f8
	ctx.lr = 0x8251684C;
	sub_825162F8(ctx, base);
	// 8251684C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82516850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82516854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82516858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516860 size=144
    let mut pc: u32 = 0x82516860;
    'dispatch: loop {
        match pc {
            0x82516860 => {
    //   block [0x82516860..0x825168F0)
	// 82516860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82516864: 4801E855  bl 0x825350b8
	ctx.lr = 0x82516868;
	sub_82535080(ctx, base);
	// 82516868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251686C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82516870: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82516874: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251687C: 419A0054  beq cr6, 0x825168d0
	if ctx.cr[6].eq {
	pc = 0x825168D0; continue 'dispatch;
	}
	// 82516880: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82516884: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82516888: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251688C: 40990044  ble cr6, 0x825168d0
	if !ctx.cr[6].gt {
	pc = 0x825168D0; continue 'dispatch;
	}
	// 82516890: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82516894: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516898: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8251689C: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 825168A0: 419A001C  beq cr6, 0x825168bc
	if ctx.cr[6].eq {
	pc = 0x825168BC; continue 'dispatch;
	}
	// 825168A4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825168A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825168AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825168B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825168B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825168B8: 4E800421  bctrl
	ctx.lr = 0x825168BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825168BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825168C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825168C4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 825168C8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825168CC: 4198FFC8  blt cr6, 0x82516894
	if ctx.cr[6].lt {
	pc = 0x82516894; continue 'dispatch;
	}
	// 825168D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825168D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825168D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825168DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825168E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825168E4: 4E800421  bctrl
	ctx.lr = 0x825168E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825168E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825168EC: 4801E81C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825168F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825168F0 size=220
    let mut pc: u32 = 0x825168F0;
    'dispatch: loop {
        match pc {
            0x825168F0 => {
    //   block [0x825168F0..0x825169CC)
	// 825168F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825168F4: 4801E7C5  bl 0x825350b8
	ctx.lr = 0x825168F8;
	sub_82535080(ctx, base);
	// 825168F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825168FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82516900: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82516904: 396B4DF0  addi r11, r11, 0x4df0
	ctx.r[11].s64 = ctx.r[11].s64 + 19952;
	// 82516908: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8251690C: 3BFC000C  addi r31, r28, 0xc
	ctx.r[31].s64 = ctx.r[28].s64 + 12;
	// 82516910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82516914: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82516918: 90FC0008  stw r7, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8251691C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82516920: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82516924: B13C0006  sth r9, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82516928: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251692C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82516930: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82516934: 419A008C  beq cr6, 0x825169c0
	if ctx.cr[6].eq {
	pc = 0x825169C0; continue 'dispatch;
	}
	// 82516938: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251693C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516940: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82516944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516948: 4E800421  bctrl
	ctx.lr = 0x8251694C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251694C: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516950: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82516954: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82516958: 40990064  ble cr6, 0x825169bc
	if !ctx.cr[6].gt {
	pc = 0x825169BC; continue 'dispatch;
	}
	// 8251695C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516960: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82516964: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82516968: 40980024  bge cr6, 0x8251698c
	if !ctx.cr[6].lt {
	pc = 0x8251698C; continue 'dispatch;
	}
	// 8251696C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516970: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82516974: 41980008  blt cr6, 0x8251697c
	if ctx.cr[6].lt {
	pc = 0x8251697C; continue 'dispatch;
	}
	// 82516978: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8251697C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82516980: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82516984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516988: 4BF57941  bl 0x8246e2c8
	ctx.lr = 0x8251698C;
	sub_8246E2C8(ctx, base);
	// 8251698C: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82516990: 4098002C  bge cr6, 0x825169bc
	if !ctx.cr[6].lt {
	pc = 0x825169BC; continue 'dispatch;
	}
	// 82516994: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82516998: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8251699C: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 825169A0: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 825169A4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825169A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825169AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825169B0: 7D2A432E  sthx r9, r10, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u16) };
	// 825169B4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 825169B8: 409AFFEC  bne cr6, 0x825169a4
	if !ctx.cr[6].eq {
	pc = 0x825169A4; continue 'dispatch;
	}
	// 825169BC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825169C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825169C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825169C8: 4801E740  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825169D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825169D0 size=88
    let mut pc: u32 = 0x825169D0;
    'dispatch: loop {
        match pc {
            0x825169D0 => {
    //   block [0x825169D0..0x82516A28)
	// 825169D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825169D4: 4801E6E5  bl 0x825350b8
	ctx.lr = 0x825169D8;
	sub_82535080(ctx, base);
	// 825169D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825169DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825169E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825169E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825169E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825169EC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825169F0: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 825169F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825169F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825169FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82516A00: 4BF4D639  bl 0x82464038
	ctx.lr = 0x82516A04;
	sub_82464038(ctx, base);
	// 82516A04: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82516A08: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82516A0C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82516A10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82516A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82516A18: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82516A1C: 4BFFFED5  bl 0x825168f0
	ctx.lr = 0x82516A20;
	sub_825168F0(ctx, base);
	// 82516A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82516A24: 4801E6E4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516A28 size=768
    let mut pc: u32 = 0x82516A28;
    'dispatch: loop {
        match pc {
            0x82516A28 => {
    //   block [0x82516A28..0x82516D28)
	// 82516A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82516A2C: 4801E669  bl 0x82535094
	ctx.lr = 0x82516A30;
	sub_82535080(ctx, base);
	// 82516A30: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516A34: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516A38: 3A800014  li r20, 0x14
	ctx.r[20].s64 = 20;
	// 82516A3C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82516A40: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82516A44: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82516A48: 7CD33378  mr r19, r6
	ctx.r[19].u64 = ctx.r[6].u64;
	// 82516A4C: 7D5CA02E  lwzx r10, r28, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82516A50: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516A54: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516A58: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516A5C: 4098002C  bge cr6, 0x82516a88
	if !ctx.cr[6].lt {
	pc = 0x82516A88; continue 'dispatch;
	}
	// 82516A60: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516A64: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82516A68: 39294E54  addi r9, r9, 0x4e54
	ctx.r[9].s64 = ctx.r[9].s64 + 20052;
	// 82516A6C: 39084E44  addi r8, r8, 0x4e44
	ctx.r[8].s64 = ctx.r[8].s64 + 20036;
	// 82516A70: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516A74: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82516A78: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516A7C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82516A80: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516A84: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516A88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82516A8C: 83F70000  lwz r31, 0(r23)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516A90: 83560000  lwz r26, 0(r22)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516A94: 80B70008  lwz r5, 8(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516A98: 80960008  lwz r4, 8(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516A9C: 480926BD  bl 0x825a9158
	ctx.lr = 0x82516AA0;
	sub_825A9158(ctx, base);
	// 82516AA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516AA8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82516AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516AB0: 4E800421  bctrl
	ctx.lr = 0x82516AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516AB4: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82516AB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82516ABC: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82516AC0: 7C7CC02E  lwzx r3, r28, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82516AC4: 557E2036  slwi r30, r11, 4
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82516AC8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516ACC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82516AD0: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82516AD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516AD8: 41990010  bgt cr6, 0x82516ae8
	if ctx.cr[6].gt {
	pc = 0x82516AE8; continue 'dispatch;
	}
	// 82516ADC: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82516AE0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82516AE4: 4800001C  b 0x82516b00
	pc = 0x82516B00; continue 'dispatch;
	// 82516AE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516AEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82516AF0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82516AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516AF8: 4E800421  bctrl
	ctx.lr = 0x82516AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516AFC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82516B00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516B04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82516B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516B0C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82516B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516B14: 4E800421  bctrl
	ctx.lr = 0x82516B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516B18: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82516B1C: 3B3DFFFF  addi r25, r29, -1
	ctx.r[25].s64 = ctx.r[29].s64 + -1;
	// 82516B20: 7D23D850  subf r9, r3, r27
	ctx.r[9].s64 = ctx.r[27].s64 - ctx.r[3].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516D28 size=1040
    let mut pc: u32 = 0x82516D28;
    'dispatch: loop {
        match pc {
            0x82516D28 => {
    //   block [0x82516D28..0x82517138)
	// 82516D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82516D2C: 4801E365  bl 0x82535090
	ctx.lr = 0x82516D30;
	sub_82535080(ctx, base);
	// 82516D30: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82516D34: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516D38: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516D3C: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 82516D40: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82516D44: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82516D48: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82516D4C: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82516D50: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82516D54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516D58: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516D5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516D60: 4098002C  bge cr6, 0x82516d8c
	if !ctx.cr[6].lt {
	pc = 0x82516D8C; continue 'dispatch;
	}
	// 82516D64: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516D68: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82516D6C: 39294E54  addi r9, r9, 0x4e54
	ctx.r[9].s64 = ctx.r[9].s64 + 20052;
	// 82516D70: 39084E98  addi r8, r8, 0x4e98
	ctx.r[8].s64 = ctx.r[8].s64 + 20120;
	// 82516D74: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516D78: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82516D7C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516D80: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82516D84: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516D88: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516D8C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82516D90: 83F50000  lwz r31, 0(r21)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516D94: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516D98: 80B50008  lwz r5, 8(r21)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516D9C: 80970008  lwz r4, 8(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82516DA0: 480923B9  bl 0x825a9158
	ctx.lr = 0x82516DA4;
	sub_825A9158(ctx, base);
	// 82516DA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516DAC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82516DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516DB4: 4E800421  bctrl
	ctx.lr = 0x82516DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516DB8: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 82516DBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82516DC0: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82516DC4: 7C78982E  lwzx r3, r24, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82516DC8: 557E2036  slwi r30, r11, 4
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82516DCC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516DD0: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82516DD4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82516DD8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516DDC: 41990010  bgt cr6, 0x82516dec
	if ctx.cr[6].gt {
	pc = 0x82516DEC; continue 'dispatch;
	}
	// 82516DE0: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 82516DE4: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82516DE8: 4800001C  b 0x82516e04
	pc = 0x82516E04; continue 'dispatch;
	// 82516DEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516DF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82516DF4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82516DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516DFC: 4E800421  bctrl
	ctx.lr = 0x82516E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516E00: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82516E04: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82516E08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516E0C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516E10: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516E14: 40980020  bge cr6, 0x82516e34
	if !ctx.cr[6].lt {
	pc = 0x82516E34; continue 'dispatch;
	}
	// 82516E18: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516E1C: 39294E88  addi r9, r9, 0x4e88
	ctx.r[9].s64 = ctx.r[9].s64 + 20104;
	// 82516E20: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516E24: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516E28: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82516E2C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516E30: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516E34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516E38: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82516E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516E40: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82516E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516E48: 4E800421  bctrl
	ctx.lr = 0x82516E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516E4C: 7D58902E  lwzx r10, r24, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82516E50: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516E54: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82516E58: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82516E5C: 40980020  bge cr6, 0x82516e7c
	if !ctx.cr[6].lt {
	pc = 0x82516E7C; continue 'dispatch;
	}
	// 82516E60: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82516E64: 39294E7C  addi r9, r9, 0x4e7c
	ctx.r[9].s64 = ctx.r[9].s64 + 20092;
	// 82516E68: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82516E6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82516E70: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82516E74: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516E78: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82516E7C: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82516E80: 3B3DFFFF  addi r25, r29, -1
	ctx.r[25].s64 = ctx.r[29].s64 + -1;
	// 82516E84: 7D23B050  subf r9, r3, r22
	ctx.r[9].s64 = ctx.r[22].s64 - ctx.r[3].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517138 size=848
    let mut pc: u32 = 0x82517138;
    'dispatch: loop {
        match pc {
            0x82517138 => {
    //   block [0x82517138..0x82517488)
	// 82517138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251713C: 4801DF55  bl 0x82535090
	ctx.lr = 0x82517140;
	sub_82535080(ctx, base);
	// 82517140: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517488 size=24
    let mut pc: u32 = 0x82517488;
    'dispatch: loop {
        match pc {
            0x82517488 => {
    //   block [0x82517488..0x825174A0)
	// 82517488: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251748C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82517490: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82517494: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82517498: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8251749C: 4BFFFC9C  b 0x82517138
	sub_82517138(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825174A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825174A0 size=20
    let mut pc: u32 = 0x825174A0;
    'dispatch: loop {
        match pc {
            0x825174A0 => {
    //   block [0x825174A0..0x825174B4)
	// 825174A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825174A4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825174A8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825174AC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825174B0: 4BFFF578  b 0x82516a28
	sub_82516A28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825174B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825174B8 size=20
    let mut pc: u32 = 0x825174B8;
    'dispatch: loop {
        match pc {
            0x825174B8 => {
    //   block [0x825174B8..0x825174CC)
	// 825174B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825174BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825174C0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825174C4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825174C8: 4BFFF860  b 0x82516d28
	sub_82516D28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825174D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825174D0 size=108
    let mut pc: u32 = 0x825174D0;
    'dispatch: loop {
        match pc {
            0x825174D0 => {
    //   block [0x825174D0..0x8251753C)
	// 825174D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825174D4: 4801DBE1  bl 0x825350b4
	ctx.lr = 0x825174D8;
	sub_82535080(ctx, base);
	// 825174D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825174DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825174E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825174E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825174E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825174EC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825174F0: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 825174F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825174F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825174FC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82517500: 4BF4CB39  bl 0x82464038
	ctx.lr = 0x82517504;
	sub_82464038(ctx, base);
	// 82517504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82517508: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 8251750C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82517510: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82517514: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82517518: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251751C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82517520: 4BFFF3D1  bl 0x825168f0
	ctx.lr = 0x82517524;
	sub_825168F0(ctx, base);
	// 82517524: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82517528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251752C: 396B4EC4  addi r11, r11, 0x4ec4
	ctx.r[11].s64 = ctx.r[11].s64 + 20164;
	// 82517530: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82517534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82517538: 4801DBCC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517540 size=204
    let mut pc: u32 = 0x82517540;
    'dispatch: loop {
        match pc {
            0x82517540 => {
    //   block [0x82517540..0x8251760C)
	// 82517540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251754C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517550: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517554: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82517558: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 8251755C: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82517560: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82517564: 390874D0  addi r8, r8, 0x74d0
	ctx.r[8].s64 = ctx.r[8].s64 + 29904;
	// 82517568: 39297B58  addi r9, r9, 0x7b58
	ctx.r[9].s64 = ctx.r[9].s64 + 31576;
	// 8251756C: 394A7BA8  addi r10, r10, 0x7ba8
	ctx.r[10].s64 = ctx.r[10].s64 + 31656;
	// 82517570: 396B7BF8  addi r11, r11, 0x7bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 31736;
	// 82517574: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82517578: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 8251757C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82517580: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82517584: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82517588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251758C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82517590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82517594: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82517598: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 8251759C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825175A0: 4BFE87E9  bl 0x824ffd88
	ctx.lr = 0x825175A4;
	sub_824FFD88(ctx, base);
	// 825175A4: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825175A8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825175AC: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 825175B0: 396B7138  addi r11, r11, 0x7138
	ctx.r[11].s64 = ctx.r[11].s64 + 28984;
	// 825175B4: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 825175B8: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 825175BC: 390869D0  addi r8, r8, 0x69d0
	ctx.r[8].s64 = ctx.r[8].s64 + 27088;
	// 825175C0: 39296A28  addi r9, r9, 0x6a28
	ctx.r[9].s64 = ctx.r[9].s64 + 27176;
	// 825175C4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825175C8: 394A6D28  addi r10, r10, 0x6d28
	ctx.r[10].s64 = ctx.r[10].s64 + 27944;
	// 825175CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825175D0: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825175D4: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 825175D8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825175DC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825175E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825175E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825175E8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 825175EC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 825175F0: 4BFE8799  bl 0x824ffd88
	ctx.lr = 0x825175F4;
	sub_824FFD88(ctx, base);
	// 825175F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825175F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825175FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517600: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82517604: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82517608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517610 size=240
    let mut pc: u32 = 0x82517610;
    'dispatch: loop {
        match pc {
            0x82517610 => {
    //   block [0x82517610..0x82517700)
	// 82517610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517618: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251761C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517620: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517700 size=164
    let mut pc: u32 = 0x82517700;
    'dispatch: loop {
        match pc {
            0x82517700 => {
    //   block [0x82517700..0x825177A4)
	// 82517700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251770C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517714: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82517718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251771C: 396B4DF0  addi r11, r11, 0x4df0
	ctx.r[11].s64 = ctx.r[11].s64 + 19952;
	// 82517720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82517724: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82517728: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251772C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82517730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82517734: 409A0020  bne cr6, 0x82517754
	if !ctx.cr[6].eq {
	pc = 0x82517754; continue 'dispatch;
	}
	// 82517738: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251773C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82517740: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82517744: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82517748: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 8251774C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82517750: 4BF4C969  bl 0x824640b8
	ctx.lr = 0x82517754;
	sub_824640B8(ctx, base);
	// 82517754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82517758: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8251775C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82517760: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82517764: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82517768: 419A0020  beq cr6, 0x82517788
	if ctx.cr[6].eq {
	pc = 0x82517788; continue 'dispatch;
	}
	// 8251776C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517770: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82517774: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82517778: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251777C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82517780: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82517784: 4BF4C935  bl 0x824640b8
	ctx.lr = 0x82517788;
	sub_824640B8(ctx, base);
	// 82517788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251778C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517798: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251779C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825177A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825177A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825177A8 size=940
    let mut pc: u32 = 0x825177A8;
    'dispatch: loop {
        match pc {
            0x825177A8 => {
    //   block [0x825177A8..0x82517B54)
	// 825177A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825177AC: 4801D8D5  bl 0x82535080
	ctx.lr = 0x825177B0;
	sub_82535080(ctx, base);
	// 825177B0: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 825177B4: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825177B8: 81ED0000  lwz r15, 0(r13)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825177BC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 825177C0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 825177C4: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 825177C8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825177CC: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 825177D0: 7D6F582E  lwzx r11, r15, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825177D4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825177D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825177DC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825177E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825177E4: 40980020  bge cr6, 0x82517804
	if !ctx.cr[6].lt {
	pc = 0x82517804; continue 'dispatch;
	}
	// 825177E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825177EC: 39294E28  addi r9, r9, 0x4e28
	ctx.r[9].s64 = ctx.r[9].s64 + 20008;
	// 825177F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825177F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825177F8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825177FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82517800: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82517804: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82517808: 83F10000  lwz r31, 0(r17)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251780C: 833B0000  lwz r25, 0(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517810: 80B10008  lwz r5, 8(r17)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) } as u64;
	// 82517814: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82517818: 48091941  bl 0x825a9158
	ctx.lr = 0x8251781C;
	sub_825A9158(ctx, base);
	// 8251781C: 39C00010  li r14, 0x10
	ctx.r[14].s64 = 16;
	// 82517820: 83D60010  lwz r30, 0x10(r22)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16 as u32) ) } as u64;
	// 82517824: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82517828: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8251782C: 7C6F702E  lwzx r3, r15, r14
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[15].u32.wrapping_add(ctx.r[14].u32)) } as u64;
	// 82517830: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82517834: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82517838: 7D4BE214  add r10, r11, r28
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8251783C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82517840: 41990010  bgt cr6, 0x82517850
	if ctx.cr[6].gt {
	pc = 0x82517850; continue 'dispatch;
	}
	// 82517844: 7D725B78  mr r18, r11
	ctx.r[18].u64 = ctx.r[11].u64;
	// 82517848: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8251784C: 4800001C  b 0x82517868
	pc = 0x82517868; continue 'dispatch;
	// 82517850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517854: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82517858: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251785C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82517860: 4E800421  bctrl
	ctx.lr = 0x82517864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82517864: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82517868: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251786C: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82517870: 83B6000C  lwz r29, 0xc(r22)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12 as u32) ) } as u64;
	// 82517874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82517878: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8251787C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82517880: 4E800421  bctrl
	ctx.lr = 0x82517884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82517884: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82517888: 3BFEFFFF  addi r31, r30, -1
	ctx.r[31].s64 = ctx.r[30].s64 + -1;
	// 8251788C: 7D239050  subf r9, r3, r18
	ctx.r[9].s64 = ctx.r[18].s64 - ctx.r[3].s64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517B58 size=76
    let mut pc: u32 = 0x82517B58;
    'dispatch: loop {
        match pc {
            0x82517B58 => {
    //   block [0x82517B58..0x82517BA4)
	// 82517B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517B64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82517B68: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82517B6C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82517B70: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82517B74: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82517B78: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82517B7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82517B80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82517B84: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82517B88: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82517B8C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82517B90: 4BFFEE99  bl 0x82516a28
	ctx.lr = 0x82517B94;
	sub_82516A28(ctx, base);
	// 82517B94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517BA8 size=80
    let mut pc: u32 = 0x82517BA8;
    'dispatch: loop {
        match pc {
            0x82517BA8 => {
    //   block [0x82517BA8..0x82517BF8)
	// 82517BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517BB4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82517BB8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82517BBC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82517BC0: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82517BC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82517BC8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82517BCC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82517BD0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82517BD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82517BD8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82517BDC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82517BE0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82517BE4: 4BFFF145  bl 0x82516d28
	ctx.lr = 0x82517BE8;
	sub_82516D28(ctx, base);
	// 82517BE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517BF8 size=232
    let mut pc: u32 = 0x82517BF8;
    'dispatch: loop {
        match pc {
            0x82517BF8 => {
    //   block [0x82517BF8..0x82517CE0)
	// 82517BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517C04: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82517C0C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82517C10: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82517C14: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82517C18: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82517C1C: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82517C20: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82517C24: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82517C28: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82517C2C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82517C30: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82517C34: 4200FFF0  bdnz 0x82517c24
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82517C24; continue 'dispatch;
	}
	// 82517C38: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82517C3C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82517C40: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82517C44: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82517C48: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82517C4C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517CE0 size=176
    let mut pc: u32 = 0x82517CE0;
    'dispatch: loop {
        match pc {
            0x82517CE0 => {
    //   block [0x82517CE0..0x82517D90)
	// 82517CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82517CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517CF0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82517CF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517CF8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82517CFC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82517D00: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82517D04: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82517D08: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517D0C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82517D10: 4BFFFA99  bl 0x825177a8
	ctx.lr = 0x82517D14;
	sub_825177A8(ctx, base);
	// 82517D14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82517D18: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82517D1C: 40980044  bge cr6, 0x82517d60
	if !ctx.cr[6].lt {
	pc = 0x82517D60; continue 'dispatch;
	}
	// 82517D20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82517D24: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82517D28: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517D90 size=68
    let mut pc: u32 = 0x82517D90;
    'dispatch: loop {
        match pc {
            0x82517D90 => {
    //   block [0x82517D90..0x82517DD4)
	// 82517D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517D9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82517DA0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82517DA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82517DA8: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 82517DAC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82517DB0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82517DB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82517DB8: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82517DBC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82517DC0: 4BFFEC69  bl 0x82516a28
	ctx.lr = 0x82517DC4;
	sub_82516A28(ctx, base);
	// 82517DC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82517DD8 size=72
    let mut pc: u32 = 0x82517DD8;
    'dispatch: loop {
        match pc {
            0x82517DD8 => {
    //   block [0x82517DD8..0x82517E20)
	// 82517DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517DE4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82517DE8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82517DEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82517DF0: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 82517DF4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82517DF8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82517DFC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82517E00: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82517E04: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82517E08: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82517E0C: 4BFFEF1D  bl 0x82516d28
	ctx.lr = 0x82517E10;
	sub_82516D28(ctx, base);
	// 82517E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82517E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517E20 size=228
    let mut pc: u32 = 0x82517E20;
    'dispatch: loop {
        match pc {
            0x82517E20 => {
    //   block [0x82517E20..0x82517F04)
	// 82517E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517E28: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517E2C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82517E30: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82517E34: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82517E38: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82517E3C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82517E40: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82517E44: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82517E48: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82517E4C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82517E50: 4200FFF0  bdnz 0x82517e40
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82517E40; continue 'dispatch;
	}
	// 82517E54: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82517E58: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82517E5C: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82517E60: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82517E64: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82517E68: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F08 size=16
    let mut pc: u32 = 0x82517F08;
    'dispatch: loop {
        match pc {
            0x82517F08 => {
    //   block [0x82517F08..0x82517F18)
	// 82517F08: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F0C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82517F10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F14: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F18 size=12
    let mut pc: u32 = 0x82517F18;
    'dispatch: loop {
        match pc {
            0x82517F18 => {
    //   block [0x82517F18..0x82517F24)
	// 82517F18: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82517F1C: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82517F20: 4BFBE6F0  b 0x824d6610
	sub_824D6610(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F24 size=4
    let mut pc: u32 = 0x82517F24;
    'dispatch: loop {
        match pc {
            0x82517F24 => {
    //   block [0x82517F24..0x82517F28)
	// 82517F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F28 size=16
    let mut pc: u32 = 0x82517F28;
    'dispatch: loop {
        match pc {
            0x82517F28 => {
    //   block [0x82517F28..0x82517F38)
	// 82517F28: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F30: 419A0008  beq cr6, 0x82517f38
	if ctx.cr[6].eq {
		sub_82517F38(ctx, base);
		return;
	}
	// 82517F34: 48012BCC  b 0x8252ab00
	sub_8252AB00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F38 size=8
    let mut pc: u32 = 0x82517F38;
    'dispatch: loop {
        match pc {
            0x82517F38 => {
    //   block [0x82517F38..0x82517F40)
	// 82517F38: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82517F3C: 480A57E4  b 0x825bd720
	sub_825BD720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F40 size=16
    let mut pc: u32 = 0x82517F40;
    'dispatch: loop {
        match pc {
            0x82517F40 => {
    //   block [0x82517F40..0x82517F50)
	// 82517F40: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F48: 419A0008  beq cr6, 0x82517f50
	if ctx.cr[6].eq {
		sub_82517F50(ctx, base);
		return;
	}
	// 82517F4C: 48012BD4  b 0x8252ab20
	sub_8252AB20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F50 size=8
    let mut pc: u32 = 0x82517F50;
    'dispatch: loop {
        match pc {
            0x82517F50 => {
    //   block [0x82517F50..0x82517F58)
	// 82517F50: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82517F54: 480A591C  b 0x825bd870
	sub_825BD870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F58 size=12
    let mut pc: u32 = 0x82517F58;
    'dispatch: loop {
        match pc {
            0x82517F58 => {
    //   block [0x82517F58..0x82517F64)
	// 82517F58: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F60: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F64 size=8
    let mut pc: u32 = 0x82517F64;
    'dispatch: loop {
        match pc {
            0x82517F64 => {
    //   block [0x82517F64..0x82517F6C)
	// 82517F64: 480137C4  b 0x8252b728
	sub_8252B728(ctx, base);
	return;
	// 82517F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F70 size=12
    let mut pc: u32 = 0x82517F70;
    'dispatch: loop {
        match pc {
            0x82517F70 => {
    //   block [0x82517F70..0x82517F7C)
	// 82517F70: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F7C size=8
    let mut pc: u32 = 0x82517F7C;
    'dispatch: loop {
        match pc {
            0x82517F7C => {
    //   block [0x82517F7C..0x82517F84)
	// 82517F7C: 480137F4  b 0x8252b770
	sub_8252B770(ctx, base);
	return;
	// 82517F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F88 size=12
    let mut pc: u32 = 0x82517F88;
    'dispatch: loop {
        match pc {
            0x82517F88 => {
    //   block [0x82517F88..0x82517F94)
	// 82517F88: 89630084  lbz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82517F8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82517F90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82517F94 size=8
    let mut pc: u32 = 0x82517F94;
    'dispatch: loop {
        match pc {
            0x82517F94 => {
    //   block [0x82517F94..0x82517F9C)
	// 82517F94: 48013824  b 0x8252b7b8
	sub_8252B7B8(ctx, base);
	return;
	// 82517F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517FA0 size=76
    let mut pc: u32 = 0x82517FA0;
    'dispatch: loop {
        match pc {
            0x82517FA0 => {
    //   block [0x82517FA0..0x82517FEC)
	// 82517FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517FA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517FAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82517FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82517FB4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82517FB8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82517FBC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82517FC0: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82517FC4: 480A5AFD  bl 0x825bdac0
	ctx.lr = 0x82517FC8;
	sub_825BDAC0(ctx, base);
	// 82517FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82517FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82517FD0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82517FD4: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82517FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82517FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82517FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82517FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82517FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82517FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82517FF0 size=112
    let mut pc: u32 = 0x82517FF0;
    'dispatch: loop {
        match pc {
            0x82517FF0 => {
    //   block [0x82517FF0..0x82518060)
	// 82517FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82517FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82517FF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82517FFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518004: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82518008: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8251800C: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82518010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518014: 419A0014  beq cr6, 0x82518028
	if ctx.cr[6].eq {
	pc = 0x82518028; continue 'dispatch;
	}
	// 82518018: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251801C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82518020: 480AA9F9  bl 0x825c2a18
	ctx.lr = 0x82518024;
	sub_825C2A18(ctx, base);
	// 82518024: 48000010  b 0x82518034
	pc = 0x82518034; continue 'dispatch;
	// 82518028: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251802C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82518030: 480A5A91  bl 0x825bdac0
	ctx.lr = 0x82518034;
	sub_825BDAC0(ctx, base);
	// 82518034: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251803C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518040: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82518048: 4E800421  bctrl
	ctx.lr = 0x8251804C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251804C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82518050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82518058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251805C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518060 size=112
    let mut pc: u32 = 0x82518060;
    'dispatch: loop {
        match pc {
            0x82518060 => {
    //   block [0x82518060..0x825180D0)
	// 82518060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82518068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251806C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518074: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82518078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251807C: 409A0040  bne cr6, 0x825180bc
	if !ctx.cr[6].eq {
	pc = 0x825180BC; continue 'dispatch;
	}
	// 82518080: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82518084: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518088: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8251808C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82518090: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82518094: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82518098: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 8251809C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825180A0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825180A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825180A8: 4E800421  bctrl
	ctx.lr = 0x825180AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825180AC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 825180B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825180B4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825180B8: 480A6A79  bl 0x825beb30
	ctx.lr = 0x825180BC;
	sub_825BEB30(ctx, base);
	// 825180BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825180C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825180C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825180C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825180CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825180D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825180D0 size=144
    let mut pc: u32 = 0x825180D0;
    'dispatch: loop {
        match pc {
            0x825180D0 => {
    //   block [0x825180D0..0x82518160)
	// 825180D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825180D4: 4801CFE1  bl 0x825350b4
	ctx.lr = 0x825180D8;
	sub_82535080(ctx, base);
	// 825180D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825180DC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825180E0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825180E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825180E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825180EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825180F0: 48013711  bl 0x8252b800
	ctx.lr = 0x825180F4;
	sub_8252B800(ctx, base);
	// 825180F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825180F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825180FC: 396B4F94  addi r11, r11, 0x4f94
	ctx.r[11].s64 = ctx.r[11].s64 + 20372;
	// 82518100: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82518104: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82518108: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251810C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518110: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82518114: 995F0084  stb r10, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[10].u8 ) };
	// 82518118: 993F0085  stb r9, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[9].u8 ) };
	// 8251811C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518120: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518124: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82518128: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251812C: 4809102D  bl 0x825a9158
	ctx.lr = 0x82518130;
	sub_825A9158(ctx, base);
	// 82518130: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82518134: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82518138: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251813C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82518140: 480ACB09  bl 0x825c4c48
	ctx.lr = 0x82518144;
	sub_825C4C48(ctx, base);
	// 82518144: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82518148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251814C: C00B2074  lfs f0, 0x2074(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82518150: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82518154: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82518158: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8251815C: 4801CFA8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518160 size=592
    let mut pc: u32 = 0x82518160;
    'dispatch: loop {
        match pc {
            0x82518160 => {
    //   block [0x82518160..0x825183B0)
	// 82518160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518164: 4801CF49  bl 0x825350ac
	ctx.lr = 0x82518168;
	sub_82535080(ctx, base);
	// 82518168: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8251816C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518170: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518174: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82518178: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251817C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82518180: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82518184: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82518188: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8251818C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518190: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518194: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82518198: 4098002C  bge cr6, 0x825181c4
	if !ctx.cr[6].lt {
	pc = 0x825181C4; continue 'dispatch;
	}
	// 8251819C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825181A0: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 825181A4: 39294FE8  addi r9, r9, 0x4fe8
	ctx.r[9].s64 = ctx.r[9].s64 + 20456;
	// 825181A8: 39084FDC  addi r8, r8, 0x4fdc
	ctx.r[8].s64 = ctx.r[8].s64 + 20444;
	// 825181AC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825181B0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 825181B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825181B8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 825181BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825181C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825181C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825181C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825181CC: 396B4F50  addi r11, r11, 0x4f50
	ctx.r[11].s64 = ctx.r[11].s64 + 20304;
	// 825181D0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825181D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825181D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825181DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825181E0: 9B810054  stb r28, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u8 ) };
	// 825181E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825181E8: 48012D39  bl 0x8252af20
	ctx.lr = 0x825181EC;
	sub_8252AF20(ctx, base);
	// 825181EC: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825181F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825181F4: 419A0074  beq cr6, 0x82518268
	if ctx.cr[6].eq {
	pc = 0x82518268; continue 'dispatch;
	}
	// 825181F8: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825181FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518200: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518204: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82518208: 40980020  bge cr6, 0x82518228
	if !ctx.cr[6].lt {
	pc = 0x82518228; continue 'dispatch;
	}
	// 8251820C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82518210: 39294FD0  addi r9, r9, 0x4fd0
	ctx.r[9].s64 = ctx.r[9].s64 + 20432;
	// 82518214: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518218: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251821C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82518220: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82518224: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82518228: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251822C: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82518230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82518234: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 82518238: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 8251823C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518240: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518248: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251824C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82518250: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82518254: 4800D15D  bl 0x825253b0
	ctx.lr = 0x82518258;
	sub_825253B0(ctx, base);
	// 82518258: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251825C: 396B228C  addi r11, r11, 0x228c
	ctx.r[11].s64 = ctx.r[11].s64 + 8844;
	// 82518260: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82518264: 48000104  b 0x82518368
	pc = 0x82518368; continue 'dispatch;
	// 82518268: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251826C: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82518270: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82518274: 396B2298  addi r11, r11, 0x2298
	ctx.r[11].s64 = ctx.r[11].s64 + 8856;
	// 82518278: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8251827C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518280: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82518284: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518288: C3EA8CB4  lfs f31, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251828C: D3E100AC  stfs f31, 0xac(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82518290: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82518294: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82518298: 48012E31  bl 0x8252b0c8
	ctx.lr = 0x8251829C;
	sub_8252B0C8(ctx, base);
	// 8251829C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 825182A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825182A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825182A8: 3B8B228C  addi r28, r11, 0x228c
	ctx.r[28].s64 = ctx.r[11].s64 + 8844;
	// 825182AC: 419A00B8  beq cr6, 0x82518364
	if ctx.cr[6].eq {
	pc = 0x82518364; continue 'dispatch;
	}
	// 825182B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825182B4: C1A100AC  lfs f13, 0xac(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825182B8: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825182BC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825182C0: 40990048  ble cr6, 0x82518308
	if !ctx.cr[6].gt {
	pc = 0x82518308; continue 'dispatch;
	}
	// 825182C4: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 825182C8: 93C100E0  stw r30, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 825182CC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825182D0: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 825182D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825183B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825183B0 size=20
    let mut pc: u32 = 0x825183B0;
    'dispatch: loop {
        match pc {
            0x825183B0 => {
    //   block [0x825183B0..0x825183C4)
	// 825183B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825183B4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825183B8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825183BC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825183C0: 4BFFFDA0  b 0x82518160
	sub_82518160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825183C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825183C8 size=308
    let mut pc: u32 = 0x825183C8;
    'dispatch: loop {
        match pc {
            0x825183C8 => {
    //   block [0x825183C8..0x825184FC)
	// 825183C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825183CC: 4801CCDD  bl 0x825350a8
	ctx.lr = 0x825183D0;
	sub_82535080(ctx, base);
	// 825183D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825183D4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825183D8: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 825183DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825183E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825183E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825183E8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 825183EC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825183F0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825183F4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825183F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825183FC: 4098002C  bge cr6, 0x82518428
	if !ctx.cr[6].lt {
	pc = 0x82518428; continue 'dispatch;
	}
	// 82518400: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82518404: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82518408: 39294FE8  addi r9, r9, 0x4fe8
	ctx.r[9].s64 = ctx.r[9].s64 + 20456;
	// 8251840C: 39084FDC  addi r8, r8, 0x4fdc
	ctx.r[8].s64 = ctx.r[8].s64 + 20444;
	// 82518410: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518414: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82518418: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251841C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82518420: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82518424: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82518428: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251842C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82518430: 396B4F50  addi r11, r11, 0x4f50
	ctx.r[11].s64 = ctx.r[11].s64 + 20304;
	// 82518434: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82518438: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251843C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82518444: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82518448: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251844C: 48012AD5  bl 0x8252af20
	ctx.lr = 0x82518450;
	sub_8252AF20(ctx, base);
	// 82518450: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82518454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82518458: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251845C: 3BEB4B88  addi r31, r11, 0x4b88
	ctx.r[31].s64 = ctx.r[11].s64 + 19336;
	// 82518460: 419A0060  beq cr6, 0x825184c0
	if ctx.cr[6].eq {
	pc = 0x825184C0; continue 'dispatch;
	}
	// 82518464: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82518468: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251846C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518470: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82518474: 40980020  bge cr6, 0x82518494
	if !ctx.cr[6].lt {
	pc = 0x82518494; continue 'dispatch;
	}
	// 82518478: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251847C: 39294FD0  addi r9, r9, 0x4fd0
	ctx.r[9].s64 = ctx.r[9].s64 + 20432;
	// 82518480: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518484: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82518488: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8251848C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82518490: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82518494: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518498: 9B61005C  stb r27, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u8 ) };
	// 8251849C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 825184A0: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 825184A4: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 825184A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825184AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825184B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825184B4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825184B8: 4800D491  bl 0x82525948
	ctx.lr = 0x825184BC;
	sub_82525948(ctx, base);
	// 825184BC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825184C0: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825184C4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825184C8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825184CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825184D0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825184D4: 40980020  bge cr6, 0x825184f4
	if !ctx.cr[6].lt {
	pc = 0x825184F4; continue 'dispatch;
	}
	// 825184D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825184DC: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 825184E0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825184E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825184E8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825184EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825184F0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825184F4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825184F8: 4801CC00  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82518500 size=20
    let mut pc: u32 = 0x82518500;
    'dispatch: loop {
        match pc {
            0x82518500 => {
    //   block [0x82518500..0x82518514)
	// 82518500: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82518504: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82518508: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251850C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82518510: 4BFFFEB8  b 0x825183c8
	sub_825183C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82518518 size=312
    let mut pc: u32 = 0x82518518;
    'dispatch: loop {
        match pc {
            0x82518518 => {
    //   block [0x82518518..0x82518650)
	// 82518518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251851C: 4801CB91  bl 0x825350ac
	ctx.lr = 0x82518520;
	sub_82535080(ctx, base);
	// 82518520: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518524: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518528: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 8251852C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82518530: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82518534: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82518538: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8251853C: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82518540: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82518544: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518548: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251854C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82518550: 4098002C  bge cr6, 0x8251857c
	if !ctx.cr[6].lt {
	pc = 0x8251857C; continue 'dispatch;
	}
	// 82518554: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82518558: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8251855C: 39294FFC  addi r9, r9, 0x4ffc
	ctx.r[9].s64 = ctx.r[9].s64 + 20476;
	// 82518560: 39084FDC  addi r8, r8, 0x4fdc
	ctx.r[8].s64 = ctx.r[8].s64 + 20444;
	// 82518564: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518568: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8251856C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82518570: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82518574: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82518578: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251857C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518580: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82518584: 396B4F5C  addi r11, r11, 0x4f5c
	ctx.r[11].s64 = ctx.r[11].s64 + 20316;
	// 82518588: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251858C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82518590: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82518594: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82518598: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251859C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825185A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825185A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825185A8: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825185AC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825185B0: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 825185B4: 48013945  bl 0x8252bef8
	ctx.lr = 0x825185B8;
	sub_8252BEF8(ctx, base);
	// 825185B8: 89610058  lbz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825185BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825185C0: 419A004C  beq cr6, 0x8251860c
	if ctx.cr[6].eq {
	pc = 0x8251860C; continue 'dispatch;
	}
	// 825185C4: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825185C8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825185CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825185D0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825185D4: 40980020  bge cr6, 0x825185f4
	if !ctx.cr[6].lt {
	pc = 0x825185F4; continue 'dispatch;
	}
	// 825185D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825185DC: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 825185E0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825185E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825185E8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825185EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825185F0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825185F4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 825185F8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825185FC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82518600: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82518604: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82518608: 48000D79  bl 0x82519380
	ctx.lr = 0x8251860C;
	sub_82519380(ctx, base);
	// 8251860C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518610: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82518614: 396B228C  addi r11, r11, 0x228c
	ctx.r[11].s64 = ctx.r[11].s64 + 8844;
	// 82518618: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251861C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82518620: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518624: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82518628: 40980020  bge cr6, 0x82518648
	if !ctx.cr[6].lt {
	pc = 0x82518648; continue 'dispatch;
	}
	// 8251862C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82518630: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82518634: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82518638: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251863C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82518640: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82518644: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82518648: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8251864C: 4801CAB0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82518650 size=24
    let mut pc: u32 = 0x82518650;
    'dispatch: loop {
        match pc {
            0x82518650 => {
    //   block [0x82518650..0x82518668)
	// 82518650: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82518654: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82518658: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251865C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82518660: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82518664: 4BFFFEB4  b 0x82518518
	sub_82518518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518668 size=348
    let mut pc: u32 = 0x82518668;
    'dispatch: loop {
        match pc {
            0x82518668 => {
    //   block [0x82518668..0x825187C4)
	// 82518668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251866C: 4801CA49  bl 0x825350b4
	ctx.lr = 0x82518670;
	sub_82535080(ctx, base);
	// 82518670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518674: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82518678: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251867C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82518680: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82518684: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82518688: 419A0100  beq cr6, 0x82518788
	if ctx.cr[6].eq {
	pc = 0x82518788; continue 'dispatch;
	}
	// 8251868C: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518690: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82518694: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518698: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251869C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825186A0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825186A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825186A8: 4E800421  bctrl
	ctx.lr = 0x825186AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825186AC: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 825186B0: 41980088  blt cr6, 0x82518738
	if ctx.cr[6].lt {
	pc = 0x82518738; continue 'dispatch;
	}
	// 825186B4: 419A0064  beq cr6, 0x82518718
	if ctx.cr[6].eq {
	pc = 0x82518718; continue 'dispatch;
	}
	// 825186B8: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 825186BC: 41980010  blt cr6, 0x825186cc
	if ctx.cr[6].lt {
	pc = 0x825186CC; continue 'dispatch;
	}
	// 825186C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825186C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825186C8: 4801CA3C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 825186CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825186D0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825186D4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825186D8: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825186DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825186E0: 4BF4B959  bl 0x82464038
	ctx.lr = 0x825186E4;
	sub_82464038(ctx, base);
	// 825186E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825186E8: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 825186EC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825186F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825186F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825186F8: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825186FC: 48013105  bl 0x8252b800
	ctx.lr = 0x82518700;
	sub_8252B800(ctx, base);
	// 82518700: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518708: 396B4F14  addi r11, r11, 0x4f14
	ctx.r[11].s64 = ctx.r[11].s64 + 20244;
	// 8251870C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518714: 4801C9F0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518718: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251871C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518720: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518724: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82518728: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251872C: 4BF4B90D  bl 0x82464038
	ctx.lr = 0x82518730;
	sub_82464038(ctx, base);
	// 82518730: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82518734: 48000070  b 0x825187a4
	pc = 0x825187A4; continue 'dispatch;
	// 82518738: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251873C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518740: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518744: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82518748: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251874C: 4BF4B8ED  bl 0x82464038
	ctx.lr = 0x82518750;
	sub_82464038(ctx, base);
	// 82518750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518754: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82518758: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8251875C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82518760: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82518764: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518768: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8251876C: 4BFFF965  bl 0x825180d0
	ctx.lr = 0x82518770;
	sub_825180D0(ctx, base);
	// 82518770: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518778: 396B5014  addi r11, r11, 0x5014
	ctx.r[11].s64 = ctx.r[11].s64 + 20500;
	// 8251877C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518784: 4801C980  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518788: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251878C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518790: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518794: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82518798: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251879C: 4BF4B89D  bl 0x82464038
	ctx.lr = 0x825187A0;
	sub_82464038(ctx, base);
	// 825187A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825187A4: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 825187A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825187AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825187B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825187B4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825187B8: 4800D5B1  bl 0x82525d68
	ctx.lr = 0x825187BC;
	sub_82525D68(ctx, base);
	// 825187BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825187C0: 4801C944  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825187C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825187C8 size=128
    let mut pc: u32 = 0x825187C8;
    'dispatch: loop {
        match pc {
            0x825187C8 => {
    //   block [0x825187C8..0x82518848)
	// 825187C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825187CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825187D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825187D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825187D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825187DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825187E0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825187E4: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 825187E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825187EC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825187F0: 480AA229  bl 0x825c2a18
	ctx.lr = 0x825187F4;
	sub_825C2A18(ctx, base);
	// 825187F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825187F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825187FC: 997E0084  stb r11, 0x84(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 82518800: 419A0014  beq cr6, 0x82518814
	if ctx.cr[6].eq {
	pc = 0x82518814; continue 'dispatch;
	}
	// 82518804: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82518808: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251880C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82518810: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82518814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518818: 480A54B9  bl 0x825bdcd0
	ctx.lr = 0x8251881C;
	sub_825BDCD0(ctx, base);
	// 8251881C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82518820: 39400019  li r10, 0x19
	ctx.r[10].s64 = 25;
	// 82518824: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82518828: D01E003C  stfs f0, 0x3c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8251882C: B15E0086  sth r10, 0x86(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 82518830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82518834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82518838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251883C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82518840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82518844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82518848 size=1448
    let mut pc: u32 = 0x82518848;
    'dispatch: loop {
        match pc {
            0x82518848 => {
    //   block [0x82518848..0x82518DF0)
	// 82518848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251884C: 4801C835  bl 0x82535080
	ctx.lr = 0x82518850;
	sub_82535080(ctx, base);
	// 82518850: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82518854: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82518858: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518DF0 size=344
    let mut pc: u32 = 0x82518DF0;
    'dispatch: loop {
        match pc {
            0x82518DF0 => {
    //   block [0x82518DF0..0x82518F48)
	// 82518DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518DF4: 4801C2C1  bl 0x825350b4
	ctx.lr = 0x82518DF8;
	sub_82535080(ctx, base);
	// 82518DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518DFC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82518E00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518E04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82518E08: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82518E0C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82518E10: 419A00E8  beq cr6, 0x82518ef8
	if ctx.cr[6].eq {
	pc = 0x82518EF8; continue 'dispatch;
	}
	// 82518E14: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518E18: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82518E1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82518E20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518E24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82518E2C: 4E800421  bctrl
	ctx.lr = 0x82518E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82518E30: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82518E34: 41980088  blt cr6, 0x82518ebc
	if ctx.cr[6].lt {
	pc = 0x82518EBC; continue 'dispatch;
	}
	// 82518E38: 419A0064  beq cr6, 0x82518e9c
	if ctx.cr[6].eq {
	pc = 0x82518E9C; continue 'dispatch;
	}
	// 82518E3C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82518E40: 41980010  blt cr6, 0x82518e50
	if ctx.cr[6].lt {
	pc = 0x82518E50; continue 'dispatch;
	}
	// 82518E44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518E4C: 4801C2B8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518E50: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518E54: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518E58: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518E5C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82518E60: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518E64: 4BF4B1D5  bl 0x82464038
	ctx.lr = 0x82518E68;
	sub_82464038(ctx, base);
	// 82518E68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518E6C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82518E70: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82518E74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518E78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518E7C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82518E80: 48012981  bl 0x8252b800
	ctx.lr = 0x82518E84;
	sub_8252B800(ctx, base);
	// 82518E84: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518E8C: 396B4F14  addi r11, r11, 0x4f14
	ctx.r[11].s64 = ctx.r[11].s64 + 20244;
	// 82518E90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518E98: 4801C26C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518E9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518EA0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518EA4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518EA8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82518EAC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518EB0: 4BF4B189  bl 0x82464038
	ctx.lr = 0x82518EB4;
	sub_82464038(ctx, base);
	// 82518EB4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82518EB8: 4800005C  b 0x82518f14
	pc = 0x82518F14; continue 'dispatch;
	// 82518EBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518EC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518EC4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518EC8: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82518ECC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518ED0: 4BF4B169  bl 0x82464038
	ctx.lr = 0x82518ED4;
	sub_82464038(ctx, base);
	// 82518ED4: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82518ED8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82518EDC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82518EE0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518EE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518EE8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82518EEC: 4BFFF1E5  bl 0x825180d0
	ctx.lr = 0x82518EF0;
	sub_825180D0(ctx, base);
	// 82518EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518EF4: 4801C210  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518EF8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518EFC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518F00: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518F04: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82518F08: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518F0C: 4BF4B12D  bl 0x82464038
	ctx.lr = 0x82518F10;
	sub_82464038(ctx, base);
	// 82518F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82518F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518F18: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82518F1C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82518F20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82518F24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518F28: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82518F2C: 4800CE3D  bl 0x82525d68
	ctx.lr = 0x82518F30;
	sub_82525D68(ctx, base);
	// 82518F30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82518F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82518F38: 396B4C60  addi r11, r11, 0x4c60
	ctx.r[11].s64 = ctx.r[11].s64 + 19552;
	// 82518F3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82518F40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518F44: 4801C1C0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82518F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82518F48 size=540
    let mut pc: u32 = 0x82518F48;
    'dispatch: loop {
        match pc {
            0x82518F48 => {
    //   block [0x82518F48..0x82519164)
	// 82518F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82518F4C: 4801C169  bl 0x825350b4
	ctx.lr = 0x82518F50;
	sub_82535080(ctx, base);
	// 82518F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82518F54: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82518F58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82518F5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82518F60: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82518F64: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82518F68: 419A01C0  beq cr6, 0x82519128
	if ctx.cr[6].eq {
	pc = 0x82519128; continue 'dispatch;
	}
	// 82518F6C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518F70: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82518F74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82518F78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82518F7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518F80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82518F88: 4E800421  bctrl
	ctx.lr = 0x82518F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82518F8C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82518F90: 41980148  blt cr6, 0x825190d8
	if ctx.cr[6].lt {
	pc = 0x825190D8; continue 'dispatch;
	}
	// 82518F94: 419A0124  beq cr6, 0x825190b8
	if ctx.cr[6].eq {
	pc = 0x825190B8; continue 'dispatch;
	}
	// 82518F98: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82518F9C: 40980038  bge cr6, 0x82518fd4
	if !ctx.cr[6].lt {
	pc = 0x82518FD4; continue 'dispatch;
	}
	// 82518FA0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518FA4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82518FA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82518FAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82518FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518FB4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82518FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82518FBC: 4E800421  bctrl
	ctx.lr = 0x82518FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82518FC0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82518FC4: 419800B8  blt cr6, 0x8251907c
	if ctx.cr[6].lt {
	pc = 0x8251907C; continue 'dispatch;
	}
	// 82518FC8: 419A0064  beq cr6, 0x8251902c
	if ctx.cr[6].eq {
	pc = 0x8251902C; continue 'dispatch;
	}
	// 82518FCC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82518FD0: 41980010  blt cr6, 0x82518fe0
	if ctx.cr[6].lt {
	pc = 0x82518FE0; continue 'dispatch;
	}
	// 82518FD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82518FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82518FDC: 4801C128  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82518FE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82518FE4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82518FE8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82518FEC: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82518FF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82518FF4: 4BF4B045  bl 0x82464038
	ctx.lr = 0x82518FF8;
	sub_82464038(ctx, base);
	// 82518FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82518FFC: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82519000: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82519004: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82519008: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251900C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82519010: 480127F1  bl 0x8252b800
	ctx.lr = 0x82519014;
	sub_8252B800(ctx, base);
	// 82519014: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251901C: 396B4F14  addi r11, r11, 0x4f14
	ctx.r[11].s64 = ctx.r[11].s64 + 20244;
	// 82519020: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519028: 4801C0DC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8251902C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519030: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82519034: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82519038: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8251903C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82519040: 4BF4AFF9  bl 0x82464038
	ctx.lr = 0x82519044;
	sub_82464038(ctx, base);
	// 82519044: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82519048: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 8251904C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82519050: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82519054: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82519058: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251905C: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82519060: 4800CD09  bl 0x82525d68
	ctx.lr = 0x82519064;
	sub_82525D68(ctx, base);
	// 82519064: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519068: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8251906C: 396B4C60  addi r11, r11, 0x4c60
	ctx.r[11].s64 = ctx.r[11].s64 + 19552;
	// 82519070: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519078: 4801C08C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8251907C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519080: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82519084: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82519088: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 8251908C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82519090: 4BF4AFA9  bl 0x82464038
	ctx.lr = 0x82519094;
	sub_82464038(ctx, base);
	// 82519094: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82519098: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8251909C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 825190A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825190A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825190A8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825190AC: 4BFFF025  bl 0x825180d0
	ctx.lr = 0x825190B0;
	sub_825180D0(ctx, base);
	// 825190B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825190B4: 4801C050  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 825190B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825190BC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825190C0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825190C4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 825190C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825190CC: 4BF4AF6D  bl 0x82464038
	ctx.lr = 0x825190D0;
	sub_82464038(ctx, base);
	// 825190D0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825190D4: 48000070  b 0x82519144
	pc = 0x82519144; continue 'dispatch;
	// 825190D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825190DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825190E0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825190E4: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 825190E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825190EC: 4BF4AF4D  bl 0x82464038
	ctx.lr = 0x825190F0;
	sub_82464038(ctx, base);
	// 825190F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825190F4: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 825190F8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825190FC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82519100: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82519104: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82519108: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8251910C: 4BFFEFC5  bl 0x825180d0
	ctx.lr = 0x82519110;
	sub_825180D0(ctx, base);
	// 82519110: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519114: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82519118: 396B5014  addi r11, r11, 0x5014
	ctx.r[11].s64 = ctx.r[11].s64 + 20500;
	// 8251911C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519124: 4801BFE0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82519128: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251912C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82519130: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82519134: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82519138: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251913C: 4BF4AEFD  bl 0x82464038
	ctx.lr = 0x82519140;
	sub_82464038(ctx, base);
	// 82519140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82519144: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82519148: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8251914C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82519150: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519154: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82519158: 4800CC11  bl 0x82525d68
	ctx.lr = 0x8251915C;
	sub_82525D68(ctx, base);
	// 8251915C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519160: 4801BFA4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519168 size=256
    let mut pc: u32 = 0x82519168;
    'dispatch: loop {
        match pc {
            0x82519168 => {
    //   block [0x82519168..0x82519268)
	// 82519168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251916C: 4801BF51  bl 0x825350bc
	ctx.lr = 0x82519170;
	sub_82535080(ctx, base);
	// 82519170: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519174: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82519178: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251917C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82519180: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82519184: 39088668  addi r8, r8, -0x7998
	ctx.r[8].s64 = ctx.r[8].s64 + -31128;
	// 82519188: 392998F8  addi r9, r9, -0x6708
	ctx.r[9].s64 = ctx.r[9].s64 + -26376;
	// 8251918C: 394A9948  addi r10, r10, -0x66b8
	ctx.r[10].s64 = ctx.r[10].s64 + -26296;
	// 82519190: 396B9998  addi r11, r11, -0x6668
	ctx.r[11].s64 = ctx.r[11].s64 + -26216;
	// 82519194: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82519198: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251919C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 825191A0: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 825191A4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 825191A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825191AC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825191B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825191B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825191B8: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 825191BC: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825191C0: 4BFE6BC9  bl 0x824ffd88
	ctx.lr = 0x825191C4;
	sub_824FFD88(ctx, base);
	// 825191C4: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 825191C8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825191CC: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 825191D0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 825191D4: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 825191D8: 39088DF0  addi r8, r8, -0x7210
	ctx.r[8].s64 = ctx.r[8].s64 + -29200;
	// 825191DC: 392983C8  addi r9, r9, -0x7c38
	ctx.r[9].s64 = ctx.r[9].s64 + -31800;
	// 825191E0: 394A8160  addi r10, r10, -0x7ea0
	ctx.r[10].s64 = ctx.r[10].s64 + -32416;
	// 825191E4: 396B8518  addi r11, r11, -0x7ae8
	ctx.r[11].s64 = ctx.r[11].s64 + -31464;
	// 825191E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825191EC: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 825191F0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825191F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825191F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825191FC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82519200: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82519204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519208: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251920C: 9BA10080  stb r29, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u8 ) };
	// 82519210: 4BFE6B79  bl 0x824ffd88
	ctx.lr = 0x82519214;
	sub_824FFD88(ctx, base);
	// 82519214: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82519218: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251921C: 9BA100A0  stb r29, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[29].u8 ) };
	// 82519220: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82519224: 9BC100A1  stb r30, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[30].u8 ) };
	// 82519228: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251922C: 39088F48  addi r8, r8, -0x70b8
	ctx.r[8].s64 = ctx.r[8].s64 + -28856;
	// 82519230: 392983C8  addi r9, r9, -0x7c38
	ctx.r[9].s64 = ctx.r[9].s64 + -31800;
	// 82519234: 394A8160  addi r10, r10, -0x7ea0
	ctx.r[10].s64 = ctx.r[10].s64 + -32416;
	// 82519238: 396B8518  addi r11, r11, -0x7ae8
	ctx.r[11].s64 = ctx.r[11].s64 + -31464;
	// 8251923C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82519240: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82519244: 91010090  stw r8, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82519248: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8251924C: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82519250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519254: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82519258: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8251925C: 4BFE6B2D  bl 0x824ffd88
	ctx.lr = 0x82519260;
	sub_824FFD88(ctx, base);
	// 82519260: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82519264: 4801BEA8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519268 size=20
    let mut pc: u32 = 0x82519268;
    'dispatch: loop {
        match pc {
            0x82519268 => {
    //   block [0x82519268..0x8251927C)
	// 82519268: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251926C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519270: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82519274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519278: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519280 size=20
    let mut pc: u32 = 0x82519280;
    'dispatch: loop {
        match pc {
            0x82519280 => {
    //   block [0x82519280..0x82519294)
	// 82519280: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519288: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251928C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519290: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519298 size=20
    let mut pc: u32 = 0x82519298;
    'dispatch: loop {
        match pc {
            0x82519298 => {
    //   block [0x82519298..0x825192AC)
	// 82519298: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251929C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825192A0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825192A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825192A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825192B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825192B0 size=20
    let mut pc: u32 = 0x825192B0;
    'dispatch: loop {
        match pc {
            0x825192B0 => {
    //   block [0x825192B0..0x825192C4)
	// 825192B0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825192B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825192B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825192BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825192C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825192C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825192C8 size=20
    let mut pc: u32 = 0x825192C8;
    'dispatch: loop {
        match pc {
            0x825192C8 => {
    //   block [0x825192C8..0x825192DC)
	// 825192C8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825192CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825192D0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 825192D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825192D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825192E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825192E0 size=76
    let mut pc: u32 = 0x825192E0;
    'dispatch: loop {
        match pc {
            0x825192E0 => {
    //   block [0x825192E0..0x8251932C)
	// 825192E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825192E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825192E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825192EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825192F0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825192F4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825192F8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825192FC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519300: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82519304: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82519308: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251930C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82519310: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82519314: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82519318: 4800C631  bl 0x82525948
	ctx.lr = 0x8251931C;
	sub_82525948(ctx, base);
	// 8251931C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519330 size=80
    let mut pc: u32 = 0x82519330;
    'dispatch: loop {
        match pc {
            0x82519330 => {
    //   block [0x82519330..0x82519380)
	// 82519330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251933C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82519340: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82519344: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519348: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251934C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82519350: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82519354: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519358: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251935C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82519360: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82519364: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82519368: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251936C: 4800C045  bl 0x825253b0
	ctx.lr = 0x82519370;
	sub_825253B0(ctx, base);
	// 82519370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251937C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519380 size=232
    let mut pc: u32 = 0x82519380;
    'dispatch: loop {
        match pc {
            0x82519380 => {
    //   block [0x82519380..0x82519468)
	// 82519380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251938C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519394: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82519398: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8251939C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825193A0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825193A4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 825193A8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825193AC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825193B0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825193B4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825193B8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825193BC: 4200FFF0  bdnz 0x825193ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825193AC; continue 'dispatch;
	}
	// 825193C0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825193C4: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 825193C8: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 825193CC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825193D0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825193D4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519468 size=140
    let mut pc: u32 = 0x82519468;
    'dispatch: loop {
        match pc {
            0x82519468 => {
    //   block [0x82519468..0x825194F4)
	// 82519468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519478: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251947C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519480: 396B228C  addi r11, r11, 0x228c
	ctx.r[11].s64 = ctx.r[11].s64 + 8844;
	// 82519484: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82519488: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251948C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82519490: 419A004C  beq cr6, 0x825194dc
	if ctx.cr[6].eq {
	pc = 0x825194DC; continue 'dispatch;
	}
	// 82519494: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519498: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251949C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825194A0: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825194A4: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825194A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825194AC: 41980018  blt cr6, 0x825194c4
	if ctx.cr[6].lt {
	pc = 0x825194C4; continue 'dispatch;
	}
	// 825194B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825194B4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825194B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825194BC: 4BF4AA5D  bl 0x82463f18
	ctx.lr = 0x825194C0;
	sub_82463F18(ctx, base);
	// 825194C0: 4800001C  b 0x825194dc
	pc = 0x825194DC; continue 'dispatch;
	// 825194C4: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825194C8: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825194CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825194D0: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 825194D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825194D8: 93EB0058  stw r31, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825194DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825194E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825194E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825194E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825194EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825194F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825194F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825194F8 size=232
    let mut pc: u32 = 0x825194F8;
    'dispatch: loop {
        match pc {
            0x825194F8 => {
    //   block [0x825194F8..0x825195E0)
	// 825194F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825194FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82519504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251950C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82519510: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82519514: C1BE1030  lfs f13, 0x1030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82519518: C01F3030  lfs f0, 0x3030(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251951C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82519520: 419A0088  beq cr6, 0x825195a8
	if ctx.cr[6].eq {
	pc = 0x825195A8; continue 'dispatch;
	}
	// 82519524: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519528: 38BF3050  addi r5, r31, 0x3050
	ctx.r[5].s64 = ctx.r[31].s64 + 12368;
	// 8251952C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519530: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82519534: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82519538: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251953C: 4E800421  bctrl
	ctx.lr = 0x82519540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519540: 397E1010  addi r11, r30, 0x1010
	ctx.r[11].s64 = ctx.r[30].s64 + 4112;
	// 82519544: 395F3010  addi r10, r31, 0x3010
	ctx.r[10].s64 = ctx.r[31].s64 + 12304;
	// 82519548: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8251954C: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82519550: 390A0030  addi r8, r10, 0x30
	ctx.r[8].s64 = ctx.r[10].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825195E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825195E0 size=376
    let mut pc: u32 = 0x825195E0;
    'dispatch: loop {
        match pc {
            0x825195E0 => {
    //   block [0x825195E0..0x82519758)
	// 825195E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825195E4: 4801BAC1  bl 0x825350a4
	ctx.lr = 0x825195E8;
	sub_82535080(ctx, base);
	// 825195E8: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825195EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825195F0: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 825195F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825195F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825195FC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82519600: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519604: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82519608: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8251960C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82519610: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82519614: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519618: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 8251961C: 409A006C  bne cr6, 0x82519688
	if !ctx.cr[6].eq {
	pc = 0x82519688; continue 'dispatch;
	}
	// 82519620: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519624: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82519628: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251962C: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82519630: 40990024  ble cr6, 0x82519654
	if !ctx.cr[6].gt {
	pc = 0x82519654; continue 'dispatch;
	}
	// 82519634: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82519638: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251963C: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82519640: 409A0108  bne cr6, 0x82519748
	if !ctx.cr[6].eq {
	pc = 0x82519748; continue 'dispatch;
	}
	// 82519644: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82519648: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8251964C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82519650: 4198FFE8  blt cr6, 0x82519638
	if ctx.cr[6].lt {
	pc = 0x82519638; continue 'dispatch;
	}
	// 82519654: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519658: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 8251965C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82519660: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82519664: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251966C: 4E800421  bctrl
	ctx.lr = 0x82519670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519670: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519674: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82519678: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 8251967C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82519680: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82519684: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82519688: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251968C: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519690: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82519694: 409A0080  bne cr6, 0x82519714
	if !ctx.cr[6].eq {
	pc = 0x82519714; continue 'dispatch;
	}
	// 82519698: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251969C: 897F0009  lbz r11, 9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 825196A0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 825196A4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825196A8: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 825196AC: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 825196B0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825196B4: 7FCAF8AE  lbzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 825196B8: 40980028  bge cr6, 0x825196e0
	if !ctx.cr[6].lt {
	pc = 0x825196E0; continue 'dispatch;
	}
	// 825196BC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825196C0: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 825196C4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825196C8: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 825196CC: 409A007C  bne cr6, 0x82519748
	if !ctx.cr[6].eq {
	pc = 0x82519748; continue 'dispatch;
	}
	// 825196D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825196D4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 825196D8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825196DC: 4198FFE8  blt cr6, 0x825196c4
	if ctx.cr[6].lt {
	pc = 0x825196C4; continue 'dispatch;
	}
	// 825196E0: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 825196E4: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 825196E8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825196EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825196F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825196F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825196F8: 4E800421  bctrl
	ctx.lr = 0x825196FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825196FC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519700: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82519704: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 82519708: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8251970C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82519710: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82519714: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519718: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 8251971C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82519720: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82519724: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82519728: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251972C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519730: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82519734: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251973C: 4E800421  bctrl
	ctx.lr = 0x82519740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519740: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82519744: 4801B9B0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 82519748: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8251974C: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 82519750: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82519754: 4801B9A0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519758 size=412
    let mut pc: u32 = 0x82519758;
    'dispatch: loop {
        match pc {
            0x82519758 => {
    //   block [0x82519758..0x825198F4)
	// 82519758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251975C: 4801B949  bl 0x825350a4
	ctx.lr = 0x82519760;
	sub_82535080(ctx, base);
	// 82519760: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82519764: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519768: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251976C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82519770: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 82519774: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82519778: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8251977C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82519780: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519784: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82519788: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 8251978C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82519790: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82519794: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519798: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 8251979C: 409A006C  bne cr6, 0x82519808
	if !ctx.cr[6].eq {
	pc = 0x82519808; continue 'dispatch;
	}
	// 825197A0: 891F0008  lbz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825197A4: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 825197A8: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825197AC: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 825197B0: 40990024  ble cr6, 0x825197d4
	if !ctx.cr[6].gt {
	pc = 0x825197D4; continue 'dispatch;
	}
	// 825197B4: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 825197B8: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825197BC: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825197C0: 409A0120  bne cr6, 0x825198e0
	if !ctx.cr[6].eq {
	pc = 0x825198E0; continue 'dispatch;
	}
	// 825197C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825197C8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 825197CC: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825197D0: 4198FFE8  blt cr6, 0x825197b8
	if ctx.cr[6].lt {
	pc = 0x825197B8; continue 'dispatch;
	}
	// 825197D4: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 825197D8: 38690014  addi r3, r9, 0x14
	ctx.r[3].s64 = ctx.r[9].s64 + 20;
	// 825197DC: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 825197E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825197E4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825197E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825197EC: 4E800421  bctrl
	ctx.lr = 0x825197F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825197F0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825197F4: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 825197F8: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 825197FC: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82519800: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82519804: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82519808: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251980C: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519810: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 82519814: 409A0080  bne cr6, 0x82519894
	if !ctx.cr[6].eq {
	pc = 0x82519894; continue 'dispatch;
	}
	// 82519818: 895F0008  lbz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251981C: 897F0009  lbz r11, 9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82519820: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82519824: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82519828: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8251982C: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82519830: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82519834: 7FCAF8AE  lbzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82519838: 40980028  bge cr6, 0x82519860
	if !ctx.cr[6].lt {
	pc = 0x82519860; continue 'dispatch;
	}
	// 8251983C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82519840: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82519844: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519848: 7F07F000  cmpw cr6, r7, r30
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8251984C: 409A0094  bne cr6, 0x825198e0
	if !ctx.cr[6].eq {
	pc = 0x825198E0; continue 'dispatch;
	}
	// 82519850: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82519854: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82519858: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8251985C: 4198FFE8  blt cr6, 0x82519844
	if ctx.cr[6].lt {
	pc = 0x82519844; continue 'dispatch;
	}
	// 82519860: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519864: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 82519868: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8251986C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82519870: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519878: 4E800421  bctrl
	ctx.lr = 0x8251987C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251987C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519880: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82519884: 3B810060  addi r28, r1, 0x60
	ctx.r[28].s64 = ctx.r[1].s64 + 96;
	// 82519888: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8251988C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82519890: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82519894: 8161033C  lwz r11, 0x33c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(828 as u32) ) } as u64;
	// 82519898: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8251989C: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 825198A0: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 825198A4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825198A8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 825198AC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825198B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825198B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825198B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825198BC: 81610334  lwz r11, 0x334(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(820 as u32) ) } as u64;
	// 825198C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825198C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825198C8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825198CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825198D0: 4E800421  bctrl
	ctx.lr = 0x825198D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825198D4: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 825198D8: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 825198DC: 4801B818  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 825198E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825198E4: 9AFB0010  stb r23, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[23].u8 ) };
	// 825198E8: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 825198EC: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 825198F0: 4801B804  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825198F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825198F8 size=76
    let mut pc: u32 = 0x825198F8;
    'dispatch: loop {
        match pc {
            0x825198F8 => {
    //   block [0x825198F8..0x82519944)
	// 825198F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825198FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519904: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82519908: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251990C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82519910: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82519914: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519918: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251991C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82519920: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519924: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82519928: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251992C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82519930: 4BFFEA99  bl 0x825183c8
	ctx.lr = 0x82519934;
	sub_825183C8(ctx, base);
	// 82519934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251993C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519948 size=80
    let mut pc: u32 = 0x82519948;
    'dispatch: loop {
        match pc {
            0x82519948 => {
    //   block [0x82519948..0x82519998)
	// 82519948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251994C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519954: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82519958: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251995C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519960: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82519964: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82519968: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251996C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519970: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519974: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82519978: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251997C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82519980: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82519984: 4BFFE7DD  bl 0x82518160
	ctx.lr = 0x82519988;
	sub_82518160(ctx, base);
	// 82519988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251998C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519998 size=232
    let mut pc: u32 = 0x82519998;
    'dispatch: loop {
        match pc {
            0x82519998 => {
    //   block [0x82519998..0x82519A80)
	// 82519998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825199A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825199A4: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825199A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825199AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825199B0: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 825199B4: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825199B8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825199BC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 825199C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825199C4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825199C8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825199CC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825199D0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825199D4: 4200FFF0  bdnz 0x825199c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825199C4; continue 'dispatch;
	}
	// 825199D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825199DC: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 825199E0: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 825199E4: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825199E8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825199EC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519A80 size=112
    let mut pc: u32 = 0x82519A80;
    'dispatch: loop {
        match pc {
            0x82519A80 => {
    //   block [0x82519A80..0x82519AF0)
	// 82519A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519A8C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519A94: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82519A98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82519A9C: 409A0040  bne cr6, 0x82519adc
	if !ctx.cr[6].eq {
	pc = 0x82519ADC; continue 'dispatch;
	}
	// 82519AA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519AA4: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519AA8: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82519AAC: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82519AB0: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82519AB4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82519AB8: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82519ABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519AC0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519AC8: 4E800421  bctrl
	ctx.lr = 0x82519ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519ACC: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82519AD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519AD4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82519AD8: 480A5059  bl 0x825beb30
	ctx.lr = 0x82519ADC;
	sub_825BEB30(ctx, base);
	// 82519ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519AF0 size=68
    let mut pc: u32 = 0x82519AF0;
    'dispatch: loop {
        match pc {
            0x82519AF0 => {
    //   block [0x82519AF0..0x82519B34)
	// 82519AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519AFC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519B00: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82519B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82519B08: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 82519B0C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82519B10: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82519B14: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519B18: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82519B1C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82519B20: 4BFFE8A9  bl 0x825183c8
	ctx.lr = 0x82519B24;
	sub_825183C8(ctx, base);
	// 82519B24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519B38 size=72
    let mut pc: u32 = 0x82519B38;
    'dispatch: loop {
        match pc {
            0x82519B38 => {
    //   block [0x82519B38..0x82519B80)
	// 82519B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519B44: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519B48: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82519B4C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82519B50: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 82519B54: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82519B58: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82519B5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519B60: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519B64: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82519B68: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82519B6C: 4BFFE5F5  bl 0x82518160
	ctx.lr = 0x82519B70;
	sub_82518160(ctx, base);
	// 82519B70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519B80 size=228
    let mut pc: u32 = 0x82519B80;
    'dispatch: loop {
        match pc {
            0x82519B80 => {
    //   block [0x82519B80..0x82519C64)
	// 82519B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519B88: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519B8C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82519B90: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82519B94: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82519B98: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82519B9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82519BA0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82519BA4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82519BA8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82519BAC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82519BB0: 4200FFF0  bdnz 0x82519ba0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82519BA0; continue 'dispatch;
	}
	// 82519BB4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82519BB8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82519BBC: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82519BC0: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82519BC4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82519BC8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519C68 size=208
    let mut pc: u32 = 0x82519C68;
    'dispatch: loop {
        match pc {
            0x82519C68 => {
    //   block [0x82519C68..0x82519D38)
	// 82519C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519C78: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82519C7C: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519C84: 39443010  addi r10, r4, 0x3010
	ctx.r[10].s64 = ctx.r[4].s64 + 12304;
	// 82519C88: 397F1010  addi r11, r31, 0x1010
	ctx.r[11].s64 = ctx.r[31].s64 + 4112;
	// 82519C8C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82519C90: C1A98CB4  lfs f13, -0x734c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82519C94: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82519C98: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82519C9C: 38EA0040  addi r7, r10, 0x40
	ctx.r[7].s64 = ctx.r[10].s64 + 64;
	// 82519CA0: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82519CA4: D1AB0020  stfs f13, 0x20(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82519CA8: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82519CAC: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82519CB0: C0091FF8  lfs f0, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519CB4: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82519CB8: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82519CBC: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519D38 size=176
    let mut pc: u32 = 0x82519D38;
    'dispatch: loop {
        match pc {
            0x82519D38 => {
    //   block [0x82519D38..0x82519DE8)
	// 82519D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82519D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519D48: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82519D4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519D50: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82519D54: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82519D58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82519D5C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82519D60: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519D64: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82519D68: 4BFFEAE1  bl 0x82518848
	ctx.lr = 0x82519D6C;
	sub_82518848(ctx, base);
	// 82519D6C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519D70: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82519D74: 40980044  bge cr6, 0x82519db8
	if !ctx.cr[6].lt {
	pc = 0x82519DB8; continue 'dispatch;
	}
	// 82519D78: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519D7C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82519D80: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519DE8 size=84
    let mut pc: u32 = 0x82519DE8;
    'dispatch: loop {
        match pc {
            0x82519DE8 => {
    //   block [0x82519DE8..0x82519E3C)
	// 82519DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519DFC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82519E00: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82519E04: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519E08: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82519E0C: 480A3CB5  bl 0x825bdac0
	ctx.lr = 0x82519E10;
	sub_825BDAC0(ctx, base);
	// 82519E10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519E14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82519E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82519E1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519E24: 4E800421  bctrl
	ctx.lr = 0x82519E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82519E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519E40 size=8
    let mut pc: u32 = 0x82519E40;
    'dispatch: loop {
        match pc {
            0x82519E40 => {
    //   block [0x82519E40..0x82519E48)
	// 82519E40: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82519E44: 480A38DC  b 0x825bd720
	sub_825BD720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519E48 size=8
    let mut pc: u32 = 0x82519E48;
    'dispatch: loop {
        match pc {
            0x82519E48 => {
    //   block [0x82519E48..0x82519E50)
	// 82519E48: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82519E4C: 480A3A24  b 0x825bd870
	sub_825BD870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519E50 size=16
    let mut pc: u32 = 0x82519E50;
    'dispatch: loop {
        match pc {
            0x82519E50 => {
    //   block [0x82519E50..0x82519E60)
	// 82519E50: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82519E54: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82519E58: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82519E5C: 4BFBC7B4  b 0x824d6610
	sub_824D6610(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519E60 size=100
    let mut pc: u32 = 0x82519E60;
    'dispatch: loop {
        match pc {
            0x82519E60 => {
    //   block [0x82519E60..0x82519EC4)
	// 82519E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82519E6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82519E74: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519E78: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82519E7C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82519E80: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82519E84: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82519E88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519E8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82519E90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519E94: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82519E98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82519E9C: 4E800421  bctrl
	ctx.lr = 0x82519EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82519EA0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82519EA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82519EA8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82519EAC: 480A4C85  bl 0x825beb30
	ctx.lr = 0x82519EB0;
	sub_825BEB30(ctx, base);
	// 82519EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82519EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82519EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519EC8 size=80
    let mut pc: u32 = 0x82519EC8;
    'dispatch: loop {
        match pc {
            0x82519EC8 => {
    //   block [0x82519EC8..0x82519F18)
	// 82519EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519ED4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82519ED8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82519EDC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519EE0: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82519EE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82519EE8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82519EEC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519EF0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519EF4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82519EF8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82519EFC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82519F00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82519F04: 4800B4AD  bl 0x825253b0
	ctx.lr = 0x82519F08;
	sub_825253B0(ctx, base);
	// 82519F08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519F18 size=72
    let mut pc: u32 = 0x82519F18;
    'dispatch: loop {
        match pc {
            0x82519F18 => {
    //   block [0x82519F18..0x82519F60)
	// 82519F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519F24: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82519F28: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82519F2C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82519F30: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 82519F34: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82519F38: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82519F3C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519F40: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519F44: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82519F48: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82519F4C: 4800B465  bl 0x825253b0
	ctx.lr = 0x82519F50;
	sub_825253B0(ctx, base);
	// 82519F50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82519F60 size=4
    let mut pc: u32 = 0x82519F60;
    'dispatch: loop {
        match pc {
            0x82519F60 => {
    //   block [0x82519F60..0x82519F64)
	// 82519F60: 4BFFF420  b 0x82519380
	sub_82519380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82519F68 size=76
    let mut pc: u32 = 0x82519F68;
    'dispatch: loop {
        match pc {
            0x82519F68 => {
    //   block [0x82519F68..0x82519FB4)
	// 82519F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82519F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519F74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82519F78: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82519F7C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82519F80: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82519F84: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82519F88: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82519F8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82519F90: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519F94: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82519F98: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82519F9C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82519FA0: 4800B9A9  bl 0x82525948
	ctx.lr = 0x82519FA4;
	sub_82525948(ctx, base);
	// 82519FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82519FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82519FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82519FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82519FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82519FB8 size=464
    let mut pc: u32 = 0x82519FB8;
    'dispatch: loop {
        match pc {
            0x82519FB8 => {
    //   block [0x82519FB8..0x8251A188)
	// 82519FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82519FBC: 4801B0F1  bl 0x825350ac
	ctx.lr = 0x82519FC0;
	sub_82535080(ctx, base);
	// 82519FC0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82519FC4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82519FC8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519FCC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82519FD0: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82519FD4: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82519FD8: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82519FDC: 90C10068  stw r6, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 82519FE0: C0060058  lfs f0, 0x58(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82519FE4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82519FE8: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519FEC: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82519FF0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82519FF4: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82519FF8: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82519FFC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251A000: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8251A004: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251A008: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8251A00C: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251A188 size=24
    let mut pc: u32 = 0x8251A188;
    'dispatch: loop {
        match pc {
            0x8251A188 => {
    //   block [0x8251A188..0x8251A1A0)
	// 8251A188: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251A18C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251A190: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251A194: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8251A198: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8251A19C: 4BFFF1E4  b 0x82519380
	sub_82519380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A1A0 size=68
    let mut pc: u32 = 0x8251A1A0;
    'dispatch: loop {
        match pc {
            0x8251A1A0 => {
    //   block [0x8251A1A0..0x8251A1E4)
	// 8251A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A1AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A1B0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251A1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A1B8: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 8251A1BC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8251A1C0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251A1C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251A1C8: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8251A1CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251A1D0: 4800B779  bl 0x82525948
	ctx.lr = 0x8251A1D4;
	sub_82525948(ctx, base);
	// 8251A1D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A1E8 size=104
    let mut pc: u32 = 0x8251A1E8;
    'dispatch: loop {
        match pc {
            0x8251A1E8 => {
    //   block [0x8251A1E8..0x8251A250)
	// 8251A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A1F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A1F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A1F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A1FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A200: 396B507C  addi r11, r11, 0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + 20604;
	// 8251A204: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8251A208: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8251A20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A210: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 8251A214: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8251A218: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A21C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8251A220: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251A224: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251A228: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8251A22C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A230: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251A234: 480A3A9D  bl 0x825bdcd0
	ctx.lr = 0x8251A238;
	sub_825BDCD0(ctx, base);
	// 8251A238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A23C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251A240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A250 size=128
    let mut pc: u32 = 0x8251A250;
    'dispatch: loop {
        match pc {
            0x8251A250 => {
    //   block [0x8251A250..0x8251A2D0)
	// 8251A250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A254: 4801AE69  bl 0x825350bc
	ctx.lr = 0x8251A258;
	sub_82535080(ctx, base);
	// 8251A258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A25C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A260: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251A264: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251A268: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251A26C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8251A270: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251A274: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251A278: 4BF49DC1  bl 0x82464038
	ctx.lr = 0x8251A27C;
	sub_82464038(ctx, base);
	// 8251A27C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A284: 396B507C  addi r11, r11, 0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + 20604;
	// 8251A288: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 8251A28C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251A290: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8251A294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A298: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8251A29C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 8251A2A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A2A4: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8251A2A8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8251A2AC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251A2B0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251A2B4: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8251A2B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A2BC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251A2C0: 480A3A11  bl 0x825bdcd0
	ctx.lr = 0x8251A2C4;
	sub_825BDCD0(ctx, base);
	// 8251A2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A2C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A2CC: 4801AE40  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251A2D0 size=68
    let mut pc: u32 = 0x8251A2D0;
    'dispatch: loop {
        match pc {
            0x8251A2D0 => {
    //   block [0x8251A2D0..0x8251A314)
	// 8251A2D0: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251A2D4: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251A2D8: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251A2DC: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251A2E0: 3908A250  addi r8, r8, -0x5db0
	ctx.r[8].s64 = ctx.r[8].s64 + -23984;
	// 8251A2E4: 392992E0  addi r9, r9, -0x6d20
	ctx.r[9].s64 = ctx.r[9].s64 + -27936;
	// 8251A2E8: 394A9330  addi r10, r10, -0x6cd0
	ctx.r[10].s64 = ctx.r[10].s64 + -27856;
	// 8251A2EC: 396B9380  addi r11, r11, -0x6c80
	ctx.r[11].s64 = ctx.r[11].s64 + -27776;
	// 8251A2F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8251A2F4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251A2F8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251A2FC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251A300: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251A304: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251A308: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 8251A30C: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 8251A310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A318 size=140
    let mut pc: u32 = 0x8251A318;
    'dispatch: loop {
        match pc {
            0x8251A318 => {
    //   block [0x8251A318..0x8251A3A4)
	// 8251A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A31C: 4801ADA1  bl 0x825350bc
	ctx.lr = 0x8251A320;
	sub_82535080(ctx, base);
	// 8251A320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A324: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A328: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251A32C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251A330: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251A334: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8251A338: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251A33C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251A340: 4BF49CF9  bl 0x82464038
	ctx.lr = 0x8251A344;
	sub_82464038(ctx, base);
	// 8251A344: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A34C: 396B507C  addi r11, r11, 0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + 20604;
	// 8251A350: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 8251A354: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251A358: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8251A35C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A360: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8251A364: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 8251A368: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A36C: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8251A370: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8251A374: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251A378: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251A37C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8251A380: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A384: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251A388: 480A3949  bl 0x825bdcd0
	ctx.lr = 0x8251A38C;
	sub_825BDCD0(ctx, base);
	// 8251A38C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A394: 396B50BC  addi r11, r11, 0x50bc
	ctx.r[11].s64 = ctx.r[11].s64 + 20668;
	// 8251A398: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A39C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A3A0: 4801AD6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251A3A8 size=64
    let mut pc: u32 = 0x8251A3A8;
    'dispatch: loop {
        match pc {
            0x8251A3A8 => {
    //   block [0x8251A3A8..0x8251A3E8)
	// 8251A3A8: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251A3AC: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251A3B0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251A3B4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251A3B8: 38EB5678  addi r7, r11, 0x5678
	ctx.r[7].s64 = ctx.r[11].s64 + 22136;
	// 8251A3BC: 3908A318  addi r8, r8, -0x5ce8
	ctx.r[8].s64 = ctx.r[8].s64 + -23784;
	// 8251A3C0: 39295948  addi r9, r9, 0x5948
	ctx.r[9].s64 = ctx.r[9].s64 + 22856;
	// 8251A3C4: 394A53B0  addi r10, r10, 0x53b0
	ctx.r[10].s64 = ctx.r[10].s64 + 21424;
	// 8251A3C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251A3CC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8251A3D0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251A3D4: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251A3D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251A3DC: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8251A3E0: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8251A3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A3E8 size=204
    let mut pc: u32 = 0x8251A3E8;
    'dispatch: loop {
        match pc {
            0x8251A3E8 => {
    //   block [0x8251A3E8..0x8251A4B4)
	// 8251A3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A3F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A3FC: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251A400: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251A404: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251A408: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251A40C: 3908A318  addi r8, r8, -0x5ce8
	ctx.r[8].s64 = ctx.r[8].s64 + -23784;
	// 8251A410: 39295948  addi r9, r9, 0x5948
	ctx.r[9].s64 = ctx.r[9].s64 + 22856;
	// 8251A414: 394A53B0  addi r10, r10, 0x53b0
	ctx.r[10].s64 = ctx.r[10].s64 + 21424;
	// 8251A418: 396B5678  addi r11, r11, 0x5678
	ctx.r[11].s64 = ctx.r[11].s64 + 22136;
	// 8251A41C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251A420: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8251A424: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251A428: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8251A42C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251A430: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A434: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251A438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A43C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251A440: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 8251A444: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251A448: 4BFE5941  bl 0x824ffd88
	ctx.lr = 0x8251A44C;
	sub_824FFD88(ctx, base);
	// 8251A44C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251A450: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251A454: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251A458: 396B9380  addi r11, r11, -0x6c80
	ctx.r[11].s64 = ctx.r[11].s64 + -27776;
	// 8251A45C: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251A460: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251A464: 3908A250  addi r8, r8, -0x5db0
	ctx.r[8].s64 = ctx.r[8].s64 + -23984;
	// 8251A468: 392992E0  addi r9, r9, -0x6d20
	ctx.r[9].s64 = ctx.r[9].s64 + -27936;
	// 8251A46C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251A470: 394A9330  addi r10, r10, -0x6cd0
	ctx.r[10].s64 = ctx.r[10].s64 + -27856;
	// 8251A474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251A478: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8251A47C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8251A480: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251A484: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251A488: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251A48C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A490: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251A494: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 8251A498: 4BFE58F1  bl 0x824ffd88
	ctx.lr = 0x8251A49C;
	sub_824FFD88(ctx, base);
	// 8251A49C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251A4A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A4A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A4A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A4AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A4B8 size=100
    let mut pc: u32 = 0x8251A4B8;
    'dispatch: loop {
        match pc {
            0x8251A4B8 => {
    //   block [0x8251A4B8..0x8251A51C)
	// 8251A4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A4C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A4C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A4C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A4CC: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A4D0: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8251A4D4: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8251A4D8: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 8251A4DC: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 8251A4E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A4E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8251A4E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A4EC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251A4F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251A4F4: 4E800421  bctrl
	ctx.lr = 0x8251A4F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251A4F8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8251A4FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251A500: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8251A504: 480A462D  bl 0x825beb30
	ctx.lr = 0x8251A508;
	sub_825BEB30(ctx, base);
	// 8251A508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251A50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251A520 size=92
    let mut pc: u32 = 0x8251A520;
    'dispatch: loop {
        match pc {
            0x8251A520 => {
    //   block [0x8251A520..0x8251A57C)
	// 8251A520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A52C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A530: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251A534: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8251A538: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 8251A53C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251A540: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251A544: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251A548: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8251A54C: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251A550: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251A554: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251A558: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8251A55C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251A560: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8251A564: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8251A568: 4800AE49  bl 0x825253b0
	ctx.lr = 0x8251A56C;
	sub_825253B0(ctx, base);
	// 8251A56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A580 size=88
    let mut pc: u32 = 0x8251A580;
    'dispatch: loop {
        match pc {
            0x8251A580 => {
    //   block [0x8251A580..0x8251A5D8)
	// 8251A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A58C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A590: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251A594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251A598: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 8251A59C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251A5A0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251A5A4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251A5A8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8251A5AC: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8251A5B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251A5B4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8251A5B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251A5BC: 99410064  stb r10, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u8 ) };
	// 8251A5C0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8251A5C4: 4800B385  bl 0x82525948
	ctx.lr = 0x8251A5C8;
	sub_82525948(ctx, base);
	// 8251A5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251A5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251A5D8 size=176
    let mut pc: u32 = 0x8251A5D8;
    'dispatch: loop {
        match pc {
            0x8251A5D8 => {
    //   block [0x8251A5D8..0x8251A688)
	// 8251A5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A5E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A5E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A5E8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251A5EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A5F0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8251A5F4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251A5F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251A5FC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251A600: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A604: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251A608: 4BFFF9B1  bl 0x82519fb8
	ctx.lr = 0x8251A60C;
	sub_82519FB8(ctx, base);
	// 8251A60C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A610: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251A614: 40980044  bge cr6, 0x8251a658
	if !ctx.cr[6].lt {
	pc = 0x8251A658; continue 'dispatch;
	}
	// 8251A618: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251A61C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 8251A620: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A688 size=352
    let mut pc: u32 = 0x8251A688;
    'dispatch: loop {
        match pc {
            0x8251A688 => {
    //   block [0x8251A688..0x8251A7E8)
	// 8251A688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A694: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A698: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251A69C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8251A6A0: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8251A6A4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8251A6A8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8251A6AC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8251A6B0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251A6B4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8251A6B8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8251A6BC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8251A6C0: 4200FFF0  bdnz 0x8251a6b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251A6B0; continue 'dispatch;
	}
	// 8251A6C4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251A6C8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 8251A6CC: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 8251A6D0: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 8251A6D4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251A6D8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A7E8 size=152
    let mut pc: u32 = 0x8251A7E8;
    'dispatch: loop {
        match pc {
            0x8251A7E8 => {
    //   block [0x8251A7E8..0x8251A880)
	// 8251A7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251A7F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251A7F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251A7F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A7FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A800: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251A804: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251A808: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8251A80C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251A810: 409A0020  bne cr6, 0x8251a830
	if !ctx.cr[6].eq {
	pc = 0x8251A830; continue 'dispatch;
	}
	// 8251A814: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A818: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8251A81C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8251A820: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251A824: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8251A828: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251A82C: 4BF4988D  bl 0x824640b8
	ctx.lr = 0x8251A830;
	sub_824640B8(ctx, base);
	// 8251A830: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251A834: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8251A838: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8251A83C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251A840: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251A844: 419A0020  beq cr6, 0x8251a864
	if ctx.cr[6].eq {
	pc = 0x8251A864; continue 'dispatch;
	}
	// 8251A848: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A84C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251A850: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 8251A854: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251A85C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251A860: 4BF49859  bl 0x824640b8
	ctx.lr = 0x8251A864;
	sub_824640B8(ctx, base);
	// 8251A864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251A86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251A870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251A874: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251A878: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251A87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251A880 size=128
    let mut pc: u32 = 0x8251A880;
    'dispatch: loop {
        match pc {
            0x8251A880 => {
    //   block [0x8251A880..0x8251A900)
	// 8251A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A884: 4801A835  bl 0x825350b8
	ctx.lr = 0x8251A888;
	sub_82535080(ctx, base);
	// 8251A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251A890: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251A894: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251A898: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A89C: 40990044  ble cr6, 0x8251a8e0
	if !ctx.cr[6].gt {
	pc = 0x8251A8E0; continue 'dispatch;
	}
	// 8251A8A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251A8A4: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8251A8A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251A8AC: 7C8BF22E  lhzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8251A8B0: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 8251A8B4: 419A001C  beq cr6, 0x8251a8d0
	if ctx.cr[6].eq {
	pc = 0x8251A8D0; continue 'dispatch;
	}
	// 8251A8B8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A8BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251A8C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A8C4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251A8C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251A8CC: 4E800421  bctrl
	ctx.lr = 0x8251A8D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251A8D0: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8251A8D4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8251A8D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251A8DC: 409AFFCC  bne cr6, 0x8251a8a8
	if !ctx.cr[6].eq {
	pc = 0x8251A8A8; continue 'dispatch;
	}
	// 8251A8E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A8E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251A8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251A8EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A8F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251A8F4: 4E800421  bctrl
	ctx.lr = 0x8251A8F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251A8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251A8FC: 4801A80C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251A900 size=640
    let mut pc: u32 = 0x8251A900;
    'dispatch: loop {
        match pc {
            0x8251A900 => {
    //   block [0x8251A900..0x8251AB80)
	// 8251A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251A904: 4801A789  bl 0x8253508c
	ctx.lr = 0x8251A908;
	sub_82535080(ctx, base);
	// 8251A908: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 8251A90C: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251A910: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A914: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 8251A918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251A91C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8251A920: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8251A924: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 8251A928: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251A92C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251A930: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251A934: 40980020  bge cr6, 0x8251a954
	if !ctx.cr[6].lt {
	pc = 0x8251A954; continue 'dispatch;
	}
	// 8251A938: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251A93C: 39295110  addi r9, r9, 0x5110
	ctx.r[9].s64 = ctx.r[9].s64 + 20752;
	// 8251A940: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251A944: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251A948: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8251A94C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251A950: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251A954: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8251A958: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A95C: 82DC0000  lwz r22, 0(r28)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251A960: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A964: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251A968: 4808E7F1  bl 0x825a9158
	ctx.lr = 0x8251A96C;
	sub_825A9158(ctx, base);
	// 8251A96C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8251A970: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251A974: 83FB0010  lwz r31, 0x10(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251A978: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 8251A97C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251A980: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 8251A984: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 8251A988: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 8251A98C: 409901B8  ble cr6, 0x8251ab44
	if !ctx.cr[6].gt {
	pc = 0x8251AB44; continue 'dispatch;
	}
	// 8251A990: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251A994: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 8251A998: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8251A99C: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 8251A9A0: 3B4B9F60  addi r26, r11, -0x60a0
	ctx.r[26].s64 = ctx.r[11].s64 + -24736;
	// 8251A9A4: C3EA1850  lfs f31, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251A9A8: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 8251A9AC: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 8251A9B0: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 8251A9B4: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 8251A9B8: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 8251A9BC: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8251A9C0: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 8251A9C4: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 8251A9C8: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 8251A9CC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8251A9D0: 92E100E0  stw r23, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[23].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251AB80 size=640
    let mut pc: u32 = 0x8251AB80;
    'dispatch: loop {
        match pc {
            0x8251AB80 => {
    //   block [0x8251AB80..0x8251AE00)
	// 8251AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AB84: 4801A509  bl 0x8253508c
	ctx.lr = 0x8251AB88;
	sub_82535080(ctx, base);
	// 8251AB88: DBE1FF78  stfd f31, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 8251AB8C: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AB90: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AB94: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 8251AB98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251AB9C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251ABA0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8251ABA4: 7D52882E  lwzx r10, r18, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 8251ABA8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251ABAC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251ABB0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251ABB4: 40980020  bge cr6, 0x8251abd4
	if !ctx.cr[6].lt {
	pc = 0x8251ABD4; continue 'dispatch;
	}
	// 8251ABB8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251ABBC: 39295110  addi r9, r9, 0x5110
	ctx.r[9].s64 = ctx.r[9].s64 + 20752;
	// 8251ABC0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251ABC4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251ABC8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8251ABCC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251ABD0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251ABD4: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8251ABD8: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251ABDC: 82DC0000  lwz r22, 0(r28)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251ABE0: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251ABE4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251ABE8: 4808E571  bl 0x825a9158
	ctx.lr = 0x8251ABEC;
	sub_825A9158(ctx, base);
	// 8251ABEC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8251ABF0: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251ABF4: 83FB0010  lwz r31, 0x10(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251ABF8: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 8251ABFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251AC00: 938100B4  stw r28, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 8251AC04: 92A10080  stw r21, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 8251AC08: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 8251AC0C: 409901B8  ble cr6, 0x8251adc4
	if !ctx.cr[6].gt {
	pc = 0x8251ADC4; continue 'dispatch;
	}
	// 8251AC10: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251AC14: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 8251AC18: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8251AC1C: 3A80FFFF  li r20, -1
	ctx.r[20].s64 = -1;
	// 8251AC20: 3B4B9F60  addi r26, r11, -0x60a0
	ctx.r[26].s64 = ctx.r[11].s64 + -24736;
	// 8251AC24: C3EA1850  lfs f31, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251AC28: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 8251AC2C: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 8251AC30: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 8251AC34: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 8251AC38: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 8251AC3C: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 8251AC40: 928100D4  stw r20, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 8251AC44: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 8251AC48: 92A10100  stw r21, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[21].u32 ) };
	// 8251AC4C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8251AC50: 92E100E0  stw r23, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[23].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251AE00 size=436
    let mut pc: u32 = 0x8251AE00;
    'dispatch: loop {
        match pc {
            0x8251AE00 => {
    //   block [0x8251AE00..0x8251AFB4)
	// 8251AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AE04: 4801A2A1  bl 0x825350a4
	ctx.lr = 0x8251AE08;
	sub_82535080(ctx, base);
	// 8251AE08: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AE0C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE10: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8251AE14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251AE18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251AE1C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8251AE20: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8251AE24: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251AE28: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251AE2C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251AE30: 40980020  bge cr6, 0x8251ae50
	if !ctx.cr[6].lt {
	pc = 0x8251AE50; continue 'dispatch;
	}
	// 8251AE34: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251AE38: 39295120  addi r9, r9, 0x5120
	ctx.r[9].s64 = ctx.r[9].s64 + 20768;
	// 8251AE3C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251AE40: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251AE44: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8251AE48: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251AE4C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251AE50: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8251AE54: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE58: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251AE5C: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251AE60: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251AE64: 4808E2F5  bl 0x825a9158
	ctx.lr = 0x8251AE68;
	sub_825A9158(ctx, base);
	// 8251AE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251AE6C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251AE70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251AE74: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251AE78: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8251AE7C: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8251AE80: C00A1850  lfs f0, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251AE84: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8251AE88: 91610110  stw r11, 0x110(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8251AE8C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8251AE90: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8251AE94: D00100E0  stfs f0, 0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8251AE98: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 8251AE9C: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 8251AEA0: 419A00DC  beq cr6, 0x8251af7c
	if ctx.cr[6].eq {
	pc = 0x8251AF7C; continue 'dispatch;
	}
	// 8251AEA4: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 8251AEA8: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251AEAC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8251AEB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251AEB4: 409A00C8  bne cr6, 0x8251af7c
	if !ctx.cr[6].eq {
	pc = 0x8251AF7C; continue 'dispatch;
	}
	// 8251AEB8: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251AFB8 size=20
    let mut pc: u32 = 0x8251AFB8;
    'dispatch: loop {
        match pc {
            0x8251AFB8 => {
    //   block [0x8251AFB8..0x8251AFCC)
	// 8251AFB8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251AFBC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251AFC0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251AFC4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8251AFC8: 4BFFFE38  b 0x8251ae00
	sub_8251AE00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251AFD0 size=192
    let mut pc: u32 = 0x8251AFD0;
    'dispatch: loop {
        match pc {
            0x8251AFD0 => {
    //   block [0x8251AFD0..0x8251B090)
	// 8251AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251AFD4: 4801A0E9  bl 0x825350bc
	ctx.lr = 0x8251AFD8;
	sub_82535080(ctx, base);
	// 8251AFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251AFDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251AFE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251AFE4: 396B513C  addi r11, r11, 0x513c
	ctx.r[11].s64 = ctx.r[11].s64 + 20796;
	// 8251AFE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8251AFEC: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 8251AFF0: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8251AFF4: 90FD0008  stw r7, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8251AFF8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251AFFC: 61290004  ori r9, r9, 4
	ctx.r[9].u64 = ctx.r[9].u64 | 4;
	// 8251B000: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8251B004: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8251B008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251B00C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8251B010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B014: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251B018: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8251B01C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B020: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8251B024: 83CA0014  lwz r30, 0x14(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B028: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8251B02C: 40980024  bge cr6, 0x8251b050
	if !ctx.cr[6].lt {
	pc = 0x8251B050; continue 'dispatch;
	}
	// 8251B030: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8251B034: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8251B038: 41980008  blt cr6, 0x8251b040
	if ctx.cr[6].lt {
	pc = 0x8251B040; continue 'dispatch;
	}
	// 8251B03C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8251B040: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8251B044: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251B048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B04C: 4BF5327D  bl 0x8246e2c8
	ctx.lr = 0x8251B050;
	sub_8246E2C8(ctx, base);
	// 8251B050: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8251B054: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8251B058: 4099002C  ble cr6, 0x8251b084
	if !ctx.cr[6].gt {
	pc = 0x8251B084; continue 'dispatch;
	}
	// 8251B05C: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8251B060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251B064: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8251B068: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 8251B06C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B070: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8251B074: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251B078: 7D28532E  sthx r9, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 8251B07C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8251B080: 409AFFEC  bne cr6, 0x8251b06c
	if !ctx.cr[6].eq {
	pc = 0x8251B06C; continue 'dispatch;
	}
	// 8251B084: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251B088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B08C: 4801A080  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B090 size=88
    let mut pc: u32 = 0x8251B090;
    'dispatch: loop {
        match pc {
            0x8251B090 => {
    //   block [0x8251B090..0x8251B0E8)
	// 8251B090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B094: 4801A025  bl 0x825350b8
	ctx.lr = 0x8251B098;
	sub_82535080(ctx, base);
	// 8251B098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B09C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B0A0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251B0A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251B0A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8251B0AC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251B0B0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8251B0B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B0B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251B0BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8251B0C0: 4BF48F79  bl 0x82464038
	ctx.lr = 0x8251B0C4;
	sub_82464038(ctx, base);
	// 8251B0C4: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8251B0C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8251B0CC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8251B0D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251B0D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B0D8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8251B0DC: 4BFFFEF5  bl 0x8251afd0
	ctx.lr = 0x8251B0E0;
	sub_8251AFD0(ctx, base);
	// 8251B0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251B0E4: 4801A024  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B0E8 size=108
    let mut pc: u32 = 0x8251B0E8;
    'dispatch: loop {
        match pc {
            0x8251B0E8 => {
    //   block [0x8251B0E8..0x8251B154)
	// 8251B0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B0EC: 48019FC9  bl 0x825350b4
	ctx.lr = 0x8251B0F0;
	sub_82535080(ctx, base);
	// 8251B0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B0F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B0F8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251B0FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251B100: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8251B104: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251B108: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8251B10C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251B110: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251B114: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8251B118: 4BF48F21  bl 0x82464038
	ctx.lr = 0x8251B11C;
	sub_82464038(ctx, base);
	// 8251B11C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B120: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8251B124: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8251B128: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8251B12C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251B130: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8251B134: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8251B138: 4BFFFE99  bl 0x8251afd0
	ctx.lr = 0x8251B13C;
	sub_8251AFD0(ctx, base);
	// 8251B13C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B144: 396B5178  addi r11, r11, 0x5178
	ctx.r[11].s64 = ctx.r[11].s64 + 20856;
	// 8251B148: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251B150: 48019FB4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B158 size=204
    let mut pc: u32 = 0x8251B158;
    'dispatch: loop {
        match pc {
            0x8251B158 => {
    //   block [0x8251B158..0x8251B224)
	// 8251B158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B168: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B16C: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251B170: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251B174: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251B178: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251B17C: 3908B0E8  addi r8, r8, -0x4f18
	ctx.r[8].s64 = ctx.r[8].s64 + -20248;
	// 8251B180: 3929B228  addi r9, r9, -0x4dd8
	ctx.r[9].s64 = ctx.r[9].s64 + -19928;
	// 8251B184: 394AB278  addi r10, r10, -0x4d88
	ctx.r[10].s64 = ctx.r[10].s64 + -19848;
	// 8251B188: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251B18C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251B190: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8251B194: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251B198: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8251B19C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251B1A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251B1A4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251B1A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B1AC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251B1B0: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 8251B1B4: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251B1B8: 4BFE4BD1  bl 0x824ffd88
	ctx.lr = 0x8251B1BC;
	sub_824FFD88(ctx, base);
	// 8251B1BC: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251B1C0: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251B1C4: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251B1C8: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251B1CC: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251B1D0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251B1D4: 3908B090  addi r8, r8, -0x4f70
	ctx.r[8].s64 = ctx.r[8].s64 + -20336;
	// 8251B1D8: 3929AE00  addi r9, r9, -0x5200
	ctx.r[9].s64 = ctx.r[9].s64 + -20992;
	// 8251B1DC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251B1E0: 394AAB80  addi r10, r10, -0x5480
	ctx.r[10].s64 = ctx.r[10].s64 + -21632;
	// 8251B1E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251B1E8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251B1EC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 8251B1F0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251B1F4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251B1F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251B1FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B200: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251B204: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 8251B208: 4BFE4B81  bl 0x824ffd88
	ctx.lr = 0x8251B20C;
	sub_824FFD88(ctx, base);
	// 8251B20C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251B210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B21C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B228 size=76
    let mut pc: u32 = 0x8251B228;
    'dispatch: loop {
        match pc {
            0x8251B228 => {
    //   block [0x8251B228..0x8251B274)
	// 8251B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B234: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B238: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251B23C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251B240: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251B244: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251B248: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251B24C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251B250: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251B254: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251B258: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251B25C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8251B260: 4BFFFBA1  bl 0x8251ae00
	ctx.lr = 0x8251B264;
	sub_8251AE00(ctx, base);
	// 8251B264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251B278 size=80
    let mut pc: u32 = 0x8251B278;
    'dispatch: loop {
        match pc {
            0x8251B278 => {
    //   block [0x8251B278..0x8251B2C8)
	// 8251B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B284: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251B288: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251B28C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251B290: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251B294: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251B298: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251B29C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251B2A0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251B2A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251B2A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251B2AC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251B2B0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251B2B4: 4BFFF8CD  bl 0x8251ab80
	ctx.lr = 0x8251B2B8;
	sub_8251AB80(ctx, base);
	// 8251B2B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251B2C8 size=724
    let mut pc: u32 = 0x8251B2C8;
    'dispatch: loop {
        match pc {
            0x8251B2C8 => {
    //   block [0x8251B2C8..0x8251B59C)
	// 8251B2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B2CC: 48019DB5  bl 0x82535080
	ctx.lr = 0x8251B2D0;
	sub_82535080(ctx, base);
	// 8251B2D0: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 8251B2D4: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B2D8: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B2DC: 39C00014  li r14, 0x14
	ctx.r[14].s64 = 20;
	// 8251B2E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251B2E4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8251B2E8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251B2EC: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 8251B2F0: 7D6EF82E  lwzx r11, r14, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8251B2F4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8251B2F8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8251B2FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251B300: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251B304: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251B308: 40980020  bge cr6, 0x8251b328
	if !ctx.cr[6].lt {
	pc = 0x8251B328; continue 'dispatch;
	}
	// 8251B30C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251B310: 39295110  addi r9, r9, 0x5110
	ctx.r[9].s64 = ctx.r[9].s64 + 20752;
	// 8251B314: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251B318: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251B31C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251B320: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251B324: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8251B328: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8251B32C: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B330: 829B0000  lwz r20, 0(r27)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B334: 80B90008  lwz r5, 8(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251B338: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251B33C: 4808DE1D  bl 0x825a9158
	ctx.lr = 0x8251B340;
	sub_825A9158(ctx, base);
	// 8251B340: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 8251B344: 81780014  lwz r11, 0x14(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251B34C: 92610080  stw r19, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[19].u32 ) };
	// 8251B350: 92610084  stw r19, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[19].u32 ) };
	// 8251B354: 83580010  lwz r26, 0x10(r24)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251B358: 40990208  ble cr6, 0x8251b560
	if !ctx.cr[6].gt {
	pc = 0x8251B560; continue 'dispatch;
	}
	// 8251B35C: 7D6F5B78  mr r15, r11
	ctx.r[15].u64 = ctx.r[11].u64;
	// 8251B360: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8251B364: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8251B368: 3AEB9F60  addi r23, r11, -0x60a0
	ctx.r[23].s64 = ctx.r[11].s64 + -24736;
	// 8251B36C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251B370: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 8251B374: 3A200010  li r17, 0x10
	ctx.r[17].s64 = 16;
	// 8251B378: C3EA1850  lfs f31, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251B37C: 3A40FFFF  li r18, -1
	ctx.r[18].s64 = -1;
	// 8251B380: 3AA0FFFF  li r21, -1
	ctx.r[21].s64 = -1;
	// 8251B384: 6176FFFF  ori r22, r11, 0xffff
	ctx.r[22].u64 = ctx.r[11].u64 | 65535;
	// 8251B388: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251B5A0 size=80
    let mut pc: u32 = 0x8251B5A0;
    'dispatch: loop {
        match pc {
            0x8251B5A0 => {
    //   block [0x8251B5A0..0x8251B5F0)
	// 8251B5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B5A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B5AC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251B5B0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251B5B4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8251B5B8: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251B5BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251B5C0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251B5C4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251B5C8: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251B5CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251B5D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251B5D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251B5D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251B5DC: 4BFFF325  bl 0x8251a900
	ctx.lr = 0x8251B5E0;
	sub_8251A900(ctx, base);
	// 8251B5E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B5F0 size=68
    let mut pc: u32 = 0x8251B5F0;
    'dispatch: loop {
        match pc {
            0x8251B5F0 => {
    //   block [0x8251B5F0..0x8251B634)
	// 8251B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B5F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B5FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B600: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251B604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251B608: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 8251B60C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8251B610: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251B614: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251B618: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8251B61C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251B620: 4BFFF7E1  bl 0x8251ae00
	ctx.lr = 0x8251B624;
	sub_8251AE00(ctx, base);
	// 8251B624: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251B638 size=176
    let mut pc: u32 = 0x8251B638;
    'dispatch: loop {
        match pc {
            0x8251B638 => {
    //   block [0x8251B638..0x8251B6E8)
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
    //   block [0x8251B6E8..0x8251B780)
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
    //   block [0x8251B780..0x8251B870)
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
    //   block [0x8251B870..0x8251B8F8)
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
    //   block [0x8251B8F8..0x8251B970)
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
    //   block [0x8251B970..0x8251BA04)
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
    //   block [0x8251BEA0..0x8251C188)
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
    //   block [0x8251C188..0x8251C47C)
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
    //   block [0x8251C550..0x8251C638)
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
    //   block [0x8251C6D8..0x8251CA60)
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
    //   block [0x8251CB00..0x8251CBB0)
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
    //   block [0x8251CC30..0x8251CCA8)
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
    //   block [0x8251D150..0x8251D400)
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
    //   block [0x8251D400..0x8251D6C8)
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
    //   block [0x8251D6C8..0x8251D940)
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
    //   block [0x8251D940..0x8251DBCC)
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
    //   block [0x8251DDF8..0x8251E110)
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
    //   block [0x8251E250..0x8251E300)
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
    //   block [0x8251E378..0x8251E3F0)
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
    //   block [0x8251E3F0..0x8251E690)
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
    //   block [0x8251EA38..0x8251EC8C)
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
    //   block [0x8251EC90..0x8251EEE4)
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
    //   block [0x8251EF50..0x8251F26C)
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
    //   block [0x8251F2D8..0x8251F340)
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
    //   block [0x8251F3A8..0x8251F510)
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
    //   block [0x8251F528..0x8251F8A0)
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
    //   block [0x8251F8A0..0x8251FBB0)
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
    //   block [0x8251FBB0..0x8251FEC0)
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
    //   block [0x825200C8..0x82520178)
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
    //   block [0x825201F0..0x82520258)
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
    //   block [0x825202D8..0x82520408)
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
    //   block [0x82520408..0x82520548)
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
    //   block [0x825206C0..0x825208AC)
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
    //   block [0x825208B0..0x82520AB4)
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
    //   block [0x82520D80..0x82520FEC)
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
    //   block [0x82521090..0x82521140)
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
    //   block [0x825211A8..0x82521210)
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
    //   block [0x82521278..0x82521440)
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
    //   block [0x825216D0..0x825219D0)
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
    //   block [0x825219D0..0x82521CD0)
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
    //   block [0x82521E38..0x82521E98)
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
    //   block [0x82521E98..0x82522204)
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
    //   block [0x825222A8..0x82522358)
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
    //   block [0x825223C0..0x82522430)
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
    //   block [0x82522430..0x82522528)
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
    //   block [0x82522528..0x82522720)
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
    //   block [0x82522720..0x825228D4)
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
    //   block [0x825228D8..0x82522A90)
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


