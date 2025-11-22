pub fn sub_82632700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632700 size=136
	// 82632700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263270C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632714: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82632718: 4BBECB41  bl 0x8221f258
	ctx.lr = 0x8263271C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263271C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632720: 419A004C  beq cr6, 0x8263276c
	if ctx.cr[6].eq {
	pc = 0x8263276C; continue 'dispatch;
	}
	// 82632724: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82632728: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263272C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632730: 392ADEB0  addi r9, r10, -0x2150
	ctx.r[9].s64 = ctx.r[10].s64 + -8528;
	// 82632734: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632738: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263273C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632740: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82632744: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82632748: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8263274C: 99030012  stb r8, 0x12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[8].u8 ) };
	// 82632750: 99630013  stb r11, 0x13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(19 as u32), ctx.r[11].u8 ) };
	// 82632754: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82632758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263275C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632768: 4E800020  blr
	return;
	// 8263276C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263277C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632780: 4E800020  blr
	return;
}

pub fn sub_82632788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632788 size=136
	// 82632788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263278C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263279C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 826327A0: 4BBECAB9  bl 0x8221f258
	ctx.lr = 0x826327A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826327A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826327A8: 419A004C  beq cr6, 0x826327f4
	if ctx.cr[6].eq {
	pc = 0x826327F4; continue 'dispatch;
	}
	// 826327AC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826327B0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826327B4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 826327B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826327BC: 390A5C88  addi r8, r10, 0x5c88
	ctx.r[8].s64 = ctx.r[10].s64 + 23688;
	// 826327C0: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 826327C4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826327C8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826327CC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826327D0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826327D4: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 826327D8: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 826327DC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826327E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826327E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826327E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826327EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826327F0: 4E800020  blr
	return;
	// 826327F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826327F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826327FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632808: 4E800020  blr
	return;
}

pub fn sub_82632810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632810 size=160
	// 82632810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263281C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82632828: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8263282C: 4BBECA2D  bl 0x8221f258
	ctx.lr = 0x82632830;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632834: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632838: 419A0058  beq cr6, 0x82632890
	if ctx.cr[6].eq {
	pc = 0x82632890; continue 'dispatch;
	}
	// 8263283C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82632840: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82632844: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632848: 392A3268  addi r9, r10, 0x3268
	ctx.r[9].s64 = ctx.r[10].s64 + 12904;
	// 8263284C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632850: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632854: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8263285C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632860: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632864: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632868: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8263286C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82632870: 991F0024  stb r8, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u8 ) };
	// 82632874: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82632878: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8263287C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82632880: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82632884: 4BD5AC35  bl 0x8238d4b8
	ctx.lr = 0x82632888;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82632888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263288C: 48000008  b 0x82632894
	pc = 0x82632894; continue 'dispatch;
	// 82632890: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263289C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826328A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826328A8: 4E800020  blr
	return;
}

pub fn sub_826328B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826328B0 size=88
	// 826328B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826328B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826328B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826328BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826328C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826328C4: 38600810  li r3, 0x810
	ctx.r[3].s64 = 2064;
	// 826328C8: 4BBEC991  bl 0x8221f258
	ctx.lr = 0x826328CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826328CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826328D0: 419A0020  beq cr6, 0x826328f0
	if ctx.cr[6].eq {
	pc = 0x826328F0; continue 'dispatch;
	}
	// 826328D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826328D8: 481D5721  bl 0x82807ff8
	ctx.lr = 0x826328DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82807FF8);
	// 826328DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826328E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826328EC: 4E800020  blr
	return;
	// 826328F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826328F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826328F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632904: 4E800020  blr
	return;
}

pub fn sub_82632908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632908 size=88
	// 82632908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263291C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82632920: 4BBEC939  bl 0x8221f258
	ctx.lr = 0x82632924;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632928: 419A0020  beq cr6, 0x82632948
	if ctx.cr[6].eq {
	pc = 0x82632948; continue 'dispatch;
	}
	// 8263292C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632930: 48000071  bl 0x826329a0
	ctx.lr = 0x82632934;
	crate::recompiler::externs::call(&mut ctx, base, 0x826329A0);
	// 82632934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632944: 4E800020  blr
	return;
	// 82632948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263294C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263295C: 4E800020  blr
	return;
}

pub fn sub_82632960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632960 size=64
	// 82632960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263296C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632970: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632978: 388B8680  addi r4, r11, -0x7980
	ctx.r[4].s64 = ctx.r[11].s64 + -31104;
	// 8263297C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632980: 4BBFA551  bl 0x8222ced0
	ctx.lr = 0x82632984;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82632984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263298C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632998: 4E800020  blr
	return;
}

pub fn sub_826329A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826329A0 size=20
	// 826329A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826329A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826329A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826329AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826329B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82632A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A80 size=160
	// 82632A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A84: 48676989  bl 0x82ca940c
	ctx.lr = 0x82632A88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82632A88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632A90: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82632A94: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82632A98: 4BB85081  bl 0x821b7b18
	ctx.lr = 0x82632A9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 82632A9C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82632AA0: 4BDA9D89  bl 0x823dc828
	ctx.lr = 0x82632AA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x823DC828);
	// 82632AA4: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82632AA8: 480FA891  bl 0x8272d338
	ctx.lr = 0x82632AAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8272D338);
	// 82632AAC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82632AB0: 4BBE9289  bl 0x8221bd38
	ctx.lr = 0x82632AB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82632AB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632AB8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82632ABC: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82632AC0: 480FA879  bl 0x8272d338
	ctx.lr = 0x82632AC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8272D338);
	// 82632AC4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82632AC8: 4BBE9271  bl 0x8221bd38
	ctx.lr = 0x82632ACC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82632ACC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82632AD0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82632AD4: 4BC67D7D  bl 0x8229a850
	ctx.lr = 0x82632AD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229A850);
	// 82632AD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632ADC: 419A0008  beq cr6, 0x82632ae4
	if ctx.cr[6].eq {
	pc = 0x82632AE4; continue 'dispatch;
	}
	// 82632AE0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82632AE4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632AE8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82632AEC: 392B1848  addi r9, r11, 0x1848
	ctx.r[9].s64 = ctx.r[11].s64 + 6216;
	// 82632AF0: 390A2850  addi r8, r10, 0x2850
	ctx.r[8].s64 = ctx.r[10].s64 + 10320;
	// 82632AF4: 57A707FE  clrlwi r7, r29, 0x1f
	ctx.r[7].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82632AF8: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632AFC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82632B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632B04: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82632B08: 419A000C  beq cr6, 0x82632b14
	if ctx.cr[6].eq {
	pc = 0x82632B14; continue 'dispatch;
	}
	// 82632B0C: 4BBE922D  bl 0x8221bd38
	ctx.lr = 0x82632B10;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82632B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632B14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632B18: 48676944  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82632B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B20 size=192
	// 82632B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B24: 486768E9  bl 0x82ca940c
	ctx.lr = 0x82632B28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82632B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82632B30: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82632B34: 4BBEC725  bl 0x8221f258
	ctx.lr = 0x82632B38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632B3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632B40: 419A0090  beq cr6, 0x82632bd0
	if ctx.cr[6].eq {
	pc = 0x82632BD0; continue 'dispatch;
	}
	// 82632B44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82632B48: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82632B4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632B50: 394B3428  addi r10, r11, 0x3428
	ctx.r[10].s64 = ctx.r[11].s64 + 13352;
	// 82632B54: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82632B58: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82632B5C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82632B60: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82632B64: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82632B68: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82632B6C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82632B70: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82632B74: 9BDF0025  stb r30, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82632B78: 4BBEC6E1  bl 0x8221f258
	ctx.lr = 0x82632B7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632B80: 419A0008  beq cr6, 0x82632b88
	if ctx.cr[6].eq {
	pc = 0x82632B88; continue 'dispatch;
	}
	// 82632B84: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632B88: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632B8C: 41820008  beq 0x82632b94
	if ctx.cr[0].eq {
	pc = 0x82632B94; continue 'dispatch;
	}
	// 82632B90: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632B94: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82632B98: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82632B9C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82632BA0: 4BBEC6B9  bl 0x8221f258
	ctx.lr = 0x82632BA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632BA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632BA8: 419A0008  beq cr6, 0x82632bb0
	if ctx.cr[6].eq {
	pc = 0x82632BB0; continue 'dispatch;
	}
	// 82632BAC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632BB0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632BB4: 41820008  beq 0x82632bbc
	if ctx.cr[0].eq {
	pc = 0x82632BBC; continue 'dispatch;
	}
	// 82632BB8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632BBC: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82632BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632BC4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82632BC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632BCC: 48676890  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82632BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632BD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632BD8: 48676884  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82632BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632BE0 size=64
	// 82632BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632BF0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632BF8: 388B872C  addi r4, r11, -0x78d4
	ctx.r[4].s64 = ctx.r[11].s64 + -30932;
	// 82632BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632C00: 4BBFA2D1  bl 0x8222ced0
	ctx.lr = 0x82632C04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82632C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632C18: 4E800020  blr
	return;
}

pub fn sub_82632C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C20 size=112
	// 82632C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82632C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632C38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82632C3C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 82632C40: 48007A49  bl 0x8263a688
	ctx.lr = 0x82632C44;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 82632C44: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82632C48: 48007A41  bl 0x8263a688
	ctx.lr = 0x82632C4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 82632C4C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82632C50: 4BCE51F9  bl 0x82317e48
	ctx.lr = 0x82632C54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82317E48);
	// 82632C54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632C58: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82632C5C: 392B2850  addi r9, r11, 0x2850
	ctx.r[9].s64 = ctx.r[11].s64 + 10320;
	// 82632C60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82632C64: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C6C: 419A000C  beq cr6, 0x82632c78
	if ctx.cr[6].eq {
	pc = 0x82632C78; continue 'dispatch;
	}
	// 82632C70: 4BBE90C9  bl 0x8221bd38
	ctx.lr = 0x82632C74;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82632C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82632C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632C8C: 4E800020  blr
	return;
}

pub fn sub_82632C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C90 size=16
	// 82632C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82632D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632D90 size=88
	// 82632D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632DA4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82632DA8: 4BBEC4B1  bl 0x8221f258
	ctx.lr = 0x82632DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632DB0: 419A0020  beq cr6, 0x82632dd0
	if ctx.cr[6].eq {
	pc = 0x82632DD0; continue 'dispatch;
	}
	// 82632DB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632DB8: 48264FA1  bl 0x82897d58
	ctx.lr = 0x82632DBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82897D58);
	// 82632DBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632DCC: 4E800020  blr
	return;
	// 82632DD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632DE4: 4E800020  blr
	return;
}

pub fn sub_82632DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632DE8 size=88
	// 82632DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632DFC: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82632E00: 4BBEC459  bl 0x8221f258
	ctx.lr = 0x82632E04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632E08: 419A0020  beq cr6, 0x82632e28
	if ctx.cr[6].eq {
	pc = 0x82632E28; continue 'dispatch;
	}
	// 82632E0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632E10: 48000071  bl 0x82632e80
	ctx.lr = 0x82632E14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82632E80);
	// 82632E14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E24: 4E800020  blr
	return;
	// 82632E28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E3C: 4E800020  blr
	return;
}

pub fn sub_82632E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632E40 size=64
	// 82632E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632E50: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82632E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632E58: 388B9554  addi r4, r11, -0x6aac
	ctx.r[4].s64 = ctx.r[11].s64 + -27308;
	// 82632E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632E60: 4BBFA071  bl 0x8222ced0
	ctx.lr = 0x82632E64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82632E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E78: 4E800020  blr
	return;
}

pub fn sub_82632E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632E80 size=12
	// 82632E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E84: 48676589  bl 0x82ca940c
	ctx.lr = 0x82632E88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82632E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82632F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632F80 size=136
	// 82632F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632F94: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82632F98: 4BBEC2C1  bl 0x8221f258
	ctx.lr = 0x82632F9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82632F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632FA0: 419A0050  beq cr6, 0x82632ff0
	if ctx.cr[6].eq {
	pc = 0x82632FF0; continue 'dispatch;
	}
	// 82632FA4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82632FA8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82632FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632FB0: 392A1A18  addi r9, r10, 0x1a18
	ctx.r[9].s64 = ctx.r[10].s64 + 6680;
	// 82632FB4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632FB8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632FBC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632FC0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632FC4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632FC8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82632FCC: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82632FD0: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 82632FD4: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 82632FD8: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 82632FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632FEC: 4E800020  blr
	return;
	// 82632FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633004: 4E800020  blr
	return;
}

pub fn sub_82633008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633008 size=88
	// 82633008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263300C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263301C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82633020: 4BBEC239  bl 0x8221f258
	ctx.lr = 0x82633024;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633028: 419A0020  beq cr6, 0x82633048
	if ctx.cr[6].eq {
	pc = 0x82633048; continue 'dispatch;
	}
	// 8263302C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633030: 480FB239  bl 0x8272e268
	ctx.lr = 0x82633034;
	crate::recompiler::externs::call(&mut ctx, base, 0x8272E268);
	// 82633034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263303C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633044: 4E800020  blr
	return;
	// 82633048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263304C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263305C: 4E800020  blr
	return;
}

pub fn sub_82633060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633060 size=152
	// 82633060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263306C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633074: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82633078: 4BBEC1E1  bl 0x8221f258
	ctx.lr = 0x8263307C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263307C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633080: 419A005C  beq cr6, 0x826330dc
	if ctx.cr[6].eq {
	pc = 0x826330DC; continue 'dispatch;
	}
	// 82633084: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633088: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263308C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633090: 392A6808  addi r9, r10, 0x6808
	ctx.r[9].s64 = ctx.r[10].s64 + 26632;
	// 82633094: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633098: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263309C: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826330A0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826330A4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826330A8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 826330AC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826330B0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 826330B4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 826330B8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826330BC: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 826330C0: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 826330C4: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 826330C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826330CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826330D8: 4E800020  blr
	return;
	// 826330DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826330E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826330E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826330F0: 4E800020  blr
	return;
}

pub fn sub_826330F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826330F8 size=120
	// 826330F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826330FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263310C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633110: 4BBEC149  bl 0x8221f258
	ctx.lr = 0x82633114;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633118: 419A0040  beq cr6, 0x82633158
	if ctx.cr[6].eq {
	pc = 0x82633158; continue 'dispatch;
	}
	// 8263311C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633128: 392A69D0  addi r9, r10, 0x69d0
	ctx.r[9].s64 = ctx.r[10].s64 + 27088;
	// 8263312C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633130: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633134: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633138: 891F0090  lbz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8263313C: 61070001  ori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 | 1;
	// 82633140: 98FF0090  stb r7, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82633144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633154: 4E800020  blr
	return;
	// 82633158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263315C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263316C: 4E800020  blr
	return;
}

pub fn sub_82633170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633170 size=152
	// 82633170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263317C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633184: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82633188: 4BBEC0D1  bl 0x8221f258
	ctx.lr = 0x8263318C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263318C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633190: 419A0060  beq cr6, 0x826331f0
	if ctx.cr[6].eq {
	pc = 0x826331F0; continue 'dispatch;
	}
	// 82633194: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633198: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263319C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826331A0: 392A37B8  addi r9, r10, 0x37b8
	ctx.r[9].s64 = ctx.r[10].s64 + 14264;
	// 826331A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826331A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826331AC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826331B0: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826331B4: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826331B8: 9903000E  stb r8, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[8].u8 ) };
	// 826331BC: 9963000F  stb r11, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 826331C0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826331C4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826331C8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826331CC: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826331D0: 88C70090  lbz r6, 0x90(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(144 as u32) ) } as u64;
	// 826331D4: 60C50002  ori r5, r6, 2
	ctx.r[5].u64 = ctx.r[6].u64 | 2;
	// 826331D8: 98A70090  stb r5, 0x90(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(144 as u32), ctx.r[5].u8 ) };
	// 826331DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826331E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826331E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826331EC: 4E800020  blr
	return;
	// 826331F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826331F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826331F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633204: 4E800020  blr
	return;
}

pub fn sub_82633208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633208 size=88
	// 82633208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263320C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263321C: 386003E0  li r3, 0x3e0
	ctx.r[3].s64 = 992;
	// 82633220: 4BBEC039  bl 0x8221f258
	ctx.lr = 0x82633224;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633224: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633228: 419A0020  beq cr6, 0x82633248
	if ctx.cr[6].eq {
	pc = 0x82633248; continue 'dispatch;
	}
	// 8263322C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633230: 48150789  bl 0x827839b8
	ctx.lr = 0x82633234;
	crate::recompiler::externs::call(&mut ctx, base, 0x827839B8);
	// 82633234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633244: 4E800020  blr
	return;
	// 82633248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263324C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263325C: 4E800020  blr
	return;
}

pub fn sub_82633260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633260 size=104
	// 82633260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263326C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633274: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82633278: 4BBEBFE1  bl 0x8221f258
	ctx.lr = 0x8263327C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263327C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633280: 419A0030  beq cr6, 0x826332b0
	if ctx.cr[6].eq {
	pc = 0x826332B0; continue 'dispatch;
	}
	// 82633284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633288: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263328C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633290: 392A4680  addi r9, r10, 0x4680
	ctx.r[9].s64 = ctx.r[10].s64 + 18048;
	// 82633294: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633298: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263329C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826332AC: 4E800020  blr
	return;
	// 826332B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826332B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826332C4: 4E800020  blr
	return;
}

pub fn sub_826332C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826332C8 size=88
	// 826332C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826332CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826332D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826332D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826332D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826332DC: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 826332E0: 4BBEBF79  bl 0x8221f258
	ctx.lr = 0x826332E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826332E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826332E8: 419A0020  beq cr6, 0x82633308
	if ctx.cr[6].eq {
	pc = 0x82633308; continue 'dispatch;
	}
	// 826332EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826332F0: 480E1121  bl 0x82714410
	ctx.lr = 0x826332F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82714410);
	// 826332F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633304: 4E800020  blr
	return;
	// 82633308: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263330C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633318: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263331C: 4E800020  blr
	return;
}

pub fn sub_82633320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633320 size=16
	// 82633320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263332C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826333C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826333C0 size=136
	// 826333C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826333C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826333C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826333CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826333D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826333D4: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 826333D8: 4BBEBE81  bl 0x8221f258
	ctx.lr = 0x826333DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826333DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826333E0: 419A0050  beq cr6, 0x82633430
	if ctx.cr[6].eq {
	pc = 0x82633430; continue 'dispatch;
	}
	// 826333E4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826333E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826333EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826333F0: 392A2930  addi r9, r10, 0x2930
	ctx.r[9].s64 = ctx.r[10].s64 + 10544;
	// 826333F4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826333F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826333FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633400: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633404: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633408: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263340C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82633410: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82633414: 9963002D  stb r11, 0x2d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(45 as u32), ctx.r[11].u8 ) };
	// 82633418: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 8263341C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263342C: 4E800020  blr
	return;
	// 82633430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263343C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633444: 4E800020  blr
	return;
}

pub fn sub_82633448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633448 size=112
	// 82633448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263345C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633460: 4BBEBDF9  bl 0x8221f258
	ctx.lr = 0x82633464;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633464: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633468: 419A0034  beq cr6, 0x8263349c
	if ctx.cr[6].eq {
	pc = 0x8263349C; continue 'dispatch;
	}
	// 8263346C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633470: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633478: 392A5BC8  addi r9, r10, 0x5bc8
	ctx.r[9].s64 = ctx.r[10].s64 + 23496;
	// 8263347C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633480: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633484: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263348C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633498: 4E800020  blr
	return;
	// 8263349C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826334A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826334A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826334B0: 4E800020  blr
	return;
}

pub fn sub_826334B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826334B8 size=88
	// 826334B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826334BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826334C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826334C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826334C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826334CC: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 826334D0: 4BBEBD89  bl 0x8221f258
	ctx.lr = 0x826334D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826334D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826334D8: 419A0020  beq cr6, 0x826334f8
	if ctx.cr[6].eq {
	pc = 0x826334F8; continue 'dispatch;
	}
	// 826334DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826334E0: 4820ACE1  bl 0x8283e1c0
	ctx.lr = 0x826334E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8283E1C0);
	// 826334E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826334E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826334F4: 4E800020  blr
	return;
	// 826334F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826334FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263350C: 4E800020  blr
	return;
}

pub fn sub_82633510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633510 size=168
	// 82633510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263351C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633524: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82633528: 4BBEBD31  bl 0x8221f258
	ctx.lr = 0x8263352C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263352C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633530: 419A006C  beq cr6, 0x8263359c
	if ctx.cr[6].eq {
	pc = 0x8263359C; continue 'dispatch;
	}
	// 82633534: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633538: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263353C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633540: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82633544: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82633548: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263354C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82633550: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633554: 38C859B8  addi r6, r8, 0x59b8
	ctx.r[6].s64 = ctx.r[8].s64 + 22968;
	// 82633558: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8263355C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82633560: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82633564: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82633568: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 8263356C: C009A4E4  lfs f0, -0x5b1c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-23324 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633570: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82633574: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82633578: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263357C: 90E30058  stw r7, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82633580: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633584: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82633588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633598: 4E800020  blr
	return;
	// 8263359C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826335A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826335A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826335A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826335AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826335B0: 4E800020  blr
	return;
}

pub fn sub_826335B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826335B8 size=200
	// 826335B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826335BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826335C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826335C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826335C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826335CC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826335D0: 4BBEBC89  bl 0x8221f258
	ctx.lr = 0x826335D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826335D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826335D8: 419A0090  beq cr6, 0x82633668
	if ctx.cr[6].eq {
	pc = 0x82633668; continue 'dispatch;
	}
	// 826335DC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826335E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826335E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 826335E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826335EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826335F0: 390A3228  addi r8, r10, 0x3228
	ctx.r[8].s64 = ctx.r[10].s64 + 12840;
	// 826335F4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 826335F8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826335FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82633600: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633604: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82633608: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8263360C: 38870B7C  addi r4, r7, 0xb7c
	ctx.r[4].s64 = ctx.r[7].s64 + 2940;
	// 82633610: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
}

pub fn sub_82633680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633680 size=112
	// 82633680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263368C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633694: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82633698: 4BBEBBC1  bl 0x8221f258
	ctx.lr = 0x8263369C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263369C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826336A0: 419A0034  beq cr6, 0x826336d4
	if ctx.cr[6].eq {
	pc = 0x826336D4; continue 'dispatch;
	}
	// 826336A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826336A8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826336AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826336B0: 392A45C0  addi r9, r10, 0x45c0
	ctx.r[9].s64 = ctx.r[10].s64 + 17856;
	// 826336B4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826336B8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826336BC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826336C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826336C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826336D0: 4E800020  blr
	return;
	// 826336D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826336D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826336DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826336E8: 4E800020  blr
	return;
}

pub fn sub_826336F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826336F0 size=16
	// 826336F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826336F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826336F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826336FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82633770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633770 size=184
	// 82633770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263377C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633784: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 82633788: 4BBEBAD1  bl 0x8221f258
	ctx.lr = 0x8263378C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263378C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633790: 419A0080  beq cr6, 0x82633810
	if ctx.cr[6].eq {
	pc = 0x82633810; continue 'dispatch;
	}
	// 82633794: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633798: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263379C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826337A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826337A4: 390AD1F8  addi r8, r10, -0x2e08
	ctx.r[8].s64 = ctx.r[10].s64 + -11784;
	// 826337A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826337AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826337B0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826337B4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826337B8: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826337BC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826337C0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826337C4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 826337C8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826337CC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 826337D0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826337D4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 826337D8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 826337DC: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 826337E0: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 826337E4: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 826337E8: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 826337EC: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 826337F0: 98E30050  stb r7, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 826337F4: 99630051  stb r11, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 826337F8: 99630052  stb r11, 0x52(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 826337FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263380C: 4E800020  blr
	return;
	// 82633810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263381C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633824: 4E800020  blr
	return;
}

pub fn sub_82633828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633828 size=184
	// 82633828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263383C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82633840: 4BBEBA19  bl 0x8221f258
	ctx.lr = 0x82633844;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633844: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633848: 419A007C  beq cr6, 0x826338c4
	if ctx.cr[6].eq {
	pc = 0x826338C4; continue 'dispatch;
	}
	// 8263384C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82633850: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633854: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633858: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263385C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82633860: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82633864: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633868: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8263386C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633870: 38C94540  addi r6, r9, 0x4540
	ctx.r[6].s64 = ctx.r[9].s64 + 17728;
	// 82633874: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633878: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263387C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633880: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82633884: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82633888: 98A3001C  stb r5, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u8 ) };
	// 8263388C: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
}

pub fn sub_826338E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826338E0 size=160
	// 826338E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826338E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826338E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826338EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826338F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826338F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826338F8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826338FC: 4BBEB95D  bl 0x8221f258
	ctx.lr = 0x82633900;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633904: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82633908: 419A0058  beq cr6, 0x82633960
	if ctx.cr[6].eq {
	pc = 0x82633960; continue 'dispatch;
	}
	// 8263390C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633910: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82633914: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263391C: 39094340  addi r8, r9, 0x4340
	ctx.r[8].s64 = ctx.r[9].s64 + 17216;
	// 82633920: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633924: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82633928: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263392C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633930: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82633934: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82633938: 997F0011  stb r11, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8263393C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633940: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82633944: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82633948: 48214669  bl 0x82847fb0
	ctx.lr = 0x8263394C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82847FB0);
	// 8263394C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633950: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82633954: 4BD59B65  bl 0x8238d4b8
	ctx.lr = 0x82633958;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82633958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263395C: 48000008  b 0x82633964
	pc = 0x82633964; continue 'dispatch;
	// 82633960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82633968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263396C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82633974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633978: 4E800020  blr
	return;
}

pub fn sub_82633980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633980 size=104
	// 82633980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263398C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633994: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82633998: 4BBEB8C1  bl 0x8221f258
	ctx.lr = 0x8263399C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263399C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826339A0: 419A0030  beq cr6, 0x826339d0
	if ctx.cr[6].eq {
	pc = 0x826339D0; continue 'dispatch;
	}
	// 826339A4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826339A8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826339AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826339B0: 392A6680  addi r9, r10, 0x6680
	ctx.r[9].s64 = ctx.r[10].s64 + 26240;
	// 826339B4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826339B8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826339BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826339C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826339CC: 4E800020  blr
	return;
	// 826339D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826339D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826339D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826339E4: 4E800020  blr
	return;
}

pub fn sub_826339E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826339E8 size=88
	// 826339E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826339EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826339F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826339F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826339F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826339FC: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82633A00: 4BBEB859  bl 0x8221f258
	ctx.lr = 0x82633A04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633A04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633A08: 419A0020  beq cr6, 0x82633a28
	if ctx.cr[6].eq {
	pc = 0x82633A28; continue 'dispatch;
	}
	// 82633A0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633A10: 4BE754D9  bl 0x824a8ee8
	ctx.lr = 0x82633A14;
	crate::recompiler::externs::call(&mut ctx, base, 0x824A8EE8);
	// 82633A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A24: 4E800020  blr
	return;
	// 82633A28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A3C: 4E800020  blr
	return;
}

pub fn sub_82633A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A40 size=88
	// 82633A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633A54: 38600310  li r3, 0x310
	ctx.r[3].s64 = 784;
	// 82633A58: 4BBEB801  bl 0x8221f258
	ctx.lr = 0x82633A5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633A5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633A60: 419A0020  beq cr6, 0x82633a80
	if ctx.cr[6].eq {
	pc = 0x82633A80; continue 'dispatch;
	}
	// 82633A64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633A68: 4BF091B9  bl 0x8253cc20
	ctx.lr = 0x82633A6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8253CC20);
	// 82633A6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A7C: 4E800020  blr
	return;
	// 82633A80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A94: 4E800020  blr
	return;
}

pub fn sub_82633A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A98 size=88
	// 82633A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633AAC: 386002E0  li r3, 0x2e0
	ctx.r[3].s64 = 736;
	// 82633AB0: 4BBEB7A9  bl 0x8221f258
	ctx.lr = 0x82633AB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633AB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633AB8: 419A0020  beq cr6, 0x82633ad8
	if ctx.cr[6].eq {
	pc = 0x82633AD8; continue 'dispatch;
	}
	// 82633ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633AC0: 4BFD5B89  bl 0x82609648
	ctx.lr = 0x82633AC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82609648);
	// 82633AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633AD4: 4E800020  blr
	return;
	// 82633AD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633AEC: 4E800020  blr
	return;
}

pub fn sub_82633AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633AF0 size=88
	// 82633AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B04: 386000B4  li r3, 0xb4
	ctx.r[3].s64 = 180;
	// 82633B08: 4BBEB751  bl 0x8221f258
	ctx.lr = 0x82633B0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633B0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633B10: 419A0020  beq cr6, 0x82633b30
	if ctx.cr[6].eq {
	pc = 0x82633B30; continue 'dispatch;
	}
	// 82633B14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633B18: 48000071  bl 0x82633b88
	ctx.lr = 0x82633B1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82633B88);
	// 82633B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B2C: 4E800020  blr
	return;
	// 82633B30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B44: 4E800020  blr
	return;
}

pub fn sub_82633B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B48 size=64
	// 82633B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82633B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82633B60: 388B9648  addi r4, r11, -0x69b8
	ctx.r[4].s64 = ctx.r[11].s64 + -27064;
	// 82633B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B68: 4BBF9369  bl 0x8222ced0
	ctx.lr = 0x82633B6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82633B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B80: 4E800020  blr
	return;
}

pub fn sub_82633B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633B88 size=336
	// 82633B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B8C: 48675881  bl 0x82ca940c
	ctx.lr = 0x82633B90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82633B90: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82633B94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82633BA0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633BA4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633BA8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82633BAC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82633BB0: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82633BB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82633BB8: 38CB04D4  addi r6, r11, 0x4d4
	ctx.r[6].s64 = ctx.r[11].s64 + 1236;
	// 82633BBC: 38AAE738  addi r5, r10, -0x18c8
	ctx.r[5].s64 = ctx.r[10].s64 + -6344;
	// 82633BC0: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82633BC4: 3868A244  addi r3, r8, -0x5dbc
	ctx.r[3].s64 = ctx.r[8].s64 + -23996;
	// 82633BC8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82633BCC: 38894A00  addi r4, r9, 0x4a00
	ctx.r[4].s64 = ctx.r[9].s64 + 18944;
	// 82633BD0: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82633BD4: 39670BAC  addi r11, r7, 0xbac
	ctx.r[11].s64 = ctx.r[7].s64 + 2988;
	// 82633BD8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82633BDC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82633BE0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82633BE4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633BE8: 4BBEB671  bl 0x8221f258
	ctx.lr = 0x82633BEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633BF0: 419A0008  beq cr6, 0x82633bf8
	if ctx.cr[6].eq {
	pc = 0x82633BF8; continue 'dispatch;
	}
	// 82633BF4: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633BF8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82633BFC: 41820008  beq 0x82633c04
	if ctx.cr[0].eq {
	pc = 0x82633C04; continue 'dispatch;
	}
	// 82633C00: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82633C08: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82633C0C: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82633C10: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82633C14: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82633C18: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82633C1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633C20: 392A0E0C  addi r9, r10, 0xe0c
	ctx.r[9].s64 = ctx.r[10].s64 + 3596;
	// 82633C24: C3EB9484  lfs f31, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82633C28: D3FF0024  stfs f31, 0x24(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82633C2C: D3FF0028  stfs f31, 0x28(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82633C30: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82633C34: D3FF0030  stfs f31, 0x30(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82633C38: D3FF0034  stfs f31, 0x34(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82633C3C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82633C40: D3FF003C  stfs f31, 0x3c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82633C44: D3FF0040  stfs f31, 0x40(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82633C48: D3FF0044  stfs f31, 0x44(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82633C4C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82633C50: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82633C54: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82633C58: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82633C5C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82633C60: D3FF0064  stfs f31, 0x64(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82633C64: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82633C68: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82633C6C: 4BBEB5ED  bl 0x8221f258
	ctx.lr = 0x82633C70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633C70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633C74: 419A0008  beq cr6, 0x82633c7c
	if ctx.cr[6].eq {
	pc = 0x82633C7C; continue 'dispatch;
	}
	// 82633C78: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C7C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82633C80: 41820008  beq 0x82633c88
	if ctx.cr[0].eq {
	pc = 0x82633C88; continue 'dispatch;
	}
	// 82633C84: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C88: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82633C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633C90: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82633C94: D3FF0078  stfs f31, 0x78(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82633C98: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82633C9C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82633CA0: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82633CA4: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82633CA8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82633CAC: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 82633CB0: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 82633CB4: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 82633CB8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82633CBC: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82633CC0: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82633CC4: 9BDF00B0  stb r30, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u8 ) };
	// 82633CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633CCC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82633CD0: 4867578C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82633CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633CD8 size=168
	// 82633CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633CDC: 48675731  bl 0x82ca940c
	ctx.lr = 0x82633CE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82633CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633CE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633CE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82633CEC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82633CF0: 4BCE6E69  bl 0x8231ab58
	ctx.lr = 0x82633CF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231AB58);
	// 82633CF4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82633CF8: 4BCE6E61  bl 0x8231ab58
	ctx.lr = 0x82633CFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8231AB58);
	// 82633CFC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82633D00: 483E1091  bl 0x82a14d90
	ctx.lr = 0x82633D04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A14D90);
	// 82633D04: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82633D08: 4BBE8031  bl 0x8221bd38
	ctx.lr = 0x82633D0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82633D0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82633D10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82633D14: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82633D18: 4BC66B39  bl 0x8229a850
	ctx.lr = 0x82633D1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229A850);
	// 82633D1C: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82633D20: 4BB83DF9  bl 0x821b7b18
	ctx.lr = 0x82633D24;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 82633D24: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82633D28: 480069C9  bl 0x8263a6f0
	ctx.lr = 0x82633D2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A6F0);
	// 82633D2C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82633D30: 4BBE8009  bl 0x8221bd38
	ctx.lr = 0x82633D34;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82633D34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82633D38: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82633D3C: 419A0008  beq cr6, 0x82633d44
	if ctx.cr[6].eq {
	pc = 0x82633D44; continue 'dispatch;
	}
	// 82633D40: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82633D44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82633D48: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82633D4C: 392B04D4  addi r9, r11, 0x4d4
	ctx.r[9].s64 = ctx.r[11].s64 + 1236;
	// 82633D50: 390A2850  addi r8, r10, 0x2850
	ctx.r[8].s64 = ctx.r[10].s64 + 10320;
	// 82633D54: 57A707FE  clrlwi r7, r29, 0x1f
	ctx.r[7].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82633D58: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633D5C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633D64: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82633D68: 419A000C  beq cr6, 0x82633d74
	if ctx.cr[6].eq {
	pc = 0x82633D74; continue 'dispatch;
	}
	// 82633D6C: 4BBE7FCD  bl 0x8221bd38
	ctx.lr = 0x82633D70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82633D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82633D78: 486756E4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82633D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633D80 size=200
	// 82633D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633D94: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82633D98: 4BBEB4C1  bl 0x8221f258
	ctx.lr = 0x82633D9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633DA0: 419A0090  beq cr6, 0x82633e30
	if ctx.cr[6].eq {
	pc = 0x82633E30; continue 'dispatch;
	}
	// 82633DA4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633DA8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633DAC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82633DB0: 390ACBBC  addi r8, r10, -0x3444
	ctx.r[8].s64 = ctx.r[10].s64 + -13380;
	// 82633DB4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82633DB8: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 82633DBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633DC0: C18ACBBC  lfs f12, -0x3444(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13380 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82633DC4: 38A904D4  addi r5, r9, 0x4d4
	ctx.r[5].s64 = ctx.r[9].s64 + 1236;
	// 82633DC8: C0080A0C  lfs f0, 0xa0c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633DCC: 388732E8  addi r4, r7, 0x32e8
	ctx.r[4].s64 = ctx.r[7].s64 + 13032;
	// 82633DD0: C1A8F248  lfs f13, -0xdb8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-3512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82633DD4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82633DD8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82633DDC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633DE0: 39264300  addi r9, r6, 0x4300
	ctx.r[9].s64 = ctx.r[6].s64 + 17152;
	// 82633DE4: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82633DE8: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82633DEC: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82633DF0: 38C80E0C  addi r6, r8, 0xe0c
	ctx.r[6].s64 = ctx.r[8].s64 + 3596;
	// 82633DF4: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82633DF8: D183002C  stfs f12, 0x2c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82633DFC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82633E00: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82633E04: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82633E08: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82633E0C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82633E10: 90E30024  stw r7, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82633E14: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82633E18: 90C30030  stw r6, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82633E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E2C: 4E800020  blr
	return;
	// 82633E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633E34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E44: 4E800020  blr
	return;
}

