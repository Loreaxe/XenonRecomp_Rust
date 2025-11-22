pub fn sub_832A14F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A14F8 size=120
	// 832A14F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A14FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1504: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A150C: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 832A1510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1518: 388A29B0  addi r4, r10, 0x29b0
	ctx.r[4].s64 = ctx.r[10].s64 + 10672;
	// 832A151C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A152C: 386AA914  addi r3, r10, -0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22252;
	// 832A1530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A153C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A1544: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1548: 4BAB46F9  bl 0x82d55c40
	ctx.lr = 0x832A154C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1558: 4E800020  blr
	return;
}

pub fn sub_832A1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1570 size=112
	// 832A1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A157C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1584: 38EB4890  addi r7, r11, 0x4890
	ctx.r[7].s64 = ctx.r[11].s64 + 18576;
	// 832A1588: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 832A158C: 388A4908  addi r4, r10, 0x4908
	ctx.r[4].s64 = ctx.r[10].s64 + 18696;
	// 832A1590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1594: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A159C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A15A0: 386AAD68  addi r3, r10, -0x5298
	ctx.r[3].s64 = ctx.r[10].s64 + -21144;
	// 832A15A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A15A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A15AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A15B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A15B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A15B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A15BC: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 832A15C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A15C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A15C8: 4BAB4679  bl 0x82d55c40
	ctx.lr = 0x832A15CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A15CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A15D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A15D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A15D8: 4E800020  blr
	return;
}

pub fn sub_832A15E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A15E0 size=112
	// 832A15E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A15E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A15E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A15EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A15F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A15F4: 38EB491C  addi r7, r11, 0x491c
	ctx.r[7].s64 = ctx.r[11].s64 + 18716;
	// 832A15F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A15FC: 388A4934  addi r4, r10, 0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + 18740;
	// 832A1600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1604: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A160C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1610: 386AAD98  addi r3, r10, -0x5268
	ctx.r[3].s64 = ctx.r[10].s64 + -21096;
	// 832A1614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A161C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A162C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A1630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1638: 4BAB4609  bl 0x82d55c40
	ctx.lr = 0x832A163C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A163C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1648: 4E800020  blr
	return;
}

pub fn sub_832A1650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1650 size=112
	// 832A1650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A165C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1660: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1664: 38AA9F54  addi r5, r10, -0x60ac
	ctx.r[5].s64 = ctx.r[10].s64 + -24748;
	// 832A1668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A166C: 390B495C  addi r8, r11, 0x495c
	ctx.r[8].s64 = ctx.r[11].s64 + 18780;
	// 832A1670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1674: 388A4974  addi r4, r10, 0x4974
	ctx.r[4].s64 = ctx.r[10].s64 + 18804;
	// 832A1678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A167C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1688: 386AADC8  addi r3, r10, -0x5238
	ctx.r[3].s64 = ctx.r[10].s64 + -21048;
	// 832A168C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A169C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A16A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A16A4: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 832A16A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A16AC: 4BAB4595  bl 0x82d55c40
	ctx.lr = 0x832A16B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A16B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A16B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A16B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A16BC: 4E800020  blr
	return;
}

pub fn sub_832A16C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A16C0 size=112
	// 832A16C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A16C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A16C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A16CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A16D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A16D4: 38EB49AC  addi r7, r11, 0x49ac
	ctx.r[7].s64 = ctx.r[11].s64 + 18860;
	// 832A16D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A16DC: 388A4A34  addi r4, r10, 0x4a34
	ctx.r[4].s64 = ctx.r[10].s64 + 18996;
	// 832A16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A16E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A16E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A16EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A16F0: 386AADF8  addi r3, r10, -0x5208
	ctx.r[3].s64 = ctx.r[10].s64 + -21000;
	// 832A16F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A16F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A16FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A170C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A1710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1718: 4BAB4529  bl 0x82d55c40
	ctx.lr = 0x832A171C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A171C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1728: 4E800020  blr
	return;
}

pub fn sub_832A1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1730 size=112
	// 832A1730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A173C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1740: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1744: 38EB49DC  addi r7, r11, 0x49dc
	ctx.r[7].s64 = ctx.r[11].s64 + 18908;
	// 832A1748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A174C: 388A4A54  addi r4, r10, 0x4a54
	ctx.r[4].s64 = ctx.r[10].s64 + 19028;
	// 832A1750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1754: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1760: 386AAE28  addi r3, r10, -0x51d8
	ctx.r[3].s64 = ctx.r[10].s64 + -20952;
	// 832A1764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A176C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A177C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1788: 4BAB44B9  bl 0x82d55c40
	ctx.lr = 0x832A178C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A178C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1798: 4E800020  blr
	return;
}

pub fn sub_832A17A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A17A0 size=112
	// 832A17A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A17A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A17A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A17AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A17B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A17B4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A17B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A17BC: 390B49F4  addi r8, r11, 0x49f4
	ctx.r[8].s64 = ctx.r[11].s64 + 18932;
	// 832A17C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A17C4: 388A4A68  addi r4, r10, 0x4a68
	ctx.r[4].s64 = ctx.r[10].s64 + 19048;
	// 832A17C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A17CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A17D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A17D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A17D8: 386AAE58  addi r3, r10, -0x51a8
	ctx.r[3].s64 = ctx.r[10].s64 + -20904;
	// 832A17DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A17E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A17E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A17E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A17EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A17F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A17F4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A17F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A17FC: 4BAB4445  bl 0x82d55c40
	ctx.lr = 0x832A1800;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A180C: 4E800020  blr
	return;
}

pub fn sub_832A1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1810 size=176
	// 832A1810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A181C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1820: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1824: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A182C: 390B4AA8  addi r8, r11, 0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 19112;
	// 832A1830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A1834: 388A4B00  addi r4, r10, 0x4b00
	ctx.r[4].s64 = ctx.r[10].s64 + 19200;
	// 832A1838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A183C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1848: 386AAE88  addi r3, r10, -0x5178
	ctx.r[3].s64 = ctx.r[10].s64 + -20856;
	// 832A184C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A185C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1864: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 832A1868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A186C: 4BAB43D5  bl 0x82d55c40
	ctx.lr = 0x832A1870;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A187C: 4E800020  blr
	return;
	// 832A1880: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A1884: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A1888: 396BC778  addi r11, r11, -0x3888
	ctx.r[11].s64 = ctx.r[11].s64 + -14472;
	// 832A188C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A1890: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A1894: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A1898: 4E800020  blr
	return;
}

pub fn sub_832A18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A18C0 size=112
	// 832A18C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A18C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A18C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A18CC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A18D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A18D4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A18D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A18DC: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 832A18E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A18E4: 388A5D2C  addi r4, r10, 0x5d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 23852;
	// 832A18E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A18EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A18F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A18F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A18F8: 386AB078  addi r3, r10, -0x4f88
	ctx.r[3].s64 = ctx.r[10].s64 + -20360;
	// 832A18FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A190C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1914: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A1918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A191C: 4BAB4325  bl 0x82d55c40
	ctx.lr = 0x832A1920;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A192C: 4E800020  blr
	return;
}

pub fn sub_832A1930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1930 size=112
	// 832A1930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A193C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1940: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1944: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A194C: 390B5E4C  addi r8, r11, 0x5e4c
	ctx.r[8].s64 = ctx.r[11].s64 + 24140;
	// 832A1950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1954: 388A5EC4  addi r4, r10, 0x5ec4
	ctx.r[4].s64 = ctx.r[10].s64 + 24260;
	// 832A1958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A195C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1968: 386AB0A8  addi r3, r10, -0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + -20312;
	// 832A196C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A197C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1984: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A198C: 4BAB42B5  bl 0x82d55c40
	ctx.lr = 0x832A1990;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A199C: 4E800020  blr
	return;
}

pub fn sub_832A19A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A19A0 size=112
	// 832A19A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A19A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A19A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A19AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A19B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A19B4: 38EB5F40  addi r7, r11, 0x5f40
	ctx.r[7].s64 = ctx.r[11].s64 + 24384;
	// 832A19B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A19BC: 388A6010  addi r4, r10, 0x6010
	ctx.r[4].s64 = ctx.r[10].s64 + 24592;
	// 832A19C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A19C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A19C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A19CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A19D0: 386AB138  addi r3, r10, -0x4ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -20168;
	// 832A19D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A19D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A19DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A19E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A19E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A19E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A19EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A19F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A19F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A19F8: 4BAB4249  bl 0x82d55c40
	ctx.lr = 0x832A19FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A19FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1A08: 4E800020  blr
	return;
}

pub fn sub_832A1A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1A10 size=112
	// 832A1A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1A1C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1A20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1A24: 38EB5F70  addi r7, r11, 0x5f70
	ctx.r[7].s64 = ctx.r[11].s64 + 24432;
	// 832A1A28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 832A1A2C: 388A602C  addi r4, r10, 0x602c
	ctx.r[4].s64 = ctx.r[10].s64 + 24620;
	// 832A1A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1A34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1A40: 386AB108  addi r3, r10, -0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -20216;
	// 832A1A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1A5C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A1A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1A68: 4BAB41D9  bl 0x82d55c40
	ctx.lr = 0x832A1A6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1A78: 4E800020  blr
	return;
}

pub fn sub_832A1A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1A80 size=112
	// 832A1A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1A8C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1A90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1A94: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1A98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1A9C: 390B5FB8  addi r8, r11, 0x5fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24504;
	// 832A1AA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A1AA4: 388A604C  addi r4, r10, 0x604c
	ctx.r[4].s64 = ctx.r[10].s64 + 24652;
	// 832A1AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1AAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1AB8: 386AB0D8  addi r3, r10, -0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + -20264;
	// 832A1ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1AD4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 832A1AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1ADC: 4BAB4165  bl 0x82d55c40
	ctx.lr = 0x832A1AE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1AEC: 4E800020  blr
	return;
}

pub fn sub_832A1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1AF0 size=112
	// 832A1AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1AFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1B04: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1B08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1B0C: 390B608C  addi r8, r11, 0x608c
	ctx.r[8].s64 = ctx.r[11].s64 + 24716;
	// 832A1B10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A1B14: 388A6104  addi r4, r10, 0x6104
	ctx.r[4].s64 = ctx.r[10].s64 + 24836;
	// 832A1B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1B1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1B28: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 832A1B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1B44: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 832A1B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1B4C: 4BAB40F5  bl 0x82d55c40
	ctx.lr = 0x832A1B50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1B5C: 4E800020  blr
	return;
}

pub fn sub_832A1B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1B60 size=112
	// 832A1B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1B6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1B74: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A1B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1B7C: 390B6130  addi r8, r11, 0x6130
	ctx.r[8].s64 = ctx.r[11].s64 + 24880;
	// 832A1B80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A1B84: 388A61B4  addi r4, r10, 0x61b4
	ctx.r[4].s64 = ctx.r[10].s64 + 25012;
	// 832A1B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1B8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1B98: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 832A1B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1BB4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1BBC: 4BAB4085  bl 0x82d55c40
	ctx.lr = 0x832A1BC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1BCC: 4E800020  blr
	return;
}

pub fn sub_832A1BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1BD0 size=112
	// 832A1BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1BDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1BE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1BE4: 38EB6238  addi r7, r11, 0x6238
	ctx.r[7].s64 = ctx.r[11].s64 + 25144;
	// 832A1BE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 832A1BEC: 388A62E0  addi r4, r10, 0x62e0
	ctx.r[4].s64 = ctx.r[10].s64 + 25312;
	// 832A1BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1BF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1C00: 386AB1C8  addi r3, r10, -0x4e38
	ctx.r[3].s64 = ctx.r[10].s64 + -20024;
	// 832A1C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1C1C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A1C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1C28: 4BAB4019  bl 0x82d55c40
	ctx.lr = 0x832A1C2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1C38: 4E800020  blr
	return;
}

pub fn sub_832A1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1C40 size=112
	// 832A1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1C4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1C54: 38EB6280  addi r7, r11, 0x6280
	ctx.r[7].s64 = ctx.r[11].s64 + 25216;
	// 832A1C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1C5C: 388A62FC  addi r4, r10, 0x62fc
	ctx.r[4].s64 = ctx.r[10].s64 + 25340;
	// 832A1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1C64: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1C70: 386AB1F8  addi r3, r10, -0x4e08
	ctx.r[3].s64 = ctx.r[10].s64 + -19976;
	// 832A1C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1C8C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1C98: 4BAB3FA9  bl 0x82d55c40
	ctx.lr = 0x832A1C9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1CA8: 4E800020  blr
	return;
}

pub fn sub_832A1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1CB0 size=112
	// 832A1CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1CBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1CC4: 38EB62B0  addi r7, r11, 0x62b0
	ctx.r[7].s64 = ctx.r[11].s64 + 25264;
	// 832A1CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1CCC: 388A631C  addi r4, r10, 0x631c
	ctx.r[4].s64 = ctx.r[10].s64 + 25372;
	// 832A1CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1CD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1CE0: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 832A1CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1CFC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A1D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1D08: 4BAB3F39  bl 0x82d55c40
	ctx.lr = 0x832A1D0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1D18: 4E800020  blr
	return;
}

pub fn sub_832A1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1D20 size=104
	// 832A1D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1D28: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A1D34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A1D38: 4BAE9051  bl 0x82d8ad88
	ctx.lr = 0x832A1D3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D8AD88);
	// 832A1D3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D40: 3CE082E5  lis r7, -0x7d1b
	ctx.r[7].s64 = -2098921472;
	// 832A1D44: 394B648C  addi r10, r11, 0x648c
	ctx.r[10].s64 = ctx.r[11].s64 + 25740;
	// 832A1D48: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D4C: 3D0082E5  lis r8, -0x7d1b
	ctx.r[8].s64 = -2098921472;
	// 832A1D50: 392B646C  addi r9, r11, 0x646c
	ctx.r[9].s64 = ctx.r[11].s64 + 25708;
	// 832A1D54: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A1D58: 396BB288  addi r11, r11, -0x4d78
	ctx.r[11].s64 = ctx.r[11].s64 + -19832;
	// 832A1D5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A1D60: 3947A5E0  addi r10, r7, -0x5a20
	ctx.r[10].s64 = ctx.r[7].s64 + -23072;
	// 832A1D64: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A1D68: 3948A5C8  addi r10, r8, -0x5a38
	ctx.r[10].s64 = ctx.r[8].s64 + -23096;
	// 832A1D6C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A1D70: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A1D74: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 832A1D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1D80: 4E800020  blr
	return;
}

pub fn sub_832A1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1D88 size=112
	// 832A1D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1D94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1D98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1D9C: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A1DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1DA4: 390B63A8  addi r8, r11, 0x63a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25512;
	// 832A1DA8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832A1DAC: 388A648C  addi r4, r10, 0x648c
	ctx.r[4].s64 = ctx.r[10].s64 + 25740;
	// 832A1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1DB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1DC0: 386AB258  addi r3, r10, -0x4da8
	ctx.r[3].s64 = ctx.r[10].s64 + -19880;
	// 832A1DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1DDC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1DE4: 4BAB3E5D  bl 0x82d55c40
	ctx.lr = 0x832A1DE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1DF4: 4E800020  blr
	return;
}

pub fn sub_832A1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1DF8 size=112
	// 832A1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1E04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1E0C: 38EB64B4  addi r7, r11, 0x64b4
	ctx.r[7].s64 = ctx.r[11].s64 + 25780;
	// 832A1E10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1E14: 388A650C  addi r4, r10, 0x650c
	ctx.r[4].s64 = ctx.r[10].s64 + 25868;
	// 832A1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1E1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1E28: 386AB2C8  addi r3, r10, -0x4d38
	ctx.r[3].s64 = ctx.r[10].s64 + -19768;
	// 832A1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1E44: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1E50: 4BAB3DF1  bl 0x82d55c40
	ctx.lr = 0x832A1E54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1E60: 4E800020  blr
	return;
}

pub fn sub_832A1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1E68 size=112
	// 832A1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1E74: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A1E78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1E7C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A1E80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1E84: 390B64E4  addi r8, r11, 0x64e4
	ctx.r[8].s64 = ctx.r[11].s64 + 25828;
	// 832A1E88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1E8C: 388A6544  addi r4, r10, 0x6544
	ctx.r[4].s64 = ctx.r[10].s64 + 25924;
	// 832A1E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1E94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1EA0: 386AB298  addi r3, r10, -0x4d68
	ctx.r[3].s64 = ctx.r[10].s64 + -19816;
	// 832A1EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1EBC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A1EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1EC4: 4BAB3D7D  bl 0x82d55c40
	ctx.lr = 0x832A1EC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1ED4: 4E800020  blr
	return;
}

pub fn sub_832A1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1ED8 size=112
	// 832A1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1EE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1EE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1EEC: 38EB6578  addi r7, r11, 0x6578
	ctx.r[7].s64 = ctx.r[11].s64 + 25976;
	// 832A1EF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1EF4: 388A6624  addi r4, r10, 0x6624
	ctx.r[4].s64 = ctx.r[10].s64 + 26148;
	// 832A1EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1EFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1F08: 386AB2F8  addi r3, r10, -0x4d08
	ctx.r[3].s64 = ctx.r[10].s64 + -19720;
	// 832A1F0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1F24: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A1F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1F2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A1F30: 4BAB3D11  bl 0x82d55c40
	ctx.lr = 0x832A1F34;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1F40: 4E800020  blr
	return;
}

pub fn sub_832A1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1F48 size=112
	// 832A1F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1F54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1F5C: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A1F60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1F64: 390B65A8  addi r8, r11, 0x65a8
	ctx.r[8].s64 = ctx.r[11].s64 + 26024;
	// 832A1F68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A1F6C: 388A664C  addi r4, r10, 0x664c
	ctx.r[4].s64 = ctx.r[10].s64 + 26188;
	// 832A1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1F74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A1F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A1F80: 386AB328  addi r3, r10, -0x4cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19672;
	// 832A1F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1F94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A1F9C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A1FA4: 4BAB3C9D  bl 0x82d55c40
	ctx.lr = 0x832A1FA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A1FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A1FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A1FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A1FB4: 4E800020  blr
	return;
}

pub fn sub_832A1FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A1FB8 size=112
	// 832A1FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A1FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A1FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A1FC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A1FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A1FCC: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 832A1FD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A1FD4: 388A686C  addi r4, r10, 0x686c
	ctx.r[4].s64 = ctx.r[10].s64 + 26732;
	// 832A1FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A1FDC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A1FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A1FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A1FE8: 386AB398  addi r3, r10, -0x4c68
	ctx.r[3].s64 = ctx.r[10].s64 + -19560;
	// 832A1FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A1FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A1FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A1FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A1FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A2004: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A2008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A200C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A2010: 4BAB3C31  bl 0x82d55c40
	ctx.lr = 0x832A2014;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A201C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2020: 4E800020  blr
	return;
}

pub fn sub_832A2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2028 size=112
	// 832A2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2038: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A203C: 38AAB398  addi r5, r10, -0x4c68
	ctx.r[5].s64 = ctx.r[10].s64 + -19560;
	// 832A2040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2044: 390B67D0  addi r8, r11, 0x67d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26576;
	// 832A2048: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A204C: 388A6888  addi r4, r10, 0x6888
	ctx.r[4].s64 = ctx.r[10].s64 + 26760;
	// 832A2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2054: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A205C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2060: 386AB358  addi r3, r10, -0x4ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -19624;
	// 832A2064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A206C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A207C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2084: 4BAB3BBD  bl 0x82d55c40
	ctx.lr = 0x832A2088;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A208C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2094: 4E800020  blr
	return;
}

pub fn sub_832A2098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2098 size=136
	// 832A2098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A209C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A20A0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A20A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A20A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A20AC: 396B6860  addi r11, r11, 0x6860
	ctx.r[11].s64 = ctx.r[11].s64 + 26720;
	// 832A20B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A20B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 832A20B8: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 832A20BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832A20C0: 4BAFC681  bl 0x82d9e740
	ctx.lr = 0x832A20C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D9E740);
	// 832A20C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A20C8: 3D0082E5  lis r8, -0x7d1b
	ctx.r[8].s64 = -2098921472;
	// 832A20CC: 394B68A4  addi r10, r11, 0x68a4
	ctx.r[10].s64 = ctx.r[11].s64 + 26788;
	// 832A20D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A20D4: 3D2082E5  lis r9, -0x7d1b
	ctx.r[9].s64 = -2098921472;
	// 832A20D8: 396BB388  addi r11, r11, -0x4c78
	ctx.r[11].s64 = ctx.r[11].s64 + -19576;
	// 832A20DC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A20E0: 3948A9A0  addi r10, r8, -0x5660
	ctx.r[10].s64 = ctx.r[8].s64 + -22112;
	// 832A20E4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A20E8: 3949A968  addi r10, r9, -0x5698
	ctx.r[10].s64 = ctx.r[9].s64 + -22168;
	// 832A20EC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A20F0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A20F4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A20F8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 832A20FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2104: 4E800020  blr
	return;
	// 832A2108: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A210C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A2110: 394ACD30  addi r10, r10, -0x32d0
	ctx.r[10].s64 = ctx.r[10].s64 + -13008;
	// 832A2114: 816BCD08  lwz r11, -0x32f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13048 as u32) ) } as u64;
	// 832A2118: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 832A211C: 4E800020  blr
	return;
}

pub fn sub_832A2120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2120 size=120
	// 832A2120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A212C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A2130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2134: 390BCD30  addi r8, r11, -0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13008;
	// 832A2138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A213C: 392A6848  addi r9, r10, 0x6848
	ctx.r[9].s64 = ctx.r[10].s64 + 26696;
	// 832A2140: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2144: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 832A2148: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A214C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2154: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A215C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2164: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A2168: 388A68A4  addi r4, r10, 0x68a4
	ctx.r[4].s64 = ctx.r[10].s64 + 26788;
	// 832A216C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A2170: 386BB3C8  addi r3, r11, -0x4c38
	ctx.r[3].s64 = ctx.r[11].s64 + -19512;
	// 832A2174: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A2178: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A217C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 832A2180: 4BAB3AC1  bl 0x82d55c40
	ctx.lr = 0x832A2184;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A218C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2190: 4E800020  blr
	return;
}

pub fn sub_832A2198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2198 size=112
	// 832A2198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A219C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A21A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A21A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A21A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A21AC: 38AA8A20  addi r5, r10, -0x75e0
	ctx.r[5].s64 = ctx.r[10].s64 + -30176;
	// 832A21B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A21B4: 390B6934  addi r8, r11, 0x6934
	ctx.r[8].s64 = ctx.r[11].s64 + 26932;
	// 832A21B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A21BC: 388A6964  addi r4, r10, 0x6964
	ctx.r[4].s64 = ctx.r[10].s64 + 26980;
	// 832A21C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A21C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A21C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A21CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A21D0: 386AB3F8  addi r3, r10, -0x4c08
	ctx.r[3].s64 = ctx.r[10].s64 + -19464;
	// 832A21D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A21D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A21DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A21E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A21E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A21E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A21EC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A21F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A21F4: 4BAB3A4D  bl 0x82d55c40
	ctx.lr = 0x832A21F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A21F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A21FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2204: 4E800020  blr
	return;
}

pub fn sub_832A2208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2208 size=112
	// 832A2208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A220C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2214: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2218: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A221C: 38AA8CC0  addi r5, r10, -0x7340
	ctx.r[5].s64 = ctx.r[10].s64 + -29504;
	// 832A2220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2224: 390B69A0  addi r8, r11, 0x69a0
	ctx.r[8].s64 = ctx.r[11].s64 + 27040;
	// 832A2228: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A222C: 388A6A24  addi r4, r10, 0x6a24
	ctx.r[4].s64 = ctx.r[10].s64 + 27172;
	// 832A2230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2234: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A223C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2240: 386AB428  addi r3, r10, -0x4bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19416;
	// 832A2244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A224C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A225C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A2260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2264: 4BAB39DD  bl 0x82d55c40
	ctx.lr = 0x832A2268;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A226C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2274: 4E800020  blr
	return;
}

