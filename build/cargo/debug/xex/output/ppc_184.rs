pub fn sub_832A6318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6318 size=112
	// 832A6318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A631C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6324: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6328: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A632C: 38AAB2C4  addi r5, r10, -0x4d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -19772;
	// 832A6330: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6334: 390B86C4  addi r8, r11, -0x793c
	ctx.r[8].s64 = ctx.r[11].s64 + -31036;
	// 832A6338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A633C: 388A86F4  addi r4, r10, -0x790c
	ctx.r[4].s64 = ctx.r[10].s64 + -30988;
	// 832A6340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6344: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A634C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6350: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 832A6354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A6358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A635C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A636C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6374: 4BAAF8CD  bl 0x82d55c40
	ctx.lr = 0x832A6378;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6384: 4E800020  blr
	return;
}

pub fn sub_832A6388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6388 size=112
	// 832A6388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A638C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6394: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A639C: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 832A63A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A63A4: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A63A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A63AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A63B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A63B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A63B8: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 832A63BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A63C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A63C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A63C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A63CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A63D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A63D4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A63D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A63DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A63E0: 4BAAF861  bl 0x82d55c40
	ctx.lr = 0x832A63E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A63E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A63E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A63EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A63F0: 4E800020  blr
	return;
}

pub fn sub_832A63F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A63F8 size=360
	// 832A63F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A63FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6404: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A640C: 38EB8848  addi r7, r11, -0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + -30648;
	// 832A6410: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A6414: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A641C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6428: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 832A642C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A643C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6444: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A644C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6450: 4BAAF7F1  bl 0x82d55c40
	ctx.lr = 0x832A6454;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A645C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6460: 4E800020  blr
	return;
}

pub fn sub_832A6560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6560 size=112
	// 832A6560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A656C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6570: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6574: 392A8A60  addi r9, r10, -0x75a0
	ctx.r[9].s64 = ctx.r[10].s64 + -30112;
	// 832A6578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A657C: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 832A6580: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 832A6584: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A6588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A658C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6594: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A6598: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 832A659C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A65A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A65A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A65A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A65AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A65B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A65B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A65B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A65BC: 4BAAF685  bl 0x82d55c40
	ctx.lr = 0x832A65C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A65C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A65C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A65C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A65CC: 4E800020  blr
	return;
}

pub fn sub_832A65D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A65D0 size=112
	// 832A65D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A65D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A65D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A65DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A65E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A65E4: 38EB8A9C  addi r7, r11, -0x7564
	ctx.r[7].s64 = ctx.r[11].s64 + -30052;
	// 832A65E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A65EC: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A65F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A65F4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A65F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A65FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6600: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 832A6604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A660C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A661C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6628: 4BAAF619  bl 0x82d55c40
	ctx.lr = 0x832A662C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A662C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6638: 4E800020  blr
	return;
}

pub fn sub_832A6640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6640 size=112
	// 832A6640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A664C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6654: 38EB8ACC  addi r7, r11, -0x7534
	ctx.r[7].s64 = ctx.r[11].s64 + -30004;
	// 832A6658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A665C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A6660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6664: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A666C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6670: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 832A6674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A667C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A668C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6698: 4BAAF5A9  bl 0x82d55c40
	ctx.lr = 0x832A669C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A669C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A66A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A66A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A66A8: 4E800020  blr
	return;
}

pub fn sub_832A66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A66B0 size=112
	// 832A66B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A66B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A66B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A66BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A66C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A66C4: 38EB8B10  addi r7, r11, -0x74f0
	ctx.r[7].s64 = ctx.r[11].s64 + -29936;
	// 832A66C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A66CC: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A66D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A66D4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A66D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A66DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A66E0: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 832A66E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A66E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A66EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A66F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A66F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A66F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A66FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6708: 4BAAF539  bl 0x82d55c40
	ctx.lr = 0x832A670C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A670C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6718: 4E800020  blr
	return;
}

pub fn sub_832A6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6720 size=408
	// 832A6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A672C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6734: 38EB8C00  addi r7, r11, -0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + -29696;
	// 832A6738: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A673C: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6744: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6750: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 832A6754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A675C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A676C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6778: 4BAAF4C9  bl 0x82d55c40
	ctx.lr = 0x832A677C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6788: 4E800020  blr
	return;
}

pub fn sub_832A68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A68B8 size=112
	// 832A68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A68C4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A68C8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A68CC: 392A8E60  addi r9, r10, -0x71a0
	ctx.r[9].s64 = ctx.r[10].s64 + -29088;
	// 832A68D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A68D4: 390B1710  addi r8, r11, 0x1710
	ctx.r[8].s64 = ctx.r[11].s64 + 5904;
	// 832A68D8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A68DC: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A68E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A68E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A68E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A68EC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A68F0: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 832A68F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A68F8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A68FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A690C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6914: 4BAAF32D  bl 0x82d55c40
	ctx.lr = 0x832A6918;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6924: 4E800020  blr
	return;
}

pub fn sub_832A6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6928 size=112
	// 832A6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6934: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A693C: 38EB8E9C  addi r7, r11, -0x7164
	ctx.r[7].s64 = ctx.r[11].s64 + -29028;
	// 832A6940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6944: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A6948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A694C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6958: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 832A695C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6974: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6980: 4BAAF2C1  bl 0x82d55c40
	ctx.lr = 0x832A6984;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A698C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6990: 4E800020  blr
	return;
}

pub fn sub_832A6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6998 size=144
	// 832A6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A69A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A69A4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A69A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A69AC: 38EB8ED0  addi r7, r11, -0x7130
	ctx.r[7].s64 = ctx.r[11].s64 + -28976;
	// 832A69B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A69B4: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A69B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A69BC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A69C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A69C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A69C8: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 832A69CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A69D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A69D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A69D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A69DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A69E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A69E4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A69E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A69EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A69F0: 4BAAF251  bl 0x82d55c40
	ctx.lr = 0x832A69F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A69F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A69F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A69FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A00: 4E800020  blr
	return;
}

pub fn sub_832A6A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A28 size=80
	// 832A6A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A6A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A38: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6A3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A6A40: 3BEBB0A0  addi r31, r11, -0x4f60
	ctx.r[31].s64 = ctx.r[11].s64 + -20320;
	// 832A6A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A6A48: 4801324D  bl 0x832b9c94
	ctx.lr = 0x832A6A4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C94);
	// 832A6A4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832A6A50: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A6A54: 386A97D0  addi r3, r10, -0x6830
	ctx.r[3].s64 = ctx.r[10].s64 + -26672;
	// 832A6A58: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 832A6A5C: 4BA034C5  bl 0x82ca9f20
	ctx.lr = 0x832A6A60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A6A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A6A70: 4E800020  blr
	return;
}

pub fn sub_832A6A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A78 size=112
	// 832A6A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A84: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6A88: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6A8C: 38EB8FB8  addi r7, r11, -0x7048
	ctx.r[7].s64 = ctx.r[11].s64 + -28744;
	// 832A6A90: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A6A94: 388A9090  addi r4, r10, -0x6f70
	ctx.r[4].s64 = ctx.r[10].s64 + -28528;
	// 832A6A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6A9C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6AA8: 386AB0DC  addi r3, r10, -0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + -20260;
	// 832A6AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6AC4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 832A6AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6AD0: 4BAAF171  bl 0x82d55c40
	ctx.lr = 0x832A6AD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6AE0: 4E800020  blr
	return;
}

pub fn sub_832A6AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6AE8 size=112
	// 832A6AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6AF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6AF8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6AFC: 38EB90B0  addi r7, r11, -0x6f50
	ctx.r[7].s64 = ctx.r[11].s64 + -28496;
	// 832A6B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6B04: 388A9170  addi r4, r10, -0x6e90
	ctx.r[4].s64 = ctx.r[10].s64 + -28304;
	// 832A6B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B0C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B18: 386AB10C  addi r3, r10, -0x4ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -20212;
	// 832A6B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6B34: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6B40: 4BAAF101  bl 0x82d55c40
	ctx.lr = 0x832A6B44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6B50: 4E800020  blr
	return;
}

pub fn sub_832A6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6B58 size=112
	// 832A6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6B64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6B68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6B6C: 38EB90E0  addi r7, r11, -0x6f20
	ctx.r[7].s64 = ctx.r[11].s64 + -28448;
	// 832A6B70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 832A6B74: 388A9178  addi r4, r10, -0x6e88
	ctx.r[4].s64 = ctx.r[10].s64 + -28296;
	// 832A6B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B88: 386AB13C  addi r3, r10, -0x4ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -20164;
	// 832A6B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6BA4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6BB0: 4BAAF091  bl 0x82d55c40
	ctx.lr = 0x832A6BB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6BC0: 4E800020  blr
	return;
}

pub fn sub_832A6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6BC8 size=144
	// 832A6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6BD4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BD8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6BDC: 392A9248  addi r9, r10, -0x6db8
	ctx.r[9].s64 = ctx.r[10].s64 + -28088;
	// 832A6BE0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BE4: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 832A6BE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 832A6BEC: 388A92B8  addi r4, r10, -0x6d48
	ctx.r[4].s64 = ctx.r[10].s64 + -27976;
	// 832A6BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6BF4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6BFC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6C00: 386AB16C  addi r3, r10, -0x4e94
	ctx.r[3].s64 = ctx.r[10].s64 + -20116;
	// 832A6C04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6C08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832A6C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6C24: 4BAAF01D  bl 0x82d55c40
	ctx.lr = 0x832A6C28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6C34: 4E800020  blr
	return;
	// 832A6C38: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6C3C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6C40: 396B18BC  addi r11, r11, 0x18bc
	ctx.r[11].s64 = ctx.r[11].s64 + 6332;
	// 832A6C44: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6C48: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6C4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6C50: 4E800020  blr
	return;
}

pub fn sub_832A6C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6C58 size=112
	// 832A6C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6C64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6C68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6C6C: 38EB93D4  addi r7, r11, -0x6c2c
	ctx.r[7].s64 = ctx.r[11].s64 + -27692;
	// 832A6C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6C74: 388A9448  addi r4, r10, -0x6bb8
	ctx.r[4].s64 = ctx.r[10].s64 + -27576;
	// 832A6C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6C7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6C88: 386AB1A4  addi r3, r10, -0x4e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20060;
	// 832A6C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6CA4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6CB0: 4BAAEF91  bl 0x82d55c40
	ctx.lr = 0x832A6CB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6CC0: 4E800020  blr
	return;
}

pub fn sub_832A6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6CC8 size=136
	// 832A6CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6CD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6CD8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6CDC: 38EB9404  addi r7, r11, -0x6bfc
	ctx.r[7].s64 = ctx.r[11].s64 + -27644;
	// 832A6CE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A6CE4: 388A946C  addi r4, r10, -0x6b94
	ctx.r[4].s64 = ctx.r[10].s64 + -27540;
	// 832A6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6CEC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6CF8: 386AB1D4  addi r3, r10, -0x4e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -20012;
	// 832A6CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6D14: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6D20: 4BAAEF21  bl 0x82d55c40
	ctx.lr = 0x832A6D24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6D30: 4E800020  blr
	return;
}

pub fn sub_832A6D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6D50 size=112
	// 832A6D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6D5C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D60: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6D64: 392A9434  addi r9, r10, -0x6bcc
	ctx.r[9].s64 = ctx.r[10].s64 + -27596;
	// 832A6D68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D6C: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 832A6D70: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A6D74: 388A9488  addi r4, r10, -0x6b78
	ctx.r[4].s64 = ctx.r[10].s64 + -27512;
	// 832A6D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6D7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6D84: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6D88: 386AB204  addi r3, r10, -0x4dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19964;
	// 832A6D8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6D90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A6D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6DAC: 4BAAEE95  bl 0x82d55c40
	ctx.lr = 0x832A6DB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6DBC: 4E800020  blr
	return;
}

pub fn sub_832A6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6DC0 size=112
	// 832A6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6DCC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6DD0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6DD4: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 832A6DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6DDC: 388A97A8  addi r4, r10, -0x6858
	ctx.r[4].s64 = ctx.r[10].s64 + -26712;
	// 832A6DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6DE4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6DF0: 386AB234  addi r3, r10, -0x4dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -19916;
	// 832A6DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E0C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E18: 4BAAEE29  bl 0x82d55c40
	ctx.lr = 0x832A6E1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E28: 4E800020  blr
	return;
}

pub fn sub_832A6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6E30 size=112
	// 832A6E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6E3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6E40: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6E44: 38EB9828  addi r7, r11, -0x67d8
	ctx.r[7].s64 = ctx.r[11].s64 + -26584;
	// 832A6E48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6E4C: 388A98E8  addi r4, r10, -0x6718
	ctx.r[4].s64 = ctx.r[10].s64 + -26392;
	// 832A6E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6E54: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6E60: 386AB264  addi r3, r10, -0x4d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19868;
	// 832A6E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E7C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E88: 4BAAEDB9  bl 0x82d55c40
	ctx.lr = 0x832A6E8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E98: 4E800020  blr
	return;
}

pub fn sub_832A6EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6EA0 size=112
	// 832A6EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6EAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6EB0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6EB4: 38EB9888  addi r7, r11, -0x6778
	ctx.r[7].s64 = ctx.r[11].s64 + -26488;
	// 832A6EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6EBC: 388A9900  addi r4, r10, -0x6700
	ctx.r[4].s64 = ctx.r[10].s64 + -26368;
	// 832A6EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6EC4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6ED0: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 832A6ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6EEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6EF8: 4BAAED49  bl 0x82d55c40
	ctx.lr = 0x832A6EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6F08: 4E800020  blr
	return;
}

pub fn sub_832A6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F10 size=12
	// 832A6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832A6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F70 size=144
	// 832A6F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6F7C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6F80: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6F84: 38EB9968  addi r7, r11, -0x6698
	ctx.r[7].s64 = ctx.r[11].s64 + -26264;
	// 832A6F88: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 832A6F8C: 388A99E0  addi r4, r10, -0x6620
	ctx.r[4].s64 = ctx.r[10].s64 + -26144;
	// 832A6F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6F94: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6FA0: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 832A6FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6FBC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A6FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6FC8: 4BAAEC79  bl 0x82d55c40
	ctx.lr = 0x832A6FCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A6FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6FD8: 4E800020  blr
	return;
}

pub fn sub_832A7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7000 size=112
	// 832A7000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A700C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7010: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7014: 38EBA0D0  addi r7, r11, -0x5f30
	ctx.r[7].s64 = ctx.r[11].s64 + -24368;
	// 832A7018: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 832A701C: 388AA1D8  addi r4, r10, -0x5e28
	ctx.r[4].s64 = ctx.r[10].s64 + -24104;
	// 832A7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7024: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A702C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7030: 386AB324  addi r3, r10, -0x4cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19676;
	// 832A7034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A7038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A703C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A7044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A704C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A7050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A7054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7058: 4BAAEBE9  bl 0x82d55c40
	ctx.lr = 0x832A705C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7068: 4E800020  blr
	return;
}

pub fn sub_832A7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7070 size=112
	// 832A7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A707C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7080: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7084: 38EBA280  addi r7, r11, -0x5d80
	ctx.r[7].s64 = ctx.r[11].s64 + -23936;
	// 832A7088: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A708C: 388AA358  addi r4, r10, -0x5ca8
	ctx.r[4].s64 = ctx.r[10].s64 + -23720;
	// 832A7090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7094: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A709C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A70A0: 386AB354  addi r3, r10, -0x4cac
	ctx.r[3].s64 = ctx.r[10].s64 + -19628;
	// 832A70A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A70A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A70AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A70B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A70B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A70B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A70BC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A70C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A70C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A70C8: 4BAAEB79  bl 0x82d55c40
	ctx.lr = 0x832A70CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82D55C40);
	// 832A70CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A70D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A70D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A70D8: 4E800020  blr
	return;
}

pub fn sub_832A70E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A70E0 size=88
	// 832A70E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A70E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A70E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A70EC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A70F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A70F4: 388BA674  addi r4, r11, -0x598c
	ctx.r[4].s64 = ctx.r[11].s64 + -22924;
	// 832A70F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A70FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7100: 386BB38C  addi r3, r11, -0x4c74
	ctx.r[3].s64 = ctx.r[11].s64 + -19572;
	// 832A7104: 4BBE29D5  bl 0x82e89ad8
	ctx.lr = 0x832A7108;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E89AD8);
	// 832A7108: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A710C: 386B9830  addi r3, r11, -0x67d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26576;
	// 832A7110: 4BA02E11  bl 0x82ca9f20
	ctx.lr = 0x832A7114;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A7114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A711C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7120: 4E800020  blr
	return;
}

pub fn sub_832A7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7138 size=312
	// 832A7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7144: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7148: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 832A714C: 388AA6AC  addi r4, r10, -0x5954
	ctx.r[4].s64 = ctx.r[10].s64 + -22868;
	// 832A7150: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7158: 386ABA88  addi r3, r10, -0x4578
	ctx.r[3].s64 = ctx.r[10].s64 + -17784;
	// 832A715C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7160: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A7164: 38ABF7D8  addi r5, r11, -0x828
	ctx.r[5].s64 = ctx.r[11].s64 + -2088;
	// 832A7168: 4BBDE659  bl 0x82e857c0
	ctx.lr = 0x832A716C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832A716C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A7170: 386B98B0  addi r3, r11, -0x6750
	ctx.r[3].s64 = ctx.r[11].s64 + -26448;
	// 832A7174: 4BA02DAD  bl 0x82ca9f20
	ctx.lr = 0x832A7178;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832A7178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A717C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7184: 4E800020  blr
	return;
	// 832A7188: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A718C: 386B98C8  addi r3, r11, -0x6738
	ctx.r[3].s64 = ctx.r[11].s64 + -26424;
	// 832A7190: 4BA02D90  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_832A7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7270 size=72
	// 832A7270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A727C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7280: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7284: 3BEB72DC  addi r31, r11, 0x72dc
	ctx.r[31].s64 = ctx.r[11].s64 + 29404;
	// 832A7288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A728C: 4B76DB05  bl 0x82a14d90
	ctx.lr = 0x832A7290;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A14D90);
	// 832A7290: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7294: 4AF74AA5  bl 0x8221bd38
	ctx.lr = 0x832A7298;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A729C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A72A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A72A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A72A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A72AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A72B0: 4E800020  blr
	return;
}

pub fn sub_832A72B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A72B8 size=496
	// 832A72B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A72BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A72C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A72C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A72C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A72CC: 3BEB72E8  addi r31, r11, 0x72e8
	ctx.r[31].s64 = ctx.r[11].s64 + 29416;
	// 832A72D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A72D4: 4B058B1D  bl 0x822ffdf0
	ctx.lr = 0x832A72D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x822FFDF0);
	// 832A72D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A72DC: 4AF74A5D  bl 0x8221bd38
	ctx.lr = 0x832A72E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A72E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A72E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A72E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A72EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A72F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A72F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A72F8: 4E800020  blr
	return;
}

pub fn sub_832A74A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A74A8 size=104
	// 832A74A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A74AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A74B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A74B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A74B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A74BC: 3BEB7498  addi r31, r11, 0x7498
	ctx.r[31].s64 = ctx.r[11].s64 + 29848;
	// 832A74C0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A74C4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A74C8: 419A0018  beq cr6, 0x832a74e0
	if ctx.cr[6].eq {
	pc = 0x832A74E0; continue 'dispatch;
	}
	// 832A74CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A74D0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A74D4: 4B09B335  bl 0x82342808
	ctx.lr = 0x832A74D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82342808);
	// 832A74D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A74DC: 4AF7485D  bl 0x8221bd38
	ctx.lr = 0x832A74E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A74E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A74E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A74E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A74EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A74F0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A74F4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A74F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A74FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7508: 4E800020  blr
	return;
}

pub fn sub_832A7510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7510 size=104
	// 832A7510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A751C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7520: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7524: 3BEB74A8  addi r31, r11, 0x74a8
	ctx.r[31].s64 = ctx.r[11].s64 + 29864;
	// 832A7528: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A752C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7530: 419A0018  beq cr6, 0x832a7548
	if ctx.cr[6].eq {
	pc = 0x832A7548; continue 'dispatch;
	}
	// 832A7534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7538: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A753C: 4B09B9CD  bl 0x82342f08
	ctx.lr = 0x832A7540;
	crate::recompiler::externs::call(&mut ctx, base, 0x82342F08);
	// 832A7540: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7544: 4AF747F5  bl 0x8221bd38
	ctx.lr = 0x832A7548;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A7548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A754C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7550: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7554: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A7558: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A755C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A7560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A756C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7570: 4E800020  blr
	return;
}

pub fn sub_832A7578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7578 size=248
	// 832A7578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A757C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7588: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A758C: 3BEB74B8  addi r31, r11, 0x74b8
	ctx.r[31].s64 = ctx.r[11].s64 + 29880;
	// 832A7590: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7594: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7598: 419A0018  beq cr6, 0x832a75b0
	if ctx.cr[6].eq {
	pc = 0x832A75B0; continue 'dispatch;
	}
	// 832A759C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A75A0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A75A4: 4B09B965  bl 0x82342f08
	ctx.lr = 0x832A75A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82342F08);
	// 832A75A8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A75AC: 4AF7478D  bl 0x8221bd38
	ctx.lr = 0x832A75B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A75B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A75B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A75B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A75BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A75C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A75C4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A75C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A75CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A75D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A75D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A75D8: 4E800020  blr
	return;
}

pub fn sub_832A7670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7670 size=104
	// 832A7670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7674: 4BA01D95  bl 0x82ca9408
	ctx.lr = 0x832A7678;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A7678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A767C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7680: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A7684: 396B74E0  addi r11, r11, 0x74e0
	ctx.r[11].s64 = ctx.r[11].s64 + 29920;
	// 832A7688: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A768C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A7690: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7694: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7698: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A769C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A76A0: 4AF1F0C9  bl 0x821c6768
	ctx.lr = 0x832A76A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A76A4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A76A8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A76AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A76B0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A76B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A76B8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A76BC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A76C0: 4082FFE8  bne 0x832a76a8
	if !ctx.cr[0].eq {
	pc = 0x832A76A8; continue 'dispatch;
	}
	// 832A76C4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A76C8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A76CC: 4080FFCC  bge 0x832a7698
	if !ctx.cr[0].lt {
	pc = 0x832A7698; continue 'dispatch;
	}
	// 832A76D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A76D4: 4BA01D84  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A76D8 size=104
	// 832A76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A76DC: 4BA01D2D  bl 0x82ca9408
	ctx.lr = 0x832A76E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A76E4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A76E8: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A76EC: 396B7510  addi r11, r11, 0x7510
	ctx.r[11].s64 = ctx.r[11].s64 + 29968;
	// 832A76F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A76F4: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A76F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A76FC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7700: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7708: 4AF1F061  bl 0x821c6768
	ctx.lr = 0x832A770C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A770C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7710: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7714: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7718: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A771C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7720: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7724: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7728: 4082FFE8  bne 0x832a7710
	if !ctx.cr[0].eq {
	pc = 0x832A7710; continue 'dispatch;
	}
	// 832A772C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7730: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7734: 4080FFCC  bge 0x832a7700
	if !ctx.cr[0].lt {
	pc = 0x832A7700; continue 'dispatch;
	}
	// 832A7738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A773C: 4BA01D1C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A7740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7740 size=104
	// 832A7740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7744: 4BA01CC5  bl 0x82ca9408
	ctx.lr = 0x832A7748;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A7748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A774C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7750: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A7754: 396B7540  addi r11, r11, 0x7540
	ctx.r[11].s64 = ctx.r[11].s64 + 30016;
	// 832A7758: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A775C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A7760: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7764: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7768: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A776C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7770: 4AF1EFF9  bl 0x821c6768
	ctx.lr = 0x832A7774;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7774: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7778: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A777C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7780: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7784: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7788: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A778C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7790: 4082FFE8  bne 0x832a7778
	if !ctx.cr[0].eq {
	pc = 0x832A7778; continue 'dispatch;
	}
	// 832A7794: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7798: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A779C: 4080FFCC  bge 0x832a7768
	if !ctx.cr[0].lt {
	pc = 0x832A7768; continue 'dispatch;
	}
	// 832A77A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A77A4: 4BA01CB4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A77A8 size=488
	// 832A77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A77AC: 4BA01C5D  bl 0x82ca9408
	ctx.lr = 0x832A77B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A77B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A77B4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A77B8: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832A77BC: 396B3830  addi r11, r11, 0x3830
	ctx.r[11].s64 = ctx.r[11].s64 + 14384;
	// 832A77C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A77C4: 3BEB004C  addi r31, r11, 0x4c
	ctx.r[31].s64 = ctx.r[11].s64 + 76;
	// 832A77C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A77CC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A77D0: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832A77D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A77D8: 4AF1EF91  bl 0x821c6768
	ctx.lr = 0x832A77DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A77DC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A77E0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A77E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A77E8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A77EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A77F0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A77F4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A77F8: 4082FFE8  bne 0x832a77e0
	if !ctx.cr[0].eq {
	pc = 0x832A77E0; continue 'dispatch;
	}
	// 832A77FC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7800: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7804: 4080FFCC  bge 0x832a77d0
	if !ctx.cr[0].lt {
	pc = 0x832A77D0; continue 'dispatch;
	}
	// 832A7808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A780C: 4BA01C4C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832A7810: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7814: 386B7570  addi r3, r11, 0x7570
	ctx.r[3].s64 = ctx.r[11].s64 + 30064;
	// 832A7818: 4AF6D5C0  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A7990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7990 size=448
	// 832A7990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A799C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A79A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A79A4: 3BEB75E8  addi r31, r11, 0x75e8
	ctx.r[31].s64 = ctx.r[11].s64 + 30184;
	// 832A79A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A79AC: 4B07AE3D  bl 0x823227e8
	ctx.lr = 0x832A79B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x823227E8);
	// 832A79B0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A79B4: 4AF74385  bl 0x8221bd38
	ctx.lr = 0x832A79B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A79B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A79BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A79C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A79C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A79C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A79CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A79D0: 4E800020  blr
	return;
}