pub fn sub_82633E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633E48 size=88
	// 82633E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633E5C: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82633E60: 4BBEB3F9  bl 0x8221f258
	ctx.lr = 0x82633E64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633E68: 419A0020  beq cr6, 0x82633e88
	if ctx.cr[6].eq {
	pc = 0x82633E88; continue 'dispatch;
	}
	// 82633E6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633E70: 4BE89F49  bl 0x824bddb8
	ctx.lr = 0x82633E74;
	crate::recompiler::externs::call(&mut ctx, base, 0x824BDDB8);
	// 82633E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E84: 4E800020  blr
	return;
	// 82633E88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633E8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E9C: 4E800020  blr
	return;
}

pub fn sub_82633EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633EA0 size=160
	// 82633EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633EB4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82633EB8: 4BBEB3A1  bl 0x8221f258
	ctx.lr = 0x82633EBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633EBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633EC0: 419A0064  beq cr6, 0x82633f24
	if ctx.cr[6].eq {
	pc = 0x82633F24; continue 'dispatch;
	}
	// 82633EC4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633EC8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633ECC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82633ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633ED4: 390A47A8  addi r8, r10, 0x47a8
	ctx.r[8].s64 = ctx.r[10].s64 + 18344;
	// 82633ED8: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82633EDC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633EE0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633EE4: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82633EE8: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633EEC: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82633EF0: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82633EF4: 9943000F  stb r10, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82633EF8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82633EFC: 99630016  stb r11, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82633F00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82633F04: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82633F08: 99430017  stb r10, 0x17(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(23 as u32), ctx.r[10].u8 ) };
	// 82633F0C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82633F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F20: 4E800020  blr
	return;
	// 82633F24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F38: 4E800020  blr
	return;
}

pub fn sub_82633F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F40 size=112
	// 82633F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633F54: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82633F58: 4BBEB301  bl 0x8221f258
	ctx.lr = 0x82633F5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633F5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633F60: 419A0038  beq cr6, 0x82633f98
	if ctx.cr[6].eq {
	pc = 0x82633F98; continue 'dispatch;
	}
	// 82633F64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633F68: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633F6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633F70: 392A41B8  addi r9, r10, 0x41b8
	ctx.r[9].s64 = ctx.r[10].s64 + 16824;
	// 82633F74: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633F78: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633F7C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633F80: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82633F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F94: 4E800020  blr
	return;
	// 82633F98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633FA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633FAC: 4E800020  blr
	return;
}

pub fn sub_82633FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633FB0 size=120
	// 82633FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633FB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633FC4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82633FC8: 4BBEB291  bl 0x8221f258
	ctx.lr = 0x82633FCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82633FCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633FD0: 419A0040  beq cr6, 0x82634010
	if ctx.cr[6].eq {
	pc = 0x82634010; continue 'dispatch;
	}
	// 82633FD4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633FD8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633FE0: 392A8990  addi r9, r10, -0x7670
	ctx.r[9].s64 = ctx.r[10].s64 + -30320;
	// 82633FE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82633FE8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633FEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633FF0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633FF4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633FF8: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82633FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263400C: 4E800020  blr
	return;
	// 82634010: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634014: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263401C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634024: 4E800020  blr
	return;
}

pub fn sub_82634028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634028 size=128
	// 82634028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263402C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263403C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82634040: 4BBEB219  bl 0x8221f258
	ctx.lr = 0x82634044;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634044: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634048: 419A0044  beq cr6, 0x8263408c
	if ctx.cr[6].eq {
	pc = 0x8263408C; continue 'dispatch;
	}
	// 8263404C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634050: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634054: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634058: 392A89D0  addi r9, r10, -0x7630
	ctx.r[9].s64 = ctx.r[10].s64 + -30256;
	// 8263405C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634060: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634064: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634068: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263406C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634070: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634074: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263407C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634088: 4E800020  blr
	return;
	// 8263408C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263409C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340A0: 4E800020  blr
	return;
}

pub fn sub_826340A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826340A8 size=88
	// 826340A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826340AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826340B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826340B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826340B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826340BC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826340C0: 4BBEB199  bl 0x8221f258
	ctx.lr = 0x826340C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826340C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826340C8: 419A0020  beq cr6, 0x826340e8
	if ctx.cr[6].eq {
	pc = 0x826340E8; continue 'dispatch;
	}
	// 826340CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826340D0: 4818A2D1  bl 0x827be3a0
	ctx.lr = 0x826340D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x827BE3A0);
	// 826340D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826340D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340E4: 4E800020  blr
	return;
	// 826340E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826340EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826340F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340FC: 4E800020  blr
	return;
}

pub fn sub_82634100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634100 size=88
	// 82634100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263410C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634114: 3860015C  li r3, 0x15c
	ctx.r[3].s64 = 348;
	// 82634118: 4BBEB141  bl 0x8221f258
	ctx.lr = 0x8263411C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263411C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634120: 419A0020  beq cr6, 0x82634140
	if ctx.cr[6].eq {
	pc = 0x82634140; continue 'dispatch;
	}
	// 82634124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634128: 4BE5ECD1  bl 0x82492df8
	ctx.lr = 0x8263412C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82492DF8);
	// 8263412C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263413C: 4E800020  blr
	return;
	// 82634140: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263414C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634154: 4E800020  blr
	return;
}

pub fn sub_82634158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634158 size=120
	// 82634158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634164: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263416C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82634170: 4BBEB0E9  bl 0x8221f258
	ctx.lr = 0x82634174;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634174: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634178: 419A0040  beq cr6, 0x826341b8
	if ctx.cr[6].eq {
	pc = 0x826341B8; continue 'dispatch;
	}
	// 8263417C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634180: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634184: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263418C: 390A77A0  addi r8, r10, 0x77a0
	ctx.r[8].s64 = ctx.r[10].s64 + 30624;
	// 82634190: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82634194: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634198: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263419C: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826341A0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826341A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826341A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826341B4: 4E800020  blr
	return;
	// 826341B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826341BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826341C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826341CC: 4E800020  blr
	return;
}

pub fn sub_826341D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826341D0 size=168
	// 826341D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826341D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826341D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826341DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826341E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826341E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826341E8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826341EC: 4BBEB06D  bl 0x8221f258
	ctx.lr = 0x826341F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826341F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826341F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826341F8: 419A0060  beq cr6, 0x82634258
	if ctx.cr[6].eq {
	pc = 0x82634258; continue 'dispatch;
	}
	// 826341FC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 82634200: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634204: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634208: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8263420C: 3BCAABC8  addi r30, r10, -0x5438
	ctx.r[30].s64 = ctx.r[10].s64 + -21560;
	// 82634210: 39094600  addi r8, r9, 0x4600
	ctx.r[8].s64 = ctx.r[9].s64 + 17920;
	// 82634214: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 82634218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263421C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634220: C02AABC8  lfs f1, -0x5438(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21560 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82634224: 98FF0008  stb r7, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82634228: C01E351C  lfs f0, 0x351c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13596 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263422C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82634230: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82634234: 80A60058  lwz r5, 0x58(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) } as u64;
	// 82634238: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263423C: 388B0078  addi r4, r11, 0x78
	ctx.r[4].s64 = ctx.r[11].s64 + 120;
	// 82634240: 4BBBC789  bl 0x821f09c8
	ctx.lr = 0x82634244;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F09C8);
	// 82634244: C01E3514  lfs f0, 0x3514(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13588 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634248: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8263424C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82634250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634254: 48000008  b 0x8263425c
	pc = 0x8263425C; continue 'dispatch;
	// 82634258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263425C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634268: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263426C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634270: 4E800020  blr
	return;
}

pub fn sub_82634278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634278 size=136
	// 82634278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263427C: 48675191  bl 0x82ca940c
	ctx.lr = 0x82634280;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82634280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634284: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634288: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8263428C: 4BBEAFCD  bl 0x8221f258
	ctx.lr = 0x82634290;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634294: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634298: 419A005C  beq cr6, 0x826342f4
	if ctx.cr[6].eq {
	pc = 0x826342F4; continue 'dispatch;
	}
	// 8263429C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 826342A0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826342A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 826342A8: 394B79F0  addi r10, r11, 0x79f0
	ctx.r[10].s64 = ctx.r[11].s64 + 31216;
	// 826342AC: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 826342B0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826342B4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826342B8: 4BBEAFA1  bl 0x8221f258
	ctx.lr = 0x826342BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826342BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826342C0: 419A0008  beq cr6, 0x826342c8
	if ctx.cr[6].eq {
	pc = 0x826342C8; continue 'dispatch;
	}
	// 826342C4: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826342C8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826342CC: 41820008  beq 0x826342d4
	if ctx.cr[0].eq {
	pc = 0x826342D4; continue 'dispatch;
	}
	// 826342D0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826342D4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 826342D8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 826342DC: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 826342E0: 48213CD1  bl 0x82847fb0
	ctx.lr = 0x826342E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82847FB0);
	// 826342E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826342E8: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 826342EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826342F0: 4867516C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826342F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826342F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826342FC: 48675160  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82634300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634300 size=152
	// 82634300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263430C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634314: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82634318: 4BBEAF41  bl 0x8221f258
	ctx.lr = 0x8263431C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263431C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634320: 419A005C  beq cr6, 0x8263437c
	if ctx.cr[6].eq {
	pc = 0x8263437C; continue 'dispatch;
	}
	// 82634324: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634328: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263432C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634330: 38EA7C08  addi r7, r10, 0x7c08
	ctx.r[7].s64 = ctx.r[10].s64 + 31752;
	// 82634334: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82634338: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263433C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634340: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82634344: 38A67088  addi r5, r6, 0x7088
	ctx.r[5].s64 = ctx.r[6].s64 + 28808;
	// 82634348: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263434C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82634350: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82634354: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82634358: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8263435C: 7D20292D  stwcx. r9, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82634360: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82634364: 4082FFE8  bne 0x8263434c
	if !ctx.cr[0].eq {
	pc = 0x8263434C; continue 'dispatch;
	}
	// 82634368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263436C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634374: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634378: 4E800020  blr
	return;
	// 8263437C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263438C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634390: 4E800020  blr
	return;
}

pub fn sub_82634398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634398 size=88
	// 82634398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826343A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826343A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826343AC: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 826343B0: 4BBEAEA9  bl 0x8221f258
	ctx.lr = 0x826343B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826343B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826343B8: 419A0020  beq cr6, 0x826343d8
	if ctx.cr[6].eq {
	pc = 0x826343D8; continue 'dispatch;
	}
	// 826343BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826343C0: 4BF522C9  bl 0x82586688
	ctx.lr = 0x826343C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82586688);
	// 826343C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826343C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826343D4: 4E800020  blr
	return;
	// 826343D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826343DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826343E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826343EC: 4E800020  blr
	return;
}

pub fn sub_826343F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826343F0 size=88
	// 826343F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826343F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826343FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634404: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 82634408: 4BBEAE51  bl 0x8221f258
	ctx.lr = 0x8263440C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263440C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634410: 419A0020  beq cr6, 0x82634430
	if ctx.cr[6].eq {
	pc = 0x82634430; continue 'dispatch;
	}
	// 82634414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634418: 48335051  bl 0x82969468
	ctx.lr = 0x8263441C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82969468);
	// 8263441C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263442C: 4E800020  blr
	return;
	// 82634430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634444: 4E800020  blr
	return;
}

pub fn sub_82634448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634448 size=144
	// 82634448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82634454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263445C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634460: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82634464: 4BBEADF5  bl 0x8221f258
	ctx.lr = 0x82634468;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263446C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634470: 419A0048  beq cr6, 0x826344b8
	if ctx.cr[6].eq {
	pc = 0x826344B8; continue 'dispatch;
	}
	// 82634474: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634478: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263447C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634480: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82634484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634488: 38E92F28  addi r7, r9, 0x2f28
	ctx.r[7].s64 = ctx.r[9].s64 + 12072;
	// 8263448C: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82634490: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634494: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82634498: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263449C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 826344A0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 826344A4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826344A8: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 826344AC: 4BB5F98D  bl 0x82193e38
	ctx.lr = 0x826344B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 826344B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826344B4: 48000008  b 0x826344bc
	pc = 0x826344BC; continue 'dispatch;
	// 826344B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826344BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826344C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826344C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826344C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826344CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826344D0: 4E800020  blr
	return;
}

pub fn sub_826344D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826344D8 size=128
	// 826344D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826344DC: 48674F31  bl 0x82ca940c
	ctx.lr = 0x826344E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826344E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826344E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826344E8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 826344EC: 4BBEAD6D  bl 0x8221f258
	ctx.lr = 0x826344F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826344F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826344F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826344F8: 419A0050  beq cr6, 0x82634548
	if ctx.cr[6].eq {
	pc = 0x82634548; continue 'dispatch;
	}
	// 826344FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82634500: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634504: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82634508: 394B7E78  addi r10, r11, 0x7e78
	ctx.r[10].s64 = ctx.r[11].s64 + 32376;
	// 8263450C: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82634510: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82634514: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82634518: 4BBEAD41  bl 0x8221f258
	ctx.lr = 0x8263451C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263451C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634520: 419A0008  beq cr6, 0x82634528
	if ctx.cr[6].eq {
	pc = 0x82634528; continue 'dispatch;
	}
	// 82634524: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82634528: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8263452C: 41820008  beq 0x82634534
	if ctx.cr[0].eq {
	pc = 0x82634534; continue 'dispatch;
	}
	// 82634530: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82634534: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82634538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263453C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82634540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634544: 48674F18  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82634548: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263454C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634550: 48674F0C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82634558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634558 size=16
	// 82634558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826345D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826345D8 size=280
	// 826345D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826345DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826345E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826345E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826345E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826345EC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826345F0: 4BBEAC69  bl 0x8221f258
	ctx.lr = 0x826345F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826345F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826345F8: 419A00E0  beq cr6, 0x826346d8
	if ctx.cr[6].eq {
	pc = 0x826346D8; continue 'dispatch;
	}
	// 826345FC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82634600: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634604: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 82634608: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263460C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634610: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 82634614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634618: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 8263461C: 38AABD44  addi r5, r10, -0x42bc
	ctx.r[5].s64 = ctx.r[10].s64 + -17084;
	// 82634620: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634624: 388855A8  addi r4, r8, 0x55a8
	ctx.r[4].s64 = ctx.r[8].s64 + 21928;
	// 82634628: C1A99490  lfs f13, -0x6b70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8263462C: 394755E8  addi r10, r7, 0x55e8
	ctx.r[10].s64 = ctx.r[7].s64 + 21992;
	// 82634630: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82634634: 39099490  addi r8, r9, -0x6b70
	ctx.r[8].s64 = ctx.r[9].s64 + -27504;
	// 82634638: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8263463C: 38E60B7C  addi r7, r6, 0xb7c
	ctx.r[7].s64 = ctx.r[6].s64 + 2940;
	// 82634640: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82634644: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82634648: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263464C: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82634650: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82634654: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263465C: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82634660: C008FFF4  lfs f0, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634664: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634668: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8263466C: 90E30020  stw r7, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82634670: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82634674: D1A30040  stfs f13, 0x40(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634678: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8263467C: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82634680: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82634684: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82634688: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8263468C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82634690: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
}

pub fn sub_826346F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826346F0 size=16
	// 826346F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826346F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826346F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826346FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82634770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634770 size=112
	// 82634770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263477C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634784: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82634788: 4BBEAAD1  bl 0x8221f258
	ctx.lr = 0x8263478C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263478C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634790: 419A0038  beq cr6, 0x826347c8
	if ctx.cr[6].eq {
	pc = 0x826347C8; continue 'dispatch;
	}
	// 82634794: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634798: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263479C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826347A0: 392A5A20  addi r9, r10, 0x5a20
	ctx.r[9].s64 = ctx.r[10].s64 + 23072;
	// 826347A4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826347A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826347AC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826347B0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826347B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826347B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826347C4: 4E800020  blr
	return;
	// 826347C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826347CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826347D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826347DC: 4E800020  blr
	return;
}

pub fn sub_826347E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826347E0 size=12
	// 826347E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826347E4: 48674C29  bl 0x82ca940c
	ctx.lr = 0x826347E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826347E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82634868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634868 size=120
	// 82634868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263487C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82634880: 4BBEA9D9  bl 0x8221f258
	ctx.lr = 0x82634884;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634884: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634888: 419A003C  beq cr6, 0x826348c4
	if ctx.cr[6].eq {
	pc = 0x826348C4; continue 'dispatch;
	}
	// 8263488C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82634890: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634898: 392AA250  addi r9, r10, -0x5db0
	ctx.r[9].s64 = ctx.r[10].s64 + -23984;
	// 8263489C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826348A0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826348A4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826348A8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826348AC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826348B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826348B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826348C0: 4E800020  blr
	return;
	// 826348C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826348C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826348CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826348D8: 4E800020  blr
	return;
}

pub fn sub_826348E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826348E0 size=64
	// 826348E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826348E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826348E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826348EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826348F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 826348F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826348F8: 388B9770  addi r4, r11, -0x6890
	ctx.r[4].s64 = ctx.r[11].s64 + -26768;
	// 826348FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634900: 4BBF85D1  bl 0x8222ced0
	ctx.lr = 0x82634904;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82634904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634918: 4E800020  blr
	return;
}

pub fn sub_82634920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634920 size=120
	// 82634920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634924: 48674AE9  bl 0x82ca940c
	ctx.lr = 0x82634928;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82634928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263492C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82634934: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82634938: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8263493C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82634940: 419A0018  beq cr6, 0x82634958
	if ctx.cr[6].eq {
	pc = 0x82634958; continue 'dispatch;
	}
	// 82634944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634948: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263494C: 48005E9D  bl 0x8263a7e8
	ctx.lr = 0x82634950;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A7E8);
	// 82634950: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82634954: 4BBE73E5  bl 0x8221bd38
	ctx.lr = 0x82634958;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82634958: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8263495C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634960: 392A2850  addi r9, r10, 0x2850
	ctx.r[9].s64 = ctx.r[10].s64 + 10320;
	// 82634964: 57A807FE  clrlwi r8, r29, 0x1f
	ctx.r[8].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82634968: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8263496C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82634970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82634974: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634978: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8263497C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634980: 419A000C  beq cr6, 0x8263498c
	if ctx.cr[6].eq {
	pc = 0x8263498C; continue 'dispatch;
	}
	// 82634984: 4BBE73B5  bl 0x8221bd38
	ctx.lr = 0x82634988;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82634988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8263498C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634990: 48674ACC  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82634998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634998 size=88
	// 82634998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263499C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826349A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826349A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826349AC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826349B0: 4BBEA8A9  bl 0x8221f258
	ctx.lr = 0x826349B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826349B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826349B8: 419A0020  beq cr6, 0x826349d8
	if ctx.cr[6].eq {
	pc = 0x826349D8; continue 'dispatch;
	}
	// 826349BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826349C0: 4BFA7E69  bl 0x825dc828
	ctx.lr = 0x826349C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x825DC828);
	// 826349C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826349C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826349D4: 4E800020  blr
	return;
	// 826349D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826349DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826349E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826349EC: 4E800020  blr
	return;
}

pub fn sub_826349F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826349F0 size=136
	// 826349F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826349F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826349FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634A04: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82634A08: 4BBEA851  bl 0x8221f258
	ctx.lr = 0x82634A0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634A0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634A10: 419A0050  beq cr6, 0x82634a60
	if ctx.cr[6].eq {
	pc = 0x82634A60; continue 'dispatch;
	}
	// 82634A14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634A18: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634A1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634A20: 392A4780  addi r9, r10, 0x4780
	ctx.r[9].s64 = ctx.r[10].s64 + 18304;
	// 82634A24: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634A28: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634A2C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634A30: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634A34: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634A38: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82634A3C: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82634A40: 9963001E  stb r11, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 82634A44: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634A48: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634A5C: 4E800020  blr
	return;
	// 82634A60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634A64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634A74: 4E800020  blr
	return;
}

pub fn sub_82634A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634A78 size=208
	// 82634A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634A8C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82634A90: 4BBEA7C9  bl 0x8221f258
	ctx.lr = 0x82634A94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634A94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634A98: 419A0094  beq cr6, 0x82634b2c
	if ctx.cr[6].eq {
	pc = 0x82634B2C; continue 'dispatch;
	}
	// 82634A9C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 82634AA0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634AA4: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634AA8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 82634AAC: 38E99484  addi r7, r9, -0x6b7c
	ctx.r[7].s64 = ctx.r[9].s64 + -27516;
	// 82634AB0: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82634AB4: 814B8824  lwz r10, -0x77dc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30684 as u32) ) } as u64;
	// 82634AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634ABC: 38A82D10  addi r5, r8, 0x2d10
	ctx.r[5].s64 = ctx.r[8].s64 + 11536;
	// 82634AC0: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634AC4: 38860B7C  addi r4, r6, 0xb7c
	ctx.r[4].s64 = ctx.r[6].s64 + 2940;
	// 82634AC8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634ACC: C1A7000C  lfs f13, 0xc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634AD0: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82634AD4: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82634AD8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634ADC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82634AE0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634AE4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634AE8: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634AEC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82634AF0: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82634AF4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82634AF8: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82634AFC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634B00: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82634B04: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82634B08: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82634B0C: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82634B10: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82634B14: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82634B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634B28: 4E800020  blr
	return;
	// 82634B2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634B40: 4E800020  blr
	return;
}

pub fn sub_82634B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634B48 size=176
	// 82634B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634B4C: 486748BD  bl 0x82ca9408
	ctx.lr = 0x82634B50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82634B50: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82634B54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634B58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82634B5C: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82634B60: 4BBEA6F9  bl 0x8221f258
	ctx.lr = 0x82634B64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634B68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634B6C: 419A007C  beq cr6, 0x82634be8
	if ctx.cr[6].eq {
	pc = 0x82634BE8; continue 'dispatch;
	}
	// 82634B70: 3F80820A  lis r28, -0x7df6
	ctx.r[28].s64 = -2113273856;
	// 82634B74: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82634B78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82634B7C: 395C9490  addi r10, r28, -0x6b70
	ctx.r[10].s64 = ctx.r[28].s64 + -27504;
	// 82634B80: 392B32A8  addi r9, r11, 0x32a8
	ctx.r[9].s64 = ctx.r[11].s64 + 12968;
	// 82634B84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82634B88: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634B8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82634B90: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82634B94: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82634B98: C3EAFFF4  lfs f31, -0xc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82634B9C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82634BA0: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82634BA4: 4BE15225  bl 0x82449dc8
	ctx.lr = 0x82634BA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82449DC8);
	// 82634BA8: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82634BAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82634BB0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82634BB4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82634BB8: 4BE15211  bl 0x82449dc8
	ctx.lr = 0x82634BBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82449DC8);
	// 82634BBC: C01C9490  lfs f0, -0x6b70(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634BC0: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634BC4: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82634BC8: D3FF0040  stfs f31, 0x40(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634BD0: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82634BD4: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 82634BD8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82634BDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82634BE0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82634BE4: 48674874  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 82634BE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634BEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82634BF0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82634BF4: 48674864  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_82634BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634BF8 size=280
	// 82634BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634C0C: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82634C10: 4BBEA649  bl 0x8221f258
	ctx.lr = 0x82634C14;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634C14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634C18: 419A00E0  beq cr6, 0x82634cf8
	if ctx.cr[6].eq {
	pc = 0x82634CF8; continue 'dispatch;
	}
	// 82634C1C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634C20: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634C24: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82634C28: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634C2C: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 82634C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634C34: 388904D4  addi r4, r9, 0x4d4
	ctx.r[4].s64 = ctx.r[9].s64 + 1236;
	// 82634C38: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82634C3C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634C40: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 82634C44: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634C48: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82634C4C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82634C50: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 82634C54: C1A8FFF4  lfs f13, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634C58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634C5C: C188FE44  lfs f12, -0x1bc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-444 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82634C60: 38E73F78  addi r7, r7, 0x3f78
	ctx.r[7].s64 = ctx.r[7].s64 + 16248;
	// 82634C64: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634C68: 38C64F50  addi r6, r6, 0x4f50
	ctx.r[6].s64 = ctx.r[6].s64 + 20304;
	// 82634C6C: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82634C70: 38A50B7C  addi r5, r5, 0xb7c
	ctx.r[5].s64 = ctx.r[5].s64 + 2940;
	// 82634C74: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82634C78: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82634C7C: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82634C80: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82634C84: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82634C88: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634C8C: D1830038  stfs f12, 0x38(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82634C90: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634C94: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82634C98: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82634C9C: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82634CA0: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82634CA4: 38E80E0C  addi r7, r8, 0xe0c
	ctx.r[7].s64 = ctx.r[8].s64 + 3596;
	// 82634CA8: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634CAC: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82634CB0: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
}

pub fn sub_82634D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D10 size=88
	// 82634D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634D24: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82634D28: 4BBEA531  bl 0x8221f258
	ctx.lr = 0x82634D2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634D30: 419A0020  beq cr6, 0x82634d50
	if ctx.cr[6].eq {
	pc = 0x82634D50; continue 'dispatch;
	}
	// 82634D34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634D38: 4BDFC341  bl 0x82431078
	ctx.lr = 0x82634D3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82431078);
	// 82634D3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634D4C: 4E800020  blr
	return;
	// 82634D50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634D64: 4E800020  blr
	return;
}

pub fn sub_82634D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D68 size=88
	// 82634D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634D74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634D7C: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82634D80: 4BBEA4D9  bl 0x8221f258
	ctx.lr = 0x82634D84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634D84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634D88: 419A0020  beq cr6, 0x82634da8
	if ctx.cr[6].eq {
	pc = 0x82634DA8; continue 'dispatch;
	}
	// 82634D8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634D90: 48175051  bl 0x827a9de0
	ctx.lr = 0x82634D94;
	crate::recompiler::externs::call(&mut ctx, base, 0x827A9DE0);
	// 82634D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634DA4: 4E800020  blr
	return;
	// 82634DA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634DBC: 4E800020  blr
	return;
}

pub fn sub_82634DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634DC0 size=168
	// 82634DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82634DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634DD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634DD8: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 82634DDC: 4BBEA47D  bl 0x8221f258
	ctx.lr = 0x82634DE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634DE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634DE8: 419A0064  beq cr6, 0x82634e4c
	if ctx.cr[6].eq {
	pc = 0x82634E4C; continue 'dispatch;
	}
	// 82634DEC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634DF0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634DF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82634DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634DFC: 39092E68  addi r8, r9, 0x2e68
	ctx.r[8].s64 = ctx.r[9].s64 + 11880;
	// 82634E00: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634E04: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82634E08: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 82634E0C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634E10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 82634E14: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634E18: 386727D8  addi r3, r7, 0x27d8
	ctx.r[3].s64 = ctx.r[7].s64 + 10200;
	// 82634E1C: 997F001E  stb r11, 0x1e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 82634E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 82634E24: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82634E28: 4BBBEF31  bl 0x821f3d58
	ctx.lr = 0x82634E2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 82634E2C: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82634E30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82634E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634E38: 80666DA0  lwz r3, 0x6da0(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28064 as u32) ) } as u64;
	// 82634E3C: 4BC9868D  bl 0x822cd4c8
	ctx.lr = 0x82634E40;
	crate::recompiler::externs::call(&mut ctx, base, 0x822CD4C8);
	// 82634E40: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82634E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634E48: 48000008  b 0x82634e50
	pc = 0x82634E50; continue 'dispatch;
	// 82634E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634E50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82634E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634E64: 4E800020  blr
	return;
}

pub fn sub_82634E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634E68 size=248
	// 82634E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634E7C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82634E80: 4BBEA3D9  bl 0x8221f258
	ctx.lr = 0x82634E84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634E84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634E88: 419A00BC  beq cr6, 0x82634f44
	if ctx.cr[6].eq {
	pc = 0x82634F44; continue 'dispatch;
	}
	// 82634E8C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634E90: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634E94: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634E98: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634E9C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82634EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634EA4: 38EA61E0  addi r7, r10, 0x61e0
	ctx.r[7].s64 = ctx.r[10].s64 + 25056;
	// 82634EA8: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82634EAC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634EB0: 38A99484  addi r5, r9, -0x6b7c
	ctx.r[5].s64 = ctx.r[9].s64 + -27516;
	// 82634EB4: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634EB8: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82634EBC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82634EC0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634EC4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82634EC8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634ECC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634ED0: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82634ED4: C1899484  lfs f12, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82634ED8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634EDC: C005000C  lfs f0, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634EE0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634EE4: C1A54144  lfs f13, 0x4144(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634EE8: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82634EEC: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 82634EF0: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634EF4: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82634EF8: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
}

pub fn sub_82634F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634F60 size=216
	// 82634F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634F70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634F74: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 82634F78: 4BBEA2E1  bl 0x8221f258
	ctx.lr = 0x82634F7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82634F7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634F80: 419A00A0  beq cr6, 0x82635020
	if ctx.cr[6].eq {
	pc = 0x82635020; continue 'dispatch;
	}
	// 82634F84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634F88: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634F8C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F94: 390A3EB8  addi r8, r10, 0x3eb8
	ctx.r[8].s64 = ctx.r[10].s64 + 16056;
	// 82634F98: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82634F9C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634FA0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634FA4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82634FA8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634FAC: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82634FB0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634FB4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634FB8: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82634FBC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634FC0: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82634FC4: 90E30024  stw r7, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82634FC8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82634FCC: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82634FD0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82634FD4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82634FD8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82634FDC: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82634FE0: 99630044  stb r11, 0x44(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 82634FE4: 99630045  stb r11, 0x45(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(69 as u32), ctx.r[11].u8 ) };
	// 82634FE8: 99630046  stb r11, 0x46(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(70 as u32), ctx.r[11].u8 ) };
	// 82634FEC: 99630047  stb r11, 0x47(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(71 as u32), ctx.r[11].u8 ) };
	// 82634FF0: 99630048  stb r11, 0x48(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82634FF4: 99630049  stb r11, 0x49(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 82634FF8: 9963004A  stb r11, 0x4a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 82634FFC: 9963004B  stb r11, 0x4b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 82635000: 98C3004C  stb r6, 0x4c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[6].u8 ) };
	// 82635004: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82635008: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263500C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263501C: 4E800020  blr
	return;
	// 82635020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263502C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635034: 4E800020  blr
	return;
}

pub fn sub_82635038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635038 size=104
	// 82635038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263503C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635040: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635044: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263504C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82635050: 4BBEA209  bl 0x8221f258
	ctx.lr = 0x82635054;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635058: 419A0030  beq cr6, 0x82635088
	if ctx.cr[6].eq {
	pc = 0x82635088; continue 'dispatch;
	}
	// 8263505C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635060: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635068: 392A46C0  addi r9, r10, 0x46c0
	ctx.r[9].s64 = ctx.r[10].s64 + 18112;
	// 8263506C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635070: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263507C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635084: 4E800020  blr
	return;
	// 82635088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263508C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263509C: 4E800020  blr
	return;
}

pub fn sub_826350A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826350A0 size=112
	// 826350A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826350A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826350A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826350AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826350B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826350B4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826350B8: 4BBEA1A1  bl 0x8221f258
	ctx.lr = 0x826350BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826350BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826350C0: 419A0038  beq cr6, 0x826350f8
	if ctx.cr[6].eq {
	pc = 0x826350F8; continue 'dispatch;
	}
	// 826350C4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826350C8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826350CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826350D0: 392A6640  addi r9, r10, 0x6640
	ctx.r[9].s64 = ctx.r[10].s64 + 26176;
	// 826350D4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826350D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826350DC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826350E0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826350E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826350E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826350EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826350F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826350F4: 4E800020  blr
	return;
	// 826350F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826350FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263510C: 4E800020  blr
	return;
}

pub fn sub_82635110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635110 size=120
	// 82635110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263511C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635124: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82635128: 4BBEA131  bl 0x8221f258
	ctx.lr = 0x8263512C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263512C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635130: 419A0040  beq cr6, 0x82635170
	if ctx.cr[6].eq {
	pc = 0x82635170; continue 'dispatch;
	}
	// 82635134: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635138: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263513C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82635140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635144: 390A6720  addi r8, r10, 0x6720
	ctx.r[8].s64 = ctx.r[10].s64 + 26400;
	// 82635148: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 8263514C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635150: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635154: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82635158: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263515C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263516C: 4E800020  blr
	return;
	// 82635170: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263517C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635184: 4E800020  blr
	return;
}

pub fn sub_82635188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635188 size=88
	// 82635188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263518C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263519C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 826351A0: 4BBEA0B9  bl 0x8221f258
	ctx.lr = 0x826351A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826351A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826351A8: 419A0020  beq cr6, 0x826351c8
	if ctx.cr[6].eq {
	pc = 0x826351C8; continue 'dispatch;
	}
	// 826351AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826351B0: 482995B1  bl 0x828ce760
	ctx.lr = 0x826351B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x828CE760);
	// 826351B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826351B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826351C4: 4E800020  blr
	return;
	// 826351C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826351CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826351D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826351DC: 4E800020  blr
	return;
}

pub fn sub_826351E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826351E0 size=12
	// 826351E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826351E4: 48674229  bl 0x82ca940c
	ctx.lr = 0x826351E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826351E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82635280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635280 size=88
	// 82635280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635294: 38600104  li r3, 0x104
	ctx.r[3].s64 = 260;
	// 82635298: 4BBE9FC1  bl 0x8221f258
	ctx.lr = 0x8263529C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263529C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826352A0: 419A0020  beq cr6, 0x826352c0
	if ctx.cr[6].eq {
	pc = 0x826352C0; continue 'dispatch;
	}
	// 826352A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826352A8: 481A1C79  bl 0x827d6f20
	ctx.lr = 0x826352AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x827D6F20);
	// 826352AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826352B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826352BC: 4E800020  blr
	return;
	// 826352C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826352C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826352C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826352D4: 4E800020  blr
	return;
}