pub fn sub_832A2278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2278 size=112
	// 832A2278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A227C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2284: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2288: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A228C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2294: 390B6A38  addi r8, r11, 0x6a38
	ctx.r[8].s64 = ctx.r[11].s64 + 27192;
	// 832A2298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A229C: 388A6A60  addi r4, r10, 0x6a60
	ctx.r[4].s64 = ctx.r[10].s64 + 27232;
	// 832A22A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A22A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A22A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A22AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A22B0: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 832A22B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A22B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A22BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A22C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A22C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A22C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A22CC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A22D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A22D4: 4BAB396D  bl 0x82d55c40
	ctx.lr = 0x832A22D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A22D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A22DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A22E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A22E4: 4E800020  blr
	return;
}

pub fn sub_832A22E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A22E8 size=112
	// 832A22E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A22EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A22F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A22F4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A22F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A22FC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2304: 390B6A84  addi r8, r11, 0x6a84
	ctx.r[8].s64 = ctx.r[11].s64 + 27268;
	// 832A2308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A230C: 388A6AAC  addi r4, r10, 0x6aac
	ctx.r[4].s64 = ctx.r[10].s64 + 27308;
	// 832A2310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2314: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A231C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2320: 386AB488  addi r3, r10, -0x4b78
	ctx.r[3].s64 = ctx.r[10].s64 + -19320;
	// 832A2324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A232C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A233C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A2340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2344: 4BAB38FD  bl 0x82d55c40
	ctx.lr = 0x832A2348;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A234C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2354: 4E800020  blr
	return;
}

pub fn sub_832A2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2358 size=112
	// 832A2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2364: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2368: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A236C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2374: 390B6B20  addi r8, r11, 0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + 27424;
	// 832A2378: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832A237C: 388A6C04  addi r4, r10, 0x6c04
	ctx.r[4].s64 = ctx.r[10].s64 + 27652;
	// 832A2380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2384: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A238C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2390: 386AB4B8  addi r3, r10, -0x4b48
	ctx.r[3].s64 = ctx.r[10].s64 + -19272;
	// 832A2394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A239C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A23A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A23A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A23A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A23AC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A23B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A23B4: 4BAB388D  bl 0x82d55c40
	ctx.lr = 0x832A23B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A23C4: 4E800020  blr
	return;
}

pub fn sub_832A23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A23C8 size=112
	// 832A23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A23D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A23D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A23DC: 396B6D60  addi r11, r11, 0x6d60
	ctx.r[11].s64 = ctx.r[11].s64 + 28000;
	// 832A23E0: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 832A23E4: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 832A23E8: 388A6F7C  addi r4, r10, 0x6f7c
	ctx.r[4].s64 = ctx.r[10].s64 + 28540;
	// 832A23EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A23F0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A23F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A23F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A23FC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 832A2400: 386AB4E8  addi r3, r10, -0x4b18
	ctx.r[3].s64 = ctx.r[10].s64 + -19224;
	// 832A2404: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832A2408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A240C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A2414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A2420: 4BAB3821  bl 0x82d55c40
	ctx.lr = 0x832A2424;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A2428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2430: 4E800020  blr
	return;
}

pub fn sub_832A2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2438 size=112
	// 832A2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2444: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A244C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2454: 390B6F98  addi r8, r11, 0x6f98
	ctx.r[8].s64 = ctx.r[11].s64 + 28568;
	// 832A2458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A245C: 388A7004  addi r4, r10, 0x7004
	ctx.r[4].s64 = ctx.r[10].s64 + 28676;
	// 832A2460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2464: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A246C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2470: 386AB518  addi r3, r10, -0x4ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -19176;
	// 832A2474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A247C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A248C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A2490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2494: 4BAB37AD  bl 0x82d55c40
	ctx.lr = 0x832A2498;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A249C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A24A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A24A4: 4E800020  blr
	return;
}

pub fn sub_832A24A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A24A8 size=112
	// 832A24A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A24AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A24B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A24B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A24B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A24BC: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A24C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A24C4: 390B7028  addi r8, r11, 0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + 28712;
	// 832A24C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A24CC: 388A70A8  addi r4, r10, 0x70a8
	ctx.r[4].s64 = ctx.r[10].s64 + 28840;
	// 832A24D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A24D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A24D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A24DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A24E0: 386AB548  addi r3, r10, -0x4ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -19128;
	// 832A24E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A24E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A24EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A24F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A24F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A24F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A24FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A2500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2504: 4BAB373D  bl 0x82d55c40
	ctx.lr = 0x832A2508;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A250C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2514: 4E800020  blr
	return;
}

pub fn sub_832A2518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2518 size=112
	// 832A2518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2524: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2528: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A252C: 38AA8540  addi r5, r10, -0x7ac0
	ctx.r[5].s64 = ctx.r[10].s64 + -31424;
	// 832A2530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A2534: 390B70D0  addi r8, r11, 0x70d0
	ctx.r[8].s64 = ctx.r[11].s64 + 28880;
	// 832A2538: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A253C: 388A7154  addi r4, r10, 0x7154
	ctx.r[4].s64 = ctx.r[10].s64 + 29012;
	// 832A2540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A2544: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A254C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2550: 386AB578  addi r3, r10, -0x4a88
	ctx.r[3].s64 = ctx.r[10].s64 + -19080;
	// 832A2554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A255C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A2564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A256C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 832A2570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A2574: 4BAB36CD  bl 0x82d55c40
	ctx.lr = 0x832A2578;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A2578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A257C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2584: 4E800020  blr
	return;
}

pub fn sub_832A2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2588 size=56
	// 832A2588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A258C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2594: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A2598: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 832A259C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A25A0: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 832A25A4: 4BE4B6DD  bl 0x830edc80
	ctx.lr = 0x832A25A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A25A8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A25AC: 906BB5C0  stw r3, -0x4a40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19008 as u32), ctx.r[3].u32 ) };
	// 832A25B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A25B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A25B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A25BC: 4E800020  blr
	return;
}

pub fn sub_832A25C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A25C0 size=56
	// 832A25C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A25C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A25C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A25CC: 38C00096  li r6, 0x96
	ctx.r[6].s64 = 150;
	// 832A25D0: 38A000C8  li r5, 0xc8
	ctx.r[5].s64 = 200;
	// 832A25D4: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 832A25D8: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 832A25DC: 4BE4B6A5  bl 0x830edc80
	ctx.lr = 0x832A25E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A25E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A25E4: 906BB5BC  stw r3, -0x4a44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19012 as u32), ctx.r[3].u32 ) };
	// 832A25E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A25EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A25F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A25F4: 4E800020  blr
	return;
}

pub fn sub_832A25F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A25F8 size=56
	// 832A25F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A25FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2604: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 832A2608: 38A000F0  li r5, 0xf0
	ctx.r[5].s64 = 240;
	// 832A260C: 388000F0  li r4, 0xf0
	ctx.r[4].s64 = 240;
	// 832A2610: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 832A2614: 4BE4B66D  bl 0x830edc80
	ctx.lr = 0x832A2618;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A2618: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A261C: 906BB5CC  stw r3, -0x4a34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18996 as u32), ctx.r[3].u32 ) };
	// 832A2620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A262C: 4E800020  blr
	return;
}

pub fn sub_832A2630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2630 size=56
	// 832A2630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A263C: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 832A2640: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 832A2644: 38800078  li r4, 0x78
	ctx.r[4].s64 = 120;
	// 832A2648: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 832A264C: 4BE4B635  bl 0x830edc80
	ctx.lr = 0x832A2650;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A2650: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A2654: 906BB5D0  stw r3, -0x4a30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18992 as u32), ctx.r[3].u32 ) };
	// 832A2658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2664: 4E800020  blr
	return;
}

pub fn sub_832A2668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2668 size=88
	// 832A2668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2674: 38C000C8  li r6, 0xc8
	ctx.r[6].s64 = 200;
	// 832A2678: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A267C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A2680: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 832A2684: 4BE4B5FD  bl 0x830edc80
	ctx.lr = 0x832A2688;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A2688: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A268C: 906BB5DC  stw r3, -0x4a24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18980 as u32), ctx.r[3].u32 ) };
	// 832A2690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A269C: 4E800020  blr
	return;
	// 832A26A0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A26A4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A26A8: 396BD42C  addi r11, r11, -0x2bd4
	ctx.r[11].s64 = ctx.r[11].s64 + -11220;
	// 832A26AC: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A26B0: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A26B4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A26B8: 4E800020  blr
	return;
}

pub fn sub_832A26C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A26C0 size=56
	// 832A26C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A26C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A26C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A26CC: 38C0008C  li r6, 0x8c
	ctx.r[6].s64 = 140;
	// 832A26D0: 38A0008C  li r5, 0x8c
	ctx.r[5].s64 = 140;
	// 832A26D4: 388000F0  li r4, 0xf0
	ctx.r[4].s64 = 240;
	// 832A26D8: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 832A26DC: 4BE4B5A5  bl 0x830edc80
	ctx.lr = 0x832A26E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A26E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A26E4: 906BB5F8  stw r3, -0x4a08(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18952 as u32), ctx.r[3].u32 ) };
	// 832A26E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A26EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A26F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A26F4: 4E800020  blr
	return;
}

pub fn sub_832A26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A26F8 size=120
	// 832A26F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A26FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2704: 38C0008C  li r6, 0x8c
	ctx.r[6].s64 = 140;
	// 832A2708: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 832A270C: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 832A2710: 38600046  li r3, 0x46
	ctx.r[3].s64 = 70;
	// 832A2714: 4BE4B56D  bl 0x830edc80
	ctx.lr = 0x832A2718;
	crate::recompiler::externs::call(&mut ctx, base, 0x830EDC80);
	// 832A2718: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A271C: 906BB5FC  stw r3, -0x4a04(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18948 as u32), ctx.r[3].u32 ) };
	// 832A2720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A272C: 4E800020  blr
	return;
	// 832A2730: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A2734: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A2738: 396BD69C  addi r11, r11, -0x2964
	ctx.r[11].s64 = ctx.r[11].s64 + -10596;
	// 832A273C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A2740: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A2744: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A2748: 4E800020  blr
	return;
}

pub fn sub_832A2770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2770 size=112
	// 832A2770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A277C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A2780: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2784: 396B84A0  addi r11, r11, -0x7b60
	ctx.r[11].s64 = ctx.r[11].s64 + -31584;
	// 832A2788: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 832A278C: 390B0120  addi r8, r11, 0x120
	ctx.r[8].s64 = ctx.r[11].s64 + 288;
	// 832A2790: 388A861C  addi r4, r10, -0x79e4
	ctx.r[4].s64 = ctx.r[10].s64 + -31204;
	// 832A2794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2798: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A279C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A27A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A27A4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A27A8: 386AB618  addi r3, r10, -0x49e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18920;
	// 832A27AC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832A27B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A27B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A27B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 832A27BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A27C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 832A27C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A27C8: 4BAB3479  bl 0x82d55c40
	ctx.lr = 0x832A27CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A27CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A27D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A27D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A27D8: 4E800020  blr
	return;
}

pub fn sub_832A27E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A27E0 size=12
	// 832A27E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A27E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A27E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832A2840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2840 size=112
	// 832A2840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A284C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A2850: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A2854: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 832A2858: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A285C: 390B8678  addi r8, r11, -0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + -31112;
	// 832A2860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A2864: 388A86EC  addi r4, r10, -0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + -30996;
	// 832A2868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A286C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A2874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2878: 386AB678  addi r3, r10, -0x4988
	ctx.r[3].s64 = ctx.r[10].s64 + -18824;
	// 832A287C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A2880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A2884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A2888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A288C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A2894: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A2898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A289C: 4BAB33A5  bl 0x82d55c40
	ctx.lr = 0x832A28A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A28A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A28A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A28A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A28AC: 4E800020  blr
	return;
}

pub fn sub_832A28B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A28B0 size=72
	// 832A28B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A28B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A28B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A28BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A28C0: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A28C4: 388A8770  addi r4, r10, -0x7890
	ctx.r[4].s64 = ctx.r[10].s64 + -30864;
	// 832A28C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A28CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A28D0: 386AB6AC  addi r3, r10, -0x4954
	ctx.r[3].s64 = ctx.r[10].s64 + -18772;
	// 832A28D4: 38ABFBF0  addi r5, r11, -0x410
	ctx.r[5].s64 = ctx.r[11].s64 + -1040;
	// 832A28D8: 4BBE9E99  bl 0x82e8c770
	ctx.lr = 0x832A28DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8C770);
	// 832A28DC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A28E0: 386B7F00  addi r3, r11, 0x7f00
	ctx.r[3].s64 = ctx.r[11].s64 + 32512;
	// 832A28E4: 4BA0763D  bl 0x82ca9f20
	ctx.lr = 0x832A28E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A28E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A28EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A28F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A28F4: 4E800020  blr
	return;
}

pub fn sub_832A28F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A28F8 size=80
	// 832A28F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A28FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2904: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2908: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A290C: 388A87F8  addi r4, r10, -0x7808
	ctx.r[4].s64 = ctx.r[10].s64 + -30728;
	// 832A2910: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2918: 386AB7C0  addi r3, r10, -0x4840
	ctx.r[3].s64 = ctx.r[10].s64 + -18496;
	// 832A291C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2920: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2924: 38AB0C10  addi r5, r11, 0xc10
	ctx.r[5].s64 = ctx.r[11].s64 + 3088;
	// 832A2928: 4BBE6D79  bl 0x82e896a0
	ctx.lr = 0x832A292C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A292C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2930: 386B7F18  addi r3, r11, 0x7f18
	ctx.r[3].s64 = ctx.r[11].s64 + 32536;
	// 832A2934: 4BA075ED  bl 0x82ca9f20
	ctx.lr = 0x832A2938;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A293C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2944: 4E800020  blr
	return;
}

pub fn sub_832A2948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2948 size=80
	// 832A2948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2954: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2958: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A295C: 388A88C4  addi r4, r10, -0x773c
	ctx.r[4].s64 = ctx.r[10].s64 + -30524;
	// 832A2960: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2968: 386AB8D4  addi r3, r10, -0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + -18220;
	// 832A296C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2970: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2974: 38AB1578  addi r5, r11, 0x1578
	ctx.r[5].s64 = ctx.r[11].s64 + 5496;
	// 832A2978: 4BBE6D29  bl 0x82e896a0
	ctx.lr = 0x832A297C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A297C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2980: 386B7F30  addi r3, r11, 0x7f30
	ctx.r[3].s64 = ctx.r[11].s64 + 32560;
	// 832A2984: 4BA0759D  bl 0x82ca9f20
	ctx.lr = 0x832A2988;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A298C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2994: 4E800020  blr
	return;
}

pub fn sub_832A2998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2998 size=80
	// 832A2998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A299C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A29A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A29A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A29A8: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A29AC: 388A88F4  addi r4, r10, -0x770c
	ctx.r[4].s64 = ctx.r[10].s64 + -30476;
	// 832A29B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A29B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A29B8: 386AB9E8  addi r3, r10, -0x4618
	ctx.r[3].s64 = ctx.r[10].s64 + -17944;
	// 832A29BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A29C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A29C4: 38AB2118  addi r5, r11, 0x2118
	ctx.r[5].s64 = ctx.r[11].s64 + 8472;
	// 832A29C8: 4BBE6CD9  bl 0x82e896a0
	ctx.lr = 0x832A29CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A29CC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A29D0: 386B7F48  addi r3, r11, 0x7f48
	ctx.r[3].s64 = ctx.r[11].s64 + 32584;
	// 832A29D4: 4BA0754D  bl 0x82ca9f20
	ctx.lr = 0x832A29D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A29D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A29E4: 4E800020  blr
	return;
}

pub fn sub_832A29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A29E8 size=80
	// 832A29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A29F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A29F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A29F8: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A29FC: 388A8938  addi r4, r10, -0x76c8
	ctx.r[4].s64 = ctx.r[10].s64 + -30408;
	// 832A2A00: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2A08: 386ABAFC  addi r3, r10, -0x4504
	ctx.r[3].s64 = ctx.r[10].s64 + -17668;
	// 832A2A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2A10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2A14: 38AB3410  addi r5, r11, 0x3410
	ctx.r[5].s64 = ctx.r[11].s64 + 13328;
	// 832A2A18: 4BBE6C89  bl 0x82e896a0
	ctx.lr = 0x832A2A1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A2A1C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2A20: 386B7F60  addi r3, r11, 0x7f60
	ctx.r[3].s64 = ctx.r[11].s64 + 32608;
	// 832A2A24: 4BA074FD  bl 0x82ca9f20
	ctx.lr = 0x832A2A28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2A34: 4E800020  blr
	return;
}

pub fn sub_832A2A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2A38 size=80
	// 832A2A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2A44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2A48: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2A4C: 388A894C  addi r4, r10, -0x76b4
	ctx.r[4].s64 = ctx.r[10].s64 + -30388;
	// 832A2A50: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2A58: 386ABC10  addi r3, r10, -0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17392;
	// 832A2A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2A60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2A64: 38AB3928  addi r5, r11, 0x3928
	ctx.r[5].s64 = ctx.r[11].s64 + 14632;
	// 832A2A68: 4BBE6C39  bl 0x82e896a0
	ctx.lr = 0x832A2A6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A2A6C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2A70: 386B7F78  addi r3, r11, 0x7f78
	ctx.r[3].s64 = ctx.r[11].s64 + 32632;
	// 832A2A74: 4BA074AD  bl 0x82ca9f20
	ctx.lr = 0x832A2A78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2A84: 4E800020  blr
	return;
}

pub fn sub_832A2A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2A88 size=72
	// 832A2A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2A94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2A98: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2A9C: 388A89B0  addi r4, r10, -0x7650
	ctx.r[4].s64 = ctx.r[10].s64 + -30288;
	// 832A2AA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2AA8: 386ABD28  addi r3, r10, -0x42d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17112;
	// 832A2AAC: 38AB50F8  addi r5, r11, 0x50f8
	ctx.r[5].s64 = ctx.r[11].s64 + 20728;
	// 832A2AB0: 4BBE9CC1  bl 0x82e8c770
	ctx.lr = 0x832A2AB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8C770);
	// 832A2AB4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2AB8: 386B7F90  addi r3, r11, 0x7f90
	ctx.r[3].s64 = ctx.r[11].s64 + 32656;
	// 832A2ABC: 4BA07465  bl 0x82ca9f20
	ctx.lr = 0x832A2AC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2ACC: 4E800020  blr
	return;
}

pub fn sub_832A2AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2AD0 size=80
	// 832A2AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2ADC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2AE0: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2AE4: 388A8AF4  addi r4, r10, -0x750c
	ctx.r[4].s64 = ctx.r[10].s64 + -29964;
	// 832A2AE8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2AF0: 386ABE3C  addi r3, r10, -0x41c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	// 832A2AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2AF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2AFC: 38AB57C0  addi r5, r11, 0x57c0
	ctx.r[5].s64 = ctx.r[11].s64 + 22464;
	// 832A2B00: 4BBE6BA1  bl 0x82e896a0
	ctx.lr = 0x832A2B04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A2B04: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2B08: 386B7FA8  addi r3, r11, 0x7fa8
	ctx.r[3].s64 = ctx.r[11].s64 + 32680;
	// 832A2B0C: 4BA07415  bl 0x82ca9f20
	ctx.lr = 0x832A2B10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2B1C: 4E800020  blr
	return;
}

pub fn sub_832A2B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2B20 size=80
	// 832A2B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2B2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2B30: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2B34: 388A8B90  addi r4, r10, -0x7470
	ctx.r[4].s64 = ctx.r[10].s64 + -29808;
	// 832A2B38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2B40: 386ABF50  addi r3, r10, -0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	// 832A2B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2B48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2B4C: 38AB8708  addi r5, r11, -0x78f8
	ctx.r[5].s64 = ctx.r[11].s64 + -30968;
	// 832A2B50: 4BBE6B51  bl 0x82e896a0
	ctx.lr = 0x832A2B54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E896A0);
	// 832A2B54: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2B58: 386B7FC0  addi r3, r11, 0x7fc0
	ctx.r[3].s64 = ctx.r[11].s64 + 32704;
	// 832A2B5C: 4BA073C5  bl 0x82ca9f20
	ctx.lr = 0x832A2B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2B6C: 4E800020  blr
	return;
}

pub fn sub_832A2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2B70 size=72
	// 832A2B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2B7C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2B80: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2B84: 388A8BB8  addi r4, r10, -0x7448
	ctx.r[4].s64 = ctx.r[10].s64 + -29768;
	// 832A2B88: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2B90: 386AC064  addi r3, r10, -0x3f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -16284;
	// 832A2B94: 38AB8EA0  addi r5, r11, -0x7160
	ctx.r[5].s64 = ctx.r[11].s64 + -29024;
	// 832A2B98: 4BBDCA11  bl 0x82e7f5a8
	ctx.lr = 0x832A2B9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2B9C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2BA0: 386B7FD8  addi r3, r11, 0x7fd8
	ctx.r[3].s64 = ctx.r[11].s64 + 32728;
	// 832A2BA4: 4BA0737D  bl 0x82ca9f20
	ctx.lr = 0x832A2BA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2BB4: 4E800020  blr
	return;
}

pub fn sub_832A2BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2BB8 size=72
	// 832A2BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2BC4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2BC8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2BCC: 388A8BE0  addi r4, r10, -0x7420
	ctx.r[4].s64 = ctx.r[10].s64 + -29728;
	// 832A2BD0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2BD8: 386AC178  addi r3, r10, -0x3e88
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	// 832A2BDC: 38AB8F08  addi r5, r11, -0x70f8
	ctx.r[5].s64 = ctx.r[11].s64 + -28920;
	// 832A2BE0: 4BBDC9C9  bl 0x82e7f5a8
	ctx.lr = 0x832A2BE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2BE4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2BE8: 386B7FF0  addi r3, r11, 0x7ff0
	ctx.r[3].s64 = ctx.r[11].s64 + 32752;
	// 832A2BEC: 4BA07335  bl 0x82ca9f20
	ctx.lr = 0x832A2BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2BFC: 4E800020  blr
	return;
}

pub fn sub_832A2C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C00 size=72
	// 832A2C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C0C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2C10: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2C14: 388A8C04  addi r4, r10, -0x73fc
	ctx.r[4].s64 = ctx.r[10].s64 + -29692;
	// 832A2C18: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2C20: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 832A2C24: 38AB8FE8  addi r5, r11, -0x7018
	ctx.r[5].s64 = ctx.r[11].s64 + -28696;
	// 832A2C28: 4BBDC981  bl 0x82e7f5a8
	ctx.lr = 0x832A2C2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2C2C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2C30: 386B8008  addi r3, r11, -0x7ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -32760;
	// 832A2C34: 4BA072ED  bl 0x82ca9f20
	ctx.lr = 0x832A2C38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2C44: 4E800020  blr
	return;
}

pub fn sub_832A2C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C48 size=72
	// 832A2C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2C58: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2C5C: 388A8C28  addi r4, r10, -0x73d8
	ctx.r[4].s64 = ctx.r[10].s64 + -29656;
	// 832A2C60: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2C68: 386AC3A0  addi r3, r10, -0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	// 832A2C6C: 38AB9068  addi r5, r11, -0x6f98
	ctx.r[5].s64 = ctx.r[11].s64 + -28568;
	// 832A2C70: 4BBDC939  bl 0x82e7f5a8
	ctx.lr = 0x832A2C74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2C74: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2C78: 386B8020  addi r3, r11, -0x7fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -32736;
	// 832A2C7C: 4BA072A5  bl 0x82ca9f20
	ctx.lr = 0x832A2C80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2C8C: 4E800020  blr
	return;
}

pub fn sub_832A2C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C90 size=72
	// 832A2C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2CA0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2CA4: 388A8C4C  addi r4, r10, -0x73b4
	ctx.r[4].s64 = ctx.r[10].s64 + -29620;
	// 832A2CA8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2CB0: 386AC4B4  addi r3, r10, -0x3b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15180;
	// 832A2CB4: 38AB9170  addi r5, r11, -0x6e90
	ctx.r[5].s64 = ctx.r[11].s64 + -28304;
	// 832A2CB8: 4BBDC8F1  bl 0x82e7f5a8
	ctx.lr = 0x832A2CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2CBC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2CC0: 386B8038  addi r3, r11, -0x7fc8
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	// 832A2CC4: 4BA0725D  bl 0x82ca9f20
	ctx.lr = 0x832A2CC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2CD4: 4E800020  blr
	return;
}