pub fn sub_832A7B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7B50 size=120
	// 832A7B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7B60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B64: 3BEB76AC  addi r31, r11, 0x76ac
	ctx.r[31].s64 = ctx.r[11].s64 + 30380;
	// 832A7B68: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7B70: 419A0018  beq cr6, 0x832a7b88
	if ctx.cr[6].eq {
	pc = 0x832A7B88; continue 'dispatch;
	}
	// 832A7B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7B78: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7B7C: 4B7EA385  bl 0x82a91f00
	ctx.lr = 0x832A7B80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A91F00);
	// 832A7B80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B84: 4AF741B5  bl 0x8221bd38
	ctx.lr = 0x832A7B88;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A7B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7B90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7B94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A7B98: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A7B9C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A7BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7BB0: 4E800020  blr
	return;
}

pub fn sub_832A7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7BC8 size=104
	// 832A7BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7BCC: 4BA0183D  bl 0x82ca9408
	ctx.lr = 0x832A7BD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A7BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7BD4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BD8: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7BDC: 396B76C0  addi r11, r11, 0x76c0
	ctx.r[11].s64 = ctx.r[11].s64 + 30400;
	// 832A7BE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7BE4: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7BE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BEC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7BF0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7BF8: 4AF1EB71  bl 0x821c6768
	ctx.lr = 0x832A7BFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7BFC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7C00: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C08: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C10: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C14: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C18: 4082FFE8  bne 0x832a7c00
	if !ctx.cr[0].eq {
	pc = 0x832A7C00; continue 'dispatch;
	}
	// 832A7C1C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C20: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C24: 4080FFCC  bge 0x832a7bf0
	if !ctx.cr[0].lt {
	pc = 0x832A7BF0; continue 'dispatch;
	}
	// 832A7C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C2C: 4BA0182C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7C30 size=264
	// 832A7C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7C34: 4BA017D5  bl 0x82ca9408
	ctx.lr = 0x832A7C38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A7C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7C3C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C40: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7C44: 396B7700  addi r11, r11, 0x7700
	ctx.r[11].s64 = ctx.r[11].s64 + 30464;
	// 832A7C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7C4C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7C50: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C54: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A7C58: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7C60: 4AF1EB09  bl 0x821c6768
	ctx.lr = 0x832A7C64;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7C64: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A7C68: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C6C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C70: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C78: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C7C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C80: 4082FFE8  bne 0x832a7c68
	if !ctx.cr[0].eq {
	pc = 0x832A7C68; continue 'dispatch;
	}
	// 832A7C84: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C88: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C8C: 4080FFCC  bge 0x832a7c58
	if !ctx.cr[0].lt {
	pc = 0x832A7C58; continue 'dispatch;
	}
	// 832A7C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C94: 4BA017C4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832A7C98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C9C: 386B7794  addi r3, r11, 0x7794
	ctx.r[3].s64 = ctx.r[11].s64 + 30612;
	// 832A7CA0: 4AF6D138  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7D38 size=272
	// 832A7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7D48: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7D4C: 3BEB2144  addi r31, r11, 0x2144
	ctx.r[31].s64 = ctx.r[11].s64 + 8516;
	// 832A7D50: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832A7D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7D58: 419A0014  beq cr6, 0x832a7d6c
	if ctx.cr[6].eq {
	pc = 0x832A7D6C; continue 'dispatch;
	}
	// 832A7D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7D60: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7D64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7D68: 4E800421  bctrl
	ctx.lr = 0x832A7D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7D6C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832A7D70: 4B0CE171  bl 0x82375ee0
	ctx.lr = 0x832A7D74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82375EE0);
	// 832A7D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7D78: 4B0CD309  bl 0x82375080
	ctx.lr = 0x832A7D7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82375080);
	// 832A7D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7D8C: 4E800020  blr
	return;
	// 832A7D90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D94: 386B77C0  addi r3, r11, 0x77c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30656;
	// 832A7D98: 4AF6D040  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7E48 size=176
	// 832A7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7E58: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7E5C: 3BEB3978  addi r31, r11, 0x3978
	ctx.r[31].s64 = ctx.r[11].s64 + 14712;
	// 832A7E60: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 832A7E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E68: 419A0014  beq cr6, 0x832a7e7c
	if ctx.cr[6].eq {
	pc = 0x832A7E7C; continue 'dispatch;
	}
	// 832A7E6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E70: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E78: 4E800421  bctrl
	ctx.lr = 0x832A7E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7E7C: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 832A7E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E84: 419A0014  beq cr6, 0x832a7e98
	if ctx.cr[6].eq {
	pc = 0x832A7E98; continue 'dispatch;
	}
	// 832A7E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E8C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E94: 4E800421  bctrl
	ctx.lr = 0x832A7E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7E98: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832A7E9C: 4B0D8285  bl 0x82380120
	ctx.lr = 0x832A7EA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82380120);
	// 832A7EA0: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7EA8: 419A0018  beq cr6, 0x832a7ec0
	if ctx.cr[6].eq {
	pc = 0x832A7EC0; continue 'dispatch;
	}
	// 832A7EAC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832A7EB0: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 832A7EB4: 4B0D889D  bl 0x82380750
	ctx.lr = 0x832A7EB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82380750);
	// 832A7EB8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EBC: 4AF73E7D  bl 0x8221bd38
	ctx.lr = 0x832A7EC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A7EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7EC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7EC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7ECC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 832A7ED0: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 832A7ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7ED8: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 832A7EDC: 4B9CADE5  bl 0x82c72cc0
	ctx.lr = 0x832A7EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C72CC0);
	// 832A7EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7EF0: 4E800020  blr
	return;
}

pub fn sub_832A7EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7EF8 size=224
	// 832A7EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7EFC: 4BA0150D  bl 0x82ca9408
	ctx.lr = 0x832A7F00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A7F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7F04: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F08: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 832A7F0C: 396B7810  addi r11, r11, 0x7810
	ctx.r[11].s64 = ctx.r[11].s64 + 30736;
	// 832A7F10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7F14: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832A7F18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F1C: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	// 832A7F20: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832A7F24: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7F2C: 419A0014  beq cr6, 0x832a7f40
	if ctx.cr[6].eq {
	pc = 0x832A7F40; continue 'dispatch;
	}
	// 832A7F30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7F38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7F3C: 4E800421  bctrl
	ctx.lr = 0x832A7F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832A7F40: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 832A7F44: 4AF1E825  bl 0x821c6768
	ctx.lr = 0x832A7F48;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7F48: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832A7F4C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7F50: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F54: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7F58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7F5C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F60: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F64: 4082FFE8  bne 0x832a7f4c
	if !ctx.cr[0].eq {
	pc = 0x832A7F4C; continue 'dispatch;
	}
	// 832A7F68: 939FFFFC  stw r28, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[28].u32 ) };
	// 832A7F6C: 387FFFF4  addi r3, r31, -0xc
	ctx.r[3].s64 = ctx.r[31].s64 + -12;
	// 832A7F70: 4AF1E7F9  bl 0x821c6768
	ctx.lr = 0x832A7F74;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7F74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 832A7F78: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832A7F7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F80: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832A7F84: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832A7F88: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F8C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F90: 4082FFE8  bne 0x832a7f78
	if !ctx.cr[0].eq {
	pc = 0x832A7F78; continue 'dispatch;
	}
	// 832A7F94: 939FFFF4  stw r28, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[28].u32 ) };
	// 832A7F98: 387FFFF0  addi r3, r31, -0x10
	ctx.r[3].s64 = ctx.r[31].s64 + -16;
	// 832A7F9C: 4AF1E7CD  bl 0x821c6768
	ctx.lr = 0x832A7FA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A7FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832A7FA4: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 832A7FA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FAC: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 832A7FB0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832A7FB4: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7FB8: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FBC: 4082FFE8  bne 0x832a7fa4
	if !ctx.cr[0].eq {
	pc = 0x832A7FA4; continue 'dispatch;
	}
	// 832A7FC0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832A7FC4: 939FFFF0  stw r28, -0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), ctx.r[28].u32 ) };
	// 832A7FC8: 4080FF58  bge 0x832a7f20
	if !ctx.cr[0].lt {
	pc = 0x832A7F20; continue 'dispatch;
	}
	// 832A7FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7FD0: 4BA01488  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A7FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7FD8 size=616
	// 832A7FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7FE8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832A7FEC: 3BEB9930  addi r31, r11, -0x66d0
	ctx.r[31].s64 = ctx.r[11].s64 + -26320;
	// 832A7FF0: 806B9930  lwz r3, -0x66d0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26320 as u32) ) } as u64;
	// 832A7FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7FF8: 419A0034  beq cr6, 0x832a802c
	if ctx.cr[6].eq {
	pc = 0x832A802C; continue 'dispatch;
	}
	// 832A7FFC: 4AF73D3D  bl 0x8221bd38
	ctx.lr = 0x832A8000;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A8000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8008: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 832A800C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A8010: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A8014: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832A8018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8028: 4E800020  blr
	return;
	// 832A802C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8030: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 832A8034: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8038: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A803C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A804C: 4E800020  blr
	return;
	// 832A8050: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8054: 386B7888  addi r3, r11, 0x7888
	ctx.r[3].s64 = ctx.r[11].s64 + 30856;
	// 832A8058: 4AF6CD80  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A8240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8240 size=12
	// 832A8240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8244: 4BA011BD  bl 0x82ca9400
	ctx.lr = 0x832A8248;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 832A8248: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832A8300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8300 size=424
	// 832A8300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A830C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8310: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8314: 3BEB7910  addi r31, r11, 0x7910
	ctx.r[31].s64 = ctx.r[11].s64 + 30992;
	// 832A8318: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A831C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A8320: 419A0008  beq cr6, 0x832a8328
	if ctx.cr[6].eq {
	pc = 0x832A8328; continue 'dispatch;
	}
	// 832A8324: 4AF73A15  bl 0x8221bd38
	ctx.lr = 0x832A8328;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A832C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8330: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8334: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8338: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A833C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A834C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8350: 4E800020  blr
	return;
}

pub fn sub_832A84A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A84A8 size=184
	// 832A84A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A84AC: 4BA00F5D  bl 0x82ca9408
	ctx.lr = 0x832A84B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A84B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A84B4: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A84B8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832A84BC: 396B6E24  addi r11, r11, 0x6e24
	ctx.r[11].s64 = ctx.r[11].s64 + 28196;
	// 832A84C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A84C4: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832A84C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A84CC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A84D0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A84D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A84D8: 4AF1E291  bl 0x821c6768
	ctx.lr = 0x832A84DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A84DC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A84E0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A84E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A84E8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A84EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A84F0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A84F4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A84F8: 4082FFE8  bne 0x832a84e0
	if !ctx.cr[0].eq {
	pc = 0x832A84E0; continue 'dispatch;
	}
	// 832A84FC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A8500: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A8504: 4080FFCC  bge 0x832a84d0
	if !ctx.cr[0].lt {
	pc = 0x832A84D0; continue 'dispatch;
	}
	// 832A8508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A850C: 4BA00F4C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832A8510: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8514: 386B79B0  addi r3, r11, 0x79b0
	ctx.r[3].s64 = ctx.r[11].s64 + 31152;
	// 832A8518: 4AF6C8C0  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A8560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8560 size=72
	// 832A8560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A856C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8570: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8574: 3BEB79C4  addi r31, r11, 0x79c4
	ctx.r[31].s64 = ctx.r[11].s64 + 31172;
	// 832A8578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A857C: 4B07A26D  bl 0x823227e8
	ctx.lr = 0x832A8580;
	crate::recompiler::externs::call(&mut ctx, base, 0x823227E8);
	// 832A8580: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8584: 4AF737B5  bl 0x8221bd38
	ctx.lr = 0x832A8588;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A8588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A858C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A859C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A85A0: 4E800020  blr
	return;
}

pub fn sub_832A85A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A85A8 size=216
	// 832A85A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A85AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A85B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A85B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A85B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A85BC: 3BEB79D0  addi r31, r11, 0x79d0
	ctx.r[31].s64 = ctx.r[11].s64 + 31184;
	// 832A85C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A85C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A85C8: 419A0008  beq cr6, 0x832a85d0
	if ctx.cr[6].eq {
	pc = 0x832A85D0; continue 'dispatch;
	}
	// 832A85CC: 4AF7376D  bl 0x8221bd38
	ctx.lr = 0x832A85D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A85D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A85D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A85D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A85DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A85E0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A85E4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A85E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A85EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A85F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A85F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A85F8: 4E800020  blr
	return;
}

pub fn sub_832A8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8680 size=152
	// 832A8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A868C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8690: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A8694: 3BEB3598  addi r31, r11, 0x3598
	ctx.r[31].s64 = ctx.r[11].s64 + 13720;
	// 832A8698: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 832A869C: 4B11A435  bl 0x823c2ad0
	ctx.lr = 0x832A86A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x823C2AD0);
	// 832A86A0: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 832A86A4: 4AF73695  bl 0x8221bd38
	ctx.lr = 0x832A86A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A86A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A86AC: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 832A86B0: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A86B4: 4AF6C725  bl 0x82214dd8
	ctx.lr = 0x832A86B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832A86B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832A86BC: 4AF6C71D  bl 0x82214dd8
	ctx.lr = 0x832A86C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832A86C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A86C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A86C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A86CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A86D0: 4E800020  blr
	return;
}

pub fn sub_832A8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8718 size=104
	// 832A8718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A871C: 4BA00CED  bl 0x82ca9408
	ctx.lr = 0x832A8720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A8720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8724: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8728: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A872C: 396B7A80  addi r11, r11, 0x7a80
	ctx.r[11].s64 = ctx.r[11].s64 + 31360;
	// 832A8730: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A8734: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A8738: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A873C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A8740: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A8744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8748: 4AF1E021  bl 0x821c6768
	ctx.lr = 0x832A874C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A874C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A8750: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A8754: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A8758: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A875C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A8760: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A8764: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A8768: 4082FFE8  bne 0x832a8750
	if !ctx.cr[0].eq {
	pc = 0x832A8750; continue 'dispatch;
	}
	// 832A876C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A8770: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A8774: 4080FFCC  bge 0x832a8740
	if !ctx.cr[0].lt {
	pc = 0x832A8740; continue 'dispatch;
	}
	// 832A8778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A877C: 4BA00CDC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A8780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8780 size=1352
	// 832A8780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8784: 4BA00C85  bl 0x82ca9408
	ctx.lr = 0x832A8788;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A8788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A878C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8790: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A8794: 396B7AC0  addi r11, r11, 0x7ac0
	ctx.r[11].s64 = ctx.r[11].s64 + 31424;
	// 832A8798: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A879C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A87A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A87A4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A87A8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A87AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A87B0: 4AF1DFB9  bl 0x821c6768
	ctx.lr = 0x832A87B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A87B4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A87B8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A87BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A87C0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A87C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A87C8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A87CC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A87D0: 4082FFE8  bne 0x832a87b8
	if !ctx.cr[0].eq {
	pc = 0x832A87B8; continue 'dispatch;
	}
	// 832A87D4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A87D8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A87DC: 4080FFCC  bge 0x832a87a8
	if !ctx.cr[0].lt {
	pc = 0x832A87A8; continue 'dispatch;
	}
	// 832A87E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A87E4: 4BA00C74  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832A87E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A87EC: 386B7B00  addi r3, r11, 0x7b00
	ctx.r[3].s64 = ctx.r[11].s64 + 31488;
	// 832A87F0: 4AF6C5E8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A8CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8CC8 size=104
	// 832A8CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A8CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8CD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8CDC: 3BEB7D2C  addi r31, r11, 0x7d2c
	ctx.r[31].s64 = ctx.r[11].s64 + 32044;
	// 832A8CE0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8CE4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A8CE8: 419A0018  beq cr6, 0x832a8d00
	if ctx.cr[6].eq {
	pc = 0x832A8D00; continue 'dispatch;
	}
	// 832A8CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8CF0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A8CF4: 4B1371D5  bl 0x823dfec8
	ctx.lr = 0x832A8CF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x823DFEC8);
	// 832A8CF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8CFC: 4AF7303D  bl 0x8221bd38
	ctx.lr = 0x832A8D00;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A8D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8D04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8D08: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8D0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8D10: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A8D14: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8D28: 4E800020  blr
	return;
}

pub fn sub_832A8D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A8D30 size=1040
	// 832A8D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A8D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A8D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A8D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A8D40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8D44: 3BEB7D3C  addi r31, r11, 0x7d3c
	ctx.r[31].s64 = ctx.r[11].s64 + 32060;
	// 832A8D48: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8D4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A8D50: 419A0018  beq cr6, 0x832a8d68
	if ctx.cr[6].eq {
	pc = 0x832A8D68; continue 'dispatch;
	}
	// 832A8D54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A8D58: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A8D5C: 4B13716D  bl 0x823dfec8
	ctx.lr = 0x832A8D60;
	crate::recompiler::externs::call(&mut ctx, base, 0x823DFEC8);
	// 832A8D60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A8D64: 4AF72FD5  bl 0x8221bd38
	ctx.lr = 0x832A8D68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832A8D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8D70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A8D74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8D78: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A8D7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A8D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8D90: 4E800020  blr
	return;
}

pub fn sub_832A99E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A99E0 size=104
	// 832A99E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A99E4: 4B9FFA25  bl 0x82ca9408
	ctx.lr = 0x832A99E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A99E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A99EC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A99F0: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832A99F4: 396B8130  addi r11, r11, -0x7ed0
	ctx.r[11].s64 = ctx.r[11].s64 + -32464;
	// 832A99F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A99FC: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832A9A00: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9A04: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9A08: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9A10: 4AF1CD59  bl 0x821c6768
	ctx.lr = 0x832A9A14;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9A14: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9A18: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9A1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9A20: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9A24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9A28: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9A2C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9A30: 4082FFE8  bne 0x832a9a18
	if !ctx.cr[0].eq {
	pc = 0x832A9A18; continue 'dispatch;
	}
	// 832A9A34: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9A38: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9A3C: 4080FFCC  bge 0x832a9a08
	if !ctx.cr[0].lt {
	pc = 0x832A9A08; continue 'dispatch;
	}
	// 832A9A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9A44: 4B9FFA14  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9A48 size=488
	// 832A9A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9A4C: 4B9FF9BD  bl 0x82ca9408
	ctx.lr = 0x832A9A50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9A54: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9A58: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832A9A5C: 396B814C  addi r11, r11, -0x7eb4
	ctx.r[11].s64 = ctx.r[11].s64 + -32436;
	// 832A9A60: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9A64: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832A9A68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9A6C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9A70: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9A78: 4AF1CCF1  bl 0x821c6768
	ctx.lr = 0x832A9A7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9A7C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9A80: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9A84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9A88: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9A8C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9A90: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9A94: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9A98: 4082FFE8  bne 0x832a9a80
	if !ctx.cr[0].eq {
	pc = 0x832A9A80; continue 'dispatch;
	}
	// 832A9A9C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9AA0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9AA4: 4080FFCC  bge 0x832a9a70
	if !ctx.cr[0].lt {
	pc = 0x832A9A70; continue 'dispatch;
	}
	// 832A9AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9AAC: 4B9FF9AC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832A9AB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9AB4: 386B8168  addi r3, r11, -0x7e98
	ctx.r[3].s64 = ctx.r[11].s64 + -32408;
	// 832A9AB8: 4AF6B320  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832A9C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9C30 size=192
	// 832A9C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A9C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A9C3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9C40: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832A9C44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832A9C48: 3BEA20B8  addi r31, r10, 0x20b8
	ctx.r[31].s64 = ctx.r[10].s64 + 8376;
	// 832A9C4C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832A9C50: 396B7E44  addi r11, r11, 0x7e44
	ctx.r[11].s64 = ctx.r[11].s64 + 32324;
	// 832A9C54: 394A7E38  addi r10, r10, 0x7e38
	ctx.r[10].s64 = ctx.r[10].s64 + 32312;
	// 832A9C58: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832A9C5C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 832A9C60: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 832A9C64: 4AF6B175  bl 0x82214dd8
	ctx.lr = 0x832A9C68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832A9C68: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832A9C6C: 4AF6B16D  bl 0x82214dd8
	ctx.lr = 0x832A9C70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832A9C70: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832A9C74: 4AF6B165  bl 0x82214dd8
	ctx.lr = 0x832A9C78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832A9C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A9C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A9C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A9C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A9C88: 4E800020  blr
	return;
}

pub fn sub_832A9CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9CF0 size=104
	// 832A9CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9CF4: 4B9FF715  bl 0x82ca9408
	ctx.lr = 0x832A9CF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9CFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9D00: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A9D04: 396B82A4  addi r11, r11, -0x7d5c
	ctx.r[11].s64 = ctx.r[11].s64 + -32092;
	// 832A9D08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9D0C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A9D10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9D14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9D18: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9D20: 4AF1CA49  bl 0x821c6768
	ctx.lr = 0x832A9D24;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9D24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9D28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9D2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9D30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9D34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9D38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9D3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9D40: 4082FFE8  bne 0x832a9d28
	if !ctx.cr[0].eq {
	pc = 0x832A9D28; continue 'dispatch;
	}
	// 832A9D44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9D48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9D4C: 4080FFCC  bge 0x832a9d18
	if !ctx.cr[0].lt {
	pc = 0x832A9D18; continue 'dispatch;
	}
	// 832A9D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9D54: 4B9FF704  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9D58 size=104
	// 832A9D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9D5C: 4B9FF6AD  bl 0x82ca9408
	ctx.lr = 0x832A9D60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9D64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9D68: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A9D6C: 396B82D4  addi r11, r11, -0x7d2c
	ctx.r[11].s64 = ctx.r[11].s64 + -32044;
	// 832A9D70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9D74: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A9D78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9D7C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9D80: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9D88: 4AF1C9E1  bl 0x821c6768
	ctx.lr = 0x832A9D8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9D8C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9D90: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9D94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9D98: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9D9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9DA0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9DA4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9DA8: 4082FFE8  bne 0x832a9d90
	if !ctx.cr[0].eq {
	pc = 0x832A9D90; continue 'dispatch;
	}
	// 832A9DAC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9DB0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9DB4: 4080FFCC  bge 0x832a9d80
	if !ctx.cr[0].lt {
	pc = 0x832A9D80; continue 'dispatch;
	}
	// 832A9DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9DBC: 4B9FF69C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9DC0 size=104
	// 832A9DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9DC4: 4B9FF645  bl 0x82ca9408
	ctx.lr = 0x832A9DC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9DCC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9DD0: 3BC00013  li r30, 0x13
	ctx.r[30].s64 = 19;
	// 832A9DD4: 396B8318  addi r11, r11, -0x7ce8
	ctx.r[11].s64 = ctx.r[11].s64 + -31976;
	// 832A9DD8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9DDC: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832A9DE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9DE4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9DE8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9DF0: 4AF1C979  bl 0x821c6768
	ctx.lr = 0x832A9DF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9DF4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9DF8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9DFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9E00: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9E04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9E08: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9E0C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9E10: 4082FFE8  bne 0x832a9df8
	if !ctx.cr[0].eq {
	pc = 0x832A9DF8; continue 'dispatch;
	}
	// 832A9E14: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9E18: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9E1C: 4080FFCC  bge 0x832a9de8
	if !ctx.cr[0].lt {
	pc = 0x832A9DE8; continue 'dispatch;
	}
	// 832A9E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9E24: 4B9FF634  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9E28 size=104
	// 832A9E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9E2C: 4B9FF5DD  bl 0x82ca9408
	ctx.lr = 0x832A9E30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9E34: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9E38: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 832A9E3C: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 832A9E40: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9E44: 3BEB0054  addi r31, r11, 0x54
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	// 832A9E48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9E4C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9E50: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9E58: 4AF1C911  bl 0x821c6768
	ctx.lr = 0x832A9E5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9E5C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9E60: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9E64: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9E68: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9E6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9E70: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9E74: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9E78: 4082FFE8  bne 0x832a9e60
	if !ctx.cr[0].eq {
	pc = 0x832A9E60; continue 'dispatch;
	}
	// 832A9E7C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9E80: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9E84: 4080FFCC  bge 0x832a9e50
	if !ctx.cr[0].lt {
	pc = 0x832A9E50; continue 'dispatch;
	}
	// 832A9E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9E8C: 4B9FF5CC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9E90 size=104
	// 832A9E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9E94: 4B9FF575  bl 0x82ca9408
	ctx.lr = 0x832A9E98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9E9C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9EA0: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 832A9EA4: 396B83D0  addi r11, r11, -0x7c30
	ctx.r[11].s64 = ctx.r[11].s64 + -31792;
	// 832A9EA8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9EAC: 3BEB0054  addi r31, r11, 0x54
	ctx.r[31].s64 = ctx.r[11].s64 + 84;
	// 832A9EB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9EB4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9EB8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9EC0: 4AF1C8A9  bl 0x821c6768
	ctx.lr = 0x832A9EC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9EC4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9EC8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9ECC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9ED0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9ED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9ED8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9EDC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9EE0: 4082FFE8  bne 0x832a9ec8
	if !ctx.cr[0].eq {
	pc = 0x832A9EC8; continue 'dispatch;
	}
	// 832A9EE4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9EE8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9EEC: 4080FFCC  bge 0x832a9eb8
	if !ctx.cr[0].lt {
	pc = 0x832A9EB8; continue 'dispatch;
	}
	// 832A9EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9EF4: 4B9FF564  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9EF8 size=104
	// 832A9EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9EFC: 4B9FF50D  bl 0x82ca9408
	ctx.lr = 0x832A9F00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9F04: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9F08: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832A9F0C: 396B8424  addi r11, r11, -0x7bdc
	ctx.r[11].s64 = ctx.r[11].s64 + -31708;
	// 832A9F10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9F14: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832A9F18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9F1C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9F20: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9F28: 4AF1C841  bl 0x821c6768
	ctx.lr = 0x832A9F2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9F2C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9F30: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9F34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9F38: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9F3C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9F40: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9F44: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9F48: 4082FFE8  bne 0x832a9f30
	if !ctx.cr[0].eq {
	pc = 0x832A9F30; continue 'dispatch;
	}
	// 832A9F4C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9F50: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9F54: 4080FFCC  bge 0x832a9f20
	if !ctx.cr[0].lt {
	pc = 0x832A9F20; continue 'dispatch;
	}
	// 832A9F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9F5C: 4B9FF4FC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9F60 size=104
	// 832A9F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9F64: 4B9FF4A5  bl 0x82ca9408
	ctx.lr = 0x832A9F68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9F6C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9F70: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832A9F74: 396B8304  addi r11, r11, -0x7cfc
	ctx.r[11].s64 = ctx.r[11].s64 + -31996;
	// 832A9F78: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9F7C: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832A9F80: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9F84: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9F88: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9F90: 4AF1C7D9  bl 0x821c6768
	ctx.lr = 0x832A9F94;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9F94: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832A9F98: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A9F9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9FA0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A9FA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A9FA8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A9FAC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A9FB0: 4082FFE8  bne 0x832a9f98
	if !ctx.cr[0].eq {
	pc = 0x832A9F98; continue 'dispatch;
	}
	// 832A9FB4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A9FB8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A9FBC: 4080FFCC  bge 0x832a9f88
	if !ctx.cr[0].lt {
	pc = 0x832A9F88; continue 'dispatch;
	}
	// 832A9FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A9FC4: 4B9FF494  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832A9FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A9FC8 size=240
	// 832A9FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A9FCC: 4B9FF43D  bl 0x82ca9408
	ctx.lr = 0x832A9FD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832A9FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A9FD4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832A9FD8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832A9FDC: 396B83BC  addi r11, r11, -0x7c44
	ctx.r[11].s64 = ctx.r[11].s64 + -31812;
	// 832A9FE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A9FE4: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832A9FE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A9FEC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832A9FF0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A9FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A9FF8: 4AF1C771  bl 0x821c6768
	ctx.lr = 0x832A9FFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832A9FFC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AA000: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AA004: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA008: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AA00C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AA010: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AA014: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA018: 4082FFE8  bne 0x832aa000
	if !ctx.cr[0].eq {
	pc = 0x832AA000; continue 'dispatch;
	}
	// 832AA01C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AA020: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AA024: 4080FFCC  bge 0x832a9ff0
	if !ctx.cr[0].lt {
	pc = 0x832A9FF0; continue 'dispatch;
	}
	// 832AA028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AA02C: 4B9FF42C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AA030: 4E800020  blr
	return;
}