pub fn sub_826352D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826352D8 size=128
	// 826352D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826352DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826352E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826352E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826352E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826352EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826352F0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826352F4: 4BBE9F65  bl 0x8221f258
	ctx.lr = 0x826352F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826352F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826352FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635300: 419A0038  beq cr6, 0x82635338
	if ctx.cr[6].eq {
	pc = 0x82635338; continue 'dispatch;
	}
	// 82635304: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635308: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263530C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635310: 392A2A50  addi r9, r10, 0x2a50
	ctx.r[9].s64 = ctx.r[10].s64 + 10832;
	// 82635314: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635318: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8263531C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635324: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635328: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263532C: 4BD5818D  bl 0x8238d4b8
	ctx.lr = 0x82635330;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82635330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635334: 48000008  b 0x8263533c
	pc = 0x8263533C; continue 'dispatch;
	// 82635338: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263533C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263534C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635350: 4E800020  blr
	return;
}

pub fn sub_82635358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635358 size=12
	// 82635358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263535C: 486740B1  bl 0x82ca940c
	ctx.lr = 0x82635360;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82635360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82635418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635418 size=88
	// 82635418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263542C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82635430: 4BBE9E29  bl 0x8221f258
	ctx.lr = 0x82635434;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635438: 419A0020  beq cr6, 0x82635458
	if ctx.cr[6].eq {
	pc = 0x82635458; continue 'dispatch;
	}
	// 8263543C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635440: 48302C49  bl 0x82938088
	ctx.lr = 0x82635444;
	crate::recompiler::externs::call(&mut ctx, base, 0x82938088);
	// 82635444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263544C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635454: 4E800020  blr
	return;
	// 82635458: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263545C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263546C: 4E800020  blr
	return;
}

pub fn sub_82635470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635470 size=88
	// 82635470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263547C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635484: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82635488: 4BBE9DD1  bl 0x8221f258
	ctx.lr = 0x8263548C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263548C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635490: 419A0020  beq cr6, 0x826354b0
	if ctx.cr[6].eq {
	pc = 0x826354B0; continue 'dispatch;
	}
	// 82635494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635498: 482C9DB1  bl 0x828ff248
	ctx.lr = 0x8263549C;
	crate::recompiler::externs::call(&mut ctx, base, 0x828FF248);
	// 8263549C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826354A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826354AC: 4E800020  blr
	return;
	// 826354B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826354B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826354B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826354C4: 4E800020  blr
	return;
}

pub fn sub_826354C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826354C8 size=168
	// 826354C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826354CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826354D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826354D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826354D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826354DC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 826354E0: 4BBE9D79  bl 0x8221f258
	ctx.lr = 0x826354E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826354E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826354E8: 419A006C  beq cr6, 0x82635554
	if ctx.cr[6].eq {
	pc = 0x82635554; continue 'dispatch;
	}
	// 826354EC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826354F0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826354F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826354F8: 38E99490  addi r7, r9, -0x6b70
	ctx.r[7].s64 = ctx.r[9].s64 + -27504;
	// 826354FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635500: 390A4CE8  addi r8, r10, 0x4ce8
	ctx.r[8].s64 = ctx.r[10].s64 + 19688;
	// 82635504: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82635508: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263550C: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635510: C1499490  lfs f10, -0x6b70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82635514: C007FFF4  lfs f0, -0xc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635518: C1A75D38  lfs f13, 0x5d38(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(23864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8263551C: C18729D4  lfs f12, 0x29d4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(10708 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82635520: C1671FF0  lfs f11, 0x1ff0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8176 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82635524: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635528: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8263552C: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635530: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635534: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82635538: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8263553C: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82635540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263554C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635550: 4E800020  blr
	return;
	// 82635554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635568: 4E800020  blr
	return;
}

pub fn sub_82635570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635570 size=104
	// 82635570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263557C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635584: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82635588: 4BBE9CD1  bl 0x8221f258
	ctx.lr = 0x8263558C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263558C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635590: 419A0030  beq cr6, 0x826355c0
	if ctx.cr[6].eq {
	pc = 0x826355C0; continue 'dispatch;
	}
	// 82635594: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635598: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263559C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826355A0: 392A7248  addi r9, r10, 0x7248
	ctx.r[9].s64 = ctx.r[10].s64 + 29256;
	// 826355A4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826355A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826355AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826355B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826355BC: 4E800020  blr
	return;
	// 826355C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826355C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826355C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826355D4: 4E800020  blr
	return;
}

pub fn sub_826355D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826355D8 size=20
	// 826355D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826355DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826355E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826355E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826355E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82635678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635678 size=88
	// 82635678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263568C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82635690: 4BBE9BC9  bl 0x8221f258
	ctx.lr = 0x82635694;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635694: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635698: 419A0020  beq cr6, 0x826356b8
	if ctx.cr[6].eq {
	pc = 0x826356B8; continue 'dispatch;
	}
	// 8263569C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826356A0: 48316E59  bl 0x8294c4f8
	ctx.lr = 0x826356A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8294C4F8);
	// 826356A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826356A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826356B4: 4E800020  blr
	return;
	// 826356B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826356BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826356C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826356CC: 4E800020  blr
	return;
}

pub fn sub_826356D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826356D0 size=88
	// 826356D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826356D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826356D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826356DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826356E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826356E4: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826356E8: 4BBE9B71  bl 0x8221f258
	ctx.lr = 0x826356EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826356EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826356F0: 419A0020  beq cr6, 0x82635710
	if ctx.cr[6].eq {
	pc = 0x82635710; continue 'dispatch;
	}
	// 826356F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826356F8: 482C1D71  bl 0x828f7468
	ctx.lr = 0x826356FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x828F7468);
	// 826356FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263570C: 4E800020  blr
	return;
	// 82635710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263571C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635724: 4E800020  blr
	return;
}

pub fn sub_82635728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635728 size=20
	// 82635728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826357E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826357E8 size=88
	// 826357E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826357EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826357F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826357F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826357F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826357FC: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 82635800: 4BBE9A59  bl 0x8221f258
	ctx.lr = 0x82635804;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635804: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635808: 419A0020  beq cr6, 0x82635828
	if ctx.cr[6].eq {
	pc = 0x82635828; continue 'dispatch;
	}
	// 8263580C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635810: 482BDF99  bl 0x828f37a8
	ctx.lr = 0x82635814;
	crate::recompiler::externs::call(&mut ctx, base, 0x828F37A8);
	// 82635814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263581C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635824: 4E800020  blr
	return;
	// 82635828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263582C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263583C: 4E800020  blr
	return;
}

pub fn sub_82635840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635840 size=192
	// 82635840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635844: 48673BC9  bl 0x82ca940c
	ctx.lr = 0x82635848;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82635848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263584C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635850: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82635854: 4BBE9A05  bl 0x8221f258
	ctx.lr = 0x82635858;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263585C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635860: 419A0090  beq cr6, 0x826358f0
	if ctx.cr[6].eq {
	pc = 0x826358F0; continue 'dispatch;
	}
	// 82635864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82635868: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263586C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635870: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635874: 392B8150  addi r9, r11, -0x7eb0
	ctx.r[9].s64 = ctx.r[11].s64 + -32432;
	// 82635878: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263587C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635880: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635884: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82635888: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8263588C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82635890: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82635894: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82635898: 4BB5E5A1  bl 0x82193e38
	ctx.lr = 0x8263589C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 8263589C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 826358A0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 826358A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826358A8: 38C79484  addi r6, r7, -0x6b7c
	ctx.r[6].s64 = ctx.r[7].s64 + -27516;
	// 826358AC: 9BDF0030  stb r30, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u8 ) };
	// 826358B0: 9BDF0031  stb r30, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[30].u8 ) };
	// 826358B4: 9BDF0032  stb r30, 0x32(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(50 as u32), ctx.r[30].u8 ) };
	// 826358B8: C1A79484  lfs f13, -0x6b7c(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 826358BC: 9BDF0033  stb r30, 0x33(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(51 as u32), ctx.r[30].u8 ) };
	// 826358C0: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 826358C4: 9BDF0034  stb r30, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 826358C8: C006000C  lfs f0, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826358CC: 9BDF0035  stb r30, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[30].u8 ) };
	// 826358D0: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 826358D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826358D8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 826358DC: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 826358E0: 4BD57BD9  bl 0x8238d4b8
	ctx.lr = 0x826358E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 826358E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826358E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826358EC: 48673B70  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826358F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826358F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826358F8: 48673B64  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82635900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635900 size=136
	// 82635900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635904: 48673B09  bl 0x82ca940c
	ctx.lr = 0x82635908;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82635908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263590C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635910: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82635914: 4BBE9945  bl 0x8221f258
	ctx.lr = 0x82635918;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263591C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635920: 419A0058  beq cr6, 0x82635978
	if ctx.cr[6].eq {
	pc = 0x82635978; continue 'dispatch;
	}
	// 82635924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82635928: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263592C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635930: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635934: 392B8190  addi r9, r11, -0x7e70
	ctx.r[9].s64 = ctx.r[11].s64 + -32368;
	// 82635938: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263593C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635940: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635944: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82635948: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8263594C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82635950: 4BB5E4E9  bl 0x82193e38
	ctx.lr = 0x82635954;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 82635954: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82635958: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8263595C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635960: 90FF0018  stw r7, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82635964: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635968: 4BD57B51  bl 0x8238d4b8
	ctx.lr = 0x8263596C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 8263596C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635974: 48673AE8  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82635978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263597C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635980: 48673ADC  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82635988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635988 size=88
	// 82635988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263599C: 386001C0  li r3, 0x1c0
	ctx.r[3].s64 = 448;
	// 826359A0: 4BBE98B9  bl 0x8221f258
	ctx.lr = 0x826359A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826359A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826359A8: 419A0020  beq cr6, 0x826359c8
	if ctx.cr[6].eq {
	pc = 0x826359C8; continue 'dispatch;
	}
	// 826359AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826359B0: 48121D49  bl 0x827576f8
	ctx.lr = 0x826359B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x827576F8);
	// 826359B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826359B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826359C4: 4E800020  blr
	return;
	// 826359C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826359CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826359D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826359DC: 4E800020  blr
	return;
}

pub fn sub_826359E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826359E0 size=136
	// 826359E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826359E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826359E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826359EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826359F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826359F4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 826359F8: 4BBE9861  bl 0x8221f258
	ctx.lr = 0x826359FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826359FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635A00: 419A004C  beq cr6, 0x82635a4c
	if ctx.cr[6].eq {
	pc = 0x82635A4C; continue 'dispatch;
	}
	// 82635A04: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635A08: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635A0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635A10: 392A6558  addi r9, r10, 0x6558
	ctx.r[9].s64 = ctx.r[10].s64 + 25944;
	// 82635A14: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635A18: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82635A1C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635A20: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635A24: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635A28: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82635A2C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82635A30: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82635A34: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82635A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635A48: 4E800020  blr
	return;
	// 82635A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635A60: 4E800020  blr
	return;
}

pub fn sub_82635A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635A68 size=232
	// 82635A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635A70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635A74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635A78: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82635A7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635A80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635A84: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82635A88: 4BBE97D1  bl 0x8221f258
	ctx.lr = 0x82635A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635A90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635A94: 419A0098  beq cr6, 0x82635b2c
	if ctx.cr[6].eq {
	pc = 0x82635B2C; continue 'dispatch;
	}
	// 82635A98: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82635A9C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635AA0: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82635AA4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82635AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635AAC: 38E95340  addi r7, r9, 0x5340
	ctx.r[7].s64 = ctx.r[9].s64 + 21312;
	// 82635AB0: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82635AB4: C3EA9484  lfs f31, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82635AB8: D3FF0014  stfs f31, 0x14(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635ABC: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635AC0: D3FF0018  stfs f31, 0x18(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635AC4: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635AC8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635ACC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82635AD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635AD4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82635AD8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82635ADC: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82635AE0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82635AE4: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82635AE8: 4BB5E351  bl 0x82193e38
	ctx.lr = 0x82635AEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 82635AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635AF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635AF4: 4BD579C5  bl 0x8238d4b8
	ctx.lr = 0x82635AF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82635AF8: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82635AFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82635B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B04: C0250A54  lfs f1, 0xa54(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(2644 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82635B08: D03F0014  stfs f1, 0x14(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635B0C: 482D0CE5  bl 0x829067f0
	ctx.lr = 0x82635B10;
	crate::recompiler::externs::call(&mut ctx, base, 0x829067F0);
	// 82635B10: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635B14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82635B18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82635B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B20: 482D0FD9  bl 0x82906af8
	ctx.lr = 0x82635B24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82906AF8);
	// 82635B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B28: 48000008  b 0x82635b30
	pc = 0x82635B30; continue 'dispatch;
	// 82635B2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B3C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82635B40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635B44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635B48: 4E800020  blr
	return;
}

pub fn sub_82635B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B50 size=88
	// 82635B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635B64: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 82635B68: 4BBE96F1  bl 0x8221f258
	ctx.lr = 0x82635B6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635B6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635B70: 419A0020  beq cr6, 0x82635b90
	if ctx.cr[6].eq {
	pc = 0x82635B90; continue 'dispatch;
	}
	// 82635B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635B78: 48144041  bl 0x82779bb8
	ctx.lr = 0x82635B7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82779BB8);
	// 82635B7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635B8C: 4E800020  blr
	return;
	// 82635B90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635BA4: 4E800020  blr
	return;
}

pub fn sub_82635BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635BA8 size=152
	// 82635BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635BBC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82635BC0: 4BBE9699  bl 0x8221f258
	ctx.lr = 0x82635BC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635BC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635BC8: 419A0060  beq cr6, 0x82635c28
	if ctx.cr[6].eq {
	pc = 0x82635C28; continue 'dispatch;
	}
	// 82635BCC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635BD0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635BD8: 38EA6C98  addi r7, r10, 0x6c98
	ctx.r[7].s64 = ctx.r[10].s64 + 27800;
	// 82635BDC: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82635BE0: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635BE4: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635BE8: 38A67088  addi r5, r6, 0x7088
	ctx.r[5].s64 = ctx.r[6].s64 + 28808;
	// 82635BEC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635BF0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82635BF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82635BF8: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82635BFC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82635C00: 7D20292D  stwcx. r9, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82635C04: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82635C08: 4082FFE8  bne 0x82635bf0
	if !ctx.cr[0].eq {
	pc = 0x82635BF0; continue 'dispatch;
	}
	// 82635C0C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635C10: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82635C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635C24: 4E800020  blr
	return;
	// 82635C28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635C2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635C3C: 4E800020  blr
	return;
}

pub fn sub_82635C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C40 size=16
	// 82635C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635C4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82635CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635CE0 size=20
	// 82635CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82635D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635D60 size=120
	// 82635D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635D74: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82635D78: 4BBE94E1  bl 0x8221f258
	ctx.lr = 0x82635D7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635D80: 419A003C  beq cr6, 0x82635dbc
	if ctx.cr[6].eq {
	pc = 0x82635DBC; continue 'dispatch;
	}
	// 82635D84: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635D88: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635D8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D90: 392A2EF0  addi r9, r10, 0x2ef0
	ctx.r[9].s64 = ctx.r[10].s64 + 12016;
	// 82635D94: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635D98: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635D9C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635DA0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82635DA4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82635DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635DB8: 4E800020  blr
	return;
	// 82635DBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635DD0: 4E800020  blr
	return;
}

pub fn sub_82635DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635DD8 size=120
	// 82635DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635DF0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82635DF4: 4BBE9465  bl 0x8221f258
	ctx.lr = 0x82635DF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635DFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635E00: 419A0030  beq cr6, 0x82635e30
	if ctx.cr[6].eq {
	pc = 0x82635E30; continue 'dispatch;
	}
	// 82635E04: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82635E08: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E10: 392B7288  addi r9, r11, 0x7288
	ctx.r[9].s64 = ctx.r[11].s64 + 29320;
	// 82635E14: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82635E18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635E1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635E20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635E24: 4BD57695  bl 0x8238d4b8
	ctx.lr = 0x82635E28;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82635E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635E2C: 48000008  b 0x82635e34
	pc = 0x82635E34; continue 'dispatch;
	// 82635E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635E34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635E48: 4E800020  blr
	return;
}

pub fn sub_82635E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635E50 size=176
	// 82635E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635E64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635E68: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82635E6C: 4BBE93ED  bl 0x8221f258
	ctx.lr = 0x82635E70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635E74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635E78: 419A006C  beq cr6, 0x82635ee4
	if ctx.cr[6].eq {
	pc = 0x82635EE4; continue 'dispatch;
	}
	// 82635E7C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82635E80: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635E84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82635E88: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 82635E8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635E90: 38E93630  addi r7, r9, 0x3630
	ctx.r[7].s64 = ctx.r[9].s64 + 13872;
	// 82635E94: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635E98: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635E9C: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82635EA0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635EA4: C008FFF4  lfs f0, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635EA8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82635EAC: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635EB0: 997F0030  stb r11, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82635EB4: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82635EB8: 997F0031  stb r11, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 82635EBC: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635EC4: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635ECC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82635ED0: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82635ED4: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82635ED8: 4BD575E1  bl 0x8238d4b8
	ctx.lr = 0x82635EDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82635EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635EE0: 48000008  b 0x82635ee8
	pc = 0x82635EE8; continue 'dispatch;
	// 82635EE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635EFC: 4E800020  blr
	return;
}

pub fn sub_82635F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635F00 size=184
	// 82635F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F04: 48673509  bl 0x82ca940c
	ctx.lr = 0x82635F08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82635F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635F10: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82635F14: 4BBE9345  bl 0x8221f258
	ctx.lr = 0x82635F18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635F1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635F20: 419A008C  beq cr6, 0x82635fac
	if ctx.cr[6].eq {
	pc = 0x82635FAC; continue 'dispatch;
	}
	// 82635F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82635F28: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82635F2C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635F30: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 82635F34: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82635F38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635F3C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635F40: 38EA46B0  addi r7, r10, 0x46b0
	ctx.r[7].s64 = ctx.r[10].s64 + 18096;
	// 82635F44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82635F48: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635F4C: 38A80B7C  addi r5, r8, 0xb7c
	ctx.r[5].s64 = ctx.r[8].s64 + 2940;
	// 82635F50: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635F54: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635F58: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635F5C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82635F60: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635F64: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82635F68: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82635F6C: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82635F70: 98DF001D  stb r6, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[6].u8 ) };
	// 82635F74: 9BDF001E  stb r30, 0x1e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[30].u8 ) };
	// 82635F78: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82635F7C: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82635F80: 4BB5DEB9  bl 0x82193e38
	ctx.lr = 0x82635F84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 82635F84: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82635F88: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82635F8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635F90: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82635F94: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 82635F98: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635F9C: 4BD5751D  bl 0x8238d4b8
	ctx.lr = 0x82635FA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82635FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635FA8: 486734B4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82635FAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635FB4: 486734A8  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82635FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635FB8 size=104
	// 82635FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635FCC: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82635FD0: 4BBE9289  bl 0x8221f258
	ctx.lr = 0x82635FD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82635FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635FD8: 419A0030  beq cr6, 0x82636008
	if ctx.cr[6].eq {
	pc = 0x82636008; continue 'dispatch;
	}
	// 82635FDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635FE0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635FE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635FE8: 392A4700  addi r9, r10, 0x4700
	ctx.r[9].s64 = ctx.r[10].s64 + 18176;
	// 82635FEC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635FF0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636004: 4E800020  blr
	return;
	// 82636008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263600C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263601C: 4E800020  blr
	return;
}

pub fn sub_82636020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636020 size=128
	// 82636020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636024: 486733E9  bl 0x82ca940c
	ctx.lr = 0x82636028;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82636028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263602C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636030: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82636034: 4BBE9225  bl 0x8221f258
	ctx.lr = 0x82636038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263603C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636040: 419A0050  beq cr6, 0x82636090
	if ctx.cr[6].eq {
	pc = 0x82636090; continue 'dispatch;
	}
	// 82636044: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82636048: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263604C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82636050: 394B4580  addi r10, r11, 0x4580
	ctx.r[10].s64 = ctx.r[11].s64 + 17792;
	// 82636054: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82636058: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8263605C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636060: 4BBE91F9  bl 0x8221f258
	ctx.lr = 0x82636064;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636064: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636068: 419A0008  beq cr6, 0x82636070
	if ctx.cr[6].eq {
	pc = 0x82636070; continue 'dispatch;
	}
	// 8263606C: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636070: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82636074: 41820008  beq 0x8263607c
	if ctx.cr[0].eq {
	pc = 0x8263607C; continue 'dispatch;
	}
	// 82636078: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8263607C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82636080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636084: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82636088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263608C: 486733D0  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82636090: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636098: 486733C4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826360A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826360A0 size=88
	// 826360A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826360A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826360AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826360B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826360B4: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 826360B8: 4BBE91A1  bl 0x8221f258
	ctx.lr = 0x826360BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826360BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826360C0: 419A0020  beq cr6, 0x826360e0
	if ctx.cr[6].eq {
	pc = 0x826360E0; continue 'dispatch;
	}
	// 826360C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826360C8: 4BDDD3E9  bl 0x824134b0
	ctx.lr = 0x826360CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x824134B0);
	// 826360CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826360D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826360DC: 4E800020  blr
	return;
	// 826360E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826360E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826360E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826360F4: 4E800020  blr
	return;
}

pub fn sub_826360F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826360F8 size=176
	// 826360F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263610C: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82636110: 4BBE9149  bl 0x8221f258
	ctx.lr = 0x82636114;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636118: 419A0074  beq cr6, 0x8263618c
	if ctx.cr[6].eq {
	pc = 0x8263618C; continue 'dispatch;
	}
	// 8263611C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82636120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636124: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82636128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263612C: 390940B8  addi r8, r9, 0x40b8
	ctx.r[8].s64 = ctx.r[9].s64 + 16568;
	// 82636130: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636134: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636138: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263613C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82636140: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82636144: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82636148: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263614C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82636150: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82636154: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82636158: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263615C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82636160: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82636164: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82636168: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8263616C: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82636170: 99630044  stb r11, 0x44(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 82636174: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82636178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263617C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636188: 4E800020  blr
	return;
	// 8263618C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263619C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826361A0: 4E800020  blr
	return;
}

pub fn sub_826361A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826361A8 size=144
	// 826361A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826361AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826361B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826361B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826361B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826361BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826361C0: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 826361C4: 4BBE9095  bl 0x8221f258
	ctx.lr = 0x826361C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826361C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826361CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826361D0: 419A0048  beq cr6, 0x82636218
	if ctx.cr[6].eq {
	pc = 0x82636218; continue 'dispatch;
	}
	// 826361D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 826361D8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826361DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826361E0: 392B43C0  addi r9, r11, 0x43c0
	ctx.r[9].s64 = ctx.r[11].s64 + 17344;
	// 826361E4: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 826361E8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 826361EC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826361F0: 4BE4A049  bl 0x82480238
	ctx.lr = 0x826361F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 826361F4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 826361F8: 4BE4A041  bl 0x82480238
	ctx.lr = 0x826361FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 826361FC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82636200: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82636204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636208: 98FF0024  stb r7, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 8263620C: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636210: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82636214: 48000008  b 0x8263621c
	pc = 0x8263621C; continue 'dispatch;
	// 82636218: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263621C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263622C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636230: 4E800020  blr
	return;
}

pub fn sub_82636238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636238 size=184
	// 82636238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263623C: 486731D1  bl 0x82ca940c
	ctx.lr = 0x82636240;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82636240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636244: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82636248: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8263624C: 4BBE900D  bl 0x8221f258
	ctx.lr = 0x82636250;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636254: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636258: 419A0088  beq cr6, 0x826362e0
	if ctx.cr[6].eq {
	pc = 0x826362E0; continue 'dispatch;
	}
	// 8263625C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82636260: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82636264: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82636268: 394B29F8  addi r10, r11, 0x29f8
	ctx.r[10].s64 = ctx.r[11].s64 + 10744;
	// 8263626C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82636270: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82636274: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636278: 4BBE8FE1  bl 0x8221f258
	ctx.lr = 0x8263627C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263627C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82636280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82636284: 419A0008  beq cr6, 0x8263628c
	if ctx.cr[6].eq {
	pc = 0x8263628C; continue 'dispatch;
	}
	// 82636288: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8263628C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82636290: 41820008  beq 0x82636298
	if ctx.cr[0].eq {
	pc = 0x82636298; continue 'dispatch;
	}
	// 82636294: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82636298: 354B0008  addic. r10, r11, 8
	ctx.xer.ca = (ctx.r[11].u32 > (!(8 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8263629C: 41820008  beq 0x826362a4
	if ctx.cr[0].eq {
	pc = 0x826362A4; continue 'dispatch;
	}
	// 826362A0: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 826362A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826362A8: 9BCB0021  stb r30, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[30].u8 ) };
	// 826362AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826362B0: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 826362B4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826362B8: 994B0021  stb r10, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 826362BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362C0: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 826362C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362C8: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826362CC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362D0: 91290008  stw r9, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 826362D4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826362D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826362DC: 48673180  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826362E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826362E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826362E8: 48673174  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826362F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826362F0 size=192
	// 826362F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826362F4: 48673119  bl 0x82ca940c
	ctx.lr = 0x826362F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826362F8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
}

pub fn sub_826363B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826363B0 size=232
	// 826363B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826363B4: 48673059  bl 0x82ca940c
	ctx.lr = 0x826363B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826363B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826363BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826363C0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 826363C4: 4BBE8E95  bl 0x8221f258
	ctx.lr = 0x826363C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826363C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826363CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826363D0: 419A00BC  beq cr6, 0x8263648c
	if ctx.cr[6].eq {
	pc = 0x8263648C; continue 'dispatch;
	}
	// 826363D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 826363D8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826363DC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 826363E0: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 826363E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826363E8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 826363EC: 38EB04D4  addi r7, r11, 0x4d4
	ctx.r[7].s64 = ctx.r[11].s64 + 1236;
	// 826363F0: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826363F4: 38CA6218  addi r6, r10, 0x6218
	ctx.r[6].s64 = ctx.r[10].s64 + 25112;
	// 826363F8: 38A96258  addi r5, r9, 0x6258
	ctx.r[5].s64 = ctx.r[9].s64 + 25176;
	// 826363FC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82636400: 3BA80B7C  addi r29, r8, 0xb7c
	ctx.r[29].s64 = ctx.r[8].s64 + 2940;
	// 82636404: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82636408: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8263640C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82636410: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82636414: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82636418: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8263641C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82636420: 4BB5DA19  bl 0x82193e38
	ctx.lr = 0x82636424;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 82636424: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 82636428: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8263642C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82636430: 39440E0C  addi r10, r4, 0xe0c
	ctx.r[10].s64 = ctx.r[4].s64 + 3596;
	// 82636434: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82636438: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8263643C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636440: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82636444: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82636448: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8263644C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636450: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82636454: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82636458: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 8263645C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82636460: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82636464: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82636468: D01F0054  stfs f0, 0x54(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8263646C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82636470: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82636474: 9BDF005C  stb r30, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u8 ) };
	// 82636478: 89090090  lbz r8, 0x90(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 8263647C: 61070001  ori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 | 1;
	// 82636480: 98E90090  stb r7, 0x90(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82636484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636488: 48672FD4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 8263648C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636494: 48672FC8  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82636498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636498 size=136
	// 82636498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826364A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826364A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826364A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826364AC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 826364B0: 4BBE8DA9  bl 0x8221f258
	ctx.lr = 0x826364B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826364B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826364B8: 419A004C  beq cr6, 0x82636504
	if ctx.cr[6].eq {
	pc = 0x82636504; continue 'dispatch;
	}
	// 826364BC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826364C0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 826364C4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 826364C8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826364CC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 826364D0: 392B6268  addi r9, r11, 0x6268
	ctx.r[9].s64 = ctx.r[11].s64 + 25192;
	// 826364D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826364D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826364DC: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 826364E0: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
}

pub fn sub_82636520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636520 size=152
	// 82636520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263652C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636534: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636538: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 8263653C: 4BBE8D1D  bl 0x8221f258
	ctx.lr = 0x82636540;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636544: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636548: 419A0050  beq cr6, 0x82636598
	if ctx.cr[6].eq {
	pc = 0x82636598; continue 'dispatch;
	}
	// 8263654C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82636550: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82636554: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82636558: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263655C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636560: 390A62A8  addi r8, r10, 0x62a8
	ctx.r[8].s64 = ctx.r[10].s64 + 25256;
	// 82636564: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636568: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8263656C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
}

pub fn sub_826365B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826365B8 size=176
	// 826365B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826365BC: 48672E51  bl 0x82ca940c
	ctx.lr = 0x826365C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826365C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826365C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826365C8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 826365CC: 4BBE8C8D  bl 0x8221f258
	ctx.lr = 0x826365D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826365D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826365D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826365D8: 419A0080  beq cr6, 0x82636658
	if ctx.cr[6].eq {
	pc = 0x82636658; continue 'dispatch;
	}
	// 826365DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 826365E0: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826365E4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 826365E8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826365EC: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 826365F0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 826365F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826365F8: 38EA8498  addi r7, r10, -0x7b68
	ctx.r[7].s64 = ctx.r[10].s64 + -31592;
	// 826365FC: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82636600: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82636604: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82636608: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8263660C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636610: 38A60B7C  addi r5, r6, 0xb7c
	ctx.r[5].s64 = ctx.r[6].s64 + 2940;
}

pub fn sub_82636668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636668 size=176
	// 82636668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263667C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82636680: 4BBE8BD9  bl 0x8221f258
	ctx.lr = 0x82636684;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636684: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636688: 419A0078  beq cr6, 0x82636700
	if ctx.cr[6].eq {
	pc = 0x82636700; continue 'dispatch;
	}
	// 8263668C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82636690: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636694: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82636698: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8263669C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826366A0: 38EA04D4  addi r7, r10, 0x4d4
	ctx.r[7].s64 = ctx.r[10].s64 + 1236;
	// 826366A4: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 826366A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826366AC: 38A93EF8  addi r5, r9, 0x3ef8
	ctx.r[5].s64 = ctx.r[9].s64 + 16120;
	// 826366B0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826366B4: 388885C8  addi r4, r8, -0x7a38
	ctx.r[4].s64 = ctx.r[8].s64 + -31288;
	// 826366B8: 39460E0C  addi r10, r6, 0xe0c
	ctx.r[10].s64 = ctx.r[6].s64 + 3596;
	// 826366BC: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 826366C0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 826366C4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826366C8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826366CC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 826366D0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 826366D4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826366D8: 99230020  stb r9, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u8 ) };
	// 826366DC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 826366E0: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 826366E4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826366E8: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 826366EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826366F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826366F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826366F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826366FC: 4E800020  blr
	return;
	// 82636700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263670C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636714: 4E800020  blr
	return;
}

pub fn sub_82636718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636718 size=144
	// 82636718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263671C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263672C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82636730: 4BBE8B29  bl 0x8221f258
	ctx.lr = 0x82636734;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636734: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636738: 419A0058  beq cr6, 0x82636790
	if ctx.cr[6].eq {
	pc = 0x82636790; continue 'dispatch;
	}
	// 8263673C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82636740: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636744: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 82636748: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8263674C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636750: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 82636754: 38CA04D4  addi r6, r10, 0x4d4
	ctx.r[6].s64 = ctx.r[10].s64 + 1236;
	// 82636758: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263675C: 38A985D8  addi r5, r9, -0x7a28
	ctx.r[5].s64 = ctx.r[9].s64 + -31272;
	// 82636760: 38888618  addi r4, r8, -0x79e8
	ctx.r[4].s64 = ctx.r[8].s64 + -31208;
	// 82636764: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82636768: 39470E0C  addi r10, r7, 0xe0c
	ctx.r[10].s64 = ctx.r[7].s64 + 3596;
	// 8263676C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82636770: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82636774: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82636778: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263677C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263678C: 4E800020  blr
	return;
	// 82636790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263679C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826367A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826367A4: 4E800020  blr
	return;
}

pub fn sub_826367A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826367A8 size=88
	// 826367A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826367AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826367B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826367B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826367B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826367BC: 38600200  li r3, 0x200
	ctx.r[3].s64 = 512;
	// 826367C0: 4BBE8A99  bl 0x8221f258
	ctx.lr = 0x826367C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826367C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826367C8: 419A0020  beq cr6, 0x826367e8
	if ctx.cr[6].eq {
	pc = 0x826367E8; continue 'dispatch;
	}
	// 826367CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826367D0: 4BF15599  bl 0x8254bd68
	ctx.lr = 0x826367D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8254BD68);
	// 826367D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826367D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826367DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826367E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826367E4: 4E800020  blr
	return;
	// 826367E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826367EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826367F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826367F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826367F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826367FC: 4E800020  blr
	return;
}

pub fn sub_82636800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636800 size=200
	// 82636800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636804: 48672C09  bl 0x82ca940c
	ctx.lr = 0x82636808;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82636808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263680C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636810: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82636814: 4BBE8A45  bl 0x8221f258
	ctx.lr = 0x82636818;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263681C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636820: 419A0098  beq cr6, 0x826368b8
	if ctx.cr[6].eq {
	pc = 0x826368B8; continue 'dispatch;
	}
	// 82636824: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82636828: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263682C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82636830: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82636834: 390A5628  addi r8, r10, 0x5628
	ctx.r[8].s64 = ctx.r[10].s64 + 22056;
	// 82636838: 993F0008  stb r9, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 8263683C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82636840: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636844: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82636848: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8263684C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82636850: 4BBA3B61  bl 0x821da3b0
	ctx.lr = 0x82636854;
	crate::recompiler::externs::call(&mut ctx, base, 0x821DA3B0);
	// 82636854: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82636858: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8263685C: 3BCB5600  addi r30, r11, 0x5600
	ctx.r[30].s64 = ctx.r[11].s64 + 22016;
	// 82636860: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82636864: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 82636868: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8263686C: 4BBBD4ED  bl 0x821f3d58
	ctx.lr = 0x82636870;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 82636870: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82636874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82636878: 4BB7BC81  bl 0x821b24f8
	ctx.lr = 0x8263687C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B24F8);
	// 8263687C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82636880: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82636884: 483C9E45  bl 0x82a006c8
	ctx.lr = 0x82636888;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A006C8);
	// 82636888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8263688C: 419A0020  beq cr6, 0x826368ac
	if ctx.cr[6].eq {
	pc = 0x826368AC; continue 'dispatch;
	}
	// 82636890: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82636894: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82636898: 4BBA3B19  bl 0x821da3b0
	ctx.lr = 0x8263689C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821DA3B0);
	// 8263689C: 4BB7BC5D  bl 0x821b24f8
	ctx.lr = 0x826368A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B24F8);
	// 826368A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 826368A4: 4BB602DD  bl 0x82196b80
	ctx.lr = 0x826368A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82196B80);
	// 826368A8: D03F000C  stfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 826368AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826368B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826368B4: 48672BA8  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826368B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826368BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826368C0: 48672B9C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826368C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826368C8 size=104
	// 826368C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826368CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826368D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826368D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826368D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826368DC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826368E0: 4BBE8979  bl 0x8221f258
	ctx.lr = 0x826368E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826368E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826368E8: 419A0030  beq cr6, 0x82636918
	if ctx.cr[6].eq {
	pc = 0x82636918; continue 'dispatch;
	}
	// 826368EC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 826368F0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826368F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826368F8: 392A86B0  addi r9, r10, -0x7950
	ctx.r[9].s64 = ctx.r[10].s64 + -31056;
	// 826368FC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636900: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82636904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263690C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636914: 4E800020  blr
	return;
	// 82636918: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263691C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263692C: 4E800020  blr
	return;
}