pub fn sub_832A2CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2CD8 size=72
	// 832A2CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2CE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2CE8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2CEC: 388A8C78  addi r4, r10, -0x7388
	ctx.r[4].s64 = ctx.r[10].s64 + -29576;
	// 832A2CF0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2CF8: 386AC5C8  addi r3, r10, -0x3a38
	ctx.r[3].s64 = ctx.r[10].s64 + -14904;
	// 832A2CFC: 38AB9260  addi r5, r11, -0x6da0
	ctx.r[5].s64 = ctx.r[11].s64 + -28064;
	// 832A2D00: 4BBDC8A9  bl 0x82e7f5a8
	ctx.lr = 0x832A2D04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2D04: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D08: 386B8050  addi r3, r11, -0x7fb0
	ctx.r[3].s64 = ctx.r[11].s64 + -32688;
	// 832A2D0C: 4BA07215  bl 0x82ca9f20
	ctx.lr = 0x832A2D10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2D1C: 4E800020  blr
	return;
}

pub fn sub_832A2D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2D20 size=72
	// 832A2D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2D2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2D30: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2D34: 388A8C8C  addi r4, r10, -0x7374
	ctx.r[4].s64 = ctx.r[10].s64 + -29556;
	// 832A2D38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2D40: 386AC6DC  addi r3, r10, -0x3924
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	// 832A2D44: 38AB92E0  addi r5, r11, -0x6d20
	ctx.r[5].s64 = ctx.r[11].s64 + -27936;
	// 832A2D48: 4BBDAFC1  bl 0x82e7dd08
	ctx.lr = 0x832A2D4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2D4C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D50: 386B8068  addi r3, r11, -0x7f98
	ctx.r[3].s64 = ctx.r[11].s64 + -32664;
	// 832A2D54: 4BA071CD  bl 0x82ca9f20
	ctx.lr = 0x832A2D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2D64: 4E800020  blr
	return;
}

pub fn sub_832A2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2D68 size=72
	// 832A2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2D74: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2D78: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2D7C: 388A8CA0  addi r4, r10, -0x7360
	ctx.r[4].s64 = ctx.r[10].s64 + -29536;
	// 832A2D80: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2D88: 386AC7F0  addi r3, r10, -0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	// 832A2D8C: 38AB9328  addi r5, r11, -0x6cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -27864;
	// 832A2D90: 4BBDAF79  bl 0x82e7dd08
	ctx.lr = 0x832A2D94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2D94: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D98: 386B8080  addi r3, r11, -0x7f80
	ctx.r[3].s64 = ctx.r[11].s64 + -32640;
	// 832A2D9C: 4BA07185  bl 0x82ca9f20
	ctx.lr = 0x832A2DA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2DAC: 4E800020  blr
	return;
}

pub fn sub_832A2DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2DB0 size=72
	// 832A2DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2DBC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2DC0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2DC4: 388A8CC8  addi r4, r10, -0x7338
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	// 832A2DC8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2DD0: 386AC904  addi r3, r10, -0x36fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14076;
	// 832A2DD4: 38AB93D0  addi r5, r11, -0x6c30
	ctx.r[5].s64 = ctx.r[11].s64 + -27696;
	// 832A2DD8: 4BBDAF31  bl 0x82e7dd08
	ctx.lr = 0x832A2DDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2DDC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2DE0: 386B8098  addi r3, r11, -0x7f68
	ctx.r[3].s64 = ctx.r[11].s64 + -32616;
	// 832A2DE4: 4BA0713D  bl 0x82ca9f20
	ctx.lr = 0x832A2DE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2DF4: 4E800020  blr
	return;
}

pub fn sub_832A2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2DF8 size=72
	// 832A2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E04: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E08: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E0C: 388A8CF4  addi r4, r10, -0x730c
	ctx.r[4].s64 = ctx.r[10].s64 + -29452;
	// 832A2E10: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2E18: 386ACA18  addi r3, r10, -0x35e8
	ctx.r[3].s64 = ctx.r[10].s64 + -13800;
	// 832A2E1C: 38AB9428  addi r5, r11, -0x6bd8
	ctx.r[5].s64 = ctx.r[11].s64 + -27608;
	// 832A2E20: 4BBDAEE9  bl 0x82e7dd08
	ctx.lr = 0x832A2E24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2E24: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2E28: 386B80B0  addi r3, r11, -0x7f50
	ctx.r[3].s64 = ctx.r[11].s64 + -32592;
	// 832A2E2C: 4BA070F5  bl 0x82ca9f20
	ctx.lr = 0x832A2E30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2E3C: 4E800020  blr
	return;
}

pub fn sub_832A2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2E40 size=72
	// 832A2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E50: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E54: 388A8D1C  addi r4, r10, -0x72e4
	ctx.r[4].s64 = ctx.r[10].s64 + -29412;
	// 832A2E58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2E60: 386ACB2C  addi r3, r10, -0x34d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	// 832A2E64: 38AB9538  addi r5, r11, -0x6ac8
	ctx.r[5].s64 = ctx.r[11].s64 + -27336;
	// 832A2E68: 4BBDAEA1  bl 0x82e7dd08
	ctx.lr = 0x832A2E6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2E6C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2E70: 386B80C8  addi r3, r11, -0x7f38
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	// 832A2E74: 4BA070AD  bl 0x82ca9f20
	ctx.lr = 0x832A2E78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2E84: 4E800020  blr
	return;
}

pub fn sub_832A2E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2E88 size=72
	// 832A2E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E98: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E9C: 388A8D40  addi r4, r10, -0x72c0
	ctx.r[4].s64 = ctx.r[10].s64 + -29376;
	// 832A2EA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2EA8: 386ACC40  addi r3, r10, -0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	// 832A2EAC: 38AB9628  addi r5, r11, -0x69d8
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	// 832A2EB0: 4BBDC6F9  bl 0x82e7f5a8
	ctx.lr = 0x832A2EB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2EB4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2EB8: 386B80E0  addi r3, r11, -0x7f20
	ctx.r[3].s64 = ctx.r[11].s64 + -32544;
	// 832A2EBC: 4BA07065  bl 0x82ca9f20
	ctx.lr = 0x832A2EC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2ECC: 4E800020  blr
	return;
}

pub fn sub_832A2ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2ED0 size=72
	// 832A2ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2EDC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2EE0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2EE4: 388A8D6C  addi r4, r10, -0x7294
	ctx.r[4].s64 = ctx.r[10].s64 + -29332;
	// 832A2EE8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2EF0: 386ACD54  addi r3, r10, -0x32ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	// 832A2EF4: 38AB9690  addi r5, r11, -0x6970
	ctx.r[5].s64 = ctx.r[11].s64 + -26992;
	// 832A2EF8: 4BBDAE11  bl 0x82e7dd08
	ctx.lr = 0x832A2EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2EFC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F00: 386B80F8  addi r3, r11, -0x7f08
	ctx.r[3].s64 = ctx.r[11].s64 + -32520;
	// 832A2F04: 4BA0701D  bl 0x82ca9f20
	ctx.lr = 0x832A2F08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2F14: 4E800020  blr
	return;
}

pub fn sub_832A2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2F18 size=72
	// 832A2F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2F24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2F28: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2F2C: 388A8D90  addi r4, r10, -0x7270
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	// 832A2F30: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2F38: 386ACE68  addi r3, r10, -0x3198
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	// 832A2F3C: 38AB9750  addi r5, r11, -0x68b0
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	// 832A2F40: 4BBDADC9  bl 0x82e7dd08
	ctx.lr = 0x832A2F44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2F44: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F48: 386B8110  addi r3, r11, -0x7ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	// 832A2F4C: 4BA06FD5  bl 0x82ca9f20
	ctx.lr = 0x832A2F50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2F5C: 4E800020  blr
	return;
}

pub fn sub_832A2F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2F60 size=72
	// 832A2F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2F6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2F70: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2F74: 388A8DB8  addi r4, r10, -0x7248
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	// 832A2F78: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2F80: 386ACF7C  addi r3, r10, -0x3084
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	// 832A2F84: 38AB97A8  addi r5, r11, -0x6858
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	// 832A2F88: 4BBDAD81  bl 0x82e7dd08
	ctx.lr = 0x832A2F8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A2F8C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F90: 386B8128  addi r3, r11, -0x7ed8
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	// 832A2F94: 4BA06F8D  bl 0x82ca9f20
	ctx.lr = 0x832A2F98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2FA4: 4E800020  blr
	return;
}

pub fn sub_832A2FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2FA8 size=72
	// 832A2FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2FB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2FB8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2FBC: 388A8DEC  addi r4, r10, -0x7214
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	// 832A2FC0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2FC8: 386AD098  addi r3, r10, -0x2f68
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	// 832A2FCC: 38AB9D08  addi r5, r11, -0x62f8
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	// 832A2FD0: 4BBDC5D9  bl 0x82e7f5a8
	ctx.lr = 0x832A2FD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A2FD4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2FD8: 386B8140  addi r3, r11, -0x7ec0
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	// 832A2FDC: 4BA06F45  bl 0x82ca9f20
	ctx.lr = 0x832A2FE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A2FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2FEC: 4E800020  blr
	return;
}

pub fn sub_832A2FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2FF0 size=72
	// 832A2FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2FFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3000: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3004: 388A8E10  addi r4, r10, -0x71f0
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	// 832A3008: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A300C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3010: 386AD1AC  addi r3, r10, -0x2e54
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	// 832A3014: 38AB9ED0  addi r5, r11, -0x6130
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	// 832A3018: 4BBDC591  bl 0x82e7f5a8
	ctx.lr = 0x832A301C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A301C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3020: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 832A3024: 4BA06EFD  bl 0x82ca9f20
	ctx.lr = 0x832A3028;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A302C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3034: 4E800020  blr
	return;
}

pub fn sub_832A3038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3038 size=72
	// 832A3038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A303C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3044: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3048: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A304C: 388A8E34  addi r4, r10, -0x71cc
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	// 832A3050: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3058: 386AD2C0  addi r3, r10, -0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	// 832A305C: 38AB9F88  addi r5, r11, -0x6078
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	// 832A3060: 4BBDC549  bl 0x82e7f5a8
	ctx.lr = 0x832A3064;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A3064: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3068: 386B8170  addi r3, r11, -0x7e90
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	// 832A306C: 4BA06EB5  bl 0x82ca9f20
	ctx.lr = 0x832A3070;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A307C: 4E800020  blr
	return;
}

pub fn sub_832A3080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3080 size=72
	// 832A3080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A308C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3090: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3094: 388A8E60  addi r4, r10, -0x71a0
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	// 832A3098: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A309C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A30A0: 386AD3D4  addi r3, r10, -0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	// 832A30A4: 38AB9FF0  addi r5, r11, -0x6010
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	// 832A30A8: 4BBDC501  bl 0x82e7f5a8
	ctx.lr = 0x832A30AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A30AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A30B0: 386B8188  addi r3, r11, -0x7e78
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	// 832A30B4: 4BA06E6D  bl 0x82ca9f20
	ctx.lr = 0x832A30B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A30B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A30BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A30C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A30C4: 4E800020  blr
	return;
}

pub fn sub_832A30C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A30C8 size=72
	// 832A30C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A30CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A30D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A30D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A30D8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A30DC: 388A8E88  addi r4, r10, -0x7178
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	// 832A30E0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A30E8: 386AD4E8  addi r3, r10, -0x2b18
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	// 832A30EC: 38ABA058  addi r5, r11, -0x5fa8
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	// 832A30F0: 4BBDC4B9  bl 0x82e7f5a8
	ctx.lr = 0x832A30F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A30F4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A30F8: 386B81A0  addi r3, r11, -0x7e60
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	// 832A30FC: 4BA06E25  bl 0x82ca9f20
	ctx.lr = 0x832A3100;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A310C: 4E800020  blr
	return;
}

pub fn sub_832A3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3110 size=72
	// 832A3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A311C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3120: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3124: 388A8EB4  addi r4, r10, -0x714c
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	// 832A3128: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A312C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3130: 386AD5FC  addi r3, r10, -0x2a04
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	// 832A3134: 38ABA128  addi r5, r11, -0x5ed8
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	// 832A3138: 4BBDC471  bl 0x82e7f5a8
	ctx.lr = 0x832A313C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A313C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3140: 386B81B8  addi r3, r11, -0x7e48
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	// 832A3144: 4BA06DDD  bl 0x82ca9f20
	ctx.lr = 0x832A3148;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A314C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3154: 4E800020  blr
	return;
}

pub fn sub_832A3158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3158 size=72
	// 832A3158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A315C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3164: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3168: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A316C: 388A8EE0  addi r4, r10, -0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	// 832A3170: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3178: 386AD710  addi r3, r10, -0x28f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	// 832A317C: 38ABA200  addi r5, r11, -0x5e00
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	// 832A3180: 4BBDAB89  bl 0x82e7dd08
	ctx.lr = 0x832A3184;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A3184: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3188: 386B81D0  addi r3, r11, -0x7e30
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	// 832A318C: 4BA06D95  bl 0x82ca9f20
	ctx.lr = 0x832A3190;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A319C: 4E800020  blr
	return;
}

pub fn sub_832A31A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A31A0 size=72
	// 832A31A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A31A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A31A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A31AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A31B0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A31B4: 388A8F10  addi r4, r10, -0x70f0
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	// 832A31B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A31BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A31C0: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 832A31C4: 38ABA258  addi r5, r11, -0x5da8
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	// 832A31C8: 4BBDAB41  bl 0x82e7dd08
	ctx.lr = 0x832A31CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A31CC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A31D0: 386B81E8  addi r3, r11, -0x7e18
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	// 832A31D4: 4BA06D4D  bl 0x82ca9f20
	ctx.lr = 0x832A31D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A31D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A31DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A31E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A31E4: 4E800020  blr
	return;
}

pub fn sub_832A31E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A31E8 size=72
	// 832A31E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A31EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A31F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A31F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A31F8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A31FC: 388A8F3C  addi r4, r10, -0x70c4
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	// 832A3200: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3208: 386AD938  addi r3, r10, -0x26c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	// 832A320C: 38ABA320  addi r5, r11, -0x5ce0
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	// 832A3210: 4BBDAAF9  bl 0x82e7dd08
	ctx.lr = 0x832A3214;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DD08);
	// 832A3214: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3218: 386B8200  addi r3, r11, -0x7e00
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	// 832A321C: 4BA06D05  bl 0x82ca9f20
	ctx.lr = 0x832A3220;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A322C: 4E800020  blr
	return;
}

pub fn sub_832A3230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3230 size=72
	// 832A3230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A323C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3240: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3244: 388A8F60  addi r4, r10, -0x70a0
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	// 832A3248: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A324C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3250: 386ADA4C  addi r3, r10, -0x25b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	// 832A3254: 38ABA378  addi r5, r11, -0x5c88
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	// 832A3258: 4BBDC351  bl 0x82e7f5a8
	ctx.lr = 0x832A325C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F5A8);
	// 832A325C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3260: 386B8218  addi r3, r11, -0x7de8
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	// 832A3264: 4BA06CBD  bl 0x82ca9f20
	ctx.lr = 0x832A3268;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A326C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3274: 4E800020  blr
	return;
}

pub fn sub_832A3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3278 size=80
	// 832A3278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3284: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3288: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A328C: 388A9014  addi r4, r10, -0x6fec
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	// 832A3290: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3298: 386ADF68  addi r3, r10, -0x2098
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	// 832A329C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A32A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A32A4: 38ABB3F8  addi r5, r11, -0x4c08
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	// 832A32A8: 4BBE2519  bl 0x82e857c0
	ctx.lr = 0x832A32AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A32AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A32B0: 386B8230  addi r3, r11, -0x7dd0
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	// 832A32B4: 4BA06C6D  bl 0x82ca9f20
	ctx.lr = 0x832A32B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A32B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A32BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A32C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A32C4: 4E800020  blr
	return;
}

pub fn sub_832A32C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A32C8 size=80
	// 832A32C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A32CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A32D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A32D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A32D8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A32DC: 388A9048  addi r4, r10, -0x6fb8
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	// 832A32E0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A32E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A32E8: 386AE07C  addi r3, r10, -0x1f84
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	// 832A32EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A32F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A32F4: 38ABC190  addi r5, r11, -0x3e70
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	// 832A32F8: 4BBE24C9  bl 0x82e857c0
	ctx.lr = 0x832A32FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A32FC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3300: 386B8260  addi r3, r11, -0x7da0
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	// 832A3304: 4BA06C1D  bl 0x82ca9f20
	ctx.lr = 0x832A3308;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A330C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3314: 4E800020  blr
	return;
}

pub fn sub_832A3318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3318 size=96
	// 832A3318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A331C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3324: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3328: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A332C: 388A9080  addi r4, r10, -0x6f80
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	// 832A3330: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3338: 386AE190  addi r3, r10, -0x1e70
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	// 832A333C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3340: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3344: 38ABC6E0  addi r5, r11, -0x3920
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	// 832A3348: 4BBE2479  bl 0x82e857c0
	ctx.lr = 0x832A334C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A334C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3350: 386B8278  addi r3, r11, -0x7d88
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	// 832A3354: 4BA06BCD  bl 0x82ca9f20
	ctx.lr = 0x832A3358;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3364: 4E800020  blr
	return;
	// 832A3368: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A336C: 386B8338  addi r3, r11, -0x7cc8
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	// 832A3370: 4BA06BB0  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_832A3378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3378 size=104
	// 832A3378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3388: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A338C: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3390: 388A9244  addi r4, r10, -0x6dbc
	ctx.r[4].s64 = ctx.r[10].s64 + -28092;
	// 832A3394: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A339C: 3BEA071C  addi r31, r10, 0x71c
	ctx.r[31].s64 = ctx.r[10].s64 + 1820;
	// 832A33A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A33A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A33A8: 38AB33C0  addi r5, r11, 0x33c0
	ctx.r[5].s64 = ctx.r[11].s64 + 13248;
	// 832A33AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A33B0: 4BBE09E9  bl 0x82e83d98
	ctx.lr = 0x832A33B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E83D98);
	// 832A33B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A33B8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A33BC: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A33C0: 386A8370  addi r3, r10, -0x7c90
	ctx.r[3].s64 = ctx.r[10].s64 + -31888;
	// 832A33C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A33C8: 4BA06B59  bl 0x82ca9f20
	ctx.lr = 0x832A33CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A33CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A33D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A33D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A33D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A33DC: 4E800020  blr
	return;
}

pub fn sub_832A33E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A33E0 size=80
	// 832A33E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A33E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A33E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A33EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A33F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A33F4: 388B945C  addi r4, r11, -0x6ba4
	ctx.r[4].s64 = ctx.r[11].s64 + -27556;
	// 832A33F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A33FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3400: 386B1048  addi r3, r11, 0x1048
	ctx.r[3].s64 = ctx.r[11].s64 + 4168;
	// 832A3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3408: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A340C: 4BBE2375  bl 0x82e85780
	ctx.lr = 0x832A3410;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85780);
	// 832A3410: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3414: 386B83A0  addi r3, r11, -0x7c60
	ctx.r[3].s64 = ctx.r[11].s64 + -31840;
	// 832A3418: 4BA06B09  bl 0x82ca9f20
	ctx.lr = 0x832A341C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A341C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3428: 4E800020  blr
	return;
}

pub fn sub_832A3430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3430 size=104
	// 832A3430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A343C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3440: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3448: 388B94FC  addi r4, r11, -0x6b04
	ctx.r[4].s64 = ctx.r[11].s64 + -27396;
	// 832A344C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3450: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3454: 3BEB1A8C  addi r31, r11, 0x1a8c
	ctx.r[31].s64 = ctx.r[11].s64 + 6796;
	// 832A3458: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A345C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3464: 4BBE380D  bl 0x82e86c70
	ctx.lr = 0x832A3468;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86C70);
	// 832A3468: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A346C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3470: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3474: 386A8400  addi r3, r10, -0x7c00
	ctx.r[3].s64 = ctx.r[10].s64 + -31744;
	// 832A3478: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A347C: 4BA06AA5  bl 0x82ca9f20
	ctx.lr = 0x832A3480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A348C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3490: 4E800020  blr
	return;
}

pub fn sub_832A3498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3498 size=104
	// 832A3498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A349C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A34A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A34A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A34A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A34AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A34B0: 388B950C  addi r4, r11, -0x6af4
	ctx.r[4].s64 = ctx.r[11].s64 + -27380;
	// 832A34B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A34B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A34BC: 3BEB1978  addi r31, r11, 0x1978
	ctx.r[31].s64 = ctx.r[11].s64 + 6520;
	// 832A34C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A34C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A34C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A34CC: 4BBE37A5  bl 0x82e86c70
	ctx.lr = 0x832A34D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86C70);
	// 832A34D0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A34D4: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A34D8: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A34DC: 386A83E8  addi r3, r10, -0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + -31768;
	// 832A34E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A34E4: 4BA06A3D  bl 0x82ca9f20
	ctx.lr = 0x832A34E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A34E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A34EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A34F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A34F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A34F8: 4E800020  blr
	return;
}

pub fn sub_832A3500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3500 size=80
	// 832A3500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A350C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3510: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3514: 388A95F4  addi r4, r10, -0x6a0c
	ctx.r[4].s64 = ctx.r[10].s64 + -27148;
	// 832A3518: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A351C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3520: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 832A3524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3528: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A352C: 38AB8608  addi r5, r11, -0x79f8
	ctx.r[5].s64 = ctx.r[11].s64 + -31224;
	// 832A3530: 4BBE2291  bl 0x82e857c0
	ctx.lr = 0x832A3534;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A3534: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3538: 386B8430  addi r3, r11, -0x7bd0
	ctx.r[3].s64 = ctx.r[11].s64 + -31696;
	// 832A353C: 4BA069E5  bl 0x82ca9f20
	ctx.lr = 0x832A3540;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A354C: 4E800020  blr
	return;
}

pub fn sub_832A3550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3550 size=104
	// 832A3550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A355C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3560: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3568: 388B964C  addi r4, r11, -0x69b4
	ctx.r[4].s64 = ctx.r[11].s64 + -27060;
	// 832A356C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3574: 3BEB20C0  addi r31, r11, 0x20c0
	ctx.r[31].s64 = ctx.r[11].s64 + 8384;
	// 832A3578: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A357C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3584: 4BBE6445  bl 0x82e899c8
	ctx.lr = 0x832A3588;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E899C8);
	// 832A3588: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A358C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3590: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3594: 386A8448  addi r3, r10, -0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -31672;
	// 832A3598: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A359C: 4BA06985  bl 0x82ca9f20
	ctx.lr = 0x832A35A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A35A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A35A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A35A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A35AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A35B0: 4E800020  blr
	return;
}

pub fn sub_832A35B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A35B8 size=104
	// 832A35B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A35BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A35C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A35C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A35C8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A35CC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A35D0: 388A96A8  addi r4, r10, -0x6958
	ctx.r[4].s64 = ctx.r[10].s64 + -26968;
	// 832A35D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A35D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A35DC: 3BEA21D8  addi r31, r10, 0x21d8
	ctx.r[31].s64 = ctx.r[10].s64 + 8664;
	// 832A35E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A35E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A35E8: 38ABA350  addi r5, r11, -0x5cb0
	ctx.r[5].s64 = ctx.r[11].s64 + -23728;
	// 832A35EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A35F0: 4BBE6F81  bl 0x82e8a570
	ctx.lr = 0x832A35F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8A570);
	// 832A35F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A35F8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A35FC: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3600: 386A8478  addi r3, r10, -0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + -31624;
	// 832A3604: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3608: 4BA06919  bl 0x82ca9f20
	ctx.lr = 0x832A360C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A360C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A361C: 4E800020  blr
	return;
}

pub fn sub_832A3620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3620 size=72
	// 832A3620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A362C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3630: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3634: 388A971C  addi r4, r10, -0x68e4
	ctx.r[4].s64 = ctx.r[10].s64 + -26852;
	// 832A3638: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3640: 386A22EC  addi r3, r10, 0x22ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8940;
	// 832A3644: 38ABB520  addi r5, r11, -0x4ae0
	ctx.r[5].s64 = ctx.r[11].s64 + -19168;
	// 832A3648: 4BBE2C89  bl 0x82e862d0
	ctx.lr = 0x832A364C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E862D0);
	// 832A364C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3650: 386B8490  addi r3, r11, -0x7b70
	ctx.r[3].s64 = ctx.r[11].s64 + -31600;
	// 832A3654: 4BA068CD  bl 0x82ca9f20
	ctx.lr = 0x832A3658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A365C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3664: 4E800020  blr
	return;
}