pub fn sub_832AA0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA0B8 size=232
	// 832AA0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA0C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA0C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA0C8: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AA0CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832AA0D0: 3BEA9C28  addi r31, r10, -0x63d8
	ctx.r[31].s64 = ctx.r[10].s64 + -25560;
	// 832AA0D4: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832AA0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA0DC: 916A9C28  stw r11, -0x63d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25560 as u32), ctx.r[11].u32 ) };
	// 832AA0E0: 4AEE9D59  bl 0x82193e38
	ctx.lr = 0x832AA0E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 832AA0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AA0E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AA0EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AA0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AA0FC: 4E800020  blr
	return;
	// 832AA100: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA104: 386B8460  addi r3, r11, -0x7ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -31648;
	// 832AA108: 4AF6ACD0  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AA1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA1A0 size=104
	// 832AA1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA1A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA1AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA1B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA1B4: 3BEB8488  addi r31, r11, -0x7b78
	ctx.r[31].s64 = ctx.r[11].s64 + -31608;
	// 832AA1B8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA1BC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AA1C0: 419A0018  beq cr6, 0x832aa1d8
	if ctx.cr[6].eq {
	pc = 0x832AA1D8; continue 'dispatch;
	}
	// 832AA1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA1C8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AA1CC: 4B225745  bl 0x824cf910
	ctx.lr = 0x832AA1D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x824CF910);
	// 832AA1D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA1D4: 4AF71B65  bl 0x8221bd38
	ctx.lr = 0x832AA1D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AA1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AA1DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AA1E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AA1E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AA1E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AA1EC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AA1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AA1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AA200: 4E800020  blr
	return;
}

pub fn sub_832AA208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA208 size=568
	// 832AA208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA21C: 3BEB8498  addi r31, r11, -0x7b68
	ctx.r[31].s64 = ctx.r[11].s64 + -31592;
	// 832AA220: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA224: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AA228: 419A0018  beq cr6, 0x832aa240
	if ctx.cr[6].eq {
	pc = 0x832AA240; continue 'dispatch;
	}
	// 832AA22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA230: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AA234: 4B2256DD  bl 0x824cf910
	ctx.lr = 0x832AA238;
	crate::recompiler::externs::call(&mut ctx, base, 0x824CF910);
	// 832AA238: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA23C: 4AF71AFD  bl 0x8221bd38
	ctx.lr = 0x832AA240;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AA240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AA244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AA248: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AA24C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AA250: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AA254: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AA258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AA25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA264: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AA268: 4E800020  blr
	return;
}

pub fn sub_832AA440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA440 size=200
	// 832AA440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA444: 4B9FEFC5  bl 0x82ca9408
	ctx.lr = 0x832AA448;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AA448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA44C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AA450: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832AA454: 396B3B34  addi r11, r11, 0x3b34
	ctx.r[11].s64 = ctx.r[11].s64 + 15156;
	// 832AA458: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AA45C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 832AA460: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AA464: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AA468: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AA46C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA470: 4AF1C2F9  bl 0x821c6768
	ctx.lr = 0x832AA474;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AA474: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AA478: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AA47C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA480: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AA484: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AA488: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AA48C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA490: 4082FFE8  bne 0x832aa478
	if !ctx.cr[0].eq {
	pc = 0x832AA478; continue 'dispatch;
	}
	// 832AA494: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AA498: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AA49C: 4080FFCC  bge 0x832aa468
	if !ctx.cr[0].lt {
	pc = 0x832AA468; continue 'dispatch;
	}
	// 832AA4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AA4A4: 4B9FEFB4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AA4A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA4AC: 386B851C  addi r3, r11, -0x7ae4
	ctx.r[3].s64 = ctx.r[11].s64 + -31460;
	// 832AA4B0: 4AF6A928  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA508 size=184
	// 832AA508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA51C: 3BEB853C  addi r31, r11, -0x7ac4
	ctx.r[31].s64 = ctx.r[11].s64 + -31428;
	// 832AA520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA524: 4B280325  bl 0x8252a848
	ctx.lr = 0x832AA528;
	crate::recompiler::externs::call(&mut ctx, base, 0x8252A848);
	// 832AA528: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832AA52C: 4AF6A8AD  bl 0x82214dd8
	ctx.lr = 0x832AA530;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832AA530: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AA538: 419A0014  beq cr6, 0x832aa54c
	if ctx.cr[6].eq {
	pc = 0x832AA54C; continue 'dispatch;
	}
	// 832AA53C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AA540: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AA544: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832AA548: 4E800421  bctrl
	ctx.lr = 0x832AA54C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832AA54C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AA550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AA554: 419A0014  beq cr6, 0x832aa568
	if ctx.cr[6].eq {
	pc = 0x832AA568; continue 'dispatch;
	}
	// 832AA558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AA55C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AA560: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832AA564: 4E800421  bctrl
	ctx.lr = 0x832AA568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832AA568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AA56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AA578: 4E800020  blr
	return;
}

pub fn sub_832AA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA5C0 size=856
	// 832AA5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA5C4: 4B9FEE45  bl 0x82ca9408
	ctx.lr = 0x832AA5C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AA5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA5CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA5D0: 3BC00029  li r30, 0x29
	ctx.r[30].s64 = 41;
	// 832AA5D4: 396B8568  addi r11, r11, -0x7a98
	ctx.r[11].s64 = ctx.r[11].s64 + -31384;
	// 832AA5D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AA5DC: 3BEB00A8  addi r31, r11, 0xa8
	ctx.r[31].s64 = ctx.r[11].s64 + 168;
	// 832AA5E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AA5E4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AA5E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AA5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AA5F0: 4AF1C179  bl 0x821c6768
	ctx.lr = 0x832AA5F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AA5F4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AA5F8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AA5FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA600: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AA604: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AA608: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AA60C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AA610: 4082FFE8  bne 0x832aa5f8
	if !ctx.cr[0].eq {
	pc = 0x832AA5F8; continue 'dispatch;
	}
	// 832AA614: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AA618: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AA61C: 4080FFCC  bge 0x832aa5e8
	if !ctx.cr[0].lt {
	pc = 0x832AA5E8; continue 'dispatch;
	}
	// 832AA620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AA624: 4B9FEE34  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AA628: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA62C: 386B8610  addi r3, r11, -0x79f0
	ctx.r[3].s64 = ctx.r[11].s64 + -31216;
	// 832AA630: 4AF6A7A8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA918 size=152
	// 832AA918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832AA924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA92C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA930: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AA934: 3BEB87F0  addi r31, r11, -0x7810
	ctx.r[31].s64 = ctx.r[11].s64 + -30736;
	// 832AA938: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA93C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AA940: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832AA944: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA948: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832AA94C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA950: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AA954: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832AA958: 419A0020  beq cr6, 0x832aa978
	if ctx.cr[6].eq {
	pc = 0x832AA978; continue 'dispatch;
	}
	// 832AA95C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832AA960: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AA964: 4AF713D5  bl 0x8221bd38
	ctx.lr = 0x832AA968;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AA968: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA96C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832AA970: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AA974: 409AFFE8  bne cr6, 0x832aa95c
	if !ctx.cr[6].eq {
	pc = 0x832AA95C; continue 'dispatch;
	}
	// 832AA978: 4AF713C1  bl 0x8221bd38
	ctx.lr = 0x832AA97C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AA97C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AA980: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AA984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AA988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832AA994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AA998: 4E800020  blr
	return;
}

pub fn sub_832AA9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AA9B0 size=184
	// 832AA9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AA9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AA9B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AA9BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AA9C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AA9C4: 3BEB87FC  addi r31, r11, -0x7804
	ctx.r[31].s64 = ctx.r[11].s64 + -30724;
	// 832AA9C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AA9CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AA9D0: 419A0008  beq cr6, 0x832aa9d8
	if ctx.cr[6].eq {
	pc = 0x832AA9D8; continue 'dispatch;
	}
	// 832AA9D4: 4AF71365  bl 0x8221bd38
	ctx.lr = 0x832AA9D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AA9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AA9DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AA9E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AA9E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AA9E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AA9EC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AA9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AA9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AA9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AA9FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAA00: 4E800020  blr
	return;
}

pub fn sub_832AAA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAA68 size=72
	// 832AAA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAA70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAA74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAA78: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AAA7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832AAA80: 3BEA9F34  addi r31, r10, -0x60cc
	ctx.r[31].s64 = ctx.r[10].s64 + -24780;
	// 832AAA84: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832AAA88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AAA8C: 916A9F34  stw r11, -0x60cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24780 as u32), ctx.r[11].u32 ) };
	// 832AAA90: 4AEE93A9  bl 0x82193e38
	ctx.lr = 0x832AAA94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 832AAA94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAA98: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AAA9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAAAC: 4E800020  blr
	return;
}

pub fn sub_832AAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAAB0 size=88
	// 832AAAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAAB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAAC0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AAAC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832AAAC8: 3BEA9F44  addi r31, r10, -0x60bc
	ctx.r[31].s64 = ctx.r[10].s64 + -24764;
	// 832AAACC: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832AAAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AAAD4: 916A9F44  stw r11, -0x60bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24764 as u32), ctx.r[11].u32 ) };
	// 832AAAD8: 4AEE9361  bl 0x82193e38
	ctx.lr = 0x832AAADC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 832AAADC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAAE0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AAAE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAAF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAAF4: 4E800020  blr
	return;
	// 832AAAF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAAFC: 386B8830  addi r3, r11, -0x77d0
	ctx.r[3].s64 = ctx.r[11].s64 + -30672;
	// 832AAB00: 4AF0D018  b 0x821b7b18
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	return;
}

pub fn sub_832AAB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAB08 size=560
	// 832AAB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAB10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAB14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAB18: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832AAB1C: 807F8838  lwz r3, -0x77c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-30664 as u32) ) } as u64;
	// 832AAB20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AAB24: 419A0044  beq cr6, 0x832aab68
	if ctx.cr[6].eq {
	pc = 0x832AAB68; continue 'dispatch;
	}
	// 832AAB28: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832AAB2C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AAB30: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AAB34: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AAB38: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AAB3C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AAB40: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AAB44: 4082FFE8  bne 0x832aab2c
	if !ctx.cr[0].eq {
	pc = 0x832AAB2C; continue 'dispatch;
	}
	// 832AAB48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832AAB4C: 409A0014  bne cr6, 0x832aab60
	if !ctx.cr[6].eq {
	pc = 0x832AAB60; continue 'dispatch;
	}
	// 832AAB50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAB54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAB58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832AAB5C: 4E800421  bctrl
	ctx.lr = 0x832AAB60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832AAB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AAB64: 917F8838  stw r11, -0x77c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-30664 as u32), ctx.r[11].u32 ) };
	// 832AAB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AAB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AAB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AAB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AAB78: 4E800020  blr
	return;
}

pub fn sub_832AAD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AAD38 size=1000
	// 832AAD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AAD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AAD40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832AAD44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AAD48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AAD4C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AAD50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AAD54: 3BEB88B0  addi r31, r11, -0x7750
	ctx.r[31].s64 = ctx.r[11].s64 + -30544;
	// 832AAD58: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD5C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAD60: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832AAD64: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD68: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832AAD6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD70: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AAD74: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832AAD78: 419A0020  beq cr6, 0x832aad98
	if ctx.cr[6].eq {
	pc = 0x832AAD98; continue 'dispatch;
	}
	// 832AAD7C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832AAD80: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AAD84: 4AF70FB5  bl 0x8221bd38
	ctx.lr = 0x832AAD88;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AAD88: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AAD8C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832AAD90: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AAD94: 409AFFE8  bne cr6, 0x832aad7c
	if !ctx.cr[6].eq {
	pc = 0x832AAD7C; continue 'dispatch;
	}
	// 832AAD98: 4AF70FA1  bl 0x8221bd38
	ctx.lr = 0x832AAD9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AAD9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AADA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AADA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AADB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832AADB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AADB8: 4E800020  blr
	return;
}

pub fn sub_832AB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB120 size=104
	// 832AB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB130: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB134: 3BEB89EC  addi r31, r11, -0x7614
	ctx.r[31].s64 = ctx.r[11].s64 + -30228;
	// 832AB138: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB13C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AB140: 419A0018  beq cr6, 0x832ab158
	if ctx.cr[6].eq {
	pc = 0x832AB158; continue 'dispatch;
	}
	// 832AB144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB148: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AB14C: 4B2247C5  bl 0x824cf910
	ctx.lr = 0x832AB150;
	crate::recompiler::externs::call(&mut ctx, base, 0x824CF910);
	// 832AB150: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB154: 4AF70BE5  bl 0x8221bd38
	ctx.lr = 0x832AB158;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB15C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB160: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB164: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB168: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB16C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB17C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB180: 4E800020  blr
	return;
}

pub fn sub_832AB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB188 size=104
	// 832AB188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB19C: 3BEB89FC  addi r31, r11, -0x7604
	ctx.r[31].s64 = ctx.r[11].s64 + -30212;
	// 832AB1A0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB1A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AB1A8: 419A0018  beq cr6, 0x832ab1c0
	if ctx.cr[6].eq {
	pc = 0x832AB1C0; continue 'dispatch;
	}
	// 832AB1AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB1B0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AB1B4: 4B22475D  bl 0x824cf910
	ctx.lr = 0x832AB1B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x824CF910);
	// 832AB1B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB1BC: 4AF70B7D  bl 0x8221bd38
	ctx.lr = 0x832AB1C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB1C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB1C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB1CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB1D0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB1D4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB1E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB1E8: 4E800020  blr
	return;
}

pub fn sub_832AB1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB1F0 size=200
	// 832AB1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB1FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB200: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB204: 3BEB8A0C  addi r31, r11, -0x75f4
	ctx.r[31].s64 = ctx.r[11].s64 + -30196;
	// 832AB208: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB20C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB210: 419A0008  beq cr6, 0x832ab218
	if ctx.cr[6].eq {
	pc = 0x832AB218; continue 'dispatch;
	}
	// 832AB214: 4AF70B25  bl 0x8221bd38
	ctx.lr = 0x832AB218;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB21C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB220: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB224: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB228: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB22C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB240: 4E800020  blr
	return;
}

pub fn sub_832AB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB2B8 size=104
	// 832AB2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB2BC: 4B9FE14D  bl 0x82ca9408
	ctx.lr = 0x832AB2C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AB2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB2C4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB2C8: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	// 832AB2CC: 396B8A38  addi r11, r11, -0x75c8
	ctx.r[11].s64 = ctx.r[11].s64 + -30152;
	// 832AB2D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB2D4: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 832AB2D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB2DC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AB2E0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB2E8: 4AF1B481  bl 0x821c6768
	ctx.lr = 0x832AB2EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AB2EC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AB2F0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB2F4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB2F8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB2FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB300: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB304: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB308: 4082FFE8  bne 0x832ab2f0
	if !ctx.cr[0].eq {
	pc = 0x832AB2F0; continue 'dispatch;
	}
	// 832AB30C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB310: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB314: 4080FFCC  bge 0x832ab2e0
	if !ctx.cr[0].lt {
	pc = 0x832AB2E0; continue 'dispatch;
	}
	// 832AB318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB31C: 4B9FE13C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832AB320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB320 size=312
	// 832AB320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB32C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB330: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB334: 3BEB8AB8  addi r31, r11, -0x7548
	ctx.r[31].s64 = ctx.r[11].s64 + -30024;
	// 832AB338: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB33C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB340: 419A0008  beq cr6, 0x832ab348
	if ctx.cr[6].eq {
	pc = 0x832AB348; continue 'dispatch;
	}
	// 832AB344: 4AF709F5  bl 0x8221bd38
	ctx.lr = 0x832AB348;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB34C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB350: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB354: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB358: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB35C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB370: 4E800020  blr
	return;
}

pub fn sub_832AB458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB458 size=104
	// 832AB458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB45C: 4B9FDFAD  bl 0x82ca9408
	ctx.lr = 0x832AB460;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AB460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB464: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB468: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 832AB46C: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 832AB470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB474: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832AB478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB47C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AB480: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB488: 4AF1B2E1  bl 0x821c6768
	ctx.lr = 0x832AB48C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AB48C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AB490: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB498: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB49C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB4A0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB4A4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB4A8: 4082FFE8  bne 0x832ab490
	if !ctx.cr[0].eq {
	pc = 0x832AB490; continue 'dispatch;
	}
	// 832AB4AC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB4B0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB4B4: 4080FFCC  bge 0x832ab480
	if !ctx.cr[0].lt {
	pc = 0x832AB480; continue 'dispatch;
	}
	// 832AB4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB4BC: 4B9FDF9C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832AB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB4C0 size=168
	// 832AB4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB4D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB4D4: 3BEB8B90  addi r31, r11, -0x7470
	ctx.r[31].s64 = ctx.r[11].s64 + -29808;
	// 832AB4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB4DC: 4B481E5D  bl 0x8272d338
	ctx.lr = 0x832AB4E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8272D338);
	// 832AB4E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB4E4: 4AF70855  bl 0x8221bd38
	ctx.lr = 0x832AB4E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB4EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB500: 4E800020  blr
	return;
}

pub fn sub_832AB568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB568 size=120
	// 832AB568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB56C: 4B9FDE9D  bl 0x82ca9408
	ctx.lr = 0x832AB570;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AB570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB574: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AB578: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832AB57C: 396B3B04  addi r11, r11, 0x3b04
	ctx.r[11].s64 = ctx.r[11].s64 + 15108;
	// 832AB580: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB584: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832AB588: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB58C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AB590: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB598: 4AF1B1D1  bl 0x821c6768
	ctx.lr = 0x832AB59C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AB59C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AB5A0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB5A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5A8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB5AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB5B0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB5B4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5B8: 4082FFE8  bne 0x832ab5a0
	if !ctx.cr[0].eq {
	pc = 0x832AB5A0; continue 'dispatch;
	}
	// 832AB5BC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB5C0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB5C4: 4080FFCC  bge 0x832ab590
	if !ctx.cr[0].lt {
	pc = 0x832AB590; continue 'dispatch;
	}
	// 832AB5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB5CC: 4B9FDE8C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AB5D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB5D4: 386B8BCC  addi r3, r11, -0x7434
	ctx.r[3].s64 = ctx.r[11].s64 + -29748;
	// 832AB5D8: 4AF69800  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB5E0 size=376
	// 832AB5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB5F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB5F4: 3BEB8BD0  addi r31, r11, -0x7430
	ctx.r[31].s64 = ctx.r[11].s64 + -29744;
	// 832AB5F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB5FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB600: 419A0008  beq cr6, 0x832ab608
	if ctx.cr[6].eq {
	pc = 0x832AB608; continue 'dispatch;
	}
	// 832AB604: 4AF70735  bl 0x8221bd38
	ctx.lr = 0x832AB608;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB610: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB614: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB618: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB61C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB630: 4E800020  blr
	return;
}

pub fn sub_832AB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB758 size=368
	// 832AB758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB768: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB76C: 3BEB92B0  addi r31, r11, -0x6d50
	ctx.r[31].s64 = ctx.r[11].s64 + -27984;
	// 832AB770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB774: 4B077075  bl 0x823227e8
	ctx.lr = 0x832AB778;
	crate::recompiler::externs::call(&mut ctx, base, 0x823227E8);
	// 832AB778: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB77C: 4AF705BD  bl 0x8221bd38
	ctx.lr = 0x832AB780;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB784: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB798: 4E800020  blr
	return;
}

pub fn sub_832AB8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB8C8 size=496
	// 832AB8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB8D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB8D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB8D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB8DC: 3BEB9EAC  addi r31, r11, -0x6154
	ctx.r[31].s64 = ctx.r[11].s64 + -24916;
	// 832AB8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB8E4: 4B37FC65  bl 0x8262b548
	ctx.lr = 0x832AB8E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262B548);
	// 832AB8E8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB8EC: 4AF7044D  bl 0x8221bd38
	ctx.lr = 0x832AB8F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AB8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB8F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB908: 4E800020  blr
	return;
}

pub fn sub_832ABAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABAB8 size=112
	// 832ABAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABAC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832ABAC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABAC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABACC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABAD0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832ABAD4: 396BA638  addi r11, r11, -0x59c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22984;
	// 832ABAD8: 3BEB02C0  addi r31, r11, 0x2c0
	ctx.r[31].s64 = ctx.r[11].s64 + 704;
	// 832ABADC: 3BFFFF50  addi r31, r31, -0xb0
	ctx.r[31].s64 = ctx.r[31].s64 + -176;
	// 832ABAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABAE4: 4B3AF085  bl 0x8265ab68
	ctx.lr = 0x832ABAE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8265AB68);
	// 832ABAE8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABAEC: 4080FFF0  bge 0x832abadc
	if !ctx.cr[0].lt {
	pc = 0x832ABADC; continue 'dispatch;
	}
	// 832ABAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832ABAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832ABB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB04: 4E800020  blr
	return;
	// 832ABB08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB0C: 386BA8F8  addi r3, r11, -0x5708
	ctx.r[3].s64 = ctx.r[11].s64 + -22280;
	// 832ABB10: 4B3B0A58  b 0x8265c568
	crate::recompiler::externs::call(&mut ctx, base, 0x8265C568);
	return;
}

pub fn sub_832ABB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB28 size=88
	// 832ABB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB3C: 3BEBA920  addi r31, r11, -0x56e0
	ctx.r[31].s64 = ctx.r[11].s64 + -22240;
	// 832ABB40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABB48: 419A0008  beq cr6, 0x832abb50
	if ctx.cr[6].eq {
	pc = 0x832ABB50; continue 'dispatch;
	}
	// 832ABB4C: 4AF701ED  bl 0x8221bd38
	ctx.lr = 0x832ABB50;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABB58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABB5C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABB60: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABB64: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB78: 4E800020  blr
	return;
}

pub fn sub_832ABB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB80 size=280
	// 832ABB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB94: 3BEBA930  addi r31, r11, -0x56d0
	ctx.r[31].s64 = ctx.r[11].s64 + -22224;
	// 832ABB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABB9C: 4B3BC5BD  bl 0x82668158
	ctx.lr = 0x832ABBA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82668158);
	// 832ABBA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABBA4: 4AF70195  bl 0x8221bd38
	ctx.lr = 0x832ABBA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABBAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABBC0: 4E800020  blr
	return;
}

pub fn sub_832ABC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABC98 size=72
	// 832ABC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCAC: 3BEBA9A4  addi r31, r11, -0x565c
	ctx.r[31].s64 = ctx.r[11].s64 + -22108;
	// 832ABCB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABCB4: 4B3C80E5  bl 0x82673d98
	ctx.lr = 0x832ABCB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82673D98);
	// 832ABCB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCBC: 4AF7007D  bl 0x8221bd38
	ctx.lr = 0x832ABCC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABCC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABCD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABCD8: 4E800020  blr
	return;
}

pub fn sub_832ABCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABCE0 size=88
	// 832ABCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCF4: 3BEBA9B0  addi r31, r11, -0x5650
	ctx.r[31].s64 = ctx.r[11].s64 + -22096;
	// 832ABCF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABD00: 419A0008  beq cr6, 0x832abd08
	if ctx.cr[6].eq {
	pc = 0x832ABD08; continue 'dispatch;
	}
	// 832ABD04: 4AF70035  bl 0x8221bd38
	ctx.lr = 0x832ABD08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABD10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABD14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABD1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD30: 4E800020  blr
	return;
}

pub fn sub_832ABD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD38 size=72
	// 832ABD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD4C: 3BEBA9D0  addi r31, r11, -0x5630
	ctx.r[31].s64 = ctx.r[11].s64 + -22064;
	// 832ABD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD54: 4AEC7185  bl 0x82172ed8
	ctx.lr = 0x832ABD58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82172ED8);
	// 832ABD58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABD5C: 4AF6FFDD  bl 0x8221bd38
	ctx.lr = 0x832ABD60;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD64: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD78: 4E800020  blr
	return;
}