pub fn sub_82636930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636930 size=16
	// 82636930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263693C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826369D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826369D0 size=104
	// 826369D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826369D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826369D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826369DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826369E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826369E4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826369E8: 4BBE8871  bl 0x8221f258
	ctx.lr = 0x826369EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826369EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826369F0: 419A0030  beq cr6, 0x82636a20
	if ctx.cr[6].eq {
	pc = 0x82636A20; continue 'dispatch;
	}
	// 826369F4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 826369F8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826369FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636A00: 392A9FD0  addi r9, r10, -0x6030
	ctx.r[9].s64 = ctx.r[10].s64 + -24624;
	// 82636A04: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636A08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82636A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636A18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636A1C: 4E800020  blr
	return;
	// 82636A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636A24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636A34: 4E800020  blr
	return;
}

pub fn sub_82636A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636A38 size=64
	// 82636A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636A48: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82636A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82636A50: 388B9A68  addi r4, r11, -0x6598
	ctx.r[4].s64 = ctx.r[11].s64 + -26008;
	// 82636A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636A58: 4BBF6479  bl 0x8222ced0
	ctx.lr = 0x82636A5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82636A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636A70: 4E800020  blr
	return;
}

pub fn sub_82636A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636A78 size=16
	// 82636A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82636AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636AF8 size=16
	// 82636AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636B00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82636B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636B78 size=88
	// 82636B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636B8C: 386000A4  li r3, 0xa4
	ctx.r[3].s64 = 164;
	// 82636B90: 4BBE86C9  bl 0x8221f258
	ctx.lr = 0x82636B94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636B94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636B98: 419A0020  beq cr6, 0x82636bb8
	if ctx.cr[6].eq {
	pc = 0x82636BB8; continue 'dispatch;
	}
	// 82636B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82636BA0: 48191061  bl 0x827c7c00
	ctx.lr = 0x82636BA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x827C7C00);
	// 82636BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636BB4: 4E800020  blr
	return;
	// 82636BB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636BCC: 4E800020  blr
	return;
}

pub fn sub_82636BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636BD0 size=104
	// 82636BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636BE4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82636BE8: 4BBE8671  bl 0x8221f258
	ctx.lr = 0x82636BEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636BF0: 419A0030  beq cr6, 0x82636c20
	if ctx.cr[6].eq {
	pc = 0x82636C20; continue 'dispatch;
	}
	// 82636BF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82636BF8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636BFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636C00: 392A3B78  addi r9, r10, 0x3b78
	ctx.r[9].s64 = ctx.r[10].s64 + 15224;
	// 82636C04: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636C08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82636C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636C18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636C1C: 4E800020  blr
	return;
	// 82636C20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636C34: 4E800020  blr
	return;
}

pub fn sub_82636C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636C38 size=176
	// 82636C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636C4C: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82636C50: 4BBE8609  bl 0x8221f258
	ctx.lr = 0x82636C54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636C58: 419A0078  beq cr6, 0x82636cd0
	if ctx.cr[6].eq {
	pc = 0x82636CD0; continue 'dispatch;
	}
	// 82636C5C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82636C60: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636C64: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82636C68: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82636C6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636C70: 38E95800  addi r7, r9, 0x5800
	ctx.r[7].s64 = ctx.r[9].s64 + 22528;
	// 82636C74: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82636C78: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636C7C: 38A80B7C  addi r5, r8, 0xb7c
	ctx.r[5].s64 = ctx.r[8].s64 + 2940;
	// 82636C80: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82636C84: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82636C88: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636C8C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82636C90: 98C30014  stb r6, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u8 ) };
	// 82636C94: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82636C98: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82636C9C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82636CA0: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82636CA4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82636CA8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82636CAC: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82636CB0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82636CB4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82636CB8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82636CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636CC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636CCC: 4E800020  blr
	return;
	// 82636CD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636CE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636CE4: 4E800020  blr
	return;
}

pub fn sub_82636CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636CE8 size=184
	// 82636CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636CF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636CF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636CFC: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82636D00: 4BBE8559  bl 0x8221f258
	ctx.lr = 0x82636D04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636D04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636D08: 419A0080  beq cr6, 0x82636d88
	if ctx.cr[6].eq {
	pc = 0x82636D88; continue 'dispatch;
	}
	// 82636D0C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82636D10: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636D14: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82636D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636D1C: 38C83570  addi r6, r8, 0x3570
	ctx.r[6].s64 = ctx.r[8].s64 + 13680;
	// 82636D20: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 82636D24: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636D28: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636D2C: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82636D30: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82636D34: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82636D38: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82636D3C: 38857088  addi r4, r5, 0x7088
	ctx.r[4].s64 = ctx.r[5].s64 + 28808;
	// 82636D40: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82636D44: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82636D48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82636D4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82636D50: 7D202028  lwarx r9, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82636D54: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82636D58: 7D20212D  stwcx. r9, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82636D5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82636D60: 4082FFE8  bne 0x82636d48
	if !ctx.cr[0].eq {
	pc = 0x82636D48; continue 'dispatch;
	}
	// 82636D64: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82636D68: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82636D6C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82636D70: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82636D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636D84: 4E800020  blr
	return;
	// 82636D88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636D8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636D9C: 4E800020  blr
	return;
}

pub fn sub_82636DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636DA0 size=128
	// 82636DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636DA4: 48672669  bl 0x82ca940c
	ctx.lr = 0x82636DA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82636DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636DB0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82636DB4: 4BBE84A5  bl 0x8221f258
	ctx.lr = 0x82636DB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636DB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636DBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636DC0: 419A0050  beq cr6, 0x82636e10
	if ctx.cr[6].eq {
	pc = 0x82636E10; continue 'dispatch;
	}
	// 82636DC4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82636DC8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82636DCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82636DD0: 394B23A8  addi r10, r11, 0x23a8
	ctx.r[10].s64 = ctx.r[11].s64 + 9128;
	// 82636DD4: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82636DD8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82636DDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636DE0: 4BBE8479  bl 0x8221f258
	ctx.lr = 0x82636DE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636DE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636DE8: 419A0008  beq cr6, 0x82636df0
	if ctx.cr[6].eq {
	pc = 0x82636DF0; continue 'dispatch;
	}
	// 82636DEC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636DF0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82636DF4: 41820008  beq 0x82636dfc
	if ctx.cr[0].eq {
	pc = 0x82636DFC; continue 'dispatch;
	}
	// 82636DF8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636DFC: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82636E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636E04: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82636E08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636E0C: 48672650  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82636E10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636E14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636E18: 48672644  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82636E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636E20 size=88
	// 82636E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636E34: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 82636E38: 4BBE8421  bl 0x8221f258
	ctx.lr = 0x82636E3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636E40: 419A0020  beq cr6, 0x82636e60
	if ctx.cr[6].eq {
	pc = 0x82636E60; continue 'dispatch;
	}
	// 82636E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82636E48: 48265CC9  bl 0x8289cb10
	ctx.lr = 0x82636E4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8289CB10);
	// 82636E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636E58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636E5C: 4E800020  blr
	return;
	// 82636E60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636E74: 4E800020  blr
	return;
}

pub fn sub_82636E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636E78 size=128
	// 82636E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636E8C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82636E90: 4BBE83C9  bl 0x8221f258
	ctx.lr = 0x82636E94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636E94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636E98: 419A0044  beq cr6, 0x82636edc
	if ctx.cr[6].eq {
	pc = 0x82636EDC; continue 'dispatch;
	}
	// 82636E9C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82636EA0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636EA8: 392A3DB0  addi r9, r10, 0x3db0
	ctx.r[9].s64 = ctx.r[10].s64 + 15792;
	// 82636EAC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636EB0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82636EB4: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82636EB8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82636EBC: 891F0090  lbz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82636EC0: 61070002  ori r7, r8, 2
	ctx.r[7].u64 = ctx.r[8].u64 | 2;
	// 82636EC4: 98FF0090  stb r7, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82636EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636ED8: 4E800020  blr
	return;
	// 82636EDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636EF0: 4E800020  blr
	return;
}

pub fn sub_82636EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636EF8 size=152
	// 82636EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82636F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636F0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636F10: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82636F14: 4BBE8345  bl 0x8221f258
	ctx.lr = 0x82636F18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636F1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636F20: 419A0050  beq cr6, 0x82636f70
	if ctx.cr[6].eq {
	pc = 0x82636F70; continue 'dispatch;
	}
	// 82636F24: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82636F28: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82636F2C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 82636F30: 390A9484  addi r8, r10, -0x6b7c
	ctx.r[8].s64 = ctx.r[10].s64 + -27516;
	// 82636F34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636F38: 38E9BD40  addi r7, r9, -0x42c0
	ctx.r[7].s64 = ctx.r[9].s64 + -17088;
	// 82636F3C: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636F40: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636F44: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82636F48: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82636F4C: C008000C  lfs f0, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636F50: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82636F54: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82636F58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82636F5C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82636F60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82636F64: 4BD56555  bl 0x8238d4b8
	ctx.lr = 0x82636F68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82636F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636F6C: 48000008  b 0x82636f74
	pc = 0x82636F74; continue 'dispatch;
	// 82636F70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82636F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636F88: 4E800020  blr
	return;
}

pub fn sub_82636F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636F90 size=128
	// 82636F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636F94: 48672479  bl 0x82ca940c
	ctx.lr = 0x82636F98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82636F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636F9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636FA0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82636FA4: 4BBE82B5  bl 0x8221f258
	ctx.lr = 0x82636FA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636FAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636FB0: 419A0050  beq cr6, 0x82637000
	if ctx.cr[6].eq {
	pc = 0x82637000; continue 'dispatch;
	}
	// 82636FB4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82636FB8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82636FBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82636FC0: 394B5C38  addi r10, r11, 0x5c38
	ctx.r[10].s64 = ctx.r[11].s64 + 23608;
	// 82636FC4: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82636FC8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82636FCC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636FD0: 4BBE8289  bl 0x8221f258
	ctx.lr = 0x82636FD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82636FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636FD8: 419A0008  beq cr6, 0x82636fe0
	if ctx.cr[6].eq {
	pc = 0x82636FE0; continue 'dispatch;
	}
	// 82636FDC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636FE0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82636FE4: 41820008  beq 0x82636fec
	if ctx.cr[0].eq {
	pc = 0x82636FEC; continue 'dispatch;
	}
	// 82636FE8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636FEC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82636FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636FF4: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82636FF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636FFC: 48672460  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82637000: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637008: 48672454  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82637010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637010 size=120
	// 82637010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263701C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637024: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82637028: 4BBE8231  bl 0x8221f258
	ctx.lr = 0x8263702C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263702C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637030: 419A003C  beq cr6, 0x8263706c
	if ctx.cr[6].eq {
	pc = 0x8263706C; continue 'dispatch;
	}
	// 82637034: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637038: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263703C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82637040: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637044: 390A5CF0  addi r8, r10, 0x5cf0
	ctx.r[8].s64 = ctx.r[10].s64 + 23792;
	// 82637048: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 8263704C: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82637050: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82637054: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82637058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263705C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637068: 4E800020  blr
	return;
	// 8263706C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263707C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637080: 4E800020  blr
	return;
}

pub fn sub_82637088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637088 size=112
	// 82637088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263709C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826370A0: 4BBE81B9  bl 0x8221f258
	ctx.lr = 0x826370A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826370A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826370A8: 419A0038  beq cr6, 0x826370e0
	if ctx.cr[6].eq {
	pc = 0x826370E0; continue 'dispatch;
	}
	// 826370AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826370B0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826370B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826370B8: 392A4400  addi r9, r10, 0x4400
	ctx.r[9].s64 = ctx.r[10].s64 + 17408;
	// 826370BC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826370C0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826370C4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826370C8: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 826370CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826370D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826370D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826370D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826370DC: 4E800020  blr
	return;
	// 826370E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826370E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826370E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826370EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826370F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826370F4: 4E800020  blr
	return;
}

pub fn sub_826370F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826370F8 size=104
	// 826370F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826370FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263710C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82637110: 4BBE8149  bl 0x8221f258
	ctx.lr = 0x82637114;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637118: 419A0030  beq cr6, 0x82637148
	if ctx.cr[6].eq {
	pc = 0x82637148; continue 'dispatch;
	}
	// 8263711C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637128: 392A6220  addi r9, r10, 0x6220
	ctx.r[9].s64 = ctx.r[10].s64 + 25120;
	// 8263712C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637130: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263713C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637144: 4E800020  blr
	return;
	// 82637148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263714C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263715C: 4E800020  blr
	return;
}

pub fn sub_82637160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637160 size=104
	// 82637160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263716C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637174: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82637178: 4BBE80E1  bl 0x8221f258
	ctx.lr = 0x8263717C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263717C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637180: 419A0030  beq cr6, 0x826371b0
	if ctx.cr[6].eq {
	pc = 0x826371B0; continue 'dispatch;
	}
	// 82637184: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637188: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263718C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637190: 392A62B0  addi r9, r10, 0x62b0
	ctx.r[9].s64 = ctx.r[10].s64 + 25264;
	// 82637194: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637198: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263719C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826371A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826371A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826371A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826371AC: 4E800020  blr
	return;
	// 826371B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826371B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826371B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826371BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826371C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826371C4: 4E800020  blr
	return;
}

pub fn sub_826371C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826371C8 size=112
	// 826371C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826371CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826371D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826371D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826371D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826371DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826371E0: 4BBE8079  bl 0x8221f258
	ctx.lr = 0x826371E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826371E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826371E8: 419A0038  beq cr6, 0x82637220
	if ctx.cr[6].eq {
	pc = 0x82637220; continue 'dispatch;
	}
	// 826371EC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 826371F0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826371F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826371F8: 392B6440  addi r9, r11, 0x6440
	ctx.r[9].s64 = ctx.r[11].s64 + 25664;
	// 826371FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82637200: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82637204: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637208: 9903000C  stb r8, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 8263720C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263721C: 4E800020  blr
	return;
	// 82637220: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263722C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637230: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637234: 4E800020  blr
	return;
}

pub fn sub_82637238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637238 size=136
	// 82637238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263723C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263724C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82637250: 4BBE8009  bl 0x8221f258
	ctx.lr = 0x82637254;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637254: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637258: 419A0044  beq cr6, 0x8263729c
	if ctx.cr[6].eq {
	pc = 0x8263729C; continue 'dispatch;
	}
	// 8263725C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82637260: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637268: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8263726C: 390938F8  addi r8, r9, 0x38f8
	ctx.r[8].s64 = ctx.r[9].s64 + 14584;
	// 82637270: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637274: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82637278: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263727C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82637280: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82637284: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82637288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263728C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637298: 4E800020  blr
	return;
	// 8263729C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826372A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826372A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826372A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826372AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826372B0: 4E800020  blr
	return;
}

pub fn sub_826372C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826372C0 size=112
	// 826372C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826372C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826372C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826372CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826372D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826372D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826372D8: 4BBE7F81  bl 0x8221f258
	ctx.lr = 0x826372DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826372DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826372E0: 419A0034  beq cr6, 0x82637314
	if ctx.cr[6].eq {
	pc = 0x82637314; continue 'dispatch;
	}
	// 826372E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826372E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826372EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826372F0: 392A34B0  addi r9, r10, 0x34b0
	ctx.r[9].s64 = ctx.r[10].s64 + 13488;
	// 826372F4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826372F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826372FC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82637300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263730C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637310: 4E800020  blr
	return;
	// 82637314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263731C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637328: 4E800020  blr
	return;
}

pub fn sub_82637330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637330 size=64
	// 82637330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263733C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637340: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82637344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82637348: 388B9B7C  addi r4, r11, -0x6484
	ctx.r[4].s64 = ctx.r[11].s64 + -25732;
	// 8263734C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637350: 4BBF5B81  bl 0x8222ced0
	ctx.lr = 0x82637354;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82637354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263735C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637368: 4E800020  blr
	return;
}

pub fn sub_82637370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637370 size=80
	// 82637370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263737C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637388: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8263738C: 482E13A5  bl 0x82918730
	ctx.lr = 0x82637390;
	crate::recompiler::externs::call(&mut ctx, base, 0x82918730);
	// 82637390: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82637394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263739C: 419A000C  beq cr6, 0x826373a8
	if ctx.cr[6].eq {
	pc = 0x826373A8; continue 'dispatch;
	}
	// 826373A0: 4BBE4999  bl 0x8221bd38
	ctx.lr = 0x826373A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 826373A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826373A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826373AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826373B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826373B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826373B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826373BC: 4E800020  blr
	return;
}

pub fn sub_826373C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826373C0 size=120
	// 826373C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826373C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826373C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826373CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826373D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826373D4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 826373D8: 4BBE7E81  bl 0x8221f258
	ctx.lr = 0x826373DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826373DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826373E0: 419A003C  beq cr6, 0x8263741c
	if ctx.cr[6].eq {
	pc = 0x8263741C; continue 'dispatch;
	}
	// 826373E4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 826373E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826373EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826373F0: 392AA0D0  addi r9, r10, -0x5f30
	ctx.r[9].s64 = ctx.r[10].s64 + -24368;
	// 826373F4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826373F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826373FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82637400: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82637404: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82637408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637418: 4E800020  blr
	return;
	// 8263741C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263742C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637430: 4E800020  blr
	return;
}

pub fn sub_82637438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637438 size=64
	// 82637438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637448: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263744C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82637450: 388B9B90  addi r4, r11, -0x6470
	ctx.r[4].s64 = ctx.r[11].s64 + -25712;
	// 82637454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637458: 4BBF5A79  bl 0x8222ced0
	ctx.lr = 0x8263745C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263745C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263746C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637470: 4E800020  blr
	return;
}

pub fn sub_82637478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637478 size=20
	// 82637478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263747C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82637484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826374D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826374D8 size=104
	// 826374D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826374DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826374E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826374E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826374E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826374EC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826374F0: 4BBE7D69  bl 0x8221f258
	ctx.lr = 0x826374F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826374F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826374F8: 419A0030  beq cr6, 0x82637528
	if ctx.cr[6].eq {
	pc = 0x82637528; continue 'dispatch;
	}
	// 826374FC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637500: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637508: 392A6008  addi r9, r10, 0x6008
	ctx.r[9].s64 = ctx.r[10].s64 + 24584;
	// 8263750C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637510: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637524: 4E800020  blr
	return;
	// 82637528: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263752C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263753C: 4E800020  blr
	return;
}

pub fn sub_82637540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637540 size=104
	// 82637540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263754C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637554: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82637558: 4BBE7D01  bl 0x8221f258
	ctx.lr = 0x8263755C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263755C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637560: 419A0030  beq cr6, 0x82637590
	if ctx.cr[6].eq {
	pc = 0x82637590; continue 'dispatch;
	}
	// 82637564: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82637568: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263756C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637570: 392A3BB8  addi r9, r10, 0x3bb8
	ctx.r[9].s64 = ctx.r[10].s64 + 15288;
	// 82637574: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637578: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263757C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263758C: 4E800020  blr
	return;
	// 82637590: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263759C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826375A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826375A4: 4E800020  blr
	return;
}

pub fn sub_826375A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826375A8 size=88
	// 826375A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826375AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826375B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826375B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826375B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826375BC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 826375C0: 4BBE7C99  bl 0x8221f258
	ctx.lr = 0x826375C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826375C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826375C8: 419A0020  beq cr6, 0x826375e8
	if ctx.cr[6].eq {
	pc = 0x826375E8; continue 'dispatch;
	}
	// 826375CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826375D0: 480BB621  bl 0x826f2bf0
	ctx.lr = 0x826375D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x826F2BF0);
	// 826375D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826375D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826375DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826375E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826375E4: 4E800020  blr
	return;
	// 826375E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826375EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826375F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826375F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826375F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826375FC: 4E800020  blr
	return;
}

pub fn sub_82637600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637600 size=120
	// 82637600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263760C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637614: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82637618: 4BBE7C41  bl 0x8221f258
	ctx.lr = 0x8263761C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263761C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637620: 419A003C  beq cr6, 0x8263765c
	if ctx.cr[6].eq {
	pc = 0x8263765C; continue 'dispatch;
	}
	// 82637624: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82637628: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263762C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82637630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637634: 39097688  addi r8, r9, 0x7688
	ctx.r[8].s64 = ctx.r[9].s64 + 30344;
	// 82637638: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263763C: C00ABE04  lfs f0, -0x41fc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16892 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637640: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82637644: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82637648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263764C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637658: 4E800020  blr
	return;
	// 8263765C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263766C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637670: 4E800020  blr
	return;
}

pub fn sub_82637678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637678 size=208
	// 82637678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263767C: 48671D91  bl 0x82ca940c
	ctx.lr = 0x82637680;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82637680: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637684: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82637688: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8263768C: 4BBE7BCD  bl 0x8221f258
	ctx.lr = 0x82637690;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637694: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82637698: 419A00A0  beq cr6, 0x82637738
	if ctx.cr[6].eq {
	pc = 0x82637738; continue 'dispatch;
	}
	// 8263769C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 826376A0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826376A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826376A8: 394B3DB8  addi r10, r11, 0x3db8
	ctx.r[10].s64 = ctx.r[11].s64 + 15800;
	// 826376AC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826376B0: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826376B4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826376B8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 826376BC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 826376C0: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 826376C4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 826376C8: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 826376CC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826376D0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 826376D4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 826376D8: C00992D4  lfs f0, -0x6d2c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826376DC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 826376E0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 826376E4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 826376E8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 826376EC: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 826376F0: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 826376F4: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 826376F8: 90FF0038  stw r7, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[7].u32 ) };
	// 826376FC: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82637700: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82637704: 90DF0040  stw r6, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[6].u32 ) };
	// 82637708: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8263770C: 90DF0048  stw r6, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[6].u32 ) };
	// 82637710: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82637714: 90DF0050  stw r6, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82637718: 4BB5C721  bl 0x82193e38
	ctx.lr = 0x8263771C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 8263771C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82637720: 9BDF005C  stb r30, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u8 ) };
	// 82637724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637728: 9BDF005D  stb r30, 0x5d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(93 as u32), ctx.r[30].u8 ) };
	// 8263772C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82637730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637734: 48671D28  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82637738: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263773C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637740: 48671D1C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82637748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637748 size=104
	// 82637748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263775C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82637760: 4BBE7AF9  bl 0x8221f258
	ctx.lr = 0x82637764;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637764: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637768: 419A0030  beq cr6, 0x82637798
	if ctx.cr[6].eq {
	pc = 0x82637798; continue 'dispatch;
	}
	// 8263776C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637770: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637778: 392A64F8  addi r9, r10, 0x64f8
	ctx.r[9].s64 = ctx.r[10].s64 + 25848;
	// 8263777C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637780: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263778C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637794: 4E800020  blr
	return;
	// 82637798: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263779C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826377A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826377A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826377A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826377AC: 4E800020  blr
	return;
}

pub fn sub_826377B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826377B0 size=208
	// 826377B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826377B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826377B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826377BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826377C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826377C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826377C8: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826377CC: 4BBE7A8D  bl 0x8221f258
	ctx.lr = 0x826377D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826377D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826377D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826377D8: 419A0088  beq cr6, 0x82637860
	if ctx.cr[6].eq {
	pc = 0x82637860; continue 'dispatch;
	}
	// 826377DC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 826377E0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826377E4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 826377E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826377EC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826377F0: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 826377F4: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826377F8: 38C73670  addi r6, r7, 0x3670
	ctx.r[6].s64 = ctx.r[7].s64 + 13936;
	// 826377FC: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637800: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82637804: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82637808: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8263780C: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82637810: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82637814: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82637818: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8263781C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82637820: D01F0064  stfs f0, 0x64(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82637824: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82637828: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8263782C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637830: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82637834: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82637838: 90BF005C  stw r5, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 8263783C: 993F0068  stb r9, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u8 ) };
	// 82637840: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82637844: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82637848: D01F0074  stfs f0, 0x74(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8263784C: D01F0078  stfs f0, 0x78(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82637850: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82637854: 4BD55C65  bl 0x8238d4b8
	ctx.lr = 0x82637858;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82637858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263785C: 48000008  b 0x82637864
	pc = 0x82637864; continue 'dispatch;
	// 82637860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263786C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82637874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637878: 4E800020  blr
	return;
}

pub fn sub_82637880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637880 size=88
	// 82637880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263788C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637894: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82637898: 4BBE79C1  bl 0x8221f258
	ctx.lr = 0x8263789C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263789C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826378A0: 419A0020  beq cr6, 0x826378c0
	if ctx.cr[6].eq {
	pc = 0x826378C0; continue 'dispatch;
	}
	// 826378A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826378A8: 4830DFD1  bl 0x82945878
	ctx.lr = 0x826378AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82945878);
	// 826378AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826378B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826378B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826378B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826378BC: 4E800020  blr
	return;
	// 826378C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826378C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826378C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826378CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826378D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826378D4: 4E800020  blr
	return;
}

pub fn sub_826378D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826378D8 size=136
	// 826378D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826378DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826378E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826378E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826378E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826378EC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 826378F0: 4BBE7969  bl 0x8221f258
	ctx.lr = 0x826378F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826378F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826378F8: 419A0050  beq cr6, 0x82637948
	if ctx.cr[6].eq {
	pc = 0x82637948; continue 'dispatch;
	}
	// 826378FC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82637900: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637904: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82637908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263790C: 39097B78  addi r8, r9, 0x7b78
	ctx.r[8].s64 = ctx.r[9].s64 + 31608;
	// 82637910: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637914: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637918: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263791C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82637920: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82637924: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82637928: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263792C: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82637930: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82637934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263793C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637944: 4E800020  blr
	return;
	// 82637948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263794C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263795C: 4E800020  blr
	return;
}

pub fn sub_82637960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637960 size=120
	// 82637960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637964: 48671AA9  bl 0x82ca940c
	ctx.lr = 0x82637968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82637968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263796C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82637970: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82637974: 4BBE78E5  bl 0x8221f258
	ctx.lr = 0x82637978;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263797C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82637980: 419A0048  beq cr6, 0x826379c8
	if ctx.cr[6].eq {
	pc = 0x826379C8; continue 'dispatch;
	}
	// 82637984: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82637988: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263798C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82637990: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82637994: 392B65D8  addi r9, r11, 0x65d8
	ctx.r[9].s64 = ctx.r[11].s64 + 26072;
	// 82637998: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263799C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826379A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826379A4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 826379A8: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 826379AC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 826379B0: 4BB5C489  bl 0x82193e38
	ctx.lr = 0x826379B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 826379B4: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 826379B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826379BC: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 826379C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826379C4: 48671A98  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826379C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826379CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826379D0: 48671A8C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826379D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826379D8 size=104
	// 826379D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826379DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826379E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826379E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826379E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826379EC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826379F0: 4BBE7869  bl 0x8221f258
	ctx.lr = 0x826379F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826379F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826379F8: 419A0030  beq cr6, 0x82637a28
	if ctx.cr[6].eq {
	pc = 0x82637A28; continue 'dispatch;
	}
	// 826379FC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82637A00: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637A04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637A08: 392AA110  addi r9, r10, -0x5ef0
	ctx.r[9].s64 = ctx.r[10].s64 + -24304;
	// 82637A0C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637A10: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637A24: 4E800020  blr
	return;
	// 82637A28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637A3C: 4E800020  blr
	return;
}

pub fn sub_82637A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637A40 size=64
	// 82637A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637A50: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82637A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82637A58: 388BBAB0  addi r4, r11, -0x4550
	ctx.r[4].s64 = ctx.r[11].s64 + -17744;
	// 82637A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637A60: 4BBF5471  bl 0x8222ced0
	ctx.lr = 0x82637A64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82637A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637A78: 4E800020  blr
	return;
}

pub fn sub_82637A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637A80 size=88
	// 82637A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637A8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637A94: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82637A98: 4BBE77C1  bl 0x8221f258
	ctx.lr = 0x82637A9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637AA0: 419A0020  beq cr6, 0x82637ac0
	if ctx.cr[6].eq {
	pc = 0x82637AC0; continue 'dispatch;
	}
	// 82637AA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637AA8: 4BF55999  bl 0x8258d440
	ctx.lr = 0x82637AAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8258D440);
	// 82637AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637ABC: 4E800020  blr
	return;
	// 82637AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637AD4: 4E800020  blr
	return;
}

pub fn sub_82637AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637AD8 size=104
	// 82637AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637AE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637AE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637AEC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82637AF0: 4BBE7769  bl 0x8221f258
	ctx.lr = 0x82637AF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637AF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637AF8: 419A0030  beq cr6, 0x82637b28
	if ctx.cr[6].eq {
	pc = 0x82637B28; continue 'dispatch;
	}
	// 82637AFC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637B00: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637B08: 392A3CB8  addi r9, r10, 0x3cb8
	ctx.r[9].s64 = ctx.r[10].s64 + 15544;
	// 82637B0C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637B10: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637B14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637B20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637B24: 4E800020  blr
	return;
	// 82637B28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637B3C: 4E800020  blr
	return;
}

pub fn sub_82637B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637B40 size=136
	// 82637B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82637B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637B54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82637B58: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82637B5C: 4BBE76FD  bl 0x8221f258
	ctx.lr = 0x82637B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637B64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82637B68: 419A0044  beq cr6, 0x82637bac
	if ctx.cr[6].eq {
	pc = 0x82637BAC; continue 'dispatch;
	}
	// 82637B6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82637B70: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82637B74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82637B78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82637B7C: 390A3FB8  addi r8, r10, 0x3fb8
	ctx.r[8].s64 = ctx.r[10].s64 + 16312;
	// 82637B80: 993F0008  stb r9, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82637B84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637B88: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637B8C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82637B90: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82637B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82637B98: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82637B9C: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82637BA0: 4BD55919  bl 0x8238d4b8
	ctx.lr = 0x82637BA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82637BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637BA8: 48000008  b 0x82637bb0
	pc = 0x82637BB0; continue 'dispatch;
	// 82637BAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82637BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637BC4: 4E800020  blr
	return;
}

pub fn sub_82637BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637BC8 size=176
	// 82637BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637BD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82637BD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637BD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637BDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82637BE0: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82637BE4: 4BBE7675  bl 0x8221f258
	ctx.lr = 0x82637BE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637BEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82637BF0: 419A0068  beq cr6, 0x82637c58
	if ctx.cr[6].eq {
	pc = 0x82637C58; continue 'dispatch;
	}
	// 82637BF4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82637BF8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82637BFC: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82637C00: 390AB7A4  addi r8, r10, -0x485c
	ctx.r[8].s64 = ctx.r[10].s64 + -18524;
	// 82637C04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637C08: 38E96A88  addi r7, r9, 0x6a88
	ctx.r[7].s64 = ctx.r[9].s64 + 27272;
	// 82637C0C: C00AB7A4  lfs f0, -0x485c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637C10: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82637C14: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82637C18: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637C1C: C008DCE0  lfs f0, -0x2320(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8992 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637C20: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82637C24: C1A8FEAC  lfs f13, -0x154(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82637C28: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82637C2C: C188DB28  lfs f12, -0x24d8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-9432 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82637C30: 98DF0020  stb r6, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u8 ) };
	// 82637C34: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82637C38: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82637C3C: D1BF0010  stfs f13, 0x10(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82637C40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637C44: D19F0014  stfs f12, 0x14(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82637C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82637C4C: 4BD5586D  bl 0x8238d4b8
	ctx.lr = 0x82637C50;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82637C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637C54: 48000008  b 0x82637c5c
	pc = 0x82637C5C; continue 'dispatch;
	// 82637C58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637C5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637C68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82637C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637C70: 4E800020  blr
	return;
}

pub fn sub_82637C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637C78 size=88
	// 82637C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637C84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637C88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637C8C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82637C90: 4BBE75C9  bl 0x8221f258
	ctx.lr = 0x82637C94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637C94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637C98: 419A0020  beq cr6, 0x82637cb8
	if ctx.cr[6].eq {
	pc = 0x82637CB8; continue 'dispatch;
	}
	// 82637C9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637CA0: 4BF03589  bl 0x8253b228
	ctx.lr = 0x82637CA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8253B228);
	// 82637CA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637CB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637CB4: 4E800020  blr
	return;
	// 82637CB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637CC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637CCC: 4E800020  blr
	return;
}

pub fn sub_82637CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637CD0 size=176
	// 82637CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637CD4: 48671739  bl 0x82ca940c
	ctx.lr = 0x82637CD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82637CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637CDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82637CE0: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82637CE4: 4BBE7575  bl 0x8221f258
	ctx.lr = 0x82637CE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637CEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82637CF0: 419A0080  beq cr6, 0x82637d70
	if ctx.cr[6].eq {
	pc = 0x82637D70; continue 'dispatch;
	}
	// 82637CF4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82637CF8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82637CFC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82637D00: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82637D04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82637D08: 390BDE04  addi r8, r11, -0x21fc
	ctx.r[8].s64 = ctx.r[11].s64 + -8700;
	// 82637D0C: 38EA4500  addi r7, r10, 0x4500
	ctx.r[7].s64 = ctx.r[10].s64 + 17664;
	// 82637D10: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82637D14: 38C910EC  addi r6, r9, 0x10ec
	ctx.r[6].s64 = ctx.r[9].s64 + 4332;
	// 82637D18: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82637D1C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82637D20: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82637D24: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82637D28: 483E0C19  bl 0x82a18940
	ctx.lr = 0x82637D2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A18940);
	// 82637D2C: 3CA0820A  lis r5, -0x7df6
	ctx.r[5].s64 = -2113273856;
	// 82637D30: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 82637D34: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82637D38: 3965B47C  addi r11, r5, -0x4b84
	ctx.r[11].s64 = ctx.r[5].s64 + -19332;
	// 82637D3C: 39440E0C  addi r10, r4, 0xe0c
	ctx.r[10].s64 = ctx.r[4].s64 + 3596;
	// 82637D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82637D44: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82637D48: C185B47C  lfs f12, -0x4b84(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-19332 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82637D4C: D19F0034  stfs f12, 0x34(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82637D50: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 82637D54: C00BE008  lfs f0, -0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637D58: 9BDF0029  stb r30, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 82637D5C: C1AB3D4C  lfs f13, 0x3d4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15692 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82637D60: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82637D64: D1BF0030  stfs f13, 0x30(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82637D68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637D6C: 486716F0  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 82637D70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82637D78: 486716E4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82637D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637D80 size=88
	// 82637D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637D94: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82637D98: 4BBE74C1  bl 0x8221f258
	ctx.lr = 0x82637D9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637DA0: 419A0020  beq cr6, 0x82637dc0
	if ctx.cr[6].eq {
	pc = 0x82637DC0; continue 'dispatch;
	}
	// 82637DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82637DA8: 480EA831  bl 0x827225d8
	ctx.lr = 0x82637DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x827225D8);
	// 82637DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637DBC: 4E800020  blr
	return;
	// 82637DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637DD4: 4E800020  blr
	return;
}