pub fn sub_832A3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3668 size=72
	// 832A3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3674: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3678: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A367C: 388A974C  addi r4, r10, -0x68b4
	ctx.r[4].s64 = ctx.r[10].s64 + -26804;
	// 832A3680: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3688: 386A2400  addi r3, r10, 0x2400
	ctx.r[3].s64 = ctx.r[10].s64 + 9216;
	// 832A368C: 38ABB800  addi r5, r11, -0x4800
	ctx.r[5].s64 = ctx.r[11].s64 + -18432;
	// 832A3690: 4BBE2C41  bl 0x82e862d0
	ctx.lr = 0x832A3694;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E862D0);
	// 832A3694: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3698: 386B84A8  addi r3, r11, -0x7b58
	ctx.r[3].s64 = ctx.r[11].s64 + -31576;
	// 832A369C: 4BA06885  bl 0x82ca9f20
	ctx.lr = 0x832A36A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A36A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A36A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A36A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A36AC: 4E800020  blr
	return;
}

pub fn sub_832A36B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A36B0 size=72
	// 832A36B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A36B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A36B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A36BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A36C0: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A36C4: 388A97A0  addi r4, r10, -0x6860
	ctx.r[4].s64 = ctx.r[10].s64 + -26720;
	// 832A36C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A36CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A36D0: 386A2514  addi r3, r10, 0x2514
	ctx.r[3].s64 = ctx.r[10].s64 + 9492;
	// 832A36D4: 38ABBC90  addi r5, r11, -0x4370
	ctx.r[5].s64 = ctx.r[11].s64 + -17264;
	// 832A36D8: 4BBE2BF9  bl 0x82e862d0
	ctx.lr = 0x832A36DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E862D0);
	// 832A36DC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A36E0: 386B84C0  addi r3, r11, -0x7b40
	ctx.r[3].s64 = ctx.r[11].s64 + -31552;
	// 832A36E4: 4BA0683D  bl 0x82ca9f20
	ctx.lr = 0x832A36E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A36E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A36EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A36F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A36F4: 4E800020  blr
	return;
}

pub fn sub_832A36F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A36F8 size=80
	// 832A36F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A36FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3704: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3708: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A370C: 388A989C  addi r4, r10, -0x6764
	ctx.r[4].s64 = ctx.r[10].s64 + -26468;
	// 832A3710: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3718: 386A2E38  addi r3, r10, 0x2e38
	ctx.r[3].s64 = ctx.r[10].s64 + 11832;
	// 832A371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3720: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3724: 38ABD820  addi r5, r11, -0x27e0
	ctx.r[5].s64 = ctx.r[11].s64 + -10208;
	// 832A3728: 4BBE2099  bl 0x82e857c0
	ctx.lr = 0x832A372C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A372C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3730: 386B84F0  addi r3, r11, -0x7b10
	ctx.r[3].s64 = ctx.r[11].s64 + -31504;
	// 832A3734: 4BA067ED  bl 0x82ca9f20
	ctx.lr = 0x832A3738;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A373C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3744: 4E800020  blr
	return;
}

pub fn sub_832A3748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3748 size=72
	// 832A3748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3754: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3758: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A375C: 388B996C  addi r4, r11, -0x6694
	ctx.r[4].s64 = ctx.r[11].s64 + -26260;
	// 832A3760: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3768: 386B581C  addi r3, r11, 0x581c
	ctx.r[3].s64 = ctx.r[11].s64 + 22556;
	// 832A376C: 4BBE248D  bl 0x82e85bf8
	ctx.lr = 0x832A3770;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A3770: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3774: 386B85F8  addi r3, r11, -0x7a08
	ctx.r[3].s64 = ctx.r[11].s64 + -31240;
	// 832A3778: 4BA067A9  bl 0x82ca9f20
	ctx.lr = 0x832A377C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A377C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3788: 4E800020  blr
	return;
}

pub fn sub_832A3790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3790 size=80
	// 832A3790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A379C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A37A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A37A4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A37A8: 388A9978  addi r4, r10, -0x6688
	ctx.r[4].s64 = ctx.r[10].s64 + -26248;
	// 832A37AC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A37B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A37B4: 38ABF078  addi r5, r11, -0xf88
	ctx.r[5].s64 = ctx.r[11].s64 + -3976;
	// 832A37B8: 386A52B8  addi r3, r10, 0x52b8
	ctx.r[3].s64 = ctx.r[10].s64 + 21176;
	// 832A37BC: 4BBE243D  bl 0x82e85bf8
	ctx.lr = 0x832A37C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A37C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A37C4: 386B8538  addi r3, r11, -0x7ac8
	ctx.r[3].s64 = ctx.r[11].s64 + -31432;
	// 832A37C8: 4BA06759  bl 0x82ca9f20
	ctx.lr = 0x832A37CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A37CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A37D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A37D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A37D8: 4E800020  blr
	return;
}

pub fn sub_832A37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A37E0 size=80
	// 832A37E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A37E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A37E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A37EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A37F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A37F4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A37F8: 388A9990  addi r4, r10, -0x6670
	ctx.r[4].s64 = ctx.r[10].s64 + -26224;
	// 832A37FC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3800: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3804: 38AB0198  addi r5, r11, 0x198
	ctx.r[5].s64 = ctx.r[11].s64 + 408;
	// 832A3808: 386A5708  addi r3, r10, 0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + 22280;
	// 832A380C: 4BBE23ED  bl 0x82e85bf8
	ctx.lr = 0x832A3810;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A3810: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3814: 386B8550  addi r3, r11, -0x7ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -31408;
	// 832A3818: 4BA06709  bl 0x82ca9f20
	ctx.lr = 0x832A381C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A381C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3828: 4E800020  blr
	return;
}

pub fn sub_832A3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3830 size=80
	// 832A3830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A383C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3840: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3844: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A3848: 388A99A8  addi r4, r10, -0x6658
	ctx.r[4].s64 = ctx.r[10].s64 + -26200;
	// 832A384C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3850: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3854: 38ABFAF0  addi r5, r11, -0x510
	ctx.r[5].s64 = ctx.r[11].s64 + -1296;
	// 832A3858: 386A53CC  addi r3, r10, 0x53cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21452;
	// 832A385C: 4BBE239D  bl 0x82e85bf8
	ctx.lr = 0x832A3860;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A3860: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3864: 386B8568  addi r3, r11, -0x7a98
	ctx.r[3].s64 = ctx.r[11].s64 + -31384;
	// 832A3868: 4BA066B9  bl 0x82ca9f20
	ctx.lr = 0x832A386C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A386C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3878: 4E800020  blr
	return;
}

pub fn sub_832A3880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3880 size=80
	// 832A3880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A388C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3890: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3894: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A3898: 388A99CC  addi r4, r10, -0x6634
	ctx.r[4].s64 = ctx.r[10].s64 + -26164;
	// 832A389C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A38A0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A38A4: 38ABFDB0  addi r5, r11, -0x250
	ctx.r[5].s64 = ctx.r[11].s64 + -592;
	// 832A38A8: 386A5B58  addi r3, r10, 0x5b58
	ctx.r[3].s64 = ctx.r[10].s64 + 23384;
	// 832A38AC: 4BBE234D  bl 0x82e85bf8
	ctx.lr = 0x832A38B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A38B0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A38B4: 386B8580  addi r3, r11, -0x7a80
	ctx.r[3].s64 = ctx.r[11].s64 + -31360;
	// 832A38B8: 4BA06669  bl 0x82ca9f20
	ctx.lr = 0x832A38BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A38BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A38C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A38C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A38C8: 4E800020  blr
	return;
}

pub fn sub_832A38D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A38D0 size=80
	// 832A38D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A38D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A38D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A38DC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A38E0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A38E4: 38CB52B8  addi r6, r11, 0x52b8
	ctx.r[6].s64 = ctx.r[11].s64 + 21176;
	// 832A38E8: 388A99E8  addi r4, r10, -0x6618
	ctx.r[4].s64 = ctx.r[10].s64 + -26136;
	// 832A38EC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A38F0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A38F4: 38ABF360  addi r5, r11, -0xca0
	ctx.r[5].s64 = ctx.r[11].s64 + -3232;
	// 832A38F8: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 832A38FC: 4BBE22FD  bl 0x82e85bf8
	ctx.lr = 0x832A3900;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 832A3900: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3904: 386B8598  addi r3, r11, -0x7a68
	ctx.r[3].s64 = ctx.r[11].s64 + -31336;
	// 832A3908: 4BA06619  bl 0x82ca9f20
	ctx.lr = 0x832A390C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A390C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3918: 4E800020  blr
	return;
}

pub fn sub_832A3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3920 size=104
	// 832A3920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A392C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3930: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3934: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3938: 388A99FC  addi r4, r10, -0x6604
	ctx.r[4].s64 = ctx.r[10].s64 + -26116;
	// 832A393C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3940: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3944: 3BEA55F4  addi r31, r10, 0x55f4
	ctx.r[31].s64 = ctx.r[10].s64 + 22004;
	// 832A3948: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3950: 38ABF450  addi r5, r11, -0xbb0
	ctx.r[5].s64 = ctx.r[11].s64 + -2992;
	// 832A3954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3958: 4BBEC9B9  bl 0x82e90310
	ctx.lr = 0x832A395C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E90310);
	// 832A395C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3960: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3964: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3968: 386A85B0  addi r3, r10, -0x7a50
	ctx.r[3].s64 = ctx.r[10].s64 + -31312;
	// 832A396C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3970: 4BA065B1  bl 0x82ca9f20
	ctx.lr = 0x832A3974;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3984: 4E800020  blr
	return;
}

pub fn sub_832A3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3988 size=104
	// 832A3988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3998: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A399C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A39A0: 388A9A18  addi r4, r10, -0x65e8
	ctx.r[4].s64 = ctx.r[10].s64 + -26088;
	// 832A39A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A39A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A39AC: 3BEA5A44  addi r31, r10, 0x5a44
	ctx.r[31].s64 = ctx.r[10].s64 + 23108;
	// 832A39B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A39B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A39B8: 38ABF498  addi r5, r11, -0xb68
	ctx.r[5].s64 = ctx.r[11].s64 + -2920;
	// 832A39BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A39C0: 4BBEC951  bl 0x82e90310
	ctx.lr = 0x832A39C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E90310);
	// 832A39C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A39C8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A39CC: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A39D0: 386A85C8  addi r3, r10, -0x7a38
	ctx.r[3].s64 = ctx.r[10].s64 + -31288;
	// 832A39D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A39D8: 4BA06549  bl 0x82ca9f20
	ctx.lr = 0x832A39DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A39DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A39E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A39E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A39E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A39EC: 4E800020  blr
	return;
}

pub fn sub_832A39F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A39F0 size=104
	// 832A39F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A39F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A39F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A39FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3A00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3A04: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3A08: 388A9A28  addi r4, r10, -0x65d8
	ctx.r[4].s64 = ctx.r[10].s64 + -26072;
	// 832A3A0C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3A10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3A14: 3BEA54E0  addi r31, r10, 0x54e0
	ctx.r[31].s64 = ctx.r[10].s64 + 21728;
	// 832A3A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3A20: 38ABF4E0  addi r5, r11, -0xb20
	ctx.r[5].s64 = ctx.r[11].s64 + -2848;
	// 832A3A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3A28: 4BBEC8E9  bl 0x82e90310
	ctx.lr = 0x832A3A2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E90310);
	// 832A3A2C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3A30: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3A34: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3A38: 386A85E0  addi r3, r10, -0x7a20
	ctx.r[3].s64 = ctx.r[10].s64 + -31264;
	// 832A3A3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3A40: 4BA064E1  bl 0x82ca9f20
	ctx.lr = 0x832A3A44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3A50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3A54: 4E800020  blr
	return;
}

pub fn sub_832A3A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3A58 size=72
	// 832A3A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3A64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3A68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3A6C: 388B9AB8  addi r4, r11, -0x6548
	ctx.r[4].s64 = ctx.r[11].s64 + -25928;
	// 832A3A70: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3A78: 386B5C6C  addi r3, r11, 0x5c6c
	ctx.r[3].s64 = ctx.r[11].s64 + 23660;
	// 832A3A7C: 4BBDBAE5  bl 0x82e7f560
	ctx.lr = 0x832A3A80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 832A3A80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3A84: 386B8628  addi r3, r11, -0x79d8
	ctx.r[3].s64 = ctx.r[11].s64 + -31192;
	// 832A3A88: 4BA06499  bl 0x82ca9f20
	ctx.lr = 0x832A3A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3A98: 4E800020  blr
	return;
}

pub fn sub_832A3AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3AA0 size=72
	// 832A3AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3AAC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3AB0: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3AB4: 388A9B64  addi r4, r10, -0x649c
	ctx.r[4].s64 = ctx.r[10].s64 + -25756;
	// 832A3AB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3ABC: 38AB6E58  addi r5, r11, 0x6e58
	ctx.r[5].s64 = ctx.r[11].s64 + 28248;
	// 832A3AC0: 386A5D84  addi r3, r10, 0x5d84
	ctx.r[3].s64 = ctx.r[10].s64 + 23940;
	// 832A3AC4: 4BBF4D1D  bl 0x82e987e0
	ctx.lr = 0x832A3AC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E987E0);
	// 832A3AC8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3ACC: 386B8640  addi r3, r11, -0x79c0
	ctx.r[3].s64 = ctx.r[11].s64 + -31168;
	// 832A3AD0: 4BA06451  bl 0x82ca9f20
	ctx.lr = 0x832A3AD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3AD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3AE0: 4E800020  blr
	return;
}

pub fn sub_832A3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3AE8 size=80
	// 832A3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3AF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3AF8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3AFC: 388A9BA0  addi r4, r10, -0x6460
	ctx.r[4].s64 = ctx.r[10].s64 + -25696;
	// 832A3B00: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3B08: 386A5E98  addi r3, r10, 0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + 24216;
	// 832A3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3B14: 38AB85C0  addi r5, r11, -0x7a40
	ctx.r[5].s64 = ctx.r[11].s64 + -31296;
	// 832A3B18: 4BBE1CA9  bl 0x82e857c0
	ctx.lr = 0x832A3B1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A3B1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3B20: 386B8658  addi r3, r11, -0x79a8
	ctx.r[3].s64 = ctx.r[11].s64 + -31144;
	// 832A3B24: 4BA063FD  bl 0x82ca9f20
	ctx.lr = 0x832A3B28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3B34: 4E800020  blr
	return;
}

pub fn sub_832A3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3B38 size=72
	// 832A3B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3B48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3B4C: 388B9BD4  addi r4, r11, -0x642c
	ctx.r[4].s64 = ctx.r[11].s64 + -25644;
	// 832A3B50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3B58: 386B60C0  addi r3, r11, 0x60c0
	ctx.r[3].s64 = ctx.r[11].s64 + 24768;
	// 832A3B5C: 4BBE5F7D  bl 0x82e89ad8
	ctx.lr = 0x832A3B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E89AD8);
	// 832A3B60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3B64: 386B8670  addi r3, r11, -0x7990
	ctx.r[3].s64 = ctx.r[11].s64 + -31120;
	// 832A3B68: 4BA063B9  bl 0x82ca9f20
	ctx.lr = 0x832A3B6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3B78: 4E800020  blr
	return;
}

pub fn sub_832A3B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3B80 size=72
	// 832A3B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3B8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3B90: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3B94: 388A9BE4  addi r4, r10, -0x641c
	ctx.r[4].s64 = ctx.r[10].s64 + -25628;
	// 832A3B98: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3BA0: 386A5FAC  addi r3, r10, 0x5fac
	ctx.r[3].s64 = ctx.r[10].s64 + 24492;
	// 832A3BA4: 38AB8908  addi r5, r11, -0x76f8
	ctx.r[5].s64 = ctx.r[11].s64 + -30456;
	// 832A3BA8: 4BBE5EE9  bl 0x82e89a90
	ctx.lr = 0x832A3BAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E89A90);
	// 832A3BAC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3BB0: 386B8688  addi r3, r11, -0x7978
	ctx.r[3].s64 = ctx.r[11].s64 + -31096;
	// 832A3BB4: 4BA0636D  bl 0x82ca9f20
	ctx.lr = 0x832A3BB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3BC4: 4E800020  blr
	return;
}

pub fn sub_832A3BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3BC8 size=72
	// 832A3BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3BD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3BD8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3BDC: 388A9C8C  addi r4, r10, -0x6374
	ctx.r[4].s64 = ctx.r[10].s64 + -25460;
	// 832A3BE0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3BE4: 38AB9578  addi r5, r11, -0x6a88
	ctx.r[5].s64 = ctx.r[11].s64 + -27272;
	// 832A3BE8: 386A61D4  addi r3, r10, 0x61d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25044;
	// 832A3BEC: 4BC0A7FD  bl 0x82eae3e8
	ctx.lr = 0x832A3BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EAE3E8);
	// 832A3BF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3BF4: 386B86A0  addi r3, r11, -0x7960
	ctx.r[3].s64 = ctx.r[11].s64 + -31072;
	// 832A3BF8: 4BA06329  bl 0x82ca9f20
	ctx.lr = 0x832A3BFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C08: 4E800020  blr
	return;
}

pub fn sub_832A3C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3C10 size=72
	// 832A3C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3C1C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3C20: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3C24: 388A9D00  addi r4, r10, -0x6300
	ctx.r[4].s64 = ctx.r[10].s64 + -25344;
	// 832A3C28: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3C30: 386A62EC  addi r3, r10, 0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + 25324;
	// 832A3C34: 38ABA210  addi r5, r11, -0x5df0
	ctx.r[5].s64 = ctx.r[11].s64 + -24048;
	// 832A3C38: 4BBFD899  bl 0x82ea14d0
	ctx.lr = 0x832A3C3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA14D0);
	// 832A3C3C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3C40: 386B86B8  addi r3, r11, -0x7948
	ctx.r[3].s64 = ctx.r[11].s64 + -31048;
	// 832A3C44: 4BA062DD  bl 0x82ca9f20
	ctx.lr = 0x832A3C48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C54: 4E800020  blr
	return;
}

pub fn sub_832A3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3C58 size=72
	// 832A3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3C64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3C68: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3C6C: 388A9D78  addi r4, r10, -0x6288
	ctx.r[4].s64 = ctx.r[10].s64 + -25224;
	// 832A3C70: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3C78: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 832A3C7C: 38ABAE28  addi r5, r11, -0x51d8
	ctx.r[5].s64 = ctx.r[11].s64 + -20952;
	// 832A3C80: 4BBFD851  bl 0x82ea14d0
	ctx.lr = 0x832A3C84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EA14D0);
	// 832A3C84: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3C88: 386B86D0  addi r3, r11, -0x7930
	ctx.r[3].s64 = ctx.r[11].s64 + -31024;
	// 832A3C8C: 4BA06295  bl 0x82ca9f20
	ctx.lr = 0x832A3C90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C9C: 4E800020  blr
	return;
}

pub fn sub_832A3CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3CA0 size=104
	// 832A3CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3CB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3CB4: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3CB8: 388A9DCC  addi r4, r10, -0x6234
	ctx.r[4].s64 = ctx.r[10].s64 + -25140;
	// 832A3CBC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3CC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3CC4: 3BEA6920  addi r31, r10, 0x6920
	ctx.r[31].s64 = ctx.r[10].s64 + 26912;
	// 832A3CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3CD0: 38ABAF28  addi r5, r11, -0x50d8
	ctx.r[5].s64 = ctx.r[11].s64 + -20696;
	// 832A3CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3CD8: 4BBF73E1  bl 0x82e9b0b8
	ctx.lr = 0x832A3CDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E9B0B8);
	// 832A3CDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3CE0: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3CE4: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3CE8: 386A86E8  addi r3, r10, -0x7918
	ctx.r[3].s64 = ctx.r[10].s64 + -31000;
	// 832A3CEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3CF0: 4BA06231  bl 0x82ca9f20
	ctx.lr = 0x832A3CF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3D04: 4E800020  blr
	return;
}

pub fn sub_832A3D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3D08 size=104
	// 832A3D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3D18: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3D1C: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3D20: 388A9E40  addi r4, r10, -0x61c0
	ctx.r[4].s64 = ctx.r[10].s64 + -25024;
	// 832A3D24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3D2C: 3BEA6A34  addi r31, r10, 0x6a34
	ctx.r[31].s64 = ctx.r[10].s64 + 27188;
	// 832A3D30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3D38: 38ABB860  addi r5, r11, -0x47a0
	ctx.r[5].s64 = ctx.r[11].s64 + -18336;
	// 832A3D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3D40: 4BBF7379  bl 0x82e9b0b8
	ctx.lr = 0x832A3D44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E9B0B8);
	// 832A3D44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3D48: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3D4C: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3D50: 386A8718  addi r3, r10, -0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30952;
	// 832A3D54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3D58: 4BA061C9  bl 0x82ca9f20
	ctx.lr = 0x832A3D5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3D5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3D6C: 4E800020  blr
	return;
}

pub fn sub_832A3D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3D70 size=104
	// 832A3D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3D80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3D84: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3D88: 388A9E9C  addi r4, r10, -0x6164
	ctx.r[4].s64 = ctx.r[10].s64 + -24932;
	// 832A3D8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3D90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3D94: 3BEA6B48  addi r31, r10, 0x6b48
	ctx.r[31].s64 = ctx.r[10].s64 + 27464;
	// 832A3D98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3DA0: 38ABBE98  addi r5, r11, -0x4168
	ctx.r[5].s64 = ctx.r[11].s64 + -16744;
	// 832A3DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3DA8: 4BBF7311  bl 0x82e9b0b8
	ctx.lr = 0x832A3DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E9B0B8);
	// 832A3DAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3DB0: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3DB4: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3DB8: 386A8730  addi r3, r10, -0x78d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30928;
	// 832A3DBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3DC0: 4BA06161  bl 0x82ca9f20
	ctx.lr = 0x832A3DC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3DD4: 4E800020  blr
	return;
}

pub fn sub_832A3DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3DD8 size=104
	// 832A3DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3DE8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3DEC: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3DF0: 388A9EF4  addi r4, r10, -0x610c
	ctx.r[4].s64 = ctx.r[10].s64 + -24844;
	// 832A3DF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3DF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3DFC: 3BEA6C5C  addi r31, r10, 0x6c5c
	ctx.r[31].s64 = ctx.r[10].s64 + 27740;
	// 832A3E00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3E08: 38ABC380  addi r5, r11, -0x3c80
	ctx.r[5].s64 = ctx.r[11].s64 + -15488;
	// 832A3E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3E10: 4BBF72A9  bl 0x82e9b0b8
	ctx.lr = 0x832A3E14;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E9B0B8);
	// 832A3E14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3E18: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3E1C: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3E20: 386A8748  addi r3, r10, -0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30904;
	// 832A3E24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3E28: 4BA060F9  bl 0x82ca9f20
	ctx.lr = 0x832A3E2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3E3C: 4E800020  blr
	return;
}

pub fn sub_832A3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3E40 size=80
	// 832A3E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3E4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3E50: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3E54: 388A9F6C  addi r4, r10, -0x6094
	ctx.r[4].s64 = ctx.r[10].s64 + -24724;
	// 832A3E58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3E60: 386A6D70  addi r3, r10, 0x6d70
	ctx.r[3].s64 = ctx.r[10].s64 + 28016;
	// 832A3E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3E68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3E6C: 38ABDB00  addi r5, r11, -0x2500
	ctx.r[5].s64 = ctx.r[11].s64 + -9472;
	// 832A3E70: 4BBE1951  bl 0x82e857c0
	ctx.lr = 0x832A3E74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A3E74: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3E78: 386B8760  addi r3, r11, -0x78a0
	ctx.r[3].s64 = ctx.r[11].s64 + -30880;
	// 832A3E7C: 4BA060A5  bl 0x82ca9f20
	ctx.lr = 0x832A3E80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3E8C: 4E800020  blr
	return;
}