pub fn sub_832ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD80 size=112
	// 832ABD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD94: 3BEBA9DC  addi r31, r11, -0x5624
	ctx.r[31].s64 = ctx.r[11].s64 + -22052;
	// 832ABD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD9C: 4B029055  bl 0x822d4df0
	ctx.lr = 0x832ABDA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x822D4DF0);
	// 832ABDA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABDA4: 4AF6FF95  bl 0x8221bd38
	ctx.lr = 0x832ABDA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABDAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABDC0: 4E800020  blr
	return;
}

pub fn sub_832ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABDF0 size=152
	// 832ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABDF4: 4B9FD615  bl 0x82ca9408
	ctx.lr = 0x832ABDF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABDFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE00: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 832ABE04: 396BAA08  addi r11, r11, -0x55f8
	ctx.r[11].s64 = ctx.r[11].s64 + -22008;
	// 832ABE08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ABE0C: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832ABE10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ABE14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ABE18: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832ABE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABE20: 4AF1A949  bl 0x821c6768
	ctx.lr = 0x832ABE24;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ABE24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ABE28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ABE2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ABE34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ABE38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ABE3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE40: 4082FFE8  bne 0x832abe28
	if !ctx.cr[0].eq {
	pc = 0x832ABE28; continue 'dispatch;
	}
	// 832ABE44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABE48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ABE4C: 4080FFCC  bge 0x832abe18
	if !ctx.cr[0].lt {
	pc = 0x832ABE18; continue 'dispatch;
	}
	// 832ABE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ABE54: 4B9FD604  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832ABE58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE5C: 386BAA90  addi r3, r11, -0x5570
	ctx.r[3].s64 = ctx.r[11].s64 + -21872;
	// 832ABE60: 4AF68F78  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832ABE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABE88 size=552
	// 832ABE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABE98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE9C: 3BEBAA9C  addi r31, r11, -0x5564
	ctx.r[31].s64 = ctx.r[11].s64 + -21860;
	// 832ABEA0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832ABEA8: 419A0018  beq cr6, 0x832abec0
	if ctx.cr[6].eq {
	pc = 0x832ABEC0; continue 'dispatch;
	}
	// 832ABEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABEB0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832ABEB4: 4B134015  bl 0x823dfec8
	ctx.lr = 0x832ABEB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x823DFEC8);
	// 832ABEB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEBC: 4AF6FE7D  bl 0x8221bd38
	ctx.lr = 0x832ABEC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832ABEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABEC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABECC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABED0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABED4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABEE8: 4E800020  blr
	return;
}

pub fn sub_832AC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AC0B0 size=2060
	// 832AC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AC0B4: 4B9FD355  bl 0x82ca9408
	ctx.lr = 0x832AC0B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AC0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AC0BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0C0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832AC0C4: 396BAB30  addi r11, r11, -0x54d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21712;
	// 832AC0C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AC0CC: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 832AC0D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AC0D4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AC0D8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AC0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AC0E0: 4AF1A689  bl 0x821c6768
	ctx.lr = 0x832AC0E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AC0E4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AC0E8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AC0EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC0F0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AC0F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AC0F8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AC0FC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC100: 4082FFE8  bne 0x832ac0e8
	if !ctx.cr[0].eq {
	pc = 0x832AC0E8; continue 'dispatch;
	}
	// 832AC104: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AC108: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AC10C: 4080FFCC  bge 0x832ac0d8
	if !ctx.cr[0].lt {
	pc = 0x832AC0D8; continue 'dispatch;
	}
	// 832AC110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AC114: 4B9FD344  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AC118: 4E800020  blr
	return;
}

pub fn sub_832ACBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACBF0 size=104
	// 832ACBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACBF4: 4B9FC815  bl 0x82ca9408
	ctx.lr = 0x832ACBF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ACBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACBFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACC00: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832ACC04: 396BB380  addi r11, r11, -0x4c80
	ctx.r[11].s64 = ctx.r[11].s64 + -19584;
	// 832ACC08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACC0C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832ACC10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACC14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACC18: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACC20: 4AF19B49  bl 0x821c6768
	ctx.lr = 0x832ACC24;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ACC24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACC28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACC2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACC34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACC38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACC3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC40: 4082FFE8  bne 0x832acc28
	if !ctx.cr[0].eq {
	pc = 0x832ACC28; continue 'dispatch;
	}
	// 832ACC44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACC48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACC4C: 4080FFCC  bge 0x832acc18
	if !ctx.cr[0].lt {
	pc = 0x832ACC18; continue 'dispatch;
	}
	// 832ACC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACC54: 4B9FC804  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832ACC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACC58 size=584
	// 832ACC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACC5C: 4B9FC7AD  bl 0x82ca9408
	ctx.lr = 0x832ACC60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ACC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACC64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACC68: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832ACC6C: 396BB3C0  addi r11, r11, -0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + -19520;
	// 832ACC70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACC74: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832ACC78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACC7C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACC80: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACC88: 4AF19AE1  bl 0x821c6768
	ctx.lr = 0x832ACC8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ACC8C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACC90: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACC94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACC98: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACC9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACCA0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACCA4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACCA8: 4082FFE8  bne 0x832acc90
	if !ctx.cr[0].eq {
	pc = 0x832ACC90; continue 'dispatch;
	}
	// 832ACCAC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACCB0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACCB4: 4080FFCC  bge 0x832acc80
	if !ctx.cr[0].lt {
	pc = 0x832ACC80; continue 'dispatch;
	}
	// 832ACCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACCBC: 4B9FC79C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832ACCC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACCC4: 386BB454  addi r3, r11, -0x4bac
	ctx.r[3].s64 = ctx.r[11].s64 + -19372;
	// 832ACCC8: 4AF68110  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832ACEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACEA0 size=104
	// 832ACEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACEA4: 4B9FC565  bl 0x82ca9408
	ctx.lr = 0x832ACEA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ACEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACEAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACEB0: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832ACEB4: 396BB5C8  addi r11, r11, -0x4a38
	ctx.r[11].s64 = ctx.r[11].s64 + -19000;
	// 832ACEB8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACEBC: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832ACEC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACEC4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACEC8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACED0: 4AF19899  bl 0x821c6768
	ctx.lr = 0x832ACED4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ACED4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACED8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACEDC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACEE0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACEE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACEE8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACEEC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACEF0: 4082FFE8  bne 0x832aced8
	if !ctx.cr[0].eq {
	pc = 0x832ACED8; continue 'dispatch;
	}
	// 832ACEF4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACEF8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACEFC: 4080FFCC  bge 0x832acec8
	if !ctx.cr[0].lt {
	pc = 0x832ACEC8; continue 'dispatch;
	}
	// 832ACF00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACF04: 4B9FC554  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832ACF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ACF08 size=720
	// 832ACF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ACF0C: 4B9FC4FD  bl 0x82ca9408
	ctx.lr = 0x832ACF10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ACF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ACF14: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ACF18: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 832ACF1C: 396BB5E4  addi r11, r11, -0x4a1c
	ctx.r[11].s64 = ctx.r[11].s64 + -18972;
	// 832ACF20: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ACF24: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832ACF28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ACF2C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ACF30: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ACF34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ACF38: 4AF19831  bl 0x821c6768
	ctx.lr = 0x832ACF3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ACF3C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ACF40: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ACF44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACF48: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ACF4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ACF50: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ACF54: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ACF58: 4082FFE8  bne 0x832acf40
	if !ctx.cr[0].eq {
	pc = 0x832ACF40; continue 'dispatch;
	}
	// 832ACF5C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ACF60: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ACF64: 4080FFCC  bge 0x832acf30
	if !ctx.cr[0].lt {
	pc = 0x832ACF30; continue 'dispatch;
	}
	// 832ACF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ACF6C: 4B9FC4EC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832ACF70: 4E800020  blr
	return;
}

pub fn sub_832AD1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AD1D8 size=1224
	// 832AD1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AD1DC: 4B9FC231  bl 0x82ca940c
	ctx.lr = 0x832AD1E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 832AD1E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AD1E4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AD1E8: 3BEB1AAC  addi r31, r11, 0x1aac
	ctx.r[31].s64 = ctx.r[11].s64 + 6828;
	// 832AD1EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AD1F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AD1F4: 419A0030  beq cr6, 0x832ad224
	if ctx.cr[6].eq {
	pc = 0x832AD224; continue 'dispatch;
	}
	// 832AD1F8: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AD1FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832AD200: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 832AD204: 419A001C  beq cr6, 0x832ad220
	if ctx.cr[6].eq {
	pc = 0x832AD220; continue 'dispatch;
	}
	// 832AD208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832AD20C: 4AF0A90D  bl 0x821b7b18
	ctx.lr = 0x832AD210;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832AD210: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 832AD214: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 832AD218: 409AFFF0  bne cr6, 0x832ad208
	if !ctx.cr[6].eq {
	pc = 0x832AD208; continue 'dispatch;
	}
	// 832AD21C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AD220: 4AF6EB19  bl 0x8221bd38
	ctx.lr = 0x832AD224;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AD224: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AD228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AD22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AD230: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AD234: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AD238: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AD23C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AD240: 4B9FC21C  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_832AD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AD6A0 size=768
	// 832AD6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AD6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AD6A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AD6AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AD6B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AD6B4: 3BEBB824  addi r31, r11, -0x47dc
	ctx.r[31].s64 = ctx.r[11].s64 + -18396;
	// 832AD6B8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AD6BC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AD6C0: 419A0018  beq cr6, 0x832ad6d8
	if ctx.cr[6].eq {
	pc = 0x832AD6D8; continue 'dispatch;
	}
	// 832AD6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AD6C8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AD6CC: 4B7E4835  bl 0x82a91f00
	ctx.lr = 0x832AD6D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A91F00);
	// 832AD6D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AD6D4: 4AF6E665  bl 0x8221bd38
	ctx.lr = 0x832AD6D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AD6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AD6DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AD6E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AD6E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AD6E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AD6EC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AD6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AD6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AD6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AD6FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AD700: 4E800020  blr
	return;
}

pub fn sub_832AD9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AD9A0 size=104
	// 832AD9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AD9A4: 4B9FBA65  bl 0x82ca9408
	ctx.lr = 0x832AD9A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AD9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AD9AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AD9B0: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832AD9B4: 396BB930  addi r11, r11, -0x46d0
	ctx.r[11].s64 = ctx.r[11].s64 + -18128;
	// 832AD9B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AD9BC: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832AD9C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AD9C4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AD9C8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AD9CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AD9D0: 4AF18D99  bl 0x821c6768
	ctx.lr = 0x832AD9D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AD9D4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AD9D8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AD9DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AD9E0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AD9E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AD9E8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AD9EC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AD9F0: 4082FFE8  bne 0x832ad9d8
	if !ctx.cr[0].eq {
	pc = 0x832AD9D8; continue 'dispatch;
	}
	// 832AD9F4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AD9F8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AD9FC: 4080FFCC  bge 0x832ad9c8
	if !ctx.cr[0].lt {
	pc = 0x832AD9C8; continue 'dispatch;
	}
	// 832ADA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ADA04: 4B9FBA54  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832ADA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ADA08 size=312
	// 832ADA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ADA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ADA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ADA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ADA18: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832ADA1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832ADA20: 3BEAA580  addi r31, r10, -0x5a80
	ctx.r[31].s64 = ctx.r[10].s64 + -23168;
	// 832ADA24: 396B0B7C  addi r11, r11, 0xb7c
	ctx.r[11].s64 = ctx.r[11].s64 + 2940;
	// 832ADA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ADA2C: 916AA580  stw r11, -0x5a80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23168 as u32), ctx.r[11].u32 ) };
	// 832ADA30: 4AEE6409  bl 0x82193e38
	ctx.lr = 0x832ADA34;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 832ADA34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ADA38: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ADA3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ADA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ADA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ADA48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ADA4C: 4E800020  blr
	return;
	// 832ADA50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ADA54: 386BBA0C  addi r3, r11, -0x45f4
	ctx.r[3].s64 = ctx.r[11].s64 + -17908;
	// 832ADA58: 4AF67380  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832ADB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ADB40 size=1176
	// 832ADB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ADB44: 4B9FB8C5  bl 0x82ca9408
	ctx.lr = 0x832ADB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832ADB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ADB4C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ADB50: 3BC0001B  li r30, 0x1b
	ctx.r[30].s64 = 27;
	// 832ADB54: 396BBA80  addi r11, r11, -0x4580
	ctx.r[11].s64 = ctx.r[11].s64 + -17792;
	// 832ADB58: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ADB5C: 3BEB0070  addi r31, r11, 0x70
	ctx.r[31].s64 = ctx.r[11].s64 + 112;
	// 832ADB60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ADB64: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832ADB68: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832ADB6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ADB70: 4AF18BF9  bl 0x821c6768
	ctx.lr = 0x832ADB74;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832ADB74: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832ADB78: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ADB7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ADB80: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ADB84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ADB88: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ADB8C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ADB90: 4082FFE8  bne 0x832adb78
	if !ctx.cr[0].eq {
	pc = 0x832ADB78; continue 'dispatch;
	}
	// 832ADB94: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ADB98: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ADB9C: 4080FFCC  bge 0x832adb68
	if !ctx.cr[0].lt {
	pc = 0x832ADB68; continue 'dispatch;
	}
	// 832ADBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ADBA4: 4B9FB8B4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832ADBA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ADBAC: 386BBAF0  addi r3, r11, -0x4510
	ctx.r[3].s64 = ctx.r[11].s64 + -17680;
	// 832ADBB0: 4AF67228  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832ADFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ADFD8 size=12
	// 832ADFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ADFDC: 4B9FB425  bl 0x82ca9400
	ctx.lr = 0x832ADFE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9400);
	// 832ADFE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832AE1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AE1D8 size=1040
	// 832AE1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AE1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AE1E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AE1E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AE1E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AE1EC: 3BEBC144  addi r31, r11, -0x3ebc
	ctx.r[31].s64 = ctx.r[11].s64 + -16060;
	// 832AE1F0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE1F4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832AE1F8: 419A0018  beq cr6, 0x832ae210
	if ctx.cr[6].eq {
	pc = 0x832AE210; continue 'dispatch;
	}
	// 832AE1FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AE200: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AE204: 4B7E3CFD  bl 0x82a91f00
	ctx.lr = 0x832AE208;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A91F00);
	// 832AE208: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE20C: 4AF6DB2D  bl 0x8221bd38
	ctx.lr = 0x832AE210;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AE210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AE214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AE218: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AE21C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AE220: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AE224: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AE228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AE22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AE230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AE234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AE238: 4E800020  blr
	return;
}

pub fn sub_832AE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AE738 size=176
	// 832AE738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AE73C: 4B9FACD1  bl 0x82ca940c
	ctx.lr = 0x832AE740;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 832AE740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AE744: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AE748: 3BCBC2B8  addi r30, r11, -0x3d48
	ctx.r[30].s64 = ctx.r[11].s64 + -15688;
	// 832AE74C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE750: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AE754: 419A0040  beq cr6, 0x832ae794
	if ctx.cr[6].eq {
	pc = 0x832AE794; continue 'dispatch;
	}
	// 832AE758: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 832AE75C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832AE760: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 832AE764: 419A002C  beq cr6, 0x832ae790
	if ctx.cr[6].eq {
	pc = 0x832AE790; continue 'dispatch;
	}
	// 832AE768: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AE76C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832AE770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AE774: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AE778: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832AE77C: 4E800421  bctrl
	ctx.lr = 0x832AE780;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832AE780: 3BFF0060  addi r31, r31, 0x60
	ctx.r[31].s64 = ctx.r[31].s64 + 96;
	// 832AE784: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 832AE788: 409AFFE0  bne cr6, 0x832ae768
	if !ctx.cr[6].eq {
	pc = 0x832AE768; continue 'dispatch;
	}
	// 832AE78C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE790: 4AF6D5A9  bl 0x8221bd38
	ctx.lr = 0x832AE794;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AE794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AE798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AE79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AE7A0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AE7A4: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AE7A8: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AE7AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AE7B0: 4B9FACAC  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_832AE7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AE7E8 size=480
	// 832AE7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AE7EC: 4B9FAC1D  bl 0x82ca9408
	ctx.lr = 0x832AE7F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AE7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AE7F4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AE7F8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832AE7FC: 396B3CF4  addi r11, r11, 0x3cf4
	ctx.r[11].s64 = ctx.r[11].s64 + 15604;
	// 832AE800: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AE804: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 832AE808: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AE80C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AE810: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AE814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AE818: 4AF17F51  bl 0x821c6768
	ctx.lr = 0x832AE81C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AE81C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AE820: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AE824: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AE828: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AE82C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AE830: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AE834: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AE838: 4082FFE8  bne 0x832ae820
	if !ctx.cr[0].eq {
	pc = 0x832AE820; continue 'dispatch;
	}
	// 832AE83C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AE840: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AE844: 4080FFCC  bge 0x832ae810
	if !ctx.cr[0].lt {
	pc = 0x832AE810; continue 'dispatch;
	}
	// 832AE848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AE84C: 4B9FAC0C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AE850: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AE854: 386BC2DC  addi r3, r11, -0x3d24
	ctx.r[3].s64 = ctx.r[11].s64 + -15652;
	// 832AE858: 4AF66580  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AE9C8 size=216
	// 832AE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AE9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AE9D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832AE9D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AE9D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AE9DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AE9E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AE9E4: 3BEBC390  addi r31, r11, -0x3c70
	ctx.r[31].s64 = ctx.r[11].s64 + -15472;
	// 832AE9E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE9EC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AE9F0: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832AE9F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AE9F8: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832AE9FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AEA00: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AEA04: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832AEA08: 419A0020  beq cr6, 0x832aea28
	if ctx.cr[6].eq {
	pc = 0x832AEA28; continue 'dispatch;
	}
	// 832AEA0C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832AEA10: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832AEA14: 4AF6D325  bl 0x8221bd38
	ctx.lr = 0x832AEA18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AEA18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AEA1C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832AEA20: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832AEA24: 409AFFE8  bne cr6, 0x832aea0c
	if !ctx.cr[6].eq {
	pc = 0x832AEA0C; continue 'dispatch;
	}
	// 832AEA28: 4AF6D311  bl 0x8221bd38
	ctx.lr = 0x832AEA2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AEA2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AEA30: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AEA34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832AEA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AEA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AEA40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832AEA44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AEA48: 4E800020  blr
	return;
}

pub fn sub_832AEAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AEAA0 size=120
	// 832AEAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AEAA4: 4B9FA965  bl 0x82ca9408
	ctx.lr = 0x832AEAA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AEAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AEAAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AEAB0: 3BC0003E  li r30, 0x3e
	ctx.r[30].s64 = 62;
	// 832AEAB4: 396BC708  addi r11, r11, -0x38f8
	ctx.r[11].s64 = ctx.r[11].s64 + -14584;
	// 832AEAB8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AEABC: 3BEB00FC  addi r31, r11, 0xfc
	ctx.r[31].s64 = ctx.r[11].s64 + 252;
	// 832AEAC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AEAC4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AEAC8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AEACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AEAD0: 4AF17C99  bl 0x821c6768
	ctx.lr = 0x832AEAD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AEAD4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AEAD8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AEADC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEAE0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AEAE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AEAE8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AEAEC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEAF0: 4082FFE8  bne 0x832aead8
	if !ctx.cr[0].eq {
	pc = 0x832AEAD8; continue 'dispatch;
	}
	// 832AEAF4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AEAF8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AEAFC: 4080FFCC  bge 0x832aeac8
	if !ctx.cr[0].lt {
	pc = 0x832AEAC8; continue 'dispatch;
	}
	// 832AEB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AEB04: 4B9FA954  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AEB08: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AEB0C: 386B3C38  addi r3, r11, 0x3c38
	ctx.r[3].s64 = ctx.r[11].s64 + 15416;
	// 832AEB10: 4AF662C8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AEB18 size=104
	// 832AEB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AEB1C: 4B9FA8ED  bl 0x82ca9408
	ctx.lr = 0x832AEB20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AEB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AEB24: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AEB28: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 832AEB2C: 396B36B4  addi r11, r11, 0x36b4
	ctx.r[11].s64 = ctx.r[11].s64 + 14004;
	// 832AEB30: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AEB34: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 832AEB38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AEB3C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AEB40: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AEB44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AEB48: 4AF17C21  bl 0x821c6768
	ctx.lr = 0x832AEB4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AEB4C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AEB50: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AEB54: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEB58: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AEB5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AEB60: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AEB64: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEB68: 4082FFE8  bne 0x832aeb50
	if !ctx.cr[0].eq {
	pc = 0x832AEB50; continue 'dispatch;
	}
	// 832AEB6C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AEB70: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AEB74: 4080FFCC  bge 0x832aeb40
	if !ctx.cr[0].lt {
	pc = 0x832AEB40; continue 'dispatch;
	}
	// 832AEB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AEB7C: 4B9FA8DC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832AEB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AEB80 size=104
	// 832AEB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AEB84: 4B9FA885  bl 0x82ca9408
	ctx.lr = 0x832AEB88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AEB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AEB8C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AEB90: 3BC00027  li r30, 0x27
	ctx.r[30].s64 = 39;
	// 832AEB94: 396B36E0  addi r11, r11, 0x36e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14048;
	// 832AEB98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AEB9C: 3BEB00A0  addi r31, r11, 0xa0
	ctx.r[31].s64 = ctx.r[11].s64 + 160;
	// 832AEBA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AEBA4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AEBA8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AEBAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AEBB0: 4AF17BB9  bl 0x821c6768
	ctx.lr = 0x832AEBB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AEBB4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AEBB8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AEBBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEBC0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AEBC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AEBC8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AEBCC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEBD0: 4082FFE8  bne 0x832aebb8
	if !ctx.cr[0].eq {
	pc = 0x832AEBB8; continue 'dispatch;
	}
	// 832AEBD4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AEBD8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AEBDC: 4080FFCC  bge 0x832aeba8
	if !ctx.cr[0].lt {
	pc = 0x832AEBA8; continue 'dispatch;
	}
	// 832AEBE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AEBE4: 4B9FA874  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832AEBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AEBE8 size=264
	// 832AEBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AEBEC: 4B9FA81D  bl 0x82ca9408
	ctx.lr = 0x832AEBF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832AEBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AEBF4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AEBF8: 3BC00027  li r30, 0x27
	ctx.r[30].s64 = 39;
	// 832AEBFC: 396B3780  addi r11, r11, 0x3780
	ctx.r[11].s64 = ctx.r[11].s64 + 14208;
	// 832AEC00: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AEC04: 3BEB00A0  addi r31, r11, 0xa0
	ctx.r[31].s64 = ctx.r[11].s64 + 160;
	// 832AEC08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AEC0C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832AEC10: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AEC14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AEC18: 4AF17B51  bl 0x821c6768
	ctx.lr = 0x832AEC1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832AEC1C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832AEC20: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AEC24: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEC28: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AEC2C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AEC30: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AEC34: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AEC38: 4082FFE8  bne 0x832aec20
	if !ctx.cr[0].eq {
	pc = 0x832AEC20; continue 'dispatch;
	}
	// 832AEC3C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AEC40: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AEC44: 4080FFCC  bge 0x832aec10
	if !ctx.cr[0].lt {
	pc = 0x832AEC10; continue 'dispatch;
	}
	// 832AEC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AEC4C: 4B9FA80C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832AEC50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AEC54: 386BC844  addi r3, r11, -0x37bc
	ctx.r[3].s64 = ctx.r[11].s64 + -14268;
	// 832AEC58: 4AF66180  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832AECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AECF0 size=184
	// 832AECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AECF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AECF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AECFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AED00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AED04: 3BEBC87C  addi r31, r11, -0x3784
	ctx.r[31].s64 = ctx.r[11].s64 + -14212;
	// 832AED08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AED0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AED10: 419A0008  beq cr6, 0x832aed18
	if ctx.cr[6].eq {
	pc = 0x832AED18; continue 'dispatch;
	}
	// 832AED14: 4AF6D025  bl 0x8221bd38
	ctx.lr = 0x832AED18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AED18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AED1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AED20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AED24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AED28: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AED2C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AED30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AED3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AED40: 4E800020  blr
	return;
}

pub fn sub_832AEDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AEDA8 size=5136
	// 832AEDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AEDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AEDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AEDB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AEDB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AEDBC: 3BEBCB48  addi r31, r11, -0x34b8
	ctx.r[31].s64 = ctx.r[11].s64 + -13496;
	// 832AEDC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AEDC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AEDC8: 419A0008  beq cr6, 0x832aedd0
	if ctx.cr[6].eq {
	pc = 0x832AEDD0; continue 'dispatch;
	}
	// 832AEDCC: 4AF6CF6D  bl 0x8221bd38
	ctx.lr = 0x832AEDD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832AEDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AEDD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AEDD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AEDDC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AEDE0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AEDE4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AEDE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AEDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AEDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AEDF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AEDF8: 4E800020  blr
	return;
}