pub fn sub_82637DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637DD8 size=144
	// 82637DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637DEC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82637DF0: 4BBE7469  bl 0x8221f258
	ctx.lr = 0x82637DF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637DF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637DF8: 419A0054  beq cr6, 0x82637e4c
	if ctx.cr[6].eq {
	pc = 0x82637E4C; continue 'dispatch;
	}
	// 82637DFC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637E00: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637E08: 392A8AB8  addi r9, r10, -0x7548
	ctx.r[9].s64 = ctx.r[10].s64 + -30024;
	// 82637E0C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637E10: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637E14: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82637E18: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82637E1C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82637E20: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82637E24: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82637E28: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82637E2C: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 82637E30: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 82637E34: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 82637E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637E48: 4E800020  blr
	return;
	// 82637E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637E5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637E60: 4E800020  blr
	return;
}

pub fn sub_82637E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82637E68 size=168
	// 82637E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637E7C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82637E80: 4BBE73D9  bl 0x8221f258
	ctx.lr = 0x82637E84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637E84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637E88: 419A0070  beq cr6, 0x82637ef8
	if ctx.cr[6].eq {
	pc = 0x82637EF8; continue 'dispatch;
	}
	// 82637E8C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82637E90: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82637E94: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82637E98: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637E9C: 390A9484  addi r8, r10, -0x6b7c
	ctx.r[8].s64 = ctx.r[10].s64 + -27516;
	// 82637EA0: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82637EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637EA8: 38C97020  addi r6, r9, 0x7020
	ctx.r[6].s64 = ctx.r[9].s64 + 28704;
	// 82637EAC: C18A9484  lfs f12, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82637EB0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82637EB4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637EB8: C0080018  lfs f0, 0x18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82637EBC: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82637EC0: C1A822AC  lfs f13, 0x22ac(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8876 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82637EC4: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82637EC8: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82637ECC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82637ED0: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
}

pub fn sub_82637F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637F10 size=120
	// 82637F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637F24: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82637F28: 4BBE7331  bl 0x8221f258
	ctx.lr = 0x82637F2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637F2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637F30: 419A0040  beq cr6, 0x82637f70
	if ctx.cr[6].eq {
	pc = 0x82637F70; continue 'dispatch;
	}
	// 82637F34: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637F38: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637F40: 392A7138  addi r9, r10, 0x7138
	ctx.r[9].s64 = ctx.r[10].s64 + 28984;
	// 82637F44: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637F48: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82637F4C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82637F50: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82637F54: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82637F58: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82637F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637F6C: 4E800020  blr
	return;
	// 82637F70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82637F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82637F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82637F80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82637F84: 4E800020  blr
	return;
}

pub fn sub_82637F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82637F88 size=160
	// 82637F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82637F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82637F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82637F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82637F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82637F9C: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82637FA0: 4BBE72B9  bl 0x8221f258
	ctx.lr = 0x82637FA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82637FA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82637FA8: 419A0064  beq cr6, 0x8263800c
	if ctx.cr[6].eq {
	pc = 0x8263800C; continue 'dispatch;
	}
	// 82637FAC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82637FB0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82637FB4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82637FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82637FBC: 390A71F8  addi r8, r10, 0x71f8
	ctx.r[8].s64 = ctx.r[10].s64 + 29176;
	// 82637FC0: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82637FC4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82637FC8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82637FCC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82637FD0: 90E30020  stw r7, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82637FD4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82637FD8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82637FDC: 90E30028  stw r7, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 82637FE0: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82637FE4: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82637FE8: 80C30004  lwz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82637FEC: 88A60090  lbz r5, 0x90(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 82637FF0: 60A40001  ori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 | 1;
	// 82637FF4: 98860090  stb r4, 0x90(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[4].u8 ) };
	// 82637FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82637FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638008: 4E800020  blr
	return;
	// 8263800C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263801C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638020: 4E800020  blr
	return;
}

pub fn sub_82638028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638028 size=88
	// 82638028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263803C: 386001A0  li r3, 0x1a0
	ctx.r[3].s64 = 416;
	// 82638040: 4BBE7219  bl 0x8221f258
	ctx.lr = 0x82638044;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638044: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638048: 419A0020  beq cr6, 0x82638068
	if ctx.cr[6].eq {
	pc = 0x82638068; continue 'dispatch;
	}
	// 8263804C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82638050: 4805D489  bl 0x826954d8
	ctx.lr = 0x82638054;
	crate::recompiler::externs::call(&mut ctx, base, 0x826954D8);
	// 82638054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263805C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638064: 4E800020  blr
	return;
	// 82638068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263806C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263807C: 4E800020  blr
	return;
}

pub fn sub_82638080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638080 size=136
	// 82638080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263808C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638090: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638094: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82638098: 4BBE71C1  bl 0x8221f258
	ctx.lr = 0x8263809C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263809C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826380A0: 419A004C  beq cr6, 0x826380ec
	if ctx.cr[6].eq {
	pc = 0x826380EC; continue 'dispatch;
	}
	// 826380A4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826380A8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826380AC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 826380B0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826380B4: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 826380B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826380BC: 392A2FE8  addi r9, r10, 0x2fe8
	ctx.r[9].s64 = ctx.r[10].s64 + 12264;
	// 826380C0: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826380C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826380C8: C0079484  lfs f0, -0x6b7c(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
}

pub fn sub_82638108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638108 size=104
	// 82638108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263810C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263811C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82638120: 4BBE7139  bl 0x8221f258
	ctx.lr = 0x82638124;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638124: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638128: 419A0030  beq cr6, 0x82638158
	if ctx.cr[6].eq {
	pc = 0x82638158; continue 'dispatch;
	}
	// 8263812C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82638130: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638134: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638138: 392AA010  addi r9, r10, -0x5ff0
	ctx.r[9].s64 = ctx.r[10].s64 + -24560;
	// 8263813C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638140: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263814C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638154: 4E800020  blr
	return;
	// 82638158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263815C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263816C: 4E800020  blr
	return;
}

pub fn sub_82638170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638170 size=64
	// 82638170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263817C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638180: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82638184: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82638188: 388B9CF0  addi r4, r11, -0x6310
	ctx.r[4].s64 = ctx.r[11].s64 + -25360;
	// 8263818C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638190: 4BBF4D41  bl 0x8222ced0
	ctx.lr = 0x82638194;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82638194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82638198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263819C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826381A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826381A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826381A8: 4E800020  blr
	return;
}

pub fn sub_826381B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826381B0 size=120
	// 826381B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826381B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826381B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826381BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826381C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826381C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826381C8: 4BBE7091  bl 0x8221f258
	ctx.lr = 0x826381CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826381CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826381D0: 419A003C  beq cr6, 0x8263820c
	if ctx.cr[6].eq {
	pc = 0x8263820C; continue 'dispatch;
	}
	// 826381D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826381D8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826381DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826381E0: 392A4B88  addi r9, r10, 0x4b88
	ctx.r[9].s64 = ctx.r[10].s64 + 19336;
	// 826381E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826381E8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826381EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826381F0: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826381F4: 9903000D  stb r8, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[8].u8 ) };
	// 826381F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826381FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638208: 4E800020  blr
	return;
	// 8263820C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263821C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638220: 4E800020  blr
	return;
}

pub fn sub_82638228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638228 size=112
	// 82638228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263823C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82638240: 4BBE7019  bl 0x8221f258
	ctx.lr = 0x82638244;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638248: 419A0038  beq cr6, 0x82638280
	if ctx.cr[6].eq {
	pc = 0x82638280; continue 'dispatch;
	}
	// 8263824C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638250: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638258: 392A7818  addi r9, r10, 0x7818
	ctx.r[9].s64 = ctx.r[10].s64 + 30744;
	// 8263825C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638260: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638264: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82638268: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8263826C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638278: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263827C: 4E800020  blr
	return;
	// 82638280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263828C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638294: 4E800020  blr
	return;
}

pub fn sub_82638298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638298 size=152
	// 82638298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826382A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826382A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826382A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826382AC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826382B0: 4BBE6FA9  bl 0x8221f258
	ctx.lr = 0x826382B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826382B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826382B8: 419A0060  beq cr6, 0x82638318
	if ctx.cr[6].eq {
	pc = 0x82638318; continue 'dispatch;
	}
	// 826382BC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 826382C0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826382C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826382C8: 392A5FA8  addi r9, r10, 0x5fa8
	ctx.r[9].s64 = ctx.r[10].s64 + 24488;
	// 826382CC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826382D0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826382D4: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826382D8: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826382DC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826382E0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826382E4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 826382E8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826382EC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 826382F0: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826382F4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 826382F8: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 826382FC: 60E60002  ori r6, r7, 2
	ctx.r[6].u64 = ctx.r[7].u64 | 2;
	// 82638300: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 82638304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263830C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638314: 4E800020  blr
	return;
	// 82638318: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263831C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263832C: 4E800020  blr
	return;
}

pub fn sub_82638330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638330 size=144
	// 82638330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263833C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638344: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82638348: 4BBE6F11  bl 0x8221f258
	ctx.lr = 0x8263834C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263834C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638350: 419A0058  beq cr6, 0x826383a8
	if ctx.cr[6].eq {
	pc = 0x826383A8; continue 'dispatch;
	}
	// 82638354: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82638358: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263835C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82638360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638364: 390A2D68  addi r8, r10, 0x2d68
	ctx.r[8].s64 = ctx.r[10].s64 + 11624;
	// 82638368: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263836C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638370: 38C90B7C  addi r6, r9, 0xb7c
	ctx.r[6].s64 = ctx.r[9].s64 + 2940;
	// 82638374: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82638378: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263837C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82638380: 98E30020  stb r7, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[7].u8 ) };
	// 82638384: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82638388: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 8263838C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82638390: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82638394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263839C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826383A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826383A4: 4E800020  blr
	return;
	// 826383A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826383AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826383B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826383B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826383B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826383BC: 4E800020  blr
	return;
}

pub fn sub_826383C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826383C0 size=112
	// 826383C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826383C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826383C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826383CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826383D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826383D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826383D8: 4BBE6E81  bl 0x8221f258
	ctx.lr = 0x826383DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826383DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826383E0: 419A0034  beq cr6, 0x82638414
	if ctx.cr[6].eq {
	pc = 0x82638414; continue 'dispatch;
	}
	// 826383E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826383E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826383EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826383F0: 392A3530  addi r9, r10, 0x3530
	ctx.r[9].s64 = ctx.r[10].s64 + 13616;
	// 826383F4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826383F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826383FC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82638400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263840C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638410: 4E800020  blr
	return;
	// 82638414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263841C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638428: 4E800020  blr
	return;
}

pub fn sub_82638430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638430 size=104
	// 82638430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263843C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638444: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82638448: 4BBE6E11  bl 0x8221f258
	ctx.lr = 0x8263844C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263844C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638450: 419A0030  beq cr6, 0x82638480
	if ctx.cr[6].eq {
	pc = 0x82638480; continue 'dispatch;
	}
	// 82638454: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 82638458: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263845C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638460: 392A8C80  addi r9, r10, -0x7380
	ctx.r[9].s64 = ctx.r[10].s64 + -29568;
	// 82638464: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638468: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263846C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263847C: 4E800020  blr
	return;
	// 82638480: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263848C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638494: 4E800020  blr
	return;
}

pub fn sub_82638498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638498 size=168
	// 82638498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263849C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826384A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826384A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826384A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826384AC: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 826384B0: 4BBE6DA9  bl 0x8221f258
	ctx.lr = 0x826384B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826384B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826384B8: 419A0070  beq cr6, 0x82638528
	if ctx.cr[6].eq {
	pc = 0x82638528; continue 'dispatch;
	}
	// 826384BC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 826384C0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826384C4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 826384C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826384CC: 39093BA0  addi r8, r9, 0x3ba0
	ctx.r[8].s64 = ctx.r[9].s64 + 15264;
	// 826384D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826384D4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826384D8: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826384DC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826384E0: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 826384E4: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826384E8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 826384EC: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826384F0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 826384F4: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 826384F8: 9963000F  stb r11, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 826384FC: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82638500: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82638504: 90E30024  stw r7, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82638508: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 8263850C: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82638510: 9963002A  stb r11, 0x2a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(42 as u32), ctx.r[11].u8 ) };
	// 82638514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263851C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638524: 4E800020  blr
	return;
	// 82638528: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263852C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263853C: 4E800020  blr
	return;
}

pub fn sub_82638540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638540 size=120
	// 82638540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263854C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638554: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82638558: 4BBE6D01  bl 0x8221f258
	ctx.lr = 0x8263855C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263855C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638560: 419A0040  beq cr6, 0x826385a0
	if ctx.cr[6].eq {
	pc = 0x826385A0; continue 'dispatch;
	}
	// 82638564: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82638568: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263856C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82638570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638574: 390AA050  addi r8, r10, -0x5fb0
	ctx.r[8].s64 = ctx.r[10].s64 + -24496;
	// 82638578: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 8263857C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638580: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82638584: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82638588: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263858C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263859C: 4E800020  blr
	return;
	// 826385A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826385A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826385A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826385AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826385B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826385B4: 4E800020  blr
	return;
}

pub fn sub_826385B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826385B8 size=64
	// 826385B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826385BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826385C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826385C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826385C8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 826385CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826385D0: 388B9D3C  addi r4, r11, -0x62c4
	ctx.r[4].s64 = ctx.r[11].s64 + -25284;
	// 826385D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826385D8: 4BBF48F9  bl 0x8222ced0
	ctx.lr = 0x826385DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 826385DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826385E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826385E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826385E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826385EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826385F0: 4E800020  blr
	return;
}

pub fn sub_826385F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826385F8 size=104
	// 826385F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826385FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263860C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82638610: 4BBE6C49  bl 0x8221f258
	ctx.lr = 0x82638614;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638614: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638618: 419A0030  beq cr6, 0x82638648
	if ctx.cr[6].eq {
	pc = 0x82638648; continue 'dispatch;
	}
	// 8263861C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638620: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638628: 392A7CA8  addi r9, r10, 0x7ca8
	ctx.r[9].s64 = ctx.r[10].s64 + 31912;
	// 8263862C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638630: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263863C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638644: 4E800020  blr
	return;
	// 82638648: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263864C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263865C: 4E800020  blr
	return;
}

pub fn sub_82638660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638660 size=112
	// 82638660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263866C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638674: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82638678: 4BBE6BE1  bl 0x8221f258
	ctx.lr = 0x8263867C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263867C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638680: 419A0034  beq cr6, 0x826386b4
	if ctx.cr[6].eq {
	pc = 0x826386B4; continue 'dispatch;
	}
	// 82638684: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82638688: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263868C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638690: 392AA090  addi r9, r10, -0x5f70
	ctx.r[9].s64 = ctx.r[10].s64 + -24432;
	// 82638694: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638698: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263869C: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826386A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826386A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826386A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826386AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826386B0: 4E800020  blr
	return;
	// 826386B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826386B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826386BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826386C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826386C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826386C8: 4E800020  blr
	return;
}

pub fn sub_826386D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826386D0 size=64
	// 826386D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826386D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826386D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826386DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826386E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 826386E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826386E8: 388B9D5C  addi r4, r11, -0x62a4
	ctx.r[4].s64 = ctx.r[11].s64 + -25252;
	// 826386EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826386F0: 4BBF47E1  bl 0x8222ced0
	ctx.lr = 0x826386F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 826386F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826386F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826386FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638708: 4E800020  blr
	return;
}

pub fn sub_82638710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638710 size=12
	// 82638710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638714: 48670CF9  bl 0x82ca940c
	ctx.lr = 0x82638718;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82638718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82638790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638790 size=88
	// 82638790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263879C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826387A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826387A4: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 826387A8: 4BBE6AB1  bl 0x8221f258
	ctx.lr = 0x826387AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826387AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826387B0: 419A0020  beq cr6, 0x826387d0
	if ctx.cr[6].eq {
	pc = 0x826387D0; continue 'dispatch;
	}
	// 826387B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826387B8: 48113751  bl 0x8274bf08
	ctx.lr = 0x826387BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8274BF08);
	// 826387BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826387C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826387C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826387C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826387CC: 4E800020  blr
	return;
	// 826387D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826387D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826387D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826387DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826387E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826387E4: 4E800020  blr
	return;
}

pub fn sub_826387E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826387E8 size=16
	// 826387E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826387EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826387F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826387F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82638868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638868 size=208
	// 82638868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263886C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263887C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82638880: 4BBE69D9  bl 0x8221f258
	ctx.lr = 0x82638884;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638884: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638888: 419A0098  beq cr6, 0x82638920
	if ctx.cr[6].eq {
	pc = 0x82638920; continue 'dispatch;
	}
	// 8263888C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82638890: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638894: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82638898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263889C: 390939C0  addi r8, r9, 0x39c0
	ctx.r[8].s64 = ctx.r[9].s64 + 14784;
	// 826388A0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826388A4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826388A8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 826388AC: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826388B0: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 826388B4: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826388B8: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 826388BC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826388C0: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 826388C4: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826388C8: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 826388CC: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 826388D0: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 826388D4: 9963000F  stb r11, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 826388D8: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 826388DC: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 826388E0: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 826388E4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 826388E8: 90C30034  stw r6, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
	// 826388EC: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826388F0: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 826388F4: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 826388F8: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 826388FC: 9963004D  stb r11, 0x4d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(77 as u32), ctx.r[11].u8 ) };
	// 82638900: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82638904: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82638908: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263890C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263891C: 4E800020  blr
	return;
	// 82638920: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638924: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263892C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638934: 4E800020  blr
	return;
}

pub fn sub_82638938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638938 size=176
	// 82638938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263893C: 48670AD1  bl 0x82ca940c
	ctx.lr = 0x82638940;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82638940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638944: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82638948: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 8263894C: 4BBE690D  bl 0x8221f258
	ctx.lr = 0x82638950;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638954: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82638958: 419A0084  beq cr6, 0x826389dc
	if ctx.cr[6].eq {
	pc = 0x826389DC; continue 'dispatch;
	}
	// 8263895C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82638960: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82638964: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82638968: 394B5E70  addi r10, r11, 0x5e70
	ctx.r[10].s64 = ctx.r[11].s64 + 24176;
	// 8263896C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82638970: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82638974: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82638978: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 8263897C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82638980: 4820F631  bl 0x82847fb0
	ctx.lr = 0x82638984;
	crate::recompiler::externs::call(&mut ctx, base, 0x82847FB0);
	// 82638984: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82638988: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 8263898C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82638990: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82638994: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82638998: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8263899C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 826389A0: 4BBE68B9  bl 0x8221f258
	ctx.lr = 0x826389A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826389A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826389A8: 419A0008  beq cr6, 0x826389b0
	if ctx.cr[6].eq {
	pc = 0x826389B0; continue 'dispatch;
	}
	// 826389AC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826389B0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826389B4: 41820008  beq 0x826389bc
	if ctx.cr[0].eq {
	pc = 0x826389BC; continue 'dispatch;
	}
	// 826389B8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826389BC: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 826389C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826389C4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 826389C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826389CC: 4BD54AED  bl 0x8238d4b8
	ctx.lr = 0x826389D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 826389D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826389D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826389D8: 48670A84  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826389DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826389E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826389E4: 48670A78  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826389E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826389E8 size=16
	// 826389E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826389EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826389F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826389F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82638A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638A88 size=152
	// 82638A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638A90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638A94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638A9C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82638AA0: 4BBE67B9  bl 0x8221f258
	ctx.lr = 0x82638AA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638AA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638AA8: 419A0060  beq cr6, 0x82638b08
	if ctx.cr[6].eq {
	pc = 0x82638B08; continue 'dispatch;
	}
	// 82638AAC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82638AB0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638AB4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82638AB8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82638ABC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638AC0: 38EA388C  addi r7, r10, 0x388c
	ctx.r[7].s64 = ctx.r[10].s64 + 14476;
	// 82638AC4: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 82638AC8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638ACC: 38A962F0  addi r5, r9, 0x62f0
	ctx.r[5].s64 = ctx.r[9].s64 + 25328;
	// 82638AD0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82638AD4: 38882A88  addi r4, r8, 0x2a88
	ctx.r[4].s64 = ctx.r[8].s64 + 10888;
	// 82638AD8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82638ADC: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82638AE0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82638AE4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82638AE8: C006D5C8  lfs f0, -0x2a38(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82638AEC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82638AF0: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82638AF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638B04: 4E800020  blr
	return;
	// 82638B08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638B0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638B1C: 4E800020  blr
	return;
}

pub fn sub_82638B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638B20 size=136
	// 82638B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638B34: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82638B38: 4BBE6721  bl 0x8221f258
	ctx.lr = 0x82638B3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638B3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638B40: 419A004C  beq cr6, 0x82638b8c
	if ctx.cr[6].eq {
	pc = 0x82638B8C; continue 'dispatch;
	}
	// 82638B44: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82638B48: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82638B4C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82638B50: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638B54: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82638B58: 392B6498  addi r9, r11, 0x6498
	ctx.r[9].s64 = ctx.r[11].s64 + 25752;
	// 82638B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82638B60: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638B64: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82638B68: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
}

pub fn sub_82638BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638BA8 size=104
	// 82638BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638BBC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82638BC0: 4BBE6699  bl 0x8221f258
	ctx.lr = 0x82638BC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638BC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638BC8: 419A0030  beq cr6, 0x82638bf8
	if ctx.cr[6].eq {
	pc = 0x82638BF8; continue 'dispatch;
	}
	// 82638BCC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638BD0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638BD8: 392A2E78  addi r9, r10, 0x2e78
	ctx.r[9].s64 = ctx.r[10].s64 + 11896;
	// 82638BDC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638BE0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638BE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638BF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638BF4: 4E800020  blr
	return;
	// 82638BF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638C08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638C0C: 4E800020  blr
	return;
}

pub fn sub_82638C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638C10 size=12
	// 82638C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638C14: 486707F9  bl 0x82ca940c
	ctx.lr = 0x82638C18;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82638C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82638CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638CB0 size=88
	// 82638CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638CC4: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82638CC8: 4BBE6591  bl 0x8221f258
	ctx.lr = 0x82638CCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638CD0: 419A0020  beq cr6, 0x82638cf0
	if ctx.cr[6].eq {
	pc = 0x82638CF0; continue 'dispatch;
	}
	// 82638CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82638CD8: 482F3E01  bl 0x8292cad8
	ctx.lr = 0x82638CDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8292CAD8);
	// 82638CDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638CEC: 4E800020  blr
	return;
	// 82638CF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638D04: 4E800020  blr
	return;
}

pub fn sub_82638D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638D08 size=120
	// 82638D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638D1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82638D20: 4BBE6539  bl 0x8221f258
	ctx.lr = 0x82638D24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638D24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638D28: 419A0040  beq cr6, 0x82638d68
	if ctx.cr[6].eq {
	pc = 0x82638D68; continue 'dispatch;
	}
	// 82638D2C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638D30: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638D34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638D38: 392A6D38  addi r9, r10, 0x6d38
	ctx.r[9].s64 = ctx.r[10].s64 + 27960;
	// 82638D3C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638D40: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638D44: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82638D48: 891F0090  lbz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82638D4C: 61070002  ori r7, r8, 2
	ctx.r[7].u64 = ctx.r[8].u64 | 2;
	// 82638D50: 98FF0090  stb r7, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82638D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638D60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638D64: 4E800020  blr
	return;
	// 82638D68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638D7C: 4E800020  blr
	return;
}

pub fn sub_82638D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638D80 size=88
	// 82638D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638D94: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82638D98: 4BBE64C1  bl 0x8221f258
	ctx.lr = 0x82638D9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638DA0: 419A0020  beq cr6, 0x82638dc0
	if ctx.cr[6].eq {
	pc = 0x82638DC0; continue 'dispatch;
	}
	// 82638DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82638DA8: 482FB509  bl 0x829342b0
	ctx.lr = 0x82638DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x829342B0);
	// 82638DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638DBC: 4E800020  blr
	return;
	// 82638DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638DD4: 4E800020  blr
	return;
}

pub fn sub_82638DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638DD8 size=120
	// 82638DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638DEC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82638DF0: 4BBE6469  bl 0x8221f258
	ctx.lr = 0x82638DF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638DF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638DF8: 419A0040  beq cr6, 0x82638e38
	if ctx.cr[6].eq {
	pc = 0x82638E38; continue 'dispatch;
	}
	// 82638DFC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638E00: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638E08: 392A6FD8  addi r9, r10, 0x6fd8
	ctx.r[9].s64 = ctx.r[10].s64 + 28632;
	// 82638E0C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638E10: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638E14: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82638E18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82638E1C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82638E20: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82638E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638E34: 4E800020  blr
	return;
	// 82638E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638E4C: 4E800020  blr
	return;
}

pub fn sub_82638E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638E50 size=104
	// 82638E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638E64: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82638E68: 4BBE63F1  bl 0x8221f258
	ctx.lr = 0x82638E6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638E6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638E70: 419A0030  beq cr6, 0x82638ea0
	if ctx.cr[6].eq {
	pc = 0x82638EA0; continue 'dispatch;
	}
	// 82638E74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82638E78: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638E7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638E80: 392A34F0  addi r9, r10, 0x34f0
	ctx.r[9].s64 = ctx.r[10].s64 + 13552;
	// 82638E84: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638E88: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638E8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638E9C: 4E800020  blr
	return;
	// 82638EA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638EA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638EB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638EB4: 4E800020  blr
	return;
}

pub fn sub_82638EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638EB8 size=104
	// 82638EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638ECC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82638ED0: 4BBE6389  bl 0x8221f258
	ctx.lr = 0x82638ED4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638ED4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82638ED8: 419A0030  beq cr6, 0x82638f08
	if ctx.cr[6].eq {
	pc = 0x82638F08; continue 'dispatch;
	}
	// 82638EDC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82638EE0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82638EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638EE8: 392A72C8  addi r9, r10, 0x72c8
	ctx.r[9].s64 = ctx.r[10].s64 + 29384;
	// 82638EEC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638EF0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82638EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638F04: 4E800020  blr
	return;
	// 82638F08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82638F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638F1C: 4E800020  blr
	return;
}

pub fn sub_82638F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82638F20 size=216
	// 82638F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82638F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82638F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638F34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82638F38: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 82638F3C: 4BBE631D  bl 0x8221f258
	ctx.lr = 0x82638F40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82638F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82638F44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82638F48: 419A0094  beq cr6, 0x82638fdc
	if ctx.cr[6].eq {
	pc = 0x82638FDC; continue 'dispatch;
	}
	// 82638F4C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82638F50: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82638F54: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82638F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638F5C: 39097610  addi r8, r9, 0x7610
	ctx.r[8].s64 = ctx.r[9].s64 + 30224;
	// 82638F60: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82638F64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82638F68: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82638F6C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82638F70: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82638F74: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82638F78: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82638F7C: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82638F80: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82638F84: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82638F88: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82638F8C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82638F90: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82638F94: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82638F98: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82638F9C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82638FA0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82638FA4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82638FA8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82638FAC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82638FB0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82638FB4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82638FB8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82638FBC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82638FC0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82638FC4: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82638FC8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638FCC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82638FD0: 4BD544E9  bl 0x8238d4b8
	ctx.lr = 0x82638FD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82638FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82638FD8: 48000008  b 0x82638fe0
	pc = 0x82638FE0; continue 'dispatch;
	// 82638FDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82638FE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82638FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82638FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82638FEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82638FF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82638FF4: 4E800020  blr
	return;
}

pub fn sub_82638FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638FF8 size=112
	// 82638FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263900C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82639010: 4BBE6249  bl 0x8221f258
	ctx.lr = 0x82639014;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639014: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639018: 419A0038  beq cr6, 0x82639050
	if ctx.cr[6].eq {
	pc = 0x82639050; continue 'dispatch;
	}
	// 8263901C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82639020: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82639024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639028: 392A76C8  addi r9, r10, 0x76c8
	ctx.r[9].s64 = ctx.r[10].s64 + 30408;
	// 8263902C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639030: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82639034: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82639038: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263903C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263904C: 4E800020  blr
	return;
	// 82639050: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263905C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639064: 4E800020  blr
	return;
}

pub fn sub_82639068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639068 size=144
	// 82639068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263906C: 486703A1  bl 0x82ca940c
	ctx.lr = 0x82639070;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82639070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639074: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82639078: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8263907C: 4BBE61DD  bl 0x8221f258
	ctx.lr = 0x82639080;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82639084: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82639088: 419A0060  beq cr6, 0x826390e8
	if ctx.cr[6].eq {
	pc = 0x826390E8; continue 'dispatch;
	}
	// 8263908C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82639090: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82639094: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82639098: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8263909C: 392B3F20  addi r9, r11, 0x3f20
	ctx.r[9].s64 = ctx.r[11].s64 + 16160;
	// 826390A0: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 826390A4: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826390A8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826390AC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 826390B0: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 826390B4: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 826390B8: 4BB5AD81  bl 0x82193e38
	ctx.lr = 0x826390BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 826390BC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826390C0: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 826390C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826390C8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 826390CC: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 826390D0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826390D4: 88C70090  lbz r6, 0x90(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(144 as u32) ) } as u64;
	// 826390D8: 60C50002  ori r5, r6, 2
	ctx.r[5].u64 = ctx.r[6].u64 | 2;
	// 826390DC: 98A70090  stb r5, 0x90(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(144 as u32), ctx.r[5].u8 ) };
	// 826390E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826390E4: 48670378  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826390E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826390EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826390F0: 4867036C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826390F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826390F8 size=120
	// 826390F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826390FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263910C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82639110: 4BBE6149  bl 0x8221f258
	ctx.lr = 0x82639114;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639118: 419A003C  beq cr6, 0x82639154
	if ctx.cr[6].eq {
	pc = 0x82639154; continue 'dispatch;
	}
	// 8263911C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82639120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82639124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639128: 392A3368  addi r9, r10, 0x3368
	ctx.r[9].s64 = ctx.r[10].s64 + 13160;
	// 8263912C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639130: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82639134: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82639138: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263913C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82639140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263914C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639150: 4E800020  blr
	return;
	// 82639154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263915C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639168: 4E800020  blr
	return;
}

pub fn sub_82639170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639170 size=16
	// 82639170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263917C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_826391F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826391F0 size=104
	// 826391F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826391F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826391F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826391FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82639204: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82639208: 4BBE6051  bl 0x8221f258
	ctx.lr = 0x8263920C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263920C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639210: 419A0030  beq cr6, 0x82639240
	if ctx.cr[6].eq {
	pc = 0x82639240; continue 'dispatch;
	}
	// 82639214: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82639218: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263921C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639220: 392A7C48  addi r9, r10, 0x7c48
	ctx.r[9].s64 = ctx.r[10].s64 + 31816;
	// 82639224: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639228: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263922C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263923C: 4E800020  blr
	return;
	// 82639240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263924C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639254: 4E800020  blr
	return;
}

pub fn sub_82639258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639258 size=104
	// 82639258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263926C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82639270: 4BBE5FE9  bl 0x8221f258
	ctx.lr = 0x82639274;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639274: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639278: 419A0030  beq cr6, 0x826392a8
	if ctx.cr[6].eq {
	pc = 0x826392A8; continue 'dispatch;
	}
	// 8263927C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 82639280: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82639284: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639288: 392A8000  addi r9, r10, -0x8000
	ctx.r[9].s64 = ctx.r[10].s64 + -32768;
	// 8263928C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639290: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82639294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263929C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826392A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826392A4: 4E800020  blr
	return;
	// 826392A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826392AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826392B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826392B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826392B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826392BC: 4E800020  blr
	return;
}

pub fn sub_826392C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826392C0 size=160
	// 826392C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826392C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826392C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826392CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826392D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826392D4: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 826392D8: 4BBE5F81  bl 0x8221f258
	ctx.lr = 0x826392DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826392DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826392E0: 419A0064  beq cr6, 0x82639344
	if ctx.cr[6].eq {
	pc = 0x82639344; continue 'dispatch;
	}
	// 826392E4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 826392E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826392EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826392F0: 38EAA150  addi r7, r10, -0x5eb0
	ctx.r[7].s64 = ctx.r[10].s64 + -24240;
	// 826392F4: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 826392F8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826392FC: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82639300: 38A67088  addi r5, r6, 0x7088
	ctx.r[5].s64 = ctx.r[6].s64 + 28808;
	// 82639304: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82639308: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8263930C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639310: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82639314: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82639318: 7D20292D  stwcx. r9, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8263931C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639320: 4082FFE8  bne 0x82639308
	if !ctx.cr[0].eq {
	pc = 0x82639308; continue 'dispatch;
	}
	// 82639324: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82639328: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263932C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82639330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263933C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639340: 4E800020  blr
	return;
	// 82639344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263934C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639358: 4E800020  blr
	return;
}

pub fn sub_82639360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639360 size=64
	// 82639360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263936C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639370: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82639374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82639378: 388B9F10  addi r4, r11, -0x60f0
	ctx.r[4].s64 = ctx.r[11].s64 + -24816;
	// 8263937C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82639380: 4BBF3B51  bl 0x8222ced0
	ctx.lr = 0x82639384;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 82639384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82639388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639398: 4E800020  blr
	return;
}

pub fn sub_826393A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826393A0 size=112
	// 826393A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826393A4: 48670069  bl 0x82ca940c
	ctx.lr = 0x826393A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826393A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826393AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826393B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 826393B4: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 826393B8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 826393BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826393C0: 419A0008  beq cr6, 0x826393c8
	if ctx.cr[6].eq {
	pc = 0x826393C8; continue 'dispatch;
	}
	// 826393C4: 4BBE2975  bl 0x8221bd38
	ctx.lr = 0x826393C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 826393C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826393CC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 826393D0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 826393D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826393D8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826393DC: 4BBDB9FD  bl 0x82214dd8
	ctx.lr = 0x826393E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 826393E0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 826393E4: 57AA07FE  clrlwi r10, r29, 0x1f
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 826393E8: 392B2850  addi r9, r11, 0x2850
	ctx.r[9].s64 = ctx.r[11].s64 + 10320;
	// 826393EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 826393F0: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826393F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 826393F8: 419A000C  beq cr6, 0x82639404
	if ctx.cr[6].eq {
	pc = 0x82639404; continue 'dispatch;
	}
	// 826393FC: 4BBE293D  bl 0x8221bd38
	ctx.lr = 0x82639400;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 82639400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82639404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82639408: 48670054  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82639410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82639410 size=168
	// 82639410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639414: 4866FFF5  bl 0x82ca9408
	ctx.lr = 0x82639418;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 82639418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263941C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82639420: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82639424: 4BBE5E35  bl 0x8221f258
	ctx.lr = 0x82639428;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263942C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82639430: 419A007C  beq cr6, 0x826394ac
	if ctx.cr[6].eq {
	pc = 0x826394AC; continue 'dispatch;
	}
	// 82639434: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82639438: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8263943C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82639440: 394B8208  addi r10, r11, -0x7df8
	ctx.r[10].s64 = ctx.r[11].s64 + -32248;
	// 82639444: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82639448: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8263944C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82639450: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82639454: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 82639458: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8263945C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82639460: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82639464: 4BBE5DF5  bl 0x8221f258
	ctx.lr = 0x82639468;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639468: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8263946C: 419A001C  beq cr6, 0x82639488
	if ctx.cr[6].eq {
	pc = 0x82639488; continue 'dispatch;
	}
	// 82639470: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82639474: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82639478: C00BBE04  lfs f0, -0x41fc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16892 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263947C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82639480: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82639484: 48000008  b 0x8263948c
	pc = 0x8263948C; continue 'dispatch;
	// 82639488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8263948C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82639490: 480D8C79  bl 0x82712108
	ctx.lr = 0x82639494;
	crate::recompiler::externs::call(&mut ctx, base, 0x82712108);
	// 82639494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82639498: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263949C: 4BD5401D  bl 0x8238d4b8
	ctx.lr = 0x826394A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 826394A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826394A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826394A8: 4866FFB0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 826394AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826394B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826394B4: 4866FFA4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_826394B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826394B8 size=168
	// 826394B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826394BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826394C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826394C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826394C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826394CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826394D0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 826394D4: 4BBE5D85  bl 0x8221f258
	ctx.lr = 0x826394D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 826394D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826394DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826394E0: 419A0060  beq cr6, 0x82639540
	if ctx.cr[6].eq {
	pc = 0x82639540; continue 'dispatch;
	}
	// 826394E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826394E8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826394EC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826394F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826394F4: 390A2FA8  addi r8, r10, 0x2fa8
	ctx.r[8].s64 = ctx.r[10].s64 + 12200;
	// 826394F8: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826394FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82639500: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82639504: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82639508: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263950C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82639510: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82639514: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82639518: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8263951C: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82639520: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82639524: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82639528: 997F0026  stb r11, 0x26(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u8 ) };
	// 8263952C: 997F0027  stb r11, 0x27(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(39 as u32), ctx.r[11].u8 ) };
	// 82639530: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82639534: 4BD53F85  bl 0x8238d4b8
	ctx.lr = 0x82639538;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 82639538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263953C: 48000008  b 0x82639544
	pc = 0x82639544; continue 'dispatch;
	// 82639540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82639548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263954C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639550: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82639554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639558: 4E800020  blr
	return;
}