pub fn sub_832A3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3E90 size=72
	// 832A3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3E9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3EA0: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3EA4: 388A9FC4  addi r4, r10, -0x603c
	ctx.r[4].s64 = ctx.r[10].s64 + -24636;
	// 832A3EA8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3EAC: 38ABE3F8  addi r5, r11, -0x1c08
	ctx.r[5].s64 = ctx.r[11].s64 + -7176;
	// 832A3EB0: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 832A3EB4: 4BBF492D  bl 0x82e987e0
	ctx.lr = 0x832A3EB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E987E0);
	// 832A3EB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3EBC: 386B8778  addi r3, r11, -0x7888
	ctx.r[3].s64 = ctx.r[11].s64 + -30856;
	// 832A3EC0: 4BA06061  bl 0x82ca9f20
	ctx.lr = 0x832A3EC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3ED0: 4E800020  blr
	return;
}

pub fn sub_832A3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3ED8 size=72
	// 832A3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3EE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3EE8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3EEC: 388AA030  addi r4, r10, -0x5fd0
	ctx.r[4].s64 = ctx.r[10].s64 + -24528;
	// 832A3EF0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3EF4: 38ABEB08  addi r5, r11, -0x14f8
	ctx.r[5].s64 = ctx.r[11].s64 + -5368;
	// 832A3EF8: 386A6F98  addi r3, r10, 0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + 28568;
	// 832A3EFC: 4BBF48E5  bl 0x82e987e0
	ctx.lr = 0x832A3F00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E987E0);
	// 832A3F00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F04: 386B8790  addi r3, r11, -0x7870
	ctx.r[3].s64 = ctx.r[11].s64 + -30832;
	// 832A3F08: 4BA06019  bl 0x82ca9f20
	ctx.lr = 0x832A3F0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3F18: 4E800020  blr
	return;
}

pub fn sub_832A3F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3F20 size=72
	// 832A3F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3F2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3F30: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3F34: 388AA070  addi r4, r10, -0x5f90
	ctx.r[4].s64 = ctx.r[10].s64 + -24464;
	// 832A3F38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3F3C: 38ABF7B8  addi r5, r11, -0x848
	ctx.r[5].s64 = ctx.r[11].s64 + -2120;
	// 832A3F40: 386A70AC  addi r3, r10, 0x70ac
	ctx.r[3].s64 = ctx.r[10].s64 + 28844;
	// 832A3F44: 4BBF489D  bl 0x82e987e0
	ctx.lr = 0x832A3F48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E987E0);
	// 832A3F48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F4C: 386B87A8  addi r3, r11, -0x7858
	ctx.r[3].s64 = ctx.r[11].s64 + -30808;
	// 832A3F50: 4BA05FD1  bl 0x82ca9f20
	ctx.lr = 0x832A3F54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3F60: 4E800020  blr
	return;
}

pub fn sub_832A3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3F68 size=72
	// 832A3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3F74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3F78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3F7C: 388BA1A8  addi r4, r11, -0x5e58
	ctx.r[4].s64 = ctx.r[11].s64 + -24152;
	// 832A3F80: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3F88: 386B75C8  addi r3, r11, 0x75c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30152;
	// 832A3F8C: 4BBE5B4D  bl 0x82e89ad8
	ctx.lr = 0x832A3F90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E89AD8);
	// 832A3F90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F94: 386B87D8  addi r3, r11, -0x7828
	ctx.r[3].s64 = ctx.r[11].s64 + -30760;
	// 832A3F98: 4BA05F89  bl 0x82ca9f20
	ctx.lr = 0x832A3F9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3FA8: 4E800020  blr
	return;
}

pub fn sub_832A3FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3FB0 size=72
	// 832A3FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3FBC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3FC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3FC4: 388BA1BC  addi r4, r11, -0x5e44
	ctx.r[4].s64 = ctx.r[11].s64 + -24132;
	// 832A3FC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3FD0: 386B76DC  addi r3, r11, 0x76dc
	ctx.r[3].s64 = ctx.r[11].s64 + 30428;
	// 832A3FD4: 4BBDB58D  bl 0x82e7f560
	ctx.lr = 0x832A3FD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 832A3FD8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3FDC: 386B87F0  addi r3, r11, -0x7810
	ctx.r[3].s64 = ctx.r[11].s64 + -30736;
	// 832A3FE0: 4BA05F41  bl 0x82ca9f20
	ctx.lr = 0x832A3FE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A3FE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3FF0: 4E800020  blr
	return;
}

pub fn sub_832A3FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3FF8 size=80
	// 832A3FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4004: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4008: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A400C: 388AA214  addi r4, r10, -0x5dec
	ctx.r[4].s64 = ctx.r[10].s64 + -24044;
	// 832A4010: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A4014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A4018: 386A7838  addi r3, r10, 0x7838
	ctx.r[3].s64 = ctx.r[10].s64 + 30776;
	// 832A401C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A4020: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4024: 38AB43C0  addi r5, r11, 0x43c0
	ctx.r[5].s64 = ctx.r[11].s64 + 17344;
	// 832A4028: 4BBE1799  bl 0x82e857c0
	ctx.lr = 0x832A402C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A402C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4030: 386B8808  addi r3, r11, -0x77f8
	ctx.r[3].s64 = ctx.r[11].s64 + -30712;
	// 832A4034: 4BA05EED  bl 0x82ca9f20
	ctx.lr = 0x832A4038;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A403C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4044: 4E800020  blr
	return;
}

pub fn sub_832A4048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4048 size=88
	// 832A4048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A404C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4054: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4058: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A405C: 388AA354  addi r4, r10, -0x5cac
	ctx.r[4].s64 = ctx.r[10].s64 + -23724;
	// 832A4060: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4064: 38ABBBC0  addi r5, r11, -0x4440
	ctx.r[5].s64 = ctx.r[11].s64 + -17472;
	// 832A4068: 386A8570  addi r3, r10, -0x7a90
	ctx.r[3].s64 = ctx.r[10].s64 + -31376;
	// 832A406C: 4BC063D5  bl 0x82eaa440
	ctx.lr = 0x832A4070;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EAA440);
	// 832A4070: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4074: 386B8838  addi r3, r11, -0x77c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30664;
	// 832A4078: 4BA05EA9  bl 0x82ca9f20
	ctx.lr = 0x832A407C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A407C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4088: 4E800020  blr
	return;
}

pub fn sub_832A40A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A40A0 size=12
	// 832A40A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A40A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A40A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832A40E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A40E0 size=88
	// 832A40E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A40E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A40E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A40EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A40F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A40F4: 38CB1048  addi r6, r11, 0x1048
	ctx.r[6].s64 = ctx.r[11].s64 + 4168;
	// 832A40F8: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 832A40FC: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4100: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4104: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A4108: 386A8C64  addi r3, r10, -0x739c
	ctx.r[3].s64 = ctx.r[10].s64 + -29596;
	// 832A410C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A4110: 38AB1588  addi r5, r11, 0x1588
	ctx.r[5].s64 = ctx.r[11].s64 + 5512;
	// 832A4114: 4BBE166D  bl 0x82e85780
	ctx.lr = 0x832A4118;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85780);
	// 832A4118: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A411C: 386B88C0  addi r3, r11, -0x7740
	ctx.r[3].s64 = ctx.r[11].s64 + -30528;
	// 832A4120: 4BA05E01  bl 0x82ca9f20
	ctx.lr = 0x832A4124;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4130: 4E800020  blr
	return;
}

pub fn sub_832A4138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4138 size=72
	// 832A4138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4144: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4148: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A414C: 388AA690  addi r4, r10, -0x5970
	ctx.r[4].s64 = ctx.r[10].s64 + -22896;
	// 832A4150: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4154: 38AB1190  addi r5, r11, 0x1190
	ctx.r[5].s64 = ctx.r[11].s64 + 4496;
	// 832A4158: 386A8B58  addi r3, r10, -0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29864;
	// 832A415C: 4B389FAD  bl 0x8262e108
	ctx.lr = 0x832A4160;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E108);
	// 832A4160: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4164: 386B88A8  addi r3, r11, -0x7758
	ctx.r[3].s64 = ctx.r[11].s64 + -30552;
	// 832A4168: 4BA05DB9  bl 0x82ca9f20
	ctx.lr = 0x832A416C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A416C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4178: 4E800020  blr
	return;
}

pub fn sub_832A4180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4180 size=72
	// 832A4180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A418C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4190: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4194: 388AA6B0  addi r4, r10, -0x5950
	ctx.r[4].s64 = ctx.r[10].s64 + -22864;
	// 832A4198: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A419C: 38AB4270  addi r5, r11, 0x4270
	ctx.r[5].s64 = ctx.r[11].s64 + 17008;
	// 832A41A0: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 832A41A4: 4B389DCD  bl 0x8262df70
	ctx.lr = 0x832A41A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DF70);
	// 832A41A8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A41AC: 386B8920  addi r3, r11, -0x76e0
	ctx.r[3].s64 = ctx.r[11].s64 + -30432;
	// 832A41B0: 4BA05D71  bl 0x82ca9f20
	ctx.lr = 0x832A41B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A41B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A41B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A41BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A41C0: 4E800020  blr
	return;
}

pub fn sub_832A41C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A41C8 size=72
	// 832A41C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A41CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A41D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A41D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A41D8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A41DC: 388AA6C4  addi r4, r10, -0x593c
	ctx.r[4].s64 = ctx.r[10].s64 + -22844;
	// 832A41E0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A41E4: 38AB4268  addi r5, r11, 0x4268
	ctx.r[5].s64 = ctx.r[11].s64 + 17000;
	// 832A41E8: 386A8D78  addi r3, r10, -0x7288
	ctx.r[3].s64 = ctx.r[10].s64 + -29320;
	// 832A41EC: 4B389E0D  bl 0x8262dff8
	ctx.lr = 0x832A41F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DFF8);
	// 832A41F0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A41F4: 386B8938  addi r3, r11, -0x76c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30408;
	// 832A41F8: 4BA05D29  bl 0x82ca9f20
	ctx.lr = 0x832A41FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A41FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4208: 4E800020  blr
	return;
}

pub fn sub_832A4210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4210 size=72
	// 832A4210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A421C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4220: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4224: 388AA6D4  addi r4, r10, -0x592c
	ctx.r[4].s64 = ctx.r[10].s64 + -22828;
	// 832A4228: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A422C: 38AB42E0  addi r5, r11, 0x42e0
	ctx.r[5].s64 = ctx.r[11].s64 + 17120;
	// 832A4230: 386A91A8  addi r3, r10, -0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + -28248;
	// 832A4234: 4B389DC5  bl 0x8262dff8
	ctx.lr = 0x832A4238;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DFF8);
	// 832A4238: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A423C: 386B8950  addi r3, r11, -0x76b0
	ctx.r[3].s64 = ctx.r[11].s64 + -30384;
	// 832A4240: 4BA05CE1  bl 0x82ca9f20
	ctx.lr = 0x832A4244;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A424C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4250: 4E800020  blr
	return;
}

pub fn sub_832A4258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4258 size=72
	// 832A4258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4264: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4268: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A426C: 388AA6EC  addi r4, r10, -0x5914
	ctx.r[4].s64 = ctx.r[10].s64 + -22804;
	// 832A4270: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4274: 38AB4300  addi r5, r11, 0x4300
	ctx.r[5].s64 = ctx.r[11].s64 + 17152;
	// 832A4278: 386A97F0  addi r3, r10, -0x6810
	ctx.r[3].s64 = ctx.r[10].s64 + -26640;
	// 832A427C: 4B389D7D  bl 0x8262dff8
	ctx.lr = 0x832A4280;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DFF8);
	// 832A4280: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4284: 386B8968  addi r3, r11, -0x7698
	ctx.r[3].s64 = ctx.r[11].s64 + -30360;
	// 832A4288: 4BA05C99  bl 0x82ca9f20
	ctx.lr = 0x832A428C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A428C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4298: 4E800020  blr
	return;
}

pub fn sub_832A42A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A42A0 size=72
	// 832A42A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A42A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A42A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A42AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A42B0: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A42B4: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 832A42B8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A42BC: 38AB4338  addi r5, r11, 0x4338
	ctx.r[5].s64 = ctx.r[11].s64 + 17208;
	// 832A42C0: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 832A42C4: 4B389D35  bl 0x8262dff8
	ctx.lr = 0x832A42C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DFF8);
	// 832A42C8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A42CC: 386B8980  addi r3, r11, -0x7680
	ctx.r[3].s64 = ctx.r[11].s64 + -30336;
	// 832A42D0: 4BA05C51  bl 0x82ca9f20
	ctx.lr = 0x832A42D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A42D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A42D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A42DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A42E0: 4E800020  blr
	return;
}

pub fn sub_832A42E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A42E8 size=72
	// 832A42E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A42EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A42F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A42F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A42F8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A42FC: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 832A4300: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4304: 38AB4DA0  addi r5, r11, 0x4da0
	ctx.r[5].s64 = ctx.r[11].s64 + 19872;
	// 832A4308: 386A8F90  addi r3, r10, -0x7070
	ctx.r[3].s64 = ctx.r[10].s64 + -28784;
	// 832A430C: 4BC10F15  bl 0x82eb5220
	ctx.lr = 0x832A4310;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB5220);
	// 832A4310: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4314: 386B8998  addi r3, r11, -0x7668
	ctx.r[3].s64 = ctx.r[11].s64 + -30312;
	// 832A4318: 4BA05C09  bl 0x82ca9f20
	ctx.lr = 0x832A431C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A431C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4328: 4E800020  blr
	return;
}

pub fn sub_832A4330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4330 size=72
	// 832A4330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A433C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4340: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A4344: 388AA724  addi r4, r10, -0x58dc
	ctx.r[4].s64 = ctx.r[10].s64 + -22748;
	// 832A4348: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A434C: 38AB63A8  addi r5, r11, 0x63a8
	ctx.r[5].s64 = ctx.r[11].s64 + 25512;
	// 832A4350: 386A94CC  addi r3, r10, -0x6b34
	ctx.r[3].s64 = ctx.r[10].s64 + -27444;
	// 832A4354: 4BC10ECD  bl 0x82eb5220
	ctx.lr = 0x832A4358;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB5220);
	// 832A4358: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A435C: 386B89B0  addi r3, r11, -0x7650
	ctx.r[3].s64 = ctx.r[11].s64 + -30288;
	// 832A4360: 4BA05BC1  bl 0x82ca9f20
	ctx.lr = 0x832A4364;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A436C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4370: 4E800020  blr
	return;
}

pub fn sub_832A4378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4378 size=72
	// 832A4378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4384: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4388: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A438C: 388AA730  addi r4, r10, -0x58d0
	ctx.r[4].s64 = ctx.r[10].s64 + -22736;
	// 832A4390: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4394: 38AB4DF0  addi r5, r11, 0x4df0
	ctx.r[5].s64 = ctx.r[11].s64 + 19952;
	// 832A4398: 386A92B4  addi r3, r10, -0x6d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27980;
	// 832A439C: 4BC10E85  bl 0x82eb5220
	ctx.lr = 0x832A43A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB5220);
	// 832A43A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A43A4: 386B89C8  addi r3, r11, -0x7638
	ctx.r[3].s64 = ctx.r[11].s64 + -30264;
	// 832A43A8: 4BA05B79  bl 0x82ca9f20
	ctx.lr = 0x832A43AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A43AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A43B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A43B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A43B8: 4E800020  blr
	return;
}

pub fn sub_832A43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A43C0 size=72
	// 832A43C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A43C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A43C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A43CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A43D0: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A43D4: 388AA748  addi r4, r10, -0x58b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22712;
	// 832A43D8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A43DC: 38AB4370  addi r5, r11, 0x4370
	ctx.r[5].s64 = ctx.r[11].s64 + 17264;
	// 832A43E0: 386A9B14  addi r3, r10, -0x64ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25836;
	// 832A43E4: 4B389C9D  bl 0x8262e080
	ctx.lr = 0x832A43E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E080);
	// 832A43E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A43EC: 386B89E0  addi r3, r11, -0x7620
	ctx.r[3].s64 = ctx.r[11].s64 + -30240;
	// 832A43F0: 4BA05B31  bl 0x82ca9f20
	ctx.lr = 0x832A43F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A43F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A43F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A43FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4400: 4E800020  blr
	return;
}

pub fn sub_832A4408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4408 size=72
	// 832A4408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4414: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4418: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A441C: 388AA754  addi r4, r10, -0x58ac
	ctx.r[4].s64 = ctx.r[10].s64 + -22700;
	// 832A4420: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4424: 38AB4530  addi r5, r11, 0x4530
	ctx.r[5].s64 = ctx.r[11].s64 + 17712;
	// 832A4428: 386A9A08  addi r3, r10, -0x65f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26104;
	// 832A442C: 4B389C55  bl 0x8262e080
	ctx.lr = 0x832A4430;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E080);
	// 832A4430: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4434: 386B89F8  addi r3, r11, -0x7608
	ctx.r[3].s64 = ctx.r[11].s64 + -30216;
	// 832A4438: 4BA05AE9  bl 0x82ca9f20
	ctx.lr = 0x832A443C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A443C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4448: 4E800020  blr
	return;
}

pub fn sub_832A4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4450 size=72
	// 832A4450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A445C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4460: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4464: 388AA764  addi r4, r10, -0x589c
	ctx.r[4].s64 = ctx.r[10].s64 + -22684;
	// 832A4468: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A446C: 38AB5028  addi r5, r11, 0x5028
	ctx.r[5].s64 = ctx.r[11].s64 + 20520;
	// 832A4470: 386A93C0  addi r3, r10, -0x6c40
	ctx.r[3].s64 = ctx.r[10].s64 + -27712;
	// 832A4474: 4BC10E35  bl 0x82eb52a8
	ctx.lr = 0x832A4478;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB52A8);
	// 832A4478: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A447C: 386B8A10  addi r3, r11, -0x75f0
	ctx.r[3].s64 = ctx.r[11].s64 + -30192;
	// 832A4480: 4BA05AA1  bl 0x82ca9f20
	ctx.lr = 0x832A4484;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A448C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4490: 4E800020  blr
	return;
}

pub fn sub_832A4498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4498 size=72
	// 832A4498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A449C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A44A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A44A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A44A8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A44AC: 388AA774  addi r4, r10, -0x588c
	ctx.r[4].s64 = ctx.r[10].s64 + -22668;
	// 832A44B0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A44B4: 38AB4E48  addi r5, r11, 0x4e48
	ctx.r[5].s64 = ctx.r[11].s64 + 20040;
	// 832A44B8: 386A9D2C  addi r3, r10, -0x62d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25300;
	// 832A44BC: 4BC10DED  bl 0x82eb52a8
	ctx.lr = 0x832A44C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB52A8);
	// 832A44C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A44C4: 386B8A28  addi r3, r11, -0x75d8
	ctx.r[3].s64 = ctx.r[11].s64 + -30168;
	// 832A44C8: 4BA05A59  bl 0x82ca9f20
	ctx.lr = 0x832A44CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A44CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A44D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A44D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A44D8: 4E800020  blr
	return;
}

pub fn sub_832A44E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A44E0 size=72
	// 832A44E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A44E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A44E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A44EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A44F0: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A44F4: 388AA780  addi r4, r10, -0x5880
	ctx.r[4].s64 = ctx.r[10].s64 + -22656;
	// 832A44F8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A44FC: 38AB63A8  addi r5, r11, 0x63a8
	ctx.r[5].s64 = ctx.r[11].s64 + 25512;
	// 832A4500: 386A95D8  addi r3, r10, -0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + -27176;
	// 832A4504: 4BC10DA5  bl 0x82eb52a8
	ctx.lr = 0x832A4508;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EB52A8);
	// 832A4508: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A450C: 386B8A40  addi r3, r11, -0x75c0
	ctx.r[3].s64 = ctx.r[11].s64 + -30144;
	// 832A4510: 4BA05A11  bl 0x82ca9f20
	ctx.lr = 0x832A4514;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4520: 4E800020  blr
	return;
}

pub fn sub_832A4528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4528 size=72
	// 832A4528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4534: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4538: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A453C: 388AA788  addi r4, r10, -0x5878
	ctx.r[4].s64 = ctx.r[10].s64 + -22648;
	// 832A4540: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4544: 38AB4BA0  addi r5, r11, 0x4ba0
	ctx.r[5].s64 = ctx.r[11].s64 + 19360;
	// 832A4548: 386A909C  addi r3, r10, -0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + -28516;
	// 832A454C: 4B389C45  bl 0x8262e190
	ctx.lr = 0x832A4550;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E190);
	// 832A4550: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4554: 386B8A58  addi r3, r11, -0x75a8
	ctx.r[3].s64 = ctx.r[11].s64 + -30120;
	// 832A4558: 4BA059C9  bl 0x82ca9f20
	ctx.lr = 0x832A455C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A455C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4568: 4E800020  blr
	return;
}

pub fn sub_832A4570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4570 size=72
	// 832A4570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A457C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4580: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4584: 388AA794  addi r4, r10, -0x586c
	ctx.r[4].s64 = ctx.r[10].s64 + -22636;
	// 832A4588: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A458C: 38AB4CD8  addi r5, r11, 0x4cd8
	ctx.r[5].s64 = ctx.r[11].s64 + 19672;
	// 832A4590: 386A96E4  addi r3, r10, -0x691c
	ctx.r[3].s64 = ctx.r[10].s64 + -26908;
	// 832A4594: 4B389BFD  bl 0x8262e190
	ctx.lr = 0x832A4598;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E190);
	// 832A4598: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A459C: 386B8A70  addi r3, r11, -0x7590
	ctx.r[3].s64 = ctx.r[11].s64 + -30096;
	// 832A45A0: 4BA05981  bl 0x82ca9f20
	ctx.lr = 0x832A45A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A45A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A45A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A45AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A45B0: 4E800020  blr
	return;
}

pub fn sub_832A45B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A45B8 size=72
	// 832A45B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A45BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A45C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A45C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A45C8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A45CC: 388AA7A8  addi r4, r10, -0x5858
	ctx.r[4].s64 = ctx.r[10].s64 + -22616;
	// 832A45D0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A45D4: 38AB4D30  addi r5, r11, 0x4d30
	ctx.r[5].s64 = ctx.r[11].s64 + 19760;
	// 832A45D8: 386A9C20  addi r3, r10, -0x63e0
	ctx.r[3].s64 = ctx.r[10].s64 + -25568;
	// 832A45DC: 4B389BB5  bl 0x8262e190
	ctx.lr = 0x832A45E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E190);
	// 832A45E0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A45E4: 386B8A88  addi r3, r11, -0x7578
	ctx.r[3].s64 = ctx.r[11].s64 + -30072;
	// 832A45E8: 4BA05939  bl 0x82ca9f20
	ctx.lr = 0x832A45EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A45EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A45F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A45F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A45F8: 4E800020  blr
	return;
}

pub fn sub_832A4600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4600 size=88
	// 832A4600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A460C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4610: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4614: 38CB1048  addi r6, r11, 0x1048
	ctx.r[6].s64 = ctx.r[11].s64 + 4168;
	// 832A4618: 388AACF0  addi r4, r10, -0x5310
	ctx.r[4].s64 = ctx.r[10].s64 + -21264;
	// 832A461C: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4620: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A4628: 386A9E5C  addi r3, r10, -0x61a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24996;
	// 832A462C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A4630: 38ABBF80  addi r5, r11, -0x4080
	ctx.r[5].s64 = ctx.r[11].s64 + -16512;
	// 832A4634: 4BBE114D  bl 0x82e85780
	ctx.lr = 0x832A4638;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85780);
	// 832A4638: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A463C: 386B8AA0  addi r3, r11, -0x7560
	ctx.r[3].s64 = ctx.r[11].s64 + -30048;
	// 832A4640: 4BA058E1  bl 0x82ca9f20
	ctx.lr = 0x832A4644;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A464C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4650: 4E800020  blr
	return;
}

pub fn sub_832A4658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4658 size=72
	// 832A4658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4664: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4668: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A466C: 388AAD0C  addi r4, r10, -0x52f4
	ctx.r[4].s64 = ctx.r[10].s64 + -21236;
	// 832A4670: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4678: 386ACBC8  addi r3, r10, -0x3438
	ctx.r[3].s64 = ctx.r[10].s64 + -13368;
	// 832A467C: 38ABDCB0  addi r5, r11, -0x2350
	ctx.r[5].s64 = ctx.r[11].s64 + -9040;
	// 832A4680: 4BC21011  bl 0x82ec5690
	ctx.lr = 0x832A4684;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5690);
	// 832A4684: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4688: 386B8BC0  addi r3, r11, -0x7440
	ctx.r[3].s64 = ctx.r[11].s64 + -29760;
	// 832A468C: 4BA05895  bl 0x82ca9f20
	ctx.lr = 0x832A4690;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A469C: 4E800020  blr
	return;
}