pub fn sub_832B0458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B0458 size=104
	// 832B0458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B045C: 4B9F8FAD  bl 0x82ca9408
	ctx.lr = 0x832B0460;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B0460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B0464: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0468: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B046C: 396BDAB8  addi r11, r11, -0x2548
	ctx.r[11].s64 = ctx.r[11].s64 + -9544;
	// 832B0470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B0474: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B0478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B047C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832B0480: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B0484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B0488: 4AF162E1  bl 0x821c6768
	ctx.lr = 0x832B048C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832B048C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832B0490: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B0494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0498: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B049C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B04A0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B04A4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B04A8: 4082FFE8  bne 0x832b0490
	if !ctx.cr[0].eq {
	pc = 0x832B0490; continue 'dispatch;
	}
	// 832B04AC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B04B0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B04B4: 4080FFCC  bge 0x832b0480
	if !ctx.cr[0].lt {
	pc = 0x832B0480; continue 'dispatch;
	}
	// 832B04B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B04BC: 4B9F8F9C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B04C0 size=2060
	// 832B04C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B04C4: 4B9F8F45  bl 0x82ca9408
	ctx.lr = 0x832B04C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B04C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B04CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B04D0: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B04D4: 396BDAF8  addi r11, r11, -0x2508
	ctx.r[11].s64 = ctx.r[11].s64 + -9480;
	// 832B04D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B04DC: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B04E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B04E4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832B04E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B04EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B04F0: 4AF16279  bl 0x821c6768
	ctx.lr = 0x832B04F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832B04F4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832B04F8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B04FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0500: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B0504: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B0508: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B050C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0510: 4082FFE8  bne 0x832b04f8
	if !ctx.cr[0].eq {
	pc = 0x832B04F8; continue 'dispatch;
	}
	// 832B0514: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B0518: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B051C: 4080FFCC  bge 0x832b04e8
	if !ctx.cr[0].lt {
	pc = 0x832B04E8; continue 'dispatch;
	}
	// 832B0520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B0524: 4B9F8F34  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B0528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B052C: 386BDAB4  addi r3, r11, -0x254c
	ctx.r[3].s64 = ctx.r[11].s64 + -9548;
	// 832B0530: 4AF648A8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B1118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1118 size=104
	// 832B1118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B111C: 4B9F82ED  bl 0x82ca9408
	ctx.lr = 0x832B1120;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B1120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1124: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1128: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B112C: 396BE080  addi r11, r11, -0x1f80
	ctx.r[11].s64 = ctx.r[11].s64 + -8064;
	// 832B1130: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B1134: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B1138: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B113C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832B1140: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B1144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B1148: 4AF15621  bl 0x821c6768
	ctx.lr = 0x832B114C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832B114C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832B1150: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B1154: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B1158: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B115C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B1160: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B1164: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B1168: 4082FFE8  bne 0x832b1150
	if !ctx.cr[0].eq {
	pc = 0x832B1150; continue 'dispatch;
	}
	// 832B116C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B1170: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B1174: 4080FFCC  bge 0x832b1140
	if !ctx.cr[0].lt {
	pc = 0x832B1140; continue 'dispatch;
	}
	// 832B1178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B117C: 4B9F82DC  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B1180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1180 size=344
	// 832B1180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1184: 4B9F8285  bl 0x82ca9408
	ctx.lr = 0x832B1188;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B1188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B118C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1190: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B1194: 396BE0C0  addi r11, r11, -0x1f40
	ctx.r[11].s64 = ctx.r[11].s64 + -8000;
	// 832B1198: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B119C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B11A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B11A4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832B11A8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B11AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B11B0: 4AF155B9  bl 0x821c6768
	ctx.lr = 0x832B11B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832B11B4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832B11B8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B11BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B11C0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B11C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B11C8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B11CC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B11D0: 4082FFE8  bne 0x832b11b8
	if !ctx.cr[0].eq {
	pc = 0x832B11B8; continue 'dispatch;
	}
	// 832B11D4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B11D8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B11DC: 4080FFCC  bge 0x832b11a8
	if !ctx.cr[0].lt {
	pc = 0x832B11A8; continue 'dispatch;
	}
	// 832B11E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B11E4: 4B9F8274  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B11E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B11EC: 386BE150  addi r3, r11, -0x1eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -7856;
	// 832B11F0: 4AF63BE8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B12D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B12D8 size=848
	// 832B12D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B12DC: 4B9F812D  bl 0x82ca9408
	ctx.lr = 0x832B12E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B12E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B12E4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B12E8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832B12EC: 396BE1E0  addi r11, r11, -0x1e20
	ctx.r[11].s64 = ctx.r[11].s64 + -7712;
	// 832B12F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B12F4: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832B12F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B12FC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	// 832B1300: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B1304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B1308: 4AF15461  bl 0x821c6768
	ctx.lr = 0x832B130C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 832B130C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 832B1310: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B1314: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B1318: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B131C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B1320: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B1324: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B1328: 4082FFE8  bne 0x832b1310
	if !ctx.cr[0].eq {
	pc = 0x832B1310; continue 'dispatch;
	}
	// 832B132C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B1330: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B1334: 4080FFCC  bge 0x832b1300
	if !ctx.cr[0].lt {
	pc = 0x832B1300; continue 'dispatch;
	}
	// 832B1338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B133C: 4B9F811C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B1340: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1344: 386BE248  addi r3, r11, -0x1db8
	ctx.r[3].s64 = ctx.r[11].s64 + -7608;
	// 832B1348: 4AF63A90  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B1628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1628 size=88
	// 832B1628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1638: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B163C: 3BEB2250  addi r31, r11, 0x2250
	ctx.r[31].s64 = ctx.r[11].s64 + 8784;
	// 832B1640: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B1644: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B1648: 419A0008  beq cr6, 0x832b1650
	if ctx.cr[6].eq {
	pc = 0x832B1650; continue 'dispatch;
	}
	// 832B164C: 4AF6A6ED  bl 0x8221bd38
	ctx.lr = 0x832B1650;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B1654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1658: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B165C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1660: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B1664: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B1668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B166C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1674: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1678: 4E800020  blr
	return;
}

pub fn sub_832B1680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1680 size=88
	// 832B1680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B168C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1690: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B1694: 3BEB2260  addi r31, r11, 0x2260
	ctx.r[31].s64 = ctx.r[11].s64 + 8800;
	// 832B1698: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B169C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B16A0: 419A0008  beq cr6, 0x832b16a8
	if ctx.cr[6].eq {
	pc = 0x832B16A8; continue 'dispatch;
	}
	// 832B16A4: 4AF6A695  bl 0x8221bd38
	ctx.lr = 0x832B16A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B16A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B16AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B16B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B16B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B16B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B16BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B16C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B16C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B16C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B16CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B16D0: 4E800020  blr
	return;
}

pub fn sub_832B16D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B16D8 size=88
	// 832B16D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B16DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B16E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B16E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B16E8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B16EC: 3BEB2270  addi r31, r11, 0x2270
	ctx.r[31].s64 = ctx.r[11].s64 + 8816;
	// 832B16F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B16F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B16F8: 419A0008  beq cr6, 0x832b1700
	if ctx.cr[6].eq {
	pc = 0x832B1700; continue 'dispatch;
	}
	// 832B16FC: 4AF6A63D  bl 0x8221bd38
	ctx.lr = 0x832B1700;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B1704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1708: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B170C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1710: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B1714: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B1718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B171C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1728: 4E800020  blr
	return;
}

pub fn sub_832B1730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1730 size=184
	// 832B1730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B173C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1740: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B1744: 3BEB2280  addi r31, r11, 0x2280
	ctx.r[31].s64 = ctx.r[11].s64 + 8832;
	// 832B1748: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B174C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B1750: 419A0008  beq cr6, 0x832b1758
	if ctx.cr[6].eq {
	pc = 0x832B1758; continue 'dispatch;
	}
	// 832B1754: 4AF6A5E5  bl 0x8221bd38
	ctx.lr = 0x832B1758;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B175C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1760: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B1764: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1768: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B176C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B1770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B177C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1780: 4E800020  blr
	return;
}

pub fn sub_832B17E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B17E8 size=376
	// 832B17E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B17EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B17F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B17F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B17F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832B17FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B1800: 3BEAE568  addi r31, r10, -0x1a98
	ctx.r[31].s64 = ctx.r[10].s64 + -6808;
	// 832B1804: 396B2544  addi r11, r11, 0x2544
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	// 832B1808: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B180C: 916AE568  stw r11, -0x1a98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6808 as u32), ctx.r[11].u32 ) };
	// 832B1810: 4B062189  bl 0x82313998
	ctx.lr = 0x832B1814;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313998);
	// 832B1814: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1818: 4B061FF1  bl 0x82313808
	ctx.lr = 0x832B181C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313808);
	// 832B181C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B182C: 4E800020  blr
	return;
	// 832B1830: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1834: 386BE578  addi r3, r11, -0x1a88
	ctx.r[3].s64 = ctx.r[11].s64 + -6792;
	// 832B1838: 4AF635A0  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B1960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1960 size=88
	// 832B1960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B196C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1970: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B1974: 3BEB1A60  addi r31, r11, 0x1a60
	ctx.r[31].s64 = ctx.r[11].s64 + 6752;
	// 832B1978: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B197C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B1980: 419A0008  beq cr6, 0x832b1988
	if ctx.cr[6].eq {
	pc = 0x832B1988; continue 'dispatch;
	}
	// 832B1984: 4AF6A3B5  bl 0x8221bd38
	ctx.lr = 0x832B1988;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B198C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1990: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B1994: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1998: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B199C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B19A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B19A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B19A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B19AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B19B0: 4E800020  blr
	return;
}

pub fn sub_832B19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B19B8 size=88
	// 832B19B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B19BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B19C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B19C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B19C8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B19CC: 3BEB1A70  addi r31, r11, 0x1a70
	ctx.r[31].s64 = ctx.r[11].s64 + 6768;
	// 832B19D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B19D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B19D8: 419A0008  beq cr6, 0x832b19e0
	if ctx.cr[6].eq {
	pc = 0x832B19E0; continue 'dispatch;
	}
	// 832B19DC: 4AF6A35D  bl 0x8221bd38
	ctx.lr = 0x832B19E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B19E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B19E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B19E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B19EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B19F0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B19F4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B19F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B19FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1A08: 4E800020  blr
	return;
}

pub fn sub_832B1A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1A10 size=88
	// 832B1A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1A20: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B1A24: 3BEB1A80  addi r31, r11, 0x1a80
	ctx.r[31].s64 = ctx.r[11].s64 + 6784;
	// 832B1A28: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B1A2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B1A30: 419A0008  beq cr6, 0x832b1a38
	if ctx.cr[6].eq {
	pc = 0x832B1A38; continue 'dispatch;
	}
	// 832B1A34: 4AF6A305  bl 0x8221bd38
	ctx.lr = 0x832B1A38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B1A3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1A40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B1A44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1A48: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B1A4C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B1A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1A60: 4E800020  blr
	return;
}

pub fn sub_832B1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1A68 size=136
	// 832B1A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1A78: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B1A7C: 3BEB1A90  addi r31, r11, 0x1a90
	ctx.r[31].s64 = ctx.r[11].s64 + 6800;
	// 832B1A80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B1A84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B1A88: 419A0008  beq cr6, 0x832b1a90
	if ctx.cr[6].eq {
	pc = 0x832B1A90; continue 'dispatch;
	}
	// 832B1A8C: 4AF6A2AD  bl 0x8221bd38
	ctx.lr = 0x832B1A90;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B1A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B1A94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B1A98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B1A9C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B1AA0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B1AA4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B1AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1AB8: 4E800020  blr
	return;
}

pub fn sub_832B1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1AF0 size=120
	// 832B1AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832B1B04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B1B08: 3BEAE63C  addi r31, r10, -0x19c4
	ctx.r[31].s64 = ctx.r[10].s64 + -6596;
	// 832B1B0C: 396B2544  addi r11, r11, 0x2544
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	// 832B1B10: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1B14: 916AE63C  stw r11, -0x19c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6596 as u32), ctx.r[11].u32 ) };
	// 832B1B18: 4B061E81  bl 0x82313998
	ctx.lr = 0x832B1B1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313998);
	// 832B1B1C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1B20: 4B061CE9  bl 0x82313808
	ctx.lr = 0x832B1B24;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313808);
	// 832B1B24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1B34: 4E800020  blr
	return;
	// 832B1B38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1B3C: 386BE64C  addi r3, r11, -0x19b4
	ctx.r[3].s64 = ctx.r[11].s64 + -6580;
	// 832B1B40: 4AF63298  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B1B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1B68 size=192
	// 832B1B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1B74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832B1B7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B1B80: 3BEAE658  addi r31, r10, -0x19a8
	ctx.r[31].s64 = ctx.r[10].s64 + -6568;
	// 832B1B84: 396B2544  addi r11, r11, 0x2544
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	// 832B1B88: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1B8C: 916AE658  stw r11, -0x19a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6568 as u32), ctx.r[11].u32 ) };
	// 832B1B90: 4B061E09  bl 0x82313998
	ctx.lr = 0x832B1B94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313998);
	// 832B1B94: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1B98: 4B061C71  bl 0x82313808
	ctx.lr = 0x832B1B9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313808);
	// 832B1B9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1BAC: 4E800020  blr
	return;
	// 832B1BB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1BB4: 386BE668  addi r3, r11, -0x1998
	ctx.r[3].s64 = ctx.r[11].s64 + -6552;
	// 832B1BB8: 4AF63220  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1C28 size=1000
	// 832B1C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1C34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832B1C3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B1C40: 3BEAE688  addi r31, r10, -0x1978
	ctx.r[31].s64 = ctx.r[10].s64 + -6520;
	// 832B1C44: 396B2544  addi r11, r11, 0x2544
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	// 832B1C48: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1C4C: 916AE688  stw r11, -0x1978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6520 as u32), ctx.r[11].u32 ) };
	// 832B1C50: 4B061D49  bl 0x82313998
	ctx.lr = 0x832B1C54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313998);
	// 832B1C54: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1C58: 4B061BB1  bl 0x82313808
	ctx.lr = 0x832B1C5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82313808);
	// 832B1C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1C6C: 4E800020  blr
	return;
	// 832B1C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1C74: 386BE698  addi r3, r11, -0x1968
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	// 832B1C78: 4AF63160  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B2010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2010 size=88
	// 832B2010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B201C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2020: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B2024: 3BEB3B40  addi r31, r11, 0x3b40
	ctx.r[31].s64 = ctx.r[11].s64 + 15168;
	// 832B2028: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B202C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2030: 419A0008  beq cr6, 0x832b2038
	if ctx.cr[6].eq {
	pc = 0x832B2038; continue 'dispatch;
	}
	// 832B2034: 4AF69D05  bl 0x8221bd38
	ctx.lr = 0x832B2038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B203C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2040: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2044: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2048: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B204C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B205C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2060: 4E800020  blr
	return;
}

pub fn sub_832B2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2068 size=88
	// 832B2068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2078: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B207C: 3BEB3B54  addi r31, r11, 0x3b54
	ctx.r[31].s64 = ctx.r[11].s64 + 15188;
	// 832B2080: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2084: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2088: 419A0008  beq cr6, 0x832b2090
	if ctx.cr[6].eq {
	pc = 0x832B2090; continue 'dispatch;
	}
	// 832B208C: 4AF69CAD  bl 0x8221bd38
	ctx.lr = 0x832B2090;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2098: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B209C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B20A0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B20A4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B20A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B20AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B20B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B20B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B20B8: 4E800020  blr
	return;
}

pub fn sub_832B20C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B20C0 size=2136
	// 832B20C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B20C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B20C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B20CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B20D0: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B20D4: 3BEB3B68  addi r31, r11, 0x3b68
	ctx.r[31].s64 = ctx.r[11].s64 + 15208;
	// 832B20D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B20DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B20E0: 419A0008  beq cr6, 0x832b20e8
	if ctx.cr[6].eq {
	pc = 0x832B20E8; continue 'dispatch;
	}
	// 832B20E4: 4AF69C55  bl 0x8221bd38
	ctx.lr = 0x832B20E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B20E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B20EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B20F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B20F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B20F8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B20FC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B210C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2110: 4E800020  blr
	return;
}

pub fn sub_832B2918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2918 size=136
	// 832B2918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B292C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2930: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2934: 3BEBE9F4  addi r31, r11, -0x160c
	ctx.r[31].s64 = ctx.r[11].s64 + -5644;
	// 832B2938: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B293C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B2940: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B2944: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2948: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B294C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2950: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B2954: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B2958: 419A0020  beq cr6, 0x832b2978
	if ctx.cr[6].eq {
	pc = 0x832B2978; continue 'dispatch;
	}
	// 832B295C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B2960: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B2964: 4AF693D5  bl 0x8221bd38
	ctx.lr = 0x832B2968;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2968: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B296C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B2970: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B2974: 409AFFE8  bne cr6, 0x832b295c
	if !ctx.cr[6].eq {
	pc = 0x832B295C; continue 'dispatch;
	}
	// 832B2978: 4AF693C1  bl 0x8221bd38
	ctx.lr = 0x832B297C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B297C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2980: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B298C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2998: 4E800020  blr
	return;
}

pub fn sub_832B29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B29A0 size=120
	// 832B29A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B29A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B29A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B29AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B29B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B29B4: 3BEBEA00  addi r31, r11, -0x1600
	ctx.r[31].s64 = ctx.r[11].s64 + -5632;
	// 832B29B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B29BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B29C0: 419A0008  beq cr6, 0x832b29c8
	if ctx.cr[6].eq {
	pc = 0x832B29C8; continue 'dispatch;
	}
	// 832B29C4: 4AF69375  bl 0x8221bd38
	ctx.lr = 0x832B29C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B29C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B29CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B29D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B29D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B29D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B29DC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B29E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B29E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B29E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B29EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B29F0: 4E800020  blr
	return;
}

pub fn sub_832B2A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2A18 size=152
	// 832B2A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2A28: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B2A2C: 807FEA18  lwz r3, -0x15e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-5608 as u32) ) } as u64;
	// 832B2A30: 4AF69309  bl 0x8221bd38
	ctx.lr = 0x832B2A34;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2A38: 917FEA18  stw r11, -0x15e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-5608 as u32), ctx.r[11].u32 ) };
	// 832B2A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2A4C: 4E800020  blr
	return;
	// 832B2A50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A54: 386BEA1C  addi r3, r11, -0x15e4
	ctx.r[3].s64 = ctx.r[11].s64 + -5604;
	// 832B2A58: 4B10C5B0  b 0x823bf008
	crate::recompiler::externs::call(&mut ctx, base, 0x823BF008);
	return;
}

pub fn sub_832B2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2AB0 size=12
	// 832B2AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2AB4: 4B9F6955  bl 0x82ca9408
	ctx.lr = 0x832B2AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B2AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2B70 size=12
	// 832B2B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2B74: 4B9F6895  bl 0x82ca9408
	ctx.lr = 0x832B2B78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B2B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B2CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2CE0 size=104
	// 832B2CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2CF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CF4: 3BEBF7A8  addi r31, r11, -0x858
	ctx.r[31].s64 = ctx.r[11].s64 + -2136;
	// 832B2CF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2CFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2D00: 419A0008  beq cr6, 0x832b2d08
	if ctx.cr[6].eq {
	pc = 0x832B2D08; continue 'dispatch;
	}
	// 832B2D04: 4AF69035  bl 0x8221bd38
	ctx.lr = 0x832B2D08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2D0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2D10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2D14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2D18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B2D1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2D30: 4E800020  blr
	return;
}

pub fn sub_832B2D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2D48 size=120
	// 832B2D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2D58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2D5C: 3BEBF7C8  addi r31, r11, -0x838
	ctx.r[31].s64 = ctx.r[11].s64 + -2104;
	// 832B2D60: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832B2D64: 4AF62075  bl 0x82214dd8
	ctx.lr = 0x832B2D68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832B2D68: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832B2D6C: 4AF6206D  bl 0x82214dd8
	ctx.lr = 0x832B2D70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832B2D70: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B2D74: 4AF62065  bl 0x82214dd8
	ctx.lr = 0x832B2D78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832B2D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2D7C: 4AF6205D  bl 0x82214dd8
	ctx.lr = 0x832B2D80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	// 832B2D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2D90: 4E800020  blr
	return;
}

pub fn sub_832B2DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2DC0 size=88
	// 832B2DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2DD4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2DD8: 83FEF7DC  lwz r31, -0x824(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2084 as u32) ) } as u64;
	// 832B2DDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2DE0: 419A0014  beq cr6, 0x832b2df4
	if ctx.cr[6].eq {
	pc = 0x832B2DF4; continue 'dispatch;
	}
	// 832B2DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2DE8: 4AFC2521  bl 0x82275308
	ctx.lr = 0x832B2DEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82275308);
	// 832B2DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2DF0: 4AF68F49  bl 0x8221bd38
	ctx.lr = 0x832B2DF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2DF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2DF8: 917EF7DC  stw r11, -0x824(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2084 as u32), ctx.r[11].u32 ) };
	// 832B2DFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2E08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2E10: 4E800020  blr
	return;
}

pub fn sub_832B2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2E18 size=88
	// 832B2E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2E2C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2E30: 83FEF7E0  lwz r31, -0x820(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2080 as u32) ) } as u64;
	// 832B2E34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2E38: 419A0014  beq cr6, 0x832b2e4c
	if ctx.cr[6].eq {
	pc = 0x832B2E4C; continue 'dispatch;
	}
	// 832B2E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E40: 4AFEAC59  bl 0x8229da98
	ctx.lr = 0x832B2E44;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E48: 4AF68EF1  bl 0x8221bd38
	ctx.lr = 0x832B2E4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2E50: 917EF7E0  stw r11, -0x820(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2080 as u32), ctx.r[11].u32 ) };
	// 832B2E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2E68: 4E800020  blr
	return;
}

pub fn sub_832B2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2E70 size=88
	// 832B2E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2E84: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2E88: 83FEF7E4  lwz r31, -0x81c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2076 as u32) ) } as u64;
	// 832B2E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2E90: 419A0014  beq cr6, 0x832b2ea4
	if ctx.cr[6].eq {
	pc = 0x832B2EA4; continue 'dispatch;
	}
	// 832B2E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E98: 4AFEAC01  bl 0x8229da98
	ctx.lr = 0x832B2E9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EA0: 4AF68E99  bl 0x8221bd38
	ctx.lr = 0x832B2EA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2EA8: 917EF7E4  stw r11, -0x81c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2076 as u32), ctx.r[11].u32 ) };
	// 832B2EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2EB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2EC0: 4E800020  blr
	return;
}

pub fn sub_832B2EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2EC8 size=88
	// 832B2EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2EDC: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2EE0: 83FEF7E8  lwz r31, -0x818(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2072 as u32) ) } as u64;
	// 832B2EE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2EE8: 419A0014  beq cr6, 0x832b2efc
	if ctx.cr[6].eq {
	pc = 0x832B2EFC; continue 'dispatch;
	}
	// 832B2EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EF0: 4AFEABA9  bl 0x8229da98
	ctx.lr = 0x832B2EF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EF8: 4AF68E41  bl 0x8221bd38
	ctx.lr = 0x832B2EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2EFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2F00: 917EF7E8  stw r11, -0x818(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2072 as u32), ctx.r[11].u32 ) };
	// 832B2F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2F10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2F18: 4E800020  blr
	return;
}

pub fn sub_832B2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2F20 size=88
	// 832B2F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2F34: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2F38: 83FEF7EC  lwz r31, -0x814(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2068 as u32) ) } as u64;
	// 832B2F3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2F40: 419A0014  beq cr6, 0x832b2f54
	if ctx.cr[6].eq {
	pc = 0x832B2F54; continue 'dispatch;
	}
	// 832B2F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2F48: 4AFEAB51  bl 0x8229da98
	ctx.lr = 0x832B2F4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2F50: 4AF68DE9  bl 0x8221bd38
	ctx.lr = 0x832B2F54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2F54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2F58: 917EF7EC  stw r11, -0x814(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2068 as u32), ctx.r[11].u32 ) };
	// 832B2F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2F68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2F70: 4E800020  blr
	return;
}

pub fn sub_832B2F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2F78 size=88
	// 832B2F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2F8C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2F90: 83FEF7F0  lwz r31, -0x810(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2064 as u32) ) } as u64;
	// 832B2F94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2F98: 419A0014  beq cr6, 0x832b2fac
	if ctx.cr[6].eq {
	pc = 0x832B2FAC; continue 'dispatch;
	}
	// 832B2F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FA0: 4AFEAAF9  bl 0x8229da98
	ctx.lr = 0x832B2FA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FA8: 4AF68D91  bl 0x8221bd38
	ctx.lr = 0x832B2FAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B2FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2FB0: 917EF7F0  stw r11, -0x810(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2064 as u32), ctx.r[11].u32 ) };
	// 832B2FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2FC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2FC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2FC8: 4E800020  blr
	return;
}

pub fn sub_832B2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2FD0 size=88
	// 832B2FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2FE4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2FE8: 83FEF7F4  lwz r31, -0x80c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2060 as u32) ) } as u64;
	// 832B2FEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2FF0: 419A0014  beq cr6, 0x832b3004
	if ctx.cr[6].eq {
	pc = 0x832B3004; continue 'dispatch;
	}
	// 832B2FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FF8: 4AFEAAA1  bl 0x8229da98
	ctx.lr = 0x832B2FFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	// 832B2FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3000: 4AF68D39  bl 0x8221bd38
	ctx.lr = 0x832B3004;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3008: 917EF7F4  stw r11, -0x80c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2060 as u32), ctx.r[11].u32 ) };
	// 832B300C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B301C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3020: 4E800020  blr
	return;
}

pub fn sub_832B3028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3028 size=224
	// 832B3028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B302C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3038: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B303C: 807FF7F8  lwz r3, -0x808(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2056 as u32) ) } as u64;
	// 832B3040: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3044: 419A0044  beq cr6, 0x832b3088
	if ctx.cr[6].eq {
	pc = 0x832B3088; continue 'dispatch;
	}
	// 832B3048: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B304C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B3050: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B3054: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B3058: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B305C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B3060: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B3064: 4082FFE8  bne 0x832b304c
	if !ctx.cr[0].eq {
	pc = 0x832B304C; continue 'dispatch;
	}
	// 832B3068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B306C: 409A0014  bne cr6, 0x832b3080
	if !ctx.cr[6].eq {
	pc = 0x832B3080; continue 'dispatch;
	}
	// 832B3070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3074: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B307C: 4E800421  bctrl
	ctx.lr = 0x832B3080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B3080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3084: 917FF7F8  stw r11, -0x808(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-2056 as u32), ctx.r[11].u32 ) };
	// 832B3088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B308C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3098: 4E800020  blr
	return;
}