pub fn sub_82639560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639560 size=152
	// 82639560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639564: 4866FEA9  bl 0x82ca940c
	ctx.lr = 0x82639568;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82639568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263956C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82639570: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82639574: 4BBE5CE5  bl 0x8221f258
	ctx.lr = 0x82639578;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263957C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82639580: 419A0068  beq cr6, 0x826395e8
	if ctx.cr[6].eq {
	pc = 0x826395E8; continue 'dispatch;
	}
	// 82639584: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82639588: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263958C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82639590: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82639594: 392B8360  addi r9, r11, -0x7ca0
	ctx.r[9].s64 = ctx.r[11].s64 + -31904;
	// 82639598: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8263959C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826395A0: 38EA0B7C  addi r7, r10, 0xb7c
	ctx.r[7].s64 = ctx.r[10].s64 + 2940;
	// 826395A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826395A8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 826395AC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 826395B0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 826395B4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826395B8: 90FF0018  stw r7, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 826395BC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 826395C0: 4BB5A879  bl 0x82193e38
	ctx.lr = 0x826395C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 826395C4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 826395C8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 826395CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826395D0: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 826395D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826395D8: 4BD53EE1  bl 0x8238d4b8
	ctx.lr = 0x826395DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8238D4B8);
	// 826395DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826395E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826395E4: 4866FE78  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826395E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826395EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826395F0: 4866FE6C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_826395F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826395F8 size=120
	// 826395F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826395FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263960C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82639610: 4BBE5C49  bl 0x8221f258
	ctx.lr = 0x82639614;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639614: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639618: 419A003C  beq cr6, 0x82639654
	if ctx.cr[6].eq {
	pc = 0x82639654; continue 'dispatch;
	}
	// 8263961C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 82639620: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82639624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639628: 392A83D0  addi r9, r10, -0x7c30
	ctx.r[9].s64 = ctx.r[10].s64 + -31792;
	// 8263962C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82639630: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639634: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82639638: 9903000C  stb r8, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 8263963C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82639640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263964C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639650: 4E800020  blr
	return;
	// 82639654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263965C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639668: 4E800020  blr
	return;
}

pub fn sub_82639670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639670 size=16
	// 82639670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263967C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_82639750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639750 size=184
	// 82639750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639754: 4866FCB9  bl 0x82ca940c
	ctx.lr = 0x82639758;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 82639758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263975C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82639760: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82639764: 4BBE5AF5  bl 0x8221f258
	ctx.lr = 0x82639768;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263976C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82639770: 419A008C  beq cr6, 0x826397fc
	if ctx.cr[6].eq {
	pc = 0x826397FC; continue 'dispatch;
	}
	// 82639774: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82639778: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263977C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82639780: 394B2EE8  addi r10, r11, 0x2ee8
	ctx.r[10].s64 = ctx.r[11].s64 + 12008;
	// 82639784: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82639788: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8263978C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82639790: 483D0E21  bl 0x82a0a5b0
	ctx.lr = 0x82639794;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 82639794: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82639798: 481C5EA1  bl 0x827ff638
	ctx.lr = 0x8263979C;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 8263979C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 826397A0: 481C5E99  bl 0x827ff638
	ctx.lr = 0x826397A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 826397A4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 826397A8: 481C5E91  bl 0x827ff638
	ctx.lr = 0x826397AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 826397AC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 826397B0: 4BFCD761  bl 0x82606f10
	ctx.lr = 0x826397B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397B4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 826397B8: 4BFCD759  bl 0x82606f10
	ctx.lr = 0x826397BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397BC: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 826397C0: 4BFCD751  bl 0x82606f10
	ctx.lr = 0x826397C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397C4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 826397C8: 4BFCD749  bl 0x82606f10
	ctx.lr = 0x826397CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397CC: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 826397D0: 4BFCD741  bl 0x82606f10
	ctx.lr = 0x826397D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397D4: 9BDF0078  stb r30, 0x78(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u8 ) };
	// 826397D8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 826397DC: 4BFCD735  bl 0x82606f10
	ctx.lr = 0x826397E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397E0: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 826397E4: 4BFCD72D  bl 0x82606f10
	ctx.lr = 0x826397E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82606F10);
	// 826397E8: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 826397EC: 481C5E4D  bl 0x827ff638
	ctx.lr = 0x826397F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 826397F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826397F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826397F8: 4866FC64  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 826397FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82639804: 4866FC58  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_82639808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639808 size=112
	// 82639808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263980C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263981C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82639820: 4BBE5A39  bl 0x8221f258
	ctx.lr = 0x82639824;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639828: 419A0038  beq cr6, 0x82639860
	if ctx.cr[6].eq {
	pc = 0x82639860; continue 'dispatch;
	}
	// 8263982C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 82639830: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82639834: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639838: 392A8428  addi r9, r10, -0x7bd8
	ctx.r[9].s64 = ctx.r[10].s64 + -31704;
	// 8263983C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639840: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82639844: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82639848: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263984C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263985C: 4E800020  blr
	return;
	// 82639860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263986C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82639874: 4E800020  blr
	return;
}

pub fn sub_82639878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639878 size=120
	// 82639878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263987C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82639884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263988C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82639890: 4BBE59C9  bl 0x8221f258
	ctx.lr = 0x82639894;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 82639894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639898: 419A003C  beq cr6, 0x826398d4
	if ctx.cr[6].eq {
	pc = 0x826398D4; continue 'dispatch;
	}
	// 8263989C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 826398A0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826398A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826398A8: 392AA410  addi r9, r10, -0x5bf0
	ctx.r[9].s64 = ctx.r[10].s64 + -23536;
	// 826398AC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826398B0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826398B4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826398B8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826398BC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826398C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826398C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826398C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826398CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826398D0: 4E800020  blr
	return;
	// 826398D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826398D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826398DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826398E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826398E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826398E8: 4E800020  blr
	return;
}

pub fn sub_826398F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826398F0 size=184
	// 826398F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826398F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826398F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826398FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82639904: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82639908: 4BBE5951  bl 0x8221f258
	ctx.lr = 0x8263990C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8263990C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82639910: 419A0080  beq cr6, 0x82639990
	if ctx.cr[6].eq {
	pc = 0x82639990; continue 'dispatch;
	}
	// 82639914: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 82639918: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263991C: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 82639920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639924: 38CA8670  addi r6, r10, -0x7990
	ctx.r[6].s64 = ctx.r[10].s64 + -31120;
	// 82639928: 38A87088  addi r5, r8, 0x7088
	ctx.r[5].s64 = ctx.r[8].s64 + 28808;
	// 8263992C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82639930: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82639934: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82639938: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8263993C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82639940: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82639944: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639948: 7D202028  lwarx r9, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8263994C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82639950: 7D20212D  stwcx. r9, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82639954: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639958: 4082FFE8  bne 0x82639940
	if !ctx.cr[0].eq {
	pc = 0x82639940; continue 'dispatch;
	}
	// 8263995C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82639960: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82639964: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639968: 7D402828  lwarx r10, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8263996C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82639970: 7D40292D  stwcx. r10, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82639974: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82639978: 4082FFE8  bne 0x82639960
	if !ctx.cr[0].eq {
	pc = 0x82639960; continue 'dispatch;
	}
	// 8263997C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263998C: 4E800020  blr
	return;
	// 82639990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82639994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82639998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263999C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826399A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826399A4: 4E800020  blr
	return;
}

pub fn sub_826399A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826399A8 size=12
	// 826399A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826399AC: 4866FA61  bl 0x82ca940c
	ctx.lr = 0x826399B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 826399B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8263A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A688 size=104
	// 8263A688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A68C: 4866ED81  bl 0x82ca940c
	ctx.lr = 0x8263A690;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 8263A690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263A698: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8263A69C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A6A0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A6A4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263A6A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A6AC: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8263A6B0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A6B4: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8263A6B8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8263A6BC: 419A001C  beq cr6, 0x8263a6d8
	if ctx.cr[6].eq {
	pc = 0x8263A6D8; continue 'dispatch;
	}
	// 8263A6C0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A6C4: 4BBE1675  bl 0x8221bd38
	ctx.lr = 0x8263A6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A6C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A6CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8263A6D0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8263A6D4: 409AFFEC  bne cr6, 0x8263a6c0
	if !ctx.cr[6].eq {
	pc = 0x8263A6C0; continue 'dispatch;
	}
	// 8263A6D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A6DC: 4BBE165D  bl 0x8221bd38
	ctx.lr = 0x8263A6E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A6E0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263A6E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263A6E8: 4866ED74  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_8263A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A6F0 size=248
	// 8263A6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A6F4: 4866ED09  bl 0x82ca93fc
	ctx.lr = 0x8263A6F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93FC);
	// 8263A6F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A6FC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8263A700: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8263A704: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A708: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A70C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263A710: 81590004  lwz r10, 4(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A714: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8263A718: 81390004  lwz r9, 4(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A71C: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8263A720: 93990008  stw r28, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8263A724: 419A00B8  beq cr6, 0x8263a7dc
	if ctx.cr[6].eq {
	pc = 0x8263A7DC; continue 'dispatch;
	}
	// 8263A728: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8263A72C: 3B4B0B7C  addi r26, r11, 0xb7c
	ctx.r[26].s64 = ctx.r[11].s64 + 2940;
	// 8263A730: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8263A734: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 8263A738: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A73C: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 8263A740: 935D0010  stw r26, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 8263A744: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A748: 419A0034  beq cr6, 0x8263a77c
	if ctx.cr[6].eq {
	pc = 0x8263A77C; continue 'dispatch;
	}
	// 8263A74C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A750: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8263A754: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8263A758: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A75C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A760: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8263A764: 409A0014  bne cr6, 0x8263a778
	if !ctx.cr[6].eq {
	pc = 0x8263A778; continue 'dispatch;
	}
	// 8263A768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A76C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A770: 409A0008  bne cr6, 0x8263a778
	if !ctx.cr[6].eq {
	pc = 0x8263A778; continue 'dispatch;
	}
	// 8263A774: 4BBE15C5  bl 0x8221bd38
	ctx.lr = 0x8263A778;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A778: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8263A77C: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8263A780: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A788: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8263A78C: 419A0034  beq cr6, 0x8263a7c0
	if ctx.cr[6].eq {
	pc = 0x8263A7C0; continue 'dispatch;
	}
	// 8263A790: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A794: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8263A798: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8263A79C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A7A0: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A7A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8263A7A8: 409A0014  bne cr6, 0x8263a7bc
	if !ctx.cr[6].eq {
	pc = 0x8263A7BC; continue 'dispatch;
	}
	// 8263A7AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A7B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A7B4: 409A0008  bne cr6, 0x8263a7bc
	if !ctx.cr[6].eq {
	pc = 0x8263A7BC; continue 'dispatch;
	}
	// 8263A7B8: 4BBE1581  bl 0x8221bd38
	ctx.lr = 0x8263A7BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A7BC: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8263A7C0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8263A7C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263A7C8: 4BBE1571  bl 0x8221bd38
	ctx.lr = 0x8263A7CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A7CC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A7D0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8263A7D4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8263A7D8: 409AFF58  bne cr6, 0x8263a730
	if !ctx.cr[6].eq {
	pc = 0x8263A730; continue 'dispatch;
	}
	// 8263A7DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8263A7E0: 4866EC6C  b 0x82ca944c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA944C);
	return;
}