pub fn sub_832A46A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A46A0 size=72
	// 832A46A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A46A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A46A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A46AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A46B0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A46B4: 388AAD58  addi r4, r10, -0x52a8
	ctx.r[4].s64 = ctx.r[10].s64 + -21160;
	// 832A46B8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A46BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A46C0: 386ACCDC  addi r3, r10, -0x3324
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	// 832A46C4: 38ABDFB8  addi r5, r11, -0x2048
	ctx.r[5].s64 = ctx.r[11].s64 + -8264;
	// 832A46C8: 4BC211A1  bl 0x82ec5868
	ctx.lr = 0x832A46CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5868);
	// 832A46CC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A46D0: 386B8BD8  addi r3, r11, -0x7428
	ctx.r[3].s64 = ctx.r[11].s64 + -29736;
	// 832A46D4: 4BA0584D  bl 0x82ca9f20
	ctx.lr = 0x832A46D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A46D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A46DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A46E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A46E4: 4E800020  blr
	return;
}

pub fn sub_832A46E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A46E8 size=72
	// 832A46E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A46EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A46F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A46F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A46F8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A46FC: 388AAD88  addi r4, r10, -0x5278
	ctx.r[4].s64 = ctx.r[10].s64 + -21112;
	// 832A4700: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4708: 386ACDF0  addi r3, r10, -0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + -12816;
	// 832A470C: 38ABE1C0  addi r5, r11, -0x1e40
	ctx.r[5].s64 = ctx.r[11].s64 + -7744;
	// 832A4710: 4BC21159  bl 0x82ec5868
	ctx.lr = 0x832A4714;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5868);
	// 832A4714: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4718: 386B8BF0  addi r3, r11, -0x7410
	ctx.r[3].s64 = ctx.r[11].s64 + -29712;
	// 832A471C: 4BA05805  bl 0x82ca9f20
	ctx.lr = 0x832A4720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A472C: 4E800020  blr
	return;
}

pub fn sub_832A4730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4730 size=72
	// 832A4730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A473C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4740: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4744: 388AADC0  addi r4, r10, -0x5240
	ctx.r[4].s64 = ctx.r[10].s64 + -21056;
	// 832A4748: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4750: 386ACF04  addi r3, r10, -0x30fc
	ctx.r[3].s64 = ctx.r[10].s64 + -12540;
	// 832A4754: 38ABE6E0  addi r5, r11, -0x1920
	ctx.r[5].s64 = ctx.r[11].s64 + -6432;
	// 832A4758: 4BC212E9  bl 0x82ec5a40
	ctx.lr = 0x832A475C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5A40);
	// 832A475C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4760: 386B8C08  addi r3, r11, -0x73f8
	ctx.r[3].s64 = ctx.r[11].s64 + -29688;
	// 832A4764: 4BA057BD  bl 0x82ca9f20
	ctx.lr = 0x832A4768;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A476C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4774: 4E800020  blr
	return;
}

pub fn sub_832A4778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4778 size=72
	// 832A4778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A477C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4784: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4788: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A478C: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 832A4790: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4798: 386AD018  addi r3, r10, -0x2fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -12264;
	// 832A479C: 38ABEC60  addi r5, r11, -0x13a0
	ctx.r[5].s64 = ctx.r[11].s64 + -5024;
	// 832A47A0: 4BC212A1  bl 0x82ec5a40
	ctx.lr = 0x832A47A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5A40);
	// 832A47A4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A47A8: 386B8C20  addi r3, r11, -0x73e0
	ctx.r[3].s64 = ctx.r[11].s64 + -29664;
	// 832A47AC: 4BA05775  bl 0x82ca9f20
	ctx.lr = 0x832A47B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A47B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A47B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A47B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A47BC: 4E800020  blr
	return;
}

pub fn sub_832A47C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A47C0 size=72
	// 832A47C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A47C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A47C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A47CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A47D0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A47D4: 388AAEC8  addi r4, r10, -0x5138
	ctx.r[4].s64 = ctx.r[10].s64 + -20792;
	// 832A47D8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A47DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A47E0: 386AD12C  addi r3, r10, -0x2ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -11988;
	// 832A47E4: 38ABF928  addi r5, r11, -0x6d8
	ctx.r[5].s64 = ctx.r[11].s64 + -1752;
	// 832A47E8: 4BC21431  bl 0x82ec5c18
	ctx.lr = 0x832A47EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5C18);
	// 832A47EC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A47F0: 386B8C38  addi r3, r11, -0x73c8
	ctx.r[3].s64 = ctx.r[11].s64 + -29640;
	// 832A47F4: 4BA0572D  bl 0x82ca9f20
	ctx.lr = 0x832A47F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A47F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A47FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4804: 4E800020  blr
	return;
}

pub fn sub_832A4808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832A4808 size=128
	// 832A4808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4814: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4818: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A481C: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 832A4820: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4828: 386AD244  addi r3, r10, -0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11708;
	// 832A482C: 38ABFDF8  addi r5, r11, -0x208
	ctx.r[5].s64 = ctx.r[11].s64 + -520;
	// 832A4830: 4BC213E9  bl 0x82ec5c18
	ctx.lr = 0x832A4834;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5C18);
	// 832A4834: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4838: 386B8C50  addi r3, r11, -0x73b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29616;
	// 832A483C: 4BA056E5  bl 0x82ca9f20
	ctx.lr = 0x832A4840;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A484C: 4E800020  blr
	return;
	// 832A4850: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4854: 816B0718  lwz r11, 0x718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) } as u64;
	// 832A4858: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832A485C: 4182000C  beq 0x832a4868
	if ctx.cr[0].eq {
	pc = 0x832A4868; continue 'dispatch;
	}
	// 832A4860: C1AB000C  lfs f13, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832A4864: 4800000C  b 0x832a4870
	pc = 0x832A4870; continue 'dispatch;
	// 832A4868: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832A486C: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832A4870: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832A4874: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832A4878: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832A487C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 832A4880: D00BD240  stfs f0, -0x2dc0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-11712 as u32), tmp.u32 ) };
	// 832A4884: 4E800020  blr
	return;
}

pub fn sub_832A4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4888 size=72
	// 832A4888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A488C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4894: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4898: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A489C: 388AAF5C  addi r4, r10, -0x50a4
	ctx.r[4].s64 = ctx.r[10].s64 + -20644;
	// 832A48A0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A48A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A48A8: 386AD358  addi r3, r10, -0x2ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -11432;
	// 832A48AC: 38AB0B38  addi r5, r11, 0xb38
	ctx.r[5].s64 = ctx.r[11].s64 + 2872;
	// 832A48B0: 4BC21369  bl 0x82ec5c18
	ctx.lr = 0x832A48B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5C18);
	// 832A48B4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A48B8: 386B8C68  addi r3, r11, -0x7398
	ctx.r[3].s64 = ctx.r[11].s64 + -29592;
	// 832A48BC: 4BA05665  bl 0x82ca9f20
	ctx.lr = 0x832A48C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A48C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A48C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A48C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A48CC: 4E800020  blr
	return;
}

pub fn sub_832A48D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A48D0 size=72
	// 832A48D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A48D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A48D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A48DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A48E0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A48E4: 388AAFA0  addi r4, r10, -0x5060
	ctx.r[4].s64 = ctx.r[10].s64 + -20576;
	// 832A48E8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A48EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A48F0: 386AD47C  addi r3, r10, -0x2b84
	ctx.r[3].s64 = ctx.r[10].s64 + -11140;
	// 832A48F4: 38AB0BC8  addi r5, r11, 0xbc8
	ctx.r[5].s64 = ctx.r[11].s64 + 3016;
	// 832A48F8: 4BC214F9  bl 0x82ec5df0
	ctx.lr = 0x832A48FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5DF0);
	// 832A48FC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4900: 386B8C80  addi r3, r11, -0x7380
	ctx.r[3].s64 = ctx.r[11].s64 + -29568;
	// 832A4904: 4BA0561D  bl 0x82ca9f20
	ctx.lr = 0x832A4908;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4914: 4E800020  blr
	return;
}

pub fn sub_832A4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4918 size=72
	// 832A4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4924: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4928: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A492C: 388AAFC4  addi r4, r10, -0x503c
	ctx.r[4].s64 = ctx.r[10].s64 + -20540;
	// 832A4930: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4938: 386AD590  addi r3, r10, -0x2a70
	ctx.r[3].s64 = ctx.r[10].s64 + -10864;
	// 832A493C: 38AB0D28  addi r5, r11, 0xd28
	ctx.r[5].s64 = ctx.r[11].s64 + 3368;
	// 832A4940: 4BC214B1  bl 0x82ec5df0
	ctx.lr = 0x832A4944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5DF0);
	// 832A4944: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4948: 386B8C98  addi r3, r11, -0x7368
	ctx.r[3].s64 = ctx.r[11].s64 + -29544;
	// 832A494C: 4BA055D5  bl 0x82ca9f20
	ctx.lr = 0x832A4950;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A495C: 4E800020  blr
	return;
}

pub fn sub_832A4960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4960 size=72
	// 832A4960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A496C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4970: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4974: 388AAFD8  addi r4, r10, -0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + -20520;
	// 832A4978: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A497C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4980: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 832A4984: 38AB1640  addi r5, r11, 0x1640
	ctx.r[5].s64 = ctx.r[11].s64 + 5696;
	// 832A4988: 4BC21641  bl 0x82ec5fc8
	ctx.lr = 0x832A498C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5FC8);
	// 832A498C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4990: 386B8CB0  addi r3, r11, -0x7350
	ctx.r[3].s64 = ctx.r[11].s64 + -29520;
	// 832A4994: 4BA0558D  bl 0x82ca9f20
	ctx.lr = 0x832A4998;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A49A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A49A4: 4E800020  blr
	return;
}

pub fn sub_832A49A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A49A8 size=72
	// 832A49A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A49AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A49B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A49B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A49B8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A49BC: 388AB018  addi r4, r10, -0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + -20456;
	// 832A49C0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A49C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A49C8: 386AD7B8  addi r3, r10, -0x2848
	ctx.r[3].s64 = ctx.r[10].s64 + -10312;
	// 832A49CC: 38AB1960  addi r5, r11, 0x1960
	ctx.r[5].s64 = ctx.r[11].s64 + 6496;
	// 832A49D0: 4BC215F9  bl 0x82ec5fc8
	ctx.lr = 0x832A49D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5FC8);
	// 832A49D4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A49D8: 386B8CC8  addi r3, r11, -0x7338
	ctx.r[3].s64 = ctx.r[11].s64 + -29496;
	// 832A49DC: 4BA05545  bl 0x82ca9f20
	ctx.lr = 0x832A49E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A49E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A49E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A49E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A49EC: 4E800020  blr
	return;
}

pub fn sub_832A49F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A49F0 size=72
	// 832A49F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A49F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A49F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A49FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A00: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A04: 388AB068  addi r4, r10, -0x4f98
	ctx.r[4].s64 = ctx.r[10].s64 + -20376;
	// 832A4A08: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4A10: 386AD8CC  addi r3, r10, -0x2734
	ctx.r[3].s64 = ctx.r[10].s64 + -10036;
	// 832A4A14: 38AB1B00  addi r5, r11, 0x1b00
	ctx.r[5].s64 = ctx.r[11].s64 + 6912;
	// 832A4A18: 4BC21789  bl 0x82ec61a0
	ctx.lr = 0x832A4A1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC61A0);
	// 832A4A1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4A20: 386B8CE0  addi r3, r11, -0x7320
	ctx.r[3].s64 = ctx.r[11].s64 + -29472;
	// 832A4A24: 4BA054FD  bl 0x82ca9f20
	ctx.lr = 0x832A4A28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4A34: 4E800020  blr
	return;
}

pub fn sub_832A4A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4A38 size=72
	// 832A4A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4A44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A48: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A4C: 388AB0A4  addi r4, r10, -0x4f5c
	ctx.r[4].s64 = ctx.r[10].s64 + -20316;
	// 832A4A50: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4A58: 386AD9E0  addi r3, r10, -0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + -9760;
	// 832A4A5C: 38AB1F28  addi r5, r11, 0x1f28
	ctx.r[5].s64 = ctx.r[11].s64 + 7976;
	// 832A4A60: 4BC21919  bl 0x82ec6378
	ctx.lr = 0x832A4A64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6378);
	// 832A4A64: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4A68: 386B8CF8  addi r3, r11, -0x7308
	ctx.r[3].s64 = ctx.r[11].s64 + -29448;
	// 832A4A6C: 4BA054B5  bl 0x82ca9f20
	ctx.lr = 0x832A4A70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4A7C: 4E800020  blr
	return;
}

pub fn sub_832A4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4A80 size=72
	// 832A4A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4A8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A90: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A94: 388AB0C4  addi r4, r10, -0x4f3c
	ctx.r[4].s64 = ctx.r[10].s64 + -20284;
	// 832A4A98: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4AA0: 386ADAF4  addi r3, r10, -0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	// 832A4AA4: 38AB2348  addi r5, r11, 0x2348
	ctx.r[5].s64 = ctx.r[11].s64 + 9032;
	// 832A4AA8: 4BC21AA9  bl 0x82ec6550
	ctx.lr = 0x832A4AAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6550);
	// 832A4AAC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4AB0: 386B8D10  addi r3, r11, -0x72f0
	ctx.r[3].s64 = ctx.r[11].s64 + -29424;
	// 832A4AB4: 4BA0546D  bl 0x82ca9f20
	ctx.lr = 0x832A4AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4AC4: 4E800020  blr
	return;
}

pub fn sub_832A4AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4AC8 size=80
	// 832A4AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4AD4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832A4AD8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4ADC: 38CBDD1C  addi r6, r11, -0x22e4
	ctx.r[6].s64 = ctx.r[11].s64 + -8932;
	// 832A4AE0: 388AB100  addi r4, r10, -0x4f00
	ctx.r[4].s64 = ctx.r[10].s64 + -20224;
	// 832A4AE4: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4AE8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4AEC: 38AB2428  addi r5, r11, 0x2428
	ctx.r[5].s64 = ctx.r[11].s64 + 9256;
	// 832A4AF0: 386ADC08  addi r3, r10, -0x23f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9208;
	// 832A4AF4: 4BC21C35  bl 0x82ec6728
	ctx.lr = 0x832A4AF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6728);
	// 832A4AF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4AFC: 386B8D28  addi r3, r11, -0x72d8
	ctx.r[3].s64 = ctx.r[11].s64 + -29400;
	// 832A4B00: 4BA05421  bl 0x82ca9f20
	ctx.lr = 0x832A4B04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4B04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4B10: 4E800020  blr
	return;
}

pub fn sub_832A4B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4B18 size=72
	// 832A4B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4B24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4B28: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4B2C: 388AB124  addi r4, r10, -0x4edc
	ctx.r[4].s64 = ctx.r[10].s64 + -20188;
	// 832A4B30: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4B38: 386ADD1C  addi r3, r10, -0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8932;
	// 832A4B3C: 38AB2708  addi r5, r11, 0x2708
	ctx.r[5].s64 = ctx.r[11].s64 + 9992;
	// 832A4B40: 4BC21BE9  bl 0x82ec6728
	ctx.lr = 0x832A4B44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6728);
	// 832A4B44: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4B48: 386B8D40  addi r3, r11, -0x72c0
	ctx.r[3].s64 = ctx.r[11].s64 + -29376;
	// 832A4B4C: 4BA053D5  bl 0x82ca9f20
	ctx.lr = 0x832A4B50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4B5C: 4E800020  blr
	return;
}

pub fn sub_832A4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4B60 size=72
	// 832A4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4B6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4B70: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4B74: 388AB160  addi r4, r10, -0x4ea0
	ctx.r[4].s64 = ctx.r[10].s64 + -20128;
	// 832A4B78: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4B80: 386ADE30  addi r3, r10, -0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8656;
	// 832A4B84: 38AB2978  addi r5, r11, 0x2978
	ctx.r[5].s64 = ctx.r[11].s64 + 10616;
	// 832A4B88: 4BC21BA1  bl 0x82ec6728
	ctx.lr = 0x832A4B8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6728);
	// 832A4B8C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4B90: 386B8D58  addi r3, r11, -0x72a8
	ctx.r[3].s64 = ctx.r[11].s64 + -29352;
	// 832A4B94: 4BA0538D  bl 0x82ca9f20
	ctx.lr = 0x832A4B98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4BA4: 4E800020  blr
	return;
}

pub fn sub_832A4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4BA8 size=72
	// 832A4BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4BB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4BB8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4BBC: 388AB1A4  addi r4, r10, -0x4e5c
	ctx.r[4].s64 = ctx.r[10].s64 + -20060;
	// 832A4BC0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4BC8: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 832A4BCC: 38AB3030  addi r5, r11, 0x3030
	ctx.r[5].s64 = ctx.r[11].s64 + 12336;
	// 832A4BD0: 4BC21D31  bl 0x82ec6900
	ctx.lr = 0x832A4BD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6900);
	// 832A4BD4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4BD8: 386B8D70  addi r3, r11, -0x7290
	ctx.r[3].s64 = ctx.r[11].s64 + -29328;
	// 832A4BDC: 4BA05345  bl 0x82ca9f20
	ctx.lr = 0x832A4BE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4BEC: 4E800020  blr
	return;
}

pub fn sub_832A4BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4BF0 size=72
	// 832A4BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4BFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C00: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4C04: 388AB1C8  addi r4, r10, -0x4e38
	ctx.r[4].s64 = ctx.r[10].s64 + -20024;
	// 832A4C08: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4C10: 386AE058  addi r3, r10, -0x1fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -8104;
	// 832A4C14: 38AB3130  addi r5, r11, 0x3130
	ctx.r[5].s64 = ctx.r[11].s64 + 12592;
	// 832A4C18: 4BC21CE9  bl 0x82ec6900
	ctx.lr = 0x832A4C1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC6900);
	// 832A4C1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4C20: 386B8D88  addi r3, r11, -0x7278
	ctx.r[3].s64 = ctx.r[11].s64 + -29304;
	// 832A4C24: 4BA052FD  bl 0x82ca9f20
	ctx.lr = 0x832A4C28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4C34: 4E800020  blr
	return;
}

pub fn sub_832A4C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4C38 size=72
	// 832A4C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4C44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C48: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4C4C: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 832A4C50: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4C54: 38AB3808  addi r5, r11, 0x3808
	ctx.r[5].s64 = ctx.r[11].s64 + 14344;
	// 832A4C58: 386AE16C  addi r3, r10, -0x1e94
	ctx.r[3].s64 = ctx.r[10].s64 + -7828;
	// 832A4C5C: 4BC3B1C5  bl 0x82edfe20
	ctx.lr = 0x832A4C60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EDFE20);
	// 832A4C60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4C64: 386B8DA0  addi r3, r11, -0x7260
	ctx.r[3].s64 = ctx.r[11].s64 + -29280;
	// 832A4C68: 4BA052B9  bl 0x82ca9f20
	ctx.lr = 0x832A4C6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4C6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4C78: 4E800020  blr
	return;
}

pub fn sub_832A4C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4C80 size=80
	// 832A4C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4C8C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4C90: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C94: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4C98: 388AB2DC  addi r4, r10, -0x4d24
	ctx.r[4].s64 = ctx.r[10].s64 + -19748;
	// 832A4C9C: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4CA0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4CA4: 38AB7D08  addi r5, r11, 0x7d08
	ctx.r[5].s64 = ctx.r[11].s64 + 32008;
	// 832A4CA8: 386AE284  addi r3, r10, -0x1d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7548;
	// 832A4CAC: 4BBE2295  bl 0x82e86f40
	ctx.lr = 0x832A4CB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4CB0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4CB4: 386B8DB8  addi r3, r11, -0x7248
	ctx.r[3].s64 = ctx.r[11].s64 + -29256;
	// 832A4CB8: 4BA05269  bl 0x82ca9f20
	ctx.lr = 0x832A4CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4CC8: 4E800020  blr
	return;
}

pub fn sub_832A4CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4CD0 size=80
	// 832A4CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4CDC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4CE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4CE4: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4CE8: 388AB330  addi r4, r10, -0x4cd0
	ctx.r[4].s64 = ctx.r[10].s64 + -19664;
	// 832A4CEC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4CF0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4CF4: 38AB8060  addi r5, r11, -0x7fa0
	ctx.r[5].s64 = ctx.r[11].s64 + -32672;
	// 832A4CF8: 386AE398  addi r3, r10, -0x1c68
	ctx.r[3].s64 = ctx.r[10].s64 + -7272;
	// 832A4CFC: 4BBE2245  bl 0x82e86f40
	ctx.lr = 0x832A4D00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4D00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4D04: 386B8DD0  addi r3, r11, -0x7230
	ctx.r[3].s64 = ctx.r[11].s64 + -29232;
	// 832A4D08: 4BA05219  bl 0x82ca9f20
	ctx.lr = 0x832A4D0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4D18: 4E800020  blr
	return;
}

pub fn sub_832A4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4D20 size=80
	// 832A4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4D2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4D30: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4D34: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 832A4D38: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A4D40: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 832A4D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A4D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4D4C: 38AB8D98  addi r5, r11, -0x7268
	ctx.r[5].s64 = ctx.r[11].s64 + -29288;
	// 832A4D50: 4BBE0A71  bl 0x82e857c0
	ctx.lr = 0x832A4D54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A4D54: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4D58: 386B8DE8  addi r3, r11, -0x7218
	ctx.r[3].s64 = ctx.r[11].s64 + -29208;
	// 832A4D5C: 4BA051C5  bl 0x82ca9f20
	ctx.lr = 0x832A4D60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4D6C: 4E800020  blr
	return;
}

pub fn sub_832A4D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4D70 size=80
	// 832A4D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4D7C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4D80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4D84: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4D88: 388AB3EC  addi r4, r10, -0x4c14
	ctx.r[4].s64 = ctx.r[10].s64 + -19476;
	// 832A4D8C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4D90: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4D94: 38AB8ED8  addi r5, r11, -0x7128
	ctx.r[5].s64 = ctx.r[11].s64 + -28968;
	// 832A4D98: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 832A4D9C: 4BBE21A5  bl 0x82e86f40
	ctx.lr = 0x832A4DA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4DA0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4DA4: 386B8E00  addi r3, r11, -0x7200
	ctx.r[3].s64 = ctx.r[11].s64 + -29184;
	// 832A4DA8: 4BA05179  bl 0x82ca9f20
	ctx.lr = 0x832A4DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4DB8: 4E800020  blr
	return;
}

pub fn sub_832A4DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4DC0 size=80
	// 832A4DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4DCC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4DD0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4DD4: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4DD8: 388AB4AC  addi r4, r10, -0x4b54
	ctx.r[4].s64 = ctx.r[10].s64 + -19284;
	// 832A4DDC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4DE0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4DE4: 38ABA100  addi r5, r11, -0x5f00
	ctx.r[5].s64 = ctx.r[11].s64 + -24320;
	// 832A4DE8: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 832A4DEC: 4BBE2155  bl 0x82e86f40
	ctx.lr = 0x832A4DF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4DF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4DF4: 386B8E18  addi r3, r11, -0x71e8
	ctx.r[3].s64 = ctx.r[11].s64 + -29160;
	// 832A4DF8: 4BA05129  bl 0x82ca9f20
	ctx.lr = 0x832A4DFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4E08: 4E800020  blr
	return;
}

pub fn sub_832A4E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4E10 size=80
	// 832A4E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4E1C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4E20: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4E24: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4E28: 388AB534  addi r4, r10, -0x4acc
	ctx.r[4].s64 = ctx.r[10].s64 + -19148;
	// 832A4E2C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4E30: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4E34: 38ABA410  addi r5, r11, -0x5bf0
	ctx.r[5].s64 = ctx.r[11].s64 + -23536;
	// 832A4E38: 386AE7E8  addi r3, r10, -0x1818
	ctx.r[3].s64 = ctx.r[10].s64 + -6168;
	// 832A4E3C: 4BBE2105  bl 0x82e86f40
	ctx.lr = 0x832A4E40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4E40: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4E44: 386B8E30  addi r3, r11, -0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29136;
	// 832A4E48: 4BA050D9  bl 0x82ca9f20
	ctx.lr = 0x832A4E4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4E58: 4E800020  blr
	return;
}