pub fn sub_832B3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3108 size=88
	// 832B3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B3114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B311C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B3120: 83FEF848  lwz r31, -0x7b8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1976 as u32) ) } as u64;
	// 832B3124: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B3128: 419A0014  beq cr6, 0x832b313c
	if ctx.cr[6].eq {
	pc = 0x832B313C; continue 'dispatch;
	}
	// 832B312C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3130: 4B7C8B09  bl 0x82a7bc38
	ctx.lr = 0x832B3134;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A7BC38);
	// 832B3134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3138: 4AF68C01  bl 0x8221bd38
	ctx.lr = 0x832B313C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B313C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3140: 917EF848  stw r11, -0x7b8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1976 as u32), ctx.r[11].u32 ) };
	// 832B3144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3158: 4E800020  blr
	return;
}

pub fn sub_832B3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3160 size=96
	// 832B3160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B316C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3174: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B3178: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B317C: 3BDFA978  addi r30, r31, -0x5688
	ctx.r[30].s64 = ctx.r[31].s64 + -22152;
	// 832B3180: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B3184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3188: 917FA978  stw r11, -0x5688(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22152 as u32), ctx.r[11].u32 ) };
	// 832B318C: 4AF48E7D  bl 0x821fc008
	ctx.lr = 0x832B3190;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3190: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3198: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B319C: 917FA978  stw r11, -0x5688(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22152 as u32), ctx.r[11].u32 ) };
	// 832B31A0: 4AF48E69  bl 0x821fc008
	ctx.lr = 0x832B31A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B31A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B31B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B31B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B31B8: 4E800020  blr
	return;
}

pub fn sub_832B31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B31C0 size=96
	// 832B31C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B31C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B31C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B31CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B31D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B31D4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B31D8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B31DC: 3BDFA980  addi r30, r31, -0x5680
	ctx.r[30].s64 = ctx.r[31].s64 + -22144;
	// 832B31E0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B31E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B31E8: 917FA980  stw r11, -0x5680(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22144 as u32), ctx.r[11].u32 ) };
	// 832B31EC: 4AF48E1D  bl 0x821fc008
	ctx.lr = 0x832B31F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B31F0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B31F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B31F8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B31FC: 917FA980  stw r11, -0x5680(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22144 as u32), ctx.r[11].u32 ) };
	// 832B3200: 4AF48E09  bl 0x821fc008
	ctx.lr = 0x832B3204;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B320C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3218: 4E800020  blr
	return;
}

pub fn sub_832B3220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3220 size=96
	// 832B3220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B322C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3234: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B3238: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B323C: 3BDFA988  addi r30, r31, -0x5678
	ctx.r[30].s64 = ctx.r[31].s64 + -22136;
	// 832B3240: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B3244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3248: 917FA988  stw r11, -0x5678(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22136 as u32), ctx.r[11].u32 ) };
	// 832B324C: 4AF48DBD  bl 0x821fc008
	ctx.lr = 0x832B3250;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3250: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3258: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B325C: 917FA988  stw r11, -0x5678(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22136 as u32), ctx.r[11].u32 ) };
	// 832B3260: 4AF48DA9  bl 0x821fc008
	ctx.lr = 0x832B3264;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B326C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3278: 4E800020  blr
	return;
}

pub fn sub_832B3280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3280 size=88
	// 832B3280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B328C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3290: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3294: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3298: 3BEBA990  addi r31, r11, -0x5670
	ctx.r[31].s64 = ctx.r[11].s64 + -22128;
	// 832B329C: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B32A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B32A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B32A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B32AC: 419A0018  beq cr6, 0x832b32c4
	if ctx.cr[6].eq {
	pc = 0x832B32C4; continue 'dispatch;
	}
	// 832B32B0: 4AF49071  bl 0x821fc320
	ctx.lr = 0x832B32B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B32B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B32B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B32BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B32C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B32C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B32C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B32CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B32D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B32D4: 4E800020  blr
	return;
}

pub fn sub_832B32D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B32D8 size=216
	// 832B32D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B32DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B32E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B32E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B32E8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B32EC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B32F0: 3BEBA99C  addi r31, r11, -0x5664
	ctx.r[31].s64 = ctx.r[11].s64 + -22116;
	// 832B32F4: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B32F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B32FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3304: 419A0018  beq cr6, 0x832b331c
	if ctx.cr[6].eq {
	pc = 0x832B331C; continue 'dispatch;
	}
	// 832B3308: 4AF49019  bl 0x821fc320
	ctx.lr = 0x832B330C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B330C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3314: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3318: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B331C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B332C: 4E800020  blr
	return;
	// 832B3330: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3334: 386BA9A8  addi r3, r11, -0x5658
	ctx.r[3].s64 = ctx.r[11].s64 + -22104;
	// 832B3338: 4AF48C58  b 0x821fbf90
	crate::recompiler::externs::call(&mut ctx, base, 0x821FBF90);
	return;
}

pub fn sub_832B33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B33B0 size=80
	// 832B33B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B33B4: 4B9F6059  bl 0x82ca940c
	ctx.lr = 0x832B33B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 832B33B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B33BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B33C0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B33C4: 396BF970  addi r11, r11, -0x690
	ctx.r[11].s64 = ctx.r[11].s64 + -1680;
	// 832B33C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832B33CC: 3BEB0044  addi r31, r11, 0x44
	ctx.r[31].s64 = ctx.r[11].s64 + 68;
	// 832B33D0: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B33D4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B33D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B33DC: 419A0008  beq cr6, 0x832b33e4
	if ctx.cr[6].eq {
	pc = 0x832B33E4; continue 'dispatch;
	}
	// 832B33E0: 4AF68959  bl 0x8221bd38
	ctx.lr = 0x832B33E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B33E4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B33E8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B33EC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832B33F0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 832B33F4: 4080FFDC  bge 0x832b33d0
	if !ctx.cr[0].lt {
	pc = 0x832B33D0; continue 'dispatch;
	}
	// 832B33F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B33FC: 4B9F6060  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_832B3400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3400 size=80
	// 832B3400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B340C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3410: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B3414: 807FF84C  lwz r3, -0x7b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-1972 as u32) ) } as u64;
	// 832B3418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B341C: 419A0018  beq cr6, 0x832b3434
	if ctx.cr[6].eq {
	pc = 0x832B3434; continue 'dispatch;
	}
	// 832B3420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3424: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B3428: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B342C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B3430: 4E800421  bctrl
	ctx.lr = 0x832B3434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B3434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3438: 917FF84C  stw r11, -0x7b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-1972 as u32), ctx.r[11].u32 ) };
	// 832B343C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B344C: 4E800020  blr
	return;
}

pub fn sub_832B3450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3450 size=88
	// 832B3450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3454: 4B9F5FB5  bl 0x82ca9408
	ctx.lr = 0x832B3458;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B345C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3460: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 832B3464: 396BF9B0  addi r11, r11, -0x650
	ctx.r[11].s64 = ctx.r[11].s64 + -1616;
	// 832B3468: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B346C: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	// 832B3470: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 832B3474: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3478: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B347C: 419A0018  beq cr6, 0x832b3494
	if ctx.cr[6].eq {
	pc = 0x832B3494; continue 'dispatch;
	}
	// 832B3480: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3484: 4AF688B5  bl 0x8221bd38
	ctx.lr = 0x832B3488;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3488: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B348C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3490: 4AF688A9  bl 0x8221bd38
	ctx.lr = 0x832B3494;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3494: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832B3498: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B349C: 4080FFD4  bge 0x832b3470
	if !ctx.cr[0].lt {
	pc = 0x832B3470; continue 'dispatch;
	}
	// 832B34A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B34A4: 4B9F5FB4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B34A8 size=128
	// 832B34A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B34AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B34B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B34B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B34B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B34BC: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B34C0: 83FEF9C0  lwz r31, -0x640(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1600 as u32) ) } as u64;
	// 832B34C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B34C8: 419A003C  beq cr6, 0x832b3504
	if ctx.cr[6].eq {
	pc = 0x832B3504; continue 'dispatch;
	}
	// 832B34CC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832B34D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B34D4: 419A0008  beq cr6, 0x832b34dc
	if ctx.cr[6].eq {
	pc = 0x832B34DC; continue 'dispatch;
	}
	// 832B34D8: 4B88A2F1  bl 0x82b3d7c8
	ctx.lr = 0x832B34DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B3D7C8);
	// 832B34DC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832B34E0: 4B3871A9  bl 0x8263a688
	ctx.lr = 0x832B34E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 832B34E4: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832B34E8: 4B3871A1  bl 0x8263a688
	ctx.lr = 0x832B34EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 832B34EC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832B34F0: 4B7EE169  bl 0x82aa1658
	ctx.lr = 0x832B34F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1658);
	// 832B34F4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B34F8: 4B7EE161  bl 0x82aa1658
	ctx.lr = 0x832B34FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1658);
	// 832B34FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3500: 4AF68839  bl 0x8221bd38
	ctx.lr = 0x832B3504;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3508: 917EF9C0  stw r11, -0x640(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1600 as u32), ctx.r[11].u32 ) };
	// 832B350C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3518: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B351C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3520: 4E800020  blr
	return;
}

pub fn sub_832B3528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3528 size=136
	// 832B3528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B352C: 4B9F5EDD  bl 0x82ca9408
	ctx.lr = 0x832B3530;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3534: 3F80834A  lis r28, -0x7cb6
	ctx.r[28].s64 = -2092302336;
	// 832B3538: 83DCF9C4  lwz r30, -0x63c(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-1596 as u32) ) } as u64;
	// 832B353C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832B3540: 419A005C  beq cr6, 0x832b359c
	if ctx.cr[6].eq {
	pc = 0x832B359C; continue 'dispatch;
	}
	// 832B3544: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 832B3548: 3BFE0024  addi r31, r30, 0x24
	ctx.r[31].s64 = ctx.r[30].s64 + 36;
	// 832B354C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832B3550: 419A0018  beq cr6, 0x832b3568
	if ctx.cr[6].eq {
	pc = 0x832B3568; continue 'dispatch;
	}
	// 832B3554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3558: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B355C: 4B7EE4AD  bl 0x82aa1a08
	ctx.lr = 0x832B3560;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1A08);
	// 832B3560: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3564: 4AF687D5  bl 0x8221bd38
	ctx.lr = 0x832B3568;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3568: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832B356C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 832B3570: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832B3574: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 832B3578: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 832B357C: 4B7EE0DD  bl 0x82aa1658
	ctx.lr = 0x832B3580;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1658);
	// 832B3580: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 832B3584: 4B7EDF15  bl 0x82aa1498
	ctx.lr = 0x832B3588;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1498);
	// 832B3588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B358C: 4AF687AD  bl 0x8221bd38
	ctx.lr = 0x832B3590;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3590: 93BCF9C4  stw r29, -0x63c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1596 as u32), ctx.r[29].u32 ) };
	// 832B3594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3598: 4B9F5EC0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B359C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B35A0: 917CF9C4  stw r11, -0x63c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1596 as u32), ctx.r[11].u32 ) };
	// 832B35A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B35A8: 4B9F5EB0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B35B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B35B0 size=160
	// 832B35B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B35B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B35B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B35BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B35C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B35C4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B35C8: 83FEF9C8  lwz r31, -0x638(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1592 as u32) ) } as u64;
	// 832B35CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B35D0: 419A003C  beq cr6, 0x832b360c
	if ctx.cr[6].eq {
	pc = 0x832B360C; continue 'dispatch;
	}
	// 832B35D4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832B35D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B35DC: 419A0008  beq cr6, 0x832b35e4
	if ctx.cr[6].eq {
	pc = 0x832B35E4; continue 'dispatch;
	}
	// 832B35E0: 4B88A1E9  bl 0x82b3d7c8
	ctx.lr = 0x832B35E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B3D7C8);
	// 832B35E4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832B35E8: 4B3870A1  bl 0x8263a688
	ctx.lr = 0x832B35EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 832B35EC: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832B35F0: 4B387099  bl 0x8263a688
	ctx.lr = 0x832B35F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8263A688);
	// 832B35F4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832B35F8: 4B7EE061  bl 0x82aa1658
	ctx.lr = 0x832B35FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1658);
	// 832B35FC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B3600: 4B7EE059  bl 0x82aa1658
	ctx.lr = 0x832B3604;
	crate::recompiler::externs::call(&mut ctx, base, 0x82AA1658);
	// 832B3604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3608: 4AF68731  bl 0x8221bd38
	ctx.lr = 0x832B360C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B360C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3610: 917EF9C8  stw r11, -0x638(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1592 as u32), ctx.r[11].u32 ) };
	// 832B3614: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B361C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3620: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3628: 4E800020  blr
	return;
}

pub fn sub_832B3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3650 size=152
	// 832B3650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3654: 4B9F5DB5  bl 0x82ca9408
	ctx.lr = 0x832B3658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B365C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3660: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B3664: 3BCBF9D8  addi r30, r11, -0x628
	ctx.r[30].s64 = ctx.r[11].s64 + -1576;
	// 832B3668: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 832B366C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3670: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3674: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3678: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B367C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3680: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3684: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3688: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B368C: 419A0034  beq cr6, 0x832b36c0
	if ctx.cr[6].eq {
	pc = 0x832B36C0; continue 'dispatch;
	}
	// 832B3690: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B3694: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B369C: 419A000C  beq cr6, 0x832b36a8
	if ctx.cr[6].eq {
	pc = 0x832B36A8; continue 'dispatch;
	}
	// 832B36A0: 4B8D3F29  bl 0x82b875c8
	ctx.lr = 0x832B36A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B875C8);
	// 832B36A4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 832B36A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B36AC: 4AF6868D  bl 0x8221bd38
	ctx.lr = 0x832B36B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B36B0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B36B4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 832B36B8: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B36BC: 409AFFD4  bne cr6, 0x832b3690
	if !ctx.cr[6].eq {
	pc = 0x832B3690; continue 'dispatch;
	}
	// 832B36C0: 4AF68679  bl 0x8221bd38
	ctx.lr = 0x832B36C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B36C4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 832B36C8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B36D0: 4B9F5D88  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B36E8 size=152
	// 832B36E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B36EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B36F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B36F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B36F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B36FC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3700: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3704: 3BEBF9F0  addi r31, r11, -0x610
	ctx.r[31].s64 = ctx.r[11].s64 + -1552;
	// 832B3708: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B370C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3710: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B3714: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3718: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B371C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3720: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3724: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B3728: 419A0020  beq cr6, 0x832b3748
	if ctx.cr[6].eq {
	pc = 0x832B3748; continue 'dispatch;
	}
	// 832B372C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B3730: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3734: 4AF68605  bl 0x8221bd38
	ctx.lr = 0x832B3738;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3738: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B373C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B3740: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3744: 409AFFE8  bne cr6, 0x832b372c
	if !ctx.cr[6].eq {
	pc = 0x832B372C; continue 'dispatch;
	}
	// 832B3748: 4AF685F1  bl 0x8221bd38
	ctx.lr = 0x832B374C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B374C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3750: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B375C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3768: 4E800020  blr
	return;
}

pub fn sub_832B3780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3780 size=88
	// 832B3780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B378C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3790: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3794: 3BEBF9FC  addi r31, r11, -0x604
	ctx.r[31].s64 = ctx.r[11].s64 + -1540;
	// 832B3798: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B379C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B37A0: 419A0008  beq cr6, 0x832b37a8
	if ctx.cr[6].eq {
	pc = 0x832B37A8; continue 'dispatch;
	}
	// 832B37A4: 4AF68595  bl 0x8221bd38
	ctx.lr = 0x832B37A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B37A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B37AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B37B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B37B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B37B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B37BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B37C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B37C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B37C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B37CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B37D0: 4E800020  blr
	return;
}

pub fn sub_832B37D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B37D8 size=80
	// 832B37D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B37DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B37E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B37E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B37E8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B37EC: 807FFA0C  lwz r3, -0x5f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-1524 as u32) ) } as u64;
	// 832B37F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B37F4: 419A0018  beq cr6, 0x832b380c
	if ctx.cr[6].eq {
	pc = 0x832B380C; continue 'dispatch;
	}
	// 832B37F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B37FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B3800: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3804: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B3808: 4E800421  bctrl
	ctx.lr = 0x832B380C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B380C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3810: 917FFA0C  stw r11, -0x5f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-1524 as u32), ctx.r[11].u32 ) };
	// 832B3814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B381C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3824: 4E800020  blr
	return;
}

pub fn sub_832B3828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3828 size=120
	// 832B3828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B382C: 4B9F5BDD  bl 0x82ca9408
	ctx.lr = 0x832B3830;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3834: 3F80834A  lis r28, -0x7cb6
	ctx.r[28].s64 = -2092302336;
	// 832B3838: 83BCFA10  lwz r29, -0x5f0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-1520 as u32) ) } as u64;
	// 832B383C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 832B3840: 419A0028  beq cr6, 0x832b3868
	if ctx.cr[6].eq {
	pc = 0x832B3868; continue 'dispatch;
	}
	// 832B3844: 3BFD0100  addi r31, r29, 0x100
	ctx.r[31].s64 = ctx.r[29].s64 + 256;
	// 832B3848: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	// 832B384C: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3854: 4AF042C5  bl 0x821b7b18
	ctx.lr = 0x832B3858;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B3858: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B385C: 4080FFF0  bge 0x832b384c
	if !ctx.cr[0].lt {
	pc = 0x832B384C; continue 'dispatch;
	}
	// 832B3860: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832B3864: 4AF684D5  bl 0x8221bd38
	ctx.lr = 0x832B3868;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B386C: 917CFA10  stw r11, -0x5f0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1520 as u32), ctx.r[11].u32 ) };
	// 832B3870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3874: 4B9F5BE4  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B3878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B387C: 806BFA14  lwz r3, -0x5ec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1516 as u32) ) } as u64;
	// 832B3880: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3884: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B3888: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B388C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B3890: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B3894: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 832B3898: 4E800020  blr
	return;
}

pub fn sub_832B38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B38A0 size=320
	// 832B38A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B38A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B38A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B38AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B38B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B38B4: 3BEBFA18  addi r31, r11, -0x5e8
	ctx.r[31].s64 = ctx.r[11].s64 + -1512;
	// 832B38B8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832B38BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B38C0: 419A0008  beq cr6, 0x832b38c8
	if ctx.cr[6].eq {
	pc = 0x832B38C8; continue 'dispatch;
	}
	// 832B38C4: 4AF68475  bl 0x8221bd38
	ctx.lr = 0x832B38C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B38C8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 832B38CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B38D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B38D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B38D8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 832B38DC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 832B38E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B38E4: 913F002C  stw r9, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 832B38E8: 419A0008  beq cr6, 0x832b38f0
	if ctx.cr[6].eq {
	pc = 0x832B38F0; continue 'dispatch;
	}
	// 832B38EC: 4AF6844D  bl 0x8221bd38
	ctx.lr = 0x832B38F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B38F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B38F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B38F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B38FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3900: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 832B3904: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 832B3908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B390C: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 832B3910: 419A0008  beq cr6, 0x832b3918
	if ctx.cr[6].eq {
	pc = 0x832B3918; continue 'dispatch;
	}
	// 832B3914: 4AF68425  bl 0x8221bd38
	ctx.lr = 0x832B3918;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B391C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3920: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3924: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3928: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B392C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B393C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3940: 4E800020  blr
	return;
}

pub fn sub_832B39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B39E0 size=88
	// 832B39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B39E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B39E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B39EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B39F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39F4: 3BEBFAB0  addi r31, r11, -0x550
	ctx.r[31].s64 = ctx.r[11].s64 + -1360;
	// 832B39F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B39FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3A00: 419A0008  beq cr6, 0x832b3a08
	if ctx.cr[6].eq {
	pc = 0x832B3A08; continue 'dispatch;
	}
	// 832B3A04: 4AF68335  bl 0x8221bd38
	ctx.lr = 0x832B3A08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3A10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3A14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3A18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3A1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3A30: 4E800020  blr
	return;
}

pub fn sub_832B3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3A38 size=88
	// 832B3A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3A48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3A4C: 3BEBFAC0  addi r31, r11, -0x540
	ctx.r[31].s64 = ctx.r[11].s64 + -1344;
	// 832B3A50: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3A54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3A58: 419A0008  beq cr6, 0x832b3a60
	if ctx.cr[6].eq {
	pc = 0x832B3A60; continue 'dispatch;
	}
	// 832B3A5C: 4AF682DD  bl 0x8221bd38
	ctx.lr = 0x832B3A60;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3A64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3A68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3A6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3A70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3A74: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3A88: 4E800020  blr
	return;
}

pub fn sub_832B3A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3A90 size=88
	// 832B3A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3A9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3AA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3AA4: 3BEBFAD0  addi r31, r11, -0x530
	ctx.r[31].s64 = ctx.r[11].s64 + -1328;
	// 832B3AA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3AAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3AB0: 419A0008  beq cr6, 0x832b3ab8
	if ctx.cr[6].eq {
	pc = 0x832B3AB8; continue 'dispatch;
	}
	// 832B3AB4: 4AF68285  bl 0x8221bd38
	ctx.lr = 0x832B3AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3ABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3AC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3AC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3AC8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3ACC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3AE0: 4E800020  blr
	return;
}

pub fn sub_832B3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3AE8 size=88
	// 832B3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3AF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3AF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3AF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3AFC: 3BEBFAE0  addi r31, r11, -0x520
	ctx.r[31].s64 = ctx.r[11].s64 + -1312;
	// 832B3B00: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3B04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3B08: 419A0008  beq cr6, 0x832b3b10
	if ctx.cr[6].eq {
	pc = 0x832B3B10; continue 'dispatch;
	}
	// 832B3B0C: 4AF6822D  bl 0x8221bd38
	ctx.lr = 0x832B3B10;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3B14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3B18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3B1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3B20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3B24: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3B34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3B38: 4E800020  blr
	return;
}

pub fn sub_832B3B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3B40 size=88
	// 832B3B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3B50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3B54: 3BEBFAF0  addi r31, r11, -0x510
	ctx.r[31].s64 = ctx.r[11].s64 + -1296;
	// 832B3B58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3B5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3B60: 419A0008  beq cr6, 0x832b3b68
	if ctx.cr[6].eq {
	pc = 0x832B3B68; continue 'dispatch;
	}
	// 832B3B64: 4AF681D5  bl 0x8221bd38
	ctx.lr = 0x832B3B68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3B70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3B74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3B78: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3B7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3B90: 4E800020  blr
	return;
}

pub fn sub_832B3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3B98 size=88
	// 832B3B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3BA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3BAC: 3BEBFB04  addi r31, r11, -0x4fc
	ctx.r[31].s64 = ctx.r[11].s64 + -1276;
	// 832B3BB0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3BB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3BB8: 419A0008  beq cr6, 0x832b3bc0
	if ctx.cr[6].eq {
	pc = 0x832B3BC0; continue 'dispatch;
	}
	// 832B3BBC: 4AF6817D  bl 0x8221bd38
	ctx.lr = 0x832B3BC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3BC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3BC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3BCC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3BD0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3BD4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3BE8: 4E800020  blr
	return;
}

pub fn sub_832B3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3BF0 size=136
	// 832B3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3BF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3BFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3C00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3C04: 3BEBFB14  addi r31, r11, -0x4ec
	ctx.r[31].s64 = ctx.r[11].s64 + -1260;
	// 832B3C08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3C0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3C10: 419A0008  beq cr6, 0x832b3c18
	if ctx.cr[6].eq {
	pc = 0x832B3C18; continue 'dispatch;
	}
	// 832B3C14: 4AF68125  bl 0x8221bd38
	ctx.lr = 0x832B3C18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3C1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3C20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3C24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3C28: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3C2C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3C40: 4E800020  blr
	return;
}

pub fn sub_832B3C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3C78 size=136
	// 832B3C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3C84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3C88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3C8C: 3BEBFB30  addi r31, r11, -0x4d0
	ctx.r[31].s64 = ctx.r[11].s64 + -1232;
	// 832B3C90: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3C94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3C98: 419A0008  beq cr6, 0x832b3ca0
	if ctx.cr[6].eq {
	pc = 0x832B3CA0; continue 'dispatch;
	}
	// 832B3C9C: 4AF6809D  bl 0x8221bd38
	ctx.lr = 0x832B3CA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3CA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3CAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3CB0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3CB4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3CC8: 4E800020  blr
	return;
}

pub fn sub_832B3D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3D00 size=256
	// 832B3D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3D04: 4B9F5709  bl 0x82ca940c
	ctx.lr = 0x832B3D08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 832B3D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3D0C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3D14: 3BEBFB54  addi r31, r11, -0x4ac
	ctx.r[31].s64 = ctx.r[11].s64 + -1196;
	// 832B3D18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D1C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3D20: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3D24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D28: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3D2C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D30: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3D34: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3D38: 419A002C  beq cr6, 0x832b3d64
	if ctx.cr[6].eq {
	pc = 0x832B3D64; continue 'dispatch;
	}
	// 832B3D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3D40: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3D44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832B3D48: 4B2B2139  bl 0x82565e80
	ctx.lr = 0x832B3D4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82565E80);
	// 832B3D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3D50: 4AF67FE9  bl 0x8221bd38
	ctx.lr = 0x832B3D54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3D54: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D58: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 832B3D5C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3D60: 409AFFDC  bne cr6, 0x832b3d3c
	if !ctx.cr[6].eq {
	pc = 0x832B3D3C; continue 'dispatch;
	}
	// 832B3D64: 4AF67FD5  bl 0x8221bd38
	ctx.lr = 0x832B3D68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3D6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3D74: 4B9F56E8  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
	// 832B3D78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D7C: 386BFB60  addi r3, r11, -0x4a0
	ctx.r[3].s64 = ctx.r[11].s64 + -1184;
	// 832B3D80: 4B7AA6B8  b 0x82a5e438
	crate::recompiler::externs::call(&mut ctx, base, 0x82A5E438);
	return;
}

pub fn sub_832B3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E00 size=72
	// 832B3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3E10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3E14: 3BEBFB9C  addi r31, r11, -0x464
	ctx.r[31].s64 = ctx.r[11].s64 + -1124;
	// 832B3E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3E1C: 4B7AFD95  bl 0x82a63bb0
	ctx.lr = 0x832B3E20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A63BB0);
	// 832B3E20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3E24: 4AF67F15  bl 0x8221bd38
	ctx.lr = 0x832B3E28;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B3E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3E2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3E40: 4E800020  blr
	return;
}