pub fn sub_8263A7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A7E8 size=136
	// 8263A7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A7EC: 4866EC1D  bl 0x82ca9408
	ctx.lr = 0x8263A7F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 8263A7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A7F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8263A7F8: 7F04E040  cmplw cr6, r4, r28
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8263A7FC: 419A0068  beq cr6, 0x8263a864
	if ctx.cr[6].eq {
	pc = 0x8263A864; continue 'dispatch;
	}
	// 8263A800: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8263A804: 3BE40004  addi r31, r4, 4
	ctx.r[31].s64 = ctx.r[4].s64 + 4;
	// 8263A808: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8263A80C: 3BAB0B7C  addi r29, r11, 0xb7c
	ctx.r[29].s64 = ctx.r[11].s64 + 2940;
	// 8263A810: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A814: 93BFFFFC  stw r29, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 8263A818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A81C: 419A0034  beq cr6, 0x8263a850
	if ctx.cr[6].eq {
	pc = 0x8263A850; continue 'dispatch;
	}
	// 8263A820: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A824: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8263A828: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8263A82C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A830: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263A834: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8263A838: 409A0014  bne cr6, 0x8263a84c
	if !ctx.cr[6].eq {
	pc = 0x8263A84C; continue 'dispatch;
	}
	// 8263A83C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A844: 409A0008  bne cr6, 0x8263a84c
	if !ctx.cr[6].eq {
	pc = 0x8263A84C; continue 'dispatch;
	}
	// 8263A848: 4BBE14F1  bl 0x8221bd38
	ctx.lr = 0x8263A84C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 8263A84C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8263A850: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8263A854: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 8263A858: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8263A85C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8263A860: 409AFFB0  bne cr6, 0x8263a810
	if !ctx.cr[6].eq {
	pc = 0x8263A810; continue 'dispatch;
	}
	// 8263A864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A868: 4866EBF0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_8263A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A870 size=368
	// 8263A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A874: 4866EB8D  bl 0x82ca9400
	ctx.lr = 0x8263A878;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263A878: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A87C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263A880: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263A884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A888: 419A014C  beq cr6, 0x8263a9d4
	if ctx.cr[6].eq {
	pc = 0x8263A9D4; continue 'dispatch;
	}
	// 8263A88C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263A890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263A894: 388B92E8  addi r4, r11, -0x6d18
	ctx.r[4].s64 = ctx.r[11].s64 + -27928;
	// 8263A898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263A89C: 4BBF2635  bl 0x8222ced0
	ctx.lr = 0x8263A8A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263A8A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263A8A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A8A8: 409A0010  bne cr6, 0x8263a8b8
	if !ctx.cr[6].eq {
	pc = 0x8263A8B8; continue 'dispatch;
	}
	// 8263A8AC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263A8B0: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263A8B4: 48000008  b 0x8263a8bc
	pc = 0x8263A8BC; continue 'dispatch;
	// 8263A8B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263A8BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263A8C0: 4BFF5A01  bl 0x826302c0
	ctx.lr = 0x8263A8C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263A8C4: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263A8C8: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263A8CC: 392B03D0  addi r9, r11, 0x3d0
	ctx.r[9].s64 = ctx.r[11].s64 + 976;
	// 8263A8D0: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263A8D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263A8D8: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263A8DC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263A8E0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263A8E4: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263A8E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263A8EC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263A8F0: 4BBB5951  bl 0x821f0240
	ctx.lr = 0x8263A8F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263A8F4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8263A8F8: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263A8FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263A900: 93410070  stw r26, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 8263A904: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263A908: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263A90C: 48019625  bl 0x82653f30
	ctx.lr = 0x8263A910;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263A910: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263A914: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263A918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A91C: 409A000C  bne cr6, 0x8263a928
	if !ctx.cr[6].eq {
	pc = 0x8263A928; continue 'dispatch;
	}
	// 8263A920: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263A924: 48000010  b 0x8263a934
	pc = 0x8263A934; continue 'dispatch;
	// 8263A928: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263A92C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263A930: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263A934: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263A938: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263A93C: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263A940: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263A944: 4098002C  bge cr6, 0x8263a970
	if !ctx.cr[6].lt {
	pc = 0x8263A970; continue 'dispatch;
	}
	// 8263A948: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263A94C: 419A0018  beq cr6, 0x8263a964
	if ctx.cr[6].eq {
	pc = 0x8263A964; continue 'dispatch;
	}
	// 8263A950: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263A954: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263A958: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263A95C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263A960: 480195D1  bl 0x82653f30
	ctx.lr = 0x8263A964;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263A964: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263A968: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263A96C: 48000020  b 0x8263a98c
	pc = 0x8263A98C; continue 'dispatch;
	// 8263A970: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263A974: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263A978: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263A97C: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263A980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263A984: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263A988: 480196D9  bl 0x82654060
	ctx.lr = 0x8263A98C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263A98C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263A990: 4BBDA449  bl 0x82214dd8
	ctx.lr = 0x8263A994;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263A994: 9B5F0011  stb r26, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[26].u8 ) };
	// 8263A998: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263A99C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263A9A0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263A9A4: 4BBB589D  bl 0x821f0240
	ctx.lr = 0x8263A9A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263A9A8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263A9AC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263A9B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263A9B4: 4BF30EAD  bl 0x8256b860
	ctx.lr = 0x8263A9B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263A9B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263A9BC: 4BBDA41D  bl 0x82214dd8
	ctx.lr = 0x8263A9C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263A9C0: 9B5D0029  stb r26, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 8263A9C4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263A9C8: 4BBDA411  bl 0x82214dd8
	ctx.lr = 0x8263A9CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263A9CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263A9D0: 4BBDA409  bl 0x82214dd8
	ctx.lr = 0x8263A9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263A9D4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263A9D8: 4866EA78  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263A9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A9E0 size=392
	// 8263A9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A9E4: 4866EA1D  bl 0x82ca9400
	ctx.lr = 0x8263A9E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263A9E8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A9EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263A9F0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263A9F4: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263A9F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263A9FC: 419A0150  beq cr6, 0x8263ab4c
	if ctx.cr[6].eq {
	pc = 0x8263AB4C; continue 'dispatch;
	}
	// 8263AA00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263AA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263AA08: 388B92F4  addi r4, r11, -0x6d0c
	ctx.r[4].s64 = ctx.r[11].s64 + -27916;
	// 8263AA0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AA10: 4BBF24C1  bl 0x8222ced0
	ctx.lr = 0x8263AA14;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263AA14: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263AA18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AA1C: 409A0010  bne cr6, 0x8263aa2c
	if !ctx.cr[6].eq {
	pc = 0x8263AA2C; continue 'dispatch;
	}
	// 8263AA20: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263AA24: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263AA28: 48000008  b 0x8263aa30
	pc = 0x8263AA30; continue 'dispatch;
	// 8263AA2C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263AA30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263AA34: 4BFF588D  bl 0x826302c0
	ctx.lr = 0x8263AA38;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263AA38: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263AA3C: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263AA40: 392B04A0  addi r9, r11, 0x4a0
	ctx.r[9].s64 = ctx.r[11].s64 + 1184;
	// 8263AA44: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263AA48: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263AA4C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263AA50: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263AA54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263AA58: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263AA5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AA60: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AA64: 4BBB57DD  bl 0x821f0240
	ctx.lr = 0x8263AA68;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AA68: 38E00047  li r7, 0x47
	ctx.r[7].s64 = 71;
	// 8263AA6C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263AA70: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263AA74: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263AA78: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263AA7C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263AA80: 480194B1  bl 0x82653f30
	ctx.lr = 0x8263AA84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AA84: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AA88: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263AA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AA90: 409A000C  bne cr6, 0x8263aa9c
	if !ctx.cr[6].eq {
	pc = 0x8263AA9C; continue 'dispatch;
	}
	// 8263AA94: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263AA98: 48000010  b 0x8263aaa8
	pc = 0x8263AAA8; continue 'dispatch;
	// 8263AA9C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263AAA0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263AAA4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263AAA8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AAAC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263AAB0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263AAB4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263AAB8: 4098002C  bge cr6, 0x8263aae4
	if !ctx.cr[6].lt {
	pc = 0x8263AAE4; continue 'dispatch;
	}
	// 8263AABC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263AAC0: 419A0018  beq cr6, 0x8263aad8
	if ctx.cr[6].eq {
	pc = 0x8263AAD8; continue 'dispatch;
	}
	// 8263AAC4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263AAC8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263AACC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263AAD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263AAD4: 4801945D  bl 0x82653f30
	ctx.lr = 0x8263AAD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AAD8: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263AADC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AAE0: 48000020  b 0x8263ab00
	pc = 0x8263AB00; continue 'dispatch;
	// 8263AAE4: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263AAE8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263AAEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263AAF0: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263AAF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AAF8: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263AAFC: 48019565  bl 0x82654060
	ctx.lr = 0x8263AB00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263AB00: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263AB04: 4BBDA2D5  bl 0x82214dd8
	ctx.lr = 0x8263AB08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AB08: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263AB0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AB10: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263AB14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AB18: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263AB1C: 4BBB5725  bl 0x821f0240
	ctx.lr = 0x8263AB20;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AB20: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263AB24: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263AB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AB2C: 4BF30D35  bl 0x8256b860
	ctx.lr = 0x8263AB30;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263AB30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AB34: 4BBDA2A5  bl 0x82214dd8
	ctx.lr = 0x8263AB38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AB38: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263AB3C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AB40: 4BBDA299  bl 0x82214dd8
	ctx.lr = 0x8263AB44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AB44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AB48: 4BBDA291  bl 0x82214dd8
	ctx.lr = 0x8263AB4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AB4C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263AB50: 419A000C  beq cr6, 0x8263ab5c
	if ctx.cr[6].eq {
	pc = 0x8263AB5C; continue 'dispatch;
	}
	// 8263AB54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263AB58: 48255BC1  bl 0x82890718
	ctx.lr = 0x8263AB5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82890718);
	// 8263AB5C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263AB60: 4866E8F0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263AB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AB68 size=392
	// 8263AB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AB6C: 4866E895  bl 0x82ca9400
	ctx.lr = 0x8263AB70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263AB70: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AB74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263AB78: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263AB7C: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263AB80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AB84: 419A0150  beq cr6, 0x8263acd4
	if ctx.cr[6].eq {
	pc = 0x8263ACD4; continue 'dispatch;
	}
	// 8263AB88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263AB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263AB90: 388B9300  addi r4, r11, -0x6d00
	ctx.r[4].s64 = ctx.r[11].s64 + -27904;
	// 8263AB94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AB98: 4BBF2339  bl 0x8222ced0
	ctx.lr = 0x8263AB9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263AB9C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263ABA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263ABA4: 409A0010  bne cr6, 0x8263abb4
	if !ctx.cr[6].eq {
	pc = 0x8263ABB4; continue 'dispatch;
	}
	// 8263ABA8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263ABAC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263ABB0: 48000008  b 0x8263abb8
	pc = 0x8263ABB8; continue 'dispatch;
	// 8263ABB4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263ABB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263ABBC: 4BFF5705  bl 0x826302c0
	ctx.lr = 0x8263ABC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263ABC0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263ABC4: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263ABC8: 392B0530  addi r9, r11, 0x530
	ctx.r[9].s64 = ctx.r[11].s64 + 1328;
	// 8263ABCC: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263ABD0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263ABD4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263ABD8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263ABDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263ABE0: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263ABE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263ABE8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263ABEC: 4BBB5655  bl 0x821f0240
	ctx.lr = 0x8263ABF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263ABF0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263ABF4: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263ABF8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263ABFC: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263AC00: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263AC04: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263AC08: 48019329  bl 0x82653f30
	ctx.lr = 0x8263AC0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AC0C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AC10: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263AC14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AC18: 409A000C  bne cr6, 0x8263ac24
	if !ctx.cr[6].eq {
	pc = 0x8263AC24; continue 'dispatch;
	}
	// 8263AC1C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263AC20: 48000010  b 0x8263ac30
	pc = 0x8263AC30; continue 'dispatch;
	// 8263AC24: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263AC28: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263AC2C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263AC30: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AC34: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263AC38: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263AC3C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263AC40: 4098002C  bge cr6, 0x8263ac6c
	if !ctx.cr[6].lt {
	pc = 0x8263AC6C; continue 'dispatch;
	}
	// 8263AC44: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263AC48: 419A0018  beq cr6, 0x8263ac60
	if ctx.cr[6].eq {
	pc = 0x8263AC60; continue 'dispatch;
	}
	// 8263AC4C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263AC50: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263AC54: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263AC58: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263AC5C: 480192D5  bl 0x82653f30
	ctx.lr = 0x8263AC60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AC60: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263AC64: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AC68: 48000020  b 0x8263ac88
	pc = 0x8263AC88; continue 'dispatch;
	// 8263AC6C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263AC70: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263AC74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263AC78: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263AC7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AC80: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263AC84: 480193DD  bl 0x82654060
	ctx.lr = 0x8263AC88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263AC88: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263AC8C: 4BBDA14D  bl 0x82214dd8
	ctx.lr = 0x8263AC90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AC90: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263AC94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AC98: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263AC9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263ACA0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263ACA4: 4BBB559D  bl 0x821f0240
	ctx.lr = 0x8263ACA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263ACA8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263ACAC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263ACB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263ACB4: 4BF30BAD  bl 0x8256b860
	ctx.lr = 0x8263ACB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263ACB8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263ACBC: 4BBDA11D  bl 0x82214dd8
	ctx.lr = 0x8263ACC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263ACC0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263ACC4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263ACC8: 4BBDA111  bl 0x82214dd8
	ctx.lr = 0x8263ACCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263ACCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263ACD0: 4BBDA109  bl 0x82214dd8
	ctx.lr = 0x8263ACD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263ACD4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263ACD8: 419A000C  beq cr6, 0x8263ace4
	if ctx.cr[6].eq {
	pc = 0x8263ACE4; continue 'dispatch;
	}
	// 8263ACDC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263ACE0: 480B1AB1  bl 0x826ec790
	ctx.lr = 0x8263ACE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263ACE4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263ACE8: 4866E768  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ACF0 size=392
	// 8263ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ACF4: 4866E70D  bl 0x82ca9400
	ctx.lr = 0x8263ACF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263ACF8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ACFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263AD00: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263AD04: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263AD08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AD0C: 419A0150  beq cr6, 0x8263ae5c
	if ctx.cr[6].eq {
	pc = 0x8263AE5C; continue 'dispatch;
	}
	// 8263AD10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263AD14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263AD18: 388B9314  addi r4, r11, -0x6cec
	ctx.r[4].s64 = ctx.r[11].s64 + -27884;
	// 8263AD1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AD20: 4BBF21B1  bl 0x8222ced0
	ctx.lr = 0x8263AD24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263AD24: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263AD28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AD2C: 409A0010  bne cr6, 0x8263ad3c
	if !ctx.cr[6].eq {
	pc = 0x8263AD3C; continue 'dispatch;
	}
	// 8263AD30: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263AD34: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263AD38: 48000008  b 0x8263ad40
	pc = 0x8263AD40; continue 'dispatch;
	// 8263AD3C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263AD40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263AD44: 4BFF557D  bl 0x826302c0
	ctx.lr = 0x8263AD48;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263AD48: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263AD4C: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263AD50: 392B06C0  addi r9, r11, 0x6c0
	ctx.r[9].s64 = ctx.r[11].s64 + 1728;
	// 8263AD54: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263AD58: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263AD5C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263AD60: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263AD64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263AD68: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263AD6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AD70: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AD74: 4BBB54CD  bl 0x821f0240
	ctx.lr = 0x8263AD78;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AD78: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263AD7C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263AD80: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263AD84: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263AD88: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263AD8C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263AD90: 480191A1  bl 0x82653f30
	ctx.lr = 0x8263AD94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AD94: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AD98: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263AD9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263ADA0: 409A000C  bne cr6, 0x8263adac
	if !ctx.cr[6].eq {
	pc = 0x8263ADAC; continue 'dispatch;
	}
	// 8263ADA4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263ADA8: 48000010  b 0x8263adb8
	pc = 0x8263ADB8; continue 'dispatch;
	// 8263ADAC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263ADB0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263ADB4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263ADB8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263ADBC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263ADC0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263ADC4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263ADC8: 4098002C  bge cr6, 0x8263adf4
	if !ctx.cr[6].lt {
	pc = 0x8263ADF4; continue 'dispatch;
	}
	// 8263ADCC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263ADD0: 419A0018  beq cr6, 0x8263ade8
	if ctx.cr[6].eq {
	pc = 0x8263ADE8; continue 'dispatch;
	}
	// 8263ADD4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263ADD8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263ADDC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263ADE0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263ADE4: 4801914D  bl 0x82653f30
	ctx.lr = 0x8263ADE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263ADE8: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263ADEC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263ADF0: 48000020  b 0x8263ae10
	pc = 0x8263AE10; continue 'dispatch;
	// 8263ADF4: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263ADF8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263ADFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263AE00: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263AE04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AE08: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263AE0C: 48019255  bl 0x82654060
	ctx.lr = 0x8263AE10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263AE10: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263AE14: 4BBD9FC5  bl 0x82214dd8
	ctx.lr = 0x8263AE18;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AE18: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263AE1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AE20: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263AE24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AE28: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263AE2C: 4BBB5415  bl 0x821f0240
	ctx.lr = 0x8263AE30;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AE30: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263AE34: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263AE38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AE3C: 4BF30A25  bl 0x8256b860
	ctx.lr = 0x8263AE40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263AE40: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AE44: 4BBD9F95  bl 0x82214dd8
	ctx.lr = 0x8263AE48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AE48: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263AE4C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AE50: 4BBD9F89  bl 0x82214dd8
	ctx.lr = 0x8263AE54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AE54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AE58: 4BBD9F81  bl 0x82214dd8
	ctx.lr = 0x8263AE5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AE5C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263AE60: 419A000C  beq cr6, 0x8263ae6c
	if ctx.cr[6].eq {
	pc = 0x8263AE6C; continue 'dispatch;
	}
	// 8263AE64: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263AE68: 480B1929  bl 0x826ec790
	ctx.lr = 0x8263AE6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263AE6C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263AE70: 4866E5E0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263AE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AE78 size=392
	// 8263AE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AE7C: 4866E585  bl 0x82ca9400
	ctx.lr = 0x8263AE80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263AE80: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AE84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263AE88: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263AE8C: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263AE90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AE94: 419A0150  beq cr6, 0x8263afe4
	if ctx.cr[6].eq {
	pc = 0x8263AFE4; continue 'dispatch;
	}
	// 8263AE98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263AE9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263AEA0: 388B932C  addi r4, r11, -0x6cd4
	ctx.r[4].s64 = ctx.r[11].s64 + -27860;
	// 8263AEA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AEA8: 4BBF2029  bl 0x8222ced0
	ctx.lr = 0x8263AEAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263AEAC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263AEB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AEB4: 409A0010  bne cr6, 0x8263aec4
	if !ctx.cr[6].eq {
	pc = 0x8263AEC4; continue 'dispatch;
	}
	// 8263AEB8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263AEBC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263AEC0: 48000008  b 0x8263aec8
	pc = 0x8263AEC8; continue 'dispatch;
	// 8263AEC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263AEC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263AECC: 4BFF53F5  bl 0x826302c0
	ctx.lr = 0x8263AED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263AED0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263AED4: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263AED8: 392B07C8  addi r9, r11, 0x7c8
	ctx.r[9].s64 = ctx.r[11].s64 + 1992;
	// 8263AEDC: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263AEE0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263AEE4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263AEE8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263AEEC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263AEF0: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263AEF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AEF8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AEFC: 4BBB5345  bl 0x821f0240
	ctx.lr = 0x8263AF00;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AF00: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263AF04: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263AF08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263AF0C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263AF10: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263AF14: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263AF18: 48019019  bl 0x82653f30
	ctx.lr = 0x8263AF1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AF1C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AF20: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263AF24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263AF28: 409A000C  bne cr6, 0x8263af34
	if !ctx.cr[6].eq {
	pc = 0x8263AF34; continue 'dispatch;
	}
	// 8263AF2C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263AF30: 48000010  b 0x8263af40
	pc = 0x8263AF40; continue 'dispatch;
	// 8263AF34: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263AF38: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263AF3C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263AF40: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263AF44: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263AF48: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263AF4C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263AF50: 4098002C  bge cr6, 0x8263af7c
	if !ctx.cr[6].lt {
	pc = 0x8263AF7C; continue 'dispatch;
	}
	// 8263AF54: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263AF58: 419A0018  beq cr6, 0x8263af70
	if ctx.cr[6].eq {
	pc = 0x8263AF70; continue 'dispatch;
	}
	// 8263AF5C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263AF60: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263AF64: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263AF68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263AF6C: 48018FC5  bl 0x82653f30
	ctx.lr = 0x8263AF70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263AF70: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263AF74: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AF78: 48000020  b 0x8263af98
	pc = 0x8263AF98; continue 'dispatch;
	// 8263AF7C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263AF80: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263AF84: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263AF88: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263AF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AF90: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263AF94: 480190CD  bl 0x82654060
	ctx.lr = 0x8263AF98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263AF98: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263AF9C: 4BBD9E3D  bl 0x82214dd8
	ctx.lr = 0x8263AFA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AFA0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263AFA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263AFA8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263AFAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AFB0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263AFB4: 4BBB528D  bl 0x821f0240
	ctx.lr = 0x8263AFB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263AFB8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263AFBC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263AFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263AFC4: 4BF3089D  bl 0x8256b860
	ctx.lr = 0x8263AFC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263AFC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263AFCC: 4BBD9E0D  bl 0x82214dd8
	ctx.lr = 0x8263AFD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AFD0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263AFD4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263AFD8: 4BBD9E01  bl 0x82214dd8
	ctx.lr = 0x8263AFDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AFDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263AFE0: 4BBD9DF9  bl 0x82214dd8
	ctx.lr = 0x8263AFE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263AFE4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263AFE8: 419A000C  beq cr6, 0x8263aff4
	if ctx.cr[6].eq {
	pc = 0x8263AFF4; continue 'dispatch;
	}
	// 8263AFEC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263AFF0: 480B17A1  bl 0x826ec790
	ctx.lr = 0x8263AFF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263AFF4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263AFF8: 4866E458  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B000 size=392
	// 8263B000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B004: 4866E3FD  bl 0x82ca9400
	ctx.lr = 0x8263B008;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B008: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B00C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B010: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B014: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B01C: 419A0150  beq cr6, 0x8263b16c
	if ctx.cr[6].eq {
	pc = 0x8263B16C; continue 'dispatch;
	}
	// 8263B020: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8263B024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B028: 388B3C00  addi r4, r11, 0x3c00
	ctx.r[4].s64 = ctx.r[11].s64 + 15360;
	// 8263B02C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B030: 4BBF1EA1  bl 0x8222ced0
	ctx.lr = 0x8263B034;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B034: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B038: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B03C: 409A0010  bne cr6, 0x8263b04c
	if !ctx.cr[6].eq {
	pc = 0x8263B04C; continue 'dispatch;
	}
	// 8263B040: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B044: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B048: 48000008  b 0x8263b050
	pc = 0x8263B050; continue 'dispatch;
	// 8263B04C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B050: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B054: 4BFF526D  bl 0x826302c0
	ctx.lr = 0x8263B058;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B058: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B05C: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263B060: 392B09E0  addi r9, r11, 0x9e0
	ctx.r[9].s64 = ctx.r[11].s64 + 2528;
	// 8263B064: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263B068: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B06C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B070: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B074: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B078: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B07C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B080: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B084: 4BBB51BD  bl 0x821f0240
	ctx.lr = 0x8263B088;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B088: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263B08C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B090: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B094: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B098: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B09C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B0A0: 48018E91  bl 0x82653f30
	ctx.lr = 0x8263B0A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B0A4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B0A8: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B0AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B0B0: 409A000C  bne cr6, 0x8263b0bc
	if !ctx.cr[6].eq {
	pc = 0x8263B0BC; continue 'dispatch;
	}
	// 8263B0B4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B0B8: 48000010  b 0x8263b0c8
	pc = 0x8263B0C8; continue 'dispatch;
	// 8263B0BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B0C0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B0C4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B0C8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B0CC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B0D0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B0D4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B0D8: 4098002C  bge cr6, 0x8263b104
	if !ctx.cr[6].lt {
	pc = 0x8263B104; continue 'dispatch;
	}
	// 8263B0DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B0E0: 419A0018  beq cr6, 0x8263b0f8
	if ctx.cr[6].eq {
	pc = 0x8263B0F8; continue 'dispatch;
	}
	// 8263B0E4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B0E8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B0EC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B0F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B0F4: 48018E3D  bl 0x82653f30
	ctx.lr = 0x8263B0F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B0F8: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B0FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B100: 48000020  b 0x8263b120
	pc = 0x8263B120; continue 'dispatch;
	// 8263B104: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B108: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B10C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B110: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B118: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B11C: 48018F45  bl 0x82654060
	ctx.lr = 0x8263B120;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B120: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B124: 4BBD9CB5  bl 0x82214dd8
	ctx.lr = 0x8263B128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B128: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B12C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B130: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B134: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B138: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B13C: 4BBB5105  bl 0x821f0240
	ctx.lr = 0x8263B140;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B140: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B144: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B14C: 4BF30715  bl 0x8256b860
	ctx.lr = 0x8263B150;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B150: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B154: 4BBD9C85  bl 0x82214dd8
	ctx.lr = 0x8263B158;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B158: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B15C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B160: 4BBD9C79  bl 0x82214dd8
	ctx.lr = 0x8263B164;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B164: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B168: 4BBD9C71  bl 0x82214dd8
	ctx.lr = 0x8263B16C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B16C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B170: 419A000C  beq cr6, 0x8263b17c
	if ctx.cr[6].eq {
	pc = 0x8263B17C; continue 'dispatch;
	}
	// 8263B174: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B178: 480B1619  bl 0x826ec790
	ctx.lr = 0x8263B17C;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263B17C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B180: 4866E2D0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B188 size=392
	// 8263B188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B18C: 4866E275  bl 0x82ca9400
	ctx.lr = 0x8263B190;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B190: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B194: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B198: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B19C: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B1A4: 419A0150  beq cr6, 0x8263b2f4
	if ctx.cr[6].eq {
	pc = 0x8263B2F4; continue 'dispatch;
	}
	// 8263B1A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8263B1AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B1B0: 388B73A0  addi r4, r11, 0x73a0
	ctx.r[4].s64 = ctx.r[11].s64 + 29600;
	// 8263B1B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B1B8: 4BBF1D19  bl 0x8222ced0
	ctx.lr = 0x8263B1BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B1BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B1C4: 409A0010  bne cr6, 0x8263b1d4
	if !ctx.cr[6].eq {
	pc = 0x8263B1D4; continue 'dispatch;
	}
	// 8263B1C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B1CC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B1D0: 48000008  b 0x8263b1d8
	pc = 0x8263B1D8; continue 'dispatch;
	// 8263B1D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B1D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B1DC: 4BFF50E5  bl 0x826302c0
	ctx.lr = 0x8263B1E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B1E0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B1E4: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263B1E8: 392B0AA0  addi r9, r11, 0xaa0
	ctx.r[9].s64 = ctx.r[11].s64 + 2720;
	// 8263B1EC: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263B1F0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B1F4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B1F8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B1FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B200: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B204: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B208: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B20C: 4BBB5035  bl 0x821f0240
	ctx.lr = 0x8263B210;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B210: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263B214: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B218: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B21C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B220: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B224: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B228: 48018D09  bl 0x82653f30
	ctx.lr = 0x8263B22C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B22C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B230: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B238: 409A000C  bne cr6, 0x8263b244
	if !ctx.cr[6].eq {
	pc = 0x8263B244; continue 'dispatch;
	}
	// 8263B23C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B240: 48000010  b 0x8263b250
	pc = 0x8263B250; continue 'dispatch;
	// 8263B244: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B248: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B24C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B250: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B254: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B258: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B25C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B260: 4098002C  bge cr6, 0x8263b28c
	if !ctx.cr[6].lt {
	pc = 0x8263B28C; continue 'dispatch;
	}
	// 8263B264: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B268: 419A0018  beq cr6, 0x8263b280
	if ctx.cr[6].eq {
	pc = 0x8263B280; continue 'dispatch;
	}
	// 8263B26C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B270: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B274: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B278: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B27C: 48018CB5  bl 0x82653f30
	ctx.lr = 0x8263B280;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B280: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B284: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B288: 48000020  b 0x8263b2a8
	pc = 0x8263B2A8; continue 'dispatch;
	// 8263B28C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B290: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B294: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B298: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B2A0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B2A4: 48018DBD  bl 0x82654060
	ctx.lr = 0x8263B2A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B2A8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B2AC: 4BBD9B2D  bl 0x82214dd8
	ctx.lr = 0x8263B2B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B2B0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B2B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B2B8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B2BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B2C0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B2C4: 4BBB4F7D  bl 0x821f0240
	ctx.lr = 0x8263B2C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B2C8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B2CC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B2D4: 4BF3058D  bl 0x8256b860
	ctx.lr = 0x8263B2D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B2D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B2DC: 4BBD9AFD  bl 0x82214dd8
	ctx.lr = 0x8263B2E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B2E0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B2E4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B2E8: 4BBD9AF1  bl 0x82214dd8
	ctx.lr = 0x8263B2EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B2F0: 4BBD9AE9  bl 0x82214dd8
	ctx.lr = 0x8263B2F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B2F4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B2F8: 419A000C  beq cr6, 0x8263b304
	if ctx.cr[6].eq {
	pc = 0x8263B304; continue 'dispatch;
	}
	// 8263B2FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B300: 480B1491  bl 0x826ec790
	ctx.lr = 0x8263B304;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263B304: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B308: 4866E148  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B310 size=392
	// 8263B310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B314: 4866E0ED  bl 0x82ca9400
	ctx.lr = 0x8263B318;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B318: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B31C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B320: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B324: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B328: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B32C: 419A0150  beq cr6, 0x8263b47c
	if ctx.cr[6].eq {
	pc = 0x8263B47C; continue 'dispatch;
	}
	// 8263B330: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8263B334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B338: 388B6DE4  addi r4, r11, 0x6de4
	ctx.r[4].s64 = ctx.r[11].s64 + 28132;
	// 8263B33C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B340: 4BBF1B91  bl 0x8222ced0
	ctx.lr = 0x8263B344;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B344: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B34C: 409A0010  bne cr6, 0x8263b35c
	if !ctx.cr[6].eq {
	pc = 0x8263B35C; continue 'dispatch;
	}
	// 8263B350: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B354: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B358: 48000008  b 0x8263b360
	pc = 0x8263B360; continue 'dispatch;
	// 8263B35C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B360: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B364: 4BFF4F5D  bl 0x826302c0
	ctx.lr = 0x8263B368;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B368: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B36C: 3D40826F  lis r10, -0x7d91
	ctx.r[10].s64 = -2106654720;
	// 8263B370: 392B0900  addi r9, r11, 0x900
	ctx.r[9].s64 = ctx.r[11].s64 + 2304;
	// 8263B374: 390ACB80  addi r8, r10, -0x3480
	ctx.r[8].s64 = ctx.r[10].s64 + -13440;
	// 8263B378: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B37C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B380: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B384: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B388: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B38C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B390: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B394: 4BBB4EAD  bl 0x821f0240
	ctx.lr = 0x8263B398;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B398: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263B39C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B3A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B3A4: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B3A8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B3AC: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B3B0: 48018B81  bl 0x82653f30
	ctx.lr = 0x8263B3B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B3B4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B3B8: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B3C0: 409A000C  bne cr6, 0x8263b3cc
	if !ctx.cr[6].eq {
	pc = 0x8263B3CC; continue 'dispatch;
	}
	// 8263B3C4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B3C8: 48000010  b 0x8263b3d8
	pc = 0x8263B3D8; continue 'dispatch;
	// 8263B3CC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B3D0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B3D4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B3D8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B3DC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B3E0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B3E4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B3E8: 4098002C  bge cr6, 0x8263b414
	if !ctx.cr[6].lt {
	pc = 0x8263B414; continue 'dispatch;
	}
	// 8263B3EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B3F0: 419A0018  beq cr6, 0x8263b408
	if ctx.cr[6].eq {
	pc = 0x8263B408; continue 'dispatch;
	}
	// 8263B3F4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B3F8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B3FC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B400: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B404: 48018B2D  bl 0x82653f30
	ctx.lr = 0x8263B408;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B408: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B40C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B410: 48000020  b 0x8263b430
	pc = 0x8263B430; continue 'dispatch;
	// 8263B414: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B418: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B41C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B420: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B428: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B42C: 48018C35  bl 0x82654060
	ctx.lr = 0x8263B430;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B430: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B434: 4BBD99A5  bl 0x82214dd8
	ctx.lr = 0x8263B438;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B438: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B43C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B440: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B444: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B448: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B44C: 4BBB4DF5  bl 0x821f0240
	ctx.lr = 0x8263B450;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B450: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B454: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B45C: 4BF30405  bl 0x8256b860
	ctx.lr = 0x8263B460;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B460: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B464: 4BBD9975  bl 0x82214dd8
	ctx.lr = 0x8263B468;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B468: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B46C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B470: 4BBD9969  bl 0x82214dd8
	ctx.lr = 0x8263B474;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B474: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B478: 4BBD9961  bl 0x82214dd8
	ctx.lr = 0x8263B47C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B47C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B480: 419A000C  beq cr6, 0x8263b48c
	if ctx.cr[6].eq {
	pc = 0x8263B48C; continue 'dispatch;
	}
	// 8263B484: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B488: 480B1309  bl 0x826ec790
	ctx.lr = 0x8263B48C;
	crate::recompiler::externs::call(&mut ctx, base, 0x826EC790);
	// 8263B48C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B490: 4866DFC0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B498 size=392
	// 8263B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B49C: 4866DF65  bl 0x82ca9400
	ctx.lr = 0x8263B4A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B4A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B4A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B4A8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B4AC: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B4B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B4B4: 419A0150  beq cr6, 0x8263b604
	if ctx.cr[6].eq {
	pc = 0x8263B604; continue 'dispatch;
	}
	// 8263B4B8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263B4BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B4C0: 388B9344  addi r4, r11, -0x6cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -27836;
	// 8263B4C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B4C8: 4BBF1A09  bl 0x8222ced0
	ctx.lr = 0x8263B4CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B4CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B4D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B4D4: 409A0010  bne cr6, 0x8263b4e4
	if !ctx.cr[6].eq {
	pc = 0x8263B4E4; continue 'dispatch;
	}
	// 8263B4D8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B4DC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B4E0: 48000008  b 0x8263b4e8
	pc = 0x8263B4E8; continue 'dispatch;
	// 8263B4E4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B4E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B4EC: 4BFF4DD5  bl 0x826302c0
	ctx.lr = 0x8263B4F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B4F0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B4F4: 3D40825A  lis r10, -0x7da6
	ctx.r[10].s64 = -2108030976;
	// 8263B4F8: 392B0B30  addi r9, r11, 0xb30
	ctx.r[9].s64 = ctx.r[11].s64 + 2864;
	// 8263B4FC: 390A8518  addi r8, r10, -0x7ae8
	ctx.r[8].s64 = ctx.r[10].s64 + -31464;
	// 8263B500: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B504: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B508: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B50C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B510: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B514: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B518: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B51C: 4BBB4D25  bl 0x821f0240
	ctx.lr = 0x8263B520;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B520: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263B524: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B528: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B52C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B530: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B534: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B538: 480189F9  bl 0x82653f30
	ctx.lr = 0x8263B53C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B53C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B540: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B548: 409A000C  bne cr6, 0x8263b554
	if !ctx.cr[6].eq {
	pc = 0x8263B554; continue 'dispatch;
	}
	// 8263B54C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B550: 48000010  b 0x8263b560
	pc = 0x8263B560; continue 'dispatch;
	// 8263B554: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B558: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B55C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B560: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B564: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B568: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B56C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B570: 4098002C  bge cr6, 0x8263b59c
	if !ctx.cr[6].lt {
	pc = 0x8263B59C; continue 'dispatch;
	}
	// 8263B574: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B578: 419A0018  beq cr6, 0x8263b590
	if ctx.cr[6].eq {
	pc = 0x8263B590; continue 'dispatch;
	}
	// 8263B57C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B580: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B584: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B588: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B58C: 480189A5  bl 0x82653f30
	ctx.lr = 0x8263B590;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B590: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B594: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B598: 48000020  b 0x8263b5b8
	pc = 0x8263B5B8; continue 'dispatch;
	// 8263B59C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B5A0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B5A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B5A8: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B5B0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B5B4: 48018AAD  bl 0x82654060
	ctx.lr = 0x8263B5B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B5B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B5BC: 4BBD981D  bl 0x82214dd8
	ctx.lr = 0x8263B5C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B5C0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B5C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B5C8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B5CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B5D0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B5D4: 4BBB4C6D  bl 0x821f0240
	ctx.lr = 0x8263B5D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B5D8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B5DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B5E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B5E4: 4BF3027D  bl 0x8256b860
	ctx.lr = 0x8263B5E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B5E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B5EC: 4BBD97ED  bl 0x82214dd8
	ctx.lr = 0x8263B5F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B5F0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B5F4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B5F8: 4BBD97E1  bl 0x82214dd8
	ctx.lr = 0x8263B5FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B5FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B600: 4BBD97D9  bl 0x82214dd8
	ctx.lr = 0x8263B604;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B604: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B608: 419A000C  beq cr6, 0x8263b614
	if ctx.cr[6].eq {
	pc = 0x8263B614; continue 'dispatch;
	}
	// 8263B60C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B610: 483154C9  bl 0x82950ad8
	ctx.lr = 0x8263B614;
	crate::recompiler::externs::call(&mut ctx, base, 0x82950AD8);
	// 8263B614: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B618: 4866DE38  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B620 size=392
	// 8263B620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B624: 4866DDDD  bl 0x82ca9400
	ctx.lr = 0x8263B628;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B628: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B62C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B630: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B634: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B63C: 419A0150  beq cr6, 0x8263b78c
	if ctx.cr[6].eq {
	pc = 0x8263B78C; continue 'dispatch;
	}
	// 8263B640: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263B644: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B648: 388B936C  addi r4, r11, -0x6c94
	ctx.r[4].s64 = ctx.r[11].s64 + -27796;
	// 8263B64C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B650: 4BBF1881  bl 0x8222ced0
	ctx.lr = 0x8263B654;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B654: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B65C: 409A0010  bne cr6, 0x8263b66c
	if !ctx.cr[6].eq {
	pc = 0x8263B66C; continue 'dispatch;
	}
	// 8263B660: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B664: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B668: 48000008  b 0x8263b670
	pc = 0x8263B670; continue 'dispatch;
	// 8263B66C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B670: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B674: 4BFF4C4D  bl 0x826302c0
	ctx.lr = 0x8263B678;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B678: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B67C: 3D40825A  lis r10, -0x7da6
	ctx.r[10].s64 = -2108030976;
	// 8263B680: 392B0BB0  addi r9, r11, 0xbb0
	ctx.r[9].s64 = ctx.r[11].s64 + 2992;
	// 8263B684: 390A8518  addi r8, r10, -0x7ae8
	ctx.r[8].s64 = ctx.r[10].s64 + -31464;
	// 8263B688: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B68C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B690: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B694: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B698: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B69C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B6A0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B6A4: 4BBB4B9D  bl 0x821f0240
	ctx.lr = 0x8263B6A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B6A8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263B6AC: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B6B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B6B4: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B6B8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B6BC: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B6C0: 48018871  bl 0x82653f30
	ctx.lr = 0x8263B6C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B6C4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B6C8: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B6D0: 409A000C  bne cr6, 0x8263b6dc
	if !ctx.cr[6].eq {
	pc = 0x8263B6DC; continue 'dispatch;
	}
	// 8263B6D4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B6D8: 48000010  b 0x8263b6e8
	pc = 0x8263B6E8; continue 'dispatch;
	// 8263B6DC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B6E0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B6E4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B6E8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B6EC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B6F0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B6F4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B6F8: 4098002C  bge cr6, 0x8263b724
	if !ctx.cr[6].lt {
	pc = 0x8263B724; continue 'dispatch;
	}
	// 8263B6FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B700: 419A0018  beq cr6, 0x8263b718
	if ctx.cr[6].eq {
	pc = 0x8263B718; continue 'dispatch;
	}
	// 8263B704: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B708: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B70C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B710: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B714: 4801881D  bl 0x82653f30
	ctx.lr = 0x8263B718;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B718: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B71C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B720: 48000020  b 0x8263b740
	pc = 0x8263B740; continue 'dispatch;
	// 8263B724: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B728: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B72C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B730: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B738: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B73C: 48018925  bl 0x82654060
	ctx.lr = 0x8263B740;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B740: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B744: 4BBD9695  bl 0x82214dd8
	ctx.lr = 0x8263B748;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B748: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B74C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B750: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B754: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B758: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B75C: 4BBB4AE5  bl 0x821f0240
	ctx.lr = 0x8263B760;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B760: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B764: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B76C: 4BF300F5  bl 0x8256b860
	ctx.lr = 0x8263B770;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B770: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B774: 4BBD9665  bl 0x82214dd8
	ctx.lr = 0x8263B778;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B778: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B77C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B780: 4BBD9659  bl 0x82214dd8
	ctx.lr = 0x8263B784;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B784: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B788: 4BBD9651  bl 0x82214dd8
	ctx.lr = 0x8263B78C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B78C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B790: 419A000C  beq cr6, 0x8263b79c
	if ctx.cr[6].eq {
	pc = 0x8263B79C; continue 'dispatch;
	}
	// 8263B794: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B798: 4BF5C801  bl 0x82597f98
	ctx.lr = 0x8263B79C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82597F98);
	// 8263B79C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B7A0: 4866DCB0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B7A8 size=392
	// 8263B7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B7AC: 4866DC55  bl 0x82ca9400
	ctx.lr = 0x8263B7B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263B7B0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B7B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B7B8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263B7BC: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B7C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B7C4: 419A0150  beq cr6, 0x8263b914
	if ctx.cr[6].eq {
	pc = 0x8263B914; continue 'dispatch;
	}
	// 8263B7C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8263B7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B7D0: 388B4CDC  addi r4, r11, 0x4cdc
	ctx.r[4].s64 = ctx.r[11].s64 + 19676;
	// 8263B7D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B7D8: 4BBF16F9  bl 0x8222ced0
	ctx.lr = 0x8263B7DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B7DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B7E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B7E4: 409A0010  bne cr6, 0x8263b7f4
	if !ctx.cr[6].eq {
	pc = 0x8263B7F4; continue 'dispatch;
	}
	// 8263B7E8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B7EC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B7F0: 48000008  b 0x8263b7f8
	pc = 0x8263B7F8; continue 'dispatch;
	// 8263B7F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B7F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B7FC: 4BFF4AC5  bl 0x826302c0
	ctx.lr = 0x8263B800;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B800: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B804: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263B808: 392B0CD0  addi r9, r11, 0xcd0
	ctx.r[9].s64 = ctx.r[11].s64 + 3280;
	// 8263B80C: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263B810: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B814: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B818: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B81C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B820: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B824: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B828: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B82C: 4BBB4A15  bl 0x821f0240
	ctx.lr = 0x8263B830;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B830: 38E0003C  li r7, 0x3c
	ctx.r[7].s64 = 60;
	// 8263B834: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B838: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B83C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B840: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B844: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B848: 480186E9  bl 0x82653f30
	ctx.lr = 0x8263B84C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B84C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B850: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B858: 409A000C  bne cr6, 0x8263b864
	if !ctx.cr[6].eq {
	pc = 0x8263B864; continue 'dispatch;
	}
	// 8263B85C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B860: 48000010  b 0x8263b870
	pc = 0x8263B870; continue 'dispatch;
	// 8263B864: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B868: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B86C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B870: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B874: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B878: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263B87C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263B880: 4098002C  bge cr6, 0x8263b8ac
	if !ctx.cr[6].lt {
	pc = 0x8263B8AC; continue 'dispatch;
	}
	// 8263B884: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263B888: 419A0018  beq cr6, 0x8263b8a0
	if ctx.cr[6].eq {
	pc = 0x8263B8A0; continue 'dispatch;
	}
	// 8263B88C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263B890: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263B894: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263B898: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263B89C: 48018695  bl 0x82653f30
	ctx.lr = 0x8263B8A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B8A0: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263B8A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263B8A8: 48000020  b 0x8263b8c8
	pc = 0x8263B8C8; continue 'dispatch;
	// 8263B8AC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263B8B0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263B8B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263B8B8: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263B8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B8C0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263B8C4: 4801879D  bl 0x82654060
	ctx.lr = 0x8263B8C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263B8C8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263B8CC: 4BBD950D  bl 0x82214dd8
	ctx.lr = 0x8263B8D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B8D0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263B8D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B8D8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263B8DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B8E0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263B8E4: 4BBB495D  bl 0x821f0240
	ctx.lr = 0x8263B8E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B8E8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263B8EC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263B8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263B8F4: 4BF2FF6D  bl 0x8256b860
	ctx.lr = 0x8263B8F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263B8F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263B8FC: 4BBD94DD  bl 0x82214dd8
	ctx.lr = 0x8263B900;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B900: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263B904: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B908: 4BBD94D1  bl 0x82214dd8
	ctx.lr = 0x8263B90C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B90C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B910: 4BBD94C9  bl 0x82214dd8
	ctx.lr = 0x8263B914;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263B914: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263B918: 419A000C  beq cr6, 0x8263b924
	if ctx.cr[6].eq {
	pc = 0x8263B924; continue 'dispatch;
	}
	// 8263B91C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263B920: 4802B481  bl 0x82666da0
	ctx.lr = 0x8263B924;
	crate::recompiler::externs::call(&mut ctx, base, 0x82666DA0);
	// 8263B924: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263B928: 4866DB28  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B930 size=368
	// 8263B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B934: 4866DAD1  bl 0x82ca9404
	ctx.lr = 0x8263B938;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263B938: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B93C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263B940: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263B944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B948: 419A0150  beq cr6, 0x8263ba98
	if ctx.cr[6].eq {
	pc = 0x8263BA98; continue 'dispatch;
	}
	// 8263B94C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263B950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263B954: 388B9394  addi r4, r11, -0x6c6c
	ctx.r[4].s64 = ctx.r[11].s64 + -27756;
	// 8263B958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263B95C: 4BBF1575  bl 0x8222ced0
	ctx.lr = 0x8263B960;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263B960: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263B964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B968: 409A0010  bne cr6, 0x8263b978
	if !ctx.cr[6].eq {
	pc = 0x8263B978; continue 'dispatch;
	}
	// 8263B96C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263B970: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263B974: 48000008  b 0x8263b97c
	pc = 0x8263B97C; continue 'dispatch;
	// 8263B978: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263B97C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263B980: 4BFF4941  bl 0x826302c0
	ctx.lr = 0x8263B984;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263B984: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263B988: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263B98C: 392B0D28  addi r9, r11, 0xd28
	ctx.r[9].s64 = ctx.r[11].s64 + 3368;
	// 8263B990: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263B994: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263B998: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263B99C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263B9A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263B9A4: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263B9A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263B9AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263B9B0: 4BBB4891  bl 0x821f0240
	ctx.lr = 0x8263B9B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263B9B4: 38E00048  li r7, 0x48
	ctx.r[7].s64 = 72;
	// 8263B9B8: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263B9BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263B9C0: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263B9C4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263B9C8: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263B9CC: 48018565  bl 0x82653f30
	ctx.lr = 0x8263B9D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263B9D0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B9D4: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263B9D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263B9DC: 409A000C  bne cr6, 0x8263b9e8
	if !ctx.cr[6].eq {
	pc = 0x8263B9E8; continue 'dispatch;
	}
	// 8263B9E0: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263B9E4: 48000010  b 0x8263b9f4
	pc = 0x8263B9F4; continue 'dispatch;
	// 8263B9E8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263B9EC: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263B9F0: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263B9F4: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263B9F8: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263B9FC: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263BA00: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263BA04: 4098002C  bge cr6, 0x8263ba30
	if !ctx.cr[6].lt {
	pc = 0x8263BA30; continue 'dispatch;
	}
	// 8263BA08: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263BA0C: 419A0018  beq cr6, 0x8263ba24
	if ctx.cr[6].eq {
	pc = 0x8263BA24; continue 'dispatch;
	}
	// 8263BA10: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263BA14: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263BA18: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263BA1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263BA20: 48018511  bl 0x82653f30
	ctx.lr = 0x8263BA24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BA24: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263BA28: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263BA2C: 48000020  b 0x8263ba4c
	pc = 0x8263BA4C; continue 'dispatch;
	// 8263BA30: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263BA34: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263BA38: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263BA3C: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263BA40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BA44: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263BA48: 48018619  bl 0x82654060
	ctx.lr = 0x8263BA4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263BA4C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263BA50: 4BBD9389  bl 0x82214dd8
	ctx.lr = 0x8263BA54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BA54: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263BA58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BA5C: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263BA60: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BA64: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263BA68: 4BBB47D9  bl 0x821f0240
	ctx.lr = 0x8263BA6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BA6C: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263BA70: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263BA74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BA78: 4BF2FDE9  bl 0x8256b860
	ctx.lr = 0x8263BA7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263BA7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BA80: 4BBD9359  bl 0x82214dd8
	ctx.lr = 0x8263BA84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BA84: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263BA88: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BA8C: 4BBD934D  bl 0x82214dd8
	ctx.lr = 0x8263BA90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BA90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BA94: 4BBD9345  bl 0x82214dd8
	ctx.lr = 0x8263BA98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BA98: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263BA9C: 4866D9B8  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263BAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BAA0 size=392
	// 8263BAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BAA4: 4866D95D  bl 0x82ca9400
	ctx.lr = 0x8263BAA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263BAA8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BAAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263BAB0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263BAB4: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263BAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BABC: 419A0150  beq cr6, 0x8263bc0c
	if ctx.cr[6].eq {
	pc = 0x8263BC0C; continue 'dispatch;
	}
	// 8263BAC0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8263BAC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263BAC8: 388B33B4  addi r4, r11, 0x33b4
	ctx.r[4].s64 = ctx.r[11].s64 + 13236;
	// 8263BACC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BAD0: 4BBF1401  bl 0x8222ced0
	ctx.lr = 0x8263BAD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263BAD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263BAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BADC: 409A0010  bne cr6, 0x8263baec
	if !ctx.cr[6].eq {
	pc = 0x8263BAEC; continue 'dispatch;
	}
	// 8263BAE0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263BAE4: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263BAE8: 48000008  b 0x8263baf0
	pc = 0x8263BAF0; continue 'dispatch;
	// 8263BAEC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263BAF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263BAF4: 4BFF47CD  bl 0x826302c0
	ctx.lr = 0x8263BAF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263BAF8: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263BAFC: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263BB00: 392B0DA8  addi r9, r11, 0xda8
	ctx.r[9].s64 = ctx.r[11].s64 + 3496;
	// 8263BB04: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263BB08: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263BB0C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263BB10: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263BB14: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263BB18: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263BB1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BB20: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BB24: 4BBB471D  bl 0x821f0240
	ctx.lr = 0x8263BB28;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BB28: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263BB2C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263BB30: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263BB34: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263BB38: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263BB3C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263BB40: 480183F1  bl 0x82653f30
	ctx.lr = 0x8263BB44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BB44: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BB48: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263BB4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BB50: 409A000C  bne cr6, 0x8263bb5c
	if !ctx.cr[6].eq {
	pc = 0x8263BB5C; continue 'dispatch;
	}
	// 8263BB54: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263BB58: 48000010  b 0x8263bb68
	pc = 0x8263BB68; continue 'dispatch;
	// 8263BB5C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263BB60: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263BB64: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263BB68: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BB6C: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263BB70: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263BB74: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263BB78: 4098002C  bge cr6, 0x8263bba4
	if !ctx.cr[6].lt {
	pc = 0x8263BBA4; continue 'dispatch;
	}
	// 8263BB7C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263BB80: 419A0018  beq cr6, 0x8263bb98
	if ctx.cr[6].eq {
	pc = 0x8263BB98; continue 'dispatch;
	}
	// 8263BB84: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263BB88: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263BB8C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263BB90: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263BB94: 4801839D  bl 0x82653f30
	ctx.lr = 0x8263BB98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BB98: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263BB9C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263BBA0: 48000020  b 0x8263bbc0
	pc = 0x8263BBC0; continue 'dispatch;
	// 8263BBA4: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263BBA8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263BBAC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263BBB0: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263BBB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BBB8: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263BBBC: 480184A5  bl 0x82654060
	ctx.lr = 0x8263BBC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263BBC0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263BBC4: 4BBD9215  bl 0x82214dd8
	ctx.lr = 0x8263BBC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BBC8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263BBCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BBD0: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263BBD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BBD8: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263BBDC: 4BBB4665  bl 0x821f0240
	ctx.lr = 0x8263BBE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BBE0: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263BBE4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263BBE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BBEC: 4BF2FC75  bl 0x8256b860
	ctx.lr = 0x8263BBF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263BBF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BBF4: 4BBD91E5  bl 0x82214dd8
	ctx.lr = 0x8263BBF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BBF8: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263BBFC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BC00: 4BBD91D9  bl 0x82214dd8
	ctx.lr = 0x8263BC04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BC04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BC08: 4BBD91D1  bl 0x82214dd8
	ctx.lr = 0x8263BC0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BC0C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263BC10: 419A000C  beq cr6, 0x8263bc1c
	if ctx.cr[6].eq {
	pc = 0x8263BC1C; continue 'dispatch;
	}
	// 8263BC14: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263BC18: 4BF393F9  bl 0x82575010
	ctx.lr = 0x8263BC1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82575010);
	// 8263BC1C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263BC20: 4866D830  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BC28 size=368
	// 8263BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BC2C: 4866D7D9  bl 0x82ca9404
	ctx.lr = 0x8263BC30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263BC30: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BC34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263BC38: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263BC3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BC40: 419A0150  beq cr6, 0x8263bd90
	if ctx.cr[6].eq {
	pc = 0x8263BD90; continue 'dispatch;
	}
	// 8263BC44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263BC48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263BC4C: 388B93A8  addi r4, r11, -0x6c58
	ctx.r[4].s64 = ctx.r[11].s64 + -27736;
	// 8263BC50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BC54: 4BBF127D  bl 0x8222ced0
	ctx.lr = 0x8263BC58;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263BC58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263BC5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BC60: 409A0010  bne cr6, 0x8263bc70
	if !ctx.cr[6].eq {
	pc = 0x8263BC70; continue 'dispatch;
	}
	// 8263BC64: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263BC68: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263BC6C: 48000008  b 0x8263bc74
	pc = 0x8263BC74; continue 'dispatch;
	// 8263BC70: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263BC74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263BC78: 4BFF4649  bl 0x826302c0
	ctx.lr = 0x8263BC7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263BC7C: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263BC80: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263BC84: 392B0E00  addi r9, r11, 0xe00
	ctx.r[9].s64 = ctx.r[11].s64 + 3584;
	// 8263BC88: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263BC8C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263BC90: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263BC94: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263BC98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263BC9C: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263BCA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BCA4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BCA8: 4BBB4599  bl 0x821f0240
	ctx.lr = 0x8263BCAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BCAC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8263BCB0: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263BCB4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263BCB8: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263BCBC: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263BCC0: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263BCC4: 4801826D  bl 0x82653f30
	ctx.lr = 0x8263BCC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BCC8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BCCC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263BCD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BCD4: 409A000C  bne cr6, 0x8263bce0
	if !ctx.cr[6].eq {
	pc = 0x8263BCE0; continue 'dispatch;
	}
	// 8263BCD8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263BCDC: 48000010  b 0x8263bcec
	pc = 0x8263BCEC; continue 'dispatch;
	// 8263BCE0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263BCE4: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263BCE8: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263BCEC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BCF0: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263BCF4: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263BCF8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263BCFC: 4098002C  bge cr6, 0x8263bd28
	if !ctx.cr[6].lt {
	pc = 0x8263BD28; continue 'dispatch;
	}
	// 8263BD00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263BD04: 419A0018  beq cr6, 0x8263bd1c
	if ctx.cr[6].eq {
	pc = 0x8263BD1C; continue 'dispatch;
	}
	// 8263BD08: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263BD0C: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263BD10: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263BD14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263BD18: 48018219  bl 0x82653f30
	ctx.lr = 0x8263BD1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BD1C: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263BD20: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263BD24: 48000020  b 0x8263bd44
	pc = 0x8263BD44; continue 'dispatch;
	// 8263BD28: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263BD2C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263BD30: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263BD34: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263BD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BD3C: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263BD40: 48018321  bl 0x82654060
	ctx.lr = 0x8263BD44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263BD44: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263BD48: 4BBD9091  bl 0x82214dd8
	ctx.lr = 0x8263BD4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BD4C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263BD50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BD54: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263BD58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BD5C: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263BD60: 4BBB44E1  bl 0x821f0240
	ctx.lr = 0x8263BD64;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BD64: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263BD68: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263BD6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BD70: 4BF2FAF1  bl 0x8256b860
	ctx.lr = 0x8263BD74;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263BD74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BD78: 4BBD9061  bl 0x82214dd8
	ctx.lr = 0x8263BD7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BD7C: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263BD80: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BD84: 4BBD9055  bl 0x82214dd8
	ctx.lr = 0x8263BD88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BD88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BD8C: 4BBD904D  bl 0x82214dd8
	ctx.lr = 0x8263BD90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BD90: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263BD94: 4866D6C0  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BD98 size=368
	// 8263BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BD9C: 4866D669  bl 0x82ca9404
	ctx.lr = 0x8263BDA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263BDA0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BDA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263BDA8: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263BDAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BDB0: 419A0150  beq cr6, 0x8263bf00
	if ctx.cr[6].eq {
	pc = 0x8263BF00; continue 'dispatch;
	}
	// 8263BDB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263BDB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263BDBC: 388B93C8  addi r4, r11, -0x6c38
	ctx.r[4].s64 = ctx.r[11].s64 + -27704;
	// 8263BDC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BDC4: 4BBF110D  bl 0x8222ced0
	ctx.lr = 0x8263BDC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263BDC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263BDCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BDD0: 409A0010  bne cr6, 0x8263bde0
	if !ctx.cr[6].eq {
	pc = 0x8263BDE0; continue 'dispatch;
	}
	// 8263BDD4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263BDD8: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263BDDC: 48000008  b 0x8263bde4
	pc = 0x8263BDE4; continue 'dispatch;
	// 8263BDE0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263BDE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263BDE8: 4BFF44D9  bl 0x826302c0
	ctx.lr = 0x8263BDEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263BDEC: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263BDF0: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263BDF4: 392B0E98  addi r9, r11, 0xe98
	ctx.r[9].s64 = ctx.r[11].s64 + 3736;
	// 8263BDF8: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263BDFC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263BE00: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263BE04: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263BE08: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263BE0C: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263BE10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BE14: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BE18: 4BBB4429  bl 0x821f0240
	ctx.lr = 0x8263BE1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BE1C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263BE20: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263BE24: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263BE28: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263BE2C: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263BE30: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263BE34: 480180FD  bl 0x82653f30
	ctx.lr = 0x8263BE38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BE38: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BE3C: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263BE40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BE44: 409A000C  bne cr6, 0x8263be50
	if !ctx.cr[6].eq {
	pc = 0x8263BE50; continue 'dispatch;
	}
	// 8263BE48: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263BE4C: 48000010  b 0x8263be5c
	pc = 0x8263BE5C; continue 'dispatch;
	// 8263BE50: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263BE54: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263BE58: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263BE5C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BE60: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263BE64: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263BE68: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263BE6C: 4098002C  bge cr6, 0x8263be98
	if !ctx.cr[6].lt {
	pc = 0x8263BE98; continue 'dispatch;
	}
	// 8263BE70: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263BE74: 419A0018  beq cr6, 0x8263be8c
	if ctx.cr[6].eq {
	pc = 0x8263BE8C; continue 'dispatch;
	}
	// 8263BE78: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263BE7C: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263BE80: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263BE84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263BE88: 480180A9  bl 0x82653f30
	ctx.lr = 0x8263BE8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BE8C: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263BE90: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263BE94: 48000020  b 0x8263beb4
	pc = 0x8263BEB4; continue 'dispatch;
	// 8263BE98: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263BE9C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263BEA0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263BEA4: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263BEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BEAC: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263BEB0: 480181B1  bl 0x82654060
	ctx.lr = 0x8263BEB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263BEB4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263BEB8: 4BBD8F21  bl 0x82214dd8
	ctx.lr = 0x8263BEBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BEBC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263BEC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BEC4: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263BEC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BECC: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263BED0: 4BBB4371  bl 0x821f0240
	ctx.lr = 0x8263BED4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BED4: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263BED8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263BEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263BEE0: 4BF2F981  bl 0x8256b860
	ctx.lr = 0x8263BEE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263BEE4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263BEE8: 4BBD8EF1  bl 0x82214dd8
	ctx.lr = 0x8263BEEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BEEC: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263BEF0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BEF4: 4BBD8EE5  bl 0x82214dd8
	ctx.lr = 0x8263BEF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BEF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BEFC: 4BBD8EDD  bl 0x82214dd8
	ctx.lr = 0x8263BF00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263BF00: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263BF04: 4866D550  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263BF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BF08 size=368
	// 8263BF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BF0C: 4866D4F9  bl 0x82ca9404
	ctx.lr = 0x8263BF10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263BF10: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BF14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263BF18: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263BF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BF20: 419A0150  beq cr6, 0x8263c070
	if ctx.cr[6].eq {
	pc = 0x8263C070; continue 'dispatch;
	}
	// 8263BF24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8263BF28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263BF2C: 388B864C  addi r4, r11, -0x79b4
	ctx.r[4].s64 = ctx.r[11].s64 + -31156;
	// 8263BF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263BF34: 4BBF0F9D  bl 0x8222ced0
	ctx.lr = 0x8263BF38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263BF38: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263BF3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BF40: 409A0010  bne cr6, 0x8263bf50
	if !ctx.cr[6].eq {
	pc = 0x8263BF50; continue 'dispatch;
	}
	// 8263BF44: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263BF48: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263BF4C: 48000008  b 0x8263bf54
	pc = 0x8263BF54; continue 'dispatch;
	// 8263BF50: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263BF54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263BF58: 4BFF4369  bl 0x826302c0
	ctx.lr = 0x8263BF5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263BF5C: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263BF60: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263BF64: 392B0F28  addi r9, r11, 0xf28
	ctx.r[9].s64 = ctx.r[11].s64 + 3880;
	// 8263BF68: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263BF6C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263BF70: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263BF74: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263BF78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263BF7C: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263BF80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263BF84: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263BF88: 4BBB42B9  bl 0x821f0240
	ctx.lr = 0x8263BF8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263BF8C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8263BF90: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263BF94: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263BF98: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263BF9C: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263BFA0: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263BFA4: 48017F8D  bl 0x82653f30
	ctx.lr = 0x8263BFA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BFA8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BFAC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263BFB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263BFB4: 409A000C  bne cr6, 0x8263bfc0
	if !ctx.cr[6].eq {
	pc = 0x8263BFC0; continue 'dispatch;
	}
	// 8263BFB8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263BFBC: 48000010  b 0x8263bfcc
	pc = 0x8263BFCC; continue 'dispatch;
	// 8263BFC0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263BFC4: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263BFC8: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263BFCC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263BFD0: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263BFD4: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263BFD8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263BFDC: 4098002C  bge cr6, 0x8263c008
	if !ctx.cr[6].lt {
	pc = 0x8263C008; continue 'dispatch;
	}
	// 8263BFE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263BFE4: 419A0018  beq cr6, 0x8263bffc
	if ctx.cr[6].eq {
	pc = 0x8263BFFC; continue 'dispatch;
	}
	// 8263BFE8: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263BFEC: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263BFF0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263BFF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263BFF8: 48017F39  bl 0x82653f30
	ctx.lr = 0x8263BFFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263BFFC: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C000: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C004: 48000020  b 0x8263c024
	pc = 0x8263C024; continue 'dispatch;
	// 8263C008: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C00C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C010: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C014: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C01C: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C020: 48018041  bl 0x82654060
	ctx.lr = 0x8263C024;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C024: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C028: 4BBD8DB1  bl 0x82214dd8
	ctx.lr = 0x8263C02C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C02C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C030: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C034: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C038: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C03C: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C040: 4BBB4201  bl 0x821f0240
	ctx.lr = 0x8263C044;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C044: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C048: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C050: 4BF2F811  bl 0x8256b860
	ctx.lr = 0x8263C054;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C054: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C058: 4BBD8D81  bl 0x82214dd8
	ctx.lr = 0x8263C05C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C05C: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C060: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C064: 4BBD8D75  bl 0x82214dd8
	ctx.lr = 0x8263C068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C068: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C06C: 4BBD8D6D  bl 0x82214dd8
	ctx.lr = 0x8263C070;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C070: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C074: 4866D3E0  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263C078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C078 size=392
	// 8263C078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C07C: 4866D385  bl 0x82ca9400
	ctx.lr = 0x8263C080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263C080: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C084: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C088: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263C08C: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C094: 419A0150  beq cr6, 0x8263c1e4
	if ctx.cr[6].eq {
	pc = 0x8263C1E4; continue 'dispatch;
	}
	// 8263C098: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8263C09C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C0A0: 388B86F8  addi r4, r11, -0x7908
	ctx.r[4].s64 = ctx.r[11].s64 + -30984;
	// 8263C0A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C0A8: 4BBF0E29  bl 0x8222ced0
	ctx.lr = 0x8263C0AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C0AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C0B4: 409A0010  bne cr6, 0x8263c0c4
	if !ctx.cr[6].eq {
	pc = 0x8263C0C4; continue 'dispatch;
	}
	// 8263C0B8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C0BC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C0C0: 48000008  b 0x8263c0c8
	pc = 0x8263C0C8; continue 'dispatch;
	// 8263C0C4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C0C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C0CC: 4BFF41F5  bl 0x826302c0
	ctx.lr = 0x8263C0D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C0D0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C0D4: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C0D8: 392B0F80  addi r9, r11, 0xf80
	ctx.r[9].s64 = ctx.r[11].s64 + 3968;
	// 8263C0DC: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C0E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C0E4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C0E8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C0EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C0F0: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C0F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C0F8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C0FC: 4BBB4145  bl 0x821f0240
	ctx.lr = 0x8263C100;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C100: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8263C104: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C108: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C10C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C110: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C114: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C118: 48017E19  bl 0x82653f30
	ctx.lr = 0x8263C11C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C11C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C120: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C128: 409A000C  bne cr6, 0x8263c134
	if !ctx.cr[6].eq {
	pc = 0x8263C134; continue 'dispatch;
	}
	// 8263C12C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C130: 48000010  b 0x8263c140
	pc = 0x8263C140; continue 'dispatch;
	// 8263C134: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C138: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C13C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C140: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C144: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C148: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C14C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C150: 4098002C  bge cr6, 0x8263c17c
	if !ctx.cr[6].lt {
	pc = 0x8263C17C; continue 'dispatch;
	}
	// 8263C154: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C158: 419A0018  beq cr6, 0x8263c170
	if ctx.cr[6].eq {
	pc = 0x8263C170; continue 'dispatch;
	}
	// 8263C15C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C160: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C164: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C168: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C16C: 48017DC5  bl 0x82653f30
	ctx.lr = 0x8263C170;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C170: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C174: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C178: 48000020  b 0x8263c198
	pc = 0x8263C198; continue 'dispatch;
	// 8263C17C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C180: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C184: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C188: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C18C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C190: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C194: 48017ECD  bl 0x82654060
	ctx.lr = 0x8263C198;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C198: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C19C: 4BBD8C3D  bl 0x82214dd8
	ctx.lr = 0x8263C1A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C1A0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C1A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C1A8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C1AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C1B0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C1B4: 4BBB408D  bl 0x821f0240
	ctx.lr = 0x8263C1B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C1B8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C1BC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C1C4: 4BF2F69D  bl 0x8256b860
	ctx.lr = 0x8263C1C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C1C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C1CC: 4BBD8C0D  bl 0x82214dd8
	ctx.lr = 0x8263C1D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C1D0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C1D4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C1D8: 4BBD8C01  bl 0x82214dd8
	ctx.lr = 0x8263C1DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C1DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C1E0: 4BBD8BF9  bl 0x82214dd8
	ctx.lr = 0x8263C1E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C1E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263C1E8: 419A000C  beq cr6, 0x8263c1f4
	if ctx.cr[6].eq {
	pc = 0x8263C1F4; continue 'dispatch;
	}
	// 8263C1EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263C1F0: 4BF7D529  bl 0x825b9718
	ctx.lr = 0x8263C1F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x825B9718);
	// 8263C1F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C1F8: 4866D258  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C200 size=392
	// 8263C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C204: 4866D1FD  bl 0x82ca9400
	ctx.lr = 0x8263C208;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263C208: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C20C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C210: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263C214: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C21C: 419A0150  beq cr6, 0x8263c36c
	if ctx.cr[6].eq {
	pc = 0x8263C36C; continue 'dispatch;
	}
	// 8263C220: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8263C224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C228: 388B86DC  addi r4, r11, -0x7924
	ctx.r[4].s64 = ctx.r[11].s64 + -31012;
	// 8263C22C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C230: 4BBF0CA1  bl 0x8222ced0
	ctx.lr = 0x8263C234;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C234: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C23C: 409A0010  bne cr6, 0x8263c24c
	if !ctx.cr[6].eq {
	pc = 0x8263C24C; continue 'dispatch;
	}
	// 8263C240: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C244: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C248: 48000008  b 0x8263c250
	pc = 0x8263C250; continue 'dispatch;
	// 8263C24C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C254: 4BFF406D  bl 0x826302c0
	ctx.lr = 0x8263C258;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C258: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C25C: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C260: 392B0FD8  addi r9, r11, 0xfd8
	ctx.r[9].s64 = ctx.r[11].s64 + 4056;
	// 8263C264: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C268: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C26C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C270: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C274: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C278: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C27C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C280: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C284: 4BBB3FBD  bl 0x821f0240
	ctx.lr = 0x8263C288;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C288: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263C28C: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C290: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C294: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C298: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C29C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C2A0: 48017C91  bl 0x82653f30
	ctx.lr = 0x8263C2A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C2A4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C2A8: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C2B0: 409A000C  bne cr6, 0x8263c2bc
	if !ctx.cr[6].eq {
	pc = 0x8263C2BC; continue 'dispatch;
	}
	// 8263C2B4: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C2B8: 48000010  b 0x8263c2c8
	pc = 0x8263C2C8; continue 'dispatch;
	// 8263C2BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C2C0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C2C4: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C2C8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C2CC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C2D0: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C2D4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C2D8: 4098002C  bge cr6, 0x8263c304
	if !ctx.cr[6].lt {
	pc = 0x8263C304; continue 'dispatch;
	}
	// 8263C2DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C2E0: 419A0018  beq cr6, 0x8263c2f8
	if ctx.cr[6].eq {
	pc = 0x8263C2F8; continue 'dispatch;
	}
	// 8263C2E4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C2E8: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C2EC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C2F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C2F4: 48017C3D  bl 0x82653f30
	ctx.lr = 0x8263C2F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C2F8: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C2FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C300: 48000020  b 0x8263c320
	pc = 0x8263C320; continue 'dispatch;
	// 8263C304: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C308: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C30C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C310: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C318: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C31C: 48017D45  bl 0x82654060
	ctx.lr = 0x8263C320;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C320: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C324: 4BBD8AB5  bl 0x82214dd8
	ctx.lr = 0x8263C328;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C328: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C32C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C330: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C334: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C338: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C33C: 4BBB3F05  bl 0x821f0240
	ctx.lr = 0x8263C340;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C340: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C344: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C34C: 4BF2F515  bl 0x8256b860
	ctx.lr = 0x8263C350;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C350: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C354: 4BBD8A85  bl 0x82214dd8
	ctx.lr = 0x8263C358;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C358: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C35C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C360: 4BBD8A79  bl 0x82214dd8
	ctx.lr = 0x8263C364;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C364: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C368: 4BBD8A71  bl 0x82214dd8
	ctx.lr = 0x8263C36C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C36C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263C370: 419A000C  beq cr6, 0x8263c37c
	if ctx.cr[6].eq {
	pc = 0x8263C37C; continue 'dispatch;
	}
	// 8263C374: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263C378: 4BF61701  bl 0x8259da78
	ctx.lr = 0x8263C37C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8259DA78);
	// 8263C37C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C380: 4866D0D0  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C388 size=392
	// 8263C388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C38C: 4866D075  bl 0x82ca9400
	ctx.lr = 0x8263C390;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263C390: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C394: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C398: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263C39C: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C3A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C3A4: 419A0150  beq cr6, 0x8263c4f4
	if ctx.cr[6].eq {
	pc = 0x8263C4F4; continue 'dispatch;
	}
	// 8263C3A8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8263C3AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C3B0: 388B57B0  addi r4, r11, 0x57b0
	ctx.r[4].s64 = ctx.r[11].s64 + 22448;
	// 8263C3B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C3B8: 4BBF0B19  bl 0x8222ced0
	ctx.lr = 0x8263C3BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C3BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C3C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C3C4: 409A0010  bne cr6, 0x8263c3d4
	if !ctx.cr[6].eq {
	pc = 0x8263C3D4; continue 'dispatch;
	}
	// 8263C3C8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C3CC: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C3D0: 48000008  b 0x8263c3d8
	pc = 0x8263C3D8; continue 'dispatch;
	// 8263C3D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C3D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C3DC: 4BFF3EE5  bl 0x826302c0
	ctx.lr = 0x8263C3E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C3E0: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C3E4: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C3E8: 392B10D0  addi r9, r11, 0x10d0
	ctx.r[9].s64 = ctx.r[11].s64 + 4304;
	// 8263C3EC: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C3F0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C3F4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C3F8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C3FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C400: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C404: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C408: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C40C: 4BBB3E35  bl 0x821f0240
	ctx.lr = 0x8263C410;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C410: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8263C414: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C418: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C41C: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C420: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C424: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C428: 48017B09  bl 0x82653f30
	ctx.lr = 0x8263C42C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C42C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C430: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C438: 409A000C  bne cr6, 0x8263c444
	if !ctx.cr[6].eq {
	pc = 0x8263C444; continue 'dispatch;
	}
	// 8263C43C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C440: 48000010  b 0x8263c450
	pc = 0x8263C450; continue 'dispatch;
	// 8263C444: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C448: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C44C: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C450: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C454: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C458: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C45C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C460: 4098002C  bge cr6, 0x8263c48c
	if !ctx.cr[6].lt {
	pc = 0x8263C48C; continue 'dispatch;
	}
	// 8263C464: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C468: 419A0018  beq cr6, 0x8263c480
	if ctx.cr[6].eq {
	pc = 0x8263C480; continue 'dispatch;
	}
	// 8263C46C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C470: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C474: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C478: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C47C: 48017AB5  bl 0x82653f30
	ctx.lr = 0x8263C480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C480: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C484: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C488: 48000020  b 0x8263c4a8
	pc = 0x8263C4A8; continue 'dispatch;
	// 8263C48C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C490: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C494: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C498: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C4A0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C4A4: 48017BBD  bl 0x82654060
	ctx.lr = 0x8263C4A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C4A8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C4AC: 4BBD892D  bl 0x82214dd8
	ctx.lr = 0x8263C4B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C4B0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C4B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C4B8: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C4BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C4C0: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C4C4: 4BBB3D7D  bl 0x821f0240
	ctx.lr = 0x8263C4C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C4C8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C4CC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C4D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C4D4: 4BF2F38D  bl 0x8256b860
	ctx.lr = 0x8263C4D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C4D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C4DC: 4BBD88FD  bl 0x82214dd8
	ctx.lr = 0x8263C4E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C4E0: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C4E4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C4E8: 4BBD88F1  bl 0x82214dd8
	ctx.lr = 0x8263C4EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C4EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C4F0: 4BBD88E9  bl 0x82214dd8
	ctx.lr = 0x8263C4F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C4F4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263C4F8: 419A000C  beq cr6, 0x8263c504
	if ctx.cr[6].eq {
	pc = 0x8263C504; continue 'dispatch;
	}
	// 8263C4FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263C500: 4BF86019  bl 0x825c2518
	ctx.lr = 0x8263C504;
	crate::recompiler::externs::call(&mut ctx, base, 0x825C2518);
	// 8263C504: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C508: 4866CF48  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