pub fn sub_832A4E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4E60 size=80
	// 832A4E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4E6C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4E70: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4E74: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4E78: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 832A4E7C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4E80: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4E84: 38ABADE8  addi r5, r11, -0x5218
	ctx.r[5].s64 = ctx.r[11].s64 + -21016;
	// 832A4E88: 386AE8FC  addi r3, r10, -0x1704
	ctx.r[3].s64 = ctx.r[10].s64 + -5892;
	// 832A4E8C: 4BBE20B5  bl 0x82e86f40
	ctx.lr = 0x832A4E90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4E90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4E94: 386B8E48  addi r3, r11, -0x71b8
	ctx.r[3].s64 = ctx.r[11].s64 + -29112;
	// 832A4E98: 4BA05089  bl 0x82ca9f20
	ctx.lr = 0x832A4E9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4EA8: 4E800020  blr
	return;
}

pub fn sub_832A4EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4EB0 size=80
	// 832A4EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4EBC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4EC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4EC4: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4EC8: 388AB600  addi r4, r10, -0x4a00
	ctx.r[4].s64 = ctx.r[10].s64 + -18944;
	// 832A4ECC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4ED0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4ED4: 38ABB110  addi r5, r11, -0x4ef0
	ctx.r[5].s64 = ctx.r[11].s64 + -20208;
	// 832A4ED8: 386AEA10  addi r3, r10, -0x15f0
	ctx.r[3].s64 = ctx.r[10].s64 + -5616;
	// 832A4EDC: 4BBE2065  bl 0x82e86f40
	ctx.lr = 0x832A4EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4EE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4EE4: 386B8E60  addi r3, r11, -0x71a0
	ctx.r[3].s64 = ctx.r[11].s64 + -29088;
	// 832A4EE8: 4BA05039  bl 0x82ca9f20
	ctx.lr = 0x832A4EEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4EF8: 4E800020  blr
	return;
}

pub fn sub_832A4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4F00 size=80
	// 832A4F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4F0C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4F10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4F14: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4F18: 388AB658  addi r4, r10, -0x49a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18856;
	// 832A4F1C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4F20: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4F24: 38ABB358  addi r5, r11, -0x4ca8
	ctx.r[5].s64 = ctx.r[11].s64 + -19624;
	// 832A4F28: 386AEB24  addi r3, r10, -0x14dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5340;
	// 832A4F2C: 4BBE2015  bl 0x82e86f40
	ctx.lr = 0x832A4F30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4F30: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4F34: 386B8E78  addi r3, r11, -0x7188
	ctx.r[3].s64 = ctx.r[11].s64 + -29064;
	// 832A4F38: 4BA04FE9  bl 0x82ca9f20
	ctx.lr = 0x832A4F3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4F48: 4E800020  blr
	return;
}

pub fn sub_832A4F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4F50 size=80
	// 832A4F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4F5C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4F60: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4F64: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4F68: 388AB6B4  addi r4, r10, -0x494c
	ctx.r[4].s64 = ctx.r[10].s64 + -18764;
	// 832A4F6C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4F70: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4F74: 38ABB7D0  addi r5, r11, -0x4830
	ctx.r[5].s64 = ctx.r[11].s64 + -18480;
	// 832A4F78: 386AEC38  addi r3, r10, -0x13c8
	ctx.r[3].s64 = ctx.r[10].s64 + -5064;
	// 832A4F7C: 4BBE1FC5  bl 0x82e86f40
	ctx.lr = 0x832A4F80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4F80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4F84: 386B8E90  addi r3, r11, -0x7170
	ctx.r[3].s64 = ctx.r[11].s64 + -29040;
	// 832A4F88: 4BA04F99  bl 0x82ca9f20
	ctx.lr = 0x832A4F8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4F98: 4E800020  blr
	return;
}

pub fn sub_832A4FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4FA0 size=80
	// 832A4FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4FAC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4FB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4FB4: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4FB8: 388AB710  addi r4, r10, -0x48f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18672;
	// 832A4FBC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4FC0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4FC4: 38ABBAC8  addi r5, r11, -0x4538
	ctx.r[5].s64 = ctx.r[11].s64 + -17720;
	// 832A4FC8: 386AED4C  addi r3, r10, -0x12b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4788;
	// 832A4FCC: 4BBE1F75  bl 0x82e86f40
	ctx.lr = 0x832A4FD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A4FD0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4FD4: 386B8EA8  addi r3, r11, -0x7158
	ctx.r[3].s64 = ctx.r[11].s64 + -29016;
	// 832A4FD8: 4BA04F49  bl 0x82ca9f20
	ctx.lr = 0x832A4FDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A4FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4FE8: 4E800020  blr
	return;
}

pub fn sub_832A4FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4FF0 size=80
	// 832A4FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4FFC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A5000: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5004: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A5008: 388AB768  addi r4, r10, -0x4898
	ctx.r[4].s64 = ctx.r[10].s64 + -18584;
	// 832A500C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A5010: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A5014: 38ABBDD0  addi r5, r11, -0x4230
	ctx.r[5].s64 = ctx.r[11].s64 + -16944;
	// 832A5018: 386AEE60  addi r3, r10, -0x11a0
	ctx.r[3].s64 = ctx.r[10].s64 + -4512;
	// 832A501C: 4BBE1F25  bl 0x82e86f40
	ctx.lr = 0x832A5020;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E86F40);
	// 832A5020: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5024: 386B8EC0  addi r3, r11, -0x7140
	ctx.r[3].s64 = ctx.r[11].s64 + -28992;
	// 832A5028: 4BA04EF9  bl 0x82ca9f20
	ctx.lr = 0x832A502C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A502C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5038: 4E800020  blr
	return;
}

pub fn sub_832A5040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5040 size=80
	// 832A5040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A504C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5050: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A5054: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 832A5058: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A505C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A5060: 386AEF78  addi r3, r10, -0x1088
	ctx.r[3].s64 = ctx.r[10].s64 + -4232;
	// 832A5064: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A5068: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A506C: 38ABC5A0  addi r5, r11, -0x3a60
	ctx.r[5].s64 = ctx.r[11].s64 + -14944;
	// 832A5070: 4BBE0751  bl 0x82e857c0
	ctx.lr = 0x832A5074;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A5074: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5078: 386B8ED8  addi r3, r11, -0x7128
	ctx.r[3].s64 = ctx.r[11].s64 + -28968;
	// 832A507C: 4BA04EA5  bl 0x82ca9f20
	ctx.lr = 0x832A5080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A508C: 4E800020  blr
	return;
}

pub fn sub_832A5090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5090 size=80
	// 832A5090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A509C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A50A0: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A50A4: 388ABA58  addi r4, r10, -0x45a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17832;
	// 832A50A8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A50AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A50B0: 386AF090  addi r3, r10, -0xf70
	ctx.r[3].s64 = ctx.r[10].s64 + -3952;
	// 832A50B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A50B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A50BC: 38AB0BA0  addi r5, r11, 0xba0
	ctx.r[5].s64 = ctx.r[11].s64 + 2976;
	// 832A50C0: 4BBE0701  bl 0x82e857c0
	ctx.lr = 0x832A50C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A50C4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A50C8: 386B8EF0  addi r3, r11, -0x7110
	ctx.r[3].s64 = ctx.r[11].s64 + -28944;
	// 832A50CC: 4BA04E55  bl 0x82ca9f20
	ctx.lr = 0x832A50D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A50D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A50D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A50D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A50DC: 4E800020  blr
	return;
}

pub fn sub_832A50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A50E0 size=72
	// 832A50E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A50E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A50E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A50EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A50F0: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A50F4: 388ABC60  addi r4, r10, -0x43a0
	ctx.r[4].s64 = ctx.r[10].s64 + -17312;
	// 832A50F8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A50FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A5100: 386A89BC  addi r3, r10, -0x7644
	ctx.r[3].s64 = ctx.r[10].s64 + -30276;
	// 832A5104: 38ABB1F8  addi r5, r11, -0x4e08
	ctx.r[5].s64 = ctx.r[11].s64 + -19976;
	// 832A5108: 4BC20939  bl 0x82ec5a40
	ctx.lr = 0x832A510C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5A40);
	// 832A510C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5110: 386B8F08  addi r3, r11, -0x70f8
	ctx.r[3].s64 = ctx.r[11].s64 + -28920;
	// 832A5114: 4BA04E0D  bl 0x82ca9f20
	ctx.lr = 0x832A5118;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A511C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5124: 4E800020  blr
	return;
}

pub fn sub_832A5128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5128 size=72
	// 832A5128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5134: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5138: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A513C: 388ABD28  addi r4, r10, -0x42d8
	ctx.r[4].s64 = ctx.r[10].s64 + -17112;
	// 832A5140: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A5148: 386A8AD0  addi r3, r10, -0x7530
	ctx.r[3].s64 = ctx.r[10].s64 + -30000;
	// 832A514C: 38ABC5E8  addi r5, r11, -0x3a18
	ctx.r[5].s64 = ctx.r[11].s64 + -14872;
	// 832A5150: 4BC208F1  bl 0x82ec5a40
	ctx.lr = 0x832A5154;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EC5A40);
	// 832A5154: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5158: 386B8F20  addi r3, r11, -0x70e0
	ctx.r[3].s64 = ctx.r[11].s64 + -28896;
	// 832A515C: 4BA04DC5  bl 0x82ca9f20
	ctx.lr = 0x832A5160;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A516C: 4E800020  blr
	return;
}

pub fn sub_832A5170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5170 size=80
	// 832A5170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A517C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5180: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A5184: 388ABD54  addi r4, r10, -0x42ac
	ctx.r[4].s64 = ctx.r[10].s64 + -17068;
	// 832A5188: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A518C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A5190: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 832A5194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A5198: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A519C: 38ABCD20  addi r5, r11, -0x32e0
	ctx.r[5].s64 = ctx.r[11].s64 + -13024;
	// 832A51A0: 4BBE0621  bl 0x82e857c0
	ctx.lr = 0x832A51A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A51A4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A51A8: 386B8F38  addi r3, r11, -0x70c8
	ctx.r[3].s64 = ctx.r[11].s64 + -28872;
	// 832A51AC: 4BA04D75  bl 0x82ca9f20
	ctx.lr = 0x832A51B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A51B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A51B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A51B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A51BC: 4E800020  blr
	return;
}

pub fn sub_832A51C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A51C0 size=72
	// 832A51C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A51C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A51C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A51CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A51D0: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A51D4: 388ABD64  addi r4, r10, -0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + -17052;
	// 832A51D8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A51DC: 38ABEA40  addi r5, r11, -0x15c0
	ctx.r[5].s64 = ctx.r[11].s64 + -5568;
	// 832A51E0: 386A8CF8  addi r3, r10, -0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + -29448;
	// 832A51E4: 4BC3AC3D  bl 0x82edfe20
	ctx.lr = 0x832A51E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EDFE20);
	// 832A51E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A51EC: 386B8F50  addi r3, r11, -0x70b0
	ctx.r[3].s64 = ctx.r[11].s64 + -28848;
	// 832A51F0: 4BA04D31  bl 0x82ca9f20
	ctx.lr = 0x832A51F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A51F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5200: 4E800020  blr
	return;
}

pub fn sub_832A5208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5208 size=72
	// 832A5208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5214: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5218: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A521C: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 832A5220: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5224: 38ABFC48  addi r5, r11, -0x3b8
	ctx.r[5].s64 = ctx.r[11].s64 + -952;
	// 832A5228: 386A8E0C  addi r3, r10, -0x71f4
	ctx.r[3].s64 = ctx.r[10].s64 + -29172;
	// 832A522C: 4BC3ABF5  bl 0x82edfe20
	ctx.lr = 0x832A5230;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EDFE20);
	// 832A5230: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5234: 386B8F68  addi r3, r11, -0x7098
	ctx.r[3].s64 = ctx.r[11].s64 + -28824;
	// 832A5238: 4BA04CE9  bl 0x82ca9f20
	ctx.lr = 0x832A523C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A523C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5248: 4E800020  blr
	return;
}

pub fn sub_832A5250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5250 size=80
	// 832A5250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A525C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5260: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A5268: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 832A526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A5270: 4BC53AD1  bl 0x82ef8d40
	ctx.lr = 0x832A5274;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EF8D40);
	// 832A5274: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832A5278: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A527C: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 832A5280: 386A8F80  addi r3, r10, -0x7080
	ctx.r[3].s64 = ctx.r[10].s64 + -28800;
	// 832A5284: 4BA04C9D  bl 0x82ca9f20
	ctx.lr = 0x832A5288;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A528C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A5298: 4E800020  blr
	return;
}

pub fn sub_832A52A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A52A0 size=56
	// 832A52A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A52A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A52A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A52AC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A52B0: 386B9010  addi r3, r11, -0x6ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -28656;
	// 832A52B4: 4BC5190D  bl 0x82ef6bc0
	ctx.lr = 0x832A52B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EF6BC0);
	// 832A52B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A52BC: 386B8FC8  addi r3, r11, -0x7038
	ctx.r[3].s64 = ctx.r[11].s64 + -28728;
	// 832A52C0: 4BA04C61  bl 0x82ca9f20
	ctx.lr = 0x832A52C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A52C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A52C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A52CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A52D0: 4E800020  blr
	return;
}

pub fn sub_832A52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A52D8 size=136
	// 832A52D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A52DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A52E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A52E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A52E8: 386B8FC8  addi r3, r11, -0x7038
	ctx.r[3].s64 = ctx.r[11].s64 + -28728;
	// 832A52EC: 4BC51845  bl 0x82ef6b30
	ctx.lr = 0x832A52F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82EF6B30);
	// 832A52F0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A52F4: 386B9008  addi r3, r11, -0x6ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -28664;
	// 832A52F8: 4BA04C29  bl 0x82ca9f20
	ctx.lr = 0x832A52FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A52FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5308: 4E800020  blr
	return;
}

pub fn sub_832A5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5360 size=152
	// 832A5360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A536C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5370: 4BCF9921  bl 0x82f9ec90
	ctx.lr = 0x832A5374;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A5374: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5378: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A537C: 3BEA9628  addi r31, r10, -0x69d8
	ctx.r[31].s64 = ctx.r[10].s64 + -27096;
	// 832A5380: 996A9628  stb r11, -0x69d8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-27096 as u32), ctx.r[11].u8 ) };
	// 832A5384: D83F0008  stfd f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 832A5388: 4BCF9909  bl 0x82f9ec90
	ctx.lr = 0x832A538C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A538C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5390: D83F0018  stfd f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 832A5394: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 832A5398: 4BCF98F9  bl 0x82f9ec90
	ctx.lr = 0x832A539C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A539C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A53A0: D83F0028  stfd f1, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.f[1].u64 ) };
	// 832A53A4: 997F0020  stb r11, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 832A53A8: 4BCF98E9  bl 0x82f9ec90
	ctx.lr = 0x832A53AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A53AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A53B0: D83F0038  stfd f1, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.f[1].u64 ) };
	// 832A53B4: 997F0030  stb r11, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 832A53B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A53BC: 386B9160  addi r3, r11, -0x6ea0
	ctx.r[3].s64 = ctx.r[11].s64 + -28320;
	// 832A53C0: 4BA04B61  bl 0x82ca9f20
	ctx.lr = 0x832A53C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A53C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A53C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A53CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A53D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A53D4: 4E800020  blr
	return;
	// 832A53D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A53DC: 386B966C  addi r3, r11, -0x6994
	ctx.r[3].s64 = ctx.r[11].s64 + -27028;
	// 832A53E0: 4BCF9838  b 0x82f9ec18
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC18);
	return;
}

pub fn sub_832A53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A53F8 size=176
	// 832A53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A5404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5408: 4BCF9889  bl 0x82f9ec90
	ctx.lr = 0x832A540C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A540C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5410: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5414: 3BEA9698  addi r31, r10, -0x6968
	ctx.r[31].s64 = ctx.r[10].s64 + -26984;
	// 832A5418: 996A9698  stb r11, -0x6968(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-26984 as u32), ctx.r[11].u8 ) };
	// 832A541C: D83F0008  stfd f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 832A5420: 4BCF9871  bl 0x82f9ec90
	ctx.lr = 0x832A5424;
	crate::recompiler::externs::call(&mut ctx, base, 0x82F9EC90);
	// 832A5424: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5428: D83F0018  stfd f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 832A542C: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 832A5430: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5434: 386B9220  addi r3, r11, -0x6de0
	ctx.r[3].s64 = ctx.r[11].s64 + -28128;
	// 832A5438: 4BA04AE9  bl 0x82ca9f20
	ctx.lr = 0x832A543C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A543C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A544C: 4E800020  blr
	return;
	// 832A5450: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5454: 386B9280  addi r3, r11, -0x6d80
	ctx.r[3].s64 = ctx.r[11].s64 + -28032;
	// 832A5458: 4BA04AC8  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_832A54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A54A8 size=8
	// 832A54A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A54AC: 4BA03F39  bl 0x82ca93e4
	ctx.lr = 0x832A54B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
}

pub fn sub_832A55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A55E8 size=56
	// 832A55E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A55EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A55F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A55F4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A55F8: 396BF784  addi r11, r11, -0x87c
	ctx.r[11].s64 = ctx.r[11].s64 + -2172;
	// 832A55FC: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832A5600: 48014685  bl 0x832b9c84
	ctx.lr = 0x832A5604;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832A5604: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5608: 386B9358  addi r3, r11, -0x6ca8
	ctx.r[3].s64 = ctx.r[11].s64 + -27816;
	// 832A560C: 4BA04915  bl 0x82ca9f20
	ctx.lr = 0x832A5610;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A561C: 4E800020  blr
	return;
}

pub fn sub_832A5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5620 size=16
	// 832A5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A562C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832A5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5680 size=56
	// 832A5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A568C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5690: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5694: 388BB320  addi r4, r11, -0x4ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	// 832A5698: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A569C: 386B97C0  addi r3, r11, -0x6840
	ctx.r[3].s64 = ctx.r[11].s64 + -26688;
	// 832A56A0: 4BD5DEF9  bl 0x83003598
	ctx.lr = 0x832A56A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A56A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56B0: 4E800020  blr
	return;
}

pub fn sub_832A56B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56B8 size=56
	// 832A56B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A56C8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A56CC: 388BB38C  addi r4, r11, -0x4c74
	ctx.r[4].s64 = ctx.r[11].s64 + -19572;
	// 832A56D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A56D4: 386B97C8  addi r3, r11, -0x6838
	ctx.r[3].s64 = ctx.r[11].s64 + -26680;
	// 832A56D8: 4BD5DEC1  bl 0x83003598
	ctx.lr = 0x832A56DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A56DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56E8: 4E800020  blr
	return;
}

pub fn sub_832A56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56F0 size=56
	// 832A56F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5700: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5704: 388BB778  addi r4, r11, -0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + -18568;
	// 832A5708: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A570C: 386B97D0  addi r3, r11, -0x6830
	ctx.r[3].s64 = ctx.r[11].s64 + -26672;
	// 832A5710: 4BD5DE89  bl 0x83003598
	ctx.lr = 0x832A5714;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A571C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5720: 4E800020  blr
	return;
}

pub fn sub_832A5728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5728 size=56
	// 832A5728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5734: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 832A5738: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A573C: 388BB808  addi r4, r11, -0x47f8
	ctx.r[4].s64 = ctx.r[11].s64 + -18424;
	// 832A5740: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5744: 386B97D8  addi r3, r11, -0x6828
	ctx.r[3].s64 = ctx.r[11].s64 + -26664;
	// 832A5748: 4BD5DE51  bl 0x83003598
	ctx.lr = 0x832A574C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A574C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5758: 4E800020  blr
	return;
}

pub fn sub_832A5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5760 size=56
	// 832A5760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A576C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5770: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5774: 388BB970  addi r4, r11, -0x4690
	ctx.r[4].s64 = ctx.r[11].s64 + -18064;
	// 832A5778: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A577C: 386B97E0  addi r3, r11, -0x6820
	ctx.r[3].s64 = ctx.r[11].s64 + -26656;
	// 832A5780: 4BD5DE19  bl 0x83003598
	ctx.lr = 0x832A5784;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A578C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5790: 4E800020  blr
	return;
}

pub fn sub_832A5798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5798 size=56
	// 832A5798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57A8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57AC: 388BB9B8  addi r4, r11, -0x4648
	ctx.r[4].s64 = ctx.r[11].s64 + -17992;
	// 832A57B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57B4: 386B97E8  addi r3, r11, -0x6818
	ctx.r[3].s64 = ctx.r[11].s64 + -26648;
	// 832A57B8: 4BD5DDE1  bl 0x83003598
	ctx.lr = 0x832A57BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A57BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A57C8: 4E800020  blr
	return;
}

pub fn sub_832A57D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A57D0 size=56
	// 832A57D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A57D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57DC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57E0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57E4: 388BBA48  addi r4, r11, -0x45b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17848;
	// 832A57E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57EC: 386B97F0  addi r3, r11, -0x6810
	ctx.r[3].s64 = ctx.r[11].s64 + -26640;
	// 832A57F0: 4BD5DDA9  bl 0x83003598
	ctx.lr = 0x832A57F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A57F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5800: 4E800020  blr
	return;
}

pub fn sub_832A5808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5808 size=56
	// 832A5808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A580C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5814: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A5818: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A581C: 388BBAD8  addi r4, r11, -0x4528
	ctx.r[4].s64 = ctx.r[11].s64 + -17704;
	// 832A5820: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5824: 386B97F8  addi r3, r11, -0x6808
	ctx.r[3].s64 = ctx.r[11].s64 + -26632;
	// 832A5828: 4BD5DD71  bl 0x83003598
	ctx.lr = 0x832A582C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A582C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5838: 4E800020  blr
	return;
}

pub fn sub_832A5840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5840 size=56
	// 832A5840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A584C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A5850: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5854: 388BBB68  addi r4, r11, -0x4498
	ctx.r[4].s64 = ctx.r[11].s64 + -17560;
	// 832A5858: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A585C: 386B9800  addi r3, r11, -0x6800
	ctx.r[3].s64 = ctx.r[11].s64 + -26624;
	// 832A5860: 4BD5DD39  bl 0x83003598
	ctx.lr = 0x832A5864;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A586C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5870: 4E800020  blr
	return;
}

pub fn sub_832A5878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5878 size=56
	// 832A5878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5884: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832A5888: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A588C: 388BBD18  addi r4, r11, -0x42e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17128;
	// 832A5890: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5894: 386B9808  addi r3, r11, -0x67f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26616;
	// 832A5898: 4BD5DD01  bl 0x83003598
	ctx.lr = 0x832A589C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A589C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58A8: 4E800020  blr
	return;
}

pub fn sub_832A58B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58B0 size=56
	// 832A58B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58BC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A58C0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58C4: 388BBDF0  addi r4, r11, -0x4210
	ctx.r[4].s64 = ctx.r[11].s64 + -16912;
	// 832A58C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A58CC: 386B9810  addi r3, r11, -0x67f0
	ctx.r[3].s64 = ctx.r[11].s64 + -26608;
	// 832A58D0: 4BD5DCC9  bl 0x83003598
	ctx.lr = 0x832A58D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58E0: 4E800020  blr
	return;
}

pub fn sub_832A58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58E8 size=56
	// 832A58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A58F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58FC: 388BBF7C  addi r4, r11, -0x4084
	ctx.r[4].s64 = ctx.r[11].s64 + -16516;
	// 832A5900: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5904: 386B9818  addi r3, r11, -0x67e8
	ctx.r[3].s64 = ctx.r[11].s64 + -26600;
	// 832A5908: 4BD5DC91  bl 0x83003598
	ctx.lr = 0x832A590C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A590C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5918: 4E800020  blr
	return;
}

pub fn sub_832A5920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5920 size=56
	// 832A5920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A592C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5930: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5934: 388BBFE8  addi r4, r11, -0x4018
	ctx.r[4].s64 = ctx.r[11].s64 + -16408;
	// 832A5938: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A593C: 386B9820  addi r3, r11, -0x67e0
	ctx.r[3].s64 = ctx.r[11].s64 + -26592;
	// 832A5940: 4BD5DC59  bl 0x83003598
	ctx.lr = 0x832A5944;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A594C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5950: 4E800020  blr
	return;
}

