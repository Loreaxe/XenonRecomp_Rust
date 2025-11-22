pub fn sub_82E4A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A738 size=24
    let mut pc: u32 = 0x82E4A738;
    'dispatch: loop {
        match pc {
            0x82E4A738 => {
    //   block [0x82E4A738..0x82E4A750)
	// 82E4A738: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A73C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A740: 396B6500  addi r11, r11, 0x6500
	ctx.r[11].s64 = ctx.r[11].s64 + 25856;
	// 82E4A744: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A748: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A750 size=12
    let mut pc: u32 = 0x82E4A750;
    'dispatch: loop {
        match pc {
            0x82E4A750 => {
    //   block [0x82E4A750..0x82E4A75C)
	// 82E4A750: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A754: 386B6500  addi r3, r11, 0x6500
	ctx.r[3].s64 = ctx.r[11].s64 + 25856;
	// 82E4A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A760 size=100
    let mut pc: u32 = 0x82E4A760;
    'dispatch: loop {
        match pc {
            0x82E4A760 => {
    //   block [0x82E4A760..0x82E4A7C4)
	// 82E4A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A76C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A77C: 4800ADC5  bl 0x82e55540
	ctx.lr = 0x82E4A780;
	sub_82E55540(ctx, base);
	// 82E4A780: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A788: 419A0020  beq cr6, 0x82e4a7a8
	if ctx.cr[6].eq {
	pc = 0x82E4A7A8; continue 'dispatch;
	}
	// 82E4A78C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A790: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A794: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4A798: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A79C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A7A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A7A4: 4BF0AB25  bl 0x82d552c8
	ctx.lr = 0x82E4A7A8;
	sub_82D552C8(ctx, base);
	// 82E4A7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A7AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A7B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A7BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7C8 size=4
    let mut pc: u32 = 0x82E4A7C8;
    'dispatch: loop {
        match pc {
            0x82E4A7C8 => {
    //   block [0x82E4A7C8..0x82E4A7CC)
	// 82E4A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7D0 size=20
    let mut pc: u32 = 0x82E4A7D0;
    'dispatch: loop {
        match pc {
            0x82E4A7D0 => {
    //   block [0x82E4A7D0..0x82E4A7E4)
	// 82E4A7D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A7D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A7D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A7DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A7E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7E8 size=12
    let mut pc: u32 = 0x82E4A7E8;
    'dispatch: loop {
        match pc {
            0x82E4A7E8 => {
    //   block [0x82E4A7E8..0x82E4A7F4)
	// 82E4A7E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4A7EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A7F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A7F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A7F4 size=8
    let mut pc: u32 = 0x82E4A7F4;
    'dispatch: loop {
        match pc {
            0x82E4A7F4 => {
    //   block [0x82E4A7F4..0x82E4A7FC)
	// 82E4A7F4: 4800003C  b 0x82e4a830
	sub_82E4A830(ctx, base);
	return;
	// 82E4A7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A800 size=44
    let mut pc: u32 = 0x82E4A800;
    'dispatch: loop {
        match pc {
            0x82E4A800 => {
    //   block [0x82E4A800..0x82E4A82C)
	// 82E4A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A808: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A80C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A814: 4800001D  bl 0x82e4a830
	ctx.lr = 0x82E4A818;
	sub_82E4A830(ctx, base);
	// 82E4A818: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4A81C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4A820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A830 size=168
    let mut pc: u32 = 0x82E4A830;
    'dispatch: loop {
        match pc {
            0x82E4A830 => {
    //   block [0x82E4A830..0x82E4A8D8)
	// 82E4A830: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4A838: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A83C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A840: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A844: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A848: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4A84C: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A850: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A854: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A858: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A85C: 38CB6614  addi r6, r11, 0x6614
	ctx.r[6].s64 = ctx.r[11].s64 + 26132;
	// 82E4A860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A864: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A868: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A86C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A870: 38AB6608  addi r5, r11, 0x6608
	ctx.r[5].s64 = ctx.r[11].s64 + 26120;
	// 82E4A874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A878: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A87C: 394B65F4  addi r10, r11, 0x65f4
	ctx.r[10].s64 = ctx.r[11].s64 + 26100;
	// 82E4A880: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A884: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A888: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E4A88C: 392B65E8  addi r9, r11, 0x65e8
	ctx.r[9].s64 = ctx.r[11].s64 + 26088;
	// 82E4A890: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A894: 388B65DC  addi r4, r11, 0x65dc
	ctx.r[4].s64 = ctx.r[11].s64 + 26076;
	// 82E4A898: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A89C: 390B65C4  addi r8, r11, 0x65c4
	ctx.r[8].s64 = ctx.r[11].s64 + 26052;
	// 82E4A8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4A8A4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E4A8A8: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4A8AC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A8B0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E4A8B4: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E4A8B8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E4A8BC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A8C0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A8C4: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4A8C8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E4A8CC: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4A8D0: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E4A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A8D8 size=100
    let mut pc: u32 = 0x82E4A8D8;
    'dispatch: loop {
        match pc {
            0x82E4A8D8 => {
    //   block [0x82E4A8D8..0x82E4A93C)
	// 82E4A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A8EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A8F4: 4800AF0D  bl 0x82e55800
	ctx.lr = 0x82E4A8F8;
	sub_82E55800(ctx, base);
	// 82E4A8F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A8FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A900: 419A0020  beq cr6, 0x82e4a920
	if ctx.cr[6].eq {
	pc = 0x82E4A920; continue 'dispatch;
	}
	// 82E4A904: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A908: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A90C: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A910: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A918: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A91C: 4BF0A9AD  bl 0x82d552c8
	ctx.lr = 0x82E4A920;
	sub_82D552C8(ctx, base);
	// 82E4A920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A940 size=8
    let mut pc: u32 = 0x82E4A940;
    'dispatch: loop {
        match pc {
            0x82E4A940 => {
    //   block [0x82E4A940..0x82E4A948)
	// 82E4A940: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A944: 4BFFFF94  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A948 size=8
    let mut pc: u32 = 0x82E4A948;
    'dispatch: loop {
        match pc {
            0x82E4A948 => {
    //   block [0x82E4A948..0x82E4A950)
	// 82E4A948: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A94C: 4BFFFF8C  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A950 size=8
    let mut pc: u32 = 0x82E4A950;
    'dispatch: loop {
        match pc {
            0x82E4A950 => {
    //   block [0x82E4A950..0x82E4A958)
	// 82E4A950: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A954: 4BFFFF84  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A958 size=8
    let mut pc: u32 = 0x82E4A958;
    'dispatch: loop {
        match pc {
            0x82E4A958 => {
    //   block [0x82E4A958..0x82E4A960)
	// 82E4A958: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4A95C: 4BFFFF7C  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A960 size=8
    let mut pc: u32 = 0x82E4A960;
    'dispatch: loop {
        match pc {
            0x82E4A960 => {
    //   block [0x82E4A960..0x82E4A968)
	// 82E4A960: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A964: 4BFFFF74  b 0x82e4a8d8
	sub_82E4A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A968 size=20
    let mut pc: u32 = 0x82E4A968;
    'dispatch: loop {
        match pc {
            0x82E4A968 => {
    //   block [0x82E4A968..0x82E4A97C)
	// 82E4A968: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A96C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A978: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A980 size=4
    let mut pc: u32 = 0x82E4A980;
    'dispatch: loop {
        match pc {
            0x82E4A980 => {
    //   block [0x82E4A980..0x82E4A984)
	// 82E4A980: 4800B3A0  b 0x82e55d20
	sub_82E55D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A988 size=4
    let mut pc: u32 = 0x82E4A988;
    'dispatch: loop {
        match pc {
            0x82E4A988 => {
    //   block [0x82E4A988..0x82E4A98C)
	// 82E4A988: 4800B398  b 0x82e55d20
	sub_82E55D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A990 size=4
    let mut pc: u32 = 0x82E4A990;
    'dispatch: loop {
        match pc {
            0x82E4A990 => {
    //   block [0x82E4A990..0x82E4A994)
	// 82E4A990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A998 size=4
    let mut pc: u32 = 0x82E4A998;
    'dispatch: loop {
        match pc {
            0x82E4A998 => {
    //   block [0x82E4A998..0x82E4A99C)
	// 82E4A998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A9A0 size=12
    let mut pc: u32 = 0x82E4A9A0;
    'dispatch: loop {
        match pc {
            0x82E4A9A0 => {
    //   block [0x82E4A9A0..0x82E4A9AC)
	// 82E4A9A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4A9A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A9A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A9AC size=32
    let mut pc: u32 = 0x82E4A9AC;
    'dispatch: loop {
        match pc {
            0x82E4A9AC => {
    //   block [0x82E4A9AC..0x82E4A9CC)
	// 82E4A9AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A9B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E4A9B4: 394A6860  addi r10, r10, 0x6860
	ctx.r[10].s64 = ctx.r[10].s64 + 26720;
	// 82E4A9B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4A9BC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E4A9C0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E4A9C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4A9C8: 4BF53D78  b 0x82d9e740
	sub_82D9E740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A9CC size=4
    let mut pc: u32 = 0x82E4A9CC;
    'dispatch: loop {
        match pc {
            0x82E4A9CC => {
    //   block [0x82E4A9CC..0x82E4A9D0)
	// 82E4A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A9D0 size=64
    let mut pc: u32 = 0x82E4A9D0;
    'dispatch: loop {
        match pc {
            0x82E4A9D0 => {
    //   block [0x82E4A9D0..0x82E4AA10)
	// 82E4A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A9D8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A9DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A9E4: 396B6860  addi r11, r11, 0x6860
	ctx.r[11].s64 = ctx.r[11].s64 + 26720;
	// 82E4A9E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A9EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4A9F0: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82E4A9F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4A9F8: 4BF53D49  bl 0x82d9e740
	ctx.lr = 0x82E4A9FC;
	sub_82D9E740(ctx, base);
	// 82E4A9FC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4AA00: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 82E4AA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AA10 size=100
    let mut pc: u32 = 0x82E4AA10;
    'dispatch: loop {
        match pc {
            0x82E4AA10 => {
    //   block [0x82E4AA10..0x82E4AA74)
	// 82E4AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AA18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AA1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AA20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AA28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AA2C: 4800B475  bl 0x82e55ea0
	ctx.lr = 0x82E4AA30;
	sub_82E55EA0(ctx, base);
	// 82E4AA30: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AA34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AA38: 419A0020  beq cr6, 0x82e4aa58
	if ctx.cr[6].eq {
	pc = 0x82E4AA58; continue 'dispatch;
	}
	// 82E4AA3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA40: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AA44: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4AA48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AA4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AA50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AA54: 4BF0A875  bl 0x82d552c8
	ctx.lr = 0x82E4AA58;
	sub_82D552C8(ctx, base);
	// 82E4AA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AA5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AA68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AA78 size=20
    let mut pc: u32 = 0x82E4AA78;
    'dispatch: loop {
        match pc {
            0x82E4AA78 => {
    //   block [0x82E4AA78..0x82E4AA8C)
	// 82E4AA78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AA80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AA84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AA88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AA90 size=8
    let mut pc: u32 = 0x82E4AA90;
    'dispatch: loop {
        match pc {
            0x82E4AA90 => {
    //   block [0x82E4AA90..0x82E4AA98)
	// 82E4AA90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AA94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AA98 size=24
    let mut pc: u32 = 0x82E4AA98;
    'dispatch: loop {
        match pc {
            0x82E4AA98 => {
    //   block [0x82E4AA98..0x82E4AAB0)
	// 82E4AA98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AA9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AAA0: 396B6950  addi r11, r11, 0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + 26960;
	// 82E4AAA4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AAA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AAB0 size=12
    let mut pc: u32 = 0x82E4AAB0;
    'dispatch: loop {
        match pc {
            0x82E4AAB0 => {
    //   block [0x82E4AAB0..0x82E4AABC)
	// 82E4AAB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AAB4: 386B6950  addi r3, r11, 0x6950
	ctx.r[3].s64 = ctx.r[11].s64 + 26960;
	// 82E4AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AAC0 size=100
    let mut pc: u32 = 0x82E4AAC0;
    'dispatch: loop {
        match pc {
            0x82E4AAC0 => {
    //   block [0x82E4AAC0..0x82E4AB24)
	// 82E4AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AAC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AAD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AAD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AAD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AADC: 4800B54D  bl 0x82e56028
	ctx.lr = 0x82E4AAE0;
	sub_82E56028(ctx, base);
	// 82E4AAE0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AAE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AAE8: 419A0020  beq cr6, 0x82e4ab08
	if ctx.cr[6].eq {
	pc = 0x82E4AB08; continue 'dispatch;
	}
	// 82E4AAEC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AAF0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AAF4: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 82E4AAF8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AAFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AB00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AB04: 4BF0A7C5  bl 0x82d552c8
	ctx.lr = 0x82E4AB08;
	sub_82D552C8(ctx, base);
	// 82E4AB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AB18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB28 size=8
    let mut pc: u32 = 0x82E4AB28;
    'dispatch: loop {
        match pc {
            0x82E4AB28 => {
    //   block [0x82E4AB28..0x82E4AB30)
	// 82E4AB28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AB2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB30 size=24
    let mut pc: u32 = 0x82E4AB30;
    'dispatch: loop {
        match pc {
            0x82E4AB30 => {
    //   block [0x82E4AB30..0x82E4AB48)
	// 82E4AB30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AB38: 396B6A04  addi r11, r11, 0x6a04
	ctx.r[11].s64 = ctx.r[11].s64 + 27140;
	// 82E4AB3C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AB40: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB48 size=20
    let mut pc: u32 = 0x82E4AB48;
    'dispatch: loop {
        match pc {
            0x82E4AB48 => {
    //   block [0x82E4AB48..0x82E4AB5C)
	// 82E4AB48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AB50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AB58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB60 size=12
    let mut pc: u32 = 0x82E4AB60;
    'dispatch: loop {
        match pc {
            0x82E4AB60 => {
    //   block [0x82E4AB60..0x82E4AB6C)
	// 82E4AB60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB64: 386B6A04  addi r3, r11, 0x6a04
	ctx.r[3].s64 = ctx.r[11].s64 + 27140;
	// 82E4AB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB70 size=20
    let mut pc: u32 = 0x82E4AB70;
    'dispatch: loop {
        match pc {
            0x82E4AB70 => {
    //   block [0x82E4AB70..0x82E4AB84)
	// 82E4AB70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AB78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AB80: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB88 size=8
    let mut pc: u32 = 0x82E4AB88;
    'dispatch: loop {
        match pc {
            0x82E4AB88 => {
    //   block [0x82E4AB88..0x82E4AB90)
	// 82E4AB88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AB8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AB90 size=24
    let mut pc: u32 = 0x82E4AB90;
    'dispatch: loop {
        match pc {
            0x82E4AB90 => {
    //   block [0x82E4AB90..0x82E4ABA8)
	// 82E4AB90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AB94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AB98: 396B6A54  addi r11, r11, 0x6a54
	ctx.r[11].s64 = ctx.r[11].s64 + 27220;
	// 82E4AB9C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4ABA0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ABA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ABA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ABA8 size=12
    let mut pc: u32 = 0x82E4ABA8;
    'dispatch: loop {
        match pc {
            0x82E4ABA8 => {
    //   block [0x82E4ABA8..0x82E4ABB4)
	// 82E4ABA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4ABAC: 386B6A54  addi r3, r11, 0x6a54
	ctx.r[3].s64 = ctx.r[11].s64 + 27220;
	// 82E4ABB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4ABB8 size=96
    let mut pc: u32 = 0x82E4ABB8;
    'dispatch: loop {
        match pc {
            0x82E4ABB8 => {
    //   block [0x82E4ABB8..0x82E4AC18)
	// 82E4ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4ABC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4ABC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4ABC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4ABCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4ABD0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4ABD4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E4ABD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4ABDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ABE0: 419A0020  beq cr6, 0x82e4ac00
	if ctx.cr[6].eq {
	pc = 0x82E4AC00; continue 'dispatch;
	}
	// 82E4ABE4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ABE8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4ABEC: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82E4ABF0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4ABF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4ABF8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4ABFC: 4BF0A6CD  bl 0x82d552c8
	ctx.lr = 0x82E4AC00;
	sub_82D552C8(ctx, base);
	// 82E4AC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4AC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AC10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC18 size=20
    let mut pc: u32 = 0x82E4AC18;
    'dispatch: loop {
        match pc {
            0x82E4AC18 => {
    //   block [0x82E4AC18..0x82E4AC2C)
	// 82E4AC18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AC1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AC20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AC24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AC28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC30 size=8
    let mut pc: u32 = 0x82E4AC30;
    'dispatch: loop {
        match pc {
            0x82E4AC30 => {
    //   block [0x82E4AC30..0x82E4AC38)
	// 82E4AC30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AC34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC38 size=24
    let mut pc: u32 = 0x82E4AC38;
    'dispatch: loop {
        match pc {
            0x82E4AC38 => {
    //   block [0x82E4AC38..0x82E4AC50)
	// 82E4AC38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AC40: 396B6AA0  addi r11, r11, 0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC44: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AC48: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AC50 size=12
    let mut pc: u32 = 0x82E4AC50;
    'dispatch: loop {
        match pc {
            0x82E4AC50 => {
    //   block [0x82E4AC50..0x82E4AC5C)
	// 82E4AC50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC54: 386B6AA0  addi r3, r11, 0x6aa0
	ctx.r[3].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AC60 size=196
    let mut pc: u32 = 0x82E4AC60;
    'dispatch: loop {
        match pc {
            0x82E4AC60 => {
    //   block [0x82E4AC60..0x82E4AD24)
	// 82E4AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AC64: 4BE5E7A9  bl 0x82ca940c
	ctx.lr = 0x82E4AC68;
	sub_82CA93D0(ctx, base);
	// 82E4AC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AC6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AC70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AC74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4AC78: 396B6AA0  addi r11, r11, 0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 27296;
	// 82E4AC7C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4AC80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4AC84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AC88: 4099005C  ble cr6, 0x82e4ace4
	if !ctx.cr[6].gt {
	pc = 0x82E4ACE4; continue 'dispatch;
	}
	// 82E4AC8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4AC90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4AC94: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4AC98: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4ACA0: 419A0030  beq cr6, 0x82e4acd0
	if ctx.cr[6].eq {
	pc = 0x82E4ACD0; continue 'dispatch;
	}
	// 82E4ACA4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4ACA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4ACAC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4ACB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4ACB4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4ACB8: 409A0018  bne cr6, 0x82e4acd0
	if !ctx.cr[6].eq {
	pc = 0x82E4ACD0; continue 'dispatch;
	}
	// 82E4ACBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4ACC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4ACCC: 4E800421  bctrl
	ctx.lr = 0x82E4ACD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4ACD0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4ACD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4ACD8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4ACDC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4ACE0: 4198FFB0  blt cr6, 0x82e4ac90
	if ctx.cr[6].lt {
	pc = 0x82E4AC90; continue 'dispatch;
	}
	// 82E4ACE4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4ACE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4ACEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4ACF0: 409A0020  bne cr6, 0x82e4ad10
	if !ctx.cr[6].eq {
	pc = 0x82E4AD10; continue 'dispatch;
	}
	// 82E4ACF4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ACF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4ACFC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4AD00: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4AD04: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4AD08: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4AD0C: 4BF0A5BD  bl 0x82d552c8
	ctx.lr = 0x82E4AD10;
	sub_82D552C8(ctx, base);
	// 82E4AD10: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4AD14: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4AD18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AD20: 4BE5E73C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AD28 size=100
    let mut pc: u32 = 0x82E4AD28;
    'dispatch: loop {
        match pc {
            0x82E4AD28 => {
    //   block [0x82E4AD28..0x82E4AD8C)
	// 82E4AD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AD30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AD34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AD38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AD3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AD40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AD44: 4BFFFF1D  bl 0x82e4ac60
	ctx.lr = 0x82E4AD48;
	sub_82E4AC60(ctx, base);
	// 82E4AD48: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AD4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AD50: 419A0020  beq cr6, 0x82e4ad70
	if ctx.cr[6].eq {
	pc = 0x82E4AD70; continue 'dispatch;
	}
	// 82E4AD54: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AD58: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AD5C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E4AD60: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AD64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AD68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AD6C: 4BF0A55D  bl 0x82d552c8
	ctx.lr = 0x82E4AD70;
	sub_82D552C8(ctx, base);
	// 82E4AD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AD74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AD80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AD84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AD90 size=8
    let mut pc: u32 = 0x82E4AD90;
    'dispatch: loop {
        match pc {
            0x82E4AD90 => {
    //   block [0x82E4AD90..0x82E4AD98)
	// 82E4AD90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AD94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AD98 size=24
    let mut pc: u32 = 0x82E4AD98;
    'dispatch: loop {
        match pc {
            0x82E4AD98 => {
    //   block [0x82E4AD98..0x82E4ADB0)
	// 82E4AD98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AD9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4ADA0: 396B6BE4  addi r11, r11, 0x6be4
	ctx.r[11].s64 = ctx.r[11].s64 + 27620;
	// 82E4ADA4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4ADA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4ADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ADB0 size=20
    let mut pc: u32 = 0x82E4ADB0;
    'dispatch: loop {
        match pc {
            0x82E4ADB0 => {
    //   block [0x82E4ADB0..0x82E4ADC4)
	// 82E4ADB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ADB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4ADB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ADBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4ADC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4ADC8 size=12
    let mut pc: u32 = 0x82E4ADC8;
    'dispatch: loop {
        match pc {
            0x82E4ADC8 => {
    //   block [0x82E4ADC8..0x82E4ADD4)
	// 82E4ADC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4ADCC: 386B6BE4  addi r3, r11, 0x6be4
	ctx.r[3].s64 = ctx.r[11].s64 + 27620;
	// 82E4ADD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4ADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4ADD8 size=100
    let mut pc: u32 = 0x82E4ADD8;
    'dispatch: loop {
        match pc {
            0x82E4ADD8 => {
    //   block [0x82E4ADD8..0x82E4AE3C)
	// 82E4ADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4ADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4ADE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4ADE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4ADE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4ADEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4ADF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4ADF4: 4BF3C7FD  bl 0x82d875f0
	ctx.lr = 0x82E4ADF8;
	sub_82D875F0(ctx, base);
	// 82E4ADF8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4ADFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AE00: 419A0020  beq cr6, 0x82e4ae20
	if ctx.cr[6].eq {
	pc = 0x82E4AE20; continue 'dispatch;
	}
	// 82E4AE04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE08: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4AE0C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82E4AE10: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4AE14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4AE18: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4AE1C: 4BF0A4AD  bl 0x82d552c8
	ctx.lr = 0x82E4AE20;
	sub_82D552C8(ctx, base);
	// 82E4AE20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AE24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4AE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AE30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4AE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE40 size=4
    let mut pc: u32 = 0x82E4AE40;
    'dispatch: loop {
        match pc {
            0x82E4AE40 => {
    //   block [0x82E4AE40..0x82E4AE44)
	// 82E4AE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE48 size=8
    let mut pc: u32 = 0x82E4AE48;
    'dispatch: loop {
        match pc {
            0x82E4AE48 => {
    //   block [0x82E4AE48..0x82E4AE50)
	// 82E4AE48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AE4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE50 size=24
    let mut pc: u32 = 0x82E4AE50;
    'dispatch: loop {
        match pc {
            0x82E4AE50 => {
    //   block [0x82E4AE50..0x82E4AE68)
	// 82E4AE50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AE54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4AE58: 396B6FE4  addi r11, r11, 0x6fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 28644;
	// 82E4AE5C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4AE60: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE68 size=20
    let mut pc: u32 = 0x82E4AE68;
    'dispatch: loop {
        match pc {
            0x82E4AE68 => {
    //   block [0x82E4AE68..0x82E4AE7C)
	// 82E4AE68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AE70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AE74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AE78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AE80 size=12
    let mut pc: u32 = 0x82E4AE80;
    'dispatch: loop {
        match pc {
            0x82E4AE80 => {
    //   block [0x82E4AE80..0x82E4AE8C)
	// 82E4AE80: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AE84: 386B6FE4  addi r3, r11, 0x6fe4
	ctx.r[3].s64 = ctx.r[11].s64 + 28644;
	// 82E4AE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AE90 size=72
    let mut pc: u32 = 0x82E4AE90;
    'dispatch: loop {
        match pc {
            0x82E4AE90 => {
    //   block [0x82E4AE90..0x82E4AED8)
	// 82E4AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AE98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AE9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AEA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AEA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AEA8: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82E4AEAC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E4AEB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4AEB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AEB8: 419A000C  beq cr6, 0x82e4aec4
	if ctx.cr[6].eq {
	pc = 0x82E4AEC4; continue 'dispatch;
	}
	// 82E4AEBC: 4B9FA8F5  bl 0x828457b0
	ctx.lr = 0x82E4AEC0;
	sub_828457B0(ctx, base);
	// 82E4AEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4AEC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4AEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4AED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AED8 size=20
    let mut pc: u32 = 0x82E4AED8;
    'dispatch: loop {
        match pc {
            0x82E4AED8 => {
    //   block [0x82E4AED8..0x82E4AEEC)
	// 82E4AED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AEDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AEE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4AEE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4AEE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AEF0 size=12
    let mut pc: u32 = 0x82E4AEF0;
    'dispatch: loop {
        match pc {
            0x82E4AEF0 => {
    //   block [0x82E4AEF0..0x82E4AEFC)
	// 82E4AEF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4AEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4AEF8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AEFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AEFC size=8
    let mut pc: u32 = 0x82E4AEFC;
    'dispatch: loop {
        match pc {
            0x82E4AEFC => {
    //   block [0x82E4AEFC..0x82E4AF04)
	// 82E4AEFC: 4800003C  b 0x82e4af38
	sub_82E4AF38(ctx, base);
	return;
	// 82E4AF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AF08 size=44
    let mut pc: u32 = 0x82E4AF08;
    'dispatch: loop {
        match pc {
            0x82E4AF08 => {
    //   block [0x82E4AF08..0x82E4AF34)
	// 82E4AF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AF10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AF14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4AF1C: 4800001D  bl 0x82e4af38
	ctx.lr = 0x82E4AF20;
	sub_82E4AF38(ctx, base);
	// 82E4AF20: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4AF24: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4AF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4AF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4AF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4AF38 size=152
    let mut pc: u32 = 0x82E4AF38;
    'dispatch: loop {
        match pc {
            0x82E4AF38 => {
    //   block [0x82E4AF38..0x82E4AFD0)
	// 82E4AF38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AF3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4AF40: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4AF44: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4AF48: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4AF4C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4AF50: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4AF54: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4AF58: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4AF5C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4AF60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4AF64: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4AF68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4AF6C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4AF70: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E4AF74: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4AF78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4AF7C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4AF80: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4AF84: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E4AF88: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4AF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4AF90: 38E77044  addi r7, r7, 0x7044
	ctx.r[7].s64 = ctx.r[7].s64 + 28740;
	// 82E4AF94: 396B709C  addi r11, r11, 0x709c
	ctx.r[11].s64 = ctx.r[11].s64 + 28828;
	// 82E4AF98: 38C67090  addi r6, r6, 0x7090
	ctx.r[6].s64 = ctx.r[6].s64 + 28816;
	// 82E4AF9C: 394A707C  addi r10, r10, 0x707c
	ctx.r[10].s64 = ctx.r[10].s64 + 28796;
	// 82E4AFA0: 39297070  addi r9, r9, 0x7070
	ctx.r[9].s64 = ctx.r[9].s64 + 28784;
	// 82E4AFA4: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82E4AFA8: 38A57064  addi r5, r5, 0x7064
	ctx.r[5].s64 = ctx.r[5].s64 + 28772;
	// 82E4AFAC: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4AFB0: 39087054  addi r8, r8, 0x7054
	ctx.r[8].s64 = ctx.r[8].s64 + 28756;
	// 82E4AFB4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4AFB8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E4AFBC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4AFC0: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4AFC4: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82E4AFC8: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4AFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4AFD0 size=100
    let mut pc: u32 = 0x82E4AFD0;
    'dispatch: loop {
        match pc {
            0x82E4AFD0 => {
    //   block [0x82E4AFD0..0x82E4B034)
	// 82E4AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4AFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4AFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4AFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4AFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4AFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4AFE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4AFEC: 4800BF3D  bl 0x82e56f28
	ctx.lr = 0x82E4AFF0;
	sub_82E56F28(ctx, base);
	// 82E4AFF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4AFF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4AFF8: 419A0020  beq cr6, 0x82e4b018
	if ctx.cr[6].eq {
	pc = 0x82E4B018; continue 'dispatch;
	}
	// 82E4AFFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B000: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B004: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4B008: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B00C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4B010: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B014: 4BF0A2B5  bl 0x82d552c8
	ctx.lr = 0x82E4B018;
	sub_82D552C8(ctx, base);
	// 82E4B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B01C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4B020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4B024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4B028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4B02C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4B030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B038 size=8
    let mut pc: u32 = 0x82E4B038;
    'dispatch: loop {
        match pc {
            0x82E4B038 => {
    //   block [0x82E4B038..0x82E4B040)
	// 82E4B038: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4B03C: 4BFFFF94  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B040 size=8
    let mut pc: u32 = 0x82E4B040;
    'dispatch: loop {
        match pc {
            0x82E4B040 => {
    //   block [0x82E4B040..0x82E4B048)
	// 82E4B040: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4B044: 4BFFFF8C  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B048 size=8
    let mut pc: u32 = 0x82E4B048;
    'dispatch: loop {
        match pc {
            0x82E4B048 => {
    //   block [0x82E4B048..0x82E4B050)
	// 82E4B048: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4B04C: 4BFFFF84  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B050 size=8
    let mut pc: u32 = 0x82E4B050;
    'dispatch: loop {
        match pc {
            0x82E4B050 => {
    //   block [0x82E4B050..0x82E4B058)
	// 82E4B050: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4B054: 4BFFFF7C  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B058 size=8
    let mut pc: u32 = 0x82E4B058;
    'dispatch: loop {
        match pc {
            0x82E4B058 => {
    //   block [0x82E4B058..0x82E4B060)
	// 82E4B058: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4B05C: 4BFFFF74  b 0x82e4afd0
	sub_82E4AFD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B060 size=8
    let mut pc: u32 = 0x82E4B060;
    'dispatch: loop {
        match pc {
            0x82E4B060 => {
    //   block [0x82E4B060..0x82E4B068)
	// 82E4B060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B064: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B068 size=24
    let mut pc: u32 = 0x82E4B068;
    'dispatch: loop {
        match pc {
            0x82E4B068 => {
    //   block [0x82E4B068..0x82E4B080)
	// 82E4B068: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B06C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4B070: 396B7134  addi r11, r11, 0x7134
	ctx.r[11].s64 = ctx.r[11].s64 + 28980;
	// 82E4B074: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4B078: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B080 size=20
    let mut pc: u32 = 0x82E4B080;
    'dispatch: loop {
        match pc {
            0x82E4B080 => {
    //   block [0x82E4B080..0x82E4B094)
	// 82E4B080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B084: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B088: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B090: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B098 size=12
    let mut pc: u32 = 0x82E4B098;
    'dispatch: loop {
        match pc {
            0x82E4B098 => {
    //   block [0x82E4B098..0x82E4B0A4)
	// 82E4B098: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B09C: 386B7134  addi r3, r11, 0x7134
	ctx.r[3].s64 = ctx.r[11].s64 + 28980;
	// 82E4B0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B0A8 size=332
    let mut pc: u32 = 0x82E4B0A8;
    'dispatch: loop {
        match pc {
            0x82E4B0A8 => {
    //   block [0x82E4B0A8..0x82E4B1F4)
	// 82E4B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B0AC: 4BE5E355  bl 0x82ca9400
	ctx.lr = 0x82E4B0B0;
	sub_82CA93D0(ctx, base);
	// 82E4B0B0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B0B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B0B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4B0BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4B0C0: 4BF36971  bl 0x82d81a30
	ctx.lr = 0x82E4B0C4;
	sub_82D81A30(ctx, base);
	// 82E4B0C4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B0C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B0CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B0D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B0D4: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B0D8: 38800310  li r4, 0x310
	ctx.r[4].s64 = 784;
	// 82E4B0DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B0E0: 409A0018  bne cr6, 0x82e4b0f8
	if !ctx.cr[6].eq {
	pc = 0x82E4B0F8; continue 'dispatch;
	}
	// 82E4B0E4: 4BF0A165  bl 0x82d55248
	ctx.lr = 0x82E4B0E8;
	sub_82D55248(ctx, base);
	// 82E4B0E8: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 82E4B0EC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E4B0F0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B0F4: 48000014  b 0x82e4b108
	pc = 0x82E4B108; continue 'dispatch;
	// 82E4B0F8: 4BF0A151  bl 0x82d55248
	ctx.lr = 0x82E4B0FC;
	sub_82D55248(ctx, base);
	// 82E4B0FC: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 82E4B100: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B104: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B108: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 82E4B10C: 60A5C3B4  ori r5, r5, 0xc3b4
	ctx.r[5].u64 = ctx.r[5].u64 | 50100;
	// 82E4B110: 4BF2F3F9  bl 0x82d7a508
	ctx.lr = 0x82E4B114;
	sub_82D7A508(ctx, base);
	// 82E4B114: 7FEB0774  extsb r11, r31
	ctx.r[11].s64 = ctx.r[31].s8 as i64;
	// 82E4B118: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4B11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B120: 419A000C  beq cr6, 0x82e4b12c
	if ctx.cr[6].eq {
	pc = 0x82E4B12C; continue 'dispatch;
	}
	// 82E4B124: 807C0074  lwz r3, 0x74(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E4B128: 4BF682F9  bl 0x82db3420
	ctx.lr = 0x82E4B12C;
	sub_82DB3420(ctx, base);
	// 82E4B12C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B130: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B134: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82E4B138: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B13C: 40990084  ble cr6, 0x82e4b1c0
	if !ctx.cr[6].gt {
	pc = 0x82E4B1C0; continue 'dispatch;
	}
	// 82E4B140: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E4B144: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E4B148: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B14C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B150: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B154: 4BF33CAD  bl 0x82d7ee00
	ctx.lr = 0x82E4B158;
	sub_82D7EE00(ctx, base);
	// 82E4B158: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B15C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B160: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B164: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B168: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B16C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B170: 4E800421  bctrl
	ctx.lr = 0x82E4B174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B174: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B17C: 419A0030  beq cr6, 0x82e4b1ac
	if ctx.cr[6].eq {
	pc = 0x82E4B1AC; continue 'dispatch;
	}
	// 82E4B180: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B184: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E4B188: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4B18C: 9BA10058  stb r29, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u8 ) };
	// 82E4B190: 9BA10059  stb r29, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[29].u8 ) };
	// 82E4B194: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E4B198: 9B610060  stb r27, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u8 ) };
	// 82E4B19C: 9B610061  stb r27, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[27].u8 ) };
	// 82E4B1A0: 9BA10062  stb r29, 0x62(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[29].u8 ) };
	// 82E4B1A4: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B1A8: 4800DEB1  bl 0x82e59058
	ctx.lr = 0x82E4B1AC;
	sub_82E59058(ctx, base);
	// 82E4B1AC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B1B0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82E4B1B4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B1B8: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B1BC: 4198FF8C  blt cr6, 0x82e4b148
	if ctx.cr[6].lt {
	pc = 0x82E4B148; continue 'dispatch;
	}
	// 82E4B1C0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B1C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B1C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B1CC: 419A0020  beq cr6, 0x82e4b1ec
	if ctx.cr[6].eq {
	pc = 0x82E4B1EC; continue 'dispatch;
	}
	// 82E4B1D0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4B1D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B1D8: 419A0014  beq cr6, 0x82e4b1ec
	if ctx.cr[6].eq {
	pc = 0x82E4B1EC; continue 'dispatch;
	}
	// 82E4B1DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E4B1E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B1E4: 4BF322DD  bl 0x82d7d4c0
	ctx.lr = 0x82E4B1E8;
	sub_82D7D4C0(ctx, base);
	// 82E4B1E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B1EC: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 82E4B1F0: 4BE5E260  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B1F8 size=232
    let mut pc: u32 = 0x82E4B1F8;
    'dispatch: loop {
        match pc {
            0x82E4B1F8 => {
    //   block [0x82E4B1F8..0x82E4B2E0)
	// 82E4B1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B1FC: 4BE5E20D  bl 0x82ca9408
	ctx.lr = 0x82E4B200;
	sub_82CA93D0(ctx, base);
	// 82E4B200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B204: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4B208: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4B20C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4B210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B214: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B218: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E4B21C: 40990024  ble cr6, 0x82e4b240
	if !ctx.cr[6].gt {
	pc = 0x82E4B240; continue 'dispatch;
	}
	// 82E4B220: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B224: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B228: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4B22C: 419A004C  beq cr6, 0x82e4b278
	if ctx.cr[6].eq {
	pc = 0x82E4B278; continue 'dispatch;
	}
	// 82E4B230: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4B234: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E4B238: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4B23C: 4198FFE8  blt cr6, 0x82e4b224
	if ctx.cr[6].lt {
	pc = 0x82E4B224; continue 'dispatch;
	}
	// 82E4B240: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82E4B244: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4B24C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E4B250: 40990038  ble cr6, 0x82e4b288
	if !ctx.cr[6].gt {
	pc = 0x82E4B288; continue 'dispatch;
	}
	// 82E4B254: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B258: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B25C: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4B260: 419A0020  beq cr6, 0x82e4b280
	if ctx.cr[6].eq {
	pc = 0x82E4B280; continue 'dispatch;
	}
	// 82E4B264: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4B268: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E4B26C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4B270: 4198FFE8  blt cr6, 0x82e4b258
	if ctx.cr[6].lt {
	pc = 0x82E4B258; continue 'dispatch;
	}
	// 82E4B274: 48000014  b 0x82e4b288
	pc = 0x82E4B288; continue 'dispatch;
	// 82E4B278: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E4B27C: 4BFFFFC8  b 0x82e4b244
	pc = 0x82E4B244; continue 'dispatch;
	// 82E4B280: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82E4B284: 409A0054  bne cr6, 0x82e4b2d8
	if !ctx.cr[6].eq {
	pc = 0x82E4B2D8; continue 'dispatch;
	}
	// 82E4B288: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82E4B28C: 419A004C  beq cr6, 0x82e4b2d8
	if ctx.cr[6].eq {
	pc = 0x82E4B2D8; continue 'dispatch;
	}
	// 82E4B290: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E4B294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B298: 409A0020  bne cr6, 0x82e4b2b8
	if !ctx.cr[6].eq {
	pc = 0x82E4B2B8; continue 'dispatch;
	}
	// 82E4B29C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B2A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B2A4: 4BF39FB5  bl 0x82d85258
	ctx.lr = 0x82E4B2A8;
	sub_82D85258(ctx, base);
	// 82E4B2A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B2B4: 419A0008  beq cr6, 0x82e4b2bc
	if ctx.cr[6].eq {
	pc = 0x82E4B2BC; continue 'dispatch;
	}
	// 82E4B2B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4B2BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B2C0: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4B2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B2C8: 4BF38121  bl 0x82d833e8
	ctx.lr = 0x82E4B2CC;
	sub_82D833E8(ctx, base);
	// 82E4B2CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4B2D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B2D4: 4BF379B5  bl 0x82d82c88
	ctx.lr = 0x82E4B2D8;
	sub_82D82C88(ctx, base);
	// 82E4B2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B2DC: 4BE5E17C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B2E0 size=128
    let mut pc: u32 = 0x82E4B2E0;
    'dispatch: loop {
        match pc {
            0x82E4B2E0 => {
    //   block [0x82E4B2E0..0x82E4B360)
	// 82E4B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B2E4: 4BE5E125  bl 0x82ca9408
	ctx.lr = 0x82E4B2E8;
	sub_82CA93D0(ctx, base);
	// 82E4B2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B2EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B2F0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4B2F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B2F8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B2FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B300: 40990040  ble cr6, 0x82e4b340
	if !ctx.cr[6].gt {
	pc = 0x82E4B340; continue 'dispatch;
	}
	// 82E4B304: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4B308: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B30C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E4B310: 806B0038  lwz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E4B314: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B318: 419A0014  beq cr6, 0x82e4b32c
	if ctx.cr[6].eq {
	pc = 0x82E4B32C; continue 'dispatch;
	}
	// 82E4B31C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4B320: 4BF0D6F1  bl 0x82d58a10
	ctx.lr = 0x82E4B324;
	sub_82D58A10(ctx, base);
	// 82E4B324: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E4B328: 419A0024  beq cr6, 0x82e4b34c
	if ctx.cr[6].eq {
	pc = 0x82E4B34C; continue 'dispatch;
	}
	// 82E4B32C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B330: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B334: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B338: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B33C: 4198FFCC  blt cr6, 0x82e4b308
	if ctx.cr[6].lt {
	pc = 0x82E4B308; continue 'dispatch;
	}
	// 82E4B340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4B344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B348: 4BE5E110  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E4B34C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B350: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4B354: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B35C: 4BE5E0FC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B360 size=168
    let mut pc: u32 = 0x82E4B360;
    'dispatch: loop {
        match pc {
            0x82E4B360 => {
    //   block [0x82E4B360..0x82E4B408)
	// 82E4B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B364: 4BE5E095  bl 0x82ca93f8
	ctx.lr = 0x82E4B368;
	sub_82CA93D0(ctx, base);
	// 82E4B368: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B36C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E4B370: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4B374: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E4B378: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B37C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B380: 40990070  ble cr6, 0x82e4b3f0
	if !ctx.cr[6].gt {
	pc = 0x82E4B3F0; continue 'dispatch;
	}
	// 82E4B384: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E4B388: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B38C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4B390: 7FCBD82E  lwzx r30, r11, r27
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4B394: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B39C: 40990040  ble cr6, 0x82e4b3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4B3DC; continue 'dispatch;
	}
	// 82E4B3A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4B3A4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B3A8: 7FBF582E  lwzx r29, r31, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B3AC: 807D0070  lwz r3, 0x70(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E4B3B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B3B4: 419A0014  beq cr6, 0x82e4b3c8
	if ctx.cr[6].eq {
	pc = 0x82E4B3C8; continue 'dispatch;
	}
	// 82E4B3B8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4B3BC: 4BF0D655  bl 0x82d58a10
	ctx.lr = 0x82E4B3C0;
	sub_82D58A10(ctx, base);
	// 82E4B3C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E4B3C4: 419A0038  beq cr6, 0x82e4b3fc
	if ctx.cr[6].eq {
	pc = 0x82E4B3FC; continue 'dispatch;
	}
	// 82E4B3C8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B3CC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E4B3D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B3D4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B3D8: 4198FFCC  blt cr6, 0x82e4b3a4
	if ctx.cr[6].lt {
	pc = 0x82E4B3A4; continue 'dispatch;
	}
	// 82E4B3DC: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B3E0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82E4B3E4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E4B3E8: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B3EC: 4198FF9C  blt cr6, 0x82e4b388
	if ctx.cr[6].lt {
	pc = 0x82E4B388; continue 'dispatch;
	}
	// 82E4B3F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4B3F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4B3F8: 4BE5E050  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82E4B3FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B400: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4B404: 4BE5E044  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4B408 size=48
    let mut pc: u32 = 0x82E4B408;
    'dispatch: loop {
        match pc {
            0x82E4B408 => {
    //   block [0x82E4B408..0x82E4B438)
	// 82E4B408: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B40C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E4B410: 394B5D20  addi r10, r11, 0x5d20
	ctx.r[10].s64 = ctx.r[11].s64 + 23840;
	// 82E4B414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4B418: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82E4B41C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E4B420: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4B424: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4B428: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E4B42C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4B430: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4B434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B438 size=264
    let mut pc: u32 = 0x82E4B438;
    'dispatch: loop {
        match pc {
            0x82E4B438 => {
    //   block [0x82E4B438..0x82E4B540)
	// 82E4B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B43C: 4BE5DFD1  bl 0x82ca940c
	ctx.lr = 0x82E4B440;
	sub_82CA93D0(ctx, base);
	// 82E4B440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B44C: 396B5D20  addi r11, r11, 0x5d20
	ctx.r[11].s64 = ctx.r[11].s64 + 23840;
	// 82E4B450: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4B458: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B45C: 419A003C  beq cr6, 0x82e4b498
	if ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B460: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B464: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B468: 419A0030  beq cr6, 0x82e4b498
	if ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B46C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4B470: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4B474: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4B478: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B47C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4B480: 409A0018  bne cr6, 0x82e4b498
	if !ctx.cr[6].eq {
	pc = 0x82E4B498; continue 'dispatch;
	}
	// 82E4B484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B488: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B48C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B494: 4E800421  bctrl
	ctx.lr = 0x82E4B498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B498: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B49C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B4A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B4A4: 4099005C  ble cr6, 0x82e4b500
	if !ctx.cr[6].gt {
	pc = 0x82E4B500; continue 'dispatch;
	}
	// 82E4B4A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4B4AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B4B0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4B4B4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B4B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B4BC: 419A0030  beq cr6, 0x82e4b4ec
	if ctx.cr[6].eq {
	pc = 0x82E4B4EC; continue 'dispatch;
	}
	// 82E4B4C0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4B4C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4B4C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4B4CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B4D0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4B4D4: 409A0018  bne cr6, 0x82e4b4ec
	if !ctx.cr[6].eq {
	pc = 0x82E4B4EC; continue 'dispatch;
	}
	// 82E4B4D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B4DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B4E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B4E8: 4E800421  bctrl
	ctx.lr = 0x82E4B4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B4EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4B4F0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B4F4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B4F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B4FC: 4198FFB0  blt cr6, 0x82e4b4ac
	if ctx.cr[6].lt {
	pc = 0x82E4B4AC; continue 'dispatch;
	}
	// 82E4B500: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B504: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4B508: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4B50C: 409A0020  bne cr6, 0x82e4b52c
	if !ctx.cr[6].eq {
	pc = 0x82E4B52C; continue 'dispatch;
	}
	// 82E4B510: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B514: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4B518: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4B51C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B520: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4B524: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4B528: 4BF09DA1  bl 0x82d552c8
	ctx.lr = 0x82E4B52C;
	sub_82D552C8(ctx, base);
	// 82E4B52C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4B530: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4B534: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4B538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4B53C: 4BE5DF20  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B540 size=256
    let mut pc: u32 = 0x82E4B540;
    'dispatch: loop {
        match pc {
            0x82E4B540 => {
    //   block [0x82E4B540..0x82E4B640)
	// 82E4B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B544: 4BE5DEC5  bl 0x82ca9408
	ctx.lr = 0x82E4B548;
	sub_82CA93D0(ctx, base);
	// 82E4B548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B54C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B550: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4B554: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4B558: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B560: 409A002C  bne cr6, 0x82e4b58c
	if !ctx.cr[6].eq {
	pc = 0x82E4B58C; continue 'dispatch;
	}
	// 82E4B564: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B56C: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82E4B570: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82E4B574: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B578: 4BF09CD1  bl 0x82d55248
	ctx.lr = 0x82E4B57C;
	sub_82D55248(ctx, base);
	// 82E4B57C: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82E4B580: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B584: 4BF364AD  bl 0x82d81a30
	ctx.lr = 0x82E4B588;
	sub_82D81A30(ctx, base);
	// 82E4B588: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4B58C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B590: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B594: 4BF2D1A5  bl 0x82d78738
	ctx.lr = 0x82E4B598;
	sub_82D78738(ctx, base);
	// 82E4B598: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82E4B59C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4B5A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4B5A4: 4BF30285  bl 0x82d7b828
	ctx.lr = 0x82E4B5A8;
	sub_82D7B828(ctx, base);
	// 82E4B5A8: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E4B5AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4B5B0: 419A0088  beq cr6, 0x82e4b638
	if ctx.cr[6].eq {
	pc = 0x82E4B638; continue 'dispatch;
	}
	// 82E4B5B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B5B8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4B5BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4B5C0: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B5C4: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82E4B5C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B5CC: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82E4B5D0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82E4B5D4: 4BF09C75  bl 0x82d55248
	ctx.lr = 0x82E4B5D8;
	sub_82D55248(ctx, base);
	// 82E4B5D8: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82E4B5DC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B5E0: 48000759  bl 0x82e4bd38
	ctx.lr = 0x82E4B5E4;
	sub_82E4BD38(ctx, base);
	// 82E4B5E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4B5E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4B5EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B5F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4B5F4: 4800CD45  bl 0x82e58338
	ctx.lr = 0x82E4B5F8;
	sub_82E58338(ctx, base);
	// 82E4B5F8: 9BBE0040  stb r29, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 82E4B5FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B600: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B604: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4B608: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B60C: 409A0010  bne cr6, 0x82e4b61c
	if !ctx.cr[6].eq {
	pc = 0x82E4B61C; continue 'dispatch;
	}
	// 82E4B610: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4B614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B618: 4BF0B981  bl 0x82d56f98
	ctx.lr = 0x82E4B61C;
	sub_82D56F98(ctx, base);
	// 82E4B61C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B620: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B624: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4B628: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E4B62C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B630: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4B634: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4B638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4B63C: 4BE5DE1C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4B640 size=1780
    let mut pc: u32 = 0x82E4B640;
    'dispatch: loop {
        match pc {
            0x82E4B640 => {
    //   block [0x82E4B640..0x82E4BD34)
	// 82E4B640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4B644: 4BE5DD8D  bl 0x82ca93d0
	ctx.lr = 0x82E4B648;
	sub_82CA93D0(ctx, base);
	// 82E4B648: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4B64C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B650: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4B654: 39C00004  li r14, 4
	ctx.r[14].s64 = 4;
	// 82E4B658: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B65C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B664: 9301012C  stw r24, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[24].u32 ) };
	// 82E4B668: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B66C: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82E4B670: 4BF09BD9  bl 0x82d55248
	ctx.lr = 0x82E4B674;
	sub_82D55248(ctx, base);
	// 82E4B674: 3B400044  li r26, 0x44
	ctx.r[26].s64 = 68;
	// 82E4B678: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B67C: 4BF377ED  bl 0x82d82e68
	ctx.lr = 0x82E4B680;
	sub_82D82E68(ctx, base);
	// 82E4B680: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B684: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4B688: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E4B68C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B690: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B694: 4099002C  ble cr6, 0x82e4b6c0
	if !ctx.cr[6].gt {
	pc = 0x82E4B6C0; continue 'dispatch;
	}
	// 82E4B698: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B69C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B6A0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B6A4: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B6A8: 4BF37D41  bl 0x82d833e8
	ctx.lr = 0x82E4B6AC;
	sub_82D833E8(ctx, base);
	// 82E4B6AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B6B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B6B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E4B6B8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B6BC: 4198FFE0  blt cr6, 0x82e4b69c
	if ctx.cr[6].lt {
	pc = 0x82E4B69C; continue 'dispatch;
	}
	// 82E4B6C0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4B6C4: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B6C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B6CC: 4099002C  ble cr6, 0x82e4b6f8
	if !ctx.cr[6].gt {
	pc = 0x82E4B6F8; continue 'dispatch;
	}
	// 82E4B6D0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B6D4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4B6D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B6DC: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B6E0: 4BF37EA9  bl 0x82d83588
	ctx.lr = 0x82E4B6E4;
	sub_82D83588(ctx, base);
	// 82E4B6E4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4B6E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B6EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B6F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B6F4: 4198FFE0  blt cr6, 0x82e4b6d4
	if ctx.cr[6].lt {
	pc = 0x82E4B6D4; continue 'dispatch;
	}
	// 82E4B6F8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B6FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B704: 4099002C  ble cr6, 0x82e4b730
	if !ctx.cr[6].gt {
	pc = 0x82E4B730; continue 'dispatch;
	}
	// 82E4B708: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B70C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B710: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B714: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B718: 4BF37DE1  bl 0x82d834f8
	ctx.lr = 0x82E4B71C;
	sub_82D834F8(ctx, base);
	// 82E4B71C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B720: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B724: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B728: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B72C: 4198FFE0  blt cr6, 0x82e4b70c
	if ctx.cr[6].lt {
	pc = 0x82E4B70C; continue 'dispatch;
	}
	// 82E4B730: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4B734: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B738: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B73C: 4099002C  ble cr6, 0x82e4b768
	if !ctx.cr[6].gt {
	pc = 0x82E4B768; continue 'dispatch;
	}
	// 82E4B740: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B744: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4B748: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B74C: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4B750: 4BF37D21  bl 0x82d83470
	ctx.lr = 0x82E4B754;
	sub_82D83470(ctx, base);
	// 82E4B754: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4B758: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4B75C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4B760: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B764: 4198FFE0  blt cr6, 0x82e4b744
	if ctx.cr[6].lt {
	pc = 0x82E4B744; continue 'dispatch;
	}
	// 82E4B768: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B76C: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B770: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B774: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82E4B778: 4BF09AD1  bl 0x82d55248
	ctx.lr = 0x82E4B77C;
	sub_82D55248(ctx, base);
	// 82E4B77C: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B780: 4BF376E9  bl 0x82d82e68
	ctx.lr = 0x82E4B784;
	sub_82D82E68(ctx, base);
	// 82E4B784: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4B78C: 396B71B0  addi r11, r11, 0x71b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29104;
	// 82E4B790: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B794: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B798: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B79C: 9B9F0040  stb r28, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B7A0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B7A4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B7A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B7AC: 40990048  ble cr6, 0x82e4b7f4
	if !ctx.cr[6].gt {
	pc = 0x82E4B7F4; continue 'dispatch;
	}
	// 82E4B7B0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82E4B7B4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B7B8: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B7BC: 896400D8  lbz r11, 0xd8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4B7C0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4B7C4: 409A001C  bne cr6, 0x82e4b7e0
	if !ctx.cr[6].eq {
	pc = 0x82E4B7E0; continue 'dispatch;
	}
	// 82E4B7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B7CC: 4BF37C1D  bl 0x82d833e8
	ctx.lr = 0x82E4B7D0;
	sub_82D833E8(ctx, base);
	// 82E4B7D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B7D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B7D8: 4BF374B1  bl 0x82d82c88
	ctx.lr = 0x82E4B7DC;
	sub_82D82C88(ctx, base);
	// 82E4B7DC: 4800000C  b 0x82e4b7e8
	pc = 0x82E4B7E8; continue 'dispatch;
	// 82E4B7E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B7E4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E4B7E8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B7EC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B7F0: 4198FFC4  blt cr6, 0x82e4b7b4
	if ctx.cr[6].lt {
	pc = 0x82E4B7B4; continue 'dispatch;
	}
	// 82E4B7F4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B7F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B7FC: 419A000C  beq cr6, 0x82e4b808
	if ctx.cr[6].eq {
	pc = 0x82E4B808; continue 'dispatch;
	}
	// 82E4B800: 93F80000  stw r31, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E4B804: 48000020  b 0x82e4b824
	pc = 0x82E4B824; continue 'dispatch;
	// 82E4B808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B80C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4B810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4B814: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4B81C: 4E800421  bctrl
	ctx.lr = 0x82E4B820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4B820: 93980000  stw r28, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4B824: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B828: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B82C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B830: 4BF09A19  bl 0x82d55248
	ctx.lr = 0x82E4B834;
	sub_82D55248(ctx, base);
	// 82E4B834: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B838: 4BF37631  bl 0x82d82e68
	ctx.lr = 0x82E4B83C;
	sub_82D82E68(ctx, base);
	// 82E4B83C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B840: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B844: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E4B848: 396B7194  addi r11, r11, 0x7194
	ctx.r[11].s64 = ctx.r[11].s64 + 29076;
	// 82E4B84C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B850: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B854: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B858: 9B830040  stb r28, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B85C: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B860: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B864: 4BF099E5  bl 0x82d55248
	ctx.lr = 0x82E4B868;
	sub_82D55248(ctx, base);
	// 82E4B868: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82E4B86C: 4BF375FD  bl 0x82d82e68
	ctx.lr = 0x82E4B870;
	sub_82D82E68(ctx, base);
	// 82E4B870: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B874: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82E4B878: 3A590014  addi r18, r25, 0x14
	ctx.r[18].s64 = ctx.r[25].s64 + 20;
	// 82E4B87C: 396B717C  addi r11, r11, 0x717c
	ctx.r[11].s64 = ctx.r[11].s64 + 29052;
	// 82E4B880: 3A790020  addi r19, r25, 0x20
	ctx.r[19].s64 = ctx.r[25].s64 + 32;
	// 82E4B884: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B888: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B88C: 9B830040  stb r28, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[28].u8 ) };
	// 82E4B890: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B894: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B898: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B89C: 40990388  ble cr6, 0x82e4bc24
	if !ctx.cr[6].gt {
	pc = 0x82E4BC24; continue 'dispatch;
	}
	// 82E4B8A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4B8A4: 3E008000  lis r16, -0x8000
	ctx.r[16].s64 = -2147483648;
	// 82E4B8A8: 396B7168  addi r11, r11, 0x7168
	ctx.r[11].s64 = ctx.r[11].s64 + 29032;
	// 82E4B8AC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E4B8B0: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4B8B4: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4B8B8: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4B8BC: 4BF0998D  bl 0x82d55248
	ctx.lr = 0x82E4B8C0;
	sub_82D55248(ctx, base);
	// 82E4B8C0: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 82E4B8C4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4B8C8: 4BF375A1  bl 0x82d82e68
	ctx.lr = 0x82E4B8CC;
	sub_82D82E68(ctx, base);
	// 82E4B8CC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4B8D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4B8D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4B8D8: 917D0038  stw r11, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4B8DC: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4B8E0: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4B8E4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8E8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8EC: 4BF3996D  bl 0x82d85258
	ctx.lr = 0x82E4B8F0;
	sub_82D85258(ctx, base);
	// 82E4B8F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4B8F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B8F8: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B8FC: 997D0040  stb r11, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4B900: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B904: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B908: 4BF37AE1  bl 0x82d833e8
	ctx.lr = 0x82E4B90C;
	sub_82D833E8(ctx, base);
	// 82E4B90C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4B910: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B914: 4BF37375  bl 0x82d82c88
	ctx.lr = 0x82E4B918;
	sub_82D82C88(ctx, base);
	// 82E4B918: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4B91C: 7F94E378  mr r20, r28
	ctx.r[20].u64 = ctx.r[28].u64;
	// 82E4B920: 7F8FE378  mr r15, r28
	ctx.r[15].u64 = ctx.r[28].u64;
	// 82E4B924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B928: 409901F4  ble cr6, 0x82e4bb1c
	if !ctx.cr[6].gt {
	pc = 0x82E4BB1C; continue 'dispatch;
	}
	// 82E4B92C: 7F91E378  mr r17, r28
	ctx.r[17].u64 = ctx.r[28].u64;
	// 82E4B930: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4B934: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4B938: 81520004  lwz r10, 4(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B93C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4B940: 7EAB882E  lwzx r21, r11, r17
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82E4B944: 40990090  ble cr6, 0x82e4b9d4
	if !ctx.cr[6].gt {
	pc = 0x82E4B9D4; continue 'dispatch;
	}
	// 82E4B948: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E4B94C: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B950: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B954: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B958: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4B95C: 419A001C  beq cr6, 0x82e4b978
	if ctx.cr[6].eq {
	pc = 0x82E4B978; continue 'dispatch;
	}
	// 82E4B960: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B964: 7F0BA840  cmplw cr6, r11, r21
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4B968: 419A0010  beq cr6, 0x82e4b978
	if ctx.cr[6].eq {
	pc = 0x82E4B978; continue 'dispatch;
	}
	// 82E4B96C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4B970: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4B974: 48000054  b 0x82e4b9c8
	pc = 0x82E4B9C8; continue 'dispatch;
	// 82E4B978: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4B97C: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82E4B980: 4BF37B79  bl 0x82d834f8
	ctx.lr = 0x82E4B984;
	sub_82D834F8(ctx, base);
	// 82E4B984: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B988: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4B98C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4B990: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4B994: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82E4B998: 7D65AA78  xor r5, r11, r21
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[21].u64;
	// 82E4B99C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E4B9A0: 419A001C  beq cr6, 0x82e4b9bc
	if ctx.cr[6].eq {
	pc = 0x82E4B9BC; continue 'dispatch;
	}
	// 82E4B9A4: 896500D8  lbz r11, 0xd8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4B9A8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4B9AC: 419A0010  beq cr6, 0x82e4b9bc
	if ctx.cr[6].eq {
	pc = 0x82E4B9BC; continue 'dispatch;
	}
	// 82E4B9B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4B9B4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B9B8: 4BFFF841  bl 0x82e4b1f8
	ctx.lr = 0x82E4B9BC;
	sub_82E4B1F8(ctx, base);
	// 82E4B9BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4B9C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4B9C4: 4BF37385  bl 0x82d82d48
	ctx.lr = 0x82E4B9C8;
	sub_82D82D48(ctx, base);
	// 82E4B9C8: 81720004  lwz r11, 4(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B9CC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4B9D0: 4198FF7C  blt cr6, 0x82e4b94c
	if ctx.cr[6].lt {
	pc = 0x82E4B94C; continue 'dispatch;
	}
	// 82E4B9D4: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4B9D8: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 82E4B9DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4B9E0: 40990128  ble cr6, 0x82e4bb08
	if !ctx.cr[6].gt {
	pc = 0x82E4BB08; continue 'dispatch;
	}
	// 82E4B9E4: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82E4B9E8: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82E4B9EC: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E4B9F0: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82E4B9F4: 92010070  stw r16, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[16].u32 ) };
	// 82E4B9F8: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4B9FC: 7C7A582E  lwzx r3, r26, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BA00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BA04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4BA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BA0C: 4E800421  bctrl
	ctx.lr = 0x82E4BA10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BA10: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BA14: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4BA18: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82E4BA1C: 7F96E378  mr r22, r28
	ctx.r[22].u64 = ctx.r[28].u64;
	// 82E4BA20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BA24: 409900A4  ble cr6, 0x82e4bac8
	if !ctx.cr[6].gt {
	pc = 0x82E4BAC8; continue 'dispatch;
	}
	// 82E4BA28: 7F97E378  mr r23, r28
	ctx.r[23].u64 = ctx.r[28].u64;
	// 82E4BA2C: 7D57202E  lwzx r10, r23, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E4BA30: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4BA34: 409A0080  bne cr6, 0x82e4bab4
	if !ctx.cr[6].eq {
	pc = 0x82E4BAB4; continue 'dispatch;
	}
	// 82E4BA38: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BA3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4BA40: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82E4BA44: 7C9A582E  lwzx r4, r26, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BA48: 4BF37B41  bl 0x82d83588
	ctx.lr = 0x82E4BA4C;
	sub_82D83588(ctx, base);
	// 82E4BA4C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4BA50: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BA54: 4BF37385  bl 0x82d82dd8
	ctx.lr = 0x82E4BA58;
	sub_82D82DD8(ctx, base);
	// 82E4BA58: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BA5C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4BA60: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E4BA64: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4BA68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BA6C: 40990048  ble cr6, 0x82e4bab4
	if !ctx.cr[6].gt {
	pc = 0x82E4BAB4; continue 'dispatch;
	}
	// 82E4BA70: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E4BA74: 7D5F202E  lwzx r10, r31, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E4BA78: 7F0AA840  cmplw cr6, r10, r21
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[21].u32, &mut ctx.xer);
	// 82E4BA7C: 419A0028  beq cr6, 0x82e4baa4
	if ctx.cr[6].eq {
	pc = 0x82E4BAA4; continue 'dispatch;
	}
	// 82E4BA80: 5545003E  slwi r5, r10, 0
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BA84: 894500D8  lbz r10, 0xd8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4BA88: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82E4BA8C: 419A0018  beq cr6, 0x82e4baa4
	if ctx.cr[6].eq {
	pc = 0x82E4BAA4; continue 'dispatch;
	}
	// 82E4BA90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4BA94: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BA98: 4BFFF761  bl 0x82e4b1f8
	ctx.lr = 0x82E4BA9C;
	sub_82E4B1F8(ctx, base);
	// 82E4BA9C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E4BAA0: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4BAA4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4BAA8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4BAAC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BAB0: 4198FFC4  blt cr6, 0x82e4ba74
	if ctx.cr[6].lt {
	pc = 0x82E4BA74; continue 'dispatch;
	}
	// 82E4BAB4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82E4BAB8: 3AF70004  addi r23, r23, 4
	ctx.r[23].s64 = ctx.r[23].s64 + 4;
	// 82E4BABC: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BAC0: 4198FF6C  blt cr6, 0x82e4ba2c
	if ctx.cr[6].lt {
	pc = 0x82E4BA2C; continue 'dispatch;
	}
	// 82E4BAC4: 82E10060  lwz r23, 0x60(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4BAC8: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82E4BACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BAD0: 409A000C  bne cr6, 0x82e4badc
	if !ctx.cr[6].eq {
	pc = 0x82E4BADC; continue 'dispatch;
	}
	// 82E4BAD4: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82E4BAD8: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 82E4BADC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E4BAE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4BAE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BAE8: 409A0014  bne cr6, 0x82e4bafc
	if !ctx.cr[6].eq {
	pc = 0x82E4BAFC; continue 'dispatch;
	}
	// 82E4BAEC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4BAF0: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4BAF4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BAF8: 4BF097D1  bl 0x82d552c8
	ctx.lr = 0x82E4BAFC;
	sub_82D552C8(ctx, base);
	// 82E4BAFC: 81730004  lwz r11, 4(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BB00: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BB04: 4198FEE4  blt cr6, 0x82e4b9e8
	if ctx.cr[6].lt {
	pc = 0x82E4B9E8; continue 'dispatch;
	}
	// 82E4BB08: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BB0C: 39EF0001  addi r15, r15, 1
	ctx.r[15].s64 = ctx.r[15].s64 + 1;
	// 82E4BB10: 3A310004  addi r17, r17, 4
	ctx.r[17].s64 = ctx.r[17].s64 + 4;
	// 82E4BB14: 7F0F5800  cmpw cr6, r15, r11
	ctx.cr[6].compare_i32(ctx.r[15].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BB18: 4198FE18  blt cr6, 0x82e4b930
	if ctx.cr[6].lt {
	pc = 0x82E4B930; continue 'dispatch;
	}
	// 82E4BB1C: 568B063E  clrlwi r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	// 82E4BB20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB24: 409AFDF4  bne cr6, 0x82e4b918
	if !ctx.cr[6].eq {
	pc = 0x82E4B918; continue 'dispatch;
	}
	// 82E4BB28: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4BB2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB30: 409A00A0  bne cr6, 0x82e4bbd0
	if !ctx.cr[6].eq {
	pc = 0x82E4BBD0; continue 'dispatch;
	}
	// 82E4BB34: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4BB38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB3C: 409A0094  bne cr6, 0x82e4bbd0
	if !ctx.cr[6].eq {
	pc = 0x82E4BBD0; continue 'dispatch;
	}
	// 82E4BB40: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BB44: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BB48: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E4BB4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BB50: 419A0020  beq cr6, 0x82e4bb70
	if ctx.cr[6].eq {
	pc = 0x82E4BB70; continue 'dispatch;
	}
	// 82E4BB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BB58: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82E4BB5C: 4BF396FD  bl 0x82d85258
	ctx.lr = 0x82E4BB60;
	sub_82D85258(ctx, base);
	// 82E4BB60: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BB64: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E4BB68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB6C: 419A0008  beq cr6, 0x82e4bb74
	if ctx.cr[6].eq {
	pc = 0x82E4BB74; continue 'dispatch;
	}
	// 82E4BB70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4BB74: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4BB78: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4BB7C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E4BB80: 419A0008  beq cr6, 0x82e4bb88
	if ctx.cr[6].eq {
	pc = 0x82E4BB88; continue 'dispatch;
	}
	// 82E4BB84: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4BB88: 89630040  lbz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E4BB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB90: 409A0014  bne cr6, 0x82e4bba4
	if !ctx.cr[6].eq {
	pc = 0x82E4BBA4; continue 'dispatch;
	}
	// 82E4BB94: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E4BB98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BB9C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E4BBA0: 419A0008  beq cr6, 0x82e4bba8
	if ctx.cr[6].eq {
	pc = 0x82E4BBA8; continue 'dispatch;
	}
	// 82E4BBA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4BBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BBAC: 99630040  stb r11, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82E4BBB0: 4BF37839  bl 0x82d833e8
	ctx.lr = 0x82E4BBB4;
	sub_82D833E8(ctx, base);
	// 82E4BBB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BBB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BBBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4BBC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BBC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BBC8: 4E800421  bctrl
	ctx.lr = 0x82E4BBCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BBCC: 48000048  b 0x82e4bc14
	pc = 0x82E4BC14; continue 'dispatch;
	// 82E4BBD0: 8161012C  lwz r11, 0x12c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BBD4: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82E4BBD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BBDC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BBE0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BBE4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BBE8: 409A0010  bne cr6, 0x82e4bbf8
	if !ctx.cr[6].eq {
	pc = 0x82E4BBF8; continue 'dispatch;
	}
	// 82E4BBEC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BBF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BBF4: 4BF0B3A5  bl 0x82d56f98
	ctx.lr = 0x82E4BBF8;
	sub_82D56F98(ctx, base);
	// 82E4BBF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BBFC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BC04: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E4BC08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BC10: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4BC14: 3B790008  addi r27, r25, 8
	ctx.r[27].s64 = ctx.r[25].s64 + 8;
	// 82E4BC18: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC20: 4199FC90  bgt cr6, 0x82e4b8b0
	if ctx.cr[6].gt {
	pc = 0x82E4B8B0; continue 'dispatch;
	}
	// 82E4BC24: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4BC28: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BC2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC30: 419A0010  beq cr6, 0x82e4bc40
	if ctx.cr[6].eq {
	pc = 0x82E4BC40; continue 'dispatch;
	}
	// 82E4BC34: 83E1012C  lwz r31, 0x12c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BC38: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4BC3C: 48000020  b 0x82e4bc5c
	pc = 0x82E4BC5C; continue 'dispatch;
	// 82E4BC40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BC48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BC50: 4E800421  bctrl
	ctx.lr = 0x82E4BC54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BC54: 83E1012C  lwz r31, 0x12c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BC58: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E4BC5C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4BC60: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BC64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC68: 419A000C  beq cr6, 0x82e4bc74
	if ctx.cr[6].eq {
	pc = 0x82E4BC74; continue 'dispatch;
	}
	// 82E4BC6C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4BC70: 4800001C  b 0x82e4bc8c
	pc = 0x82E4BC8C; continue 'dispatch;
	// 82E4BC74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BC7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BC80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BC84: 4E800421  bctrl
	ctx.lr = 0x82E4BC88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BC88: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E4BC8C: 3BB9002C  addi r29, r25, 0x2c
	ctx.r[29].s64 = ctx.r[25].s64 + 44;
	// 82E4BC90: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BC94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BC98: 40990078  ble cr6, 0x82e4bd10
	if !ctx.cr[6].gt {
	pc = 0x82E4BD10; continue 'dispatch;
	}
	// 82E4BC9C: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82E4BCA0: 7C6EB82E  lwzx r3, r14, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[14].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82E4BCA4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E4BCA8: 4BF095A1  bl 0x82d55248
	ctx.lr = 0x82E4BCAC;
	sub_82D55248(ctx, base);
	// 82E4BCAC: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 82E4BCB0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4BCB4: 4BF371B5  bl 0x82d82e68
	ctx.lr = 0x82E4BCB8;
	sub_82D82E68(ctx, base);
	// 82E4BCB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BCBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BCC0: 396B891C  addi r11, r11, -0x76e4
	ctx.r[11].s64 = ctx.r[11].s64 + -30436;
	// 82E4BCC4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E4BCC8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4BCCC: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E4BCD0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4BCD4: 8161012C  lwz r11, 0x12c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 82E4BCD8: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4BCDC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BCE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4BCE4: 40990030  ble cr6, 0x82e4bd14
	if !ctx.cr[6].gt {
	pc = 0x82E4BD14; continue 'dispatch;
	}
	// 82E4BCE8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BCF0: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BCF4: 4BF3777D  bl 0x82d83470
	ctx.lr = 0x82E4BCF8;
	sub_82D83470(ctx, base);
	// 82E4BCF8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BCFC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4BD00: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E4BD04: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BD08: 4198FFE0  blt cr6, 0x82e4bce8
	if ctx.cr[6].lt {
	pc = 0x82E4BCE8; continue 'dispatch;
	}
	// 82E4BD0C: 48000008  b 0x82e4bd14
	pc = 0x82E4BD14; continue 'dispatch;
	// 82E4BD10: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E4BD14: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BD18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BD1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4BD20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BD24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BD28: 4E800421  bctrl
	ctx.lr = 0x82E4BD2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BD2C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82E4BD30: 4BE5D6F0  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BD38 size=80
    let mut pc: u32 = 0x82E4BD38;
    'dispatch: loop {
        match pc {
            0x82E4BD38 => {
    //   block [0x82E4BD38..0x82E4BD88)
	// 82E4BD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4BD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4BD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BD4C: 4BF3711D  bl 0x82d82e68
	ctx.lr = 0x82E4BD50;
	sub_82D82E68(ctx, base);
	// 82E4BD50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BD54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4BD58: 396B6950  addi r11, r11, 0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + 26960;
	// 82E4BD5C: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E4BD60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BD64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BD68: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82E4BD6C: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E4BD70: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82E4BD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4BD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4BD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4BD80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BD88 size=168
    let mut pc: u32 = 0x82E4BD88;
    'dispatch: loop {
        match pc {
            0x82E4BD88 => {
    //   block [0x82E4BD88..0x82E4BE30)
	// 82E4BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BD8C: 4BE5D681  bl 0x82ca940c
	ctx.lr = 0x82E4BD90;
	sub_82CA93D0(ctx, base);
	// 82E4BD90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BD94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4BD98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4BD9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4BDA0: 396B71E4  addi r11, r11, 0x71e4
	ctx.r[11].s64 = ctx.r[11].s64 + 29156;
	// 82E4BDA4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BDA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BDAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BDB0: 40990040  ble cr6, 0x82e4bdf0
	if !ctx.cr[6].gt {
	pc = 0x82E4BDF0; continue 'dispatch;
	}
	// 82E4BDB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4BDB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BDBC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4BDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4BDC4: 419A0018  beq cr6, 0x82e4bddc
	if ctx.cr[6].eq {
	pc = 0x82E4BDDC; continue 'dispatch;
	}
	// 82E4BDC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BDCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4BDD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BDD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4BDD8: 4E800421  bctrl
	ctx.lr = 0x82E4BDDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4BDDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4BDE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4BDE4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4BDE8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BDEC: 4198FFCC  blt cr6, 0x82e4bdb8
	if ctx.cr[6].lt {
	pc = 0x82E4BDB8; continue 'dispatch;
	}
	// 82E4BDF0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4BDF4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4BDF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4BDFC: 409A0020  bne cr6, 0x82e4be1c
	if !ctx.cr[6].eq {
	pc = 0x82E4BE1C; continue 'dispatch;
	}
	// 82E4BE00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE04: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4BE08: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4BE0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BE10: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4BE14: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4BE18: 4BF094B1  bl 0x82d552c8
	ctx.lr = 0x82E4BE1C;
	sub_82D552C8(ctx, base);
	// 82E4BE1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4BE20: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E4BE24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4BE28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4BE2C: 4BE5D630  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4BE30 size=632
    let mut pc: u32 = 0x82E4BE30;
    'dispatch: loop {
        match pc {
            0x82E4BE30 => {
    //   block [0x82E4BE30..0x82E4C0A8)
	// 82E4BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4BE34: 4BE5D5C9  bl 0x82ca93fc
	ctx.lr = 0x82E4BE38;
	sub_82CA93D0(ctx, base);
	// 82E4BE38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4BE3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4BE40: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BE44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E4BE48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4BE4C: 388BA314  addi r4, r11, -0x5cec
	ctx.r[4].s64 = ctx.r[11].s64 + -23788;
	// 82E4BE50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BE54: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE58: 4BF09939  bl 0x82d55790
	ctx.lr = 0x82E4BE5C;
	sub_82D55790(ctx, base);
	// 82E4BE5C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE60: 3B200070  li r25, 0x70
	ctx.r[25].s64 = 112;
	// 82E4BE64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BE68: 419A008C  beq cr6, 0x82e4bef4
	if ctx.cr[6].eq {
	pc = 0x82E4BEF4; continue 'dispatch;
	}
	// 82E4BE6C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BE70: 3B4B9DD4  addi r26, r11, -0x622c
	ctx.r[26].s64 = ctx.r[11].s64 + -25132;
	// 82E4BE74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE78: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BE7C: 419A0078  beq cr6, 0x82e4bef4
	if ctx.cr[6].eq {
	pc = 0x82E4BEF4; continue 'dispatch;
	}
	// 82E4BE80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE84: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BE88: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BE8C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BE90: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E4BE94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BE98: 4BF093B1  bl 0x82d55248
	ctx.lr = 0x82E4BE9C;
	sub_82D55248(ctx, base);
	// 82E4BE9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BEA0: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 82E4BEA4: 4BF796A5  bl 0x82dc5548
	ctx.lr = 0x82E4BEA8;
	sub_82DC5548(ctx, base);
	// 82E4BEA8: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4BEAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4BEB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BEB4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BEB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BEBC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BEC0: 409A0010  bne cr6, 0x82e4bed0
	if !ctx.cr[6].eq {
	pc = 0x82E4BED0; continue 'dispatch;
	}
	// 82E4BEC4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BECC: 4BF0B0CD  bl 0x82d56f98
	ctx.lr = 0x82E4BED0;
	sub_82D56F98(ctx, base);
	// 82E4BED0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BED4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BED8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BEDC: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4BEE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BEE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BEE8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4BEEC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4BEF0: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E4BEF4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BEF8: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BEFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BF00: 388BA114  addi r4, r11, -0x5eec
	ctx.r[4].s64 = ctx.r[11].s64 + -24300;
	// 82E4BF04: 4BF0988D  bl 0x82d55790
	ctx.lr = 0x82E4BF08;
	sub_82D55790(ctx, base);
	// 82E4BF08: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BF10: 419A0098  beq cr6, 0x82e4bfa8
	if ctx.cr[6].eq {
	pc = 0x82E4BFA8; continue 'dispatch;
	}
	// 82E4BF14: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BF18: 3B4B9D24  addi r26, r11, -0x62dc
	ctx.r[26].s64 = ctx.r[11].s64 + -25308;
	// 82E4BF1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF20: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BF24: 419A0084  beq cr6, 0x82e4bfa8
	if ctx.cr[6].eq {
	pc = 0x82E4BFA8; continue 'dispatch;
	}
	// 82E4BF28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF2C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BF30: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BF34: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF38: 38800130  li r4, 0x130
	ctx.r[4].s64 = 304;
	// 82E4BF3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BF40: 4BF09309  bl 0x82d55248
	ctx.lr = 0x82E4BF44;
	sub_82D55248(ctx, base);
	// 82E4BF44: 39600130  li r11, 0x130
	ctx.r[11].s64 = 304;
	// 82E4BF48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BF4C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4BF50: 4BF77D79  bl 0x82dc3cc8
	ctx.lr = 0x82E4BF54;
	sub_82DC3CC8(ctx, base);
	// 82E4BF54: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4BF58: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4BF5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4BF60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4BF68: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4BF6C: 409A00C0  bne cr6, 0x82e4c02c
	if !ctx.cr[6].eq {
	pc = 0x82E4C02C; continue 'dispatch;
	}
	// 82E4BF70: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4BF74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4BF78: 4BF0B021  bl 0x82d56f98
	ctx.lr = 0x82E4BF7C;
	sub_82D56F98(ctx, base);
	// 82E4BF7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF80: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BF84: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4BF88: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4BF8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4BF90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4BF94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4BF98: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4BF9C: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E4BFA0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4BFA4: 4BE5D4A8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E4BFA8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BFAC: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4BFB4: 388BA854  addi r4, r11, -0x57ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22444;
	// 82E4BFB8: 4BF097D9  bl 0x82d55790
	ctx.lr = 0x82E4BFBC;
	sub_82D55790(ctx, base);
	// 82E4BFBC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4BFC4: 419A0094  beq cr6, 0x82e4c058
	if ctx.cr[6].eq {
	pc = 0x82E4C058; continue 'dispatch;
	}
	// 82E4BFC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4BFCC: 3B4B9E04  addi r26, r11, -0x61fc
	ctx.r[26].s64 = ctx.r[11].s64 + -25084;
	// 82E4BFD0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFD4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E4BFD8: 419A0080  beq cr6, 0x82e4c058
	if ctx.cr[6].eq {
	pc = 0x82E4C058; continue 'dispatch;
	}
	// 82E4BFDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFE0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4BFE4: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 82E4BFE8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4BFEC: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E4BFF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4BFF4: 4BF09255  bl 0x82d55248
	ctx.lr = 0x82E4BFF8;
	sub_82D55248(ctx, base);
	// 82E4BFF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4BFFC: B3230004  sth r25, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u16 ) };
	// 82E4C000: 48294901  bl 0x830e0900
	ctx.lr = 0x82E4C004;
	sub_830E0900(ctx, base);
	// 82E4C004: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82E4C008: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4C00C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4C010: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C014: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4C018: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4C01C: 409A0010  bne cr6, 0x82e4c02c
	if !ctx.cr[6].eq {
	pc = 0x82E4C02C; continue 'dispatch;
	}
	// 82E4C020: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4C024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C028: 4BF0AF71  bl 0x82d56f98
	ctx.lr = 0x82E4C02C;
	sub_82D56F98(ctx, base);
	// 82E4C02C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C030: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C034: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4C038: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E4C03C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C040: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4C044: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4C048: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4C04C: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E4C050: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C054: 4BE5D3F8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82E4C058: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C05C: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C064: 388B8FF0  addi r4, r11, -0x7010
	ctx.r[4].s64 = ctx.r[11].s64 + -28688;
	// 82E4C068: 4BF09729  bl 0x82d55790
	ctx.lr = 0x82E4C06C;
	sub_82D55790(ctx, base);
	// 82E4C06C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C074: 419A002C  beq cr6, 0x82e4c0a0
	if ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C078: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C07C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4C080: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4C084: 419A001C  beq cr6, 0x82e4c0a0
	if ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C088: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E4C08C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4C090: 409A0010  bne cr6, 0x82e4c0a0
	if !ctx.cr[6].eq {
	pc = 0x82E4C0A0; continue 'dispatch;
	}
	// 82E4C094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4C098: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4C09C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4C0A0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C0A4: 4BE5D3A8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C0A8 size=408
    let mut pc: u32 = 0x82E4C0A8;
    'dispatch: loop {
        match pc {
            0x82E4C0A8 => {
    //   block [0x82E4C0A8..0x82E4C240)
	// 82E4C0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C0AC: 4BE5D349  bl 0x82ca93f4
	ctx.lr = 0x82E4C0B0;
	sub_82CA93D0(ctx, base);
	// 82E4C0B0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C0B4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82E4C0B8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82E4C0BC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E4C0C0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E4C0C4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E4C0C8: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 82E4C0CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4C0D0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82E4C0D4: 419A015C  beq cr6, 0x82e4c230
	if ctx.cr[6].eq {
	pc = 0x82E4C230; continue 'dispatch;
	}
	// 82E4C0D8: 7CEB0774  extsb r11, r7
	ctx.r[11].s64 = ctx.r[7].s8 as i64;
	// 82E4C0DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C0E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4C0E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C0E8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82E4C0EC: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 82E4C0F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C0F4: 419A0018  beq cr6, 0x82e4c10c
	if ctx.cr[6].eq {
	pc = 0x82E4C10C; continue 'dispatch;
	}
	// 82E4C0F8: 4BF09151  bl 0x82d55248
	ctx.lr = 0x82E4C0FC;
	sub_82D55248(ctx, base);
	// 82E4C0FC: 396000A8  li r11, 0xa8
	ctx.r[11].s64 = 168;
	// 82E4C100: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4C104: 4828944D  bl 0x830d5550
	ctx.lr = 0x82E4C108;
	sub_830D5550(ctx, base);
	// 82E4C108: 48000014  b 0x82e4c11c
	pc = 0x82E4C11C; continue 'dispatch;
	// 82E4C10C: 4BF0913D  bl 0x82d55248
	ctx.lr = 0x82E4C110;
	sub_82D55248(ctx, base);
	// 82E4C110: 396000A8  li r11, 0xa8
	ctx.r[11].s64 = 168;
	// 82E4C114: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E4C118: 48288E69  bl 0x830d4f80
	ctx.lr = 0x82E4C11C;
	sub_830D4F80(ctx, base);
	// 82E4C11C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C120: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82E4C124: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82E4C128: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82E4C12C: 396B71E4  addi r11, r11, 0x71e4
	ctx.r[11].s64 = ctx.r[11].s64 + 29156;
	// 82E4C130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C134: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C138: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E4C13C: B3810076  sth r28, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[28].u16 ) };
	// 82E4C140: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4C144: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4C148: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E4C14C: 409A0008  bne cr6, 0x82e4c154
	if !ctx.cr[6].eq {
	pc = 0x82E4C154; continue 'dispatch;
	}
	// 82E4C150: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82E4C154: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C158: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E4C15C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E4C160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C164: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4C168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C16C: 4E800421  bctrl
	ctx.lr = 0x82E4C170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4C170: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E4C174: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E4C178: 4BF254E1  bl 0x82d71658
	ctx.lr = 0x82E4C17C;
	sub_82D71658(ctx, base);
	// 82E4C17C: 39600073  li r11, 0x73
	ctx.r[11].s64 = 115;
	// 82E4C180: 9B810068  stb r28, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u8 ) };
	// 82E4C184: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E4C188: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E4C18C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82E4C190: 3960006E  li r11, 0x6e
	ctx.r[11].s64 = 110;
	// 82E4C194: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82E4C198: 39600061  li r11, 0x61
	ctx.r[11].s64 = 97;
	// 82E4C19C: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 82E4C1A0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E4C1A4: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 82E4C1A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4C1AC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E4C1B0: 419A0018  beq cr6, 0x82e4c1c8
	if ctx.cr[6].eq {
	pc = 0x82E4C1C8; continue 'dispatch;
	}
	// 82E4C1B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E4C1B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C1BC: 4BF254AD  bl 0x82d71668
	ctx.lr = 0x82E4C1C0;
	sub_82D71668(ctx, base);
	// 82E4C1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C1C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E4C1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C1CC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4C1D0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E4C1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C1D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E4C1DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C1E0: 4E800421  bctrl
	ctx.lr = 0x82E4C1E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4C1E4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C1E8: 7C6A0034  cntlzw r10, r3
	ctx.r[10].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E4C1EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C1F0: 555EDFFE  rlwinm r30, r10, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E4C1F4: 419A0034  beq cr6, 0x82e4c228
	if ctx.cr[6].eq {
	pc = 0x82E4C228; continue 'dispatch;
	}
	// 82E4C1F8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4C1FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4C200: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E4C204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4C208: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E4C20C: 409A001C  bne cr6, 0x82e4c228
	if !ctx.cr[6].eq {
	pc = 0x82E4C228; continue 'dispatch;
	}
	// 82E4C210: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C214: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4C218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C21C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4C224: 4E800421  bctrl
	ctx.lr = 0x82E4C228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4C228: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4C22C: 4BFFFB5D  bl 0x82e4bd88
	ctx.lr = 0x82E4C230;
	sub_82E4BD88(ctx, base);
	// 82E4C230: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E4C234: 9BD70000  stb r30, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E4C238: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4C23C: 4BE5D208  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C240 size=308
    let mut pc: u32 = 0x82E4C240;
    'dispatch: loop {
        match pc {
            0x82E4C240 => {
    //   block [0x82E4C240..0x82E4C374)
	// 82E4C240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C244: 4BE5D1C5  bl 0x82ca9408
	ctx.lr = 0x82E4C248;
	sub_82CA93D0(ctx, base);
	// 82E4C248: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C24C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4C250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4C254: 419A0054  beq cr6, 0x82e4c2a8
	if ctx.cr[6].eq {
	pc = 0x82E4C2A8; continue 'dispatch;
	}
	// 82E4C258: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E4C25C: 419A004C  beq cr6, 0x82e4c2a8
	if ctx.cr[6].eq {
	pc = 0x82E4C2A8; continue 'dispatch;
	}
	// 82E4C260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C264: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4C268: 396B71F8  addi r11, r11, 0x71f8
	ctx.r[11].s64 = ctx.r[11].s64 + 29176;
	// 82E4C26C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4C270: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C274: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C278: B1410066  sth r10, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[10].u16 ) };
	// 82E4C27C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E4C280: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4C284: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E4C288: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82E4C28C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4C290: 48288B89  bl 0x830d4e18
	ctx.lr = 0x82E4C294;
	sub_830D4E18(ctx, base);
	// 82E4C294: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4C298: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C29C: 409A0018  bne cr6, 0x82e4c2b4
	if !ctx.cr[6].eq {
	pc = 0x82E4C2B4; continue 'dispatch;
	}
	// 82E4C2A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C2A4: 482887AD  bl 0x830d4a50
	ctx.lr = 0x82E4C2A8;
	sub_830D4A50(ctx, base);
	// 82E4C2A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4C2AC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C2B0: 4BE5D1A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E4C2B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C2B8: 386BB078  addi r3, r11, -0x4f88
	ctx.r[3].s64 = ctx.r[11].s64 + -20360;
	// 82E4C2BC: 4BF09495  bl 0x82d55750
	ctx.lr = 0x82E4C2C0;
	sub_82D55750(ctx, base);
	// 82E4C2C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C2C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4C2C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4C2CC: 4BF2520D  bl 0x82d714d8
	ctx.lr = 0x82E4C2D0;
	sub_82D714D8(ctx, base);
	// 82E4C2D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4C2D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C2D8: 409A0064  bne cr6, 0x82e4c33c
	if !ctx.cr[6].eq {
	pc = 0x82E4C33C; continue 'dispatch;
	}
	// 82E4C2DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C2E0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E4C2E4: 388B7204  addi r4, r11, 0x7204
	ctx.r[4].s64 = ctx.r[11].s64 + 29188;
	// 82E4C2E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4C2EC: 396B5D2C  addi r11, r11, 0x5d2c
	ctx.r[11].s64 = ctx.r[11].s64 + 23852;
	// 82E4C2F0: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82E4C2F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4C2F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4C2FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4C300: 4BF252A9  bl 0x82d715a8
	ctx.lr = 0x82E4C304;
	sub_82D715A8(ctx, base);
	// 82E4C304: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4C308: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C30C: 409A0030  bne cr6, 0x82e4c33c
	if !ctx.cr[6].eq {
	pc = 0x82E4C33C; continue 'dispatch;
	}
	// 82E4C310: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4C314: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4C318: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4C31C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C320: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E4C324: 409AFFD4  bne cr6, 0x82e4c2f8
	if !ctx.cr[6].eq {
	pc = 0x82E4C2F8; continue 'dispatch;
	}
	// 82E4C328: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C32C: 48288725  bl 0x830d4a50
	ctx.lr = 0x82E4C330;
	sub_830D4A50(ctx, base);
	// 82E4C330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C334: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C338: 4BE5D120  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E4C33C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4C340: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C344: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4C348: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C34C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4C350: 419A0010  beq cr6, 0x82e4c360
	if ctx.cr[6].eq {
	pc = 0x82E4C360; continue 'dispatch;
	}
	// 82E4C354: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E4C358: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4C35C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4C360: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C364: 482886ED  bl 0x830d4a50
	ctx.lr = 0x82E4C368;
	sub_830D4A50(ctx, base);
	// 82E4C368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C36C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4C370: 4BE5D0E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C378 size=128
    let mut pc: u32 = 0x82E4C378;
    'dispatch: loop {
        match pc {
            0x82E4C378 => {
    //   block [0x82E4C378..0x82E4C3F8)
	// 82E4C378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C37C: 4BE5D085  bl 0x82ca9400
	ctx.lr = 0x82E4C380;
	sub_82CA93D0(ctx, base);
	// 82E4C380: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C384: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E4C388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4C394: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4C398: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E4C39C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E4C3A0: 4BF093B1  bl 0x82d55750
	ctx.lr = 0x82E4C3A4;
	sub_82D55750(ctx, base);
	// 82E4C3A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C3A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C3AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C3B0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4C3B4: 4BF25065  bl 0x82d71418
	ctx.lr = 0x82E4C3B8;
	sub_82D71418(ctx, base);
	// 82E4C3B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E4C3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4C3C0: 38AB7BC4  addi r5, r11, 0x7bc4
	ctx.r[5].s64 = ctx.r[11].s64 + 31684;
	// 82E4C3C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C3C8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E4C3CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C3D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C3D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C3D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4C3DC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4C3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C3E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4C3E8: 4BFFFCC1  bl 0x82e4c0a8
	ctx.lr = 0x82E4C3EC;
	sub_82E4C0A8(ctx, base);
	// 82E4C3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C3F0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C3F4: 4BE5D05C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C3F8 size=132
    let mut pc: u32 = 0x82E4C3F8;
    'dispatch: loop {
        match pc {
            0x82E4C3F8 => {
    //   block [0x82E4C3F8..0x82E4C47C)
	// 82E4C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C3FC: 4BE5D005  bl 0x82ca9400
	ctx.lr = 0x82E4C400;
	sub_82CA93D0(ctx, base);
	// 82E4C400: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C404: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E4C408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C40C: 3BCBB078  addi r30, r11, -0x4f88
	ctx.r[30].s64 = ctx.r[11].s64 + -20360;
	// 82E4C410: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4C414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4C418: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E4C41C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E4C420: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E4C424: 4BF0932D  bl 0x82d55750
	ctx.lr = 0x82E4C428;
	sub_82D55750(ctx, base);
	// 82E4C428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4C42C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C430: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C434: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4C438: 4BF24FE1  bl 0x82d71418
	ctx.lr = 0x82E4C43C;
	sub_82D71418(ctx, base);
	// 82E4C43C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E4C440: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4C444: 38AB7BC4  addi r5, r11, 0x7bc4
	ctx.r[5].s64 = ctx.r[11].s64 + 31684;
	// 82E4C448: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C44C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E4C450: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C454: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C458: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C45C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4C460: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4C464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C468: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4C46C: 4BFFFC3D  bl 0x82e4c0a8
	ctx.lr = 0x82E4C470;
	sub_82E4C0A8(ctx, base);
	// 82E4C470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C474: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C478: 4BE5CFD8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C480 size=152
    let mut pc: u32 = 0x82E4C480;
    'dispatch: loop {
        match pc {
            0x82E4C480 => {
    //   block [0x82E4C480..0x82E4C518)
	// 82E4C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C484: 4BE5CF7D  bl 0x82ca9400
	ctx.lr = 0x82E4C488;
	sub_82CA93D0(ctx, base);
	// 82E4C488: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C48C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C494: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4C498: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4C49C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E4C4A0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E4C4A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4C4A8: 419A005C  beq cr6, 0x82e4c504
	if ctx.cr[6].eq {
	pc = 0x82E4C504; continue 'dispatch;
	}
	// 82E4C4AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E4C4B0: 419A0054  beq cr6, 0x82e4c504
	if ctx.cr[6].eq {
	pc = 0x82E4C504; continue 'dispatch;
	}
	// 82E4C4B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4B8: 4BFFEF51  bl 0x82e4b408
	ctx.lr = 0x82E4C4BC;
	sub_82E4B408(ctx, base);
	// 82E4C4BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E4C4C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4C4C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4C8: 4BFFF079  bl 0x82e4b540
	ctx.lr = 0x82E4C4CC;
	sub_82E4B540(ctx, base);
	// 82E4C4CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4C4D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4C4D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4C4D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E4C4DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4C4E0: 4BFFFF19  bl 0x82e4c3f8
	ctx.lr = 0x82E4C4E4;
	sub_82E4C3F8(ctx, base);
	// 82E4C4E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4C4E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4C4EC: 8BCB0000  lbz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C4F0: 4BFFEF49  bl 0x82e4b438
	ctx.lr = 0x82E4C4F4;
	sub_82E4B438(ctx, base);
	// 82E4C4F4: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E4C4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C4FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C500: 4BE5CF50  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82E4C504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4C508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C50C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E4C510: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E4C514: 4BE5CF3C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C518 size=100
    let mut pc: u32 = 0x82E4C518;
    'dispatch: loop {
        match pc {
            0x82E4C518 => {
    //   block [0x82E4C518..0x82E4C57C)
	// 82E4C518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4C524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4C528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C52C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C530: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C534: 4BFFF855  bl 0x82e4bd88
	ctx.lr = 0x82E4C538;
	sub_82E4BD88(ctx, base);
	// 82E4C538: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4C53C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C540: 419A0020  beq cr6, 0x82e4c560
	if ctx.cr[6].eq {
	pc = 0x82E4C560; continue 'dispatch;
	}
	// 82E4C544: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C548: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C54C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82E4C550: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4C558: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C55C: 4BF08D6D  bl 0x82d552c8
	ctx.lr = 0x82E4C560;
	sub_82D552C8(ctx, base);
	// 82E4C560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4C568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4C56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4C570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4C574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4C578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4C580 size=100
    let mut pc: u32 = 0x82E4C580;
    'dispatch: loop {
        match pc {
            0x82E4C580 => {
    //   block [0x82E4C580..0x82E4C5E4)
	// 82E4C580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4C58C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4C590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4C598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4C59C: 482884B5  bl 0x830d4a50
	ctx.lr = 0x82E4C5A0;
	sub_830D4A50(ctx, base);
	// 82E4C5A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4C5A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4C5A8: 419A0020  beq cr6, 0x82e4c5c8
	if ctx.cr[6].eq {
	pc = 0x82E4C5C8; continue 'dispatch;
	}
	// 82E4C5AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C5B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4C5B4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82E4C5B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C5BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4C5C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4C5C4: 4BF08D05  bl 0x82d552c8
	ctx.lr = 0x82E4C5C8;
	sub_82D552C8(ctx, base);
	// 82E4C5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4C5CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4C5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4C5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4C5D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4C5DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4C5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C5E8 size=24
    let mut pc: u32 = 0x82E4C5E8;
    'dispatch: loop {
        match pc {
            0x82E4C5E8 => {
    //   block [0x82E4C5E8..0x82E4C600)
	// 82E4C5E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C5EC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C5F0: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82E4C5F4: 4199000C  bgt cr6, 0x82e4c600
	if ctx.cr[6].gt {
		sub_82E4C600(ctx, base);
		return;
	}
	// 82E4C5F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4C5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C600 size=144
    let mut pc: u32 = 0x82E4C600;
    'dispatch: loop {
        match pc {
            0x82E4C600 => {
    //   block [0x82E4C600..0x82E4C690)
	// 82E4C600: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E4C604: 4099FFF4  ble cr6, 0x82e4c5f8
	if !ctx.cr[6].gt {
		sub_82E4C5E8(ctx, base);
		return;
	}
	// 82E4C608: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C60C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4C610: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E4C614: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82E4C618: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E4C61C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E4C620: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C624: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C690 size=24
    let mut pc: u32 = 0x82E4C690;
    'dispatch: loop {
        match pc {
            0x82E4C690 => {
    //   block [0x82E4C690..0x82E4C6A8)
	// 82E4C690: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C694: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C698: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E4C69C: 4199000C  bgt cr6, 0x82e4c6a8
	if ctx.cr[6].gt {
		sub_82E4C6A8(ctx, base);
		return;
	}
	// 82E4C6A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4C6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C6A8 size=180
    let mut pc: u32 = 0x82E4C6A8;
    'dispatch: loop {
        match pc {
            0x82E4C6A8 => {
    //   block [0x82E4C6A8..0x82E4C75C)
	// 82E4C6A8: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C6AC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4C6B0: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C6B4: ED0D0372  fmuls f8, f13, f13
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C6B8: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C6BC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E4C6C0: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C6C4: ECEC0332  fmuls f7, f12, f12
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4C6C8: ED200032  fmuls f9, f0, f0
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C6CC: 39650020  addi r11, r5, 0x20
	ctx.r[11].s64 = ctx.r[5].s64 + 32;
	// 82E4C6D0: C1490C14  lfs f10, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E4C6D4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82E4C6D8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C760 size=72
    let mut pc: u32 = 0x82E4C760;
    'dispatch: loop {
        match pc {
            0x82E4C760 => {
    //   block [0x82E4C760..0x82E4C7A8)
	// 82E4C760: C1A40050  lfs f13, 0x50(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C764: C1840064  lfs f12, 0x64(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C768: EC0D6028  fsubs f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C76C: C1640078  lfs f11, 0x78(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4C770: FC00636E  fsel f0, f0, f13, f12
	ctx.f[0].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 82E4C774: ED405828  fsubs f10, f0, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4C778: FC0A582E  fsel f0, f10, f0, f11
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[11].f64 };
	// 82E4C77C: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82E4C780: ED4D0028  fsubs f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C784: ED2C0028  fsubs f9, f12, f0
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C788: ED0B0028  fsubs f8, f11, f0
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C78C: FDAA036E  fsel f13, f10, f13, f0
	ctx.f[13].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82E4C790: D1A40050  stfs f13, 0x50(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4C794: FDA9032E  fsel f13, f9, f12, f0
	ctx.f[13].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[0].f64 };
	// 82E4C798: D1A40064  stfs f13, 0x64(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4C79C: FC0802EE  fsel f0, f8, f11, f0
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[0].f64 };
	// 82E4C7A0: D0040078  stfs f0, 0x78(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C7A8 size=172
    let mut pc: u32 = 0x82E4C7A8;
    'dispatch: loop {
        match pc {
            0x82E4C7A8 => {
    //   block [0x82E4C7A8..0x82E4C854)
	// 82E4C7A8: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7AC: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7B0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7B4: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7B8: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7BC: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7C0: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4C7C4: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7C8: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7CC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7D0: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7D4: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7D8: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7DC: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4C7E0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C7E4: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C7E8: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C7EC: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C7F0: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C7F4: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C7F8: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4C7FC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C800: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C804: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C808: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C80C: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C810: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4C814: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4C818: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C81C: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C820: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C824: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C828: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C82C: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4C830: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4C834: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C838: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C83C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C840: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C844: EC00637A  fmadds f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C848: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4C84C: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C858 size=172
    let mut pc: u32 = 0x82E4C858;
    'dispatch: loop {
        match pc {
            0x82E4C858 => {
    //   block [0x82E4C858..0x82E4C904)
	// 82E4C858: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C85C: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C860: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C864: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C868: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C86C: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C870: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4C874: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C878: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C87C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C880: C1850014  lfs f12, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C884: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C888: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C88C: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4C890: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C894: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4C898: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C89C: C1850028  lfs f12, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8A0: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4C8A4: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4C8A8: D0050028  stfs f0, 0x28(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4C8AC: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8B0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C8B4: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8B8: C1850004  lfs f12, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8BC: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8C0: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4C8C4: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4C8C8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8CC: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4C8D0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8D4: C1850018  lfs f12, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8D8: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8DC: D0050018  stfs f0, 0x18(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4C8E0: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4C8E4: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C8E8: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4C8EC: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C8F0: C1850020  lfs f12, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4C8F4: EC00637C  fnmsubs f0, f0, f13, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4C8F8: D0050020  stfs f0, 0x20(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4C8FC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4C900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4C908 size=80
    let mut pc: u32 = 0x82E4C908;
    'dispatch: loop {
        match pc {
            0x82E4C908 => {
    //   block [0x82E4C908..0x82E4C958)
	// 82E4C908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4C910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4C914: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E4C918: D0210084  stfs f1, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E4C91C: 39410084  addi r10, r1, 0x84
	ctx.r[10].s64 = ctx.r[1].s64 + 132;
	// 82E4C920: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E4C924: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4C928: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C958 size=76
    let mut pc: u32 = 0x82E4C958;
    'dispatch: loop {
        match pc {
            0x82E4C958 => {
    //   block [0x82E4C958..0x82E4C9A4)
	// 82E4C958: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4C95C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82E4C960: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C964: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82E4C968: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4C96C: C1830028  lfs f12, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4C9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4C9A8 size=688
    let mut pc: u32 = 0x82E4C9A8;
    'dispatch: loop {
        match pc {
            0x82E4C9A8 => {
    //   block [0x82E4C9A8..0x82E4CC58)
	// 82E4C9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4C9AC: 4BE5CA61  bl 0x82ca940c
	ctx.lr = 0x82E4C9B0;
	sub_82CA93D0(ctx, base);
	// 82E4C9B0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4C9B4: 4BE612F1  bl 0x82cadca4
	ctx.lr = 0x82E4C9B8;
	sub_82CADCA0(ctx, base);
	// 82E4C9B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4C9BC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4C9C0: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4C9C4: 3FA08200  lis r29, -0x7e00
	ctx.r[29].s64 = -2113929216;
	// 82E4C9C8: 3CC08204  lis r6, -0x7dfc
	ctx.r[6].s64 = -2113667072;
	// 82E4C9CC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E4C9D0: 57E9103A  slwi r9, r31, 2
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4C9D4: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E4C9D8: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4C9DC: 3CA05555  lis r5, 0x5555
	ctx.r[5].s64 = 1431633920;
	// 82E4C9E0: C0BD0A4C  lfs f5, 0xa4c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2636 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4C9E4: C0C6BDF4  lfs f6, -0x420c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E4C9E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4C9EC: C0270C4C  lfs f1, 0xc4c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E4C9F0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E4C9F4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4C9F8: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82E4C9FC: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CA00: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82E4CA04: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CA08: 60A55556  ori r5, r5, 0x5556
	ctx.r[5].u64 = ctx.r[5].u64 | 21846;
	// 82E4CA0C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CA10: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CA14: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CA18: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CA1C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CA20: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CA24: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4CA28: 7CEB2896  mulhw r7, r11, r5
	ctx.r[7].s64 = ((ctx.r[11].s32 as i64 * ctx.r[5].s32 as i64) >> 32);
	// 82E4CA2C: C1890000  lfs f12, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CA30: 54E60FFE  srwi r6, r7, 0x1f
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA34: C1680000  lfs f11, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4CA38: ED2C0332  fmuls f9, f12, f12
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CA3C: C323000C  lfs f25, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82E4CA40: C2E30010  lfs f23, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82E4CA44: EC8B02F2  fmuls f4, f11, f11
	ctx.f[4].f64 = (((ctx.f[11].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CA48: C2A30024  lfs f21, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82E4CA4C: C2C30018  lfs f22, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82E4CA50: C263001C  lfs f19, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 82E4CA54: C2430028  lfs f18, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[18].f64 = (tmp.f32 as f64);
	// 82E4CA58: C2830030  lfs f20, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 82E4CA5C: C223002C  lfs f17, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[17].f64 = (tmp.f32 as f64);
	// 82E4CA60: EC690332  fmuls f3, f9, f12
	ctx.f[3].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CA64: ED0402F2  fmuls f8, f4, f11
	ctx.f[8].f64 = (((ctx.f[4].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CA68: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E4CA6C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82E4CA70: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82E4CA74: 54E6083C  slwi r6, r7, 1
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA78: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82E4CA7C: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82E4CA80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4CA84: 7CE75850  subf r7, r7, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82E4CA88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4CA8C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4CA90: 7CC7FA14  add r6, r7, r31
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82E4CA94: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82E4CA98: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4CA9C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4CAA0: 7C06242E  lfsx f0, r6, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CAA4: EF800332  fmuls f28, f0, f12
	ctx.f[28].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CAA8: 7DA7242E  lfsx f13, r7, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CAAC: ED4D5828  fsubs f10, f13, f11
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4CAB0: EFE0602A  fadds f31, f0, f12
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 82E4CAB4: EC400032  fmuls f2, f0, f0
	ctx.f[2].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CAB8: EFAD582A  fadds f29, f13, f11
	ctx.f[29].f64 = ((ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64;
	// 82E4CABC: EFCD0372  fmuls f30, f13, f13
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CAC0: ECE06028  fsubs f7, f0, f12
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CAC4: EF9C0072  fmuls f28, f28, f1
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CAC8: EF1FCABA  fmadds f24, f31, f10, f25
	ctx.f[24].f64 = (((ctx.f[31].f64 * ctx.f[10].f64 + ctx.f[25].f64) as f32) as f64);
	// 82E4CACC: D303000C  stfs f24, 0xc(r3)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4CAD0: EF5F483A  fmadds f26, f31, f0, f9
	ctx.f[26].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 82E4CAD4: EFBD237A  fmadds f29, f29, f13, f4
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[4].f64) as f32) as f64);
	// 82E4CAD8: EF7E0372  fmuls f27, f30, f13
	ctx.f[27].f64 = (((ctx.f[30].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CADC: EFDE02F2  fmuls f30, f30, f11
	ctx.f[30].f64 = (((ctx.f[30].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CAE0: EC840372  fmuls f4, f4, f13
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CAE4: EF22E1BA  fmadds f25, f2, f6, f28
	ctx.f[25].f64 = (((ctx.f[2].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CAE8: EF89E1BA  fmadds f28, f9, f6, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[6].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CAEC: EFFD437A  fmadds f31, f29, f13, f8
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[8].f64) as f32) as f64);
	// 82E4CAF0: ED39482A  fadds f9, f25, f9
	ctx.f[9].f64 = ((ctx.f[25].f64 + ctx.f[9].f64) as f32) as f64;
	// 82E4CAF4: EF3A183A  fmadds f25, f26, f0, f3
	ctx.f[25].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CAF8: EF5ABABA  fmadds f26, f26, f10, f23
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[10].f64 + ctx.f[23].f64) as f32) as f64);
	// 82E4CAFC: D3430010  stfs f26, 0x10(r3)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CB00: EF9C102A  fadds f28, f28, f2
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[2].f64) as f32) as f64;
	// 82E4CB04: EC420032  fmuls f2, f2, f0
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB08: EDFF0372  fmuls f15, f31, f13
	ctx.f[15].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CB0C: EE090332  fmuls f16, f9, f12
	ctx.f[16].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CB10: EEF90032  fmuls f23, f25, f0
	ctx.f[23].f64 = (((ctx.f[25].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB14: EF39B2BA  fmadds f25, f25, f10, f22
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[10].f64 + ctx.f[22].f64) as f32) as f64);
	// 82E4CB18: C2C30014  lfs f22, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82E4CB1C: D3230018  stfs f25, 0x18(r3)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CB20: EC42817A  fmadds f2, f2, f5, f16
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[5].f64 + ctx.f[16].f64) as f32) as f64);
	// 82E4CB24: EEE3BB3A  fmadds f23, f3, f12, f23
	ctx.f[23].f64 = (((ctx.f[3].f64 * ctx.f[12].f64 + ctx.f[23].f64) as f32) as f64);
	// 82E4CB28: EC630172  fmuls f3, f3, f5
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E4CB2C: EE087AFA  fmadds f16, f8, f11, f15
	ctx.f[16].f64 = (((ctx.f[8].f64 * ctx.f[11].f64 + ctx.f[15].f64) as f32) as f64);
	// 82E4CB30: EEF7AABA  fmadds f23, f23, f10, f21
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[10].f64 + ctx.f[21].f64) as f32) as f64);
	// 82E4CB34: C2A30020  lfs f21, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82E4CB38: EC7C183A  fmadds f3, f28, f0, f3
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CB3C: D2E30024  stfs f23, 0x24(r3)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CB40: EF9C02F2  fmuls f28, f28, f11
	ctx.f[28].f64 = (((ctx.f[28].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CB44: EC6302F2  fmuls f3, f3, f11
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CB48: EF89E37A  fmadds f28, f9, f13, f28
	ctx.f[28].f64 = (((ctx.f[9].f64 * ctx.f[13].f64 + ctx.f[28].f64) as f32) as f64);
	// 82E4CB4C: ED3FA9FA  fmadds f9, f31, f7, f21
	ctx.f[9].f64 = (((ctx.f[31].f64 * ctx.f[7].f64 + ctx.f[21].f64) as f32) as f64);
	// 82E4CB50: ED7DB1FA  fmadds f11, f29, f7, f22
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[7].f64 + ctx.f[22].f64) as f32) as f64);
	// 82E4CB54: EFFE0072  fmuls f31, f30, f1
	ctx.f[31].f64 = (((ctx.f[30].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CB58: EC421B7A  fmadds f2, f2, f13, f3
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[13].f64 + ctx.f[3].f64) as f32) as f64);
	// 82E4CB5C: EC7C9ABA  fmadds f3, f28, f10, f19
	ctx.f[3].f64 = (((ctx.f[28].f64 * ctx.f[10].f64 + ctx.f[19].f64) as f32) as f64);
	// 82E4CB60: EDB0A1FA  fmadds f13, f16, f7, f20
	ctx.f[13].f64 = (((ctx.f[16].f64 * ctx.f[7].f64 + ctx.f[20].f64) as f32) as f64);
	// 82E4CB64: ED4292BA  fmadds f10, f2, f10, f18
	ctx.f[10].f64 = (((ctx.f[2].f64 * ctx.f[10].f64 + ctx.f[18].f64) as f32) as f64);
	// 82E4CB68: EC5E01B2  fmuls f2, f30, f6
	ctx.f[2].f64 = (((ctx.f[30].f64 * ctx.f[6].f64) as f32) as f64);
	// 82E4CB6C: EC44107A  fmadds f2, f4, f1, f2
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64);
	// 82E4CB70: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CB74: EC84F9BA  fmadds f4, f4, f6, f31
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[6].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CB78: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CB7C: D1A30030  stfs f13, 0x30(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4CB80: D063001C  stfs f3, 0x1c(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CB84: D1430028  stfs f10, 0x28(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CB88: EC5B117A  fmadds f2, f27, f5, f2
	ctx.f[2].f64 = (((ctx.f[27].f64 * ctx.f[5].f64 + ctx.f[2].f64) as f32) as f64);
	// 82E4CB8C: EC88217A  fmadds f4, f8, f5, f4
	ctx.f[4].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[4].f64) as f32) as f64);
	// 82E4CB90: ED02402A  fadds f8, f2, f8
	ctx.f[8].f64 = ((ctx.f[2].f64 + ctx.f[8].f64) as f32) as f64;
	// 82E4CB94: EC84D82A  fadds f4, f4, f27
	ctx.f[4].f64 = ((ctx.f[4].f64 + ctx.f[27].f64) as f32) as f64;
	// 82E4CB98: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CB9C: EC04033A  fmadds f0, f4, f12, f0
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CBA0: EC0089FA  fmadds f0, f0, f7, f17
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[7].f64 + ctx.f[17].f64) as f32) as f64);
	// 82E4CBA4: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CBA8: 409AFE80  bne cr6, 0x82e4ca28
	if !ctx.cr[6].eq {
	pc = 0x82E4CA28; continue 'dispatch;
	}
	// 82E4CBAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBB0: C18B0BFC  lfs f12, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E4CBB8: ED180332  fmuls f8, f24, f12
	ctx.f[8].f64 = (((ctx.f[24].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBBC: D103000C  stfs f8, 0xc(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4CBC0: C18BC40C  lfs f12, -0x3bf4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15348 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBC8: ECFA0332  fmuls f7, f26, f12
	ctx.f[7].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBCC: D0E30010  stfs f7, 0x10(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CBD0: C18B0C7C  lfs f12, 0xc7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3196 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CBD8: ED190332  fmuls f8, f25, f12
	ctx.f[8].f64 = (((ctx.f[25].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBDC: D1030018  stfs f8, 0x18(r3)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CBE0: C18B0AB4  lfs f12, 0xab4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2740 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CBE8: ECF70332  fmuls f7, f23, f12
	ctx.f[7].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBEC: D0E30024  stfs f7, 0x24(r3)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CBF0: C18B2F00  lfs f12, 0x2f00(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12032 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CBF4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CBF8: ED6B0332  fmuls f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CBFC: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CC00: C18B721C  lfs f12, 0x721c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC04: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82E4CC08: ED290332  fmuls f9, f9, f12
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CC0C: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CC10: C18B1354  lfs f12, 0x1354(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CC18: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CC1C: D1830030  stfs f12, 0x30(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E4CC20: C1AB0AA8  lfs f13, 0xaa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2728 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4CC28: ED630372  fmuls f11, f3, f13
	ctx.f[11].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC2C: D163001C  stfs f11, 0x1c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E4CC30: C1AB0AA0  lfs f13, 0xaa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2720 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4CC38: ED8A0372  fmuls f12, f10, f13
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC3C: D1830028  stfs f12, 0x28(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CC40: C1AB7218  lfs f13, 0x7218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29208 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC44: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC48: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E4CC4C: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4CC50: 4BE610A1  bl 0x82cadcf0
	ctx.lr = 0x82E4CC54;
	sub_82CADCEC(ctx, base);
	// 82E4CC54: 4BE5C808  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4CC58 size=584
    let mut pc: u32 = 0x82E4CC58;
    'dispatch: loop {
        match pc {
            0x82E4CC58 => {
    //   block [0x82E4CC58..0x82E4CEA0)
	// 82E4CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4CC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4CC64: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82E4CC68: 4BE61055  bl 0x82cadcbc
	ctx.lr = 0x82E4CC6C;
	sub_82CADCA0(ctx, base);
	// 82E4CC6C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4CC70: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4CC74: 4BFFFD35  bl 0x82e4c9a8
	ctx.lr = 0x82E4CC78;
	sub_82E4C9A8(ctx, base);
	// 82E4CC78: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CC7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4CC80: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CC84: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4CC88: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CC8C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4CC90: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CC94: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4CC98: C17F0004  lfs f11, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E4CC9C: C0A40008  lfs f5, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CCA0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4CCA4: C03F0008  lfs f1, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E4CCA8: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4CCAC: 7D29FC2E  lfsx f9, r9, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E4CCB0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4CCB4: C1430010  lfs f10, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E4CCB8: C1030014  lfs f8, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82E4CCBC: C3E3000C  lfs f31, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4CCC0: C0E30018  lfs f7, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E4CCC4: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CCC8: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CCCC: EC004824  fdivs f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[9].f64) as f32) as f64;
	// 82E4CCD0: C0C30020  lfs f6, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E4CCD4: C0630024  lfs f3, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E4CCD8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4CCDC: C083001C  lfs f4, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E4CCE0: C0430030  lfs f2, 0x30(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E4CCE4: C1290C4C  lfs f9, 0xc4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3148 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E4CCE8: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82E4CCEC: EDAC6AFC  fnmsubs f13, f12, f11, f13
	ctx.f[13].f64 = -(((ctx.f[12].f64 * ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4CCF0: ED870032  fmuls f12, f7, f0
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCF4: EFC60032  fmuls f30, f6, f0
	ctx.f[30].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCF8: EFA30032  fmuls f29, f3, f0
	ctx.f[29].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CCFC: EF820032  fmuls f28, f2, f0
	ctx.f[28].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD00: ED65687C  fnmsubs f11, f5, f1, f13
	ctx.f[11].f64 = -(((ctx.f[5].f64 * ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4CD04: EDAA0032  fmuls f13, f10, f0
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD08: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4CD0C: EDA80032  fmuls f13, f8, f0
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD10: D1A30038  stfs f13, 0x38(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4CD14: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CD18: EC200032  fmuls f1, f0, f0
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD1C: EDA80372  fmuls f13, f8, f13
	ctx.f[13].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD20: 7CABFC2E  lfsx f5, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CD24: D3C30044  stfs f30, 0x44(r3)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82E4CD28: D1830040  stfs f12, 0x40(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E4CD2C: EFFF02F2  fmuls f31, f31, f11
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CD30: EFC10032  fmuls f30, f1, f0
	ctx.f[30].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD34: EDA56ABA  fmadds f13, f5, f10, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CD38: EF1E0032  fmuls f24, f30, f0
	ctx.f[24].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CD3C: EDADF82A  fadds f13, f13, f31
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E4CD40: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CD44: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CD48: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82E4CD4C: 7D8BFC2E  lfsx f12, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CD50: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CD54: ECAC02B2  fmuls f5, f12, f10
	ctx.f[5].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E4CD58: D3A3004C  stfs f29, 0x4c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82E4CD5C: EFA40332  fmuls f29, f4, f12
	ctx.f[29].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CD60: D3830050  stfs f28, 0x50(r3)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4CD64: EF870332  fmuls f28, f7, f12
	ctx.f[28].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CD68: ECAD2A3A  fmadds f5, f13, f8, f5
	ctx.f[5].f64 = (((ctx.f[13].f64 * ctx.f[8].f64 + ctx.f[5].f64) as f32) as f64);
	// 82E4CD6C: EFBD0372  fmuls f29, f29, f13
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD70: ECA5FA7A  fmadds f5, f5, f9, f31
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CD74: EFBD0272  fmuls f29, f29, f9
	ctx.f[29].f64 = (((ctx.f[29].f64 * ctx.f[9].f64) as f32) as f64);
	// 82E4CD78: ECA5EAFA  fmadds f5, f5, f11, f29
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[11].f64 + ctx.f[29].f64) as f32) as f64);
	// 82E4CD7C: EFA60372  fmuls f29, f6, f13
	ctx.f[29].f64 = (((ctx.f[6].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CD80: EDBD2B7A  fmadds f13, f29, f13, f5
	ctx.f[13].f64 = (((ctx.f[29].f64 * ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64);
	// 82E4CD84: C3A3002C  lfs f29, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E4CD88: C0A9BDF4  lfs f5, -0x420c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16908 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E4CD8C: EDBC6B3A  fmadds f13, f28, f12, f13
	ctx.f[13].f64 = (((ctx.f[28].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CD90: C3830028  lfs f28, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4CD94: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4CD98: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82E4CD9C: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CDA0: EF4D02B2  fmuls f26, f13, f10
	ctx.f[26].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E4CDA4: 7D8AFC2E  lfsx f12, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CDA8: EF640372  fmuls f27, f4, f13
	ctx.f[27].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDAC: EF230372  fmuls f25, f3, f13
	ctx.f[25].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDB0: ED4A02F2  fmuls f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CDB4: EC1C0032  fmuls f0, f28, f0
	ctx.f[0].f64 = (((ctx.f[28].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CDB8: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E4CDBC: EC0CD23A  fmadds f0, f12, f8, f26
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[8].f64 + ctx.f[26].f64) as f32) as f64);
	// 82E4CDC0: EF7B0332  fmuls f27, f27, f12
	ctx.f[27].f64 = (((ctx.f[27].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDC4: EEFD0372  fmuls f23, f29, f13
	ctx.f[23].f64 = (((ctx.f[29].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDC8: EEC70372  fmuls f22, f7, f13
	ctx.f[22].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDCC: EEBC0332  fmuls f21, f28, f12
	ctx.f[21].f64 = (((ctx.f[28].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDD0: EF390372  fmuls f25, f25, f13
	ctx.f[25].f64 = (((ctx.f[25].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDD4: EC00F97A  fmadds f0, f0, f5, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CDD8: EFE60332  fmuls f31, f6, f12
	ctx.f[31].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDDC: EF570332  fmuls f26, f23, f12
	ctx.f[26].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDE0: EEE20332  fmuls f23, f2, f12
	ctx.f[23].f64 = (((ctx.f[2].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDE4: ED150372  fmuls f8, f21, f13
	ctx.f[8].f64 = (((ctx.f[21].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDE8: EC0002F2  fmuls f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E4CDEC: EFFF0332  fmuls f31, f31, f12
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF0: EF5A0332  fmuls f26, f26, f12
	ctx.f[26].f64 = (((ctx.f[26].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF4: EEF70332  fmuls f23, f23, f12
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E4CDF8: ED080372  fmuls f8, f8, f13
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CDFC: EFFBFA7A  fmadds f31, f27, f9, f31
	ctx.f[31].f64 = (((ctx.f[27].f64 * ctx.f[9].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CE00: EF7A0172  fmuls f27, f26, f5
	ctx.f[27].f64 = (((ctx.f[26].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E4CE04: EFF6FB7A  fmadds f31, f22, f13, f31
	ctx.f[31].f64 = (((ctx.f[22].f64 * ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E4CE08: EC1F017A  fmadds f0, f31, f5, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE0C: EC00DAFA  fmadds f0, f0, f11, f27
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[27].f64) as f32) as f64);
	// 82E4CE10: EC08017A  fmadds f0, f8, f5, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[5].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE14: EC17033A  fmadds f0, f23, f12, f0
	ctx.f[0].f64 = (((ctx.f[23].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE18: EC19037A  fmadds f0, f25, f13, f0
	ctx.f[0].f64 = (((ctx.f[25].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE1C: EC000632  fmuls f0, f0, f24
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[24].f64) as f32) as f64);
	// 82E4CE20: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CE24: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E4CE28: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CE2C: EC020032  fmuls f0, f2, f0
	ctx.f[0].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE30: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CE34: EC0D077A  fmadds f0, f13, f29, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE38: EC0602FA  fmadds f0, f6, f11, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE3C: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4CE40: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CE44: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E4CE48: 7DABFC2E  lfsx f13, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CE4C: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE50: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CE54: ED1C0372  fmuls f8, f28, f13
	ctx.f[8].f64 = (((ctx.f[28].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE58: ED84603A  fmadds f12, f4, f0, f12
	ctx.f[12].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4CE5C: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE60: ED8C527A  fmadds f12, f12, f9, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[10].f64) as f32) as f64);
	// 82E4CE64: ED480272  fmuls f10, f8, f9
	ctx.f[10].f64 = (((ctx.f[8].f64 * ctx.f[9].f64) as f32) as f64);
	// 82E4CE68: ED8C52FA  fmadds f12, f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 82E4CE6C: ED7D0032  fmuls f11, f29, f0
	ctx.f[11].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CE70: ED430372  fmuls f10, f3, f13
	ctx.f[10].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CE74: EC0B603A  fmadds f0, f11, f0, f12
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E4CE78: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CE7C: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4CE80: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E4CE84: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E4CE88: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82E4CE8C: 4BE60E7D  bl 0x82cadd08
	ctx.lr = 0x82E4CE90;
	sub_82CADCEC(ctx, base);
	// 82E4CE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4CE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4CE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4CE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4CEA0 size=340
    let mut pc: u32 = 0x82E4CEA0;
    'dispatch: loop {
        match pc {
            0x82E4CEA0 => {
    //   block [0x82E4CEA0..0x82E4CFF4)
	// 82E4CEA0: C1A30064  lfs f13, 0x64(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEA4: C0030068  lfs f0, 0x68(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEA8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CEAC: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CEB0: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEB4: C003006C  lfs f0, 0x6c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEB8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CEBC: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CEC0: C1A30064  lfs f13, 0x64(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEC4: C0030070  lfs f0, 0x70(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEC8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E4CECC: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CED0: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CED4: C0030078  lfs f0, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CED8: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CEDC: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CEE0: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CEE4: C1A3007C  lfs f13, 0x7c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEE8: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CEEC: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CEF0: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CEF4: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CEF8: C1A30078  lfs f13, 0x78(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CEFC: C0030074  lfs f0, 0x74(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF00: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E4CF04: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF08: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CF0C: C0030080  lfs f0, 0x80(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF10: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF14: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF18: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CF1C: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CF20: C0030084  lfs f0, 0x84(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF24: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF28: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF2C: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CF30: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CF34: C0030088  lfs f0, 0x88(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF38: EC0000B2  fmuls f0, f0, f2
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82E4CF3C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E4CF40: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CF44: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CF48: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF4C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E4CF50: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF54: C1870000  lfs f12, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF58: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E4CF5C: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF60: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E4CF64: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF68: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CF6C: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF70: C1870014  lfs f12, 0x14(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF74: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CF78: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF7C: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4CF80: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CF84: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CF88: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CF8C: C1870028  lfs f12, 0x28(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4CF90: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CF94: EC00607C  fnmsubs f0, f0, f1, f12
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[1].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E4CF98: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4CF9C: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFA0: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFA4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFA8: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFAC: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFB0: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4CFB4: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4CFB8: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFBC: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFC0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFC4: C1A70018  lfs f13, 0x18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFC8: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFCC: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4CFD0: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4CFD4: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFD8: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4CFDC: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E4CFE0: C1A70020  lfs f13, 0x20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4CFE4: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E4CFE8: D0070020  stfs f0, 0x20(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E4CFEC: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4CFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4CFF8 size=116
    let mut pc: u32 = 0x82E4CFF8;
    'dispatch: loop {
        match pc {
            0x82E4CFF8 => {
    //   block [0x82E4CFF8..0x82E4D06C)
	// 82E4CFF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4D06C size=68
    let mut pc: u32 = 0x82E4D06C;
    'dispatch: loop {
        match pc {
            0x82E4D06C => {
    //   block [0x82E4D06C..0x82E4D0B0)
	// 82E4D06C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4D070: 114D004A  vsubfp v10, v13, v0
	ctx.fpscr.enable_flush_mode_unconditional();
	for i in 0..4 {
		ctx.v[10].f32[i] = ctx.v[13].f32[i] - ctx.v[0].f32[i];
	}
	// 82E4D074: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E4D078: C00B0BFC  lfs f0, 0xbfc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4D07C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E4D080: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4D0B0 size=760
    let mut pc: u32 = 0x82E4D0B0;
    'dispatch: loop {
        match pc {
            0x82E4D0B0 => {
    //   block [0x82E4D0B0..0x82E4D3A8)
	// 82E4D0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D0B4: 4BE5C341  bl 0x82ca93f4
	ctx.lr = 0x82E4D0B8;
	sub_82CA93D0(ctx, base);
	// 82E4D0B8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82E4D0BC: 4BE60C11  bl 0x82cadccc
	ctx.lr = 0x82E4D0C0;
	sub_82CADCA0(ctx, base);
	// 82E4D0C0: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D3A8 size=424
    let mut pc: u32 = 0x82E4D3A8;
    'dispatch: loop {
        match pc {
            0x82E4D3A8 => {
    //   block [0x82E4D3A8..0x82E4D550)
	// 82E4D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4D3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4D3B4: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82E4D3B8: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E4D3BC: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E4D3C0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D3C4: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 82E4D3C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4D3CC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E4D3D0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4D3D4: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 82E4D3D8: 4199000C  bgt cr6, 0x82e4d3e4
	if ctx.cr[6].gt {
	pc = 0x82E4D3E4; continue 'dispatch;
	}
	// 82E4D3DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4D3E0: 48000150  b 0x82e4d530
	pc = 0x82E4D530; continue 'dispatch;
	// 82E4D3E4: FF011800  fcmpu cr6, f1, f3
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[3].f64);
	// 82E4D3E8: 4099FFF4  ble cr6, 0x82e4d3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4D3DC; continue 'dispatch;
	}
	// 82E4D3EC: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 82E4D3F0: 4099FFEC  ble cr6, 0x82e4d3dc
	if !ctx.cr[6].gt {
	pc = 0x82E4D3DC; continue 'dispatch;
	}
	// 82E4D3F4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 82E4D3F8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4D3FC: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82E4D400: EDA10072  fmuls f13, f1, f1
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E4D404: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82E4D408: ED411828  fsubs f10, f1, f3
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 82E4D40C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D550 size=516
    let mut pc: u32 = 0x82E4D550;
    'dispatch: loop {
        match pc {
            0x82E4D550 => {
    //   block [0x82E4D550..0x82E4D754)
	// 82E4D550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4D558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4D55C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82E4D560: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E4D564: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E4D568: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D56C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E4D570: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4D574: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E4D578: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4D57C: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 82E4D580: 409901B0  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D584: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D588: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D58C: 409901A4  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D590: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D594: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D598: 40990198  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D59C: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D5A0: FF0D1000  fcmpu cr6, f13, f2
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[2].f64);
	// 82E4D5A4: 4099018C  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D5A8: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 82E4D5AC: 40990184  ble cr6, 0x82e4d730
	if !ctx.cr[6].gt {
	pc = 0x82E4D730; continue 'dispatch;
	}
	// 82E4D5B0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E4D5B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4D5B8: D00100B0  stfs f0, 0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82E4D5BC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4D5C0: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 82E4D5C4: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4D5C8: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4D758 size=608
    let mut pc: u32 = 0x82E4D758;
    'dispatch: loop {
        match pc {
            0x82E4D758 => {
    //   block [0x82E4D758..0x82E4D9B8)
	// 82E4D758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D75C: 4BE5BCA1  bl 0x82ca93fc
	ctx.lr = 0x82E4D760;
	sub_82CA93D0(ctx, base);
	// 82E4D760: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82E4D764: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82E4D768: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E4D76C: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4D9B8 size=1104
    let mut pc: u32 = 0x82E4D9B8;
    'dispatch: loop {
        match pc {
            0x82E4D9B8 => {
    //   block [0x82E4D9B8..0x82E4DE08)
	// 82E4D9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4D9BC: 4BE5BA15  bl 0x82ca93d0
	ctx.lr = 0x82E4D9C0;
	sub_82CA93D0(ctx, base);
	// 82E4D9C0: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82E4D9C4: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82E4D9C8: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82E4D9CC: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4D9D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4D9D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4D9D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E4D9DC: 3B230088  addi r25, r3, 0x88
	ctx.r[25].s64 = ctx.r[3].s64 + 136;
	// 82E4D9E0: 3B030084  addi r24, r3, 0x84
	ctx.r[24].s64 = ctx.r[3].s64 + 132;
	// 82E4D9E4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4D9E8: 3AE30080  addi r23, r3, 0x80
	ctx.r[23].s64 = ctx.r[3].s64 + 128;
	// 82E4D9EC: C3AA0C18  lfs f29, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E4D9F0: 3AC3007C  addi r22, r3, 0x7c
	ctx.r[22].s64 = ctx.r[3].s64 + 124;
	// 82E4D9F4: D3A30088  stfs f29, 0x88(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82E4D9F8: 3AA30078  addi r21, r3, 0x78
	ctx.r[21].s64 = ctx.r[3].s64 + 120;
	// 82E4D9FC: D3A30084  stfs f29, 0x84(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E4DA00: 3A830074  addi r20, r3, 0x74
	ctx.r[20].s64 = ctx.r[3].s64 + 116;
	// 82E4DA04: D3A30080  stfs f29, 0x80(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E4DA08: 3A630070  addi r19, r3, 0x70
	ctx.r[19].s64 = ctx.r[3].s64 + 112;
	// 82E4DA0C: D3A3007C  stfs f29, 0x7c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E4DA10: 3A43006C  addi r18, r3, 0x6c
	ctx.r[18].s64 = ctx.r[3].s64 + 108;
	// 82E4DA14: D3A30078  stfs f29, 0x78(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4DA18: 3A230068  addi r17, r3, 0x68
	ctx.r[17].s64 = ctx.r[3].s64 + 104;
	// 82E4DA1C: D3A30074  stfs f29, 0x74(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E4DA20: 3BC30064  addi r30, r3, 0x64
	ctx.r[30].s64 = ctx.r[3].s64 + 100;
	// 82E4DA24: D3A30070  stfs f29, 0x70(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E4DA28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4DA2C: D3A3006C  stfs f29, 0x6c(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E4DA30: D3A30068  stfs f29, 0x68(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E4DA34: D3A30064  stfs f29, 0x64(r3)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4DA38: 40990340  ble cr6, 0x82e4dd78
	if !ctx.cr[6].gt {
	pc = 0x82E4DD78; continue 'dispatch;
	}
	// 82E4DA3C: 7D705B78  mr r16, r11
	ctx.r[16].u64 = ctx.r[11].u64;
	// 82E4DA40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DA44: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82E4DA48: 3D405555  lis r10, 0x5555
	ctx.r[10].s64 = 1431633920;
	// 82E4DA4C: 3B5D000C  addi r26, r29, 0xc
	ctx.r[26].s64 = ctx.r[29].s64 + 12;
	// 82E4DA50: 7DFC7B78  mr r28, r15
	ctx.r[28].u64 = ctx.r[15].u64;
	// 82E4DA54: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E4DA58: 615F5556  ori r31, r10, 0x5556
	ctx.r[31].u64 = ctx.r[10].u64 | 21846;
	// 82E4DA5C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4DE08 size=564
    let mut pc: u32 = 0x82E4DE08;
    'dispatch: loop {
        match pc {
            0x82E4DE08 => {
    //   block [0x82E4DE08..0x82E4E03C)
	// 82E4DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4DE10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4DE14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4DE18: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E4DE1C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E4DE20: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4DE24: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4DE28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DE2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4DE30: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4DE34: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4DE38: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4DE3C: 4199000C  bgt cr6, 0x82e4de48
	if ctx.cr[6].gt {
	pc = 0x82E4DE48; continue 'dispatch;
	}
	// 82E4DE40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4DE44: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E4DE48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E4DE4C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4DE50: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 82E4DE54: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4DE58: 4BFFF1A1  bl 0x82e4cff8
	ctx.lr = 0x82E4DE5C;
	sub_82E4CFF8(ctx, base);
	// 82E4DE5C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4DE60: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4DE64: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4DE68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E040 size=496
    let mut pc: u32 = 0x82E4E040;
    'dispatch: loop {
        match pc {
            0x82E4E040 => {
    //   block [0x82E4E040..0x82E4E230)
	// 82E4E040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E04C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E050: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E4E054: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E4E058: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E05C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4E060: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4E064: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E068: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4E06C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4E070: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4E074: 4199000C  bgt cr6, 0x82e4e080
	if ctx.cr[6].gt {
	pc = 0x82E4E080; continue 'dispatch;
	}
	// 82E4E078: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E07C: 48000194  b 0x82e4e210
	pc = 0x82E4E210; continue 'dispatch;
	// 82E4E080: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E4E084: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E088: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4E08C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E090: 4BFFEF69  bl 0x82e4cff8
	ctx.lr = 0x82E4E094;
	sub_82E4CFF8(ctx, base);
	// 82E4E094: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E4E098: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E4E09C: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4E0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E230 size=1664
    let mut pc: u32 = 0x82E4E230;
    'dispatch: loop {
        match pc {
            0x82E4E230 => {
    //   block [0x82E4E230..0x82E4E8B0)
	// 82E4E230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E234: 4BE5B1D5  bl 0x82ca9408
	ctx.lr = 0x82E4E238;
	sub_82CA93D0(ctx, base);
	// 82E4E238: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E4E23C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E240: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E4E244: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4E248: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4E24C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E250: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82E4E254: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4E258: 41990014  bgt cr6, 0x82e4e26c
	if ctx.cr[6].gt {
	pc = 0x82E4E26C; continue 'dispatch;
	}
	// 82E4E25C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E260: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4E264: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E4E268: 4BE5B1F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E4E26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4E274: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4E278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E4E27C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4E280: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82E4E284: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E4E288: 40990018  ble cr6, 0x82e4e2a0
	if !ctx.cr[6].gt {
	pc = 0x82E4E2A0; continue 'dispatch;
	}
	// 82E4E28C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4E290: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4E294: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4E298: 4BF08C79  bl 0x82d56f10
	ctx.lr = 0x82E4E29C;
	sub_82D56F10(ctx, base);
	// 82E4E29C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E2A0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4E2A4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E4E2A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4E2AC: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82E4E2B0: 419800F4  blt cr6, 0x82e4e3a4
	if ctx.cr[6].lt {
	pc = 0x82E4E3A4; continue 'dispatch;
	}
	// 82E4E2B4: 393CFFFC  addi r9, r28, -4
	ctx.r[9].s64 = ctx.r[28].s64 + -4;
	// 82E4E2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E2BC: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4E2C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4E2C4: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4E2C8: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2CC: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82E4E2D0: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82E4E2D4: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E2D8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2DC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E2E0: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E2E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E2E8: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E2EC: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E2F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E2F4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E2F8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E2FC: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E300: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E304: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E308: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E30C: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4E310: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E314: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E318: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E31C: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4E320: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E324: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E328: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E32C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E330: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4E334: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E338: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E33C: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82E4E340: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82E4E344: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E348: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E34C: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E350: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4E354: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E358: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E35C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E360: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E364: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4E368: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E36C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E370: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4E374: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E378: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E37C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E380: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4E384: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E388: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E38C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E390: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E394: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E4E398: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4E39C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3A0: 409AFF28  bne cr6, 0x82e4e2c8
	if !ctx.cr[6].eq {
	pc = 0x82E4E2C8; continue 'dispatch;
	}
	// 82E4E3A4: 7F06E000  cmpw cr6, r6, r28
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E4E3A8: 4098004C  bge cr6, 0x82e4e3f4
	if !ctx.cr[6].lt {
	pc = 0x82E4E3F4; continue 'dispatch;
	}
	// 82E4E3AC: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4E3B0: 7D26E050  subf r9, r6, r28
	ctx.r[9].s64 = ctx.r[28].s64 - ctx.r[6].s64;
	// 82E4E3B4: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3B8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E3BC: 7C0B252E  stfsx f0, r11, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 82E4E3C0: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3C4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E3CC: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E3D0: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E3D4: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3D8: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E3DC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E4E3E0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4E3E4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E4E3E8: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E3EC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4E3F0: 409AFFC4  bne cr6, 0x82e4e3b4
	if !ctx.cr[6].eq {
	pc = 0x82E4E3B4; continue 'dispatch;
	}
	// 82E4E3F4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E4E3F8: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E3FC: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82E4E400: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E4E404: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E4E408: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4E8B0 size=668
    let mut pc: u32 = 0x82E4E8B0;
    'dispatch: loop {
        match pc {
            0x82E4E8B0 => {
    //   block [0x82E4E8B0..0x82E4EB4C)
	// 82E4E8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E8B4: 4BE5AB4D  bl 0x82ca9400
	ctx.lr = 0x82E4E8B8;
	sub_82CA93D0(ctx, base);
	// 82E4E8B8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E4E8BC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E8C0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4E8C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4E8C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E8CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E8D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4E8D4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E4E8D8: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E8DC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82E4E8E0: 41990014  bgt cr6, 0x82e4e8f4
	if ctx.cr[6].gt {
	pc = 0x82E4E8F4; continue 'dispatch;
	}
	// 82E4E8E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4E8E8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4E8EC: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4E8F0: 4BE5AB60  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82E4E8F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4E8F8: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82E4E8FC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E4E900: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82E4E904: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82E4E908: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E4E90C: 4099001C  ble cr6, 0x82e4e928
	if !ctx.cr[6].gt {
	pc = 0x82E4E928; continue 'dispatch;
	}
	// 82E4E910: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4E914: 41980008  blt cr6, 0x82e4e91c
	if ctx.cr[6].lt {
	pc = 0x82E4E91C; continue 'dispatch;
	}
	// 82E4E918: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4E91C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4E920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E924: 4BF085ED  bl 0x82d56f10
	ctx.lr = 0x82E4E928;
	sub_82D56F10(ctx, base);
	// 82E4E928: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82E4E92C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82E4E930: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4E934: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82E4E938: 419800F4  blt cr6, 0x82e4ea2c
	if ctx.cr[6].lt {
	pc = 0x82E4EA2C; continue 'dispatch;
	}
	// 82E4E93C: 393DFFFC  addi r9, r29, -4
	ctx.r[9].s64 = ctx.r[29].s64 + -4;
	// 82E4E940: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E4E944: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4E948: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4E94C: 5526103A  slwi r6, r9, 2
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E4E950: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E954: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E958: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82E4E95C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4E960: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4E964: 7C0B3D2E  stfsx f0, r11, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4E968: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E96C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E970: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E974: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4E978: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E97C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E980: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E984: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E988: D0070008  stfs f0, 8(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4E98C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E990: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E994: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E998: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4E99C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9A0: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9A4: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9A8: D0070014  stfs f0, 0x14(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E4E9AC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9B0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9B4: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E9B8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9BC: D0070018  stfs f0, 0x18(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E4E9C0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9C4: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9C8: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82E4E9CC: D007FFF0  stfs f0, -0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82E4E9D0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9D4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9D8: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9DC: D0070024  stfs f0, 0x24(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E4E9E0: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9E4: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9E8: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4E9EC: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E4E9F0: D0070028  stfs f0, 0x28(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E4E9F4: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E9F8: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4E9FC: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82E4EA00: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA04: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA08: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA0C: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E4EA10: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA14: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA18: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4EA1C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA20: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E4EA24: D0080038  stfs f0, 0x38(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E4EA28: 409AFF28  bne cr6, 0x82e4e950
	if !ctx.cr[6].eq {
	pc = 0x82E4E950; continue 'dispatch;
	}
	// 82E4EA2C: 7F06E800  cmpw cr6, r6, r29
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E4EA30: 4098004C  bge cr6, 0x82e4ea7c
	if !ctx.cr[6].lt {
	pc = 0x82E4EA7C; continue 'dispatch;
	}
	// 82E4EA34: 54CB2036  slwi r11, r6, 4
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4EA38: 7D26E850  subf r9, r6, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[6].s64;
	// 82E4EA3C: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA40: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA44: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E4EA48: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E4EA4C: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82E4EA50: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA54: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA58: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA5C: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4EA60: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EA64: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EA68: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E4EA6C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4EA70: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E4EA74: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4EA78: 409AFFC4  bne cr6, 0x82e4ea3c
	if !ctx.cr[6].eq {
	pc = 0x82E4EA3C; continue 'dispatch;
	}
	// 82E4EA7C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E4EA80: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82E4EA84: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E4EA88: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82E4EA8C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4EA90: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82E4EA94: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E4EA98: 93610088  stw r27, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 82E4EA9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4EAA0: 9381008C  stw r28, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 82E4EAA4: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E4EAA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EAAC: 93810090  stw r28, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 82E4EAB0: 93610094  stw r27, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 82E4EAB4: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82E4EAB8: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4EABC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E4EAC0: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82E4EAC4: 48295CA5  bl 0x830e4768
	ctx.lr = 0x82E4EAC8;
	sub_830E4768(ctx, base);
	// 82E4EAC8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E4EACC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4EAD0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4EAD4: 4BFFF335  bl 0x82e4de08
	ctx.lr = 0x82E4EAD8;
	sub_82E4DE08(ctx, base);
	// 82E4EAD8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4EADC: D3FA0004  stfs f31, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4EAE0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4EAE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4EAE8: 409A0020  bne cr6, 0x82e4eb08
	if !ctx.cr[6].eq {
	pc = 0x82E4EB08; continue 'dispatch;
	}
	// 82E4EAEC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4EAF0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4EAF4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4EAF8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4EAFC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4EB00: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4EB04: 4BF067C5  bl 0x82d552c8
	ctx.lr = 0x82E4EB08;
	sub_82D552C8(ctx, base);
	// 82E4EB08: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4EB0C: 4BFF76BD  bl 0x82e461c8
	ctx.lr = 0x82E4EB10;
	sub_82E461C8(ctx, base);
	// 82E4EB10: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4EB14: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4EB18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4EB1C: 409A0020  bne cr6, 0x82e4eb3c
	if !ctx.cr[6].eq {
	pc = 0x82E4EB3C; continue 'dispatch;
	}
	// 82E4EB20: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4EB24: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4EB28: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4EB2C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EB30: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4EB34: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4EB38: 4BF06791  bl 0x82d552c8
	ctx.lr = 0x82E4EB3C;
	sub_82D552C8(ctx, base);
	// 82E4EB3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E4EB40: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E4EB44: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4EB48: 4BE5A908  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4EB50 size=2688
    let mut pc: u32 = 0x82E4EB50;
    'dispatch: loop {
        match pc {
            0x82E4EB50 => {
    //   block [0x82E4EB50..0x82E4F5D0)
	// 82E4EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EB54: 4BE5A8B5  bl 0x82ca9408
	ctx.lr = 0x82E4EB58;
	sub_82CA93D0(ctx, base);
	// 82E4EB58: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82E4EB5C: 4BE5F165  bl 0x82cadcc0
	ctx.lr = 0x82E4EB60;
	sub_82CADCA0(ctx, base);
	// 82E4EB60: 9421FAA0  stwu r1, -0x560(r1)
	ea = ctx.r[1].u32.wrapping_add(-1376 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EB64: FEC01090  fmr f22, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[22].f64 = ctx.f[2].f64;
	// 82E4EB68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4EB6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EB70: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E4EB74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EB78: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E4EB7C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4EB80: FF16F800  fcmpu cr6, f22, f31
	ctx.cr[6].compare_f64(ctx.f[22].f64, ctx.f[31].f64);
	// 82E4EB84: 41990018  bgt cr6, 0x82e4eb9c
	if ctx.cr[6].gt {
	pc = 0x82E4EB9C; continue 'dispatch;
	}
	// 82E4EB88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E4EB8C: 38210560  addi r1, r1, 0x560
	ctx.r[1].s64 = ctx.r[1].s64 + 1376;
	// 82E4EB90: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82E4EB94: 4BE5F179  bl 0x82cadd0c
	ctx.lr = 0x82E4EB98;
	sub_82CADCEC(ctx, base);
	// 82E4EB98: 4BE5A8C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E4EB9C: FF1DF800  fcmpu cr6, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 82E4EBA0: 4099FFE8  ble cr6, 0x82e4eb88
	if !ctx.cr[6].gt {
	pc = 0x82E4EB88; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4F5D0 size=864
    let mut pc: u32 = 0x82E4F5D0;
    'dispatch: loop {
        match pc {
            0x82E4F5D0 => {
    //   block [0x82E4F5D0..0x82E4F930)
	// 82E4F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F5D4: 4BE59E39  bl 0x82ca940c
	ctx.lr = 0x82E4F5D8;
	sub_82CA93D0(ctx, base);
	// 82E4F5D8: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82E4F5DC: 4BE5E6FD  bl 0x82cadcd8
	ctx.lr = 0x82E4F5E0;
	sub_82CADCA0(ctx, base);
	// 82E4F5E0: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F5E4: FF801090  fmr f28, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[2].f64;
	// 82E4F5E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4F5EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F5F0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4F5F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F5F8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E4F5FC: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E4F600: FF1CF800  fcmpu cr6, f28, f31
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[31].f64);
	// 82E4F604: 40990318  ble cr6, 0x82e4f91c
	if !ctx.cr[6].gt {
	pc = 0x82E4F91C; continue 'dispatch;
	}
	// 82E4F608: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 82E4F60C: 40990310  ble cr6, 0x82e4f91c
	if !ctx.cr[6].gt {
	pc = 0x82E4F91C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4F930 size=644
    let mut pc: u32 = 0x82E4F930;
    'dispatch: loop {
        match pc {
            0x82E4F930 => {
    //   block [0x82E4F930..0x82E4FBB4)
	// 82E4F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F934: 4BE59AD1  bl 0x82ca9404
	ctx.lr = 0x82E4F938;
	sub_82CA93D0(ctx, base);
	// 82E4F938: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4F93C: 4BE5E39D  bl 0x82cadcd8
	ctx.lr = 0x82E4F940;
	sub_82CADCA0(ctx, base);
	// 82E4F940: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F944: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4F948: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E4F94C: 3941008C  addi r10, r1, 0x8c
	ctx.r[10].s64 = ctx.r[1].s64 + 140;
	// 82E4F950: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E4F954: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E4F958: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F95C: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82E4F960: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82E4F964: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82E4F968: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E4F96C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4F970: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82E4F974: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82E4F978: 4198009C  blt cr6, 0x82e4fa14
	if ctx.cr[6].lt {
	pc = 0x82E4FA14; continue 'dispatch;
	}
	// 82E4F97C: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E4F980: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F984: 7FFE502E  lwzx r31, r30, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4F988: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4F98C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4F990: 409A000C  bne cr6, 0x82e4f99c
	if !ctx.cr[6].eq {
	pc = 0x82E4F99C; continue 'dispatch;
	}
	// 82E4F994: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4F998: 48000010  b 0x82e4f9a8
	pc = 0x82E4F9A8; continue 'dispatch;
	// 82E4F99C: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4F9A0: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4F9A4: 409A0060  bne cr6, 0x82e4fa04
	if !ctx.cr[6].eq {
	pc = 0x82E4FA04; continue 'dispatch;
	}
	// 82E4F9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F9AC: 419A0058  beq cr6, 0x82e4fa04
	if ctx.cr[6].eq {
	pc = 0x82E4FA04; continue 'dispatch;
	}
	// 82E4F9B0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F9B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4F9B8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4F9BC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4F9C0: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4F9C4: 7D7E512E  stwx r11, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 82E4F9C8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4F9CC: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4F9D4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4F9D8: 409A0010  bne cr6, 0x82e4f9e8
	if !ctx.cr[6].eq {
	pc = 0x82E4F9E8; continue 'dispatch;
	}
	// 82E4F9DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E4F9E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4F9E4: 4BF075B5  bl 0x82d56f98
	ctx.lr = 0x82E4F9E8;
	sub_82D56F98(ctx, base);
	// 82E4F9E8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9EC: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4F9F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4F9F4: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82E4F9F8: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4F9FC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82E4FA00: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82E4FA04: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82E4FA08: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E4FA0C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E4FA10: 4098FF70  bge cr6, 0x82e4f980
	if !ctx.cr[6].lt {
	pc = 0x82E4F980; continue 'dispatch;
	}
	// 82E4FA14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4FA18: 3BC9FFFF  addi r30, r9, -1
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	// 82E4FA1C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4FA20: C38B0C18  lfs f28, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4FA24: FFA0E090  fmr f29, f28
	ctx.f[29].f64 = ctx.f[28].f64;
	// 82E4FA28: FFE0E090  fmr f31, f28
	ctx.f[31].f64 = ctx.f[28].f64;
	// 82E4FA2C: 4198004C  blt cr6, 0x82e4fa78
	if ctx.cr[6].lt {
	pc = 0x82E4FA78; continue 'dispatch;
	}
	// 82E4FA30: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E4FA34: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FA38: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4FA3C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FA40: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E4FA44: 409A0008  bne cr6, 0x82e4fa4c
	if !ctx.cr[6].eq {
	pc = 0x82E4FA4C; continue 'dispatch;
	}
	// 82E4FA48: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4FA4C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82E4FA50: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E4FA54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4FA58: 4BFFFED9  bl 0x82e4f930
	ctx.lr = 0x82E4FA5C;
	sub_82E4F930(ctx, base);
	// 82E4FA5C: EC01F828  fsubs f0, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FA60: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E4FA64: EFA1E82A  fadds f29, f1, f29
	ctx.f[29].f64 = ((ctx.f[1].f64 + ctx.f[29].f64) as f32) as f64;
	// 82E4FA68: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E4FA6C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4FA70: FFE0F86E  fsel f31, f0, f1, f31
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 82E4FA74: 4098FFC0  bge cr6, 0x82e4fa34
	if !ctx.cr[6].lt {
	pc = 0x82E4FA34; continue 'dispatch;
	}
	// 82E4FA78: 897B00D8  lbz r11, 0xd8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E4FA7C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E4FA80: 419A0010  beq cr6, 0x82e4fa90
	if ctx.cr[6].eq {
	pc = 0x82E4FA90; continue 'dispatch;
	}
	// 82E4FA84: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E4FA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4FA8C: 409A0008  bne cr6, 0x82e4fa94
	if !ctx.cr[6].eq {
	pc = 0x82E4FA94; continue 'dispatch;
	}
	// 82E4FA90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4FA94: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E4FA98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4FA9C: 419A0044  beq cr6, 0x82e4fae0
	if ctx.cr[6].eq {
	pc = 0x82E4FAE0; continue 'dispatch;
	}
	// 82E4FAA0: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4FAA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FAA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FAAC: 409A0020  bne cr6, 0x82e4facc
	if !ctx.cr[6].eq {
	pc = 0x82E4FACC; continue 'dispatch;
	}
	// 82E4FAB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FAB4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4FAB8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FABC: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FAC0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FAC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4FAC8: 4BF05801  bl 0x82d552c8
	ctx.lr = 0x82E4FACC;
	sub_82D552C8(ctx, base);
	// 82E4FACC: FC20E090  fmr f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[28].f64;
	// 82E4FAD0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E4FAD4: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4FAD8: 4BE5E24D  bl 0x82cadd24
	ctx.lr = 0x82E4FADC;
	sub_82CADCEC(ctx, base);
	// 82E4FADC: 4BE59978  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E4FAE0: 817B00D0  lwz r11, 0xd0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(208 as u32) ) } as u64;
	// 82E4FAE4: 387B00D0  addi r3, r27, 0xd0
	ctx.r[3].s64 = ctx.r[27].s64 + 208;
	// 82E4FAE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FAEC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FAF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FAF4: 4E800421  bctrl
	ctx.lr = 0x82E4FAF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FAF8: C0010064  lfs f0, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4FAFC: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E4FB00: FF1DE000  fcmpu cr6, f29, f28
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[28].f64);
	// 82E4FB04: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E4FB08: FD6C682E  fsel f11, f12, f0, f13
	ctx.f[11].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 82E4FB0C: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E4FB10: ED4C5828  fsubs f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E4FB14: FF8A5B2E  fsel f28, f10, f12, f11
	ctx.f[28].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[11].f64 };
	// 82E4FB18: 419AFF88  beq cr6, 0x82e4faa0
	if ctx.cr[6].eq {
	pc = 0x82E4FAA0; continue 'dispatch;
	}
	// 82E4FB1C: EFDC07B2  fmuls f30, f28, f30
	ctx.f[30].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E4FB20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FB24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E4FB28: ED7DF028  fsubs f11, f29, f30
	ctx.f[11].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 82E4FB2C: FD6BEFAE  fsel f11, f11, f30, f29
	ctx.f[11].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[29].f64 };
	// 82E4FB30: ED4BF828  fsubs f10, f11, f31
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB34: FFEAFAEE  fsel f31, f10, f11, f31
	ctx.f[31].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[31].f64 };
	// 82E4FB38: ED40F828  fsubs f10, f0, f31
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB3C: ED6CF828  fsubs f11, f12, f31
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB40: ED2DF828  fsubs f9, f13, f31
	ctx.f[9].f64 = (((ctx.f[13].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB44: FC0AF82E  fsel f0, f10, f0, f31
	ctx.f[0].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 82E4FB48: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E4FB4C: FD8BFB2E  fsel f12, f11, f12, f31
	ctx.f[12].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[31].f64 };
	// 82E4FB50: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E4FB54: FC09FB6E  fsel f0, f9, f13, f31
	ctx.f[0].f64 = if ctx.f[9].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[31].f64 };
	// 82E4FB58: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E4FB5C: 4BF343A5  bl 0x82d83f00
	ctx.lr = 0x82E4FB60;
	sub_82D83F00(ctx, base);
	// 82E4FB60: EC1CE82A  fadds f0, f28, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[28].f64 + ctx.f[29].f64) as f32) as f64;
	// 82E4FB64: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E4FB68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FB6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FB70: EDBE0028  fsubs f13, f30, f0
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E4FB74: FC0DF02E  fsel f0, f13, f0, f30
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[30].f64 };
	// 82E4FB78: EDA0F828  fsubs f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E4FB7C: FFEDF82E  fsel f31, f13, f0, f31
	ctx.f[31].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 82E4FB80: 409A0020  bne cr6, 0x82e4fba0
	if !ctx.cr[6].eq {
	pc = 0x82E4FBA0; continue 'dispatch;
	}
	// 82E4FB84: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FB88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4FB8C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FB90: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4FB94: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FB98: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4FB9C: 4BF0572D  bl 0x82d552c8
	ctx.lr = 0x82E4FBA0;
	sub_82D552C8(ctx, base);
	// 82E4FBA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4FBA4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E4FBA8: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82E4FBAC: 4BE5E179  bl 0x82cadd24
	ctx.lr = 0x82E4FBB0;
	sub_82CADCEC(ctx, base);
	// 82E4FBB0: 4BE598A4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FBB8 size=308
    let mut pc: u32 = 0x82E4FBB8;
    'dispatch: loop {
        match pc {
            0x82E4FBB8 => {
    //   block [0x82E4FBB8..0x82E4FCEC)
	// 82E4FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FBBC: 4BE59845  bl 0x82ca9400
	ctx.lr = 0x82E4FBC0;
	sub_82CA93D0(ctx, base);
	// 82E4FBC0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E4FBC4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4FBCC: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FBD0: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82E4FBD4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4FBD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4FBDC: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82E4FBE0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82E4FBE4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4FBE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4FBEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4FBF0: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E4FBF4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FBF8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E4FBFC: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82E4FC00: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4FC04: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4FC08: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E4FC0C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E4FC10: 4199000C  bgt cr6, 0x82e4fc1c
	if ctx.cr[6].gt {
	pc = 0x82E4FC1C; continue 'dispatch;
	}
	// 82E4FC14: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E4FC18: 4800001C  b 0x82e4fc34
	pc = 0x82E4FC34; continue 'dispatch;
	// 82E4FC1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FC20: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4FC24: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4FC28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FC2C: 4E800421  bctrl
	ctx.lr = 0x82E4FC30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FC30: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E4FC34: 7FEBF378  or r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 82E4FC38: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4FC3C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E4FC40: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E4FC44: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E4FC48: 4099003C  ble cr6, 0x82e4fc84
	if !ctx.cr[6].gt {
	pc = 0x82E4FC84; continue 'dispatch;
	}
	// 82E4FC4C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4FC50: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E4FC54: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E4FC58: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FC5C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E4FC60: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4FC64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E4FC68: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E4FC6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4FC70: 7D09392E  stwx r8, r9, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u32) };
	// 82E4FC74: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4FC78: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E4FC7C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E4FC80: 409AFFD8  bne cr6, 0x82e4fc58
	if !ctx.cr[6].eq {
	pc = 0x82E4FC58; continue 'dispatch;
	}
	// 82E4FC84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E4FC88: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4FC8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FC90: 4BFFFCA1  bl 0x82e4f930
	ctx.lr = 0x82E4FC94;
	sub_82E4F930(ctx, base);
	// 82E4FC94: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FC98: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4FC9C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E4FCA0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82E4FCA4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4FCA8: 409A0014  bne cr6, 0x82e4fcbc
	if !ctx.cr[6].eq {
	pc = 0x82E4FCBC; continue 'dispatch;
	}
	// 82E4FCAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FCB0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4FCB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FCB8: 4E800421  bctrl
	ctx.lr = 0x82E4FCBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FCBC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4FCC0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4FCC4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FCC8: 409A0018  bne cr6, 0x82e4fce0
	if !ctx.cr[6].eq {
	pc = 0x82E4FCE0; continue 'dispatch;
	}
	// 82E4FCCC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4FCD0: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E4FCD4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4FCD8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4FCDC: 4BF055ED  bl 0x82d552c8
	ctx.lr = 0x82E4FCE0;
	sub_82D552C8(ctx, base);
	// 82E4FCE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4FCE4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4FCE8: 4BE59768  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4FCF0 size=2232
    let mut pc: u32 = 0x82E4FCF0;
    'dispatch: loop {
        match pc {
            0x82E4FCF0 => {
    //   block [0x82E4FCF0..0x82E505A8)
	// 82E4FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FCF4: 4BE596F9  bl 0x82ca93ec
	ctx.lr = 0x82E4FCF8;
	sub_82CA93D0(ctx, base);
	// 82E4FCF8: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E4FCFC: 4BE5DFC1  bl 0x82cadcbc
	ctx.lr = 0x82E4FD00;
	sub_82CADCA0(ctx, base);
	// 82E4FD00: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E505A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E505A8 size=1228
    let mut pc: u32 = 0x82E505A8;
    'dispatch: loop {
        match pc {
            0x82E505A8 => {
    //   block [0x82E505A8..0x82E50A74)
	// 82E505A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E505AC: 4BE58E49  bl 0x82ca93f4
	ctx.lr = 0x82E505B0;
	sub_82CA93D0(ctx, base);
	// 82E505B0: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82E505B4: 4BE5D71D  bl 0x82cadcd0
	ctx.lr = 0x82E505B8;
	sub_82CADCA0(ctx, base);
	// 82E505B8: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50A78 size=1704
    //   switch @ 0x82E50B14: r11 with 27 label(s)
    //       case  0 → 0x82E50E38
    //       case  1 → 0x82E50B84
    //       case  2 → 0x82E50EE4
    //       case  3 → 0x82E50C84
    //       case  4 → 0x82E50B9C
    //       case  5 → 0x82E50EC4
    //       case  6 → 0x82E50BC0
    //       case  7 → 0x82E50BC0
    //       case  8 → 0x82E50E38
    //       case  9 → 0x82E50E38
    //       case 10 → 0x82E50E38
    //       case 11 → 0x82E50CE4
    //       case 12 → 0x82E50D50
    //       case 13 → 0x82E51108
    //       case 14 → 0x82E51108
    //       case 15 → 0x82E51108
    //       case 16 → 0x82E50D98
    //       case 17 → 0x82E50E38
    //       case 18 → 0x82E50E38
    //       case 19 → 0x82E50E38
    //       case 20 → 0x82E51108
    //       case 21 → 0x82E51108
    //       case 22 → 0x82E51108
    //       case 23 → 0x82E51108
    //       case 24 → 0x82E50CD0
    //       case 25 → 0x82E51108
    //       case 26 → 0x82E50D74
    let mut pc: u32 = 0x82E50A78;
    'dispatch: loop {
        match pc {
            0x82E50A78 => {
    //   block [0x82E50A78..0x82E50B84)
	// 82E50A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50A7C: 4BE58979  bl 0x82ca93f4
	ctx.lr = 0x82E50A80;
	sub_82CA93D0(ctx, base);
	// 82E50A80: DBC1FFA0  stfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 82E50A84: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82E50A88: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E50A8C: 481B5F29  bl 0x830069b4
	ctx.lr = 0x82E50A90;
	sub_83006760(ctx, base);
	// 82E50A90: 9421FA50  stwu r1, -0x5b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1456 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50A94: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82E50A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50A9C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E50AA0: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	pc = 0x82E50B84; continue 'dispatch;
            }
            0x82E50B84 => {
    //   block [0x82E50B84..0x82E50B9C)
	// 82E50B84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50B88: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50B8C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E50B90: C04B0C14  lfs f2, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E50B94: 4BFFBA55  bl 0x82e4c5e8
	ctx.lr = 0x82E50B98;
	sub_82E4C5E8(ctx, base);
	// 82E50B98: 48000374  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50B9C => {
    //   block [0x82E50B9C..0x82E50BC0)
	// 82E50B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x82E50BC0; continue 'dispatch;
            }
            0x82E50BC0 => {
    //   block [0x82E50BC0..0x82E50C84)
	// 82E50BC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BC4: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82E50BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50BCC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E50BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50BD4: 4E800421  bctrl
	ctx.lr = 0x82E50BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50BD8: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BDC: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82E50BE0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E50BE4: 39790001  addi r11, r25, 1
	ctx.r[11].s64 = ctx.r[25].s64 + 1;
	// 82E50BE8: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E50BEC: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E50BF0: 83A30020  lwz r29, 0x20(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E50BF4: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E50BF8: 7D7D2214  add r11, r29, r4
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[4].u64;
	// 82E50BFC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E50C00: 4199000C  bgt cr6, 0x82e50c0c
	if ctx.cr[6].gt {
	pc = 0x82E50C0C; continue 'dispatch;
	}
	// 82E50C04: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E50C08: 48000018  b 0x82e50c20
	pc = 0x82E50C20; continue 'dispatch;
	// 82E50C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50C14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C18: 4E800421  bctrl
	ctx.lr = 0x82E50C1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50C20: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E50C24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E50C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50C2C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82E50C30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C34: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C3C: 4E800421  bctrl
	ctx.lr = 0x82E50C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50C44: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E50C48: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E50C4C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E50C50: C02B0C14  lfs f1, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50C54: 4BFFDC5D  bl 0x82e4e8b0
	ctx.lr = 0x82E50C58;
	sub_82E4E8B0(ctx, base);
	// 82E50C58: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E50C5C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50C60: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82E50C64: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E50C68: 409A02A4  bne cr6, 0x82e50f0c
	if !ctx.cr[6].eq {
	pc = 0x82E50F0C; continue 'dispatch;
	}
	// 82E50C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50C70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E50C74: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50C7C: 4E800421  bctrl
	ctx.lr = 0x82E50C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50C80: 4800028C  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50C84 => {
    //   block [0x82E50C84..0x82E50CD0)
	// 82E50C84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	pc = 0x82E50CD0; continue 'dispatch;
            }
            0x82E50CD0 => {
    //   block [0x82E50CD0..0x82E50CE4)
	// 82E50CD0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50CD4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50CD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50CDC: 4BFFFD9D  bl 0x82e50a78
	ctx.lr = 0x82E50CE0;
	sub_82E50A78(ctx, base);
	// 82E50CE0: 48000428  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50CE4 => {
    //   block [0x82E50CE4..0x82E50D50)
	// 82E50CE4: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82E50CE8: 38A100E0  addi r5, r1, 0xe0
	ctx.r[5].s64 = ctx.r[1].s64 + 224;
	// 82E50CEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50CF0: 38610250  addi r3, r1, 0x250
	ctx.r[3].s64 = ctx.r[1].s64 + 592;
	pc = 0x82E50D50; continue 'dispatch;
            }
            0x82E50D50 => {
    //   block [0x82E50D50..0x82E50D74)
	// 82E50D50: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82E50D54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50D58: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 82E50D5C: 4BF065A5  bl 0x82d57300
	ctx.lr = 0x82E50D60;
	sub_82D57300(ctx, base);
	// 82E50D60: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50D64: 38810290  addi r4, r1, 0x290
	ctx.r[4].s64 = ctx.r[1].s64 + 656;
	// 82E50D68: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50D6C: 4BFFFD0D  bl 0x82e50a78
	ctx.lr = 0x82E50D70;
	sub_82E50A78(ctx, base);
	// 82E50D70: 48000398  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50D74 => {
    //   block [0x82E50D74..0x82E50D98)
	// 82E50D74: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82E50D78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50D7C: 38610210  addi r3, r1, 0x210
	ctx.r[3].s64 = ctx.r[1].s64 + 528;
	// 82E50D80: 4BF06581  bl 0x82d57300
	ctx.lr = 0x82E50D84;
	sub_82D57300(ctx, base);
	// 82E50D84: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50D88: 38810210  addi r4, r1, 0x210
	ctx.r[4].s64 = ctx.r[1].s64 + 528;
	// 82E50D8C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50D90: 4BFFFCE9  bl 0x82e50a78
	ctx.lr = 0x82E50D94;
	sub_82E50A78(ctx, base);
	// 82E50D94: 48000374  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50D98 => {
    //   block [0x82E50D98..0x82E50E38)
	// 82E50D98: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E50D9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E50DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50DA4: 40990364  ble cr6, 0x82e51108
	if !ctx.cr[6].gt {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50DA8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E50DAC: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82E50DB0: 3B5C0020  addi r26, r28, 0x20
	ctx.r[26].s64 = ctx.r[28].s64 + 32;
	// 82E50DB4: 3B3C0030  addi r25, r28, 0x30
	ctx.r[25].s64 = ctx.r[28].s64 + 48;
	// 82E50DB8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E50DBC: 3B0B39E0  addi r24, r11, 0x39e0
	ctx.r[24].s64 = ctx.r[11].s64 + 14816;
	// 82E50DC0: 39610120  addi r11, r1, 0x120
	ctx.r[11].s64 = ctx.r[1].s64 + 288;
	pc = 0x82E50E38; continue 'dispatch;
            }
            0x82E50E38 => {
    //   block [0x82E50E38..0x82E50EC4)
	// 82E50E38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50E40: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E48: 4E800421  bctrl
	ctx.lr = 0x82E50E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50E50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E54: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E5C: 4E800421  bctrl
	ctx.lr = 0x82E50E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50E64: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E50E68: 419A02A0  beq cr6, 0x82e51108
	if ctx.cr[6].eq {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50E6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E70: 38A102D0  addi r5, r1, 0x2d0
	ctx.r[5].s64 = ctx.r[1].s64 + 720;
	// 82E50E74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E50E78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50E7C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E50E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E84: 4E800421  bctrl
	ctx.lr = 0x82E50E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50E8C: 419A0010  beq cr6, 0x82e50e9c
	if ctx.cr[6].eq {
	pc = 0x82E50E9C; continue 'dispatch;
	}
	// 82E50E90: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82E50E94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50E98: 4BFFFBE1  bl 0x82e50a78
	ctx.lr = 0x82E50E9C;
	sub_82E50A78(ctx, base);
	// 82E50E9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50EA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E50EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50EA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E50EAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50EB0: 4E800421  bctrl
	ctx.lr = 0x82E50EB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50EB8: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E50EBC: 409AFFB0  bne cr6, 0x82e50e6c
	if !ctx.cr[6].eq {
	pc = 0x82E50E6C; continue 'dispatch;
	}
	// 82E50EC0: 48000248  b 0x82e51108
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E50EC4 => {
    //   block [0x82E50EC4..0x82E50EE4)
	// 82E50EC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50EC8: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50ECC: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E50ED0: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E50ED4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E50ED8: C04B0C14  lfs f2, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E50EDC: 4BFFDC75  bl 0x82e4eb50
	ctx.lr = 0x82E50EE0;
	sub_82E4EB50(ctx, base);
	// 82E50EE0: 4800002C  b 0x82e50f0c
	pc = 0x82E50F0C; continue 'dispatch;
            }
            0x82E50EE4 => {
    //   block [0x82E50EE4..0x82E51108)
	// 82E50EE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50EEC: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82E50EF0: C3EB0C14  lfs f31, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E50EF4: 4BF6C14D  bl 0x82dbd040
	ctx.lr = 0x82E50EF8;
	sub_82DBD040(ctx, base);
	// 82E50EF8: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82E50EFC: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82E50F00: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E50F04: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E50F08: 4BFFE6C9  bl 0x82e4f5d0
	ctx.lr = 0x82E50F0C;
	sub_82E4F5D0(ctx, base);
	// 82E50F0C: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50F10: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 82E50F14: 419A01F4  beq cr6, 0x82e51108
	if ctx.cr[6].eq {
	pc = 0x82E51108; continue 'dispatch;
	}
	// 82E50F18: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E50F1C: 4BFFB9ED  bl 0x82e4c908
	ctx.lr = 0x82E50F20;
	sub_82E4C908(ctx, base);
	// 82E50F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E50F24: 38610180  addi r3, r1, 0x180
	ctx.r[3].s64 = ctx.r[1].s64 + 384;
	// 82E50F28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E50F2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E50F30: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E50F34: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E50F38: 480004E9  bl 0x82e51420
	ctx.lr = 0x82E50F3C;
	sub_82E51420(ctx, base);
	// 82E50F3C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E50F40: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82E50F44: C3F70000  lfs f31, 0(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E50F48: 39570020  addi r10, r23, 0x20
	ctx.r[10].s64 = ctx.r[23].s64 + 32;
	// 82E50F4C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E50F50: C3D70004  lfs f30, 4(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E50F54: 39610190  addi r11, r1, 0x190
	ctx.r[11].s64 = ctx.r[1].s64 + 400;
	// 82E50F58: D3E10180  stfs f31, 0x180(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82E50F5C: D3C10184  stfs f30, 0x184(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), tmp.u32 ) };
	pc = 0x82E51108; continue 'dispatch;
            }
            0x82E51108 => {
    //   block [0x82E51108..0x82E51120)
	// 82E51108: 382105B0  addi r1, r1, 0x5b0
	ctx.r[1].s64 = ctx.r[1].s64 + 1456;
	// 82E5110C: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82E51110: 481B5B3D  bl 0x83006c4c
	ctx.lr = 0x82E51114;
	sub_830069F8(ctx, base);
	// 82E51114: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82E51118: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82E5111C: 4BE58328  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51120 size=380
    let mut pc: u32 = 0x82E51120;
    'dispatch: loop {
        match pc {
            0x82E51120 => {
    //   block [0x82E51120..0x82E5129C)
	// 82E51120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5112C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E51130: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E51134: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51138: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E5113C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E51140: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E51144: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E51148: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E512A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E512A0 size=384
    let mut pc: u32 = 0x82E512A0;
    'dispatch: loop {
        match pc {
            0x82E512A0 => {
    //   block [0x82E512A0..0x82E51420)
	// 82E512A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E512A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E512A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E512AC: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E512B0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E512B4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E512B8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E512BC: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E512C0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E512C4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E512C8: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E51420 size=136
    let mut pc: u32 = 0x82E51420;
    'dispatch: loop {
        match pc {
            0x82E51420 => {
    //   block [0x82E51420..0x82E514A8)
	// 82E51420: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E51424: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82E51428: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82E5142C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E51430: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E51434: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51438: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E5143C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E51440: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E51444: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E514A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E514A8 size=72
    let mut pc: u32 = 0x82E514A8;
    'dispatch: loop {
        match pc {
            0x82E514A8 => {
    //   block [0x82E514A8..0x82E514F0)
	// 82E514A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E514AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E514B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E514B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E514B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E514BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E514C0: 396B7228  addi r11, r11, 0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + 29224;
	// 82E514C4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E514C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E514CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E514D0: 419A000C  beq cr6, 0x82e514dc
	if ctx.cr[6].eq {
	pc = 0x82E514DC; continue 'dispatch;
	}
	// 82E514D4: 4B9F42DD  bl 0x828457b0
	ctx.lr = 0x82E514D8;
	sub_828457B0(ctx, base);
	// 82E514D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E514DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E514E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E514E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E514E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E514EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E514F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E514F0 size=96
    let mut pc: u32 = 0x82E514F0;
    'dispatch: loop {
        match pc {
            0x82E514F0 => {
    //   block [0x82E514F0..0x82E51550)
	// 82E514F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E514F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E514F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E514FC: 48012105  bl 0x82e63600
	ctx.lr = 0x82E51500;
	sub_82E63600(ctx, base);
	// 82E51500: 480119E1  bl 0x82e62ee0
	ctx.lr = 0x82E51504;
	sub_82E62EE0(ctx, base);
	// 82E51504: 48010E7D  bl 0x82e62380
	ctx.lr = 0x82E51508;
	sub_82E62380(ctx, base);
	// 82E51508: 48010431  bl 0x82e61938
	ctx.lr = 0x82E5150C;
	sub_82E61938(ctx, base);
	// 82E5150C: 4800F2C5  bl 0x82e607d0
	ctx.lr = 0x82E51510;
	sub_82E607D0(ctx, base);
	// 82E51510: 4800EDA9  bl 0x82e602b8
	ctx.lr = 0x82E51514;
	sub_82E602B8(ctx, base);
	// 82E51514: 4800EC7D  bl 0x82e60190
	ctx.lr = 0x82E51518;
	sub_82E60190(ctx, base);
	// 82E51518: 4800EAE9  bl 0x82e60000
	ctx.lr = 0x82E5151C;
	sub_82E60000(ctx, base);
	// 82E5151C: 4800DD6D  bl 0x82e5f288
	ctx.lr = 0x82E51520;
	sub_82E5F288(ctx, base);
	// 82E51520: 4800D301  bl 0x82e5e820
	ctx.lr = 0x82E51524;
	sub_82E5E820(ctx, base);
	// 82E51524: 4800CCED  bl 0x82e5e210
	ctx.lr = 0x82E51528;
	sub_82E5E210(ctx, base);
	// 82E51528: 4800B789  bl 0x82e5ccb0
	ctx.lr = 0x82E5152C;
	sub_82E5CCB0(ctx, base);
	// 82E5152C: 4800A715  bl 0x82e5bc40
	ctx.lr = 0x82E51530;
	sub_82E5BC40(ctx, base);
	// 82E51530: 48009A89  bl 0x82e5afb8
	ctx.lr = 0x82E51534;
	sub_82E5AFB8(ctx, base);
	// 82E51534: 48009345  bl 0x82e5a878
	ctx.lr = 0x82E51538;
	sub_82E5A878(ctx, base);
	// 82E51538: 48008721  bl 0x82e59c58
	ctx.lr = 0x82E5153C;
	sub_82E59C58(ctx, base);
	// 82E5153C: 4800813D  bl 0x82e59678
	ctx.lr = 0x82E51540;
	sub_82E59678(ctx, base);
	// 82E51540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51550 size=12
    let mut pc: u32 = 0x82E51550;
    'dispatch: loop {
        match pc {
            0x82E51550 => {
    //   block [0x82E51550..0x82E5155C)
	// 82E51550: 8063FFB0  lwz r3, -0x50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-80 as u32) ) } as u64;
	// 82E51554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51558: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5155C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5155C size=20
    let mut pc: u32 = 0x82E5155C;
    'dispatch: loop {
        match pc {
            0x82E5155C => {
    //   block [0x82E5155C..0x82E51570)
	// 82E5155C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82E51560: 38CB87A4  addi r6, r11, -0x785c
	ctx.r[6].s64 = ctx.r[11].s64 + -30812;
	// 82E51564: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E51568: 38AB8860  addi r5, r11, -0x77a0
	ctx.r[5].s64 = ctx.r[11].s64 + -30624;
	// 82E5156C: 4BFE56AC  b 0x82e36c18
	sub_82E36C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51570 size=4
    let mut pc: u32 = 0x82E51570;
    'dispatch: loop {
        match pc {
            0x82E51570 => {
    //   block [0x82E51570..0x82E51574)
	// 82E51570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51578 size=12
    let mut pc: u32 = 0x82E51578;
    'dispatch: loop {
        match pc {
            0x82E51578 => {
    //   block [0x82E51578..0x82E51584)
	// 82E51578: 8063FFB0  lwz r3, -0x50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-80 as u32) ) } as u64;
	// 82E5157C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51580: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51584 size=8
    let mut pc: u32 = 0x82E51584;
    'dispatch: loop {
        match pc {
            0x82E51584 => {
    //   block [0x82E51584..0x82E5158C)
	// 82E51584: 4BFE5374  b 0x82e368f8
	sub_82E368F8(ctx, base);
	return;
	// 82E51588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51590 size=12
    let mut pc: u32 = 0x82E51590;
    'dispatch: loop {
        match pc {
            0x82E51590 => {
    //   block [0x82E51590..0x82E5159C)
	// 82E51590: 8063FFAC  lwz r3, -0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-84 as u32) ) } as u64;
	// 82E51594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51598: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5159C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5159C size=20
    let mut pc: u32 = 0x82E5159C;
    'dispatch: loop {
        match pc {
            0x82E5159C => {
    //   block [0x82E5159C..0x82E515B0)
	// 82E5159C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E515A0: 38CB891C  addi r6, r11, -0x76e4
	ctx.r[6].s64 = ctx.r[11].s64 + -30436;
	// 82E515A4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E515A8: 38AB8AC0  addi r5, r11, -0x7540
	ctx.r[5].s64 = ctx.r[11].s64 + -30016;
	// 82E515AC: 4BFE566C  b 0x82e36c18
	sub_82E36C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E515B0 size=4
    let mut pc: u32 = 0x82E515B0;
    'dispatch: loop {
        match pc {
            0x82E515B0 => {
    //   block [0x82E515B0..0x82E515B4)
	// 82E515B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E515B8 size=12
    let mut pc: u32 = 0x82E515B8;
    'dispatch: loop {
        match pc {
            0x82E515B8 => {
    //   block [0x82E515B8..0x82E515C4)
	// 82E515B8: 8063FFAC  lwz r3, -0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-84 as u32) ) } as u64;
	// 82E515BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E515C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E515C4 size=8
    let mut pc: u32 = 0x82E515C4;
    'dispatch: loop {
        match pc {
            0x82E515C4 => {
    //   block [0x82E515C4..0x82E515CC)
	// 82E515C4: 4BFE5334  b 0x82e368f8
	sub_82E368F8(ctx, base);
	return;
	// 82E515C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E515D0 size=128
    let mut pc: u32 = 0x82E515D0;
    'dispatch: loop {
        match pc {
            0x82E515D0 => {
    //   block [0x82E515D0..0x82E51650)
	// 82E515D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E515D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E515D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E515DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E515E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E515E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E515E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E515EC: 817EFFA8  lwz r11, -0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-88 as u32) ) } as u64;
	// 82E515F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E515F4: 419A0044  beq cr6, 0x82e51638
	if ctx.cr[6].eq {
	pc = 0x82E51638; continue 'dispatch;
	}
	// 82E515F8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E515FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E51600: 419A0038  beq cr6, 0x82e51638
	if ctx.cr[6].eq {
	pc = 0x82E51638; continue 'dispatch;
	}
	// 82E51604: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51608: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5160C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51610: 4E800421  bctrl
	ctx.lr = 0x82E51614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51614: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 82E51618: 419A0020  beq cr6, 0x82e51638
	if ctx.cr[6].eq {
	pc = 0x82E51638; continue 'dispatch;
	}
	// 82E5161C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E51620: 807EFFA8  lwz r3, -0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-88 as u32) ) } as u64;
	// 82E51624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E51628: 38CBE738  addi r6, r11, -0x18c8
	ctx.r[6].s64 = ctx.r[11].s64 + -6344;
	// 82E5162C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E51630: 38AB91B0  addi r5, r11, -0x6e50
	ctx.r[5].s64 = ctx.r[11].s64 + -28240;
	// 82E51634: 4BFE55E5  bl 0x82e36c18
	ctx.lr = 0x82E51638;
	sub_82E36C18(ctx, base);
	// 82E51638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5163C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51644: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5164C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51650 size=112
    let mut pc: u32 = 0x82E51650;
    'dispatch: loop {
        match pc {
            0x82E51650 => {
    //   block [0x82E51650..0x82E516C0)
	// 82E51650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5165C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51664: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E51668: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E5166C: 817EFFA8  lwz r11, -0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-88 as u32) ) } as u64;
	// 82E51670: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51674: 419A0034  beq cr6, 0x82e516a8
	if ctx.cr[6].eq {
	pc = 0x82E516A8; continue 'dispatch;
	}
	// 82E51678: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5167C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82E51680: 419A0028  beq cr6, 0x82e516a8
	if ctx.cr[6].eq {
	pc = 0x82E516A8; continue 'dispatch;
	}
	// 82E51684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51688: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5168C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51690: 4E800421  bctrl
	ctx.lr = 0x82E51694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51694: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 82E51698: 419A0010  beq cr6, 0x82e516a8
	if ctx.cr[6].eq {
	pc = 0x82E516A8; continue 'dispatch;
	}
	// 82E5169C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E516A0: 807EFFA8  lwz r3, -0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-88 as u32) ) } as u64;
	// 82E516A4: 4BFE5255  bl 0x82e368f8
	ctx.lr = 0x82E516A8;
	sub_82E368F8(ctx, base);
	// 82E516A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E516AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E516B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E516B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E516B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E516BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E516C0 size=12
    let mut pc: u32 = 0x82E516C0;
    'dispatch: loop {
        match pc {
            0x82E516C0 => {
    //   block [0x82E516C0..0x82E516CC)
	// 82E516C0: 8063FFA4  lwz r3, -0x5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-92 as u32) ) } as u64;
	// 82E516C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E516C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E516CC size=20
    let mut pc: u32 = 0x82E516CC;
    'dispatch: loop {
        match pc {
            0x82E516CC => {
    //   block [0x82E516CC..0x82E516E0)
	// 82E516CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E516D0: 38CBECE0  addi r6, r11, -0x1320
	ctx.r[6].s64 = ctx.r[11].s64 + -4896;
	// 82E516D4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E516D8: 38AB8440  addi r5, r11, -0x7bc0
	ctx.r[5].s64 = ctx.r[11].s64 + -31680;
	// 82E516DC: 4BFE553C  b 0x82e36c18
	sub_82E36C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E516E0 size=4
    let mut pc: u32 = 0x82E516E0;
    'dispatch: loop {
        match pc {
            0x82E516E0 => {
    //   block [0x82E516E0..0x82E516E4)
	// 82E516E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E516E8 size=12
    let mut pc: u32 = 0x82E516E8;
    'dispatch: loop {
        match pc {
            0x82E516E8 => {
    //   block [0x82E516E8..0x82E516F4)
	// 82E516E8: 8063FFA4  lwz r3, -0x5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-92 as u32) ) } as u64;
	// 82E516EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E516F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E516F4 size=8
    let mut pc: u32 = 0x82E516F4;
    'dispatch: loop {
        match pc {
            0x82E516F4 => {
    //   block [0x82E516F4..0x82E516FC)
	// 82E516F4: 4BFE5204  b 0x82e368f8
	sub_82E368F8(ctx, base);
	return;
	// 82E516F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51700 size=488
    let mut pc: u32 = 0x82E51700;
    'dispatch: loop {
        match pc {
            0x82E51700 => {
    //   block [0x82E51700..0x82E518E8)
	// 82E51700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51704: 4BE57CF1  bl 0x82ca93f4
	ctx.lr = 0x82E51708;
	sub_82CA93D0(ctx, base);
	// 82E51708: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5170C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51710: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E51714: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5171C: 419A01C4  beq cr6, 0x82e518e0
	if ctx.cr[6].eq {
	pc = 0x82E518E0; continue 'dispatch;
	}
	// 82E51720: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E51724: 419A01BC  beq cr6, 0x82e518e0
	if ctx.cr[6].eq {
	pc = 0x82E518E0; continue 'dispatch;
	}
	// 82E51728: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E5172C: 4BF27AE5  bl 0x82d79210
	ctx.lr = 0x82E51730;
	sub_82D79210(ctx, base);
	// 82E51730: 3BDF005C  addi r30, r31, 0x5c
	ctx.r[30].s64 = ctx.r[31].s64 + 92;
	// 82E51734: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5173C: 4BF27F1D  bl 0x82d79658
	ctx.lr = 0x82E51740;
	sub_82D79658(ctx, base);
	// 82E51740: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 82E51744: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51748: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5174C: 4BF27F7D  bl 0x82d796c8
	ctx.lr = 0x82E51750;
	sub_82D796C8(ctx, base);
	// 82E51750: 3B3F0068  addi r25, r31, 0x68
	ctx.r[25].s64 = ctx.r[31].s64 + 104;
	// 82E51754: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51758: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E5175C: 4BF27E1D  bl 0x82d79578
	ctx.lr = 0x82E51760;
	sub_82D79578(ctx, base);
	// 82E51760: 3AFF0064  addi r23, r31, 0x64
	ctx.r[23].s64 = ctx.r[31].s64 + 100;
	// 82E51764: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51768: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82E5176C: 4BF27E7D  bl 0x82d795e8
	ctx.lr = 0x82E51770;
	sub_82D795E8(ctx, base);
	// 82E51770: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51774: 4BF2774D  bl 0x82d78ec0
	ctx.lr = 0x82E51778;
	sub_82D78EC0(ctx, base);
	// 82E51778: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E5177C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E51780: 3BF80008  addi r31, r24, 8
	ctx.r[31].s64 = ctx.r[24].s64 + 8;
	// 82E51784: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51788: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5178C: 40990038  ble cr6, 0x82e517c4
	if !ctx.cr[6].gt {
	pc = 0x82E517C4; continue 'dispatch;
	}
	// 82E51790: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E51794: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5179C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E517A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E517A4: 7C8AE02E  lwzx r4, r10, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E517A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E517AC: 4E800421  bctrl
	ctx.lr = 0x82E517B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E517B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E517B4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E517B8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E517BC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E517C0: 4198FFD4  blt cr6, 0x82e51794
	if ctx.cr[6].lt {
	pc = 0x82E51794; continue 'dispatch;
	}
	// 82E517C4: 3BB8002C  addi r29, r24, 0x2c
	ctx.r[29].s64 = ctx.r[24].s64 + 44;
	// 82E517C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E517CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E517D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E517D4: 40990038  ble cr6, 0x82e5180c
	if !ctx.cr[6].gt {
	pc = 0x82E5180C; continue 'dispatch;
	}
	// 82E517D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E517DC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E517E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E517E4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E517E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E517EC: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E517F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E517F4: 4E800421  bctrl
	ctx.lr = 0x82E517F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E517F8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E517FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51800: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51804: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51808: 4198FFD4  blt cr6, 0x82e517dc
	if ctx.cr[6].lt {
	pc = 0x82E517DC; continue 'dispatch;
	}
	// 82E5180C: 3BB80020  addi r29, r24, 0x20
	ctx.r[29].s64 = ctx.r[24].s64 + 32;
	// 82E51810: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E51814: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51818: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5181C: 40990038  ble cr6, 0x82e51854
	if !ctx.cr[6].gt {
	pc = 0x82E51854; continue 'dispatch;
	}
	// 82E51820: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51824: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51828: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E5182C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51830: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51834: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E51838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5183C: 4E800421  bctrl
	ctx.lr = 0x82E51840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51840: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51844: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51848: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5184C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51850: 4198FFD4  blt cr6, 0x82e51824
	if ctx.cr[6].lt {
	pc = 0x82E51824; continue 'dispatch;
	}
	// 82E51854: 3BB80014  addi r29, r24, 0x14
	ctx.r[29].s64 = ctx.r[24].s64 + 20;
	// 82E51858: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5185C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51860: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51864: 40990038  ble cr6, 0x82e5189c
	if !ctx.cr[6].gt {
	pc = 0x82E5189C; continue 'dispatch;
	}
	// 82E51868: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E5186C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51870: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E51874: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51878: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5187C: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E51880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51884: 4E800421  bctrl
	ctx.lr = 0x82E51888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51888: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5188C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51890: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51894: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51898: 4198FFD4  blt cr6, 0x82e5186c
	if ctx.cr[6].lt {
	pc = 0x82E5186C; continue 'dispatch;
	}
	// 82E5189C: A1780004  lhz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E518A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E518A4: 419A0034  beq cr6, 0x82e518d8
	if ctx.cr[6].eq {
	pc = 0x82E518D8; continue 'dispatch;
	}
	// 82E518A8: A1780006  lhz r11, 6(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E518AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E518B0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E518B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E518B8: B1780006  sth r11, 6(r24)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[24].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E518BC: 409A001C  bne cr6, 0x82e518d8
	if !ctx.cr[6].eq {
	pc = 0x82E518D8; continue 'dispatch;
	}
	// 82E518C0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E518C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E518C8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E518CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E518D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E518D4: 4E800421  bctrl
	ctx.lr = 0x82E518D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E518D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E518DC: 4BF2793D  bl 0x82d79218
	ctx.lr = 0x82E518E0;
	sub_82D79218(ctx, base);
	// 82E518E0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E518E4: 4BE57B60  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E518E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E518E8 size=472
    let mut pc: u32 = 0x82E518E8;
    'dispatch: loop {
        match pc {
            0x82E518E8 => {
    //   block [0x82E518E8..0x82E51AC0)
	// 82E518E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E518EC: 4BE57B0D  bl 0x82ca93f8
	ctx.lr = 0x82E518F0;
	sub_82CA93D0(ctx, base);
	// 82E518F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E518F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E518F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E518FC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51904: 419A01B4  beq cr6, 0x82e51ab8
	if ctx.cr[6].eq {
	pc = 0x82E51AB8; continue 'dispatch;
	}
	// 82E51908: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E5190C: 419A01AC  beq cr6, 0x82e51ab8
	if ctx.cr[6].eq {
	pc = 0x82E51AB8; continue 'dispatch;
	}
	// 82E51910: 3BBE005C  addi r29, r30, 0x5c
	ctx.r[29].s64 = ctx.r[30].s64 + 92;
	// 82E51914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51918: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5191C: 4BF2683D  bl 0x82d78158
	ctx.lr = 0x82E51920;
	sub_82D78158(ctx, base);
	// 82E51920: 3B7E0060  addi r27, r30, 0x60
	ctx.r[27].s64 = ctx.r[30].s64 + 96;
	// 82E51924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51928: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5192C: 4BF26875  bl 0x82d781a0
	ctx.lr = 0x82E51930;
	sub_82D781A0(ctx, base);
	// 82E51930: 3B5E0068  addi r26, r30, 0x68
	ctx.r[26].s64 = ctx.r[30].s64 + 104;
	// 82E51934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51938: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E5193C: 4BF2678D  bl 0x82d780c8
	ctx.lr = 0x82E51940;
	sub_82D780C8(ctx, base);
	// 82E51940: 3B1E0064  addi r24, r30, 0x64
	ctx.r[24].s64 = ctx.r[30].s64 + 100;
	// 82E51944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51948: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E5194C: 4BF267C5  bl 0x82d78110
	ctx.lr = 0x82E51950;
	sub_82D78110(ctx, base);
	// 82E51950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51954: 4BF2756D  bl 0x82d78ec0
	ctx.lr = 0x82E51958;
	sub_82D78EC0(ctx, base);
	// 82E51958: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E5195C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51960: 3BF90008  addi r31, r25, 8
	ctx.r[31].s64 = ctx.r[25].s64 + 8;
	// 82E51964: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51968: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5196C: 40990038  ble cr6, 0x82e519a4
	if !ctx.cr[6].gt {
	pc = 0x82E519A4; continue 'dispatch;
	}
	// 82E51970: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E51974: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51978: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5197C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51980: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51984: 7C8AE02E  lwzx r4, r10, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E51988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5198C: 4E800421  bctrl
	ctx.lr = 0x82E51990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51990: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51994: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E51998: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E5199C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E519A0: 4198FFD4  blt cr6, 0x82e51974
	if ctx.cr[6].lt {
	pc = 0x82E51974; continue 'dispatch;
	}
	// 82E519A4: 3BB9002C  addi r29, r25, 0x2c
	ctx.r[29].s64 = ctx.r[25].s64 + 44;
	// 82E519A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E519AC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E519B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E519B4: 40990038  ble cr6, 0x82e519ec
	if !ctx.cr[6].gt {
	pc = 0x82E519EC; continue 'dispatch;
	}
	// 82E519B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E519BC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E519C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E519C4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E519C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E519CC: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E519D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E519D4: 4E800421  bctrl
	ctx.lr = 0x82E519D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E519D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E519DC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E519E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E519E4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E519E8: 4198FFD4  blt cr6, 0x82e519bc
	if ctx.cr[6].lt {
	pc = 0x82E519BC; continue 'dispatch;
	}
	// 82E519EC: 3BB90020  addi r29, r25, 0x20
	ctx.r[29].s64 = ctx.r[25].s64 + 32;
	// 82E519F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E519F4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E519F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E519FC: 40990038  ble cr6, 0x82e51a34
	if !ctx.cr[6].gt {
	pc = 0x82E51A34; continue 'dispatch;
	}
	// 82E51A00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51A04: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51A08: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E51A0C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51A10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51A14: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E51A18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51A1C: 4E800421  bctrl
	ctx.lr = 0x82E51A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51A20: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51A24: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51A28: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51A2C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51A30: 4198FFD4  blt cr6, 0x82e51a04
	if ctx.cr[6].lt {
	pc = 0x82E51A04; continue 'dispatch;
	}
	// 82E51A34: 3BB90014  addi r29, r25, 0x14
	ctx.r[29].s64 = ctx.r[25].s64 + 20;
	// 82E51A38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E51A3C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51A40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51A44: 40990038  ble cr6, 0x82e51a7c
	if !ctx.cr[6].gt {
	pc = 0x82E51A7C; continue 'dispatch;
	}
	// 82E51A48: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51A4C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51A50: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E51A54: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51A58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51A5C: 7C9E502E  lwzx r4, r30, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E51A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51A64: 4E800421  bctrl
	ctx.lr = 0x82E51A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51A68: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51A6C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51A70: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51A74: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51A78: 4198FFD4  blt cr6, 0x82e51a4c
	if ctx.cr[6].lt {
	pc = 0x82E51A4C; continue 'dispatch;
	}
	// 82E51A7C: A1790004  lhz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51A84: 419A0034  beq cr6, 0x82e51ab8
	if ctx.cr[6].eq {
	pc = 0x82E51AB8; continue 'dispatch;
	}
	// 82E51A88: A1790006  lhz r11, 6(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E51A8C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E51A90: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E51A94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51A98: B1790006  sth r11, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E51A9C: 409A001C  bne cr6, 0x82e51ab8
	if !ctx.cr[6].eq {
	pc = 0x82E51AB8; continue 'dispatch;
	}
	// 82E51AA0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51AA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E51AA8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E51AAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51AB4: 4E800421  bctrl
	ctx.lr = 0x82E51AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51AB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E51ABC: 4BE5798C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51AC0 size=56
    let mut pc: u32 = 0x82E51AC0;
    'dispatch: loop {
        match pc {
            0x82E51AC0 => {
    //   block [0x82E51AC0..0x82E51AF8)
	// 82E51AC0: 81230070  lwz r9, 0x70(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E51AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E51AC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E51ACC: 40990024  ble cr6, 0x82e51af0
	if !ctx.cr[6].gt {
	pc = 0x82E51AF0; continue 'dispatch;
	}
	// 82E51AD0: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E51AD4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51AD8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E51ADC: 419A001C  beq cr6, 0x82e51af8
	if ctx.cr[6].eq {
		sub_82E51AF8(ctx, base);
		return;
	}
	// 82E51AE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E51AE4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E51AE8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E51AEC: 4198FFE8  blt cr6, 0x82e51ad4
	if ctx.cr[6].lt {
	pc = 0x82E51AD4; continue 'dispatch;
	}
	// 82E51AF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E51AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51AF8 size=8
    let mut pc: u32 = 0x82E51AF8;
    'dispatch: loop {
        match pc {
            0x82E51AF8 => {
    //   block [0x82E51AF8..0x82E51B00)
	// 82E51AF8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E51AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51B00 size=16
    let mut pc: u32 = 0x82E51B00;
    'dispatch: loop {
        match pc {
            0x82E51B00 => {
    //   block [0x82E51B00..0x82E51B10)
	// 82E51B00: 8123007C  lwz r9, 0x7c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51B08: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E51B0C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51B10 size=36
    let mut pc: u32 = 0x82E51B10;
    'dispatch: loop {
        match pc {
            0x82E51B10 => {
    //   block [0x82E51B10..0x82E51B34)
	// 82E51B10: 81430078  lwz r10, 0x78(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E51B14: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51B18: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E51B1C: 419A0018  beq cr6, 0x82e51b34
	if ctx.cr[6].eq {
		sub_82E51B34(ctx, base);
		return;
	}
	// 82E51B20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E51B24: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E51B28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E51B2C: 4198FFE8  blt cr6, 0x82e51b14
	if ctx.cr[6].lt {
	pc = 0x82E51B14; continue 'dispatch;
	}
	// 82E51B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51B34 size=8
    let mut pc: u32 = 0x82E51B34;
    'dispatch: loop {
        match pc {
            0x82E51B34 => {
    //   block [0x82E51B34..0x82E51B3C)
	// 82E51B34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51B38: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51B3C size=36
    let mut pc: u32 = 0x82E51B3C;
    'dispatch: loop {
        match pc {
            0x82E51B3C => {
    //   block [0x82E51B3C..0x82E51B60)
	// 82E51B3C: 8143007C  lwz r10, 0x7c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51B40: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E51B44: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E51B48: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E51B4C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E51B50: 9143007C  stw r10, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82E51B54: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E51B58: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E51B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51B60 size=172
    let mut pc: u32 = 0x82E51B60;
    'dispatch: loop {
        match pc {
            0x82E51B60 => {
    //   block [0x82E51B60..0x82E51C0C)
	// 82E51B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51B64: 4BE578A1  bl 0x82ca9404
	ctx.lr = 0x82E51B68;
	sub_82CA93D0(ctx, base);
	// 82E51B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51B70: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E51B74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51B7C: 419A0040  beq cr6, 0x82e51bbc
	if ctx.cr[6].eq {
	pc = 0x82E51BBC; continue 'dispatch;
	}
	// 82E51B80: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E51B84: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E51B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51B8C: 40990030  ble cr6, 0x82e51bbc
	if !ctx.cr[6].gt {
	pc = 0x82E51BBC; continue 'dispatch;
	}
	// 82E51B90: 3B9FFFF8  addi r28, r31, -8
	ctx.r[28].s64 = ctx.r[31].s64 + -8;
	// 82E51B94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51B98: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E51B9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E51BA0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E51BA4: 4BFFFD45  bl 0x82e518e8
	ctx.lr = 0x82E51BA8;
	sub_82E518E8(ctx, base);
	// 82E51BA8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E51BAC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E51BB0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51BB4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51BB8: 4198FFE0  blt cr6, 0x82e51b98
	if ctx.cr[6].lt {
	pc = 0x82E51B98; continue 'dispatch;
	}
	// 82E51BBC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E51BC0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E51BC4: 419A0040  beq cr6, 0x82e51c04
	if ctx.cr[6].eq {
	pc = 0x82E51C04; continue 'dispatch;
	}
	// 82E51BC8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E51BCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E51BD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51BD4: 40990030  ble cr6, 0x82e51c04
	if !ctx.cr[6].gt {
	pc = 0x82E51C04; continue 'dispatch;
	}
	// 82E51BD8: 3B9FFFF8  addi r28, r31, -8
	ctx.r[28].s64 = ctx.r[31].s64 + -8;
	// 82E51BDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51BE0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E51BE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E51BE8: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E51BEC: 4BFFFB15  bl 0x82e51700
	ctx.lr = 0x82E51BF0;
	sub_82E51700(ctx, base);
	// 82E51BF0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E51BF4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E51BF8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51BFC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51C00: 4198FFE0  blt cr6, 0x82e51be0
	if ctx.cr[6].lt {
	pc = 0x82E51BE0; continue 'dispatch;
	}
	// 82E51C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E51C08: 4BE5784C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51C10 size=228
    let mut pc: u32 = 0x82E51C10;
    'dispatch: loop {
        match pc {
            0x82E51C10 => {
    //   block [0x82E51C10..0x82E51CF4)
	// 82E51C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51C14: 4BE577F1  bl 0x82ca9404
	ctx.lr = 0x82E51C18;
	sub_82CA93D0(ctx, base);
	// 82E51C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51C20: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E51C24: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E51C28: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E51C2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E51C30: 409900BC  ble cr6, 0x82e51cec
	if !ctx.cr[6].gt {
	pc = 0x82E51CEC; continue 'dispatch;
	}
	// 82E51C34: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E51C38: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51C3C: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E51C40: 419A001C  beq cr6, 0x82e51c5c
	if ctx.cr[6].eq {
	pc = 0x82E51C5C; continue 'dispatch;
	}
	// 82E51C44: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82E51C48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E51C4C: 7F1B5000  cmpw cr6, r27, r10
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E51C50: 4198FFE8  blt cr6, 0x82e51c38
	if ctx.cr[6].lt {
	pc = 0x82E51C38; continue 'dispatch;
	}
	// 82E51C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E51C58: 4BE577FC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E51C5C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E51C60: 4198008C  blt cr6, 0x82e51cec
	if ctx.cr[6].lt {
	pc = 0x82E51CEC; continue 'dispatch;
	}
	// 82E51C64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E51C68: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 82E51C6C: 409A0008  bne cr6, 0x82e51c74
	if !ctx.cr[6].eq {
	pc = 0x82E51C74; continue 'dispatch;
	}
	// 82E51C70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E51C74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E51C78: 4BF26769  bl 0x82d783e0
	ctx.lr = 0x82E51C7C;
	sub_82D783E0(ctx, base);
	// 82E51C7C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51C80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51C84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51C88: 40990038  ble cr6, 0x82e51cc0
	if !ctx.cr[6].gt {
	pc = 0x82E51CC0; continue 'dispatch;
	}
	// 82E51C8C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E51C90: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E51C94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E51C98: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E51C9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51CA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51CA8: 4E800421  bctrl
	ctx.lr = 0x82E51CAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51CAC: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51CB0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E51CB4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E51CB8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51CBC: 4198FFD4  blt cr6, 0x82e51c90
	if ctx.cr[6].lt {
	pc = 0x82E51C90; continue 'dispatch;
	}
	// 82E51CC0: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E51CC4: 5769103A  slwi r9, r27, 2
	ctx.r[9].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E51CC8: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E51CCC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E51CD0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E51CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51CD8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E51CDC: 915F0070  stw r10, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E51CE0: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E51CE4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E51CE8: 4BFFFC01  bl 0x82e518e8
	ctx.lr = 0x82E51CEC;
	sub_82E518E8(ctx, base);
	// 82E51CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E51CF0: 4BE57764  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51CF8 size=256
    let mut pc: u32 = 0x82E51CF8;
    'dispatch: loop {
        match pc {
            0x82E51CF8 => {
    //   block [0x82E51CF8..0x82E51DF8)
	// 82E51CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51CFC: 4BE5770D  bl 0x82ca9408
	ctx.lr = 0x82E51D00;
	sub_82CA93D0(ctx, base);
	// 82E51D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51D04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E51D08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E51D0C: 3BDD006C  addi r30, r29, 0x6c
	ctx.r[30].s64 = ctx.r[29].s64 + 108;
	// 82E51D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51D14: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51D18: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E51D1C: 40990030  ble cr6, 0x82e51d4c
	if !ctx.cr[6].gt {
	pc = 0x82E51D4C; continue 'dispatch;
	}
	// 82E51D20: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51D24: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51D28: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E51D2C: 419A0018  beq cr6, 0x82e51d44
	if ctx.cr[6].eq {
	pc = 0x82E51D44; continue 'dispatch;
	}
	// 82E51D30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E51D34: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E51D38: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E51D3C: 4198FFE8  blt cr6, 0x82e51d24
	if ctx.cr[6].lt {
	pc = 0x82E51D24; continue 'dispatch;
	}
	// 82E51D40: 4800000C  b 0x82e51d4c
	pc = 0x82E51D4C; continue 'dispatch;
	// 82E51D44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51D48: 409800A8  bge cr6, 0x82e51df0
	if !ctx.cr[6].lt {
	pc = 0x82E51DF0; continue 'dispatch;
	}
	// 82E51D4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E51D50: 389D0058  addi r4, r29, 0x58
	ctx.r[4].s64 = ctx.r[29].s64 + 88;
	// 82E51D54: 409A0008  bne cr6, 0x82e51d5c
	if !ctx.cr[6].eq {
	pc = 0x82E51D5C; continue 'dispatch;
	}
	// 82E51D58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E51D5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E51D60: 4BF27CE9  bl 0x82d79a48
	ctx.lr = 0x82E51D64;
	sub_82D79A48(ctx, base);
	// 82E51D64: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51D68: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51D6C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E51D70: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51D74: 409A0010  bne cr6, 0x82e51d84
	if !ctx.cr[6].eq {
	pc = 0x82E51D84; continue 'dispatch;
	}
	// 82E51D78: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E51D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E51D80: 4BF05219  bl 0x82d56f98
	ctx.lr = 0x82E51D84;
	sub_82D56F98(ctx, base);
	// 82E51D84: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51D88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E51D8C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51D90: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E51D94: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82E51D98: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51D9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E51DA0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51DA4: 817D007C  lwz r11, 0x7c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51DA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51DAC: 40990038  ble cr6, 0x82e51de4
	if !ctx.cr[6].gt {
	pc = 0x82E51DE4; continue 'dispatch;
	}
	// 82E51DB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E51DB4: 817D0078  lwz r11, 0x78(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E51DB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E51DBC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E51DC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51DC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51DC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51DCC: 4E800421  bctrl
	ctx.lr = 0x82E51DD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51DD0: 817D007C  lwz r11, 0x7c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E51DD4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E51DD8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E51DDC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51DE0: 4198FFD4  blt cr6, 0x82e51db4
	if ctx.cr[6].lt {
	pc = 0x82E51DB4; continue 'dispatch;
	}
	// 82E51DE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E51DE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E51DEC: 4BFFF915  bl 0x82e51700
	ctx.lr = 0x82E51DF0;
	sub_82E51700(ctx, base);
	// 82E51DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E51DF4: 4BE57664  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51DF8 size=8
    let mut pc: u32 = 0x82E51DF8;
    'dispatch: loop {
        match pc {
            0x82E51DF8 => {
    //   block [0x82E51DF8..0x82E51E00)
	// 82E51DF8: 3863FFA8  addi r3, r3, -0x58
	ctx.r[3].s64 = ctx.r[3].s64 + -88;
	// 82E51DFC: 4BFFFE14  b 0x82e51c10
	sub_82E51C10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51E00 size=172
    let mut pc: u32 = 0x82E51E00;
    'dispatch: loop {
        match pc {
            0x82E51E00 => {
    //   block [0x82E51E00..0x82E51EAC)
	// 82E51E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51E08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E51E0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51E14: 3BE30078  addi r31, r3, 0x78
	ctx.r[31].s64 = ctx.r[3].s64 + 120;
	// 82E51E18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E51E1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51E20: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51E24: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E51E28: 40990030  ble cr6, 0x82e51e58
	if !ctx.cr[6].gt {
	pc = 0x82E51E58; continue 'dispatch;
	}
	// 82E51E2C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51E30: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51E34: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E51E38: 419A0018  beq cr6, 0x82e51e50
	if ctx.cr[6].eq {
	pc = 0x82E51E50; continue 'dispatch;
	}
	// 82E51E3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E51E40: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E51E44: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E51E48: 4198FFE8  blt cr6, 0x82e51e30
	if ctx.cr[6].lt {
	pc = 0x82E51E30; continue 'dispatch;
	}
	// 82E51E4C: 4800000C  b 0x82e51e58
	pc = 0x82E51E58; continue 'dispatch;
	// 82E51E50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51E54: 40980040  bge cr6, 0x82e51e94
	if !ctx.cr[6].lt {
	pc = 0x82E51E94; continue 'dispatch;
	}
	// 82E51E58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51E5C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51E60: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E51E64: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E51E68: 409A0010  bne cr6, 0x82e51e78
	if !ctx.cr[6].eq {
	pc = 0x82E51E78; continue 'dispatch;
	}
	// 82E51E6C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E51E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51E74: 4BF05125  bl 0x82d56f98
	ctx.lr = 0x82E51E78;
	sub_82D56F98(ctx, base);
	// 82E51E78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51E7C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51E80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E51E84: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E51E88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51E8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E51E90: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51EA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51EB0 size=280
    let mut pc: u32 = 0x82E51EB0;
    'dispatch: loop {
        match pc {
            0x82E51EB0 => {
    //   block [0x82E51EB0..0x82E51FC8)
	// 82E51EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51EB4: 4BE57531  bl 0x82ca93e4
	ctx.lr = 0x82E51EB8;
	sub_82CA93D0(ctx, base);
	// 82E51EB8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51EBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E51EC0: 38E9E7E0  addi r7, r9, -0x1820
	ctx.r[7].s64 = ctx.r[9].s64 + -6176;
	// 82E51EC4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E51EC8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E51ECC: 61290006  ori r9, r9, 6
	ctx.r[9].u64 = ctx.r[9].u64 | 6;
	// 82E51ED0: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 82E51ED4: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82E51ED8: 7D364B78  mr r22, r9
	ctx.r[22].u64 = ctx.r[9].u64;
	// 82E51EDC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51EE0: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82E51EE4: 390B7238  addi r8, r11, 0x7238
	ctx.r[8].s64 = ctx.r[11].s64 + 29240;
	// 82E51EE8: 38A95D58  addi r5, r9, 0x5d58
	ctx.r[5].s64 = ctx.r[9].s64 + 23896;
	// 82E51EEC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51EF4: 38897044  addi r4, r9, 0x7044
	ctx.r[4].s64 = ctx.r[9].s64 + 28740;
	// 82E51EF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51EFC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E51F00: 390A002C  addi r8, r10, 0x2c
	ctx.r[8].s64 = ctx.r[10].s64 + 44;
	// 82E51F04: 3BE97228  addi r31, r9, 0x7228
	ctx.r[31].s64 = ctx.r[9].s64 + 29224;
	// 82E51F08: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F0C: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51F10: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E51F14: 3BC972B4  addi r30, r9, 0x72b4
	ctx.r[30].s64 = ctx.r[9].s64 + 29364;
	// 82E51F18: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F1C: 3A68000C  addi r19, r8, 0xc
	ctx.r[19].s64 = ctx.r[8].s64 + 12;
	// 82E51F20: 3BA972A4  addi r29, r9, 0x72a4
	ctx.r[29].s64 = ctx.r[9].s64 + 29348;
	// 82E51F24: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F28: 38C65D40  addi r6, r6, 0x5d40
	ctx.r[6].s64 = ctx.r[6].s64 + 23872;
	// 82E51F2C: 3B897298  addi r28, r9, 0x7298
	ctx.r[28].s64 = ctx.r[9].s64 + 29336;
	// 82E51F30: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F34: 3B697280  addi r27, r9, 0x7280
	ctx.r[27].s64 = ctx.r[9].s64 + 29312;
	// 82E51F38: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F3C: 3B497268  addi r26, r9, 0x7268
	ctx.r[26].s64 = ctx.r[9].s64 + 29288;
	// 82E51F40: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F44: 3B297258  addi r25, r9, 0x7258
	ctx.r[25].s64 = ctx.r[9].s64 + 29272;
	// 82E51F48: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51F4C: 3B097248  addi r24, r9, 0x7248
	ctx.r[24].s64 = ctx.r[9].s64 + 29256;
	// 82E51F50: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E51F54: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 82E51F58: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82E51F5C: 392A0008  addi r9, r10, 8
	ctx.r[9].s64 = ctx.r[10].s64 + 8;
	// 82E51F60: 3949000C  addi r10, r9, 0xc
	ctx.r[10].s64 = ctx.r[9].s64 + 12;
	// 82E51F64: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51F68: 92E90008  stw r23, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82E51F6C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E51F70: 92680000  stw r19, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 82E51F74: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51F78: 92C80008  stw r22, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82E51F7C: 90E30058  stw r7, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82E51F80: 90C3005C  stw r6, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82E51F84: 90A30060  stw r5, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E51F88: 90830064  stw r4, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 82E51F8C: 93E30068  stw r31, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E51F90: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E51F94: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E51F98: 93830058  stw r28, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82E51F9C: 9363005C  stw r27, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82E51FA0: 93430060  stw r26, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 82E51FA4: 93230064  stw r25, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82E51FA8: 93030068  stw r24, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[24].u32 ) };
	// 82E51FAC: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E51FB0: 91630070  stw r11, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E51FB4: 92A30074  stw r21, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[21].u32 ) };
	// 82E51FB8: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E51FBC: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E51FC0: 92830080  stw r20, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[20].u32 ) };
	// 82E51FC4: 4BE57470  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51FC8 size=424
    let mut pc: u32 = 0x82E51FC8;
    'dispatch: loop {
        match pc {
            0x82E51FC8 => {
    //   block [0x82E51FC8..0x82E52170)
	// 82E51FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51FCC: 4BE5743D  bl 0x82ca9408
	ctx.lr = 0x82E51FD0;
	sub_82CA93D0(ctx, base);
	// 82E51FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51FD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51FD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E51FDC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E51FE0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E51FE4: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E51FE8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E51FEC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E51FF0: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E51FF4: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82E51FF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E51FFC: 394A72B4  addi r10, r10, 0x72b4
	ctx.r[10].s64 = ctx.r[10].s64 + 29364;
	// 82E52000: 392972A4  addi r9, r9, 0x72a4
	ctx.r[9].s64 = ctx.r[9].s64 + 29348;
	// 82E52004: 396B7298  addi r11, r11, 0x7298
	ctx.r[11].s64 = ctx.r[11].s64 + 29336;
	// 82E52008: 39087280  addi r8, r8, 0x7280
	ctx.r[8].s64 = ctx.r[8].s64 + 29312;
	// 82E5200C: 38E77268  addi r7, r7, 0x7268
	ctx.r[7].s64 = ctx.r[7].s64 + 29288;
	// 82E52010: 38C67258  addi r6, r6, 0x7258
	ctx.r[6].s64 = ctx.r[6].s64 + 29272;
	// 82E52014: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E52018: 38A57248  addi r5, r5, 0x7248
	ctx.r[5].s64 = ctx.r[5].s64 + 29256;
	// 82E5201C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E52020: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 82E52024: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E52028: 911F005C  stw r8, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82E5202C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E52030: 90FF0060  stw r7, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82E52034: 90DF0064  stw r6, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82E52038: 90BF0068  stw r5, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[5].u32 ) };
	// 82E5203C: 41980028  blt cr6, 0x82e52064
	if ctx.cr[6].lt {
	pc = 0x82E52064; continue 'dispatch;
	}
	// 82E52040: 57DC103A  slwi r28, r30, 2
	ctx.r[28].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82E52044: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E52048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5204C: 7C8BE02E  lwzx r4, r11, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E52050: 4BFFFBC1  bl 0x82e51c10
	ctx.lr = 0x82E52054;
	sub_82E51C10(ctx, base);
	// 82E52054: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E52058: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 82E5205C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E52060: 4098FFE4  bge cr6, 0x82e52044
	if !ctx.cr[6].lt {
	pc = 0x82E52044; continue 'dispatch;
	}
	// 82E52064: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E52068: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5206C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E52070: 409A0020  bne cr6, 0x82e52090
	if !ctx.cr[6].eq {
	pc = 0x82E52090; continue 'dispatch;
	}
	// 82E52074: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52078: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5207C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E52080: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E52084: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E52088: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5208C: 4BF0323D  bl 0x82d552c8
	ctx.lr = 0x82E52090;
	sub_82D552C8(ctx, base);
	// 82E52090: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E52094: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E52098: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5209C: 409A0020  bne cr6, 0x82e520bc
	if !ctx.cr[6].eq {
	pc = 0x82E520BC; continue 'dispatch;
	}
	// 82E520A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E520A4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E520A8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E520AC: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E520B0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E520B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E520B8: 4BF03211  bl 0x82d552c8
	ctx.lr = 0x82E520BC;
	sub_82D552C8(ctx, base);
	// 82E520BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E520C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E520C4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E520C8: 396B7228  addi r11, r11, 0x7228
	ctx.r[11].s64 = ctx.r[11].s64 + 29224;
	// 82E520CC: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E520D0: 394A7044  addi r10, r10, 0x7044
	ctx.r[10].s64 = ctx.r[10].s64 + 28740;
	// 82E520D4: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E520D8: 39295D58  addi r9, r9, 0x5d58
	ctx.r[9].s64 = ctx.r[9].s64 + 23896;
	// 82E520DC: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E520E0: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E520E4: 39085D40  addi r8, r8, 0x5d40
	ctx.r[8].s64 = ctx.r[8].s64 + 23872;
	// 82E520E8: 38E7E7E0  addi r7, r7, -0x1820
	ctx.r[7].s64 = ctx.r[7].s64 + -6176;
	// 82E520EC: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82E520F0: 38C67238  addi r6, r6, 0x7238
	ctx.r[6].s64 = ctx.r[6].s64 + 29240;
	// 82E520F4: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E520F8: 911F005C  stw r8, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82E520FC: 90FF0058  stw r7, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82E52100: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E52104: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E52108: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5210C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E52110: 409A0020  bne cr6, 0x82e52130
	if !ctx.cr[6].eq {
	pc = 0x82E52130; continue 'dispatch;
	}
	// 82E52114: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5211C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E52120: 809D002C  lwz r4, 0x2c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E52124: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E52128: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5212C: 4BF0319D  bl 0x82d552c8
	ctx.lr = 0x82E52130;
	sub_82D552C8(ctx, base);
	// 82E52130: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E52134: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E52138: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5213C: 409A0020  bne cr6, 0x82e5215c
	if !ctx.cr[6].eq {
	pc = 0x82E5215C; continue 'dispatch;
	}
	// 82E52140: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52144: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E52148: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E5214C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52150: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E52154: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E52158: 4BF03171  bl 0x82d552c8
	ctx.lr = 0x82E5215C;
	sub_82D552C8(ctx, base);
	// 82E5215C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E52160: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E52164: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5216C: 4BE572EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52170 size=140
    let mut pc: u32 = 0x82E52170;
    'dispatch: loop {
        match pc {
            0x82E52170 => {
    //   block [0x82E52170..0x82E521FC)
	// 82E52170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5217C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52180: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52188: 396B7238  addi r11, r11, 0x7238
	ctx.r[11].s64 = ctx.r[11].s64 + 29240;
	// 82E5218C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52190: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E52194: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E52198: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5219C: 409A0020  bne cr6, 0x82e521bc
	if !ctx.cr[6].eq {
	pc = 0x82E521BC; continue 'dispatch;
	}
	// 82E521A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E521A4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E521A8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E521AC: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E521B0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E521B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E521B8: 4BF03111  bl 0x82d552c8
	ctx.lr = 0x82E521BC;
	sub_82D552C8(ctx, base);
	// 82E521BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E521C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E521C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E521C8: 409A0020  bne cr6, 0x82e521e8
	if !ctx.cr[6].eq {
	pc = 0x82E521E8; continue 'dispatch;
	}
	// 82E521CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E521D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E521D4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E521D8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E521DC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E521E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E521E4: 4BF030E5  bl 0x82d552c8
	ctx.lr = 0x82E521E8;
	sub_82D552C8(ctx, base);
	// 82E521E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E521EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E521F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E521F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E521F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52200 size=80
    let mut pc: u32 = 0x82E52200;
    'dispatch: loop {
        match pc {
            0x82E52200 => {
    //   block [0x82E52200..0x82E52250)
	// 82E52200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5220C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5221C: 4BFFFF55  bl 0x82e52170
	ctx.lr = 0x82E52220;
	sub_82E52170(ctx, base);
	// 82E52220: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E52224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5222C: 419A000C  beq cr6, 0x82e52238
	if ctx.cr[6].eq {
	pc = 0x82E52238; continue 'dispatch;
	}
	// 82E52230: 4B9F3581  bl 0x828457b0
	ctx.lr = 0x82E52234;
	sub_828457B0(ctx, base);
	// 82E52234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5223C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52244: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E52248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5224C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52250 size=12
    let mut pc: u32 = 0x82E52250;
    'dispatch: loop {
        match pc {
            0x82E52250 => {
    //   block [0x82E52250..0x82E5225C)
	// 82E52250: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82E52254: 386B6D8C  addi r3, r11, 0x6d8c
	ctx.r[3].s64 = ctx.r[11].s64 + 28044;
	// 82E52258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52260 size=8
    let mut pc: u32 = 0x82E52260;
    'dispatch: loop {
        match pc {
            0x82E52260 => {
    //   block [0x82E52260..0x82E52268)
	// 82E52260: 3863FF98  addi r3, r3, -0x68
	ctx.r[3].s64 = ctx.r[3].s64 + -104;
	// 82E52264: 4800002C  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52268 size=8
    let mut pc: u32 = 0x82E52268;
    'dispatch: loop {
        match pc {
            0x82E52268 => {
    //   block [0x82E52268..0x82E52270)
	// 82E52268: 3863FFA8  addi r3, r3, -0x58
	ctx.r[3].s64 = ctx.r[3].s64 + -88;
	// 82E5226C: 48000024  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52270 size=8
    let mut pc: u32 = 0x82E52270;
    'dispatch: loop {
        match pc {
            0x82E52270 => {
    //   block [0x82E52270..0x82E52278)
	// 82E52270: 3863FFA4  addi r3, r3, -0x5c
	ctx.r[3].s64 = ctx.r[3].s64 + -92;
	// 82E52274: 4800001C  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52278 size=8
    let mut pc: u32 = 0x82E52278;
    'dispatch: loop {
        match pc {
            0x82E52278 => {
    //   block [0x82E52278..0x82E52280)
	// 82E52278: 3863FFA0  addi r3, r3, -0x60
	ctx.r[3].s64 = ctx.r[3].s64 + -96;
	// 82E5227C: 48000014  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52280 size=8
    let mut pc: u32 = 0x82E52280;
    'dispatch: loop {
        match pc {
            0x82E52280 => {
    //   block [0x82E52280..0x82E52288)
	// 82E52280: 3863FF9C  addi r3, r3, -0x64
	ctx.r[3].s64 = ctx.r[3].s64 + -100;
	// 82E52284: 4800000C  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52288 size=8
    let mut pc: u32 = 0x82E52288;
    'dispatch: loop {
        match pc {
            0x82E52288 => {
    //   block [0x82E52288..0x82E52290)
	// 82E52288: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E5228C: 48000004  b 0x82e52290
	sub_82E52290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52290 size=100
    let mut pc: u32 = 0x82E52290;
    'dispatch: loop {
        match pc {
            0x82E52290 => {
    //   block [0x82E52290..0x82E522F4)
	// 82E52290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5229C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E522A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E522A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E522A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E522AC: 4BFFFD1D  bl 0x82e51fc8
	ctx.lr = 0x82E522B0;
	sub_82E51FC8(ctx, base);
	// 82E522B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E522B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E522B8: 419A0020  beq cr6, 0x82e522d8
	if ctx.cr[6].eq {
	pc = 0x82E522D8; continue 'dispatch;
	}
	// 82E522BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E522C0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E522C4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E522C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E522CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E522D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E522D4: 4BF02FF5  bl 0x82d552c8
	ctx.lr = 0x82E522D8;
	sub_82D552C8(ctx, base);
	// 82E522D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E522DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E522E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E522E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E522E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E522EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E522F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E522F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E522F8 size=208
    let mut pc: u32 = 0x82E522F8;
    'dispatch: loop {
        match pc {
            0x82E522F8 => {
    //   block [0x82E522F8..0x82E523C8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E523C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E523C8 size=24
    let mut pc: u32 = 0x82E523C8;
    'dispatch: loop {
        match pc {
            0x82E523C8 => {
    //   block [0x82E523C8..0x82E523E0)
	// 82E523C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E523CC: 394A2AF0  addi r10, r10, 0x2af0
	ctx.r[10].s64 = ctx.r[10].s64 + 10992;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E523E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E523E0 size=108
    let mut pc: u32 = 0x82E523E0;
    'dispatch: loop {
        match pc {
            0x82E523E0 => {
    //   block [0x82E523E0..0x82E5244C)
	// 82E523E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E523E4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E523E8: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 82E523EC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E523F0: C00A0C14  lfs f0, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E523F4: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82E523F8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52450 size=376
    let mut pc: u32 = 0x82E52450;
    'dispatch: loop {
        match pc {
            0x82E52450 => {
    //   block [0x82E52450..0x82E525C8)
	// 82E52450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5245C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E52460: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E52464: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E525C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E525C8 size=708
    let mut pc: u32 = 0x82E525C8;
    'dispatch: loop {
        match pc {
            0x82E525C8 => {
    //   block [0x82E525C8..0x82E5288C)
	// 82E525C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E525CC: 4BE56E3D  bl 0x82ca9408
	ctx.lr = 0x82E525D0;
	sub_82CA93D0(ctx, base);
	// 82E525D0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E525D4: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E52890 size=560
    let mut pc: u32 = 0x82E52890;
    'dispatch: loop {
        match pc {
            0x82E52890 => {
    //   block [0x82E52890..0x82E52AC0)
	// 82E52890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52894: 4BE56B79  bl 0x82ca940c
	ctx.lr = 0x82E52898;
	sub_82CA93D0(ctx, base);
	// 82E52898: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E5289C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E528A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E528A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E528A8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E528AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82E528B0: D3C100F4  stfs f30, 0xf4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 82E528B4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52AC0 size=720
    let mut pc: u32 = 0x82E52AC0;
    'dispatch: loop {
        match pc {
            0x82E52AC0 => {
    //   block [0x82E52AC0..0x82E52D90)
	// 82E52AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52AC4: 4BE56949  bl 0x82ca940c
	ctx.lr = 0x82E52AC8;
	sub_82CA93D0(ctx, base);
	// 82E52AC8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E52ACC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E52AD0: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52D90 size=12
    let mut pc: u32 = 0x82E52D90;
    'dispatch: loop {
        match pc {
            0x82E52D90 => {
    //   block [0x82E52D90..0x82E52D9C)
	// 82E52D90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52D94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52DA0 size=12
    let mut pc: u32 = 0x82E52DA0;
    'dispatch: loop {
        match pc {
            0x82E52DA0 => {
    //   block [0x82E52DA0..0x82E52DAC)
	// 82E52DA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52DA4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52DB0 size=12
    let mut pc: u32 = 0x82E52DB0;
    'dispatch: loop {
        match pc {
            0x82E52DB0 => {
    //   block [0x82E52DB0..0x82E52DBC)
	// 82E52DB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52DB4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52DC0 size=12
    let mut pc: u32 = 0x82E52DC0;
    'dispatch: loop {
        match pc {
            0x82E52DC0 => {
    //   block [0x82E52DC0..0x82E52DCC)
	// 82E52DC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52DC4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52DD0 size=4
    let mut pc: u32 = 0x82E52DD0;
    'dispatch: loop {
        match pc {
            0x82E52DD0 => {
    //   block [0x82E52DD0..0x82E52DD4)
	// 82E52DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52DD8 size=72
    let mut pc: u32 = 0x82E52DD8;
    'dispatch: loop {
        match pc {
            0x82E52DD8 => {
    //   block [0x82E52DD8..0x82E52E20)
	// 82E52DD8: 81040030  lwz r8, 0x30(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E52DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E52DE0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E52DE4: 40990030  ble cr6, 0x82e52e14
	if !ctx.cr[6].gt {
	pc = 0x82E52E14; continue 'dispatch;
	}
	// 82E52DE8: 8144002C  lwz r10, 0x2c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E52DEC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52DF0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E52DF4: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82E52DF8: 419A0028  beq cr6, 0x82e52e20
	if ctx.cr[6].eq {
		sub_82E52E20(ctx, base);
		return;
	}
	// 82E52DFC: 7F0B3040  cmplw cr6, r11, r6
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E52E00: 419A0020  beq cr6, 0x82e52e20
	if ctx.cr[6].eq {
		sub_82E52E20(ctx, base);
		return;
	}
	// 82E52E04: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E52E08: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E52E0C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E52E10: 4198FFDC  blt cr6, 0x82e52dec
	if ctx.cr[6].lt {
	pc = 0x82E52DEC; continue 'dispatch;
	}
	// 82E52E14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52E18: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52E20 size=12
    let mut pc: u32 = 0x82E52E20;
    'dispatch: loop {
        match pc {
            0x82E52E20 => {
    //   block [0x82E52E20..0x82E52E2C)
	// 82E52E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52E24: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52E30 size=68
    let mut pc: u32 = 0x82E52E30;
    'dispatch: loop {
        match pc {
            0x82E52E30 => {
    //   block [0x82E52E30..0x82E52E74)
	// 82E52E30: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E52E34: 419A0034  beq cr6, 0x82e52e68
	if ctx.cr[6].eq {
	pc = 0x82E52E68; continue 'dispatch;
	}
	// 82E52E38: 81240038  lwz r9, 0x38(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E52E3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52E40: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E52E44: 40990024  ble cr6, 0x82e52e68
	if !ctx.cr[6].gt {
	pc = 0x82E52E68; continue 'dispatch;
	}
	// 82E52E48: 81440034  lwz r10, 0x34(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E52E4C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52E50: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82E52E54: 419A0020  beq cr6, 0x82e52e74
	if ctx.cr[6].eq {
		sub_82E52E74(ctx, base);
		return;
	}
	// 82E52E58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E52E5C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E52E60: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E52E64: 4198FFE8  blt cr6, 0x82e52e4c
	if ctx.cr[6].lt {
	pc = 0x82E52E4C; continue 'dispatch;
	}
	// 82E52E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52E6C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52E74 size=44
    let mut pc: u32 = 0x82E52E74;
    'dispatch: loop {
        match pc {
            0x82E52E74 => {
    //   block [0x82E52E74..0x82E52EA0)
	// 82E52E74: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E52E78: 81440038  lwz r10, 0x38(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E52E7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52E80: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E52E84: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E52E88: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E52E8C: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E52E90: 91440038  stw r10, 0x38(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E52E94: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E52E98: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E52E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52EA0 size=80
    let mut pc: u32 = 0x82E52EA0;
    'dispatch: loop {
        match pc {
            0x82E52EA0 => {
    //   block [0x82E52EA0..0x82E52EF0)
	// 82E52EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52EAC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E52EB0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82E52EB4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E52EB8: 419A0028  beq cr6, 0x82e52ee0
	if ctx.cr[6].eq {
	pc = 0x82E52EE0; continue 'dispatch;
	}
	// 82E52EBC: 3887FFD0  addi r4, r7, -0x30
	ctx.r[4].s64 = ctx.r[7].s64 + -48;
	// 82E52EC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52EC4: 4BFFFF6D  bl 0x82e52e30
	ctx.lr = 0x82E52EC8;
	sub_82E52E30(ctx, base);
	// 82E52EC8: 3567FFD0  addic. r11, r7, -0x30
	ctx.xer.ca = (ctx.r[7].u32 > (!(-48 as u32)));
	ctx.r[11].s64 = ctx.r[7].s64 + -48;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52ECC: 40820008  bne 0x82e52ed4
	if !ctx.cr[0].eq {
	pc = 0x82E52ED4; continue 'dispatch;
	}
	// 82E52ED0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E52ED4: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82E52ED8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E52EDC: 4BF32F3D  bl 0x82d85e18
	ctx.lr = 0x82E52EE0;
	sub_82D85E18(ctx, base);
	// 82E52EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E52EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52EF0 size=168
    let mut pc: u32 = 0x82E52EF0;
    'dispatch: loop {
        match pc {
            0x82E52EF0 => {
    //   block [0x82E52EF0..0x82E52F98)
	// 82E52EF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52EF4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E52EF8: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E52EFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E52F00: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E52F04: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E52F08: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E52F0C: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E52F10: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E52F14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F18: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E52F1C: 38CB5EB8  addi r6, r11, 0x5eb8
	ctx.r[6].s64 = ctx.r[11].s64 + 24248;
	// 82E52F20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F24: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E52F28: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E52F2C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E52F30: 38AB5EAC  addi r5, r11, 0x5eac
	ctx.r[5].s64 = ctx.r[11].s64 + 24236;
	// 82E52F34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F38: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E52F3C: 394B5E98  addi r10, r11, 0x5e98
	ctx.r[10].s64 = ctx.r[11].s64 + 24216;
	// 82E52F40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F44: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E52F48: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E52F4C: 392B5E8C  addi r9, r11, 0x5e8c
	ctx.r[9].s64 = ctx.r[11].s64 + 24204;
	// 82E52F50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F54: 388B5E80  addi r4, r11, 0x5e80
	ctx.r[4].s64 = ctx.r[11].s64 + 24192;
	// 82E52F58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52F5C: 390B5E68  addi r8, r11, 0x5e68
	ctx.r[8].s64 = ctx.r[11].s64 + 24168;
	// 82E52F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52F64: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E52F68: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E52F6C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E52F70: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E52F74: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E52F78: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E52F7C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E52F80: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E52F84: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E52F88: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E52F8C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E52F90: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E52F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52F98 size=344
    let mut pc: u32 = 0x82E52F98;
    'dispatch: loop {
        match pc {
            0x82E52F98 => {
    //   block [0x82E52F98..0x82E530F0)
	// 82E52F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52F9C: 4BE5646D  bl 0x82ca9408
	ctx.lr = 0x82E52FA0;
	sub_82CA93D0(ctx, base);
	// 82E52FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52FA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E52FAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E52FB0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E52FB4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E52FB8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E52FBC: 80BF0038  lwz r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E52FC0: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E52FC4: 396B5EB8  addi r11, r11, 0x5eb8
	ctx.r[11].s64 = ctx.r[11].s64 + 24248;
	// 82E52FC8: 394A5EAC  addi r10, r10, 0x5eac
	ctx.r[10].s64 = ctx.r[10].s64 + 24236;
	// 82E52FCC: 39295E98  addi r9, r9, 0x5e98
	ctx.r[9].s64 = ctx.r[9].s64 + 24216;
	// 82E52FD0: 39085E8C  addi r8, r8, 0x5e8c
	ctx.r[8].s64 = ctx.r[8].s64 + 24204;
	// 82E52FD4: 38E75E80  addi r7, r7, 0x5e80
	ctx.r[7].s64 = ctx.r[7].s64 + 24192;
	// 82E52FD8: 38C65E68  addi r6, r6, 0x5e68
	ctx.r[6].s64 = ctx.r[6].s64 + 24168;
	// 82E52FDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52FE0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E52FE4: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82E52FE8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E52FEC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E52FF0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E52FF4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E52FF8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82E52FFC: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82E53000: 40990074  ble cr6, 0x82e53074
	if !ctx.cr[6].gt {
	pc = 0x82E53074; continue 'dispatch;
	}
	// 82E53004: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E53008: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5300C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53010: 480001F1  bl 0x82e53200
	ctx.lr = 0x82E53014;
	sub_82E53200(ctx, base);
	// 82E53014: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5301C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E53020: 40990040  ble cr6, 0x82e53060
	if !ctx.cr[6].gt {
	pc = 0x82E53060; continue 'dispatch;
	}
	// 82E53024: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53028: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5302C: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53030: 419A0018  beq cr6, 0x82e53048
	if ctx.cr[6].eq {
	pc = 0x82E53048; continue 'dispatch;
	}
	// 82E53034: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53038: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5303C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E53040: 4198FFE8  blt cr6, 0x82e53028
	if ctx.cr[6].lt {
	pc = 0x82E53028; continue 'dispatch;
	}
	// 82E53044: 4800001C  b 0x82e53060
	pc = 0x82E53060; continue 'dispatch;
	// 82E53048: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5304C: 41980014  blt cr6, 0x82e53060
	if ctx.cr[6].lt {
	pc = 0x82E53060; continue 'dispatch;
	}
	// 82E53050: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E53054: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E53058: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5305C: 4BF32DBD  bl 0x82d85e18
	ctx.lr = 0x82E53060;
	sub_82D85E18(ctx, base);
	// 82E53060: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E53064: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E53068: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E5306C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53070: 4198FF98  blt cr6, 0x82e53008
	if ctx.cr[6].lt {
	pc = 0x82E53008; continue 'dispatch;
	}
	// 82E53074: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E53078: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5307C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53080: 409A0020  bne cr6, 0x82e530a0
	if !ctx.cr[6].eq {
	pc = 0x82E530A0; continue 'dispatch;
	}
	// 82E53084: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53088: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5308C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53090: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E53094: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53098: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5309C: 4BF0222D  bl 0x82d552c8
	ctx.lr = 0x82E530A0;
	sub_82D552C8(ctx, base);
	// 82E530A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E530A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E530A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E530AC: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E530B0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E530B4: 394A8604  addi r10, r10, -0x79fc
	ctx.r[10].s64 = ctx.r[10].s64 + -31228;
	// 82E530B8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E530BC: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E530C0: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 82E530C4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E530C8: 39088610  addi r8, r8, -0x79f0
	ctx.r[8].s64 = ctx.r[8].s64 + -31216;
	// 82E530CC: 38E785E8  addi r7, r7, -0x7a18
	ctx.r[7].s64 = ctx.r[7].s64 + -31256;
	// 82E530D0: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E530D4: 38C639E0  addi r6, r6, 0x39e0
	ctx.r[6].s64 = ctx.r[6].s64 + 14816;
	// 82E530D8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E530DC: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82E530E0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E530E4: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E530E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E530EC: 4BE5636C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E530F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E530F0 size=268
    let mut pc: u32 = 0x82E530F0;
    'dispatch: loop {
        match pc {
            0x82E530F0 => {
    //   block [0x82E530F0..0x82E531FC)
	// 82E530F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E530F4: 4BE56315  bl 0x82ca9408
	ctx.lr = 0x82E530F8;
	sub_82CA93D0(ctx, base);
	// 82E530F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E530FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E53100: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E53104: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53108: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E5310C: 409A0018  bne cr6, 0x82e53124
	if !ctx.cr[6].eq {
	pc = 0x82E53124; continue 'dispatch;
	}
	// 82E53110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53114: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E53118: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E5311C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E53120: 4BE56338  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82E53124: 813E0038  lwz r9, 0x38(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E53128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5312C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E53130: 40990024  ble cr6, 0x82e53154
	if !ctx.cr[6].gt {
	pc = 0x82E53154; continue 'dispatch;
	}
	// 82E53134: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E53138: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5313C: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53140: 419AFFD0  beq cr6, 0x82e53110
	if ctx.cr[6].eq {
	pc = 0x82E53110; continue 'dispatch;
	}
	// 82E53144: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E53148: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E5314C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E53150: 4198FFE8  blt cr6, 0x82e53138
	if ctx.cr[6].lt {
	pc = 0x82E53138; continue 'dispatch;
	}
	// 82E53154: 3BFE0034  addi r31, r30, 0x34
	ctx.r[31].s64 = ctx.r[30].s64 + 52;
	// 82E53158: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5315C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53160: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53164: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53168: 409A0010  bne cr6, 0x82e53178
	if !ctx.cr[6].eq {
	pc = 0x82E53178; continue 'dispatch;
	}
	// 82E5316C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E53170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53174: 4BF03E25  bl 0x82d56f98
	ctx.lr = 0x82E53178;
	sub_82D56F98(ctx, base);
	// 82E53178: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5317C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E53180: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53184: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82E53188: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5318C: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E53190: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53194: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53198: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5319C: 48000065  bl 0x82e53200
	ctx.lr = 0x82E531A0;
	sub_82E53200(ctx, base);
	// 82E531A0: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E531A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E531A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E531AC: 40990030  ble cr6, 0x82e531dc
	if !ctx.cr[6].gt {
	pc = 0x82E531DC; continue 'dispatch;
	}
	// 82E531B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E531B4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E531B8: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E531BC: 419A0018  beq cr6, 0x82e531d4
	if ctx.cr[6].eq {
	pc = 0x82E531D4; continue 'dispatch;
	}
	// 82E531C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E531C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E531C8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E531CC: 4198FFE8  blt cr6, 0x82e531b4
	if ctx.cr[6].lt {
	pc = 0x82E531B4; continue 'dispatch;
	}
	// 82E531D0: 4800000C  b 0x82e531dc
	pc = 0x82E531DC; continue 'dispatch;
	// 82E531D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E531D8: 40980010  bge cr6, 0x82e531e8
	if !ctx.cr[6].lt {
	pc = 0x82E531E8; continue 'dispatch;
	}
	// 82E531DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E531E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E531E4: 4BF32B05  bl 0x82d85ce8
	ctx.lr = 0x82E531E8;
	sub_82D85CE8(ctx, base);
	// 82E531E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E531EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E531F0: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E531F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E531F8: 4BE56260  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53200 size=176
    let mut pc: u32 = 0x82E53200;
    'dispatch: loop {
        match pc {
            0x82E53200 => {
    //   block [0x82E53200..0x82E532B0)
	// 82E53200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5320C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53210: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53214: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82E53218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5321C: 409A0078  bne cr6, 0x82e53294
	if !ctx.cr[6].eq {
	pc = 0x82E53294; continue 'dispatch;
	}
	// 82E53220: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53224: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E53228: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5322C: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E53230: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53234: 419A001C  beq cr6, 0x82e53250
	if ctx.cr[6].eq {
	pc = 0x82E53250; continue 'dispatch;
	}
	// 82E53238: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E5323C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E53240: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E53244: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53248: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E5324C: 48000010  b 0x82e5325c
	pc = 0x82E5325C; continue 'dispatch;
	// 82E53250: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E53254: 4BF01DFD  bl 0x82d55050
	ctx.lr = 0x82E53258;
	sub_82D55050(ctx, base);
	// 82E53258: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E5325C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53260: 419A002C  beq cr6, 0x82e5328c
	if ctx.cr[6].eq {
	pc = 0x82E5328C; continue 'dispatch;
	}
	// 82E53264: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82E53268: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E5326C: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82E53270: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E53274: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82E53278: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E5327C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E53280: B12B000C  sth r9, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82E53284: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82E53288: 48000008  b 0x82e53290
	pc = 0x82E53290; continue 'dispatch;
	// 82E5328C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53290: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82E53294: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82E53298: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E5329C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E532A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E532A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E532A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E532AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E532B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E532B0 size=276
    let mut pc: u32 = 0x82E532B0;
    'dispatch: loop {
        match pc {
            0x82E532B0 => {
    //   block [0x82E532B0..0x82E533C4)
	// 82E532B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E532B4: 4BE56149  bl 0x82ca93fc
	ctx.lr = 0x82E532B8;
	sub_82CA93D0(ctx, base);
	// 82E532B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E532BC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E532C0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E532C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E532C8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82E532CC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82E532D0: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E532D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E532D8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E532DC: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E532E0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E532E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E532E8: 409900D4  ble cr6, 0x82e533bc
	if !ctx.cr[6].gt {
	pc = 0x82E533BC; continue 'dispatch;
	}
	// 82E532EC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E532F0: 815A0014  lwz r10, 0x14(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E532F4: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82E532F8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E532FC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53300: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53304: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53308: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5330C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53310: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53314: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E53318: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E5331C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E53320: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53324: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E53328: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5332C: 419A003C  beq cr6, 0x82e53368
	if ctx.cr[6].eq {
	pc = 0x82E53368; continue 'dispatch;
	}
	// 82E53330: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53338: 419A0030  beq cr6, 0x82e53368
	if ctx.cr[6].eq {
	pc = 0x82E53368; continue 'dispatch;
	}
	// 82E5333C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53340: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53344: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5334C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53350: 409A0018  bne cr6, 0x82e53368
	if !ctx.cr[6].eq {
	pc = 0x82E53368; continue 'dispatch;
	}
	// 82E53354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53358: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5335C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53364: 4E800421  bctrl
	ctx.lr = 0x82E53368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53368: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5336C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82E53370: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E53374: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53378: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E5337C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53380: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E53384: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E53388: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5338C: 7F2B492E  stwx r25, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u32) };
	// 82E53390: 419A001C  beq cr6, 0x82e533ac
	if ctx.cr[6].eq {
	pc = 0x82E533AC; continue 'dispatch;
	}
	// 82E53394: A1790004  lhz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5339C: 419A0010  beq cr6, 0x82e533ac
	if ctx.cr[6].eq {
	pc = 0x82E533AC; continue 'dispatch;
	}
	// 82E533A0: A1790006  lhz r11, 6(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E533A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E533A8: B1790006  sth r11, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E533AC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E533B0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82E533B4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E533B8: 4198FF34  blt cr6, 0x82e532ec
	if ctx.cr[6].lt {
	pc = 0x82E532EC; continue 'dispatch;
	}
	// 82E533BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E533C0: 4BE5608C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E533C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E533C8 size=36
    let mut pc: u32 = 0x82E533C8;
    'dispatch: loop {
        match pc {
            0x82E533C8 => {
    //   block [0x82E533C8..0x82E533EC)
	// 82E533C8: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E533CC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E533D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E533D4: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82E533D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E533DC: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E533E0: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E533E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E533E8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E533EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E533EC size=80
    let mut pc: u32 = 0x82E533EC;
    'dispatch: loop {
        match pc {
            0x82E533EC => {
    //   block [0x82E533EC..0x82E5343C)
	// 82E533EC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82E533F0: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53440 size=212
    let mut pc: u32 = 0x82E53440;
    'dispatch: loop {
        match pc {
            0x82E53440 => {
    //   block [0x82E53440..0x82E53514)
	// 82E53440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53444: 4BE55FBD  bl 0x82ca9400
	ctx.lr = 0x82E53448;
	sub_82CA93D0(ctx, base);
	// 82E53448: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5344C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E53450: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53454: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E53458: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82E5345C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E53460: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53464: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53468: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5346C: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53470: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53478: 40990094  ble cr6, 0x82e5350c
	if !ctx.cr[6].gt {
	pc = 0x82E5350C; continue 'dispatch;
	}
	// 82E5347C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53480: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E53484: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E53488: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E5348C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53490: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53494: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53498: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E5349C: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E534A0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E534A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E534A8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82E534AC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E534B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E534B4: 7FEB482E  lwzx r31, r11, r9
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E534B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E534BC: 419A0040  beq cr6, 0x82e534fc
	if ctx.cr[6].eq {
	pc = 0x82E534FC; continue 'dispatch;
	}
	// 82E534C0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E534C4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E534C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E534CC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E534D0: 409A0010  bne cr6, 0x82e534e0
	if !ctx.cr[6].eq {
	pc = 0x82E534E0; continue 'dispatch;
	}
	// 82E534D4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E534D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E534DC: 4BF03ABD  bl 0x82d56f98
	ctx.lr = 0x82E534E0;
	sub_82D56F98(ctx, base);
	// 82E534E0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E534E4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E534E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E534EC: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82E534F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E534F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E534F8: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E534FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53500: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E53504: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53508: 4198FF74  blt cr6, 0x82e5347c
	if ctx.cr[6].lt {
	pc = 0x82E5347C; continue 'dispatch;
	}
	// 82E5350C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E53510: 4BE55F40  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53518 size=2984
    let mut pc: u32 = 0x82E53518;
    'dispatch: loop {
        match pc {
            0x82E53518 => {
    //   block [0x82E53518..0x82E540C0)
	// 82E53518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5351C: 4BE55EB5  bl 0x82ca93d0
	ctx.lr = 0x82E53520;
	sub_82CA93D0(ctx, base);
	// 82E53520: 9421FC70  stwu r1, -0x390(r1)
	ea = ctx.r[1].u32.wrapping_add(-912 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53524: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53528: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E5352C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53530: B06103A6  sth r3, 0x3a6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(934 as u32), ctx.r[3].u16 ) };
	// 82E53534: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 82E53538: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 82E5353C: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 82E53540: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82E53544: 934103BC  stw r26, 0x3bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(956 as u32), ctx.r[26].u32 ) };
	// 82E53548: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E5354C: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82E53550: 4BF01CF9  bl 0x82d55248
	ctx.lr = 0x82E53554;
	sub_82D55248(ctx, base);
	// 82E53554: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82E53558: 3940002C  li r10, 0x2c
	ctx.r[10].s64 = 44;
	// 82E5355C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53560: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E53564: 396B6004  addi r11, r11, 0x6004
	ctx.r[11].s64 = ctx.r[11].s64 + 24580;
	// 82E53568: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 82E5356C: B1550004  sth r10, 4(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82E53570: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E53574: 3B350008  addi r25, r21, 8
	ctx.r[25].s64 = ctx.r[21].s64 + 8;
	// 82E53578: 3A550014  addi r18, r21, 0x14
	ctx.r[18].s64 = ctx.r[21].s64 + 20;
	// 82E5357C: 3AF50020  addi r23, r21, 0x20
	ctx.r[23].s64 = ctx.r[21].s64 + 32;
	// 82E53580: 91750000  stw r11, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53584: B1550006  sth r10, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E53588: 93B90000  stw r29, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E5358C: 93B90004  stw r29, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E53590: 92D90008  stw r22, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82E53594: 93B20000  stw r29, 0(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E53598: 93B20004  stw r29, 4(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E5359C: 92D20008  stw r22, 8(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82E535A0: 93B70000  stw r29, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E535A4: 93B70004  stw r29, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E535A8: 92D70008  stw r22, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82E535AC: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E535B0: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82E535B4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E535B8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E535BC: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82E535C0: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82E535C4: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82E535C8: 92C10080  stw r22, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[22].u32 ) };
	// 82E535CC: 4099001C  ble cr6, 0x82e535e8
	if !ctx.cr[6].gt {
	pc = 0x82E535E8; continue 'dispatch;
	}
	// 82E535D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E535D4: 41980008  blt cr6, 0x82e535dc
	if ctx.cr[6].lt {
	pc = 0x82E535DC; continue 'dispatch;
	}
	// 82E535D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E535DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E535E0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E535E4: 4BF0392D  bl 0x82d56f10
	ctx.lr = 0x82E535E8;
	sub_82D56F10(ctx, base);
	// 82E535E8: 57E5103A  slwi r5, r31, 2
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E535EC: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E535F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E535F4: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 82E535F8: 4BF05749  bl 0x82d58d40
	ctx.lr = 0x82E535FC;
	sub_82D58D40(ctx, base);
	// 82E535FC: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53600: 4B413941  bl 0x82266f40
	ctx.lr = 0x82E53604;
	sub_82266F40(ctx, base);
	// 82E53604: 816E0004  lwz r11, 4(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53608: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E5360C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53610: 40990030  ble cr6, 0x82e53640
	if !ctx.cr[6].gt {
	pc = 0x82E53640; continue 'dispatch;
	}
	// 82E53614: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E53618: 816E0000  lwz r11, 0(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5361C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E53620: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53624: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53628: 4B4125D9  bl 0x82265c00
	ctx.lr = 0x82E5362C;
	sub_82265C00(ctx, base);
	// 82E5362C: 816E0004  lwz r11, 4(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53630: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E53634: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E53638: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5363C: 4198FFDC  blt cr6, 0x82e53618
	if ctx.cr[6].lt {
	pc = 0x82E53618; continue 'dispatch;
	}
	// 82E53640: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53644: 83EE0004  lwz r31, 4(r14)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53648: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5364C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E53650: 40980024  bge cr6, 0x82e53674
	if !ctx.cr[6].lt {
	pc = 0x82E53674; continue 'dispatch;
	}
	// 82E53654: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53658: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5365C: 41980008  blt cr6, 0x82e53664
	if ctx.cr[6].lt {
	pc = 0x82E53664; continue 'dispatch;
	}
	// 82E53660: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E53664: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82E53668: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5366C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E53670: 4BF038A1  bl 0x82d56f10
	ctx.lr = 0x82E53674;
	sub_82D56F10(ctx, base);
	// 82E53674: 93F90004  stw r31, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E53678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5367C: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53680: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53684: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53688: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E5368C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53690: 4BF056B1  bl 0x82d58d40
	ctx.lr = 0x82E53694;
	sub_82D58D40(ctx, base);
	// 82E53694: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53698: 7FB1EB78  mr r17, r29
	ctx.r[17].u64 = ctx.r[29].u64;
	// 82E5369C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E536A0: 40990240  ble cr6, 0x82e538e0
	if !ctx.cr[6].gt {
	pc = 0x82E538E0; continue 'dispatch;
	}
	// 82E536A4: 89E103A7  lbz r15, 0x3a7(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(935 as u32) ) } as u64;
	// 82E536A8: 7FB4EB78  mr r20, r29
	ctx.r[20].u64 = ctx.r[29].u64;
	// 82E536AC: 8A6103A6  lbz r19, 0x3a6(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(934 as u32) ) } as u64;
	// 82E536B0: 81700000  lwz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E536B4: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 82E536B8: 38C10088  addi r6, r1, 0x88
	ctx.r[6].s64 = ctx.r[1].s64 + 136;
	// 82E536BC: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82E536C0: 7D745A14  add r11, r20, r11
	ctx.r[11].u64 = ctx.r[20].u64 + ctx.r[11].u64;
	// 82E536C4: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82E536C8: 7DC37378  mr r3, r14
	ctx.r[3].u64 = ctx.r[14].u64;
	// 82E536CC: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 82E536D0: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82E536D4: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82E536D8: 92C10070  stw r22, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[22].u32 ) };
	// 82E536DC: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E536E0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E536E4: 480113A5  bl 0x82e64a88
	ctx.lr = 0x82E536E8;
	sub_82E64A88(ctx, base);
	// 82E536E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E536EC: 419A03F4  beq cr6, 0x82e53ae0
	if ctx.cr[6].eq {
	pc = 0x82E53AE0; continue 'dispatch;
	}
	// 82E536F0: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82E536F4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E536F8: 48010B29  bl 0x82e64220
	ctx.lr = 0x82E536FC;
	sub_82E64220(ctx, base);
	// 82E536FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E53700: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E53704: 419A0534  beq cr6, 0x82e53c38
	if ctx.cr[6].eq {
	pc = 0x82E53C38; continue 'dispatch;
	}
	// 82E53708: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5370C: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53710: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53714: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53718: 409A0010  bne cr6, 0x82e53728
	if !ctx.cr[6].eq {
	pc = 0x82E53728; continue 'dispatch;
	}
	// 82E5371C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E53720: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E53724: 4BF03875  bl 0x82d56f98
	ctx.lr = 0x82E53728;
	sub_82D56F98(ctx, base);
	// 82E53728: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5372C: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 82E53730: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53734: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53738: 7F6B512E  stwx r27, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 82E5373C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53740: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53744: 91770004  stw r11, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E53748: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5374C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53750: 40990120  ble cr6, 0x82e53870
	if !ctx.cr[6].gt {
	pc = 0x82E53870; continue 'dispatch;
	}
	// 82E53754: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82E53758: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E5375C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53760: 7C9C582E  lwzx r4, r28, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53764: 4BF0CECD  bl 0x82d60630
	ctx.lr = 0x82E53768;
	sub_82D60630(ctx, base);
	// 82E53768: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E5376C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E53770: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53774: 40990008  ble cr6, 0x82e5377c
	if !ctx.cr[6].gt {
	pc = 0x82E5377C; continue 'dispatch;
	}
	// 82E53778: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E5377C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E53780: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53784: 419A060C  beq cr6, 0x82e53d90
	if ctx.cr[6].eq {
	pc = 0x82E53D90; continue 'dispatch;
	}
	// 82E53788: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E5378C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E53790: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53794: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E53798: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E5379C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E537A0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E537A4: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E537A8: 81010098  lwz r8, 0x98(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E537AC: 7FEA402E  lwzx r31, r10, r8
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E537B0: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E537B4: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E537B8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E537BC: 7F4A4A14  add r26, r10, r9
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E537C0: 409A0014  bne cr6, 0x82e537d4
	if !ctx.cr[6].eq {
	pc = 0x82E537D4; continue 'dispatch;
	}
	// 82E537C4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E537C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E537CC: 4BF037CD  bl 0x82d56f98
	ctx.lr = 0x82E537D0;
	sub_82D56F98(ctx, base);
	// 82E537D0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E537D4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82E537D8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E537DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82E537E0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E537E4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E537E8: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E537EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E537F0: 7FCB4A14  add r30, r11, r9
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E537F4: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E537F8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E537FC: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53800: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E53804: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82E53808: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5380C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53810: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E53814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53818: 4E800421  bctrl
	ctx.lr = 0x82E5381C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5381C: 2F030066  cmpwi cr6, r3, 0x66
	ctx.cr[6].compare_i32(ctx.r[3].s32, 102, &mut ctx.xer);
	// 82E53820: 409A0708  bne cr6, 0x82e53f28
	if !ctx.cr[6].eq {
	pc = 0x82E53F28; continue 'dispatch;
	}
	// 82E53824: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53828: 7E6A0774  extsb r10, r19
	ctx.r[10].s64 = ctx.r[19].s8 as i64;
	// 82E5382C: 931E0004  stw r24, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82E53830: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53834: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53838: 419A0020  beq cr6, 0x82e53858
	if ctx.cr[6].eq {
	pc = 0x82E53858; continue 'dispatch;
	}
	// 82E5383C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53844: 409A0014  bne cr6, 0x82e53858
	if !ctx.cr[6].eq {
	pc = 0x82E53858; continue 'dispatch;
	}
	// 82E53848: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E5384C: 7C7C582E  lwzx r3, r28, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53850: 4800FE81  bl 0x82e636d0
	ctx.lr = 0x82E53854;
	sub_82E636D0(ctx, base);
	// 82E53854: 907A0008  stw r3, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E53858: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E5385C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82E53860: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E53864: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53868: 4198FEF0  blt cr6, 0x82e53758
	if ctx.cr[6].lt {
	pc = 0x82E53758; continue 'dispatch;
	}
	// 82E5386C: 834103BC  lwz r26, 0x3bc(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(956 as u32) ) } as u64;
	// 82E53870: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E53874: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53878: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5387C: 409A0020  bne cr6, 0x82e5389c
	if !ctx.cr[6].eq {
	pc = 0x82E5389C; continue 'dispatch;
	}
	// 82E53880: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53884: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E53888: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E5388C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E53890: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53894: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E53898: 4BF01A31  bl 0x82d552c8
	ctx.lr = 0x82E5389C;
	sub_82D552C8(ctx, base);
	// 82E5389C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E538A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E538A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E538A8: 409A0020  bne cr6, 0x82e538c8
	if !ctx.cr[6].eq {
	pc = 0x82E538C8; continue 'dispatch;
	}
	// 82E538AC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E538B0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E538B4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E538B8: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E538BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E538C0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E538C4: 4BF01A05  bl 0x82d552c8
	ctx.lr = 0x82E538C8;
	sub_82D552C8(ctx, base);
	// 82E538C8: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E538CC: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82E538D0: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E538D4: 3A940008  addi r20, r20, 8
	ctx.r[20].s64 = ctx.r[20].s64 + 8;
	// 82E538D8: 7F115800  cmpw cr6, r17, r11
	ctx.cr[6].compare_i32(ctx.r[17].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E538DC: 4198FDD4  blt cr6, 0x82e536b0
	if ctx.cr[6].lt {
	pc = 0x82E536B0; continue 'dispatch;
	}
	// 82E538E0: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E538E4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E538E8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82E538EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E538F0: 40990044  ble cr6, 0x82e53934
	if !ctx.cr[6].gt {
	pc = 0x82E53934; continue 'dispatch;
	}
	// 82E538F4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E538F8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E538FC: 81190000  lwz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53900: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E53904: 7FEB412E  stwx r31, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[31].u32) };
	// 82E53908: 80F90000  lwz r7, 0(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5390C: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E53910: 7CEB3A14  add r7, r11, r7
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E53914: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E53918: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E5391C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E53920: 93A70004  stw r29, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E53924: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82E53928: 8115000C  lwz r8, 0xc(r21)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5392C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E53930: 4198FFCC  blt cr6, 0x82e538fc
	if ctx.cr[6].lt {
	pc = 0x82E538FC; continue 'dispatch;
	}
	// 82E53934: 81720008  lwz r11, 8(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53938: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5393C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E53940: 40980024  bge cr6, 0x82e53964
	if !ctx.cr[6].lt {
	pc = 0x82E53964; continue 'dispatch;
	}
	// 82E53944: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53948: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5394C: 41980008  blt cr6, 0x82e53954
	if ctx.cr[6].lt {
	pc = 0x82E53954; continue 'dispatch;
	}
	// 82E53950: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E53954: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E53958: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5395C: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 82E53960: 4BF035B1  bl 0x82d56f10
	ctx.lr = 0x82E53964;
	sub_82D56F10(ctx, base);
	// 82E53964: 93F20004  stw r31, 4(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E53968: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82E5396C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E53970: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53974: 40990070  ble cr6, 0x82e539e4
	if !ctx.cr[6].gt {
	pc = 0x82E539E4; continue 'dispatch;
	}
	// 82E53978: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82E5397C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53980: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E53984: 80F90000  lwz r7, 0(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53988: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E5398C: 80D20000  lwz r6, 0(r18)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53990: 3908000C  addi r8, r8, 0xc
	ctx.r[8].s64 = ctx.r[8].s64 + 12;
	// 82E53994: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53998: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5399C: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E539A0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E539A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E539A8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E539AC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E539B0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E539B4: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 82E539B8: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E539BC: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82E539C0: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E539C4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E539C8: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E539CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E539D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E539D4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E539D8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E539DC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E539E0: 4198FF9C  blt cr6, 0x82e5397c
	if ctx.cr[6].lt {
	pc = 0x82E5397C; continue 'dispatch;
	}
	// 82E539E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E539E8: 419A0088  beq cr6, 0x82e53a70
	if ctx.cr[6].eq {
	pc = 0x82E53A70; continue 'dispatch;
	}
	// 82E539EC: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E539F0: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82E539F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E539F8: 40990078  ble cr6, 0x82e53a70
	if !ctx.cr[6].gt {
	pc = 0x82E53A70; continue 'dispatch;
	}
	// 82E539FC: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E53A00: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53A04: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E53A08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53A10: 409A0048  bne cr6, 0x82e53a58
	if !ctx.cr[6].eq {
	pc = 0x82E53A58; continue 'dispatch;
	}
	// 82E53A14: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53A18: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A1C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53A20: 83CE0000  lwz r30, 0(r14)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53A24: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53A28: 409A0010  bne cr6, 0x82e53a38
	if !ctx.cr[6].eq {
	pc = 0x82E53A38; continue 'dispatch;
	}
	// 82E53A2C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E53A30: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E53A34: 4BF03565  bl 0x82d56f98
	ctx.lr = 0x82E53A38;
	sub_82D56F98(ctx, base);
	// 82E53A38: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A3C: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E53A40: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53A44: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53A48: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82E53A4C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A50: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53A54: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E53A58: 8175000C  lwz r11, 0xc(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53A5C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E53A60: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82E53A64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E53A68: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E53A6C: 4198FF94  blt cr6, 0x82e53a00
	if ctx.cr[6].lt {
	pc = 0x82E53A00; continue 'dispatch;
	}
	// 82E53A70: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53A74: 4B413495  bl 0x82266f08
	ctx.lr = 0x82E53A78;
	sub_82266F08(ctx, base);
	// 82E53A78: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E53A7C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53A80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53A84: 409A001C  bne cr6, 0x82e53aa0
	if !ctx.cr[6].eq {
	pc = 0x82E53AA0; continue 'dispatch;
	}
	// 82E53A88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53A8C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E53A90: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53A94: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53A98: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53A9C: 4BF0182D  bl 0x82d552c8
	ctx.lr = 0x82E53AA0;
	sub_82D552C8(ctx, base);
	// 82E53AA0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E53AA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53AA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53AAC: 409A0028  bne cr6, 0x82e53ad4
	if !ctx.cr[6].eq {
	pc = 0x82E53AD4; continue 'dispatch;
	}
	// 82E53AB0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E53AB4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53AB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53ABC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53AC0: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53AC4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53AC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53ACC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53AD0: 4BF017F9  bl 0x82d552c8
	ctx.lr = 0x82E53AD4;
	sub_82D552C8(ctx, base);
	// 82E53AD4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E53AD8: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82E53ADC: 4BE55944  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
	// 82E53AE0: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E53AE4: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E53AE8: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82E53AEC: 4BF03EFD  bl 0x82d579e8
	ctx.lr = 0x82E53AF0;
	sub_82D579E8(ctx, base);
	// 82E53AF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53AF4: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82E53AF8: 388B7340  addi r4, r11, 0x7340
	ctx.r[4].s64 = ctx.r[11].s64 + 29504;
	// 82E53AFC: 4BF042F5  bl 0x82d57df0
	ctx.lr = 0x82E53B00;
	sub_82D57DF0(ctx, base);
	// 82E53B00: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E53B04: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82E53B08: 39000044  li r8, 0x44
	ctx.r[8].s64 = 68;
	// 82E53B0C: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82E53B10: 60A5AA88  ori r5, r5, 0xaa88
	ctx.r[5].u64 = ctx.r[5].u64 | 43656;
	// 82E53B14: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E53B18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53B1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53B20: 38EB7310  addi r7, r11, 0x7310
	ctx.r[7].s64 = ctx.r[11].s64 + 29456;
	// 82E53B24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53B28: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53B2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53B30: 4E800421  bctrl
	ctx.lr = 0x82E53B34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53B34: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82E53B38: 4BF048F9  bl 0x82d58430
	ctx.lr = 0x82E53B3C;
	sub_82D58430(ctx, base);
	// 82E53B3C: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53B40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53B44: 419A0034  beq cr6, 0x82e53b78
	if ctx.cr[6].eq {
	pc = 0x82E53B78; continue 'dispatch;
	}
	// 82E53B48: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53B4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53B50: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53B58: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53B5C: 409A001C  bne cr6, 0x82e53b78
	if !ctx.cr[6].eq {
	pc = 0x82E53B78; continue 'dispatch;
	}
	// 82E53B60: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53B64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53B68: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E53B6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53B74: 4E800421  bctrl
	ctx.lr = 0x82E53B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53B78: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E53B7C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53B80: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53B84: 409A001C  bne cr6, 0x82e53ba0
	if !ctx.cr[6].eq {
	pc = 0x82E53BA0; continue 'dispatch;
	}
	// 82E53B88: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53B8C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E53B90: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53B94: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53B98: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53B9C: 4BF0172D  bl 0x82d552c8
	ctx.lr = 0x82E53BA0;
	sub_82D552C8(ctx, base);
	// 82E53BA0: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E53BA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53BA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53BAC: 409A001C  bne cr6, 0x82e53bc8
	if !ctx.cr[6].eq {
	pc = 0x82E53BC8; continue 'dispatch;
	}
	// 82E53BB0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53BB4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E53BB8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53BBC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53BC0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53BC4: 4BF01705  bl 0x82d552c8
	ctx.lr = 0x82E53BC8;
	sub_82D552C8(ctx, base);
	// 82E53BC8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53BCC: 4B41333D  bl 0x82266f08
	ctx.lr = 0x82E53BD0;
	sub_82266F08(ctx, base);
	// 82E53BD0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E53BD4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53BD8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53BDC: 409A001C  bne cr6, 0x82e53bf8
	if !ctx.cr[6].eq {
	pc = 0x82E53BF8; continue 'dispatch;
	}
	// 82E53BE0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53BE4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E53BE8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53BEC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53BF0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53BF4: 4BF016D5  bl 0x82d552c8
	ctx.lr = 0x82E53BF8;
	sub_82D552C8(ctx, base);
	// 82E53BF8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E53BFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53C00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53C04: 409A0028  bne cr6, 0x82e53c2c
	if !ctx.cr[6].eq {
	pc = 0x82E53C2C; continue 'dispatch;
	}
	// 82E53C08: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E53C0C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53C10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53C14: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53C18: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53C1C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53C20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53C24: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53C28: 4BF016A1  bl 0x82d552c8
	ctx.lr = 0x82E53C2C;
	sub_82D552C8(ctx, base);
	// 82E53C2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E53C30: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82E53C34: 4BE557EC  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
	// 82E53C38: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E53C3C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E53C40: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82E53C44: 4BF03DA5  bl 0x82d579e8
	ctx.lr = 0x82E53C48;
	sub_82D579E8(ctx, base);
	// 82E53C48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53C4C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82E53C50: 388B72F4  addi r4, r11, 0x72f4
	ctx.r[4].s64 = ctx.r[11].s64 + 29428;
	// 82E53C54: 4BF0419D  bl 0x82d57df0
	ctx.lr = 0x82E53C58;
	sub_82D57DF0(ctx, base);
	// 82E53C58: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E53C5C: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82E53C60: 3900004E  li r8, 0x4e
	ctx.r[8].s64 = 78;
	// 82E53C64: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82E53C68: 60A5DDAA  ori r5, r5, 0xddaa
	ctx.r[5].u64 = ctx.r[5].u64 | 56746;
	// 82E53C6C: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E53C70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53C74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53C78: 38EB7310  addi r7, r11, 0x7310
	ctx.r[7].s64 = ctx.r[11].s64 + 29456;
	// 82E53C7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53C80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53C84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53C88: 4E800421  bctrl
	ctx.lr = 0x82E53C8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53C8C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82E53C90: 4BF047A1  bl 0x82d58430
	ctx.lr = 0x82E53C94;
	sub_82D58430(ctx, base);
	// 82E53C94: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53C98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53C9C: 419A0034  beq cr6, 0x82e53cd0
	if ctx.cr[6].eq {
	pc = 0x82E53CD0; continue 'dispatch;
	}
	// 82E53CA0: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53CA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53CA8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53CAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53CB0: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53CB4: 409A001C  bne cr6, 0x82e53cd0
	if !ctx.cr[6].eq {
	pc = 0x82E53CD0; continue 'dispatch;
	}
	// 82E53CB8: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53CBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53CC0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E53CC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53CC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53CCC: 4E800421  bctrl
	ctx.lr = 0x82E53CD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53CD0: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E53CD4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53CD8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53CDC: 409A001C  bne cr6, 0x82e53cf8
	if !ctx.cr[6].eq {
	pc = 0x82E53CF8; continue 'dispatch;
	}
	// 82E53CE0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53CE4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E53CE8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53CEC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53CF0: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53CF4: 4BF015D5  bl 0x82d552c8
	ctx.lr = 0x82E53CF8;
	sub_82D552C8(ctx, base);
	// 82E53CF8: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E53CFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53D00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53D04: 409A001C  bne cr6, 0x82e53d20
	if !ctx.cr[6].eq {
	pc = 0x82E53D20; continue 'dispatch;
	}
	// 82E53D08: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53D0C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E53D10: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53D14: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53D18: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53D1C: 4BF015AD  bl 0x82d552c8
	ctx.lr = 0x82E53D20;
	sub_82D552C8(ctx, base);
	// 82E53D20: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53D24: 4B4131E5  bl 0x82266f08
	ctx.lr = 0x82E53D28;
	sub_82266F08(ctx, base);
	// 82E53D28: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E53D2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53D30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53D34: 409A001C  bne cr6, 0x82e53d50
	if !ctx.cr[6].eq {
	pc = 0x82E53D50; continue 'dispatch;
	}
	// 82E53D38: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53D3C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E53D40: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53D44: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53D48: 7C6BC02E  lwzx r3, r11, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53D4C: 4BF0157D  bl 0x82d552c8
	ctx.lr = 0x82E53D50;
	sub_82D552C8(ctx, base);
	// 82E53D50: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E53D54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53D58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53D5C: 409AFED0  bne cr6, 0x82e53c2c
	if !ctx.cr[6].eq {
	pc = 0x82E53C2C; continue 'dispatch;
	}
	// 82E53D60: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E53D64: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53D68: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53D6C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53D70: 7C6AC02E  lwzx r3, r10, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E53D74: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53D78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53D7C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53D80: 4BF01549  bl 0x82d552c8
	ctx.lr = 0x82E53D84;
	sub_82D552C8(ctx, base);
	// 82E53D84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E53D88: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82E53D8C: 4BE55694  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
	// 82E53D90: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E53D94: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E53D98: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82E53D9C: 4BF03C4D  bl 0x82d579e8
	ctx.lr = 0x82E53DA0;
	sub_82D579E8(ctx, base);
	// 82E53DA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53DA4: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82E53DA8: 388B72E4  addi r4, r11, 0x72e4
	ctx.r[4].s64 = ctx.r[11].s64 + 29412;
	// 82E53DAC: 4BF04045  bl 0x82d57df0
	ctx.lr = 0x82E53DB0;
	sub_82D57DF0(ctx, base);
	// 82E53DB0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E53DB4: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82E53DB8: 3900005D  li r8, 0x5d
	ctx.r[8].s64 = 93;
	// 82E53DBC: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82E53DC0: 60A599DD  ori r5, r5, 0x99dd
	ctx.r[5].u64 = ctx.r[5].u64 | 39389;
	// 82E53DC4: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E53DC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53DCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53DD0: 38EB7310  addi r7, r11, 0x7310
	ctx.r[7].s64 = ctx.r[11].s64 + 29456;
	// 82E53DD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53DD8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53DDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53DE0: 4E800421  bctrl
	ctx.lr = 0x82E53DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53DE4: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 82E53DE8: 4BF04649  bl 0x82d58430
	ctx.lr = 0x82E53DEC;
	sub_82D58430(ctx, base);
	// 82E53DEC: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53DF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53DF4: 419A0034  beq cr6, 0x82e53e28
	if ctx.cr[6].eq {
	pc = 0x82E53E28; continue 'dispatch;
	}
	// 82E53DF8: A17B0006  lhz r11, 6(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53DFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53E00: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53E04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53E08: B17B0006  sth r11, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53E0C: 409A001C  bne cr6, 0x82e53e28
	if !ctx.cr[6].eq {
	pc = 0x82E53E28; continue 'dispatch;
	}
	// 82E53E10: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53E14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53E18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E53E1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53E24: 4E800421  bctrl
	ctx.lr = 0x82E53E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53E28: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53E2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53E30: 419A0034  beq cr6, 0x82e53e64
	if ctx.cr[6].eq {
	pc = 0x82E53E64; continue 'dispatch;
	}
	// 82E53E34: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53E38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53E3C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53E40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53E44: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53E48: 409A001C  bne cr6, 0x82e53e64
	if !ctx.cr[6].eq {
	pc = 0x82E53E64; continue 'dispatch;
	}
	// 82E53E4C: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53E50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53E54: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E53E58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53E5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53E60: 4E800421  bctrl
	ctx.lr = 0x82E53E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53E64: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E53E68: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E53E6C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53E70: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53E74: 409A001C  bne cr6, 0x82e53e90
	if !ctx.cr[6].eq {
	pc = 0x82E53E90; continue 'dispatch;
	}
	// 82E53E78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53E7C: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E53E80: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53E84: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53E88: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E53E8C: 4BF0143D  bl 0x82d552c8
	ctx.lr = 0x82E53E90;
	sub_82D552C8(ctx, base);
	// 82E53E90: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E53E94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53E98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53E9C: 409A001C  bne cr6, 0x82e53eb8
	if !ctx.cr[6].eq {
	pc = 0x82E53EB8; continue 'dispatch;
	}
	// 82E53EA0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53EA4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E53EA8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53EAC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53EB0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E53EB4: 4BF01415  bl 0x82d552c8
	ctx.lr = 0x82E53EB8;
	sub_82D552C8(ctx, base);
	// 82E53EB8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E53EBC: 4B41304D  bl 0x82266f08
	ctx.lr = 0x82E53EC0;
	sub_82266F08(ctx, base);
	// 82E53EC0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E53EC4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53EC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53ECC: 409A001C  bne cr6, 0x82e53ee8
	if !ctx.cr[6].eq {
	pc = 0x82E53EE8; continue 'dispatch;
	}
	// 82E53ED0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53ED4: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E53ED8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E53EDC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53EE0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E53EE4: 4BF013E5  bl 0x82d552c8
	ctx.lr = 0x82E53EE8;
	sub_82D552C8(ctx, base);
	// 82E53EE8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E53EEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E53EF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53EF4: 409A0028  bne cr6, 0x82e53f1c
	if !ctx.cr[6].eq {
	pc = 0x82E53F1C; continue 'dispatch;
	}
	// 82E53EF8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E53EFC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53F00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E53F04: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E53F08: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E53F0C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E53F10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53F14: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E53F18: 4BF013B1  bl 0x82d552c8
	ctx.lr = 0x82E53F1C;
	sub_82D552C8(ctx, base);
	// 82E53F1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E53F20: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82E53F24: 4BE554FC  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
	// 82E53F28: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E53F2C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E53F30: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82E53F34: 4BF03AB5  bl 0x82d579e8
	ctx.lr = 0x82E53F38;
	sub_82D579E8(ctx, base);
	// 82E53F38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53F3C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82E53F40: 388B72C0  addi r4, r11, 0x72c0
	ctx.r[4].s64 = ctx.r[11].s64 + 29376;
	// 82E53F44: 4BF03EAD  bl 0x82d57df0
	ctx.lr = 0x82E53F48;
	sub_82D57DF0(ctx, base);
	// 82E53F48: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E53F4C: 3CA0ABBA  lis r5, -0x5446
	ctx.r[5].s64 = -1413873664;
	// 82E53F50: 3900006C  li r8, 0x6c
	ctx.r[8].s64 = 108;
	// 82E53F54: 38C100F0  addi r6, r1, 0xf0
	ctx.r[6].s64 = ctx.r[1].s64 + 240;
	// 82E53F58: 60A59D6D  ori r5, r5, 0x9d6d
	ctx.r[5].u64 = ctx.r[5].u64 | 40301;
	// 82E53F5C: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82E53F60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53F64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53F68: 38EB7310  addi r7, r11, 0x7310
	ctx.r[7].s64 = ctx.r[11].s64 + 29456;
	// 82E53F6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53F70: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E53F74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53F78: 4E800421  bctrl
	ctx.lr = 0x82E53F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53F7C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 82E53F80: 4BF044B1  bl 0x82d58430
	ctx.lr = 0x82E53F84;
	sub_82D58430(ctx, base);
	// 82E53F84: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53F8C: 419A0034  beq cr6, 0x82e53fc0
	if ctx.cr[6].eq {
	pc = 0x82E53FC0; continue 'dispatch;
	}
	// 82E53F90: A17B0006  lhz r11, 6(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53F94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53F98: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53F9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53FA0: B17B0006  sth r11, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53FA4: 409A001C  bne cr6, 0x82e53fc0
	if !ctx.cr[6].eq {
	pc = 0x82E53FC0; continue 'dispatch;
	}
	// 82E53FA8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53FAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53FB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E53FB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53FBC: 4E800421  bctrl
	ctx.lr = 0x82E53FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53FC0: A1750004  lhz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53FC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E53FC8: 419A0034  beq cr6, 0x82e53ffc
	if ctx.cr[6].eq {
	pc = 0x82E53FFC; continue 'dispatch;
	}
	// 82E53FCC: A1750006  lhz r11, 6(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[21].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E53FD0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E53FD4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E53FD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53FDC: B1750006  sth r11, 6(r21)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[21].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E53FE0: 409A001C  bne cr6, 0x82e53ffc
	if !ctx.cr[6].eq {
	pc = 0x82E53FFC; continue 'dispatch;
	}
	// 82E53FE4: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53FE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53FEC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82E53FF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53FF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53FF8: 4E800421  bctrl
	ctx.lr = 0x82E53FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53FFC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E54000: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E54004: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54008: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5400C: 409A001C  bne cr6, 0x82e54028
	if !ctx.cr[6].eq {
	pc = 0x82E54028; continue 'dispatch;
	}
	// 82E54010: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E54014: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E54018: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E5401C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54020: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E54024: 4BF012A5  bl 0x82d552c8
	ctx.lr = 0x82E54028;
	sub_82D552C8(ctx, base);
	// 82E54028: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E5402C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54030: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E54034: 409A001C  bne cr6, 0x82e54050
	if !ctx.cr[6].eq {
	pc = 0x82E54050; continue 'dispatch;
	}
	// 82E54038: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5403C: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E54040: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E54044: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54048: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5404C: 4BF0127D  bl 0x82d552c8
	ctx.lr = 0x82E54050;
	sub_82D552C8(ctx, base);
	// 82E54050: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E54054: 4B412EB5  bl 0x82266f08
	ctx.lr = 0x82E54058;
	sub_82266F08(ctx, base);
	// 82E54058: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E5405C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54060: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E54064: 409A001C  bne cr6, 0x82e54080
	if !ctx.cr[6].eq {
	pc = 0x82E54080; continue 'dispatch;
	}
	// 82E54068: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E5406C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E54070: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E54074: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54078: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E5407C: 4BF0124D  bl 0x82d552c8
	ctx.lr = 0x82E54080;
	sub_82D552C8(ctx, base);
	// 82E54080: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E54084: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54088: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5408C: 409AFE90  bne cr6, 0x82e53f1c
	if !ctx.cr[6].eq {
	pc = 0x82E53F1C; continue 'dispatch;
	}
	// 82E54090: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E54094: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E54098: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5409C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E540A0: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E540A4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E540A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E540AC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E540B0: 4BF01219  bl 0x82d552c8
	ctx.lr = 0x82E540B4;
	sub_82D552C8(ctx, base);
	// 82E540B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E540B8: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82E540BC: 4BE55364  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E540C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E540C0 size=412
    let mut pc: u32 = 0x82E540C0;
    'dispatch: loop {
        match pc {
            0x82E540C0 => {
    //   block [0x82E540C0..0x82E5425C)
	// 82E540C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E540C4: 4BE55349  bl 0x82ca940c
	ctx.lr = 0x82E540C8;
	sub_82CA93D0(ctx, base);
	// 82E540C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E540CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E540D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E540D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E540D8: 396B6004  addi r11, r11, 0x6004
	ctx.r[11].s64 = ctx.r[11].s64 + 24580;
	// 82E540DC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E540E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E540E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E540E8: 40990068  ble cr6, 0x82e54150
	if !ctx.cr[6].gt {
	pc = 0x82E54150; continue 'dispatch;
	}
	// 82E540EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E540F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E540F4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E540F8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E540FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E54100: 419A003C  beq cr6, 0x82e5413c
	if ctx.cr[6].eq {
	pc = 0x82E5413C; continue 'dispatch;
	}
	// 82E54104: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E54108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5410C: 419A0030  beq cr6, 0x82e5413c
	if ctx.cr[6].eq {
	pc = 0x82E5413C; continue 'dispatch;
	}
	// 82E54110: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E54114: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E54118: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E5411C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54120: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E54124: 409A0018  bne cr6, 0x82e5413c
	if !ctx.cr[6].eq {
	pc = 0x82E5413C; continue 'dispatch;
	}
	// 82E54128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5412C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E54130: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54138: 4E800421  bctrl
	ctx.lr = 0x82E5413C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5413C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54140: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E54144: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82E54148: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E5414C: 4198FFA4  blt cr6, 0x82e540f0
	if ctx.cr[6].lt {
	pc = 0x82E540F0; continue 'dispatch;
	}
	// 82E54150: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E54154: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E54158: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5415C: 4099005C  ble cr6, 0x82e541b8
	if !ctx.cr[6].gt {
	pc = 0x82E541B8; continue 'dispatch;
	}
	// 82E54160: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E54164: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E54168: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E5416C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E54170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54174: 419A0030  beq cr6, 0x82e541a4
	if ctx.cr[6].eq {
	pc = 0x82E541A4; continue 'dispatch;
	}
	// 82E54178: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E5417C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E54180: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E54184: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54188: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E5418C: 409A0018  bne cr6, 0x82e541a4
	if !ctx.cr[6].eq {
	pc = 0x82E541A4; continue 'dispatch;
	}
	// 82E54190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54194: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E54198: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5419C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E541A0: 4E800421  bctrl
	ctx.lr = 0x82E541A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E541A4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E541A8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E541AC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E541B0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E541B4: 4198FFB0  blt cr6, 0x82e54164
	if ctx.cr[6].lt {
	pc = 0x82E54164; continue 'dispatch;
	}
	// 82E541B8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E541BC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E541C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E541C4: 409A0020  bne cr6, 0x82e541e4
	if !ctx.cr[6].eq {
	pc = 0x82E541E4; continue 'dispatch;
	}
	// 82E541C8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E541CC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E541D0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E541D4: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E541D8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E541DC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E541E0: 4BF010E9  bl 0x82d552c8
	ctx.lr = 0x82E541E4;
	sub_82D552C8(ctx, base);
	// 82E541E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E541E8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E541EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E541F0: 409A0020  bne cr6, 0x82e54210
	if !ctx.cr[6].eq {
	pc = 0x82E54210; continue 'dispatch;
	}
	// 82E541F4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E541F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E541FC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54200: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E54204: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E54208: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5420C: 4BF010BD  bl 0x82d552c8
	ctx.lr = 0x82E54210;
	sub_82D552C8(ctx, base);
	// 82E54210: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54214: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54218: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E5421C: 409A002C  bne cr6, 0x82e54248
	if !ctx.cr[6].eq {
	pc = 0x82E54248; continue 'dispatch;
	}
	// 82E54220: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54224: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E54228: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E5422C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E54230: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54234: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E54238: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E5423C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E54240: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E54244: 4BF01085  bl 0x82d552c8
	ctx.lr = 0x82E54248;
	sub_82D552C8(ctx, base);
	// 82E54248: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E5424C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E54250: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E54258: 4BE55204  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54260 size=236
    let mut pc: u32 = 0x82E54260;
    'dispatch: loop {
        match pc {
            0x82E54260 => {
    //   block [0x82E54260..0x82E5434C)
	// 82E54260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5426C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E54270: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E54274: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54278: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E5427C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54280: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E54284: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E54288: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82E5428C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E54290: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E54294: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E54298: 4BFFF1A9  bl 0x82e53440
	ctx.lr = 0x82E5429C;
	sub_82E53440(ctx, base);
	// 82E5429C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E542A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E542A4: 419A0060  beq cr6, 0x82e54304
	if ctx.cr[6].eq {
	pc = 0x82E54304; continue 'dispatch;
	}
	// 82E542A8: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 82E542AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E542B0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E542B4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E542B8: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82E542BC: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E542C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E542C4: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E542C8: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E542CC: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E542D0: EDA007F2  fmuls f13, f0, f31
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E542D4: EC0007B2  fmuls f0, f0, f30
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E542D8: 4099002C  ble cr6, 0x82e54304
	if !ctx.cr[6].gt {
	pc = 0x82E54304; continue 'dispatch;
	}
	// 82E542DC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E542E0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E542E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E542E8: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E542EC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E542F0: D1A9000C  stfs f13, 0xc(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E542F4: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E542F8: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E542FC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E54300: 4198FFE0  blt cr6, 0x82e542e0
	if ctx.cr[6].lt {
	pc = 0x82E542E0; continue 'dispatch;
	}
	// 82E54304: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E54308: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5430C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E54310: 409A0020  bne cr6, 0x82e54330
	if !ctx.cr[6].eq {
	pc = 0x82E54330; continue 'dispatch;
	}
	// 82E54314: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54318: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E5431C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E54320: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E54324: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E54328: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E5432C: 4BF00F9D  bl 0x82d552c8
	ctx.lr = 0x82E54330;
	sub_82D552C8(ctx, base);
	// 82E54330: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E54334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5433C: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E54340: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E54344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54350 size=52
    let mut pc: u32 = 0x82E54350;
    'dispatch: loop {
        match pc {
            0x82E54350 => {
    //   block [0x82E54350..0x82E54384)
	// 82E54350: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E54354: 419A03D4  beq cr6, 0x82e54728
	if ctx.cr[6].eq {
		sub_82E54728(ctx, base);
		return;
	}
	// 82E54358: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82E5435C: 419A03CC  beq cr6, 0x82e54728
	if ctx.cr[6].eq {
		sub_82E54728(ctx, base);
		return;
	}
	// 82E54360: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E54364: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54368: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5436C: 419A00E0  beq cr6, 0x82e5444c
	if ctx.cr[6].eq {
		sub_82E54444(ctx, base);
		return;
	}
	// 82E54370: 54AB07FE  clrlwi r11, r5, 0x1f
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00000001u64;
	// 82E54374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54378: 419A000C  beq cr6, 0x82e54384
	if ctx.cr[6].eq {
		sub_82E54384(ctx, base);
		return;
	}
	// 82E5437C: 81440034  lwz r10, 0x34(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E54380: 48000008  b 0x82e54388
	sub_82E54384(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54384 size=24
    let mut pc: u32 = 0x82E54384;
    'dispatch: loop {
        match pc {
            0x82E54384 => {
    //   block [0x82E54384..0x82E5439C)
	// 82E54384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E54388: 54AB07BC  rlwinm r11, r5, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5438C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54390: 419A000C  beq cr6, 0x82e5439c
	if ctx.cr[6].eq {
		sub_82E5439C(ctx, base);
		return;
	}
	// 82E54394: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E54398: 48000008  b 0x82e543a0
	sub_82E5439C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5439C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5439C size=28
    let mut pc: u32 = 0x82E5439C;
    'dispatch: loop {
        match pc {
            0x82E5439C => {
    //   block [0x82E5439C..0x82E543B8)
	// 82E5439C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E543A0: 54A9077A  rlwinm r9, r5, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E543A4: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E543A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E543AC: 419A000C  beq cr6, 0x82e543b8
	if ctx.cr[6].eq {
		sub_82E543B8(ctx, base);
		return;
	}
	// 82E543B0: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E543B4: 48000008  b 0x82e543bc
	sub_82E543B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E543B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E543B8 size=28
    let mut pc: u32 = 0x82E543B8;
    'dispatch: loop {
        match pc {
            0x82E543B8 => {
    //   block [0x82E543B8..0x82E543D4)
	// 82E543B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E543BC: 54A90738  rlwinm r9, r5, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E543C0: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E543C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E543C8: 419A000C  beq cr6, 0x82e543d4
	if ctx.cr[6].eq {
		sub_82E543D4(ctx, base);
		return;
	}
	// 82E543CC: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E543D0: 48000008  b 0x82e543d8
	sub_82E543D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E543D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E543D4 size=28
    let mut pc: u32 = 0x82E543D4;
    'dispatch: loop {
        match pc {
            0x82E543D4 => {
    //   block [0x82E543D4..0x82E543F0)
	// 82E543D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E543D8: 54A906F6  rlwinm r9, r5, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E543DC: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E543E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E543E4: 419A000C  beq cr6, 0x82e543f0
	if ctx.cr[6].eq {
		sub_82E543F0(ctx, base);
		return;
	}
	// 82E543E8: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E543EC: 48000008  b 0x82e543f4
	sub_82E543F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E543F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E543F0 size=28
    let mut pc: u32 = 0x82E543F0;
    'dispatch: loop {
        match pc {
            0x82E543F0 => {
    //   block [0x82E543F0..0x82E5440C)
	// 82E543F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E543F4: 54A906B4  rlwinm r9, r5, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E543F8: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E543FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54400: 419A000C  beq cr6, 0x82e5440c
	if ctx.cr[6].eq {
		sub_82E5440C(ctx, base);
		return;
	}
	// 82E54404: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E54408: 48000008  b 0x82e54410
	sub_82E5440C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5440C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5440C size=28
    let mut pc: u32 = 0x82E5440C;
    'dispatch: loop {
        match pc {
            0x82E5440C => {
    //   block [0x82E5440C..0x82E54428)
	// 82E5440C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54410: 54A90672  rlwinm r9, r5, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54414: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54418: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E5441C: 419A000C  beq cr6, 0x82e54428
	if ctx.cr[6].eq {
		sub_82E54428(ctx, base);
		return;
	}
	// 82E54420: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E54424: 48000008  b 0x82e5442c
	sub_82E54428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54428 size=28
    let mut pc: u32 = 0x82E54428;
    'dispatch: loop {
        match pc {
            0x82E54428 => {
    //   block [0x82E54428..0x82E54444)
	// 82E54428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5442C: 54A90630  rlwinm r9, r5, 0, 0x18, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54430: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54434: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54438: 419A000C  beq cr6, 0x82e54444
	if ctx.cr[6].eq {
		sub_82E54444(ctx, base);
		return;
	}
	// 82E5443C: 81640050  lwz r11, 0x50(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E54440: 48000008  b 0x82e54448
	sub_82E54444(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54444(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54444 size=40
    let mut pc: u32 = 0x82E54444;
    'dispatch: loop {
        match pc {
            0x82E54444 => {
    //   block [0x82E54444..0x82E5446C)
	// 82E54444: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54448: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E5444C: 54AA042E  rlwinm r10, r5, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54450: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E54454: 419A00E4  beq cr6, 0x82e54538
	if ctx.cr[6].eq {
		sub_82E54530(ctx, base);
		return;
	}
	// 82E54458: 54AA05EE  rlwinm r10, r5, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5445C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E54460: 419A000C  beq cr6, 0x82e5446c
	if ctx.cr[6].eq {
		sub_82E5446C(ctx, base);
		return;
	}
	// 82E54464: 81440054  lwz r10, 0x54(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E54468: 48000008  b 0x82e54470
	sub_82E5446C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5446C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5446C size=28
    let mut pc: u32 = 0x82E5446C;
    'dispatch: loop {
        match pc {
            0x82E5446C => {
    //   block [0x82E5446C..0x82E54488)
	// 82E5446C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E54470: 54A905AC  rlwinm r9, r5, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54474: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E54478: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E5447C: 419A000C  beq cr6, 0x82e54488
	if ctx.cr[6].eq {
		sub_82E54488(ctx, base);
		return;
	}
	// 82E54480: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E54484: 48000008  b 0x82e5448c
	sub_82E54488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54488 size=28
    let mut pc: u32 = 0x82E54488;
    'dispatch: loop {
        match pc {
            0x82E54488 => {
    //   block [0x82E54488..0x82E544A4)
	// 82E54488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5448C: 54A9056A  rlwinm r9, r5, 0, 0x15, 0x15
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54490: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54494: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54498: 419A000C  beq cr6, 0x82e544a4
	if ctx.cr[6].eq {
		sub_82E544A4(ctx, base);
		return;
	}
	// 82E5449C: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E544A0: 48000008  b 0x82e544a8
	sub_82E544A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E544A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E544A4 size=28
    let mut pc: u32 = 0x82E544A4;
    'dispatch: loop {
        match pc {
            0x82E544A4 => {
    //   block [0x82E544A4..0x82E544C0)
	// 82E544A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E544A8: 54A90528  rlwinm r9, r5, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E544AC: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E544B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E544B4: 419A000C  beq cr6, 0x82e544c0
	if ctx.cr[6].eq {
		sub_82E544C0(ctx, base);
		return;
	}
	// 82E544B8: 81640060  lwz r11, 0x60(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E544BC: 48000008  b 0x82e544c4
	sub_82E544C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E544C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E544C0 size=28
    let mut pc: u32 = 0x82E544C0;
    'dispatch: loop {
        match pc {
            0x82E544C0 => {
    //   block [0x82E544C0..0x82E544DC)
	// 82E544C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E544C4: 54A904E6  rlwinm r9, r5, 0, 0x13, 0x13
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E544C8: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E544CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E544D0: 419A000C  beq cr6, 0x82e544dc
	if ctx.cr[6].eq {
		sub_82E544DC(ctx, base);
		return;
	}
	// 82E544D4: 81640064  lwz r11, 0x64(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E544D8: 48000008  b 0x82e544e0
	sub_82E544DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E544DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E544DC size=28
    let mut pc: u32 = 0x82E544DC;
    'dispatch: loop {
        match pc {
            0x82E544DC => {
    //   block [0x82E544DC..0x82E544F8)
	// 82E544DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E544E0: 54A904A4  rlwinm r9, r5, 0, 0x12, 0x12
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E544E4: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E544E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E544EC: 419A000C  beq cr6, 0x82e544f8
	if ctx.cr[6].eq {
		sub_82E544F8(ctx, base);
		return;
	}
	// 82E544F0: 81640068  lwz r11, 0x68(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E544F4: 48000008  b 0x82e544fc
	sub_82E544F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E544F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E544F8 size=28
    let mut pc: u32 = 0x82E544F8;
    'dispatch: loop {
        match pc {
            0x82E544F8 => {
    //   block [0x82E544F8..0x82E54514)
	// 82E544F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E544FC: 54A90462  rlwinm r9, r5, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54500: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54504: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54508: 419A000C  beq cr6, 0x82e54514
	if ctx.cr[6].eq {
		sub_82E54514(ctx, base);
		return;
	}
	// 82E5450C: 8164006C  lwz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E54510: 48000008  b 0x82e54518
	sub_82E54514(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54514(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54514 size=28
    let mut pc: u32 = 0x82E54514;
    'dispatch: loop {
        match pc {
            0x82E54514 => {
    //   block [0x82E54514..0x82E54530)
	// 82E54514: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54518: 54A90420  rlwinm r9, r5, 0, 0x10, 0x10
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5451C: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54520: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54524: 419A000C  beq cr6, 0x82e54530
	if ctx.cr[6].eq {
		sub_82E54530(ctx, base);
		return;
	}
	// 82E54528: 81640070  lwz r11, 0x70(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E5452C: 48000008  b 0x82e54534
	sub_82E54530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54530 size=40
    let mut pc: u32 = 0x82E54530;
    'dispatch: loop {
        match pc {
            0x82E54530 => {
    //   block [0x82E54530..0x82E54558)
	// 82E54530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54534: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54538: 54AA021E  rlwinm r10, r5, 0, 8, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5453C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E54540: 419A00E4  beq cr6, 0x82e54624
	if ctx.cr[6].eq {
		sub_82E5461C(ctx, base);
		return;
	}
	// 82E54544: 54AA03DE  rlwinm r10, r5, 0, 0xf, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54548: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5454C: 419A000C  beq cr6, 0x82e54558
	if ctx.cr[6].eq {
		sub_82E54558(ctx, base);
		return;
	}
	// 82E54550: 81440074  lwz r10, 0x74(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E54554: 48000008  b 0x82e5455c
	sub_82E54558(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54558 size=28
    let mut pc: u32 = 0x82E54558;
    'dispatch: loop {
        match pc {
            0x82E54558 => {
    //   block [0x82E54558..0x82E54574)
	// 82E54558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E5455C: 54A9039C  rlwinm r9, r5, 0, 0xe, 0xe
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54560: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E54564: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54568: 419A000C  beq cr6, 0x82e54574
	if ctx.cr[6].eq {
		sub_82E54574(ctx, base);
		return;
	}
	// 82E5456C: 81640078  lwz r11, 0x78(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E54570: 48000008  b 0x82e54578
	sub_82E54574(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54574(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54574 size=28
    let mut pc: u32 = 0x82E54574;
    'dispatch: loop {
        match pc {
            0x82E54574 => {
    //   block [0x82E54574..0x82E54590)
	// 82E54574: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54578: 54A9035A  rlwinm r9, r5, 0, 0xd, 0xd
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5457C: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54580: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54584: 419A000C  beq cr6, 0x82e54590
	if ctx.cr[6].eq {
		sub_82E54590(ctx, base);
		return;
	}
	// 82E54588: 8164007C  lwz r11, 0x7c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E5458C: 48000008  b 0x82e54594
	sub_82E54590(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54590 size=28
    let mut pc: u32 = 0x82E54590;
    'dispatch: loop {
        match pc {
            0x82E54590 => {
    //   block [0x82E54590..0x82E545AC)
	// 82E54590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54594: 54A90318  rlwinm r9, r5, 0, 0xc, 0xc
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54598: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E5459C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E545A0: 419A000C  beq cr6, 0x82e545ac
	if ctx.cr[6].eq {
		sub_82E545AC(ctx, base);
		return;
	}
	// 82E545A4: 81640080  lwz r11, 0x80(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E545A8: 48000008  b 0x82e545b0
	sub_82E545AC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E545AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E545AC size=28
    let mut pc: u32 = 0x82E545AC;
    'dispatch: loop {
        match pc {
            0x82E545AC => {
    //   block [0x82E545AC..0x82E545C8)
	// 82E545AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E545B0: 54A902D6  rlwinm r9, r5, 0, 0xb, 0xb
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E545B4: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E545B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E545BC: 419A000C  beq cr6, 0x82e545c8
	if ctx.cr[6].eq {
		sub_82E545C8(ctx, base);
		return;
	}
	// 82E545C0: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E545C4: 48000008  b 0x82e545cc
	sub_82E545C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E545C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E545C8 size=28
    let mut pc: u32 = 0x82E545C8;
    'dispatch: loop {
        match pc {
            0x82E545C8 => {
    //   block [0x82E545C8..0x82E545E4)
	// 82E545C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E545CC: 54A90294  rlwinm r9, r5, 0, 0xa, 0xa
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E545D0: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E545D4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E545D8: 419A000C  beq cr6, 0x82e545e4
	if ctx.cr[6].eq {
		sub_82E545E4(ctx, base);
		return;
	}
	// 82E545DC: 81640088  lwz r11, 0x88(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E545E0: 48000008  b 0x82e545e8
	sub_82E545E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E545E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E545E4 size=28
    let mut pc: u32 = 0x82E545E4;
    'dispatch: loop {
        match pc {
            0x82E545E4 => {
    //   block [0x82E545E4..0x82E54600)
	// 82E545E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E545E8: 54A90252  rlwinm r9, r5, 0, 9, 9
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E545EC: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E545F0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E545F4: 419A000C  beq cr6, 0x82e54600
	if ctx.cr[6].eq {
		sub_82E54600(ctx, base);
		return;
	}
	// 82E545F8: 8164008C  lwz r11, 0x8c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E545FC: 48000008  b 0x82e54604
	sub_82E54600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54600 size=28
    let mut pc: u32 = 0x82E54600;
    'dispatch: loop {
        match pc {
            0x82E54600 => {
    //   block [0x82E54600..0x82E5461C)
	// 82E54600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54604: 54A90210  rlwinm r9, r5, 0, 8, 8
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54608: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E5460C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54610: 419A000C  beq cr6, 0x82e5461c
	if ctx.cr[6].eq {
		sub_82E5461C(ctx, base);
		return;
	}
	// 82E54614: 81640090  lwz r11, 0x90(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E54618: 48000008  b 0x82e54620
	sub_82E5461C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5461C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5461C size=40
    let mut pc: u32 = 0x82E5461C;
    'dispatch: loop {
        match pc {
            0x82E5461C => {
    //   block [0x82E5461C..0x82E54644)
	// 82E5461C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54620: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54624: 54AA000E  rlwinm r10, r5, 0, 0, 7
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E5462C: 419A00E4  beq cr6, 0x82e54710
	if ctx.cr[6].eq {
		sub_82E54708(ctx, base);
		return;
	}
	// 82E54630: 54AA01CE  rlwinm r10, r5, 0, 7, 7
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54634: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E54638: 419A000C  beq cr6, 0x82e54644
	if ctx.cr[6].eq {
		sub_82E54644(ctx, base);
		return;
	}
	// 82E5463C: 81440094  lwz r10, 0x94(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E54640: 48000008  b 0x82e54648
	sub_82E54644(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54644 size=28
    let mut pc: u32 = 0x82E54644;
    'dispatch: loop {
        match pc {
            0x82E54644 => {
    //   block [0x82E54644..0x82E54660)
	// 82E54644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E54648: 54A9018C  rlwinm r9, r5, 0, 6, 6
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E5464C: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E54650: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54654: 419A000C  beq cr6, 0x82e54660
	if ctx.cr[6].eq {
		sub_82E54660(ctx, base);
		return;
	}
	// 82E54658: 81640098  lwz r11, 0x98(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E5465C: 48000008  b 0x82e54664
	sub_82E54660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54660 size=28
    let mut pc: u32 = 0x82E54660;
    'dispatch: loop {
        match pc {
            0x82E54660 => {
    //   block [0x82E54660..0x82E5467C)
	// 82E54660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54664: 54A9014A  rlwinm r9, r5, 0, 5, 5
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54668: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E5466C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E54670: 419A000C  beq cr6, 0x82e5467c
	if ctx.cr[6].eq {
		sub_82E5467C(ctx, base);
		return;
	}
	// 82E54674: 8164009C  lwz r11, 0x9c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E54678: 48000008  b 0x82e54680
	sub_82E5467C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5467C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5467C size=28
    let mut pc: u32 = 0x82E5467C;
    'dispatch: loop {
        match pc {
            0x82E5467C => {
    //   block [0x82E5467C..0x82E54698)
	// 82E5467C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54680: 54A90108  rlwinm r9, r5, 0, 4, 4
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E54684: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54688: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E5468C: 419A000C  beq cr6, 0x82e54698
	if ctx.cr[6].eq {
		sub_82E54698(ctx, base);
		return;
	}
	// 82E54690: 816400A0  lwz r11, 0xa0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E54694: 48000008  b 0x82e5469c
	sub_82E54698(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54698 size=28
    let mut pc: u32 = 0x82E54698;
    'dispatch: loop {
        match pc {
            0x82E54698 => {
    //   block [0x82E54698..0x82E546B4)
	// 82E54698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5469C: 54A900C6  rlwinm r9, r5, 0, 3, 3
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E546A0: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E546A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E546A8: 419A000C  beq cr6, 0x82e546b4
	if ctx.cr[6].eq {
		sub_82E546B4(ctx, base);
		return;
	}
	// 82E546AC: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E546B0: 48000008  b 0x82e546b8
	sub_82E546B4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E546B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E546B4 size=28
    let mut pc: u32 = 0x82E546B4;
    'dispatch: loop {
        match pc {
            0x82E546B4 => {
    //   block [0x82E546B4..0x82E546D0)
	// 82E546B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E546B8: 54A90084  rlwinm r9, r5, 0, 2, 2
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E546BC: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E546C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E546C4: 419A000C  beq cr6, 0x82e546d0
	if ctx.cr[6].eq {
		sub_82E546D0(ctx, base);
		return;
	}
	// 82E546C8: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E546CC: 48000008  b 0x82e546d4
	sub_82E546D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E546D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E546D0 size=28
    let mut pc: u32 = 0x82E546D0;
    'dispatch: loop {
        match pc {
            0x82E546D0 => {
    //   block [0x82E546D0..0x82E546EC)
	// 82E546D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E546D4: 54A90042  rlwinm r9, r5, 0, 1, 1
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E546D8: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E546DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E546E0: 419A000C  beq cr6, 0x82e546ec
	if ctx.cr[6].eq {
		sub_82E546EC(ctx, base);
		return;
	}
	// 82E546E4: 816400AC  lwz r11, 0xac(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E546E8: 48000008  b 0x82e546f0
	sub_82E546EC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E546EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E546EC size=28
    let mut pc: u32 = 0x82E546EC;
    'dispatch: loop {
        match pc {
            0x82E546EC => {
    //   block [0x82E546EC..0x82E54708)
	// 82E546EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E546F0: 54A90000  rlwinm r9, r5, 0, 0, 0
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E546F4: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E546F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E546FC: 419A000C  beq cr6, 0x82e54708
	if ctx.cr[6].eq {
		sub_82E54708(ctx, base);
		return;
	}
	// 82E54700: 816400B0  lwz r11, 0xb0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E54704: 48000008  b 0x82e5470c
	sub_82E54708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54708 size=32
    let mut pc: u32 = 0x82E54708;
    'dispatch: loop {
        match pc {
            0x82E54708 => {
    //   block [0x82E54708..0x82E54728)
	// 82E54708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5470C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E54710: 7D6B3038  and r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[6].u64;
	// 82E54714: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E54718: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E5471C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82E54720: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E54724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54728 size=12
    let mut pc: u32 = 0x82E54728;
    'dispatch: loop {
        match pc {
            0x82E54728 => {
    //   block [0x82E54728..0x82E54734)
	// 82E54728: 89640030  lbz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5472C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E54730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54738 size=16
    let mut pc: u32 = 0x82E54738;
    'dispatch: loop {
        match pc {
            0x82E54738 => {
    //   block [0x82E54738..0x82E54748)
	// 82E54738: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5473C: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 82E54740: 80A5001C  lwz r5, 0x1c(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E54744: 4BFFFC0C  b 0x82e54350
	sub_82E54350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54748 size=104
    let mut pc: u32 = 0x82E54748;
    'dispatch: loop {
        match pc {
            0x82E54748 => {
    //   block [0x82E54748..0x82E547B0)
	// 82E54748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5474C: 4BE54CC1  bl 0x82ca940c
	ctx.lr = 0x82E54750;
	sub_82CA93D0(ctx, base);
	// 82E54750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54754: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54758: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E5475C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E54760: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E54764: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82E54768: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82E5476C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54774: 4E800421  bctrl
	ctx.lr = 0x82E54778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54778: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5477C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E54780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54784: 419A0014  beq cr6, 0x82e54798
	if ctx.cr[6].eq {
	pc = 0x82E54798; continue 'dispatch;
	}
	// 82E54788: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E5478C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54794: 409AFFF4  bne cr6, 0x82e54788
	if !ctx.cr[6].eq {
	pc = 0x82E54788; continue 'dispatch;
	}
	// 82E54798: 389DFFF4  addi r4, r29, -0xc
	ctx.r[4].s64 = ctx.r[29].s64 + -12;
	// 82E5479C: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E547A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E547A4: 4BFFFBAD  bl 0x82e54350
	ctx.lr = 0x82E547A8;
	sub_82E54350(ctx, base);
	// 82E547A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E547AC: 4BE54CB0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E547B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E547B0 size=104
    let mut pc: u32 = 0x82E547B0;
    'dispatch: loop {
        match pc {
            0x82E547B0 => {
    //   block [0x82E547B0..0x82E54818)
	// 82E547B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E547B4: 4BE54C55  bl 0x82ca9408
	ctx.lr = 0x82E547B8;
	sub_82CA93D0(ctx, base);
	// 82E547B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E547BC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E547C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E547C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E547C8: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82E547CC: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82E547D0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82E547D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E547D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E547DC: 4E800421  bctrl
	ctx.lr = 0x82E547E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E547E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E547E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E547E8: 808100D4  lwz r4, 0xd4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82E547EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E547F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E547F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E547F8: 4E800421  bctrl
	ctx.lr = 0x82E547FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E547FC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E54800: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E54804: 389DFFF4  addi r4, r29, -0xc
	ctx.r[4].s64 = ctx.r[29].s64 + -12;
	// 82E54808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5480C: 4BFFFB45  bl 0x82e54350
	ctx.lr = 0x82E54810;
	sub_82E54350(ctx, base);
	// 82E54810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54814: 4BE54C44  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54818 size=76
    let mut pc: u32 = 0x82E54818;
    'dispatch: loop {
        match pc {
            0x82E54818 => {
    //   block [0x82E54818..0x82E54864)
	// 82E54818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5481C: 4BE54BF1  bl 0x82ca940c
	ctx.lr = 0x82E54820;
	sub_82CA93D0(ctx, base);
	// 82E54820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54824: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5482C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54830: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E54834: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82E54838: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82E5483C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54844: 4E800421  bctrl
	ctx.lr = 0x82E54848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54848: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E5484C: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 82E54850: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E54854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54858: 4BFFFAF9  bl 0x82e54350
	ctx.lr = 0x82E5485C;
	sub_82E54350(ctx, base);
	// 82E5485C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E54860: 4BE54BFC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54868 size=16
    let mut pc: u32 = 0x82E54868;
    'dispatch: loop {
        match pc {
            0x82E54868 => {
    //   block [0x82E54868..0x82E54878)
	// 82E54868: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E5486C: 3884FFEC  addi r4, r4, -0x14
	ctx.r[4].s64 = ctx.r[4].s64 + -20;
	// 82E54870: 80A50024  lwz r5, 0x24(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E54874: 4BFFFADC  b 0x82e54350
	sub_82E54350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54878 size=28
    let mut pc: u32 = 0x82E54878;
    'dispatch: loop {
        match pc {
            0x82E54878 => {
    //   block [0x82E54878..0x82E54894)
	// 82E54878: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E5487C: 409A0018  bne cr6, 0x82e54894
	if !ctx.cr[6].eq {
		sub_82E54894(ctx, base);
		return;
	}
	// 82E54880: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E54884: 409A0010  bne cr6, 0x82e54894
	if !ctx.cr[6].eq {
		sub_82E54894(ctx, base);
		return;
	}
	// 82E54888: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5488C: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82E54890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54894 size=84
    let mut pc: u32 = 0x82E54894;
    'dispatch: loop {
        match pc {
            0x82E54894 => {
    //   block [0x82E54894..0x82E548E8)
	// 82E54894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E54898: 39430034  addi r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 + 52;
	// 82E5489C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E548A0: 7D0B4830  slw r11, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82E548A4: 7D672038  and r7, r11, r4
	ctx.r[7].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E548A8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E548AC: 419A0010  beq cr6, 0x82e548bc
	if ctx.cr[6].eq {
	pc = 0x82E548BC; continue 'dispatch;
	}
	// 82E548B0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E548B4: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 82E548B8: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E548BC: 7D6B2838  and r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[5].u64;
	// 82E548C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E548C4: 419A0010  beq cr6, 0x82e548d4
	if ctx.cr[6].eq {
	pc = 0x82E548D4; continue 'dispatch;
	}
	// 82E548C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E548CC: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82E548D0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E548D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E548D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E548DC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 82E548E0: 4198FFC0  blt cr6, 0x82e548a0
	if ctx.cr[6].lt {
	pc = 0x82E548A0; continue 'dispatch;
	}
	// 82E548E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E548E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E548E8 size=28
    let mut pc: u32 = 0x82E548E8;
    'dispatch: loop {
        match pc {
            0x82E548E8 => {
    //   block [0x82E548E8..0x82E54904)
	// 82E548E8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E548EC: 409A0018  bne cr6, 0x82e54904
	if !ctx.cr[6].eq {
		sub_82E54904(ctx, base);
		return;
	}
	// 82E548F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E548F4: 409A0010  bne cr6, 0x82e54904
	if !ctx.cr[6].eq {
		sub_82E54904(ctx, base);
		return;
	}
	// 82E548F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E548FC: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82E54900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54904 size=84
    let mut pc: u32 = 0x82E54904;
    'dispatch: loop {
        match pc {
            0x82E54904 => {
    //   block [0x82E54904..0x82E54958)
	// 82E54904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E54908: 39430034  addi r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 + 52;
	// 82E5490C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E54910: 7D0B4830  slw r11, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82E54914: 7D672038  and r7, r11, r4
	ctx.r[7].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82E54918: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E5491C: 419A0010  beq cr6, 0x82e5492c
	if ctx.cr[6].eq {
	pc = 0x82E5492C; continue 'dispatch;
	}
	// 82E54920: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54924: 7CE72878  andc r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[5].u64;
	// 82E54928: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E5492C: 7D6B2838  and r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[5].u64;
	// 82E54930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54934: 419A0010  beq cr6, 0x82e54944
	if ctx.cr[6].eq {
	pc = 0x82E54944; continue 'dispatch;
	}
	// 82E54938: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5493C: 7D6B2078  andc r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[4].u64;
	// 82E54940: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54944: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E54948: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5494C: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 82E54950: 4198FFC0  blt cr6, 0x82e54910
	if ctx.cr[6].lt {
	pc = 0x82E54910; continue 'dispatch;
	}
	// 82E54954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54958 size=172
    let mut pc: u32 = 0x82E54958;
    'dispatch: loop {
        match pc {
            0x82E54958 => {
    //   block [0x82E54958..0x82E54A04)
	// 82E54958: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82E5495C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E54960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54964: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E54968: 394A85E8  addi r10, r10, -0x7a18
	ctx.r[10].s64 = ctx.r[10].s64 + -31256;
	// 82E5496C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E54970: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E54974: 39298610  addi r9, r9, -0x79f0
	ctx.r[9].s64 = ctx.r[9].s64 + -31216;
	// 82E54978: B3C30006  sth r30, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[30].u16 ) };
	// 82E5497C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E54980: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E54984: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E54988: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E5498C: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82E54990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54994: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E54998: 3FE08203  lis r31, -0x7dfd
	ctx.r[31].s64 = -2113732608;
	// 82E5499C: 39088624  addi r8, r8, -0x79dc
	ctx.r[8].s64 = ctx.r[8].s64 + -31196;
	// 82E549A0: 38E78604  addi r7, r7, -0x79fc
	ctx.r[7].s64 = ctx.r[7].s64 + -31228;
	// 82E549A4: 394A60CC  addi r10, r10, 0x60cc
	ctx.r[10].s64 = ctx.r[10].s64 + 24780;
	// 82E549A8: 38C660F8  addi r6, r6, 0x60f8
	ctx.r[6].s64 = ctx.r[6].s64 + 24824;
	// 82E549AC: 38A560EC  addi r5, r5, 0x60ec
	ctx.r[5].s64 = ctx.r[5].s64 + 24812;
	// 82E549B0: 388460D8  addi r4, r4, 0x60d8
	ctx.r[4].s64 = ctx.r[4].s64 + 24792;
	// 82E549B4: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E549B8: 3BFF60C0  addi r31, r31, 0x60c0
	ctx.r[31].s64 = ctx.r[31].s64 + 24768;
	// 82E549BC: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82E549C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E549C4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E549C8: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E549CC: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82E549D0: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E549D4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82E549D8: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82E549DC: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82E549E0: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E549E4: 9BC30030  stb r30, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[30].u8 ) };
	// 82E549E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E549EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E549F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E549F4: 4200FFF8  bdnz 0x82e549ec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E549EC; continue 'dispatch;
	}
	// 82E549F8: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E549FC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82E54A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54A08 size=188
    let mut pc: u32 = 0x82E54A08;
    'dispatch: loop {
        match pc {
            0x82E54A08 => {
    //   block [0x82E54A08..0x82E54AC4)
	// 82E54A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54A18: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E54A1C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E54A20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54A24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E54A28: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E54A30: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E54A34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54A38: 4BF36351  bl 0x82d8ad88
	ctx.lr = 0x82E54A3C;
	sub_82D8AD88(ctx, base);
	// 82E54A3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54A40: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82E54A44: 394A6194  addi r10, r10, 0x6194
	ctx.r[10].s64 = ctx.r[10].s64 + 24980;
	// 82E54A48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E54A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54A50: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54AC8 size=156
    let mut pc: u32 = 0x82E54AC8;
    'dispatch: loop {
        match pc {
            0x82E54AC8 => {
    //   block [0x82E54AC8..0x82E54B64)
	// 82E54AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54ADC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54AE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54AE4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E54AE8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E54AEC: 419A000C  beq cr6, 0x82e54af8
	if ctx.cr[6].eq {
	pc = 0x82E54AF8; continue 'dispatch;
	}
	// 82E54AF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E54AF4: 48000058  b 0x82e54b4c
	pc = 0x82E54B4C; continue 'dispatch;
	// 82E54AF8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E54AFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54B00: 409AFFF0  bne cr6, 0x82e54af0
	if !ctx.cr[6].eq {
	pc = 0x82E54AF0; continue 'dispatch;
	}
	// 82E54B04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54B08: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E54B0C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E54B10: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82E54B14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E54B18: 4BF00731  bl 0x82d55248
	ctx.lr = 0x82E54B1C;
	sub_82D55248(ctx, base);
	// 82E54B1C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82E54B20: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82E54B24: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E54B28: C05F0034  lfs f2, 0x34(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E54B2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54B30: C03F0030  lfs f1, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E54B34: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54B38: 4BFFFED1  bl 0x82e54a08
	ctx.lr = 0x82E54B3C;
	sub_82E54A08(ctx, base);
	// 82E54B3C: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E54B40: 99630038  stb r11, 0x38(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 82E54B44: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54B48: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E54B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E54B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54B58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E54B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54B68 size=248
    let mut pc: u32 = 0x82E54B68;
    'dispatch: loop {
        match pc {
            0x82E54B68 => {
    //   block [0x82E54B68..0x82E54C60)
	// 82E54B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54B6C: 4BE54899  bl 0x82ca9404
	ctx.lr = 0x82E54B70;
	sub_82CA93D0(ctx, base);
	// 82E54B70: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E54B74: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54B78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E54B7C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E54B80: 897E0038  lbz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E54B84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54B88: 419A00CC  beq cr6, 0x82e54c54
	if ctx.cr[6].eq {
	pc = 0x82E54C54; continue 'dispatch;
	}
	// 82E54B8C: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E54B90: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E54B94: 3BBF00E0  addi r29, r31, 0xe0
	ctx.r[29].s64 = ctx.r[31].s64 + 224;
	// 82E54B98: 38BF01B0  addi r5, r31, 0x1b0
	ctx.r[5].s64 = ctx.r[31].s64 + 432;
	// 82E54B9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E54BA0: 4BF018D9  bl 0x82d56478
	ctx.lr = 0x82E54BA4;
	sub_82D56478(ctx, base);
	// 82E54BA4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E54BA8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82E54BAC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E54BB0: 3B9F00D0  addi r28, r31, 0xd0
	ctx.r[28].s64 = ctx.r[31].s64 + 208;
	// 82E54BB4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54C60 size=188
    let mut pc: u32 = 0x82E54C60;
    'dispatch: loop {
        match pc {
            0x82E54C60 => {
    //   block [0x82E54C60..0x82E54D1C)
	// 82E54C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54C68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54C6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54C74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82E54C78: 117F038C  vspltisw v11, -1
	for i in 0..4 {
		ctx.v[11].u32[i] = 4294967295;
	}
	// 82E54C7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E54C80: 3BC30030  addi r30, r3, 0x30
	ctx.r[30].s64 = ctx.r[3].s64 + 48;
	// 82E54C84: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 82E54C88: C00BBE14  lfs f0, -0x41ec(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E54C8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E54D20 size=8
    let mut pc: u32 = 0x82E54D20;
    'dispatch: loop {
        match pc {
            0x82E54D20 => {
    //   block [0x82E54D20..0x82E54D28)
	// 82E54D20: D0230048  stfs f1, 0x48(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82E54D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54D28 size=52
    let mut pc: u32 = 0x82E54D28;
    'dispatch: loop {
        match pc {
            0x82E54D28 => {
    //   block [0x82E54D28..0x82E54D5C)
	// 82E54D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54D3C: 4BF35FAD  bl 0x82d8ace8
	ctx.lr = 0x82E54D40;
	sub_82D8ACE8(ctx, base);
	// 82E54D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54D44: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E54D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E54D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54D54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54D60 size=172
    let mut pc: u32 = 0x82E54D60;
    'dispatch: loop {
        match pc {
            0x82E54D60 => {
    //   block [0x82E54D60..0x82E54E0C)
	// 82E54D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54D70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E54D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54D78: 4BF36011  bl 0x82d8ad88
	ctx.lr = 0x82E54D7C;
	sub_82D8AD88(ctx, base);
	// 82E54D7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E54D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E54D84: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82E54D88: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82E54D8C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82E54D90: C00A0BFC  lfs f0, 0xbfc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E54D94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54D98: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E54D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54DA0: 392A646C  addi r9, r10, 0x646c
	ctx.r[9].s64 = ctx.r[10].s64 + 25708;
	// 82E54DA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E54DA8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E54DAC: C1AA0B24  lfs f13, 0xb24(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2852 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E54DB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54DB4: D1BF0044  stfs f13, 0x44(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82E54DB8: C18A7388  lfs f12, 0x7388(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29576 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E54DBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54DC0: D19F0048  stfs f12, 0x48(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82E54DC4: C16A7384  lfs f11, 0x7384(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29572 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E54DC8: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82E54DCC: D17F004C  stfs f11, 0x4c(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82E54DD0: 911F0054  stw r8, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82E54DD4: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82E54DD8: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54E10 size=704
    let mut pc: u32 = 0x82E54E10;
    'dispatch: loop {
        match pc {
            0x82E54E10 => {
    //   block [0x82E54E10..0x82E550D0)
	// 82E54E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54E14: 4BE545E5  bl 0x82ca93f8
	ctx.lr = 0x82E54E18;
	sub_82CA93D0(ctx, base);
	// 82E54E18: DBA1FFA0  stfd f29, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[29].u64 ) };
	// 82E54E1C: DBC1FFA8  stfd f30, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 82E54E20: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82E54E24: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54E28: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 82E54E2C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54E30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E54E34: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E54E38: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82E54E3C: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82E54E40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E54E44: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E54E48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54E4C: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82E54E50: 4BF35F39  bl 0x82d8ad88
	ctx.lr = 0x82E54E54;
	sub_82D8AD88(ctx, base);
	// 82E54E54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E54E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E54E5C: 396B646C  addi r11, r11, 0x646c
	ctx.r[11].s64 = ctx.r[11].s64 + 25708;
	// 82E54E60: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82E54E64: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82E54E68: 3BDF0054  addi r30, r31, 0x54
	ctx.r[30].s64 = ctx.r[31].s64 + 84;
	// 82E54E6C: C00A7388  lfs f0, 0x7388(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29576 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E54E70: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E54E74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54E78: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E550D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E550D0 size=792
    let mut pc: u32 = 0x82E550D0;
    'dispatch: loop {
        match pc {
            0x82E550D0 => {
    //   block [0x82E550D0..0x82E553E8)
	// 82E550D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E550D4: 4BE54331  bl 0x82ca9404
	ctx.lr = 0x82E550D8;
	sub_82CA93D0(ctx, base);
	// 82E550D8: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E553E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E553E8 size=344
    let mut pc: u32 = 0x82E553E8;
    'dispatch: loop {
        match pc {
            0x82E553E8 => {
    //   block [0x82E553E8..0x82E55540)
	// 82E553E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E553EC: 4BE54015  bl 0x82ca9400
	ctx.lr = 0x82E553F0;
	sub_82CA93D0(ctx, base);
	// 82E553F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E553F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E553F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E553FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55400: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E55404: 419A0010  beq cr6, 0x82e55414
	if ctx.cr[6].eq {
	pc = 0x82E55414; continue 'dispatch;
	}
	// 82E55408: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5540C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E55410: 4BE54040  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82E55414: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55418: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5541C: 409AFFEC  bne cr6, 0x82e55408
	if !ctx.cr[6].eq {
	pc = 0x82E55408; continue 'dispatch;
	}
	// 82E55420: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55424: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82E55428: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E5542C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E55430: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E55434: 4BEFFE15  bl 0x82d55248
	ctx.lr = 0x82E55438;
	sub_82D55248(ctx, base);
	// 82E55438: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E5543C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E55440: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55444: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55448: 4BFFF919  bl 0x82e54d60
	ctx.lr = 0x82E5544C;
	sub_82E54D60(ctx, base);
	// 82E5544C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82E55450: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55454: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82E55458: 3BBE0054  addi r29, r30, 0x54
	ctx.r[29].s64 = ctx.r[30].s64 + 84;
	// 82E5545C: 3B9F0054  addi r28, r31, 0x54
	ctx.r[28].s64 = ctx.r[31].s64 + 84;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55540 size=148
    let mut pc: u32 = 0x82E55540;
    'dispatch: loop {
        match pc {
            0x82E55540 => {
    //   block [0x82E55540..0x82E555D4)
	// 82E55540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55544: 4BE53EC9  bl 0x82ca940c
	ctx.lr = 0x82E55548;
	sub_82CA93D0(ctx, base);
	// 82E55548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5554C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55550: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E55554: 396B6500  addi r11, r11, 0x6500
	ctx.r[11].s64 = ctx.r[11].s64 + 25856;
	// 82E55558: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5555C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E55560: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55564: 40990024  ble cr6, 0x82e55588
	if !ctx.cr[6].gt {
	pc = 0x82E55588; continue 'dispatch;
	}
	// 82E55568: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E5556C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55570: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E55574: 4BF2AE6D  bl 0x82d803e0
	ctx.lr = 0x82E55578;
	sub_82D803E0(ctx, base);
	// 82E55578: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E5557C: 3BBD0050  addi r29, r29, 0x50
	ctx.r[29].s64 = ctx.r[29].s64 + 80;
	// 82E55580: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E55584: 409AFFE8  bne cr6, 0x82e5556c
	if !ctx.cr[6].eq {
	pc = 0x82E5556C; continue 'dispatch;
	}
	// 82E55588: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5558C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E55590: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55594: 409A002C  bne cr6, 0x82e555c0
	if !ctx.cr[6].eq {
	pc = 0x82E555C0; continue 'dispatch;
	}
	// 82E55598: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5559C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E555A0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E555A4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E555A8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E555AC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E555B0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E555B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E555B8: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E555BC: 4BEFFD0D  bl 0x82d552c8
	ctx.lr = 0x82E555C0;
	sub_82D552C8(ctx, base);
	// 82E555C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E555C4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E555C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E555CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E555D0: 4BE53E8C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E555D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E555D8 size=12
    let mut pc: u32 = 0x82E555D8;
    'dispatch: loop {
        match pc {
            0x82E555D8 => {
    //   block [0x82E555D8..0x82E555E4)
	// 82E555D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E555DC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82E555E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