pub fn sub_832B3E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E48 size=80
	// 832B3E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B3E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3E5C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3E60: 3BC00031  li r30, 0x31
	ctx.r[30].s64 = 49;
	// 832B3E64: 396BFBD0  addi r11, r11, -0x430
	ctx.r[11].s64 = ctx.r[11].s64 + -1072;
	// 832B3E68: 3BEB12C0  addi r31, r11, 0x12c0
	ctx.r[31].s64 = ctx.r[11].s64 + 4800;
	// 832B3E6C: 3BFFFFA0  addi r31, r31, -0x60
	ctx.r[31].s64 = ctx.r[31].s64 + -96;
	// 832B3E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3E74: 4B7AE645  bl 0x82a624b8
	ctx.lr = 0x832B3E78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A624B8);
	// 832B3E78: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B3E7C: 4080FFF0  bge 0x832b3e6c
	if !ctx.cr[0].lt {
	pc = 0x832B3E6C; continue 'dispatch;
	}
	// 832B3E80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3E8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3E94: 4E800020  blr
	return;
}

pub fn sub_832B3E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E98 size=16
	// 832B3E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3EA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3ED8 size=16
	// 832B3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3F68 size=88
	// 832B3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3F6C: 4B9F549D  bl 0x82ca9408
	ctx.lr = 0x832B3F70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3F74: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F78: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3F7C: 396B0EA4  addi r11, r11, 0xea4
	ctx.r[11].s64 = ctx.r[11].s64 + 3748;
	// 832B3F80: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832B3F84: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 832B3F88: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3F8C: 3BAA2A40  addi r29, r10, 0x2a40
	ctx.r[29].s64 = ctx.r[10].s64 + 10816;
	// 832B3F90: 3B8B2A30  addi r28, r11, 0x2a30
	ctx.r[28].s64 = ctx.r[11].s64 + 10800;
	// 832B3F94: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3F9C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B3FA0: 4AF48069  bl 0x821fc008
	ctx.lr = 0x832B3FA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3FA4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B3FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3FAC: 4AF4805D  bl 0x821fc008
	ctx.lr = 0x832B3FB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3FB0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B3FB4: 4080FFE0  bge 0x832b3f94
	if !ctx.cr[0].lt {
	pc = 0x832B3F94; continue 'dispatch;
	}
	// 832B3FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3FBC: 4B9F549C  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_832B3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3FC0 size=296
	// 832B3FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3FC4: 4B9F5445  bl 0x82ca9408
	ctx.lr = 0x832B3FC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 832B3FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3FCC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3FD0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3FD4: 396B0F60  addi r11, r11, 0xf60
	ctx.r[11].s64 = ctx.r[11].s64 + 3936;
	// 832B3FD8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832B3FDC: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 832B3FE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3FE4: 3BAA2A40  addi r29, r10, 0x2a40
	ctx.r[29].s64 = ctx.r[10].s64 + 10816;
	// 832B3FE8: 3B8B2A30  addi r28, r11, 0x2a30
	ctx.r[28].s64 = ctx.r[11].s64 + 10800;
	// 832B3FEC: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3FF4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B3FF8: 4AF48011  bl 0x821fc008
	ctx.lr = 0x832B3FFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B3FFC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B4004: 4AF48005  bl 0x821fc008
	ctx.lr = 0x832B4008;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4008: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B400C: 4080FFE0  bge 0x832b3fec
	if !ctx.cr[0].lt {
	pc = 0x832B3FEC; continue 'dispatch;
	}
	// 832B4010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B4014: 4B9F5444  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
	// 832B4018: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B401C: 386B0F78  addi r3, r11, 0xf78
	ctx.r[3].s64 = ctx.r[11].s64 + 3960;
	// 832B4020: 4AF60DB8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B40E8 size=88
	// 832B40E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B40EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B40F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B40F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B40F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B40FC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B4100: 3BEBAB50  addi r31, r11, -0x54b0
	ctx.r[31].s64 = ctx.r[11].s64 + -21680;
	// 832B4104: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B4108: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B410C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B4110: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4114: 419A0018  beq cr6, 0x832b412c
	if ctx.cr[6].eq {
	pc = 0x832B412C; continue 'dispatch;
	}
	// 832B4118: 4AF48209  bl 0x821fc320
	ctx.lr = 0x832B411C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B411C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4124: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4128: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B412C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B413C: 4E800020  blr
	return;
}

pub fn sub_832B4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4140 size=304
	// 832B4140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B414C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4150: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4154: 3BEB0FF0  addi r31, r11, 0xff0
	ctx.r[31].s64 = ctx.r[11].s64 + 4080;
	// 832B4158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B415C: 4AEC8D1D  bl 0x8217ce78
	ctx.lr = 0x832B4160;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217CE78);
	// 832B4160: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4164: 4AF67BD5  bl 0x8221bd38
	ctx.lr = 0x832B4168;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B4168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B416C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B417C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4180: 4E800020  blr
	return;
}

pub fn sub_832B4270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4270 size=168
	// 832B4270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B427C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4284: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4288: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B428C: 396B10F0  addi r11, r11, 0x10f0
	ctx.r[11].s64 = ctx.r[11].s64 + 4336;
	// 832B4290: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B4294: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B4298: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B429C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42A0: 419A0008  beq cr6, 0x832b42a8
	if ctx.cr[6].eq {
	pc = 0x832B42A8; continue 'dispatch;
	}
	// 832B42A4: 4AF4807D  bl 0x821fc320
	ctx.lr = 0x832B42A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B42A8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B42AC: 4080FFE8  bge 0x832b4294
	if !ctx.cr[0].lt {
	pc = 0x832B4294; continue 'dispatch;
	}
	// 832B42B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B42B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B42B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B42BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B42C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B42C4: 4E800020  blr
	return;
	// 832B42C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42CC: 806B1140  lwz r3, 0x1140(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4416 as u32) ) } as u64;
	// 832B42D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B42D8: 4AF48048  b 0x821fc320
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	return;
	// 832B42DC: 4E800020  blr
	return;
	// 832B42E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42E4: 806B1154  lwz r3, 0x1154(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4436 as u32) ) } as u64;
	// 832B42E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B42F0: 4AF48030  b 0x821fc320
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	return;
	// 832B42F4: 4E800020  blr
	return;
	// 832B42F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42FC: 386B1190  addi r3, r11, 0x1190
	ctx.r[3].s64 = ctx.r[11].s64 + 4496;
	// 832B4300: 4AF60AD8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4318 size=232
	// 832B4318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B432C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4330: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B4334: 396B1198  addi r11, r11, 0x1198
	ctx.r[11].s64 = ctx.r[11].s64 + 4504;
	// 832B4338: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B433C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B4340: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4344: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4348: 419A0008  beq cr6, 0x832b4350
	if ctx.cr[6].eq {
	pc = 0x832B4350; continue 'dispatch;
	}
	// 832B434C: 4AF47FD5  bl 0x821fc320
	ctx.lr = 0x832B4350;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B4350: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B4354: 4080FFE8  bge 0x832b433c
	if !ctx.cr[0].lt {
	pc = 0x832B433C; continue 'dispatch;
	}
	// 832B4358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B435C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B436C: 4E800020  blr
	return;
	// 832B4370: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4374: 806B11E8  lwz r3, 0x11e8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4584 as u32) ) } as u64;
	// 832B4378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B437C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4380: 4AF47FA0  b 0x821fc320
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	return;
	// 832B4384: 4E800020  blr
	return;
	// 832B4388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B438C: 386B11FC  addi r3, r11, 0x11fc
	ctx.r[3].s64 = ctx.r[11].s64 + 4604;
	// 832B4390: 4AF60A48  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4400 size=160
	// 832B4400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B440C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4410: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B4414: 3BEB2300  addi r31, r11, 0x2300
	ctx.r[31].s64 = ctx.r[11].s64 + 8960;
	// 832B4418: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 832B441C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4420: 419A0008  beq cr6, 0x832b4428
	if ctx.cr[6].eq {
	pc = 0x832B4428; continue 'dispatch;
	}
	// 832B4424: 4AF67915  bl 0x8221bd38
	ctx.lr = 0x832B4428;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B4428: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 832B442C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B4438: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832B443C: 915F006C  stw r10, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 832B4440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4444: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 832B4448: 419A0008  beq cr6, 0x832b4450
	if ctx.cr[6].eq {
	pc = 0x832B4450; continue 'dispatch;
	}
	// 832B444C: 4AF678ED  bl 0x8221bd38
	ctx.lr = 0x832B4450;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B4450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4458: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B445C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 832B4460: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B4464: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 832B4468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B446C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4478: 4E800020  blr
	return;
}

pub fn sub_832B44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B44A0 size=184
	// 832B44A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B44A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B44A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B44AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B44B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B44B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B44B8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B44BC: 396B12B0  addi r11, r11, 0x12b0
	ctx.r[11].s64 = ctx.r[11].s64 + 4784;
	// 832B44C0: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B44C4: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B44C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B44CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B44D0: 419A0008  beq cr6, 0x832b44d8
	if ctx.cr[6].eq {
	pc = 0x832B44D8; continue 'dispatch;
	}
	// 832B44D4: 4AF47E4D  bl 0x821fc320
	ctx.lr = 0x832B44D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B44D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B44DC: 4080FFE8  bge 0x832b44c4
	if !ctx.cr[0].lt {
	pc = 0x832B44C4; continue 'dispatch;
	}
	// 832B44E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B44E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B44E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B44EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B44F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B44F4: 4E800020  blr
	return;
	// 832B44F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B44FC: 386B1328  addi r3, r11, 0x1328
	ctx.r[3].s64 = ctx.r[11].s64 + 4904;
	// 832B4500: 4AF608D8  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B4558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4558 size=120
	// 832B4558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4568: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B456C: 807F4340  lwz r3, 0x4340(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17216 as u32) ) } as u64;
	// 832B4570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4574: 419A0044  beq cr6, 0x832b45b8
	if ctx.cr[6].eq {
	pc = 0x832B45B8; continue 'dispatch;
	}
	// 832B4578: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B457C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4580: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4584: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4588: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B458C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4590: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4594: 4082FFE8  bne 0x832b457c
	if !ctx.cr[0].eq {
	pc = 0x832B457C; continue 'dispatch;
	}
	// 832B4598: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B459C: 409A0014  bne cr6, 0x832b45b0
	if !ctx.cr[6].eq {
	pc = 0x832B45B0; continue 'dispatch;
	}
	// 832B45A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B45A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B45A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B45AC: 4E800421  bctrl
	ctx.lr = 0x832B45B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B45B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B45B4: 917F4340  stw r11, 0x4340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17216 as u32), ctx.r[11].u32 ) };
	// 832B45B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B45BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B45C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B45C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B45C8: 4E800020  blr
	return;
}

pub fn sub_832B45D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B45D0 size=120
	// 832B45D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B45D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B45D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B45DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B45E0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B45E4: 807F4344  lwz r3, 0x4344(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17220 as u32) ) } as u64;
	// 832B45E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B45EC: 419A0044  beq cr6, 0x832b4630
	if ctx.cr[6].eq {
	pc = 0x832B4630; continue 'dispatch;
	}
	// 832B45F0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B45F4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B45F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B45FC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4600: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B4604: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4608: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B460C: 4082FFE8  bne 0x832b45f4
	if !ctx.cr[0].eq {
	pc = 0x832B45F4; continue 'dispatch;
	}
	// 832B4610: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4614: 409A0014  bne cr6, 0x832b4628
	if !ctx.cr[6].eq {
	pc = 0x832B4628; continue 'dispatch;
	}
	// 832B4618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B461C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4620: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4624: 4E800421  bctrl
	ctx.lr = 0x832B4628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B462C: 917F4344  stw r11, 0x4344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17220 as u32), ctx.r[11].u32 ) };
	// 832B4630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B463C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4640: 4E800020  blr
	return;
}

pub fn sub_832B4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4648 size=120
	// 832B4648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4658: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B465C: 807F4348  lwz r3, 0x4348(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17224 as u32) ) } as u64;
	// 832B4660: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4664: 419A0044  beq cr6, 0x832b46a8
	if ctx.cr[6].eq {
	pc = 0x832B46A8; continue 'dispatch;
	}
	// 832B4668: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B466C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4670: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4674: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4678: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B467C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4680: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4684: 4082FFE8  bne 0x832b466c
	if !ctx.cr[0].eq {
	pc = 0x832B466C; continue 'dispatch;
	}
	// 832B4688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B468C: 409A0014  bne cr6, 0x832b46a0
	if !ctx.cr[6].eq {
	pc = 0x832B46A0; continue 'dispatch;
	}
	// 832B4690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4694: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4698: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B469C: 4E800421  bctrl
	ctx.lr = 0x832B46A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B46A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B46A4: 917F4348  stw r11, 0x4348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17224 as u32), ctx.r[11].u32 ) };
	// 832B46A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B46AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B46B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B46B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B46B8: 4E800020  blr
	return;
}

pub fn sub_832B46C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B46C0 size=120
	// 832B46C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B46C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B46C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B46CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B46D0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B46D4: 807F434C  lwz r3, 0x434c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17228 as u32) ) } as u64;
	// 832B46D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B46DC: 419A0044  beq cr6, 0x832b4720
	if ctx.cr[6].eq {
	pc = 0x832B4720; continue 'dispatch;
	}
	// 832B46E0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B46E4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B46E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B46EC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B46F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B46F4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B46F8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B46FC: 4082FFE8  bne 0x832b46e4
	if !ctx.cr[0].eq {
	pc = 0x832B46E4; continue 'dispatch;
	}
	// 832B4700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4704: 409A0014  bne cr6, 0x832b4718
	if !ctx.cr[6].eq {
	pc = 0x832B4718; continue 'dispatch;
	}
	// 832B4708: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B470C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4710: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4714: 4E800421  bctrl
	ctx.lr = 0x832B4718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B471C: 917F434C  stw r11, 0x434c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17228 as u32), ctx.r[11].u32 ) };
	// 832B4720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B472C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4730: 4E800020  blr
	return;
}

pub fn sub_832B4738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4738 size=120
	// 832B4738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4748: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B474C: 807F4350  lwz r3, 0x4350(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17232 as u32) ) } as u64;
	// 832B4750: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4754: 419A0044  beq cr6, 0x832b4798
	if ctx.cr[6].eq {
	pc = 0x832B4798; continue 'dispatch;
	}
	// 832B4758: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B475C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4760: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4764: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4768: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B476C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4770: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4774: 4082FFE8  bne 0x832b475c
	if !ctx.cr[0].eq {
	pc = 0x832B475C; continue 'dispatch;
	}
	// 832B4778: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B477C: 409A0014  bne cr6, 0x832b4790
	if !ctx.cr[6].eq {
	pc = 0x832B4790; continue 'dispatch;
	}
	// 832B4780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4784: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4788: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B478C: 4E800421  bctrl
	ctx.lr = 0x832B4790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4794: 917F4350  stw r11, 0x4350(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17232 as u32), ctx.r[11].u32 ) };
	// 832B4798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B47A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B47A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B47A8: 4E800020  blr
	return;
}

pub fn sub_832B47B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B47B0 size=120
	// 832B47B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B47B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B47B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B47BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B47C0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B47C4: 807F4354  lwz r3, 0x4354(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17236 as u32) ) } as u64;
	// 832B47C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B47CC: 419A0044  beq cr6, 0x832b4810
	if ctx.cr[6].eq {
	pc = 0x832B4810; continue 'dispatch;
	}
	// 832B47D0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B47D4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B47D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B47DC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B47E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B47E4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B47E8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B47EC: 4082FFE8  bne 0x832b47d4
	if !ctx.cr[0].eq {
	pc = 0x832B47D4; continue 'dispatch;
	}
	// 832B47F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B47F4: 409A0014  bne cr6, 0x832b4808
	if !ctx.cr[6].eq {
	pc = 0x832B4808; continue 'dispatch;
	}
	// 832B47F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B47FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4800: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4804: 4E800421  bctrl
	ctx.lr = 0x832B4808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B480C: 917F4354  stw r11, 0x4354(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17236 as u32), ctx.r[11].u32 ) };
	// 832B4810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B481C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4820: 4E800020  blr
	return;
}

pub fn sub_832B4828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4828 size=120
	// 832B4828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4838: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B483C: 807F4358  lwz r3, 0x4358(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17240 as u32) ) } as u64;
	// 832B4840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4844: 419A0044  beq cr6, 0x832b4888
	if ctx.cr[6].eq {
	pc = 0x832B4888; continue 'dispatch;
	}
	// 832B4848: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B484C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4850: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4854: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4858: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B485C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4860: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4864: 4082FFE8  bne 0x832b484c
	if !ctx.cr[0].eq {
	pc = 0x832B484C; continue 'dispatch;
	}
	// 832B4868: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B486C: 409A0014  bne cr6, 0x832b4880
	if !ctx.cr[6].eq {
	pc = 0x832B4880; continue 'dispatch;
	}
	// 832B4870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4874: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4878: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B487C: 4E800421  bctrl
	ctx.lr = 0x832B4880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4884: 917F4358  stw r11, 0x4358(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17240 as u32), ctx.r[11].u32 ) };
	// 832B4888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B488C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4898: 4E800020  blr
	return;
}

pub fn sub_832B48A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B48A0 size=120
	// 832B48A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B48A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B48A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B48AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B48B0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B48B4: 807F435C  lwz r3, 0x435c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17244 as u32) ) } as u64;
	// 832B48B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B48BC: 419A0044  beq cr6, 0x832b4900
	if ctx.cr[6].eq {
	pc = 0x832B4900; continue 'dispatch;
	}
	// 832B48C0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B48C4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B48C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B48CC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B48D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B48D4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B48D8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B48DC: 4082FFE8  bne 0x832b48c4
	if !ctx.cr[0].eq {
	pc = 0x832B48C4; continue 'dispatch;
	}
	// 832B48E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B48E4: 409A0014  bne cr6, 0x832b48f8
	if !ctx.cr[6].eq {
	pc = 0x832B48F8; continue 'dispatch;
	}
	// 832B48E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B48EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B48F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B48F4: 4E800421  bctrl
	ctx.lr = 0x832B48F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B48F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B48FC: 917F435C  stw r11, 0x435c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17244 as u32), ctx.r[11].u32 ) };
	// 832B4900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B490C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4910: 4E800020  blr
	return;
}

pub fn sub_832B4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4918 size=136
	// 832B4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4928: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B492C: 807F4360  lwz r3, 0x4360(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17248 as u32) ) } as u64;
	// 832B4930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4934: 419A0044  beq cr6, 0x832b4978
	if ctx.cr[6].eq {
	pc = 0x832B4978; continue 'dispatch;
	}
	// 832B4938: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B493C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4940: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4944: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4948: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B494C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4950: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4954: 4082FFE8  bne 0x832b493c
	if !ctx.cr[0].eq {
	pc = 0x832B493C; continue 'dispatch;
	}
	// 832B4958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B495C: 409A0014  bne cr6, 0x832b4970
	if !ctx.cr[6].eq {
	pc = 0x832B4970; continue 'dispatch;
	}
	// 832B4960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4964: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4968: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B496C: 4E800421  bctrl
	ctx.lr = 0x832B4970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4974: 917F4360  stw r11, 0x4360(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17248 as u32), ctx.r[11].u32 ) };
	// 832B4978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4988: 4E800020  blr
	return;
}

pub fn sub_832B49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B49A0 size=168
	// 832B49A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B49A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B49A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B49AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B49B0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B49B4: 807F4364  lwz r3, 0x4364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17252 as u32) ) } as u64;
	// 832B49B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B49BC: 419A0044  beq cr6, 0x832b4a00
	if ctx.cr[6].eq {
	pc = 0x832B4A00; continue 'dispatch;
	}
	// 832B49C0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B49C4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B49C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B49CC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B49D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B49D4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B49D8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B49DC: 4082FFE8  bne 0x832b49c4
	if !ctx.cr[0].eq {
	pc = 0x832B49C4; continue 'dispatch;
	}
	// 832B49E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B49E4: 409A0014  bne cr6, 0x832b49f8
	if !ctx.cr[6].eq {
	pc = 0x832B49F8; continue 'dispatch;
	}
	// 832B49E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B49EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B49F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B49F4: 4E800421  bctrl
	ctx.lr = 0x832B49F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B49F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B49FC: 917F4364  stw r11, 0x4364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17252 as u32), ctx.r[11].u32 ) };
	// 832B4A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4A10: 4E800020  blr
	return;
}

pub fn sub_832B4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4A48 size=96
	// 832B4A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4A5C: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4A60: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4A64: 3BDFABB8  addi r30, r31, -0x5448
	ctx.r[30].s64 = ctx.r[31].s64 + -21576;
	// 832B4A68: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4A6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4A70: 917FABB8  stw r11, -0x5448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21576 as u32), ctx.r[11].u32 ) };
	// 832B4A74: 4AF47595  bl 0x821fc008
	ctx.lr = 0x832B4A78;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4A78: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4A7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4A80: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4A84: 917FABB8  stw r11, -0x5448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21576 as u32), ctx.r[11].u32 ) };
	// 832B4A88: 4AF47581  bl 0x821fc008
	ctx.lr = 0x832B4A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4AA0: 4E800020  blr
	return;
}

pub fn sub_832B4AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4AA8 size=456
	// 832B4AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4ABC: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4AC0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4AC4: 3BDFABC0  addi r30, r31, -0x5440
	ctx.r[30].s64 = ctx.r[31].s64 + -21568;
	// 832B4AC8: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4AD0: 917FABC0  stw r11, -0x5440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21568 as u32), ctx.r[11].u32 ) };
	// 832B4AD4: 4AF47535  bl 0x821fc008
	ctx.lr = 0x832B4AD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4AD8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4ADC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4AE0: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4AE4: 917FABC0  stw r11, -0x5440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21568 as u32), ctx.r[11].u32 ) };
	// 832B4AE8: 4AF47521  bl 0x821fc008
	ctx.lr = 0x832B4AEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4AF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4AFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4B00: 4E800020  blr
	return;
}

pub fn sub_832B4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4C70 size=192
	// 832B4C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4C74: 4B9F4785  bl 0x82ca93f8
	ctx.lr = 0x832B4C78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F8);
	// 832B4C78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4C7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C80: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832B4C84: 396B4440  addi r11, r11, 0x4440
	ctx.r[11].s64 = ctx.r[11].s64 + 17472;
	// 832B4C88: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832B4C8C: 3BCB08A0  addi r30, r11, 0x8a0
	ctx.r[30].s64 = ctx.r[11].s64 + 2208;
	// 832B4C90: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B4C94: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832B4C98: 3B800016  li r28, 0x16
	ctx.r[28].s64 = 22;
	// 832B4C9C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 832B4CA0: 3B682A40  addi r27, r8, 0x2a40
	ctx.r[27].s64 = ctx.r[8].s64 + 10816;
	// 832B4CA4: 3B492A30  addi r26, r9, 0x2a30
	ctx.r[26].s64 = ctx.r[9].s64 + 10800;
	// 832B4CA8: 3B2A2A2C  addi r25, r10, 0x2a2c
	ctx.r[25].s64 = ctx.r[10].s64 + 10796;
	// 832B4CAC: 3BABB00C  addi r29, r11, -0x4ff4
	ctx.r[29].s64 = ctx.r[11].s64 + -20468;
	// 832B4CB0: 3BDEFFA0  addi r30, r30, -0x60
	ctx.r[30].s64 = ctx.r[30].s64 + -96;
	// 832B4CB4: 37FE0010  addic. r31, r30, 0x10
	ctx.xer.ca = (ctx.r[30].u32 > (!(16 as u32)));
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832B4CB8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 832B4CBC: 40820008  bne 0x832b4cc4
	if !ctx.cr[0].eq {
	pc = 0x832B4CC4; continue 'dispatch;
	}
	// 832B4CC0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 832B4CC4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4CC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4CD0: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 832B4CD4: 419A0010  beq cr6, 0x832b4ce4
	if ctx.cr[6].eq {
	pc = 0x832B4CE4; continue 'dispatch;
	}
	// 832B4CD8: 4AF47649  bl 0x821fc320
	ctx.lr = 0x832B4CDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B4CDC: 931F0004  stw r24, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 832B4CE0: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 832B4CE4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832B4CE8: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 832B4CEC: 409A0008  bne cr6, 0x832b4cf4
	if !ctx.cr[6].eq {
	pc = 0x832B4CF4; continue 'dispatch;
	}
	// 832B4CF0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 832B4CF4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4CFC: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 832B4D00: 4AF47309  bl 0x821fc008
	ctx.lr = 0x832B4D04;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4D04: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 832B4D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4D0C: 4AF472FD  bl 0x821fc008
	ctx.lr = 0x832B4D10;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4D10: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832B4D14: 4080FF9C  bge 0x832b4cb0
	if !ctx.cr[0].lt {
	pc = 0x832B4CB0; continue 'dispatch;
	}
	// 832B4D18: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 832B4D1C: 4B9F472C  b 0x82ca9448
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9448);
	return;
	// 832B4D20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4D24: 386B43F4  addi r3, r11, 0x43f4
	ctx.r[3].s64 = ctx.r[11].s64 + 17396;
	// 832B4D28: 4AFE8D70  b 0x8229da98
	crate::recompiler::externs::call(&mut ctx, base, 0x8229DA98);
	return;
}

pub fn sub_832B4D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4D30 size=168
	// 832B4D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4D40: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B4D44: 807F4CE0  lwz r3, 0x4ce0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19680 as u32) ) } as u64;
	// 832B4D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4D4C: 419A0044  beq cr6, 0x832b4d90
	if ctx.cr[6].eq {
	pc = 0x832B4D90; continue 'dispatch;
	}
	// 832B4D50: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B4D54: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4D58: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4D5C: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4D60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B4D64: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4D68: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4D6C: 4082FFE8  bne 0x832b4d54
	if !ctx.cr[0].eq {
	pc = 0x832B4D54; continue 'dispatch;
	}
	// 832B4D70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4D74: 409A0014  bne cr6, 0x832b4d88
	if !ctx.cr[6].eq {
	pc = 0x832B4D88; continue 'dispatch;
	}
	// 832B4D78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4D7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4D80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4D84: 4E800421  bctrl
	ctx.lr = 0x832B4D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B4D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4D8C: 917F4CE0  stw r11, 0x4ce0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19680 as u32), ctx.r[11].u32 ) };
	// 832B4D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4D9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4DA0: 4E800020  blr
	return;
}