pub fn sub_832A5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5958 size=56
	// 832A5958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A595C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5964: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 832A5968: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A596C: 388BC030  addi r4, r11, -0x3fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -16336;
	// 832A5970: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5974: 386B9828  addi r3, r11, -0x67d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26584;
	// 832A5978: 4BD5DC21  bl 0x83003598
	ctx.lr = 0x832A597C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A597C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5988: 4E800020  blr
	return;
}

pub fn sub_832A5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5990 size=48
	// 832A5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A599C: 4BD5DBA5  bl 0x83003540
	ctx.lr = 0x832A59A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003540);
	// 832A59A0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59A4: 396B9830  addi r11, r11, -0x67d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26576;
	// 832A59A8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 832A59AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59B8: 4E800020  blr
	return;
}

pub fn sub_832A59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59C0 size=56
	// 832A59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A59C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A59CC: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 832A59D0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A59D4: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 832A59D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59DC: 386B9834  addi r3, r11, -0x67cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26572;
	// 832A59E0: 4BD5DBB9  bl 0x83003598
	ctx.lr = 0x832A59E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A59E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59F0: 4E800020  blr
	return;
}

pub fn sub_832A59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59F8 size=56
	// 832A59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A04: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A08: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 832A5A0C: 4BD5F6E5  bl 0x830050f0
	ctx.lr = 0x832A5A10;
	crate::recompiler::externs::call(&mut ctx, base, 0x830050F0);
	// 832A5A10: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A14: 386B9370  addi r3, r11, -0x6c90
	ctx.r[3].s64 = ctx.r[11].s64 + -27792;
	// 832A5A18: 4BA04509  bl 0x82ca9f20
	ctx.lr = 0x832A5A1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A28: 4E800020  blr
	return;
}

pub fn sub_832A5A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A30 size=56
	// 832A5A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A3C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A40: 386B9870  addi r3, r11, -0x6790
	ctx.r[3].s64 = ctx.r[11].s64 + -26512;
	// 832A5A44: 4BD5DBE5  bl 0x83003628
	ctx.lr = 0x832A5A48;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5A48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A4C: 386B9398  addi r3, r11, -0x6c68
	ctx.r[3].s64 = ctx.r[11].s64 + -27752;
	// 832A5A50: 4BA044D1  bl 0x82ca9f20
	ctx.lr = 0x832A5A54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A60: 4E800020  blr
	return;
}

pub fn sub_832A5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A68 size=56
	// 832A5A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A74: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A78: 386B9864  addi r3, r11, -0x679c
	ctx.r[3].s64 = ctx.r[11].s64 + -26524;
	// 832A5A7C: 4BD5EDED  bl 0x83004868
	ctx.lr = 0x832A5A80;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5A80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A84: 386B93C0  addi r3, r11, -0x6c40
	ctx.r[3].s64 = ctx.r[11].s64 + -27712;
	// 832A5A88: 4BA04499  bl 0x82ca9f20
	ctx.lr = 0x832A5A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A98: 4E800020  blr
	return;
}

pub fn sub_832A5AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AA0 size=56
	// 832A5AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AAC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AB0: 386B98A0  addi r3, r11, -0x6760
	ctx.r[3].s64 = ctx.r[11].s64 + -26464;
	// 832A5AB4: 4BD5DB75  bl 0x83003628
	ctx.lr = 0x832A5AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5AB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5ABC: 386B93E8  addi r3, r11, -0x6c18
	ctx.r[3].s64 = ctx.r[11].s64 + -27672;
	// 832A5AC0: 4BA04461  bl 0x82ca9f20
	ctx.lr = 0x832A5AC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5AD0: 4E800020  blr
	return;
}

pub fn sub_832A5AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AD8 size=56
	// 832A5AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AE4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AE8: 386B9894  addi r3, r11, -0x676c
	ctx.r[3].s64 = ctx.r[11].s64 + -26476;
	// 832A5AEC: 4BD5ED7D  bl 0x83004868
	ctx.lr = 0x832A5AF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5AF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5AF4: 386B9410  addi r3, r11, -0x6bf0
	ctx.r[3].s64 = ctx.r[11].s64 + -27632;
	// 832A5AF8: 4BA04429  bl 0x82ca9f20
	ctx.lr = 0x832A5AFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B08: 4E800020  blr
	return;
}

pub fn sub_832A5B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B10 size=56
	// 832A5B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B1C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B20: 386B98D0  addi r3, r11, -0x6730
	ctx.r[3].s64 = ctx.r[11].s64 + -26416;
	// 832A5B24: 4BD5DB05  bl 0x83003628
	ctx.lr = 0x832A5B28;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5B28: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B2C: 386B9438  addi r3, r11, -0x6bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -27592;
	// 832A5B30: 4BA043F1  bl 0x82ca9f20
	ctx.lr = 0x832A5B34;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B40: 4E800020  blr
	return;
}

pub fn sub_832A5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B48 size=56
	// 832A5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B54: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B58: 386B98C4  addi r3, r11, -0x673c
	ctx.r[3].s64 = ctx.r[11].s64 + -26428;
	// 832A5B5C: 4BD5ED0D  bl 0x83004868
	ctx.lr = 0x832A5B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5B60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B64: 386B9460  addi r3, r11, -0x6ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -27552;
	// 832A5B68: 4BA043B9  bl 0x82ca9f20
	ctx.lr = 0x832A5B6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B78: 4E800020  blr
	return;
}

pub fn sub_832A5B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B80 size=56
	// 832A5B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5B90: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5B94: 388BC490  addi r4, r11, -0x3b70
	ctx.r[4].s64 = ctx.r[11].s64 + -15216;
	// 832A5B98: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B9C: 386B98F4  addi r3, r11, -0x670c
	ctx.r[3].s64 = ctx.r[11].s64 + -26380;
	// 832A5BA0: 4BD5D9F9  bl 0x83003598
	ctx.lr = 0x832A5BA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BB0: 4E800020  blr
	return;
}

pub fn sub_832A5BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BB8 size=56
	// 832A5BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5BC8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5BCC: 388BC4B4  addi r4, r11, -0x3b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -15180;
	// 832A5BD0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5BD4: 386B98FC  addi r3, r11, -0x6704
	ctx.r[3].s64 = ctx.r[11].s64 + -26372;
	// 832A5BD8: 4BD5D9C1  bl 0x83003598
	ctx.lr = 0x832A5BDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BE8: 4E800020  blr
	return;
}

pub fn sub_832A5BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BF0 size=56
	// 832A5BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C00: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C04: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 832A5C08: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C0C: 386B9904  addi r3, r11, -0x66fc
	ctx.r[3].s64 = ctx.r[11].s64 + -26364;
	// 832A5C10: 4BD5D989  bl 0x83003598
	ctx.lr = 0x832A5C14;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C20: 4E800020  blr
	return;
}

pub fn sub_832A5C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C28 size=56
	// 832A5C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C38: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C3C: 388BC4FC  addi r4, r11, -0x3b04
	ctx.r[4].s64 = ctx.r[11].s64 + -15108;
	// 832A5C40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C44: 386B990C  addi r3, r11, -0x66f4
	ctx.r[3].s64 = ctx.r[11].s64 + -26356;
	// 832A5C48: 4BD5D951  bl 0x83003598
	ctx.lr = 0x832A5C4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C58: 4E800020  blr
	return;
}

pub fn sub_832A5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C60 size=56
	// 832A5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C70: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C74: 388BC520  addi r4, r11, -0x3ae0
	ctx.r[4].s64 = ctx.r[11].s64 + -15072;
	// 832A5C78: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C7C: 386B9914  addi r3, r11, -0x66ec
	ctx.r[3].s64 = ctx.r[11].s64 + -26348;
	// 832A5C80: 4BD5D919  bl 0x83003598
	ctx.lr = 0x832A5C84;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C90: 4E800020  blr
	return;
}

pub fn sub_832A5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C98 size=56
	// 832A5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CA8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CAC: 388BC544  addi r4, r11, -0x3abc
	ctx.r[4].s64 = ctx.r[11].s64 + -15036;
	// 832A5CB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CB4: 386B991C  addi r3, r11, -0x66e4
	ctx.r[3].s64 = ctx.r[11].s64 + -26340;
	// 832A5CB8: 4BD5D8E1  bl 0x83003598
	ctx.lr = 0x832A5CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5CC8: 4E800020  blr
	return;
}

pub fn sub_832A5CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5CD0 size=56
	// 832A5CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CE0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CE4: 388BC568  addi r4, r11, -0x3a98
	ctx.r[4].s64 = ctx.r[11].s64 + -15000;
	// 832A5CE8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CEC: 386B9924  addi r3, r11, -0x66dc
	ctx.r[3].s64 = ctx.r[11].s64 + -26332;
	// 832A5CF0: 4BD5D8A9  bl 0x83003598
	ctx.lr = 0x832A5CF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D00: 4E800020  blr
	return;
}

pub fn sub_832A5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D08 size=56
	// 832A5D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5D18: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5D1C: 388BC58C  addi r4, r11, -0x3a74
	ctx.r[4].s64 = ctx.r[11].s64 + -14964;
	// 832A5D20: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D24: 386B992C  addi r3, r11, -0x66d4
	ctx.r[3].s64 = ctx.r[11].s64 + -26324;
	// 832A5D28: 4BD5D871  bl 0x83003598
	ctx.lr = 0x832A5D2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A5D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D38: 4E800020  blr
	return;
}

pub fn sub_832A5D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D40 size=56
	// 832A5D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D4C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D50: 386B9940  addi r3, r11, -0x66c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26304;
	// 832A5D54: 4BD5D8D5  bl 0x83003628
	ctx.lr = 0x832A5D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5D58: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D5C: 386B9488  addi r3, r11, -0x6b78
	ctx.r[3].s64 = ctx.r[11].s64 + -27512;
	// 832A5D60: 4BA041C1  bl 0x82ca9f20
	ctx.lr = 0x832A5D64;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D70: 4E800020  blr
	return;
}

pub fn sub_832A5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D78 size=56
	// 832A5D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D84: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D88: 386B9934  addi r3, r11, -0x66cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26316;
	// 832A5D8C: 4BD5EADD  bl 0x83004868
	ctx.lr = 0x832A5D90;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5D90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D94: 386B94B0  addi r3, r11, -0x6b50
	ctx.r[3].s64 = ctx.r[11].s64 + -27472;
	// 832A5D98: 4BA04189  bl 0x82ca9f20
	ctx.lr = 0x832A5D9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DA8: 4E800020  blr
	return;
}

pub fn sub_832A5DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DB0 size=56
	// 832A5DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DC0: 386B9A00  addi r3, r11, -0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + -26112;
	// 832A5DC4: 4BD5D865  bl 0x83003628
	ctx.lr = 0x832A5DC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5DC8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5DCC: 386B94D8  addi r3, r11, -0x6b28
	ctx.r[3].s64 = ctx.r[11].s64 + -27432;
	// 832A5DD0: 4BA04151  bl 0x82ca9f20
	ctx.lr = 0x832A5DD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DE0: 4E800020  blr
	return;
}

pub fn sub_832A5DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DE8 size=56
	// 832A5DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DF4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DF8: 386B99F4  addi r3, r11, -0x660c
	ctx.r[3].s64 = ctx.r[11].s64 + -26124;
	// 832A5DFC: 4BD5EA6D  bl 0x83004868
	ctx.lr = 0x832A5E00;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5E00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E04: 386B9500  addi r3, r11, -0x6b00
	ctx.r[3].s64 = ctx.r[11].s64 + -27392;
	// 832A5E08: 4BA04119  bl 0x82ca9f20
	ctx.lr = 0x832A5E0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E18: 4E800020  blr
	return;
}

pub fn sub_832A5E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E20 size=56
	// 832A5E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E2C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E30: 386B99D0  addi r3, r11, -0x6630
	ctx.r[3].s64 = ctx.r[11].s64 + -26160;
	// 832A5E34: 4BD5D7F5  bl 0x83003628
	ctx.lr = 0x832A5E38;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5E38: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E3C: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 832A5E40: 4BA040E1  bl 0x82ca9f20
	ctx.lr = 0x832A5E44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E50: 4E800020  blr
	return;
}

pub fn sub_832A5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E58 size=56
	// 832A5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E64: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E68: 386B99C4  addi r3, r11, -0x663c
	ctx.r[3].s64 = ctx.r[11].s64 + -26172;
	// 832A5E6C: 4BD5E9FD  bl 0x83004868
	ctx.lr = 0x832A5E70;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5E70: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E74: 386B9550  addi r3, r11, -0x6ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -27312;
	// 832A5E78: 4BA040A9  bl 0x82ca9f20
	ctx.lr = 0x832A5E7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E88: 4E800020  blr
	return;
}

pub fn sub_832A5E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E90 size=56
	// 832A5E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E9C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5EA0: 386B99A0  addi r3, r11, -0x6660
	ctx.r[3].s64 = ctx.r[11].s64 + -26208;
	// 832A5EA4: 4BD5D785  bl 0x83003628
	ctx.lr = 0x832A5EA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5EA8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EAC: 386B9578  addi r3, r11, -0x6a88
	ctx.r[3].s64 = ctx.r[11].s64 + -27272;
	// 832A5EB0: 4BA04071  bl 0x82ca9f20
	ctx.lr = 0x832A5EB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EC0: 4E800020  blr
	return;
}

pub fn sub_832A5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5EC8 size=56
	// 832A5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5ED4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5ED8: 386B9994  addi r3, r11, -0x666c
	ctx.r[3].s64 = ctx.r[11].s64 + -26220;
	// 832A5EDC: 4BD5E98D  bl 0x83004868
	ctx.lr = 0x832A5EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5EE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EE4: 386B95A0  addi r3, r11, -0x6a60
	ctx.r[3].s64 = ctx.r[11].s64 + -27232;
	// 832A5EE8: 4BA04039  bl 0x82ca9f20
	ctx.lr = 0x832A5EEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EF8: 4E800020  blr
	return;
}

pub fn sub_832A5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F00 size=56
	// 832A5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F10: 386B9970  addi r3, r11, -0x6690
	ctx.r[3].s64 = ctx.r[11].s64 + -26256;
	// 832A5F14: 4BD5D715  bl 0x83003628
	ctx.lr = 0x832A5F18;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5F18: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F1C: 386B95C8  addi r3, r11, -0x6a38
	ctx.r[3].s64 = ctx.r[11].s64 + -27192;
	// 832A5F20: 4BA04001  bl 0x82ca9f20
	ctx.lr = 0x832A5F24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F30: 4E800020  blr
	return;
}

pub fn sub_832A5F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F38 size=56
	// 832A5F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F44: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F48: 386B9964  addi r3, r11, -0x669c
	ctx.r[3].s64 = ctx.r[11].s64 + -26268;
	// 832A5F4C: 4BD5E91D  bl 0x83004868
	ctx.lr = 0x832A5F50;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5F50: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F54: 386B95F0  addi r3, r11, -0x6a10
	ctx.r[3].s64 = ctx.r[11].s64 + -27152;
	// 832A5F58: 4BA03FC9  bl 0x82ca9f20
	ctx.lr = 0x832A5F5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F68: 4E800020  blr
	return;
}

pub fn sub_832A5F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F70 size=56
	// 832A5F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F7C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F80: 386B9A34  addi r3, r11, -0x65cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26060;
	// 832A5F84: 4BD5D6A5  bl 0x83003628
	ctx.lr = 0x832A5F88;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5F88: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F8C: 386B9618  addi r3, r11, -0x69e8
	ctx.r[3].s64 = ctx.r[11].s64 + -27112;
	// 832A5F90: 4BA03F91  bl 0x82ca9f20
	ctx.lr = 0x832A5F94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FA0: 4E800020  blr
	return;
}

pub fn sub_832A5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FA8 size=56
	// 832A5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FB4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FB8: 386B9A28  addi r3, r11, -0x65d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26072;
	// 832A5FBC: 4BD5E8AD  bl 0x83004868
	ctx.lr = 0x832A5FC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A5FC0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FC4: 386B9640  addi r3, r11, -0x69c0
	ctx.r[3].s64 = ctx.r[11].s64 + -27072;
	// 832A5FC8: 4BA03F59  bl 0x82ca9f20
	ctx.lr = 0x832A5FCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A5FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FD8: 4E800020  blr
	return;
}

pub fn sub_832A5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FE0 size=56
	// 832A5FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FEC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FF0: 386B9A64  addi r3, r11, -0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + -26012;
	// 832A5FF4: 4BD5D635  bl 0x83003628
	ctx.lr = 0x832A5FF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A5FF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FFC: 386B9668  addi r3, r11, -0x6998
	ctx.r[3].s64 = ctx.r[11].s64 + -27032;
	// 832A6000: 4BA03F21  bl 0x82ca9f20
	ctx.lr = 0x832A6004;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A6004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6010: 4E800020  blr
	return;
}

pub fn sub_832A6018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6018 size=56
	// 832A6018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6024: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6028: 386B9A58  addi r3, r11, -0x65a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26024;
	// 832A602C: 4BD5E83D  bl 0x83004868
	ctx.lr = 0x832A6030;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A6030: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6034: 386B9690  addi r3, r11, -0x6970
	ctx.r[3].s64 = ctx.r[11].s64 + -26992;
	// 832A6038: 4BA03EE9  bl 0x82ca9f20
	ctx.lr = 0x832A603C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A603C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6048: 4E800020  blr
	return;
}

pub fn sub_832A6050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6050 size=56
	// 832A6050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A605C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6060: 386B9AC4  addi r3, r11, -0x653c
	ctx.r[3].s64 = ctx.r[11].s64 + -25916;
	// 832A6064: 4BD5D5C5  bl 0x83003628
	ctx.lr = 0x832A6068;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A6068: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A606C: 386B96B8  addi r3, r11, -0x6948
	ctx.r[3].s64 = ctx.r[11].s64 + -26952;
	// 832A6070: 4BA03EB1  bl 0x82ca9f20
	ctx.lr = 0x832A6074;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A6074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A607C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6080: 4E800020  blr
	return;
}

pub fn sub_832A6088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6088 size=56
	// 832A6088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A608C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6094: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6098: 386B9AB8  addi r3, r11, -0x6548
	ctx.r[3].s64 = ctx.r[11].s64 + -25928;
	// 832A609C: 4BD5E7CD  bl 0x83004868
	ctx.lr = 0x832A60A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A60A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60A4: 386B96E0  addi r3, r11, -0x6920
	ctx.r[3].s64 = ctx.r[11].s64 + -26912;
	// 832A60A8: 4BA03E79  bl 0x82ca9f20
	ctx.lr = 0x832A60AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60B8: 4E800020  blr
	return;
}

pub fn sub_832A60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60C0 size=56
	// 832A60C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A60C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A60CC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A60D0: 386B9A94  addi r3, r11, -0x656c
	ctx.r[3].s64 = ctx.r[11].s64 + -25964;
	// 832A60D4: 4BD5D555  bl 0x83003628
	ctx.lr = 0x832A60D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A60D8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60DC: 386B9708  addi r3, r11, -0x68f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26872;
	// 832A60E0: 4BA03E41  bl 0x82ca9f20
	ctx.lr = 0x832A60E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A60E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60F0: 4E800020  blr
	return;
}

pub fn sub_832A60F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60F8 size=56
	// 832A60F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6104: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6108: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 832A610C: 4BD5E75D  bl 0x83004868
	ctx.lr = 0x832A6110;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004868);
	// 832A6110: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6114: 386B9730  addi r3, r11, -0x68d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26832;
	// 832A6118: 4BA03E09  bl 0x82ca9f20
	ctx.lr = 0x832A611C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A611C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6128: 4E800020  blr
	return;
}

pub fn sub_832A6130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6130 size=48
	// 832A6130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A613C: 4BD5EE5D  bl 0x83004f98
	ctx.lr = 0x832A6140;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004F98);
	// 832A6140: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6144: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 832A6148: F86B0000  std r3, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 832A614C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6158: 4E800020  blr
	return;
}

pub fn sub_832A6160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6160 size=48
	// 832A6160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A616C: 4BD5EE5D  bl 0x83004fc8
	ctx.lr = 0x832A6170;
	crate::recompiler::externs::call(&mut ctx, base, 0x83004FC8);
	// 832A6170: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6174: 396B9AF0  addi r11, r11, -0x6510
	ctx.r[11].s64 = ctx.r[11].s64 + -25872;
	// 832A6178: D82B0000  stfd f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 832A617C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6188: 4E800020  blr
	return;
}

pub fn sub_832A6190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6190 size=56
	// 832A6190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A619C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A61A0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61A4: 388BD340  addi r4, r11, -0x2cc0
	ctx.r[4].s64 = ctx.r[11].s64 + -11456;
	// 832A61A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61AC: 386B9AFC  addi r3, r11, -0x6504
	ctx.r[3].s64 = ctx.r[11].s64 + -25860;
	// 832A61B0: 4BD5D3E9  bl 0x83003598
	ctx.lr = 0x832A61B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A61B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61C0: 4E800020  blr
	return;
}

pub fn sub_832A61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A61C8 size=56
	// 832A61C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A61CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A61D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A61D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A61D8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61DC: 388BD3D0  addi r4, r11, -0x2c30
	ctx.r[4].s64 = ctx.r[11].s64 + -11312;
	// 832A61E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61E4: 386B9B04  addi r3, r11, -0x64fc
	ctx.r[3].s64 = ctx.r[11].s64 + -25852;
	// 832A61E8: 4BD5D3B1  bl 0x83003598
	ctx.lr = 0x832A61EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A61EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61F8: 4E800020  blr
	return;
}

pub fn sub_832A6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6200 size=56
	// 832A6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A620C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6210: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A6214: 388BD4C8  addi r4, r11, -0x2b38
	ctx.r[4].s64 = ctx.r[11].s64 + -11064;
	// 832A6218: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A621C: 386B9B0C  addi r3, r11, -0x64f4
	ctx.r[3].s64 = ctx.r[11].s64 + -25844;
	// 832A6220: 4BD5D379  bl 0x83003598
	ctx.lr = 0x832A6224;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A6224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A622C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6230: 4E800020  blr
	return;
}

pub fn sub_832A6238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6238 size=56
	// 832A6238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6244: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6248: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A624C: 388BD558  addi r4, r11, -0x2aa8
	ctx.r[4].s64 = ctx.r[11].s64 + -10920;
	// 832A6250: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6254: 386B9B14  addi r3, r11, -0x64ec
	ctx.r[3].s64 = ctx.r[11].s64 + -25836;
	// 832A6258: 4BD5D341  bl 0x83003598
	ctx.lr = 0x832A625C;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003598);
	// 832A625C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6268: 4E800020  blr
	return;
}

pub fn sub_832A6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6270 size=56
	// 832A6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A627C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6280: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 832A6284: 4BD5EE6D  bl 0x830050f0
	ctx.lr = 0x832A6288;
	crate::recompiler::externs::call(&mut ctx, base, 0x830050F0);
	// 832A6288: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A628C: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 832A6290: 4BA03C91  bl 0x82ca9f20
	ctx.lr = 0x832A6294;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A6294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A629C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62A0: 4E800020  blr
	return;
}

pub fn sub_832A62A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62A8 size=56
	// 832A62A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62B4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62B8: 386B9B44  addi r3, r11, -0x64bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25788;
	// 832A62BC: 4BD5D36D  bl 0x83003628
	ctx.lr = 0x832A62C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x83003628);
	// 832A62C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62C4: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 832A62C8: 4BA03C59  bl 0x82ca9f20
	ctx.lr = 0x832A62CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A62CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A62D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A62D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62D8: 4E800020  blr
	return;
}

pub fn sub_832A62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62E0 size=56
	// 832A62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62F0: 386B9B68  addi r3, r11, -0x6498
	ctx.r[3].s64 = ctx.r[11].s64 + -25752;
	// 832A62F4: 4BD5FC4D  bl 0x83005f40
	ctx.lr = 0x832A62F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x83005F40);
	// 832A62F8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62FC: 386B97A8  addi r3, r11, -0x6858
	ctx.r[3].s64 = ctx.r[11].s64 + -26712;
	// 832A6300: 4BA03C21  bl 0x82ca9f20
	ctx.lr = 0x832A6304;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A6304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A630C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6310: 4E800020  blr
	return;
}