pub fn sub_8263C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C510 size=368
	// 8263C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C514: 4866CEF1  bl 0x82ca9404
	ctx.lr = 0x8263C518;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263C518: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C51C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C520: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C528: 419A0150  beq cr6, 0x8263c678
	if ctx.cr[6].eq {
	pc = 0x8263C678; continue 'dispatch;
	}
	// 8263C52C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263C530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C534: 388B93F0  addi r4, r11, -0x6c10
	ctx.r[4].s64 = ctx.r[11].s64 + -27664;
	// 8263C538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C53C: 4BBF0995  bl 0x8222ced0
	ctx.lr = 0x8263C540;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C540: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C548: 409A0010  bne cr6, 0x8263c558
	if !ctx.cr[6].eq {
	pc = 0x8263C558; continue 'dispatch;
	}
	// 8263C54C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C550: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C554: 48000008  b 0x8263c55c
	pc = 0x8263C55C; continue 'dispatch;
	// 8263C558: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C55C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C560: 4BFF3D61  bl 0x826302c0
	ctx.lr = 0x8263C564;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C564: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C568: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C56C: 392B1208  addi r9, r11, 0x1208
	ctx.r[9].s64 = ctx.r[11].s64 + 4616;
	// 8263C570: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C574: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C578: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C57C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C580: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C584: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C58C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C590: 4BBB3CB1  bl 0x821f0240
	ctx.lr = 0x8263C594;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C594: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263C598: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C59C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C5A0: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C5A4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C5A8: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C5AC: 48017985  bl 0x82653f30
	ctx.lr = 0x8263C5B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C5B0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C5B4: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C5B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C5BC: 409A000C  bne cr6, 0x8263c5c8
	if !ctx.cr[6].eq {
	pc = 0x8263C5C8; continue 'dispatch;
	}
	// 8263C5C0: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C5C4: 48000010  b 0x8263c5d4
	pc = 0x8263C5D4; continue 'dispatch;
	// 8263C5C8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C5CC: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C5D0: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C5D4: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C5D8: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C5DC: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C5E0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C5E4: 4098002C  bge cr6, 0x8263c610
	if !ctx.cr[6].lt {
	pc = 0x8263C610; continue 'dispatch;
	}
	// 8263C5E8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C5EC: 419A0018  beq cr6, 0x8263c604
	if ctx.cr[6].eq {
	pc = 0x8263C604; continue 'dispatch;
	}
	// 8263C5F0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C5F4: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C5F8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C5FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C600: 48017931  bl 0x82653f30
	ctx.lr = 0x8263C604;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C604: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C608: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C60C: 48000020  b 0x8263c62c
	pc = 0x8263C62C; continue 'dispatch;
	// 8263C610: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C614: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C618: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C61C: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C624: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C628: 48017A39  bl 0x82654060
	ctx.lr = 0x8263C62C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C62C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C630: 4BBD87A9  bl 0x82214dd8
	ctx.lr = 0x8263C634;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C634: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C638: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C63C: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C640: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C644: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C648: 4BBB3BF9  bl 0x821f0240
	ctx.lr = 0x8263C64C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C64C: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C650: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C658: 4BF2F209  bl 0x8256b860
	ctx.lr = 0x8263C65C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C65C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C660: 4BBD8779  bl 0x82214dd8
	ctx.lr = 0x8263C664;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C664: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C668: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C66C: 4BBD876D  bl 0x82214dd8
	ctx.lr = 0x8263C670;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C674: 4BBD8765  bl 0x82214dd8
	ctx.lr = 0x8263C678;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C678: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C67C: 4866CDD8  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C680 size=368
	// 8263C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C684: 4866CD81  bl 0x82ca9404
	ctx.lr = 0x8263C688;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263C688: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C68C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C690: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C698: 419A0150  beq cr6, 0x8263c7e8
	if ctx.cr[6].eq {
	pc = 0x8263C7E8; continue 'dispatch;
	}
	// 8263C69C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263C6A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C6A4: 388B9DF0  addi r4, r11, -0x6210
	ctx.r[4].s64 = ctx.r[11].s64 + -25104;
	// 8263C6A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C6AC: 4BBF0825  bl 0x8222ced0
	ctx.lr = 0x8263C6B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C6B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C6B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C6B8: 409A0010  bne cr6, 0x8263c6c8
	if !ctx.cr[6].eq {
	pc = 0x8263C6C8; continue 'dispatch;
	}
	// 8263C6BC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C6C0: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C6C4: 48000008  b 0x8263c6cc
	pc = 0x8263C6CC; continue 'dispatch;
	// 8263C6C8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C6CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C6D0: 4BFF3BF1  bl 0x826302c0
	ctx.lr = 0x8263C6D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C6D4: 3D608264  lis r11, -0x7d9c
	ctx.r[11].s64 = -2107375616;
	// 8263C6D8: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C6DC: 392B8A88  addi r9, r11, -0x7578
	ctx.r[9].s64 = ctx.r[11].s64 + -30072;
	// 8263C6E0: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C6E4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C6E8: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C6EC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C6F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C6F4: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C6F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C6FC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C700: 4BBB3B41  bl 0x821f0240
	ctx.lr = 0x8263C704;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C704: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8263C708: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C70C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C710: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C714: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C718: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C71C: 48017815  bl 0x82653f30
	ctx.lr = 0x8263C720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C720: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C724: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C72C: 409A000C  bne cr6, 0x8263c738
	if !ctx.cr[6].eq {
	pc = 0x8263C738; continue 'dispatch;
	}
	// 8263C730: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C734: 48000010  b 0x8263c744
	pc = 0x8263C744; continue 'dispatch;
	// 8263C738: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C73C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C740: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C744: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C748: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C74C: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C750: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C754: 4098002C  bge cr6, 0x8263c780
	if !ctx.cr[6].lt {
	pc = 0x8263C780; continue 'dispatch;
	}
	// 8263C758: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C75C: 419A0018  beq cr6, 0x8263c774
	if ctx.cr[6].eq {
	pc = 0x8263C774; continue 'dispatch;
	}
	// 8263C760: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C764: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C768: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C76C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C770: 480177C1  bl 0x82653f30
	ctx.lr = 0x8263C774;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C774: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C778: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C77C: 48000020  b 0x8263c79c
	pc = 0x8263C79C; continue 'dispatch;
	// 8263C780: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C784: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C788: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C78C: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C794: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C798: 480178C9  bl 0x82654060
	ctx.lr = 0x8263C79C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C79C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C7A0: 4BBD8639  bl 0x82214dd8
	ctx.lr = 0x8263C7A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C7A4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C7A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C7AC: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C7B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C7B4: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C7B8: 4BBB3A89  bl 0x821f0240
	ctx.lr = 0x8263C7BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C7BC: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C7C0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C7C8: 4BF2F099  bl 0x8256b860
	ctx.lr = 0x8263C7CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C7CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C7D0: 4BBD8609  bl 0x82214dd8
	ctx.lr = 0x8263C7D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C7D4: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C7D8: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C7DC: 4BBD85FD  bl 0x82214dd8
	ctx.lr = 0x8263C7E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C7E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C7E4: 4BBD85F5  bl 0x82214dd8
	ctx.lr = 0x8263C7E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C7E8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C7EC: 4866CC68  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263C7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C7F0 size=368
	// 8263C7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C7F4: 4866CC11  bl 0x82ca9404
	ctx.lr = 0x8263C7F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 8263C7F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C7FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C800: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C808: 419A0150  beq cr6, 0x8263c958
	if ctx.cr[6].eq {
	pc = 0x8263C958; continue 'dispatch;
	}
	// 8263C80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8263C810: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C814: 388B4CB4  addi r4, r11, 0x4cb4
	ctx.r[4].s64 = ctx.r[11].s64 + 19636;
	// 8263C818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C81C: 4BBF06B5  bl 0x8222ced0
	ctx.lr = 0x8263C820;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C820: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C828: 409A0010  bne cr6, 0x8263c838
	if !ctx.cr[6].eq {
	pc = 0x8263C838; continue 'dispatch;
	}
	// 8263C82C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C830: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C834: 48000008  b 0x8263c83c
	pc = 0x8263C83C; continue 'dispatch;
	// 8263C838: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C83C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C840: 4BFF3A81  bl 0x826302c0
	ctx.lr = 0x8263C844;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C844: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C848: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C84C: 392B1298  addi r9, r11, 0x1298
	ctx.r[9].s64 = ctx.r[11].s64 + 4760;
	// 8263C850: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C854: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8263C858: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C85C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C860: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C864: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C868: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C86C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C870: 4BBB39D1  bl 0x821f0240
	ctx.lr = 0x8263C874;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C874: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8263C878: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C87C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C880: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C884: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C888: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263C88C: 480176A5  bl 0x82653f30
	ctx.lr = 0x8263C890;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C890: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C894: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263C898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C89C: 409A000C  bne cr6, 0x8263c8a8
	if !ctx.cr[6].eq {
	pc = 0x8263C8A8; continue 'dispatch;
	}
	// 8263C8A0: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8263C8A4: 48000010  b 0x8263c8b4
	pc = 0x8263C8B4; continue 'dispatch;
	// 8263C8A8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263C8AC: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263C8B0: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263C8B4: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263C8B8: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263C8BC: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263C8C0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263C8C4: 4098002C  bge cr6, 0x8263c8f0
	if !ctx.cr[6].lt {
	pc = 0x8263C8F0; continue 'dispatch;
	}
	// 8263C8C8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263C8CC: 419A0018  beq cr6, 0x8263c8e4
	if ctx.cr[6].eq {
	pc = 0x8263C8E4; continue 'dispatch;
	}
	// 8263C8D0: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263C8D4: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263C8D8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263C8DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263C8E0: 48017651  bl 0x82653f30
	ctx.lr = 0x8263C8E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263C8E4: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263C8E8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C8EC: 48000020  b 0x8263c90c
	pc = 0x8263C90C; continue 'dispatch;
	// 8263C8F0: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263C8F4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263C8F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263C8FC: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8263C900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C904: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263C908: 48017759  bl 0x82654060
	ctx.lr = 0x8263C90C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263C90C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263C910: 4BBD84C9  bl 0x82214dd8
	ctx.lr = 0x8263C914;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C914: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8263C918: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C91C: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 8263C920: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C924: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263C928: 4BBB3919  bl 0x821f0240
	ctx.lr = 0x8263C92C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C92C: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263C930: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263C934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263C938: 4BF2EF29  bl 0x8256b860
	ctx.lr = 0x8263C93C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263C93C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263C940: 4BBD8499  bl 0x82214dd8
	ctx.lr = 0x8263C944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C944: 9BDD0029  stb r30, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[30].u8 ) };
	// 8263C948: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C94C: 4BBD848D  bl 0x82214dd8
	ctx.lr = 0x8263C950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C954: 4BBD8485  bl 0x82214dd8
	ctx.lr = 0x8263C958;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263C958: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263C95C: 4866CAF8  b 0x82ca9454
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9454);
	return;
}

pub fn sub_8263C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C960 size=392
	// 8263C960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C964: 4866CA9D  bl 0x82ca9400
	ctx.lr = 0x8263C968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 8263C968: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C96C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8263C970: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8263C974: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8263C978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C97C: 419A0150  beq cr6, 0x8263cacc
	if ctx.cr[6].eq {
	pc = 0x8263CACC; continue 'dispatch;
	}
	// 8263C980: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263C984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8263C988: 388B940C  addi r4, r11, -0x6bf4
	ctx.r[4].s64 = ctx.r[11].s64 + -27636;
	// 8263C98C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263C990: 4BBF0541  bl 0x8222ced0
	ctx.lr = 0x8263C994;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8263C994: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8263C998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263C99C: 409A0010  bne cr6, 0x8263c9ac
	if !ctx.cr[6].eq {
	pc = 0x8263C9AC; continue 'dispatch;
	}
	// 8263C9A0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8263C9A4: 388BFFDF  addi r4, r11, -0x21
	ctx.r[4].s64 = ctx.r[11].s64 + -33;
	// 8263C9A8: 48000008  b 0x8263c9b0
	pc = 0x8263C9B0; continue 'dispatch;
	// 8263C9AC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263C9B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8263C9B4: 4BFF390D  bl 0x826302c0
	ctx.lr = 0x8263C9B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x826302C0);
	// 8263C9B8: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 8263C9BC: 3D4082B9  lis r10, -0x7d47
	ctx.r[10].s64 = -2101805056;
	// 8263C9C0: 392B1338  addi r9, r11, 0x1338
	ctx.r[9].s64 = ctx.r[11].s64 + 4920;
	// 8263C9C4: 390A6E78  addi r8, r10, 0x6e78
	ctx.r[8].s64 = ctx.r[10].s64 + 28280;
	// 8263C9C8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8263C9CC: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8263C9D0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8263C9D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8263C9D8: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 8263C9DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263C9E0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263C9E4: 4BBB385D  bl 0x821f0240
	ctx.lr = 0x8263C9E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263C9E8: 38E00043  li r7, 0x43
	ctx.r[7].s64 = 67;
	// 8263C9EC: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8263C9F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8263C9F4: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 8263C9F8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 8263C9FC: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 8263CA00: 48017531  bl 0x82653f30
	ctx.lr = 0x8263CA04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263CA04: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263CA08: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8263CA0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263CA10: 409A000C  bne cr6, 0x8263ca1c
	if !ctx.cr[6].eq {
	pc = 0x8263CA1C; continue 'dispatch;
	}
	// 8263CA14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CA18: 48000010  b 0x8263ca28
	pc = 0x8263CA28; continue 'dispatch;
	// 8263CA1C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263CA20: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8263CA24: 7D484BD6  divw r10, r8, r9
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 8263CA28: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263CA2C: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8263CA30: 7D2B4BD6  divw r9, r11, r9
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8263CA34: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8263CA38: 4098002C  bge cr6, 0x8263ca64
	if !ctx.cr[6].lt {
	pc = 0x8263CA64; continue 'dispatch;
	}
	// 8263CA3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8263CA40: 419A0018  beq cr6, 0x8263ca58
	if ctx.cr[6].eq {
	pc = 0x8263CA58; continue 'dispatch;
	}
	// 8263CA44: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8263CA48: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 8263CA4C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8263CA50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8263CA54: 480174DD  bl 0x82653f30
	ctx.lr = 0x8263CA58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82653F30);
	// 8263CA58: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8263CA5C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263CA60: 48000024  b 0x8263ca84
	pc = 0x8263CA84; continue 'dispatch;
	// 8263CA64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CA68: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8263CA6C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8263CA70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263CA74: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8263CA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263CA7C: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8263CA80: 480175E1  bl 0x82654060
	ctx.lr = 0x8263CA84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82654060);
	// 8263CA84: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8263CA88: 4BBD8351  bl 0x82214dd8
	ctx.lr = 0x8263CA8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263CA8C: 9B7F0011  stb r27, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[27].u8 ) };
	// 8263CA90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8263CA94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263CA98: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 8263CA9C: 4BBB37A5  bl 0x821f0240
	ctx.lr = 0x8263CAA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F0240);
	// 8263CAA0: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8263CAA4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8263CAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263CAAC: 4BF2EDB5  bl 0x8256b860
	ctx.lr = 0x8263CAB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8256B860);
	// 8263CAB0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8263CAB4: 4BBD8325  bl 0x82214dd8
	ctx.lr = 0x8263CAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263CAB8: 9B7D0029  stb r27, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[27].u8 ) };
	// 8263CABC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8263CAC0: 4BBD8319  bl 0x82214dd8
	ctx.lr = 0x8263CAC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263CAC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8263CAC8: 4BBD8311  bl 0x82214dd8
	ctx.lr = 0x8263CACC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 8263CACC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8263CAD0: 419A000C  beq cr6, 0x8263cadc
	if ctx.cr[6].eq {
	pc = 0x8263CADC; continue 'dispatch;
	}
	// 8263CAD4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8263CAD8: 481F2731  bl 0x8282f208
	ctx.lr = 0x8263CADC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8282F208);
	// 8263CADC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8263CAE0: 4866C970  b 0x82ca9450
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9450);
	return;
}