pub fn sub_832B4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4DD8 size=96
	// 832B4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4DEC: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4DF0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4DF4: 3BDFAC08  addi r30, r31, -0x53f8
	ctx.r[30].s64 = ctx.r[31].s64 + -21496;
	// 832B4DF8: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4DFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E00: 917FAC08  stw r11, -0x53f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21496 as u32), ctx.r[11].u32 ) };
	// 832B4E04: 4AF47205  bl 0x821fc008
	ctx.lr = 0x832B4E08;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4E08: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E10: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4E14: 917FAC08  stw r11, -0x53f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21496 as u32), ctx.r[11].u32 ) };
	// 832B4E18: 4AF471F1  bl 0x821fc008
	ctx.lr = 0x832B4E1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4E1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4E28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4E30: 4E800020  blr
	return;
}

pub fn sub_832B4E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4E38 size=224
	// 832B4E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4E4C: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4E50: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E54: 3BDFAC10  addi r30, r31, -0x53f0
	ctx.r[30].s64 = ctx.r[31].s64 + -21488;
	// 832B4E58: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E60: 917FAC10  stw r11, -0x53f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21488 as u32), ctx.r[11].u32 ) };
	// 832B4E64: 4AF471A5  bl 0x821fc008
	ctx.lr = 0x832B4E68;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4E68: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E70: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4E74: 917FAC10  stw r11, -0x53f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21488 as u32), ctx.r[11].u32 ) };
	// 832B4E78: 4AF47191  bl 0x821fc008
	ctx.lr = 0x832B4E7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B4E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4E88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4E90: 4E800020  blr
	return;
}

pub fn sub_832B4F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4F18 size=184
	// 832B4F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4F28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F2C: 3BEB4DE4  addi r31, r11, 0x4de4
	ctx.r[31].s64 = ctx.r[11].s64 + 19940;
	// 832B4F30: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4F34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4F38: 419A0008  beq cr6, 0x832b4f40
	if ctx.cr[6].eq {
	pc = 0x832B4F40; continue 'dispatch;
	}
	// 832B4F3C: 4AF66DFD  bl 0x8221bd38
	ctx.lr = 0x832B4F40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B4F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4F44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4F48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B4F4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4F50: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B4F54: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B4F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4F68: 4E800020  blr
	return;
}

pub fn sub_832B4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4FD0 size=80
	// 832B4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4FE4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FE8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B4FEC: 396B4E0C  addi r11, r11, 0x4e0c
	ctx.r[11].s64 = ctx.r[11].s64 + 19980;
	// 832B4FF0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B4FF4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B4FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B4FFC: 4AF02B1D  bl 0x821b7b18
	ctx.lr = 0x832B5000;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B5000: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5004: 4080FFF0  bge 0x832b4ff4
	if !ctx.cr[0].lt {
	pc = 0x832B4FF4; continue 'dispatch;
	}
	// 832B5008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B500C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5014: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B501C: 4E800020  blr
	return;
}

pub fn sub_832B5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5020 size=80
	// 832B5020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B502C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5034: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5038: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B503C: 396B4E1C  addi r11, r11, 0x4e1c
	ctx.r[11].s64 = ctx.r[11].s64 + 19996;
	// 832B5040: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5044: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B504C: 4AF02ACD  bl 0x821b7b18
	ctx.lr = 0x832B5050;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B5050: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5054: 4080FFF0  bge 0x832b5044
	if !ctx.cr[0].lt {
	pc = 0x832B5044; continue 'dispatch;
	}
	// 832B5058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B505C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B506C: 4E800020  blr
	return;
}

pub fn sub_832B5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5070 size=80
	// 832B5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B507C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5084: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5088: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B508C: 396B4E2C  addi r11, r11, 0x4e2c
	ctx.r[11].s64 = ctx.r[11].s64 + 20012;
	// 832B5090: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5094: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B509C: 4AF02A7D  bl 0x821b7b18
	ctx.lr = 0x832B50A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B50A0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50A4: 4080FFF0  bge 0x832b5094
	if !ctx.cr[0].lt {
	pc = 0x832B5094; continue 'dispatch;
	}
	// 832B50A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B50B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B50B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B50B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B50BC: 4E800020  blr
	return;
}

pub fn sub_832B50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B50C0 size=80
	// 832B50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B50C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B50C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B50CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B50D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B50D4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B50D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B50DC: 396B4E3C  addi r11, r11, 0x4e3c
	ctx.r[11].s64 = ctx.r[11].s64 + 20028;
	// 832B50E0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B50E4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B50E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B50EC: 4AF02A2D  bl 0x821b7b18
	ctx.lr = 0x832B50F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B50F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50F4: 4080FFF0  bge 0x832b50e4
	if !ctx.cr[0].lt {
	pc = 0x832B50E4; continue 'dispatch;
	}
	// 832B50F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B510C: 4E800020  blr
	return;
}

pub fn sub_832B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5110 size=80
	// 832B5110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B511C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5124: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5128: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B512C: 396B4E4C  addi r11, r11, 0x4e4c
	ctx.r[11].s64 = ctx.r[11].s64 + 20044;
	// 832B5130: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5134: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B513C: 4AF029DD  bl 0x821b7b18
	ctx.lr = 0x832B5140;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B5140: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5144: 4080FFF0  bge 0x832b5134
	if !ctx.cr[0].lt {
	pc = 0x832B5134; continue 'dispatch;
	}
	// 832B5148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B514C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B515C: 4E800020  blr
	return;
}

pub fn sub_832B5160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5160 size=80
	// 832B5160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B516C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5174: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5178: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B517C: 396B4E5C  addi r11, r11, 0x4e5c
	ctx.r[11].s64 = ctx.r[11].s64 + 20060;
	// 832B5180: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832B5184: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B518C: 4AF0298D  bl 0x821b7b18
	ctx.lr = 0x832B5190;
	crate::recompiler::externs::call(&mut ctx, base, 0x821B7B18);
	// 832B5190: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5194: 4080FFF0  bge 0x832b5184
	if !ctx.cr[0].lt {
	pc = 0x832B5184; continue 'dispatch;
	}
	// 832B5198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B519C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B51A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B51A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B51AC: 4E800020  blr
	return;
}

pub fn sub_832B51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B51B0 size=208
	// 832B51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B51B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B51BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B51C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B51C4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B51C8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51CC: 3BDFAC18  addi r30, r31, -0x53e8
	ctx.r[30].s64 = ctx.r[31].s64 + -21480;
	// 832B51D0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B51D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51D8: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51DC: 4AF46E2D  bl 0x821fc008
	ctx.lr = 0x832B51E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B51E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51E8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B51EC: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51F0: 4AF46E19  bl 0x821fc008
	ctx.lr = 0x832B51F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC008);
	// 832B51F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5208: 4E800020  blr
	return;
}

pub fn sub_832B5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5280 size=120
	// 832B5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5290: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5294: 3BEB4E94  addi r31, r11, 0x4e94
	ctx.r[31].s64 = ctx.r[11].s64 + 20116;
	// 832B5298: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B529C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B52A0: 419A0008  beq cr6, 0x832b52a8
	if ctx.cr[6].eq {
	pc = 0x832B52A8; continue 'dispatch;
	}
	// 832B52A4: 4AF66A95  bl 0x8221bd38
	ctx.lr = 0x832B52A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B52A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B52AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B52B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B52B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B52B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B52BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B52C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B52C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B52C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B52CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B52D0: 4E800020  blr
	return;
}

pub fn sub_832B52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B52F8 size=168
	// 832B52F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B52FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B530C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5310: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B5314: 396B4EC0  addi r11, r11, 0x4ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 20160;
	// 832B5318: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B531C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B5320: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5328: 419A0008  beq cr6, 0x832b5330
	if ctx.cr[6].eq {
	pc = 0x832B5330; continue 'dispatch;
	}
	// 832B532C: 4AF46FF5  bl 0x821fc320
	ctx.lr = 0x832B5330;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B5330: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5334: 4080FFE8  bge 0x832b531c
	if !ctx.cr[0].lt {
	pc = 0x832B531C; continue 'dispatch;
	}
	// 832B5338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B534C: 4E800020  blr
	return;
	// 832B5350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5354: 806B4EAC  lwz r3, 0x4eac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20140 as u32) ) } as u64;
	// 832B5358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B535C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B5360: 4AF46FC0  b 0x821fc320
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	return;
	// 832B5364: 4E800020  blr
	return;
	// 832B5368: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B536C: 806B4F10  lwz r3, 0x4f10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20240 as u32) ) } as u64;
	// 832B5370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5374: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B5378: 4AF46FA8  b 0x821fc320
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	return;
	// 832B537C: 4E800020  blr
	return;
	// 832B5380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5384: 386B4F24  addi r3, r11, 0x4f24
	ctx.r[3].s64 = ctx.r[11].s64 + 20260;
	// 832B5388: 4AF5FA50  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53A0 size=88
	// 832B53A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B53A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B53AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B53B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B53B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B53B8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B53BC: 396B4F30  addi r11, r11, 0x4f30
	ctx.r[11].s64 = ctx.r[11].s64 + 20272;
	// 832B53C0: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 832B53C4: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B53C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B53CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B53D0: 419A0008  beq cr6, 0x832b53d8
	if ctx.cr[6].eq {
	pc = 0x832B53D8; continue 'dispatch;
	}
	// 832B53D4: 4AF46F4D  bl 0x821fc320
	ctx.lr = 0x832B53D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821FC320);
	// 832B53D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B53DC: 4080FFE8  bge 0x832b53c4
	if !ctx.cr[0].lt {
	pc = 0x832B53C4; continue 'dispatch;
	}
	// 832B53E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B53E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B53E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B53EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B53F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B53F4: 4E800020  blr
	return;
}

pub fn sub_832B53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53F8 size=272
	// 832B53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5408: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B540C: 807F4F80  lwz r3, 0x4f80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20352 as u32) ) } as u64;
	// 832B5410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5414: 419A0044  beq cr6, 0x832b5458
	if ctx.cr[6].eq {
	pc = 0x832B5458; continue 'dispatch;
	}
	// 832B5418: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	// 832B541C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B5420: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5424: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B5428: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B542C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B5430: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5434: 4082FFE8  bne 0x832b541c
	if !ctx.cr[0].eq {
	pc = 0x832B541C; continue 'dispatch;
	}
	// 832B5438: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B543C: 409A0014  bne cr6, 0x832b5450
	if !ctx.cr[6].eq {
	pc = 0x832B5450; continue 'dispatch;
	}
	// 832B5440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5444: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5448: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B544C: 4E800421  bctrl
	ctx.lr = 0x832B5450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B5450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5454: 917F4F80  stw r11, 0x4f80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20352 as u32), ctx.r[11].u32 ) };
	// 832B5458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B545C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5468: 4E800020  blr
	return;
}

pub fn sub_832B5508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5508 size=120
	// 832B5508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5518: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B551C: 3BEB1A50  addi r31, r11, 0x1a50
	ctx.r[31].s64 = ctx.r[11].s64 + 6736;
	// 832B5520: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5528: 419A0008  beq cr6, 0x832b5530
	if ctx.cr[6].eq {
	pc = 0x832B5530; continue 'dispatch;
	}
	// 832B552C: 4AF6680D  bl 0x8221bd38
	ctx.lr = 0x832B5530;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5538: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B553C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5540: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5544: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B554C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5558: 4E800020  blr
	return;
}

pub fn sub_832B5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5580 size=88
	// 832B5580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B558C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5590: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5594: 3BEB4FF8  addi r31, r11, 0x4ff8
	ctx.r[31].s64 = ctx.r[11].s64 + 20472;
	// 832B5598: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B559C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55A0: 419A0008  beq cr6, 0x832b55a8
	if ctx.cr[6].eq {
	pc = 0x832B55A8; continue 'dispatch;
	}
	// 832B55A4: 4AF66795  bl 0x8221bd38
	ctx.lr = 0x832B55A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B55A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B55AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B55B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B55B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B55B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B55BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B55C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B55C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B55C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B55CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B55D0: 4E800020  blr
	return;
}

pub fn sub_832B55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B55D8 size=88
	// 832B55D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B55DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B55E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B55E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B55E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B55EC: 3BEB5008  addi r31, r11, 0x5008
	ctx.r[31].s64 = ctx.r[11].s64 + 20488;
	// 832B55F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B55F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55F8: 419A0008  beq cr6, 0x832b5600
	if ctx.cr[6].eq {
	pc = 0x832B5600; continue 'dispatch;
	}
	// 832B55FC: 4AF6673D  bl 0x8221bd38
	ctx.lr = 0x832B5600;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5608: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B560C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5610: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5614: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B561C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5628: 4E800020  blr
	return;
}

pub fn sub_832B5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5630 size=120
	// 832B5630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B563C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5640: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5644: 3BEB5018  addi r31, r11, 0x5018
	ctx.r[31].s64 = ctx.r[11].s64 + 20504;
	// 832B5648: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B564C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5650: 419A0008  beq cr6, 0x832b5658
	if ctx.cr[6].eq {
	pc = 0x832B5658; continue 'dispatch;
	}
	// 832B5654: 4AF666E5  bl 0x8221bd38
	ctx.lr = 0x832B5658;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B565C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5660: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5664: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5668: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B566C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B567C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5680: 4E800020  blr
	return;
}

pub fn sub_832B56A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56A8 size=16
	// 832B56A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56E8 size=72
	// 832B56E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B56F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B56FC: 3BEB5034  addi r31, r11, 0x5034
	ctx.r[31].s64 = ctx.r[11].s64 + 20532;
	// 832B5700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5704: 4B75F68D  bl 0x82a14d90
	ctx.lr = 0x832B5708;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A14D90);
	// 832B5708: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B570C: 4AF6662D  bl 0x8221bd38
	ctx.lr = 0x832B5710;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5714: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5728: 4E800020  blr
	return;
}

pub fn sub_832B5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5730 size=72
	// 832B5730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B573C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5740: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5744: 3BEB5040  addi r31, r11, 0x5040
	ctx.r[31].s64 = ctx.r[11].s64 + 20544;
	// 832B5748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B574C: 4B75F645  bl 0x82a14d90
	ctx.lr = 0x832B5750;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A14D90);
	// 832B5750: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5754: 4AF665E5  bl 0x8221bd38
	ctx.lr = 0x832B5758;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B575C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B576C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5770: 4E800020  blr
	return;
}

pub fn sub_832B5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5778 size=136
	// 832B5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B578C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5790: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5794: 3BEB504C  addi r31, r11, 0x504c
	ctx.r[31].s64 = ctx.r[11].s64 + 20556;
	// 832B5798: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B579C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57A0: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B57A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57A8: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B57AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57B0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57B4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B57B8: 419A0020  beq cr6, 0x832b57d8
	if ctx.cr[6].eq {
	pc = 0x832B57D8; continue 'dispatch;
	}
	// 832B57BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B57C0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57C4: 4AF66575  bl 0x8221bd38
	ctx.lr = 0x832B57C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B57C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B57D0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57D4: 409AFFE8  bne cr6, 0x832b57bc
	if !ctx.cr[6].eq {
	pc = 0x832B57BC; continue 'dispatch;
	}
	// 832B57D8: 4AF66561  bl 0x8221bd38
	ctx.lr = 0x832B57DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B57DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B57E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B57E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B57E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B57EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B57F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B57F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B57F8: 4E800020  blr
	return;
}

pub fn sub_832B5800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5800 size=104
	// 832B5800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B580C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5814: 3BEB5058  addi r31, r11, 0x5058
	ctx.r[31].s64 = ctx.r[11].s64 + 20568;
	// 832B5818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B581C: 4B7E18C5  bl 0x82a970e0
	ctx.lr = 0x832B5820;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A970E0);
	// 832B5820: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5824: 4AF66515  bl 0x8221bd38
	ctx.lr = 0x832B5828;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B582C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B583C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5840: 4E800020  blr
	return;
}

pub fn sub_832B5868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5868 size=16
	// 832B5868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B586C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B58A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58A8 size=16
	// 832B58A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58E8 size=16
	// 832B58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5928 size=16
	// 832B5928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B592C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5968 size=16
	// 832B5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832B59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B59C0 size=384
	// 832B59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B59C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B59CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B59D0: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B59D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59D8: 387FAD18  addi r3, r31, -0x52e8
	ctx.r[3].s64 = ctx.r[31].s64 + -21224;
	// 832B59DC: 396B7EFC  addi r11, r11, 0x7efc
	ctx.r[11].s64 = ctx.r[11].s64 + 32508;
	// 832B59E0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59E4: 4B935955  bl 0x82beb338
	ctx.lr = 0x832B59E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82BEB338);
	// 832B59E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59EC: 396B2B90  addi r11, r11, 0x2b90
	ctx.r[11].s64 = ctx.r[11].s64 + 11152;
	// 832B59F0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B59F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B59FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5A04: 4E800020  blr
	return;
	// 832B5A08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A0C: 386B5158  addi r3, r11, 0x5158
	ctx.r[3].s64 = ctx.r[11].s64 + 20824;
	// 832B5A10: 4B7E38C8  b 0x82a992d8
	crate::recompiler::externs::call(&mut ctx, base, 0x82A992D8);
	return;
}

pub fn sub_832B5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B40 size=72
	// 832B5B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B54: 3BEB56C0  addi r31, r11, 0x56c0
	ctx.r[31].s64 = ctx.r[11].s64 + 22208;
	// 832B5B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5B5C: 4AEC67DD  bl 0x8217c338
	ctx.lr = 0x832B5B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C338);
	// 832B5B60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5B64: 4AF661D5  bl 0x8221bd38
	ctx.lr = 0x832B5B68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5B6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5B80: 4E800020  blr
	return;
}

pub fn sub_832B5B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B88 size=72
	// 832B5B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B9C: 3BEB56CC  addi r31, r11, 0x56cc
	ctx.r[31].s64 = ctx.r[11].s64 + 22220;
	// 832B5BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BA4: 4AEC69ED  bl 0x8217c590
	ctx.lr = 0x832B5BA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C590);
	// 832B5BA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BAC: 4AF6618D  bl 0x8221bd38
	ctx.lr = 0x832B5BB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5BC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5BC8: 4E800020  blr
	return;
}

pub fn sub_832B5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5BD0 size=72
	// 832B5BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5BE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5BE4: 3BEB56D8  addi r31, r11, 0x56d8
	ctx.r[31].s64 = ctx.r[11].s64 + 22232;
	// 832B5BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BEC: 4AEC683D  bl 0x8217c428
	ctx.lr = 0x832B5BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C428);
	// 832B5BF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BF4: 4AF66145  bl 0x8221bd38
	ctx.lr = 0x832B5BF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C10: 4E800020  blr
	return;
}

pub fn sub_832B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C18 size=72
	// 832B5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C2C: 3BEB56E4  addi r31, r11, 0x56e4
	ctx.r[31].s64 = ctx.r[11].s64 + 22244;
	// 832B5C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C34: 4AEC68E5  bl 0x8217c518
	ctx.lr = 0x832B5C38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C518);
	// 832B5C38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C3C: 4AF660FD  bl 0x8221bd38
	ctx.lr = 0x832B5C40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C58: 4E800020  blr
	return;
}

pub fn sub_832B5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C60 size=72
	// 832B5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C74: 3BEB56F0  addi r31, r11, 0x56f0
	ctx.r[31].s64 = ctx.r[11].s64 + 22256;
	// 832B5C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C7C: 4AEC6735  bl 0x8217c3b0
	ctx.lr = 0x832B5C80;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C3B0);
	// 832B5C80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C84: 4AF660B5  bl 0x8221bd38
	ctx.lr = 0x832B5C88;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C8C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CA0: 4E800020  blr
	return;
}

pub fn sub_832B5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CA8 size=72
	// 832B5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5CB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5CBC: 3BEB56FC  addi r31, r11, 0x56fc
	ctx.r[31].s64 = ctx.r[11].s64 + 22268;
	// 832B5CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5CC4: 4AEC67DD  bl 0x8217c4a0
	ctx.lr = 0x832B5CC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8217C4A0);
	// 832B5CC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5CCC: 4AF6606D  bl 0x8221bd38
	ctx.lr = 0x832B5CD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5CD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CE8: 4E800020  blr
	return;
}

pub fn sub_832B5CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CF0 size=240
	// 832B5CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5D04: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B5D08: 83FE5708  lwz r31, 0x5708(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(22280 as u32) ) } as u64;
	// 832B5D0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B5D10: 419A0014  beq cr6, 0x832b5d24
	if ctx.cr[6].eq {
	pc = 0x832B5D24; continue 'dispatch;
	}
	// 832B5D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D18: 4B8C2689  bl 0x82b783a0
	ctx.lr = 0x832B5D1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82B783A0);
	// 832B5D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D20: 4AF66019  bl 0x8221bd38
	ctx.lr = 0x832B5D24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5D28: 917E5708  stw r11, 0x5708(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(22280 as u32), ctx.r[11].u32 ) };
	// 832B5D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5D40: 4E800020  blr
	return;
}

pub fn sub_832B5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5DE0 size=144
	// 832B5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5DF0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B5DF4: 807F5760  lwz r3, 0x5760(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22368 as u32) ) } as u64;
	// 832B5DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5DFC: 419A0018  beq cr6, 0x832b5e14
	if ctx.cr[6].eq {
	pc = 0x832B5E14; continue 'dispatch;
	}
	// 832B5E00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B5E08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B5E10: 4E800421  bctrl
	ctx.lr = 0x832B5E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832B5E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5E18: 917F5760  stw r11, 0x5760(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22368 as u32), ctx.r[11].u32 ) };
	// 832B5E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5E2C: 4E800020  blr
	return;
	// 832B5E30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E34: 386B5764  addi r3, r11, 0x5764
	ctx.r[3].s64 = ctx.r[11].s64 + 22372;
	// 832B5E38: 4AF5EFA0  b 0x82214dd8
	crate::recompiler::externs::call(&mut ctx, base, 0x82214DD8);
	return;
}

pub fn sub_832B5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5E70 size=88
	// 832B5E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5E78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5E80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E84: 3BEB5774  addi r31, r11, 0x5774
	ctx.r[31].s64 = ctx.r[11].s64 + 22388;
	// 832B5E88: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5E8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5E90: 419A0008  beq cr6, 0x832b5e98
	if ctx.cr[6].eq {
	pc = 0x832B5E98; continue 'dispatch;
	}
	// 832B5E94: 4AF65EA5  bl 0x8221bd38
	ctx.lr = 0x832B5E98;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5EA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5EA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5EA8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5EAC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5EC0: 4E800020  blr
	return;
}

pub fn sub_832B5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5EC8 size=88
	// 832B5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5ED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5ED8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5EDC: 3BEB5784  addi r31, r11, 0x5784
	ctx.r[31].s64 = ctx.r[11].s64 + 22404;
	// 832B5EE0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5EE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5EE8: 419A0008  beq cr6, 0x832b5ef0
	if ctx.cr[6].eq {
	pc = 0x832B5EF0; continue 'dispatch;
	}
	// 832B5EEC: 4AF65E4D  bl 0x8221bd38
	ctx.lr = 0x832B5EF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5EF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5EFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5F00: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5F04: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5F18: 4E800020  blr
	return;
}

pub fn sub_832B5F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5F20 size=136
	// 832B5F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5F30: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5F34: 3BEB2240  addi r31, r11, 0x2240
	ctx.r[31].s64 = ctx.r[11].s64 + 8768;
	// 832B5F38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5F3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5F40: 419A0008  beq cr6, 0x832b5f48
	if ctx.cr[6].eq {
	pc = 0x832B5F48; continue 'dispatch;
	}
	// 832B5F44: 4AF65DF5  bl 0x8221bd38
	ctx.lr = 0x832B5F48;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5F50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5F54: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5F58: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5F5C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5F70: 4E800020  blr
	return;
}

pub fn sub_832B5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5FA8 size=152
	// 832B5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5FB8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5FBC: 3BEB19C4  addi r31, r11, 0x19c4
	ctx.r[31].s64 = ctx.r[11].s64 + 6596;
	// 832B5FC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5FC8: 419A0008  beq cr6, 0x832b5fd0
	if ctx.cr[6].eq {
	pc = 0x832B5FD0; continue 'dispatch;
	}
	// 832B5FCC: 4AF65D6D  bl 0x8221bd38
	ctx.lr = 0x832B5FD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B5FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5FD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5FDC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5FE0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5FE4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5FF8: 4E800020  blr
	return;
}

pub fn sub_832B6040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6040 size=1120
	// 832B6040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B604C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6054: 3BEB57AC  addi r31, r11, 0x57ac
	ctx.r[31].s64 = ctx.r[11].s64 + 22444;
	// 832B6058: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B605C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6060: 419A0008  beq cr6, 0x832b6068
	if ctx.cr[6].eq {
	pc = 0x832B6068; continue 'dispatch;
	}
	// 832B6064: 4AF65CD5  bl 0x8221bd38
	ctx.lr = 0x832B6068;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B6068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B606C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6070: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6074: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6078: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B607C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B608C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6090: 4E800020  blr
	return;
}

pub fn sub_832B64A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B64A0 size=536
	// 832B64A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B64A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B64A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B64AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B64B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B64B4: 3BEB5D4C  addi r31, r11, 0x5d4c
	ctx.r[31].s64 = ctx.r[11].s64 + 23884;
	// 832B64B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B64BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B64C0: 419A0008  beq cr6, 0x832b64c8
	if ctx.cr[6].eq {
	pc = 0x832B64C8; continue 'dispatch;
	}
	// 832B64C4: 4AF65875  bl 0x8221bd38
	ctx.lr = 0x832B64C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221BD38);
	// 832B64C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B64CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B64D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B64D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B64D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B64DC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B64E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B64E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B64E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B64EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B64F0: 4E800020  blr
	return;
}

