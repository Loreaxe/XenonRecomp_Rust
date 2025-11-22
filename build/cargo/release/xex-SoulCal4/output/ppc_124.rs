pub fn sub_826D11F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D11F0 size=112
    let mut pc: u32 = 0x826D11F0;
    'dispatch: loop {
        match pc {
            0x826D11F0 => {
    //   block [0x826D11F0..0x826D1260)
	// 826D11F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D11F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D11F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D11FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1204: 38AA911C  addi r5, r10, -0x6ee4
	ctx.r[5].s64 = ctx.r[10].s64 + -28388;
	// 826D1208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D120C: 390BDD28  addi r8, r11, -0x22d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8920;
	// 826D1210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1214: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826D1218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D121C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1228: 386AA73C  addi r3, r10, -0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22724;
	// 826D122C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D123C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D124C: 4BD95BD5  bl 0x82466e20
	ctx.lr = 0x826D1250;
	sub_82466E20(ctx, base);
	// 826D1250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1260 size=108
    let mut pc: u32 = 0x826D1260;
    'dispatch: loop {
        match pc {
            0x826D1260 => {
    //   block [0x826D1260..0x826D12CC)
	// 826D1260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D126C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1274: 38EBDD40  addi r7, r11, -0x22c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8896;
	// 826D1278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D127C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826D1280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1284: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D128C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1290: 386AA76C  addi r3, r10, -0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + -22676;
	// 826D1294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D129C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D12A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D12A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D12A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D12AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D12B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D12B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D12B8: 4BD95B69  bl 0x82466e20
	ctx.lr = 0x826D12BC;
	sub_82466E20(ctx, base);
	// 826D12BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D12C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D12C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D12C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D12D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D12D0 size=112
    let mut pc: u32 = 0x826D12D0;
    'dispatch: loop {
        match pc {
            0x826D12D0 => {
    //   block [0x826D12D0..0x826D1340)
	// 826D12D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D12D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D12D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D12DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D12E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D12E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D12E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D12EC: 390BDD58  addi r8, r11, -0x22a8
	ctx.r[8].s64 = ctx.r[11].s64 + -8872;
	// 826D12F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D12F4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826D12F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D12FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1308: 386AA79C  addi r3, r10, -0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + -22628;
	// 826D130C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D131C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D132C: 4BD95AF5  bl 0x82466e20
	ctx.lr = 0x826D1330;
	sub_82466E20(ctx, base);
	// 826D1330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1340 size=108
    let mut pc: u32 = 0x826D1340;
    'dispatch: loop {
        match pc {
            0x826D1340 => {
    //   block [0x826D1340..0x826D13AC)
	// 826D1340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D134C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1354: 38EBDDA0  addi r7, r11, -0x2260
	ctx.r[7].s64 = ctx.r[11].s64 + -8800;
	// 826D1358: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D135C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826D1360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D136C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1370: 386AA7CC  addi r3, r10, -0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + -22580;
	// 826D1374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D137C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1398: 4BD95A89  bl 0x82466e20
	ctx.lr = 0x826D139C;
	sub_82466E20(ctx, base);
	// 826D139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D13A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D13A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D13A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D13B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D13B0 size=112
    let mut pc: u32 = 0x826D13B0;
    'dispatch: loop {
        match pc {
            0x826D13B0 => {
    //   block [0x826D13B0..0x826D1420)
	// 826D13B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D13B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D13B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D13BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D13C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D13C4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D13C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D13CC: 390BDDB8  addi r8, r11, -0x2248
	ctx.r[8].s64 = ctx.r[11].s64 + -8776;
	// 826D13D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D13D4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826D13D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D13DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D13E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D13E8: 386AA7FC  addi r3, r10, -0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + -22532;
	// 826D13EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D13F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D13F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D13F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D13FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D140C: 4BD95A15  bl 0x82466e20
	ctx.lr = 0x826D1410;
	sub_82466E20(ctx, base);
	// 826D1410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1420 size=112
    let mut pc: u32 = 0x826D1420;
    'dispatch: loop {
        match pc {
            0x826D1420 => {
    //   block [0x826D1420..0x826D1490)
	// 826D1420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D142C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1430: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1434: 38AAA8BC  addi r5, r10, -0x5744
	ctx.r[5].s64 = ctx.r[10].s64 + -22340;
	// 826D1438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D143C: 390BDDE8  addi r8, r11, -0x2218
	ctx.r[8].s64 = ctx.r[11].s64 + -8728;
	// 826D1440: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D1444: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 826D1448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D144C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1458: 386AA82C  addi r3, r10, -0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22484;
	// 826D145C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D146C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D147C: 4BD959A5  bl 0x82466e20
	ctx.lr = 0x826D1480;
	sub_82466E20(ctx, base);
	// 826D1480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D148C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1490 size=108
    let mut pc: u32 = 0x826D1490;
    'dispatch: loop {
        match pc {
            0x826D1490 => {
    //   block [0x826D1490..0x826D14FC)
	// 826D1490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D149C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D14A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D14A4: 38EBDE60  addi r7, r11, -0x21a0
	ctx.r[7].s64 = ctx.r[11].s64 + -8608;
	// 826D14A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D14AC: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 826D14B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D14B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D14B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D14BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D14C0: 386AA85C  addi r3, r10, -0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22436;
	// 826D14C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D14C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D14CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D14D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D14D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D14D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D14DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D14E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D14E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D14E8: 4BD95939  bl 0x82466e20
	ctx.lr = 0x826D14EC;
	sub_82466E20(ctx, base);
	// 826D14EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D14F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D14F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D14F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1500 size=108
    let mut pc: u32 = 0x826D1500;
    'dispatch: loop {
        match pc {
            0x826D1500 => {
    //   block [0x826D1500..0x826D156C)
	// 826D1500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D150C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D1514: 38EBDEA8  addi r7, r11, -0x2158
	ctx.r[7].s64 = ctx.r[11].s64 + -8536;
	// 826D1518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D151C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 826D1520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1524: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D152C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1530: 386AA88C  addi r3, r10, -0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + -22388;
	// 826D1534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D153C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D154C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1558: 4BD958C9  bl 0x82466e20
	ctx.lr = 0x826D155C;
	sub_82466E20(ctx, base);
	// 826D155C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1570 size=116
    let mut pc: u32 = 0x826D1570;
    'dispatch: loop {
        match pc {
            0x826D1570 => {
    //   block [0x826D1570..0x826D15E4)
	// 826D1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D157C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1580: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D1584: 390ADEF0  addi r8, r10, -0x2110
	ctx.r[8].s64 = ctx.r[10].s64 + -8464;
	// 826D1588: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D158C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D1590: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D1594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1598: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D159C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D15A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D15A4: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826D15A8: 396B3B78  addi r11, r11, 0x3b78
	ctx.r[11].s64 = ctx.r[11].s64 + 15224;
	// 826D15AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D15B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D15B4: 386AA8BC  addi r3, r10, -0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + -22340;
	// 826D15B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D15BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D15C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D15C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D15C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D15CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D15D0: 4BD95851  bl 0x82466e20
	ctx.lr = 0x826D15D4;
	sub_82466E20(ctx, base);
	// 826D15D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D15D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D15DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D15E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D15E8 size=112
    let mut pc: u32 = 0x826D15E8;
    'dispatch: loop {
        match pc {
            0x826D15E8 => {
    //   block [0x826D15E8..0x826D1658)
	// 826D15E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D15EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D15F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D15F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D15F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D15FC: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D1600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1604: 390BDFE0  addi r8, r11, -0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + -8224;
	// 826D1608: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D160C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826D1610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D161C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1620: 386AA8EC  addi r3, r10, -0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + -22292;
	// 826D1624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D162C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D163C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1644: 4BD957DD  bl 0x82466e20
	ctx.lr = 0x826D1648;
	sub_82466E20(ctx, base);
	// 826D1648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1658 size=112
    let mut pc: u32 = 0x826D1658;
    'dispatch: loop {
        match pc {
            0x826D1658 => {
    //   block [0x826D1658..0x826D16C8)
	// 826D1658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1668: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D166C: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D1670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1674: 390BE028  addi r8, r11, -0x1fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -8152;
	// 826D1678: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D167C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826D1680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D168C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1690: 386AA91C  addi r3, r10, -0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22244;
	// 826D1694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D169C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D16A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D16A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D16A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D16AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D16B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D16B4: 4BD9576D  bl 0x82466e20
	ctx.lr = 0x826D16B8;
	sub_82466E20(ctx, base);
	// 826D16B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D16BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D16C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D16C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D16C8 size=112
    let mut pc: u32 = 0x826D16C8;
    'dispatch: loop {
        match pc {
            0x826D16C8 => {
    //   block [0x826D16C8..0x826D1738)
	// 826D16C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D16CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D16D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D16D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D16D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D16DC: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D16E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D16E4: 390BE088  addi r8, r11, -0x1f78
	ctx.r[8].s64 = ctx.r[11].s64 + -8056;
	// 826D16E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D16EC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826D16F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D16F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D16F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D16FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1700: 386AA94C  addi r3, r10, -0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22196;
	// 826D1704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D170C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D171C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1724: 4BD956FD  bl 0x82466e20
	ctx.lr = 0x826D1728;
	sub_82466E20(ctx, base);
	// 826D1728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1738 size=112
    let mut pc: u32 = 0x826D1738;
    'dispatch: loop {
        match pc {
            0x826D1738 => {
    //   block [0x826D1738..0x826D17A8)
	// 826D1738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D173C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1748: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D174C: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D1750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1754: 390BE0E8  addi r8, r11, -0x1f18
	ctx.r[8].s64 = ctx.r[11].s64 + -7960;
	// 826D1758: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D175C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826D1760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D176C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1770: 386AA97C  addi r3, r10, -0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + -22148;
	// 826D1774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D177C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D178C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1794: 4BD9568D  bl 0x82466e20
	ctx.lr = 0x826D1798;
	sub_82466E20(ctx, base);
	// 826D1798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D179C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D17A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D17A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D17A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D17A8 size=112
    let mut pc: u32 = 0x826D17A8;
    'dispatch: loop {
        match pc {
            0x826D17A8 => {
    //   block [0x826D17A8..0x826D1818)
	// 826D17A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D17AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D17B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D17B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D17B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D17BC: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D17C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D17C4: 390BE148  addi r8, r11, -0x1eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7864;
	// 826D17C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D17CC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826D17D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D17D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D17D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D17DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D17E0: 386AA9AC  addi r3, r10, -0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + -22100;
	// 826D17E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D17E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D17EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D17F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D17F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D17F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D17FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1804: 4BD9561D  bl 0x82466e20
	ctx.lr = 0x826D1808;
	sub_82466E20(ctx, base);
	// 826D1808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D180C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1818 size=108
    let mut pc: u32 = 0x826D1818;
    'dispatch: loop {
        match pc {
            0x826D1818 => {
    //   block [0x826D1818..0x826D1884)
	// 826D1818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D181C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1824: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D182C: 38EBE208  addi r7, r11, -0x1df8
	ctx.r[7].s64 = ctx.r[11].s64 + -7672;
	// 826D1830: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826D1834: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826D1838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D183C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1848: 386AA9DC  addi r3, r10, -0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + -22052;
	// 826D184C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D186C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1870: 4BD955B1  bl 0x82466e20
	ctx.lr = 0x826D1874;
	sub_82466E20(ctx, base);
	// 826D1874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D187C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1888 size=112
    let mut pc: u32 = 0x826D1888;
    'dispatch: loop {
        match pc {
            0x826D1888 => {
    //   block [0x826D1888..0x826D18F8)
	// 826D1888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D188C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1894: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1898: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D189C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D18A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D18A4: 390BE3A0  addi r8, r11, -0x1c60
	ctx.r[8].s64 = ctx.r[11].s64 + -7264;
	// 826D18A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D18AC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826D18B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D18B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D18B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D18BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D18C0: 386AAA0C  addi r3, r10, -0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22004;
	// 826D18C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D18C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D18CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D18D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D18D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D18D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D18DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D18E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D18E4: 4BD9553D  bl 0x82466e20
	ctx.lr = 0x826D18E8;
	sub_82466E20(ctx, base);
	// 826D18E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D18EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D18F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D18F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D18F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D18F8 size=112
    let mut pc: u32 = 0x826D18F8;
    'dispatch: loop {
        match pc {
            0x826D18F8 => {
    //   block [0x826D18F8..0x826D1968)
	// 826D18F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D18FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1908: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D190C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1914: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 826D1918: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D191C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826D1920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1924: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D192C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1930: 386AAA3C  addi r3, r10, -0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21956;
	// 826D1934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D193C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1944: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D1948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D194C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1954: 4BD954CD  bl 0x82466e20
	ctx.lr = 0x826D1958;
	sub_82466E20(ctx, base);
	// 826D1958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D195C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1968 size=112
    let mut pc: u32 = 0x826D1968;
    'dispatch: loop {
        match pc {
            0x826D1968 => {
    //   block [0x826D1968..0x826D19D8)
	// 826D1968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D196C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1978: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D197C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1984: 390BE3D0  addi r8, r11, -0x1c30
	ctx.r[8].s64 = ctx.r[11].s64 + -7216;
	// 826D1988: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D198C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826D1990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D199C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D19A0: 386AAA6C  addi r3, r10, -0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + -21908;
	// 826D19A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D19A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D19AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D19B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D19B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D19B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D19BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D19C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D19C4: 4BD9545D  bl 0x82466e20
	ctx.lr = 0x826D19C8;
	sub_82466E20(ctx, base);
	// 826D19C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D19CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D19D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D19D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D19D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D19D8 size=108
    let mut pc: u32 = 0x826D19D8;
    'dispatch: loop {
        match pc {
            0x826D19D8 => {
    //   block [0x826D19D8..0x826D1A44)
	// 826D19D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D19DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D19E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D19E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D19E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D19EC: 38EBE400  addi r7, r11, -0x1c00
	ctx.r[7].s64 = ctx.r[11].s64 + -7168;
	// 826D19F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D19F4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826D19F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D19FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1A08: 386AAA9C  addi r3, r10, -0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + -21860;
	// 826D1A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1A30: 4BD953F1  bl 0x82466e20
	ctx.lr = 0x826D1A34;
	sub_82466E20(ctx, base);
	// 826D1A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1A48 size=112
    let mut pc: u32 = 0x826D1A48;
    'dispatch: loop {
        match pc {
            0x826D1A48 => {
    //   block [0x826D1A48..0x826D1AB8)
	// 826D1A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1A54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1A5C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1A64: 390BE430  addi r8, r11, -0x1bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -7120;
	// 826D1A68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1A6C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826D1A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1A80: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 826D1A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1A94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D1A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1AA4: 4BD9537D  bl 0x82466e20
	ctx.lr = 0x826D1AA8;
	sub_82466E20(ctx, base);
	// 826D1AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1AB8 size=112
    let mut pc: u32 = 0x826D1AB8;
    'dispatch: loop {
        match pc {
            0x826D1AB8 => {
    //   block [0x826D1AB8..0x826D1B28)
	// 826D1AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1AC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1AC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1ACC: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D1AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1AD4: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 826D1AD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D1ADC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826D1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1AE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1AF0: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 826D1AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1B14: 4BD9530D  bl 0x82466e20
	ctx.lr = 0x826D1B18;
	sub_82466E20(ctx, base);
	// 826D1B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1B28 size=108
    let mut pc: u32 = 0x826D1B28;
    'dispatch: loop {
        match pc {
            0x826D1B28 => {
    //   block [0x826D1B28..0x826D1B94)
	// 826D1B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1B34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1B3C: 38EBE4D8  addi r7, r11, -0x1b28
	ctx.r[7].s64 = ctx.r[11].s64 + -6952;
	// 826D1B40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D1B44: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826D1B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1B58: 386AAB2C  addi r3, r10, -0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21716;
	// 826D1B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1B80: 4BD952A1  bl 0x82466e20
	ctx.lr = 0x826D1B84;
	sub_82466E20(ctx, base);
	// 826D1B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1B98 size=108
    let mut pc: u32 = 0x826D1B98;
    'dispatch: loop {
        match pc {
            0x826D1B98 => {
    //   block [0x826D1B98..0x826D1C04)
	// 826D1B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1BA4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1BAC: 38EBE520  addi r7, r11, -0x1ae0
	ctx.r[7].s64 = ctx.r[11].s64 + -6880;
	// 826D1BB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1BB4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826D1BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1BBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1BC8: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 826D1BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1BF0: 4BD95231  bl 0x82466e20
	ctx.lr = 0x826D1BF4;
	sub_82466E20(ctx, base);
	// 826D1BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1C08 size=108
    let mut pc: u32 = 0x826D1C08;
    'dispatch: loop {
        match pc {
            0x826D1C08 => {
    //   block [0x826D1C08..0x826D1C74)
	// 826D1C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1C14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1C1C: 38EBE550  addi r7, r11, -0x1ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -6832;
	// 826D1C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1C24: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826D1C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1C2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1C38: 386AAB8C  addi r3, r10, -0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + -21620;
	// 826D1C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1C60: 4BD951C1  bl 0x82466e20
	ctx.lr = 0x826D1C64;
	sub_82466E20(ctx, base);
	// 826D1C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1C78 size=112
    let mut pc: u32 = 0x826D1C78;
    'dispatch: loop {
        match pc {
            0x826D1C78 => {
    //   block [0x826D1C78..0x826D1CE8)
	// 826D1C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1C88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1C8C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1C94: 390BE580  addi r8, r11, -0x1a80
	ctx.r[8].s64 = ctx.r[11].s64 + -6784;
	// 826D1C98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D1C9C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826D1CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1CB0: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 826D1CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1CD4: 4BD9514D  bl 0x82466e20
	ctx.lr = 0x826D1CD8;
	sub_82466E20(ctx, base);
	// 826D1CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1CE8 size=112
    let mut pc: u32 = 0x826D1CE8;
    'dispatch: loop {
        match pc {
            0x826D1CE8 => {
    //   block [0x826D1CE8..0x826D1D58)
	// 826D1CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1CF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1CFC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1D04: 390BE5B0  addi r8, r11, -0x1a50
	ctx.r[8].s64 = ctx.r[11].s64 + -6736;
	// 826D1D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1D0C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826D1D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1D14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1D20: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 826D1D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1D44: 4BD950DD  bl 0x82466e20
	ctx.lr = 0x826D1D48;
	sub_82466E20(ctx, base);
	// 826D1D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1D58 size=112
    let mut pc: u32 = 0x826D1D58;
    'dispatch: loop {
        match pc {
            0x826D1D58 => {
    //   block [0x826D1D58..0x826D1DC8)
	// 826D1D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1D6C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1D74: 390BE5C8  addi r8, r11, -0x1a38
	ctx.r[8].s64 = ctx.r[11].s64 + -6712;
	// 826D1D78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1D7C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826D1D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1D84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1D90: 386AAC1C  addi r3, r10, -0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21476;
	// 826D1D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1DB4: 4BD9506D  bl 0x82466e20
	ctx.lr = 0x826D1DB8;
	sub_82466E20(ctx, base);
	// 826D1DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1DC8 size=108
    let mut pc: u32 = 0x826D1DC8;
    'dispatch: loop {
        match pc {
            0x826D1DC8 => {
    //   block [0x826D1DC8..0x826D1E34)
	// 826D1DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1DD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1DDC: 38EBE5E0  addi r7, r11, -0x1a20
	ctx.r[7].s64 = ctx.r[11].s64 + -6688;
	// 826D1DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1DE4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826D1DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1DEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1DF8: 386AAC4C  addi r3, r10, -0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21428;
	// 826D1DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1E20: 4BD95001  bl 0x82466e20
	ctx.lr = 0x826D1E24;
	sub_82466E20(ctx, base);
	// 826D1E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1E38 size=112
    let mut pc: u32 = 0x826D1E38;
    'dispatch: loop {
        match pc {
            0x826D1E38 => {
    //   block [0x826D1E38..0x826D1EA8)
	// 826D1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1E48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1E4C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1E54: 390BE610  addi r8, r11, -0x19f0
	ctx.r[8].s64 = ctx.r[11].s64 + -6640;
	// 826D1E58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1E5C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826D1E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1E70: 386AAC7C  addi r3, r10, -0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + -21380;
	// 826D1E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1E94: 4BD94F8D  bl 0x82466e20
	ctx.lr = 0x826D1E98;
	sub_82466E20(ctx, base);
	// 826D1E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1EA8 size=108
    let mut pc: u32 = 0x826D1EA8;
    'dispatch: loop {
        match pc {
            0x826D1EA8 => {
    //   block [0x826D1EA8..0x826D1F14)
	// 826D1EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1EB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1EBC: 38EBE628  addi r7, r11, -0x19d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6616;
	// 826D1EC0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D1EC4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826D1EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1ED8: 386AACAC  addi r3, r10, -0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + -21332;
	// 826D1EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1F00: 4BD94F21  bl 0x82466e20
	ctx.lr = 0x826D1F04;
	sub_82466E20(ctx, base);
	// 826D1F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1F18 size=112
    let mut pc: u32 = 0x826D1F18;
    'dispatch: loop {
        match pc {
            0x826D1F18 => {
    //   block [0x826D1F18..0x826D1F88)
	// 826D1F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1F28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1F2C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1F34: 390BE700  addi r8, r11, -0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + -6400;
	// 826D1F38: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826D1F3C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826D1F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1F50: 386AACDC  addi r3, r10, -0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + -21284;
	// 826D1F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1F74: 4BD94EAD  bl 0x82466e20
	ctx.lr = 0x826D1F78;
	sub_82466E20(ctx, base);
	// 826D1F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1F88 size=108
    let mut pc: u32 = 0x826D1F88;
    'dispatch: loop {
        match pc {
            0x826D1F88 => {
    //   block [0x826D1F88..0x826D1FF4)
	// 826D1F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1F94: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1F9C: 38EBE8B0  addi r7, r11, -0x1750
	ctx.r[7].s64 = ctx.r[11].s64 + -5968;
	// 826D1FA0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826D1FA4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826D1FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1FAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1FB8: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 826D1FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1FE0: 4BD94E41  bl 0x82466e20
	ctx.lr = 0x826D1FE4;
	sub_82466E20(ctx, base);
	// 826D1FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1FF8 size=112
    let mut pc: u32 = 0x826D1FF8;
    'dispatch: loop {
        match pc {
            0x826D1FF8 => {
    //   block [0x826D1FF8..0x826D2068)
	// 826D1FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2008: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D200C: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D2010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2014: 390BEA48  addi r8, r11, -0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5560;
	// 826D2018: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 826D201C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826D2020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2030: 386AAD3C  addi r3, r10, -0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21188;
	// 826D2034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D203C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D204C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2054: 4BD94DCD  bl 0x82466e20
	ctx.lr = 0x826D2058;
	sub_82466E20(ctx, base);
	// 826D2058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2068 size=100
    let mut pc: u32 = 0x826D2068;
    'dispatch: loop {
        match pc {
            0x826D2068 => {
    //   block [0x826D2068..0x826D20CC)
	// 826D2068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D207C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2088: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826D208C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D209C: 386AAD6C  addi r3, r10, -0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + -21140;
	// 826D20A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D20A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D20A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D20AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D20B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D20B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D20B8: 4BD94D69  bl 0x82466e20
	ctx.lr = 0x826D20BC;
	sub_82466E20(ctx, base);
	// 826D20BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D20C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D20C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D20C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D20D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D20D0 size=112
    let mut pc: u32 = 0x826D20D0;
    'dispatch: loop {
        match pc {
            0x826D20D0 => {
    //   block [0x826D20D0..0x826D2140)
	// 826D20D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D20D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D20D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D20DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D20E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D20E4: 38AAAD6C  addi r5, r10, -0x5294
	ctx.r[5].s64 = ctx.r[10].s64 + -21140;
	// 826D20E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D20EC: 390BECB8  addi r8, r11, -0x1348
	ctx.r[8].s64 = ctx.r[11].s64 + -4936;
	// 826D20F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D20F4: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826D20F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D20FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2108: 386AAD9C  addi r3, r10, -0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + -21092;
	// 826D210C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D211C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D212C: 4BD94CF5  bl 0x82466e20
	ctx.lr = 0x826D2130;
	sub_82466E20(ctx, base);
	// 826D2130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D213C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2140 size=100
    let mut pc: u32 = 0x826D2140;
    'dispatch: loop {
        match pc {
            0x826D2140 => {
    //   block [0x826D2140..0x826D21A4)
	// 826D2140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D214C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2154: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D215C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2160: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826D2164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D216C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2174: 386AADCC  addi r3, r10, -0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + -21044;
	// 826D2178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D217C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2190: 4BD94C91  bl 0x82466e20
	ctx.lr = 0x826D2194;
	sub_82466E20(ctx, base);
	// 826D2194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D219C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D21A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D21A8 size=108
    let mut pc: u32 = 0x826D21A8;
    'dispatch: loop {
        match pc {
            0x826D21A8 => {
    //   block [0x826D21A8..0x826D2214)
	// 826D21A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D21AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D21B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D21B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D21B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D21BC: 38EBED30  addi r7, r11, -0x12d0
	ctx.r[7].s64 = ctx.r[11].s64 + -4816;
	// 826D21C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D21C4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826D21C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D21CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D21D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D21D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D21D8: 386AADFC  addi r3, r10, -0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + -20996;
	// 826D21DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D21E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D21E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D21E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D21EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D21F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D21F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D21F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D21FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2200: 4BD94C21  bl 0x82466e20
	ctx.lr = 0x826D2204;
	sub_82466E20(ctx, base);
	// 826D2204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2218 size=112
    let mut pc: u32 = 0x826D2218;
    'dispatch: loop {
        match pc {
            0x826D2218 => {
    //   block [0x826D2218..0x826D2288)
	// 826D2218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D222C: 38AAADCC  addi r5, r10, -0x5234
	ctx.r[5].s64 = ctx.r[10].s64 + -21044;
	// 826D2230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2234: 390BED78  addi r8, r11, -0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + -4744;
	// 826D2238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D223C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826D2240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D224C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2250: 386AAE2C  addi r3, r10, -0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + -20948;
	// 826D2254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D225C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D226C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2274: 4BD94BAD  bl 0x82466e20
	ctx.lr = 0x826D2278;
	sub_82466E20(ctx, base);
	// 826D2278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D227C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2288 size=100
    let mut pc: u32 = 0x826D2288;
    'dispatch: loop {
        match pc {
            0x826D2288 => {
    //   block [0x826D2288..0x826D22EC)
	// 826D2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D229C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D22A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D22A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D22A8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826D22AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D22B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D22B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D22B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D22BC: 386AAE5C  addi r3, r10, -0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + -20900;
	// 826D22C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D22C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D22C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D22CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D22D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D22D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D22D8: 4BD94B49  bl 0x82466e20
	ctx.lr = 0x826D22DC;
	sub_82466E20(ctx, base);
	// 826D22DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D22E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D22E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D22E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D22F0 size=100
    let mut pc: u32 = 0x826D22F0;
    'dispatch: loop {
        match pc {
            0x826D22F0 => {
    //   block [0x826D22F0..0x826D2354)
	// 826D22F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D22F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D22F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D22FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2304: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D230C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2310: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826D2314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D231C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2324: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 826D2328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D232C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2330: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D233C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2340: 4BD94AE1  bl 0x82466e20
	ctx.lr = 0x826D2344;
	sub_82466E20(ctx, base);
	// 826D2344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D234C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2358 size=112
    let mut pc: u32 = 0x826D2358;
    'dispatch: loop {
        match pc {
            0x826D2358 => {
    //   block [0x826D2358..0x826D23C8)
	// 826D2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2368: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D236C: 38AAAE5C  addi r5, r10, -0x51a4
	ctx.r[5].s64 = ctx.r[10].s64 + -20900;
	// 826D2370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2374: 390BEDA8  addi r8, r11, -0x1258
	ctx.r[8].s64 = ctx.r[11].s64 + -4696;
	// 826D2378: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D237C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826D2380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D238C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2390: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 826D2394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D239C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D23A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D23A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D23A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D23AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D23B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D23B4: 4BD94A6D  bl 0x82466e20
	ctx.lr = 0x826D23B8;
	sub_82466E20(ctx, base);
	// 826D23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D23C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D23C8 size=112
    let mut pc: u32 = 0x826D23C8;
    'dispatch: loop {
        match pc {
            0x826D23C8 => {
    //   block [0x826D23C8..0x826D2438)
	// 826D23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D23D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D23D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D23DC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 826D23E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D23E4: 390BEE08  addi r8, r11, -0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + -4600;
	// 826D23E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D23EC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826D23F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D23F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D23F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D23FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2400: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 826D2404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D240C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D241C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2424: 4BD949FD  bl 0x82466e20
	ctx.lr = 0x826D2428;
	sub_82466E20(ctx, base);
	// 826D2428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D242C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2438 size=100
    let mut pc: u32 = 0x826D2438;
    'dispatch: loop {
        match pc {
            0x826D2438 => {
    //   block [0x826D2438..0x826D249C)
	// 826D2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D244C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2458: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826D245C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D246C: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 826D2470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D247C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D2484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2488: 4BD94999  bl 0x82466e20
	ctx.lr = 0x826D248C;
	sub_82466E20(ctx, base);
	// 826D248C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D24A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D24A0 size=112
    let mut pc: u32 = 0x826D24A0;
    'dispatch: loop {
        match pc {
            0x826D24A0 => {
    //   block [0x826D24A0..0x826D2510)
	// 826D24A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D24A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D24A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D24AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D24B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D24B4: 38AAAF1C  addi r5, r10, -0x50e4
	ctx.r[5].s64 = ctx.r[10].s64 + -20708;
	// 826D24B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D24BC: 390BEE68  addi r8, r11, -0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + -4504;
	// 826D24C0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D24C4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826D24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D24CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D24D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D24D8: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 826D24DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D24E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D24E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D24E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D24F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D24F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D24F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D24FC: 4BD94925  bl 0x82466e20
	ctx.lr = 0x826D2500;
	sub_82466E20(ctx, base);
	// 826D2500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2510 size=100
    let mut pc: u32 = 0x826D2510;
    'dispatch: loop {
        match pc {
            0x826D2510 => {
    //   block [0x826D2510..0x826D2574)
	// 826D2510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D251C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2524: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D252C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2530: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826D2534: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2544: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 826D2548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D254C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D255C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2560: 4BD948C1  bl 0x82466e20
	ctx.lr = 0x826D2564;
	sub_82466E20(ctx, base);
	// 826D2564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D256C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2578 size=112
    let mut pc: u32 = 0x826D2578;
    'dispatch: loop {
        match pc {
            0x826D2578 => {
    //   block [0x826D2578..0x826D25E8)
	// 826D2578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D257C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2588: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D258C: 38AAAF7C  addi r5, r10, -0x5084
	ctx.r[5].s64 = ctx.r[10].s64 + -20612;
	// 826D2590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2594: 390BEF58  addi r8, r11, -0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + -4264;
	// 826D2598: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D259C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826D25A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D25A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D25A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D25AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D25B0: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 826D25B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D25B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D25BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D25C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D25C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D25C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D25CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D25D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D25D4: 4BD9484D  bl 0x82466e20
	ctx.lr = 0x826D25D8;
	sub_82466E20(ctx, base);
	// 826D25D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D25DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D25E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D25E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D25E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D25E8 size=108
    let mut pc: u32 = 0x826D25E8;
    'dispatch: loop {
        match pc {
            0x826D25E8 => {
    //   block [0x826D25E8..0x826D2654)
	// 826D25E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D25EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D25F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D25F4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D25F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D25FC: 38EBEFA0  addi r7, r11, -0x1060
	ctx.r[7].s64 = ctx.r[11].s64 + -4192;
	// 826D2600: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D2604: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826D2608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D260C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2618: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 826D261C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D262C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D263C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2640: 4BD947E1  bl 0x82466e20
	ctx.lr = 0x826D2644;
	sub_82466E20(ctx, base);
	// 826D2644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D264C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2658 size=112
    let mut pc: u32 = 0x826D2658;
    'dispatch: loop {
        match pc {
            0x826D2658 => {
    //   block [0x826D2658..0x826D26C8)
	// 826D2658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D265C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2668: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D266C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2674: 390BEFE8  addi r8, r11, -0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + -4120;
	// 826D2678: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D267C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826D2680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D268C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2690: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 826D2694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D269C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D26A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D26A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D26A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D26AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D26B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D26B4: 4BD9476D  bl 0x82466e20
	ctx.lr = 0x826D26B8;
	sub_82466E20(ctx, base);
	// 826D26B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D26BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D26C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D26C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D26C8 size=108
    let mut pc: u32 = 0x826D26C8;
    'dispatch: loop {
        match pc {
            0x826D26C8 => {
    //   block [0x826D26C8..0x826D2734)
	// 826D26C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D26CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D26D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D26D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D26D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D26DC: 38EBF000  addi r7, r11, -0x1000
	ctx.r[7].s64 = ctx.r[11].s64 + -4096;
	// 826D26E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D26E4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826D26E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D26EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D26F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D26F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D26F8: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 826D26FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D271C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2720: 4BD94701  bl 0x82466e20
	ctx.lr = 0x826D2724;
	sub_82466E20(ctx, base);
	// 826D2724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D272C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2738 size=112
    let mut pc: u32 = 0x826D2738;
    'dispatch: loop {
        match pc {
            0x826D2738 => {
    //   block [0x826D2738..0x826D27A8)
	// 826D2738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2748: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D274C: 38AAB00C  addi r5, r10, -0x4ff4
	ctx.r[5].s64 = ctx.r[10].s64 + -20468;
	// 826D2750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2754: 390BF048  addi r8, r11, -0xfb8
	ctx.r[8].s64 = ctx.r[11].s64 + -4024;
	// 826D2758: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D275C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826D2760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2770: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 826D2774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2794: 4BD9468D  bl 0x82466e20
	ctx.lr = 0x826D2798;
	sub_82466E20(ctx, base);
	// 826D2798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D27A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D27A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D27A8 size=100
    let mut pc: u32 = 0x826D27A8;
    'dispatch: loop {
        match pc {
            0x826D27A8 => {
    //   block [0x826D27A8..0x826D280C)
	// 826D27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D27AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D27B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D27B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D27B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D27BC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D27C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D27C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D27C8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826D27CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D27D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D27D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D27D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D27DC: 386AB09C  addi r3, r10, -0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + -20324;
	// 826D27E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D27E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D27E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D27F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D27F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D27F8: 4BD94629  bl 0x82466e20
	ctx.lr = 0x826D27FC;
	sub_82466E20(ctx, base);
	// 826D27FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2810 size=112
    let mut pc: u32 = 0x826D2810;
    'dispatch: loop {
        match pc {
            0x826D2810 => {
    //   block [0x826D2810..0x826D2880)
	// 826D2810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D281C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2820: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2824: 38AAB09C  addi r5, r10, -0x4f64
	ctx.r[5].s64 = ctx.r[10].s64 + -20324;
	// 826D2828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D282C: 390BF060  addi r8, r11, -0xfa0
	ctx.r[8].s64 = ctx.r[11].s64 + -4000;
	// 826D2830: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D2834: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826D2838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D283C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2848: 386AB0CC  addi r3, r10, -0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + -20276;
	// 826D284C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D285C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D286C: 4BD945B5  bl 0x82466e20
	ctx.lr = 0x826D2870;
	sub_82466E20(ctx, base);
	// 826D2870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D287C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2880 size=108
    let mut pc: u32 = 0x826D2880;
    'dispatch: loop {
        match pc {
            0x826D2880 => {
    //   block [0x826D2880..0x826D28EC)
	// 826D2880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D288C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2894: 38EBF108  addi r7, r11, -0xef8
	ctx.r[7].s64 = ctx.r[11].s64 + -3832;
	// 826D2898: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D289C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826D28A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D28A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D28A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D28AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D28B0: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 826D28B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D28B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D28BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D28C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D28C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D28C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D28CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D28D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D28D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D28D8: 4BD94549  bl 0x82466e20
	ctx.lr = 0x826D28DC;
	sub_82466E20(ctx, base);
	// 826D28DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D28E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D28E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D28E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D28F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D28F0 size=112
    let mut pc: u32 = 0x826D28F0;
    'dispatch: loop {
        match pc {
            0x826D28F0 => {
    //   block [0x826D28F0..0x826D2960)
	// 826D28F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D28F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D28F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D28FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2900: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2904: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D290C: 390BF138  addi r8, r11, -0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + -3784;
	// 826D2910: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2914: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826D2918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D291C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2928: 386AB12C  addi r3, r10, -0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -20180;
	// 826D292C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D293C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D294C: 4BD944D5  bl 0x82466e20
	ctx.lr = 0x826D2950;
	sub_82466E20(ctx, base);
	// 826D2950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D295C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2960 size=112
    let mut pc: u32 = 0x826D2960;
    'dispatch: loop {
        match pc {
            0x826D2960 => {
    //   block [0x826D2960..0x826D29D0)
	// 826D2960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D296C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2970: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2974: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D297C: 390BF180  addi r8, r11, -0xe80
	ctx.r[8].s64 = ctx.r[11].s64 + -3712;
	// 826D2980: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2984: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826D2988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D298C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2998: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 826D299C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D29A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D29A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D29A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D29AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D29B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D29B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D29B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D29BC: 4BD94465  bl 0x82466e20
	ctx.lr = 0x826D29C0;
	sub_82466E20(ctx, base);
	// 826D29C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D29C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D29C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D29CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D29D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D29D0 size=100
    let mut pc: u32 = 0x826D29D0;
    'dispatch: loop {
        match pc {
            0x826D29D0 => {
    //   block [0x826D29D0..0x826D2A34)
	// 826D29D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D29D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D29D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D29DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D29E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D29E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D29E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D29EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D29F0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826D29F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D29F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D29FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2A04: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 826D2A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2A10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2A18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D2A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2A20: 4BD94401  bl 0x82466e20
	ctx.lr = 0x826D2A24;
	sub_82466E20(ctx, base);
	// 826D2A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2A38 size=112
    let mut pc: u32 = 0x826D2A38;
    'dispatch: loop {
        match pc {
            0x826D2A38 => {
    //   block [0x826D2A38..0x826D2AA8)
	// 826D2A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2A44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2A48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2A4C: 38AAB18C  addi r5, r10, -0x4e74
	ctx.r[5].s64 = ctx.r[10].s64 + -20084;
	// 826D2A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2A54: 390BF1C8  addi r8, r11, -0xe38
	ctx.r[8].s64 = ctx.r[11].s64 + -3640;
	// 826D2A58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2A5C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826D2A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2A64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2A70: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 826D2A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2A94: 4BD9438D  bl 0x82466e20
	ctx.lr = 0x826D2A98;
	sub_82466E20(ctx, base);
	// 826D2A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2AA8 size=112
    let mut pc: u32 = 0x826D2AA8;
    'dispatch: loop {
        match pc {
            0x826D2AA8 => {
    //   block [0x826D2AA8..0x826D2B18)
	// 826D2AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2AB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2AB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2ABC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2AC4: 390BF210  addi r8, r11, -0xdf0
	ctx.r[8].s64 = ctx.r[11].s64 + -3568;
	// 826D2AC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2ACC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826D2AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2AE0: 386AB1EC  addi r3, r10, -0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + -19988;
	// 826D2AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2B04: 4BD9431D  bl 0x82466e20
	ctx.lr = 0x826D2B08;
	sub_82466E20(ctx, base);
	// 826D2B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2B18 size=112
    let mut pc: u32 = 0x826D2B18;
    'dispatch: loop {
        match pc {
            0x826D2B18 => {
    //   block [0x826D2B18..0x826D2B88)
	// 826D2B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2B24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2B2C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2B34: 390BF228  addi r8, r11, -0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + -3544;
	// 826D2B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2B3C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826D2B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2B50: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 826D2B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2B64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D2B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2B74: 4BD942AD  bl 0x82466e20
	ctx.lr = 0x826D2B78;
	sub_82466E20(ctx, base);
	// 826D2B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2B88 size=112
    let mut pc: u32 = 0x826D2B88;
    'dispatch: loop {
        match pc {
            0x826D2B88 => {
    //   block [0x826D2B88..0x826D2BF8)
	// 826D2B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2B94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2B9C: 38AAB1EC  addi r5, r10, -0x4e14
	ctx.r[5].s64 = ctx.r[10].s64 + -19988;
	// 826D2BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2BA4: 390BF240  addi r8, r11, -0xdc0
	ctx.r[8].s64 = ctx.r[11].s64 + -3520;
	// 826D2BA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2BAC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826D2BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2BC0: 386AB24C  addi r3, r10, -0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + -19892;
	// 826D2BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2BE4: 4BD9423D  bl 0x82466e20
	ctx.lr = 0x826D2BE8;
	sub_82466E20(ctx, base);
	// 826D2BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2BF8 size=72
    let mut pc: u32 = 0x826D2BF8;
    'dispatch: loop {
        match pc {
            0x826D2BF8 => {
    //   block [0x826D2BF8..0x826D2C40)
	// 826D2BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2C04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D2C08: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 826D2C0C: 38CB32B0  addi r6, r11, 0x32b0
	ctx.r[6].s64 = ctx.r[11].s64 + 12976;
	// 826D2C10: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D2C14: 388B3BA8  addi r4, r11, 0x3ba8
	ctx.r[4].s64 = ctx.r[11].s64 + 15272;
	// 826D2C18: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D2C1C: 386BB27C  addi r3, r11, -0x4d84
	ctx.r[3].s64 = ctx.r[11].s64 + -19844;
	// 826D2C20: 4BDA8E69  bl 0x8247ba88
	ctx.lr = 0x826D2C24;
	sub_8247BA88(ctx, base);
	// 826D2C24: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826D2C28: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 826D2C2C: 4BE5FF0D  bl 0x82532b38
	ctx.lr = 0x826D2C30;
	sub_82532B38(ctx, base);
	// 826D2C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826D2C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2C40 size=108
    let mut pc: u32 = 0x826D2C40;
    'dispatch: loop {
        match pc {
            0x826D2C40 => {
    //   block [0x826D2C40..0x826D2CAC)
	// 826D2C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2C4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2C54: 38EBFB10  addi r7, r11, -0x4f0
	ctx.r[7].s64 = ctx.r[11].s64 + -1264;
	// 826D2C58: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D2C5C: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 826D2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2C70: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 826D2C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2C98: 4BD94189  bl 0x82466e20
	ctx.lr = 0x826D2C9C;
	sub_82466E20(ctx, base);
	// 826D2C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D2CB0 size=24
    let mut pc: u32 = 0x826D2CB0;
    'dispatch: loop {
        match pc {
            0x826D2CB0 => {
    //   block [0x826D2CB0..0x826D2CC8)
	// 826D2CB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2CB4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D2CB8: 394A91B8  addi r10, r10, -0x6e48
	ctx.r[10].s64 = ctx.r[10].s64 + -28232;
	// 826D2CBC: 816BFB88  lwz r11, -0x478(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1144 as u32) ) } as u64;
	// 826D2CC0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D2CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2CC8 size=112
    let mut pc: u32 = 0x826D2CC8;
    'dispatch: loop {
        match pc {
            0x826D2CC8 => {
    //   block [0x826D2CC8..0x826D2D38)
	// 826D2CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2CD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D2CD8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D2CDC: 392A40DC  addi r9, r10, 0x40dc
	ctx.r[9].s64 = ctx.r[10].s64 + 16604;
	// 826D2CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2CE4: 390B91B8  addi r8, r11, -0x6e48
	ctx.r[8].s64 = ctx.r[11].s64 + -28232;
	// 826D2CE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D2CEC: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 826D2CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2D00: 386AB2C4  addi r3, r10, -0x4d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19772;
	// 826D2D04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D2D08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D2D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2D24: 4BD940FD  bl 0x82466e20
	ctx.lr = 0x826D2D28;
	sub_82466E20(ctx, base);
	// 826D2D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2D38 size=108
    let mut pc: u32 = 0x826D2D38;
    'dispatch: loop {
        match pc {
            0x826D2D38 => {
    //   block [0x826D2D38..0x826D2DA4)
	// 826D2D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2D44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2D48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2D4C: 38EBFB8C  addi r7, r11, -0x474
	ctx.r[7].s64 = ctx.r[11].s64 + -1140;
	// 826D2D50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D2D54: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 826D2D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2D68: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 826D2D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2D90: 4BD94091  bl 0x82466e20
	ctx.lr = 0x826D2D94;
	sub_82466E20(ctx, base);
	// 826D2D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2DA8 size=108
    let mut pc: u32 = 0x826D2DA8;
    'dispatch: loop {
        match pc {
            0x826D2DA8 => {
    //   block [0x826D2DA8..0x826D2E14)
	// 826D2DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2DB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2DBC: 38EBFBBC  addi r7, r11, -0x444
	ctx.r[7].s64 = ctx.r[11].s64 + -1092;
	// 826D2DC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D2DC4: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 826D2DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2DD8: 386AB324  addi r3, r10, -0x4cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19676;
	// 826D2DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2E00: 4BD94021  bl 0x82466e20
	ctx.lr = 0x826D2E04;
	sub_82466E20(ctx, base);
	// 826D2E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D2E18 size=24
    let mut pc: u32 = 0x826D2E18;
    'dispatch: loop {
        match pc {
            0x826D2E18 => {
    //   block [0x826D2E18..0x826D2E30)
	// 826D2E18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2E1C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D2E20: 394A9200  addi r10, r10, -0x6e00
	ctx.r[10].s64 = ctx.r[10].s64 + -28160;
	// 826D2E24: 816BFBEC  lwz r11, -0x414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1044 as u32) ) } as u64;
	// 826D2E28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D2E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2E30 size=116
    let mut pc: u32 = 0x826D2E30;
    'dispatch: loop {
        match pc {
            0x826D2E30 => {
    //   block [0x826D2E30..0x826D2EA4)
	// 826D2E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2E3C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D2E40: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D2E44: 390B9200  addi r8, r11, -0x6e00
	ctx.r[8].s64 = ctx.r[11].s64 + -28160;
	// 826D2E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2E4C: 392A4118  addi r9, r10, 0x4118
	ctx.r[9].s64 = ctx.r[10].s64 + 16664;
	// 826D2E50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2E54: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D2E58: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D2E5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2E64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2E74: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D2E78: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 826D2E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D2E80: 386BB354  addi r3, r11, -0x4cac
	ctx.r[3].s64 = ctx.r[11].s64 + -19628;
	// 826D2E84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D2E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2E90: 4BD93F91  bl 0x82466e20
	ctx.lr = 0x826D2E94;
	sub_82466E20(ctx, base);
	// 826D2E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2EA8 size=108
    let mut pc: u32 = 0x826D2EA8;
    'dispatch: loop {
        match pc {
            0x826D2EA8 => {
    //   block [0x826D2EA8..0x826D2F14)
	// 826D2EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2EB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2EBC: 38EBFBF0  addi r7, r11, -0x410
	ctx.r[7].s64 = ctx.r[11].s64 + -1040;
	// 826D2EC0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D2EC4: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 826D2EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2ED8: 386AB384  addi r3, r10, -0x4c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19580;
	// 826D2EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2F00: 4BD93F21  bl 0x82466e20
	ctx.lr = 0x826D2F04;
	sub_82466E20(ctx, base);
	// 826D2F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2F18 size=112
    let mut pc: u32 = 0x826D2F18;
    'dispatch: loop {
        match pc {
            0x826D2F18 => {
    //   block [0x826D2F18..0x826D2F88)
	// 826D2F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2F2C: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D2F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2F34: 390BFC80  addi r8, r11, -0x380
	ctx.r[8].s64 = ctx.r[11].s64 + -896;
	// 826D2F38: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826D2F3C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826D2F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2F50: 386AB3B4  addi r3, r10, -0x4c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19532;
	// 826D2F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2F74: 4BD93EAD  bl 0x82466e20
	ctx.lr = 0x826D2F78;
	sub_82466E20(ctx, base);
	// 826D2F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2F88 size=112
    let mut pc: u32 = 0x826D2F88;
    'dispatch: loop {
        match pc {
            0x826D2F88 => {
    //   block [0x826D2F88..0x826D2FF8)
	// 826D2F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2F9C: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D2FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2FA4: 390BFDA0  addi r8, r11, -0x260
	ctx.r[8].s64 = ctx.r[11].s64 + -608;
	// 826D2FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2FAC: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 826D2FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2FB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2FC0: 386AB3E4  addi r3, r10, -0x4c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19484;
	// 826D2FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2FE4: 4BD93E3D  bl 0x82466e20
	ctx.lr = 0x826D2FE8;
	sub_82466E20(ctx, base);
	// 826D2FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2FF8 size=108
    let mut pc: u32 = 0x826D2FF8;
    'dispatch: loop {
        match pc {
            0x826D2FF8 => {
    //   block [0x826D2FF8..0x826D3064)
	// 826D2FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3004: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D300C: 38EBFDB8  addi r7, r11, -0x248
	ctx.r[7].s64 = ctx.r[11].s64 + -584;
	// 826D3010: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D3014: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826D3018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D301C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3028: 386AB414  addi r3, r10, -0x4bec
	ctx.r[3].s64 = ctx.r[10].s64 + -19436;
	// 826D302C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D303C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D304C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3050: 4BD93DD1  bl 0x82466e20
	ctx.lr = 0x826D3054;
	sub_82466E20(ctx, base);
	// 826D3054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D305C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3068 size=108
    let mut pc: u32 = 0x826D3068;
    'dispatch: loop {
        match pc {
            0x826D3068 => {
    //   block [0x826D3068..0x826D30D4)
	// 826D3068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D306C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3074: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D307C: 38EBFE90  addi r7, r11, -0x170
	ctx.r[7].s64 = ctx.r[11].s64 + -368;
	// 826D3080: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D3084: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 826D3088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D308C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3098: 386AB444  addi r3, r10, -0x4bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19388;
	// 826D309C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D30A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D30A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D30A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D30AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D30B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D30B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D30B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D30BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D30C0: 4BD93D61  bl 0x82466e20
	ctx.lr = 0x826D30C4;
	sub_82466E20(ctx, base);
	// 826D30C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D30C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D30CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D30D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D30D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D30D8 size=112
    let mut pc: u32 = 0x826D30D8;
    'dispatch: loop {
        match pc {
            0x826D30D8 => {
    //   block [0x826D30D8..0x826D3148)
	// 826D30D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D30DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D30E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D30E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D30E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D30EC: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D30F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D30F4: 390BFF20  addi r8, r11, -0xe0
	ctx.r[8].s64 = ctx.r[11].s64 + -224;
	// 826D30F8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D30FC: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826D3100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3104: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D310C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3110: 386AB474  addi r3, r10, -0x4b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19340;
	// 826D3114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D311C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D312C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3134: 4BD93CED  bl 0x82466e20
	ctx.lr = 0x826D3138;
	sub_82466E20(ctx, base);
	// 826D3138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D313C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3148 size=108
    let mut pc: u32 = 0x826D3148;
    'dispatch: loop {
        match pc {
            0x826D3148 => {
    //   block [0x826D3148..0x826D31B4)
	// 826D3148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D314C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3154: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3158: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D315C: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 826D3160: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D3164: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826D3168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D316C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3178: 386AB4A4  addi r3, r10, -0x4b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19292;
	// 826D317C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D318C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D319C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D31A0: 4BD93C81  bl 0x82466e20
	ctx.lr = 0x826D31A4;
	sub_82466E20(ctx, base);
	// 826D31A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D31B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D31B8 size=108
    let mut pc: u32 = 0x826D31B8;
    'dispatch: loop {
        match pc {
            0x826D31B8 => {
    //   block [0x826D31B8..0x826D3224)
	// 826D31B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D31BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D31C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D31C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D31C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D31CC: 38EB0028  addi r7, r11, 0x28
	ctx.r[7].s64 = ctx.r[11].s64 + 40;
	// 826D31D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D31D4: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826D31D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D31DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D31E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D31E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D31E8: 386AB4D4  addi r3, r10, -0x4b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19244;
	// 826D31EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D31F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D31F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D31F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D31FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D320C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3210: 4BD93C11  bl 0x82466e20
	ctx.lr = 0x826D3214;
	sub_82466E20(ctx, base);
	// 826D3214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D321C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3228 size=116
    let mut pc: u32 = 0x826D3228;
    'dispatch: loop {
        match pc {
            0x826D3228 => {
    //   block [0x826D3228..0x826D329C)
	// 826D3228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3234: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3238: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D323C: 390B008C  addi r8, r11, 0x8c
	ctx.r[8].s64 = ctx.r[11].s64 + 140;
	// 826D3240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3244: 392A4144  addi r9, r10, 0x4144
	ctx.r[9].s64 = ctx.r[10].s64 + 16708;
	// 826D3248: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D324C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D3250: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3254: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D325C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D326C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D3270: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826D3274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3278: 386BB504  addi r3, r11, -0x4afc
	ctx.r[3].s64 = ctx.r[11].s64 + -19196;
	// 826D327C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3280: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3288: 4BD93B99  bl 0x82466e20
	ctx.lr = 0x826D328C;
	sub_82466E20(ctx, base);
	// 826D328C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D32A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D32A0 size=108
    let mut pc: u32 = 0x826D32A0;
    'dispatch: loop {
        match pc {
            0x826D32A0 => {
    //   block [0x826D32A0..0x826D330C)
	// 826D32A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D32A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D32A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D32AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D32B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D32B4: 38EB00A8  addi r7, r11, 0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + 168;
	// 826D32B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D32BC: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826D32C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D32C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D32C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D32CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D32D0: 386AB534  addi r3, r10, -0x4acc
	ctx.r[3].s64 = ctx.r[10].s64 + -19148;
	// 826D32D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D32D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D32DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D32E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D32E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D32E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D32EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D32F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D32F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D32F8: 4BD93B29  bl 0x82466e20
	ctx.lr = 0x826D32FC;
	sub_82466E20(ctx, base);
	// 826D32FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3310 size=108
    let mut pc: u32 = 0x826D3310;
    'dispatch: loop {
        match pc {
            0x826D3310 => {
    //   block [0x826D3310..0x826D337C)
	// 826D3310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D331C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3324: 38EB00F0  addi r7, r11, 0xf0
	ctx.r[7].s64 = ctx.r[11].s64 + 240;
	// 826D3328: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D332C: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826D3330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D333C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3340: 386AB564  addi r3, r10, -0x4a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19100;
	// 826D3344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D334C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D335C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3368: 4BD93AB9  bl 0x82466e20
	ctx.lr = 0x826D336C;
	sub_82466E20(ctx, base);
	// 826D336C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3380 size=108
    let mut pc: u32 = 0x826D3380;
    'dispatch: loop {
        match pc {
            0x826D3380 => {
    //   block [0x826D3380..0x826D33EC)
	// 826D3380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D338C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3390: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3394: 38EB0180  addi r7, r11, 0x180
	ctx.r[7].s64 = ctx.r[11].s64 + 384;
	// 826D3398: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D339C: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826D33A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D33A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D33A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D33AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D33B0: 386AB594  addi r3, r10, -0x4a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19052;
	// 826D33B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D33B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D33BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D33C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D33C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D33C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D33CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D33D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D33D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D33D8: 4BD93A49  bl 0x82466e20
	ctx.lr = 0x826D33DC;
	sub_82466E20(ctx, base);
	// 826D33DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D33E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D33E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D33E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D33F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D33F0 size=100
    let mut pc: u32 = 0x826D33F0;
    'dispatch: loop {
        match pc {
            0x826D33F0 => {
    //   block [0x826D33F0..0x826D3454)
	// 826D33F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D33F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D33F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D33FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3404: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3410: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826D3414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D341C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3424: 386AB5C4  addi r3, r10, -0x4a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19004;
	// 826D3428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D342C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3430: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D343C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3440: 4BD939E1  bl 0x82466e20
	ctx.lr = 0x826D3444;
	sub_82466E20(ctx, base);
	// 826D3444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3458 size=112
    let mut pc: u32 = 0x826D3458;
    'dispatch: loop {
        match pc {
            0x826D3458 => {
    //   block [0x826D3458..0x826D34C8)
	// 826D3458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3468: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D346C: 38AAB5C4  addi r5, r10, -0x4a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -19004;
	// 826D3470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3474: 390B0210  addi r8, r11, 0x210
	ctx.r[8].s64 = ctx.r[11].s64 + 528;
	// 826D3478: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D347C: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826D3480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D348C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3490: 386AB5F4  addi r3, r10, -0x4a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -18956;
	// 826D3494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D349C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D34A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D34A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D34A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D34AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D34B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D34B4: 4BD9396D  bl 0x82466e20
	ctx.lr = 0x826D34B8;
	sub_82466E20(ctx, base);
	// 826D34B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D34BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D34C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D34C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D34C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D34C8 size=108
    let mut pc: u32 = 0x826D34C8;
    'dispatch: loop {
        match pc {
            0x826D34C8 => {
    //   block [0x826D34C8..0x826D3534)
	// 826D34C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D34CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D34D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D34D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D34D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D34DC: 38EB0270  addi r7, r11, 0x270
	ctx.r[7].s64 = ctx.r[11].s64 + 624;
	// 826D34E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D34E4: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826D34E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D34EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D34F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D34F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D34F8: 386AB624  addi r3, r10, -0x49dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18908;
	// 826D34FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D350C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D351C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3520: 4BD93901  bl 0x82466e20
	ctx.lr = 0x826D3524;
	sub_82466E20(ctx, base);
	// 826D3524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D352C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3538 size=108
    let mut pc: u32 = 0x826D3538;
    'dispatch: loop {
        match pc {
            0x826D3538 => {
    //   block [0x826D3538..0x826D35A4)
	// 826D3538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3544: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D354C: 38EB02A0  addi r7, r11, 0x2a0
	ctx.r[7].s64 = ctx.r[11].s64 + 672;
	// 826D3550: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D3554: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826D3558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D355C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3568: 386AB654  addi r3, r10, -0x49ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18860;
	// 826D356C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D357C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D358C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3590: 4BD93891  bl 0x82466e20
	ctx.lr = 0x826D3594;
	sub_82466E20(ctx, base);
	// 826D3594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D359C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D35A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D35A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D35A8 size=108
    let mut pc: u32 = 0x826D35A8;
    'dispatch: loop {
        match pc {
            0x826D35A8 => {
    //   block [0x826D35A8..0x826D3614)
	// 826D35A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D35AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D35B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D35B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D35B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D35BC: 38EB0300  addi r7, r11, 0x300
	ctx.r[7].s64 = ctx.r[11].s64 + 768;
	// 826D35C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D35C4: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826D35C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D35CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D35D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D35D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D35D8: 386AB684  addi r3, r10, -0x497c
	ctx.r[3].s64 = ctx.r[10].s64 + -18812;
	// 826D35DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D35E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D35E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D35E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D35EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D35F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D35F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D35F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D35FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3600: 4BD93821  bl 0x82466e20
	ctx.lr = 0x826D3604;
	sub_82466E20(ctx, base);
	// 826D3604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3618 size=112
    let mut pc: u32 = 0x826D3618;
    'dispatch: loop {
        match pc {
            0x826D3618 => {
    //   block [0x826D3618..0x826D3688)
	// 826D3618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3624: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D3628: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826D362C: 38EA0360  addi r7, r10, 0x360
	ctx.r[7].s64 = ctx.r[10].s64 + 864;
	// 826D3630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3638: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826D363C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3644: 396B4158  addi r11, r11, 0x4158
	ctx.r[11].s64 = ctx.r[11].s64 + 16728;
	// 826D3648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D364C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3654: 386AB6B4  addi r3, r10, -0x494c
	ctx.r[3].s64 = ctx.r[10].s64 + -18764;
	// 826D3658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D365C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D3660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D3668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D366C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3674: 4BD937AD  bl 0x82466e20
	ctx.lr = 0x826D3678;
	sub_82466E20(ctx, base);
	// 826D3678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D367C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3688 size=96
    let mut pc: u32 = 0x826D3688;
    'dispatch: loop {
        match pc {
            0x826D3688 => {
    //   block [0x826D3688..0x826D36E8)
	// 826D3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3694: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D369C: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826D36A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D36A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D36A8: 386AB6E4  addi r3, r10, -0x491c
	ctx.r[3].s64 = ctx.r[10].s64 + -18716;
	// 826D36AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D36B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D36B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D36B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D36BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D36C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D36C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D36C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D36CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D36D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D36D4: 4BD9374D  bl 0x82466e20
	ctx.lr = 0x826D36D8;
	sub_82466E20(ctx, base);
	// 826D36D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D36DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D36E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D36E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D36E8 size=112
    let mut pc: u32 = 0x826D36E8;
    'dispatch: loop {
        match pc {
            0x826D36E8 => {
    //   block [0x826D36E8..0x826D3758)
	// 826D36E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D36EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D36F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D36F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D36F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D36FC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3704: 390B0480  addi r8, r11, 0x480
	ctx.r[8].s64 = ctx.r[11].s64 + 1152;
	// 826D3708: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D370C: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826D3710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3720: 386AB714  addi r3, r10, -0x48ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18668;
	// 826D3724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D372C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3744: 4BD936DD  bl 0x82466e20
	ctx.lr = 0x826D3748;
	sub_82466E20(ctx, base);
	// 826D3748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D374C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3758 size=24
    let mut pc: u32 = 0x826D3758;
    'dispatch: loop {
        match pc {
            0x826D3758 => {
    //   block [0x826D3758..0x826D3770)
	// 826D3758: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D375C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3760: 394A9278  addi r10, r10, -0x6d88
	ctx.r[10].s64 = ctx.r[10].s64 + -28040;
	// 826D3764: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 826D3768: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826D376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3770 size=116
    let mut pc: u32 = 0x826D3770;
    'dispatch: loop {
        match pc {
            0x826D3770 => {
    //   block [0x826D3770..0x826D37E4)
	// 826D3770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D377C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3780: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3784: 390B9278  addi r8, r11, -0x6d88
	ctx.r[8].s64 = ctx.r[11].s64 + -28040;
	// 826D3788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D378C: 392A41D4  addi r9, r10, 0x41d4
	ctx.r[9].s64 = ctx.r[10].s64 + 16852;
	// 826D3790: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3794: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D3798: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D379C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D37A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D37A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D37A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D37AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D37B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D37B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D37B8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826D37BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D37C0: 386BB744  addi r3, r11, -0x48bc
	ctx.r[3].s64 = ctx.r[11].s64 + -18620;
	// 826D37C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D37C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D37CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D37D0: 4BD93651  bl 0x82466e20
	ctx.lr = 0x826D37D4;
	sub_82466E20(ctx, base);
	// 826D37D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D37D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D37DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D37E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D37E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D37E8 size=112
    let mut pc: u32 = 0x826D37E8;
    'dispatch: loop {
        match pc {
            0x826D37E8 => {
    //   block [0x826D37E8..0x826D3858)
	// 826D37E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D37EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D37F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D37F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D37F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D37FC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3804: 390B04E0  addi r8, r11, 0x4e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1248;
	// 826D3808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D380C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 826D3810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D381C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3820: 386AB774  addi r3, r10, -0x488c
	ctx.r[3].s64 = ctx.r[10].s64 + -18572;
	// 826D3824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D382C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D383C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3844: 4BD935DD  bl 0x82466e20
	ctx.lr = 0x826D3848;
	sub_82466E20(ctx, base);
	// 826D3848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D384C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3858 size=112
    let mut pc: u32 = 0x826D3858;
    'dispatch: loop {
        match pc {
            0x826D3858 => {
    //   block [0x826D3858..0x826D38C8)
	// 826D3858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3868: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D386C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3874: 390B0528  addi r8, r11, 0x528
	ctx.r[8].s64 = ctx.r[11].s64 + 1320;
	// 826D3878: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D387C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826D3880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3884: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D388C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3890: 386AB7A4  addi r3, r10, -0x485c
	ctx.r[3].s64 = ctx.r[10].s64 + -18524;
	// 826D3894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D389C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D38A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D38A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D38A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D38AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D38B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D38B4: 4BD9356D  bl 0x82466e20
	ctx.lr = 0x826D38B8;
	sub_82466E20(ctx, base);
	// 826D38B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D38BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D38C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D38C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D38C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D38C8 size=24
    let mut pc: u32 = 0x826D38C8;
    'dispatch: loop {
        match pc {
            0x826D38C8 => {
    //   block [0x826D38C8..0x826D38E0)
	// 826D38C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D38CC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D38D0: 394A9350  addi r10, r10, -0x6cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -27824;
	// 826D38D4: 816B0570  lwz r11, 0x570(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1392 as u32) ) } as u64;
	// 826D38D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D38DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D38E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D38E0 size=116
    let mut pc: u32 = 0x826D38E0;
    'dispatch: loop {
        match pc {
            0x826D38E0 => {
    //   block [0x826D38E0..0x826D3954)
	// 826D38E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D38E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D38E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D38EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D38F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D38F4: 390B9350  addi r8, r11, -0x6cb0
	ctx.r[8].s64 = ctx.r[11].s64 + -27824;
	// 826D38F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D38FC: 392A4200  addi r9, r10, 0x4200
	ctx.r[9].s64 = ctx.r[10].s64 + 16896;
	// 826D3900: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3904: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826D3908: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D390C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3914: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D391C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3924: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D3928: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826D392C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3930: 386BB7D4  addi r3, r11, -0x482c
	ctx.r[3].s64 = ctx.r[11].s64 + -18476;
	// 826D3934: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D393C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3940: 4BD934E1  bl 0x82466e20
	ctx.lr = 0x826D3944;
	sub_82466E20(ctx, base);
	// 826D3944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D394C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3958 size=108
    let mut pc: u32 = 0x826D3958;
    'dispatch: loop {
        match pc {
            0x826D3958 => {
    //   block [0x826D3958..0x826D39C4)
	// 826D3958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3964: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D396C: 38EB0578  addi r7, r11, 0x578
	ctx.r[7].s64 = ctx.r[11].s64 + 1400;
	// 826D3970: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D3974: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826D3978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D397C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3988: 386AB804  addi r3, r10, -0x47fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18428;
	// 826D398C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D399C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D39A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D39A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D39A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D39AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D39B0: 4BD93471  bl 0x82466e20
	ctx.lr = 0x826D39B4;
	sub_82466E20(ctx, base);
	// 826D39B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D39B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D39BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D39C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D39C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D39C8 size=108
    let mut pc: u32 = 0x826D39C8;
    'dispatch: loop {
        match pc {
            0x826D39C8 => {
    //   block [0x826D39C8..0x826D3A34)
	// 826D39C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D39CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D39D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D39D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D39D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D39DC: 38EB05D8  addi r7, r11, 0x5d8
	ctx.r[7].s64 = ctx.r[11].s64 + 1496;
	// 826D39E0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D39E4: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826D39E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D39EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D39F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D39F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D39F8: 386AB834  addi r3, r10, -0x47cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18380;
	// 826D39FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3A20: 4BD93401  bl 0x82466e20
	ctx.lr = 0x826D3A24;
	sub_82466E20(ctx, base);
	// 826D3A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3A38 size=112
    let mut pc: u32 = 0x826D3A38;
    'dispatch: loop {
        match pc {
            0x826D3A38 => {
    //   block [0x826D3A38..0x826D3AA8)
	// 826D3A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3A44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3A4C: 392B4234  addi r9, r11, 0x4234
	ctx.r[9].s64 = ctx.r[11].s64 + 16948;
	// 826D3A50: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826D3A54: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D3A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3A5C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826D3A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3A64: 396B0680  addi r11, r11, 0x680
	ctx.r[11].s64 = ctx.r[11].s64 + 1664;
	// 826D3A68: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D3A6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3A70: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D3A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3A78: 386AB864  addi r3, r10, -0x479c
	ctx.r[3].s64 = ctx.r[10].s64 + -18332;
	// 826D3A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3A80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D3A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3A88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D3A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3A94: 4BD9338D  bl 0x82466e20
	ctx.lr = 0x826D3A98;
	sub_82466E20(ctx, base);
	// 826D3A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3AA8 size=112
    let mut pc: u32 = 0x826D3AA8;
    'dispatch: loop {
        match pc {
            0x826D3AA8 => {
    //   block [0x826D3AA8..0x826D3B18)
	// 826D3AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3AB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3AB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3ABC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3AC4: 390B07D0  addi r8, r11, 0x7d0
	ctx.r[8].s64 = ctx.r[11].s64 + 2000;
	// 826D3AC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D3ACC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826D3AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3AE0: 386AB894  addi r3, r10, -0x476c
	ctx.r[3].s64 = ctx.r[10].s64 + -18284;
	// 826D3AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3B04: 4BD9331D  bl 0x82466e20
	ctx.lr = 0x826D3B08;
	sub_82466E20(ctx, base);
	// 826D3B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3B18 size=112
    let mut pc: u32 = 0x826D3B18;
    'dispatch: loop {
        match pc {
            0x826D3B18 => {
    //   block [0x826D3B18..0x826D3B88)
	// 826D3B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3B24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3B2C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3B34: 390B0878  addi r8, r11, 0x878
	ctx.r[8].s64 = ctx.r[11].s64 + 2168;
	// 826D3B38: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D3B3C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826D3B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3B50: 386AB8C4  addi r3, r10, -0x473c
	ctx.r[3].s64 = ctx.r[10].s64 + -18236;
	// 826D3B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3B74: 4BD932AD  bl 0x82466e20
	ctx.lr = 0x826D3B78;
	sub_82466E20(ctx, base);
	// 826D3B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3B88 size=112
    let mut pc: u32 = 0x826D3B88;
    'dispatch: loop {
        match pc {
            0x826D3B88 => {
    //   block [0x826D3B88..0x826D3BF8)
	// 826D3B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3B94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3B9C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3BA4: 390B0908  addi r8, r11, 0x908
	ctx.r[8].s64 = ctx.r[11].s64 + 2312;
	// 826D3BA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D3BAC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826D3BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3BC0: 386AB8F4  addi r3, r10, -0x470c
	ctx.r[3].s64 = ctx.r[10].s64 + -18188;
	// 826D3BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3BE4: 4BD9323D  bl 0x82466e20
	ctx.lr = 0x826D3BE8;
	sub_82466E20(ctx, base);
	// 826D3BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3BF8 size=108
    let mut pc: u32 = 0x826D3BF8;
    'dispatch: loop {
        match pc {
            0x826D3BF8 => {
    //   block [0x826D3BF8..0x826D3C64)
	// 826D3BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3C04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3C0C: 38EB0980  addi r7, r11, 0x980
	ctx.r[7].s64 = ctx.r[11].s64 + 2432;
	// 826D3C10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D3C14: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826D3C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3C1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3C20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3C28: 386AB924  addi r3, r10, -0x46dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18140;
	// 826D3C2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3C50: 4BD931D1  bl 0x82466e20
	ctx.lr = 0x826D3C54;
	sub_82466E20(ctx, base);
	// 826D3C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3C68 size=112
    let mut pc: u32 = 0x826D3C68;
    'dispatch: loop {
        match pc {
            0x826D3C68 => {
    //   block [0x826D3C68..0x826D3CD8)
	// 826D3C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3C74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3C78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3C7C: 392A4290  addi r9, r10, 0x4290
	ctx.r[9].s64 = ctx.r[10].s64 + 17040;
	// 826D3C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3C84: 390B0A2C  addi r8, r11, 0xa2c
	ctx.r[8].s64 = ctx.r[11].s64 + 2604;
	// 826D3C88: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D3C8C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826D3C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3C94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3CA0: 386AB954  addi r3, r10, -0x46ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18092;
	// 826D3CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3CA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3CC4: 4BD9315D  bl 0x82466e20
	ctx.lr = 0x826D3CC8;
	sub_82466E20(ctx, base);
	// 826D3CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3CD8 size=100
    let mut pc: u32 = 0x826D3CD8;
    'dispatch: loop {
        match pc {
            0x826D3CD8 => {
    //   block [0x826D3CD8..0x826D3D3C)
	// 826D3CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3CE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3CEC: 38AAC164  addi r5, r10, -0x3e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -16028;
	// 826D3CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3CF8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826D3CFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3D0C: 386AB984  addi r3, r10, -0x467c
	ctx.r[3].s64 = ctx.r[10].s64 + -18044;
	// 826D3D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3D28: 4BD930F9  bl 0x82466e20
	ctx.lr = 0x826D3D2C;
	sub_82466E20(ctx, base);
	// 826D3D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3D40 size=24
    let mut pc: u32 = 0x826D3D40;
    'dispatch: loop {
        match pc {
            0x826D3D40 => {
    //   block [0x826D3D40..0x826D3D58)
	// 826D3D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3D44: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3D48: 394A9470  addi r10, r10, -0x6b90
	ctx.r[10].s64 = ctx.r[10].s64 + -27536;
	// 826D3D4C: 816B0A60  lwz r11, 0xa60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2656 as u32) ) } as u64;
	// 826D3D50: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3D58 size=108
    let mut pc: u32 = 0x826D3D58;
    'dispatch: loop {
        match pc {
            0x826D3D58 => {
    //   block [0x826D3D58..0x826D3DC4)
	// 826D3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3D64: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3D6C: 38EB9470  addi r7, r11, -0x6b90
	ctx.r[7].s64 = ctx.r[11].s64 + -27536;
	// 826D3D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D3D74: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826D3D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3D7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3D88: 386AB9B4  addi r3, r10, -0x464c
	ctx.r[3].s64 = ctx.r[10].s64 + -17996;
	// 826D3D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3DB0: 4BD93071  bl 0x82466e20
	ctx.lr = 0x826D3DB4;
	sub_82466E20(ctx, base);
	// 826D3DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3DC8 size=112
    let mut pc: u32 = 0x826D3DC8;
    'dispatch: loop {
        match pc {
            0x826D3DC8 => {
    //   block [0x826D3DC8..0x826D3E38)
	// 826D3DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3DD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3DD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3DDC: 392A4310  addi r9, r10, 0x4310
	ctx.r[9].s64 = ctx.r[10].s64 + 17168;
	// 826D3DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3DE4: 390B0A68  addi r8, r11, 0xa68
	ctx.r[8].s64 = ctx.r[11].s64 + 2664;
	// 826D3DE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D3DEC: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826D3DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3DF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3E00: 386AB9E4  addi r3, r10, -0x461c
	ctx.r[3].s64 = ctx.r[10].s64 + -17948;
	// 826D3E04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3E08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D3E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3E24: 4BD92FFD  bl 0x82466e20
	ctx.lr = 0x826D3E28;
	sub_82466E20(ctx, base);
	// 826D3E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3E38 size=112
    let mut pc: u32 = 0x826D3E38;
    'dispatch: loop {
        match pc {
            0x826D3E38 => {
    //   block [0x826D3E38..0x826D3EA8)
	// 826D3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3E48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3E4C: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3E54: 390B0AB0  addi r8, r11, 0xab0
	ctx.r[8].s64 = ctx.r[11].s64 + 2736;
	// 826D3E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D3E5C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826D3E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3E70: 386ABA14  addi r3, r10, -0x45ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17900;
	// 826D3E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3E94: 4BD92F8D  bl 0x82466e20
	ctx.lr = 0x826D3E98;
	sub_82466E20(ctx, base);
	// 826D3E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3EA8 size=116
    let mut pc: u32 = 0x826D3EA8;
    'dispatch: loop {
        match pc {
            0x826D3EA8 => {
    //   block [0x826D3EA8..0x826D3F1C)
	// 826D3EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3EB4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D3EB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D3EBC: 390A0AE0  addi r8, r10, 0xae0
	ctx.r[8].s64 = ctx.r[10].s64 + 2784;
	// 826D3EC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3EC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3EC8: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D3ECC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3ED0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3EDC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826D3EE0: 396B4338  addi r11, r11, 0x4338
	ctx.r[11].s64 = ctx.r[11].s64 + 17208;
	// 826D3EE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3EE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3EEC: 386ABA44  addi r3, r10, -0x45bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17852;
	// 826D3EF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D3EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3EF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D3EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3F08: 4BD92F19  bl 0x82466e20
	ctx.lr = 0x826D3F0C;
	sub_82466E20(ctx, base);
	// 826D3F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3F20 size=100
    let mut pc: u32 = 0x826D3F20;
    'dispatch: loop {
        match pc {
            0x826D3F20 => {
    //   block [0x826D3F20..0x826D3F84)
	// 826D3F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3F34: 38AABA44  addi r5, r10, -0x45bc
	ctx.r[5].s64 = ctx.r[10].s64 + -17852;
	// 826D3F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3F40: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826D3F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3F54: 386ABA74  addi r3, r10, -0x458c
	ctx.r[3].s64 = ctx.r[10].s64 + -17804;
	// 826D3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3F5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3F60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3F68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3F70: 4BD92EB1  bl 0x82466e20
	ctx.lr = 0x826D3F74;
	sub_82466E20(ctx, base);
	// 826D3F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3F88 size=24
    let mut pc: u32 = 0x826D3F88;
    'dispatch: loop {
        match pc {
            0x826D3F88 => {
    //   block [0x826D3F88..0x826D3FA0)
	// 826D3F88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3F8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3F90: 394A94B8  addi r10, r10, -0x6b48
	ctx.r[10].s64 = ctx.r[10].s64 + -27464;
	// 826D3F94: 816B0A64  lwz r11, 0xa64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2660 as u32) ) } as u64;
	// 826D3F98: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D3F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3FA0 size=116
    let mut pc: u32 = 0x826D3FA0;
    'dispatch: loop {
        match pc {
            0x826D3FA0 => {
    //   block [0x826D3FA0..0x826D4014)
	// 826D3FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3FAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3FB0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3FB4: 392B4374  addi r9, r11, 0x4374
	ctx.r[9].s64 = ctx.r[11].s64 + 17268;
	// 826D3FB8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3FC0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D3FC4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D3FC8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3FCC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826D3FD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3FD4: 396B94B8  addi r11, r11, -0x6b48
	ctx.r[11].s64 = ctx.r[11].s64 + -27464;
	// 826D3FD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D3FDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3FE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D3FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3FE8: 386ABAA4  addi r3, r10, -0x455c
	ctx.r[3].s64 = ctx.r[10].s64 + -17756;
	// 826D3FEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3FF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D3FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3FF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D3FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4000: 4BD92E21  bl 0x82466e20
	ctx.lr = 0x826D4004;
	sub_82466E20(ctx, base);
	// 826D4004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D400C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4018 size=116
    let mut pc: u32 = 0x826D4018;
    'dispatch: loop {
        match pc {
            0x826D4018 => {
    //   block [0x826D4018..0x826D408C)
	// 826D4018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D401C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4024: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4028: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D402C: 392B43D0  addi r9, r11, 0x43d0
	ctx.r[9].s64 = ctx.r[11].s64 + 17360;
	// 826D4030: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D4034: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4038: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826D403C: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826D4040: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4044: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826D4048: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D404C: 396B0B90  addi r11, r11, 0xb90
	ctx.r[11].s64 = ctx.r[11].s64 + 2960;
	// 826D4050: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D4054: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4058: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D405C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4060: 386ABAD4  addi r3, r10, -0x452c
	ctx.r[3].s64 = ctx.r[10].s64 + -17708;
	// 826D4064: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4068: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D406C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4070: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D4074: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4078: 4BD92DA9  bl 0x82466e20
	ctx.lr = 0x826D407C;
	sub_82466E20(ctx, base);
	// 826D407C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4090 size=108
    let mut pc: u32 = 0x826D4090;
    'dispatch: loop {
        match pc {
            0x826D4090 => {
    //   block [0x826D4090..0x826D40FC)
	// 826D4090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D409C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D40A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D40A4: 38EB0CE0  addi r7, r11, 0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + 3296;
	// 826D40A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D40AC: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826D40B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D40B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D40B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D40BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D40C0: 386ABB04  addi r3, r10, -0x44fc
	ctx.r[3].s64 = ctx.r[10].s64 + -17660;
	// 826D40C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D40C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D40CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D40D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D40D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D40D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D40DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D40E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D40E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D40E8: 4BD92D39  bl 0x82466e20
	ctx.lr = 0x826D40EC;
	sub_82466E20(ctx, base);
	// 826D40EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D40F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D40F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D40F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4100 size=24
    let mut pc: u32 = 0x826D4100;
    'dispatch: loop {
        match pc {
            0x826D4100 => {
    //   block [0x826D4100..0x826D4118)
	// 826D4100: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4104: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4108: 394A9560  addi r10, r10, -0x6aa0
	ctx.r[10].s64 = ctx.r[10].s64 + -27296;
	// 826D410C: 816B0B8C  lwz r11, 0xb8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2956 as u32) ) } as u64;
	// 826D4110: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D4114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4118 size=116
    let mut pc: u32 = 0x826D4118;
    'dispatch: loop {
        match pc {
            0x826D4118 => {
    //   block [0x826D4118..0x826D418C)
	// 826D4118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4124: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4128: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D412C: 392B4468  addi r9, r11, 0x4468
	ctx.r[9].s64 = ctx.r[11].s64 + 17512;
	// 826D4130: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D4134: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4138: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826D413C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 826D4140: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4144: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826D4148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D414C: 396B9560  addi r11, r11, -0x6aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -27296;
	// 826D4150: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D4154: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4158: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D415C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4160: 386ABB34  addi r3, r10, -0x44cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17612;
	// 826D4164: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D4168: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D416C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4170: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D4174: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D4178: 4BD92CA9  bl 0x82466e20
	ctx.lr = 0x826D417C;
	sub_82466E20(ctx, base);
	// 826D417C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4190 size=112
    let mut pc: u32 = 0x826D4190;
    'dispatch: loop {
        match pc {
            0x826D4190 => {
    //   block [0x826D4190..0x826D4200)
	// 826D4190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D419C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D41A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D41A4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D41A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D41AC: 390B0D44  addi r8, r11, 0xd44
	ctx.r[8].s64 = ctx.r[11].s64 + 3396;
	// 826D41B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D41B4: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826D41B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D41BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D41C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D41C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D41C8: 386ABB64  addi r3, r10, -0x449c
	ctx.r[3].s64 = ctx.r[10].s64 + -17564;
	// 826D41CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D41D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D41D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D41D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D41DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D41E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D41E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D41E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D41EC: 4BD92C35  bl 0x82466e20
	ctx.lr = 0x826D41F0;
	sub_82466E20(ctx, base);
	// 826D41F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D41F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D41F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D41FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4200 size=24
    let mut pc: u32 = 0x826D4200;
    'dispatch: loop {
        match pc {
            0x826D4200 => {
    //   block [0x826D4200..0x826D4218)
	// 826D4200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4204: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4208: 394A9788  addi r10, r10, -0x6878
	ctx.r[10].s64 = ctx.r[10].s64 + -26744;
	// 826D420C: 816B0D74  lwz r11, 0xd74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3444 as u32) ) } as u64;
	// 826D4210: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D4214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4218 size=116
    let mut pc: u32 = 0x826D4218;
    'dispatch: loop {
        match pc {
            0x826D4218 => {
    //   block [0x826D4218..0x826D428C)
	// 826D4218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4224: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4228: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D422C: 390B9788  addi r8, r11, -0x6878
	ctx.r[8].s64 = ctx.r[11].s64 + -26744;
	// 826D4230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4234: 392A450C  addi r9, r10, 0x450c
	ctx.r[9].s64 = ctx.r[10].s64 + 17676;
	// 826D4238: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D423C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D4240: 38AABAD4  addi r5, r10, -0x452c
	ctx.r[5].s64 = ctx.r[10].s64 + -17708;
	// 826D4244: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D424C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D425C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D4260: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826D4264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4268: 386BBB94  addi r3, r11, -0x446c
	ctx.r[3].s64 = ctx.r[11].s64 + -17516;
	// 826D426C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4278: 4BD92BA9  bl 0x82466e20
	ctx.lr = 0x826D427C;
	sub_82466E20(ctx, base);
	// 826D427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4290 size=112
    let mut pc: u32 = 0x826D4290;
    'dispatch: loop {
        match pc {
            0x826D4290 => {
    //   block [0x826D4290..0x826D4300)
	// 826D4290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D429C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D42A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D42A4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D42A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D42AC: 390B0D78  addi r8, r11, 0xd78
	ctx.r[8].s64 = ctx.r[11].s64 + 3448;
	// 826D42B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D42B4: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826D42B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D42BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D42C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D42C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D42C8: 386ABBC4  addi r3, r10, -0x443c
	ctx.r[3].s64 = ctx.r[10].s64 + -17468;
	// 826D42CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D42D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D42D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D42D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D42DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D42E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D42E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D42E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D42EC: 4BD92B35  bl 0x82466e20
	ctx.lr = 0x826D42F0;
	sub_82466E20(ctx, base);
	// 826D42F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D42F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D42F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D42FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4300 size=100
    let mut pc: u32 = 0x826D4300;
    'dispatch: loop {
        match pc {
            0x826D4300 => {
    //   block [0x826D4300..0x826D4364)
	// 826D4300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D430C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4314: 38AAC164  addi r5, r10, -0x3e9c
	ctx.r[5].s64 = ctx.r[10].s64 + -16028;
	// 826D4318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D431C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4320: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826D4324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D432C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4334: 386ABBF4  addi r3, r10, -0x440c
	ctx.r[3].s64 = ctx.r[10].s64 + -17420;
	// 826D4338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D433C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D4344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D434C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4350: 4BD92AD1  bl 0x82466e20
	ctx.lr = 0x826D4354;
	sub_82466E20(ctx, base);
	// 826D4354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4368 size=112
    let mut pc: u32 = 0x826D4368;
    'dispatch: loop {
        match pc {
            0x826D4368 => {
    //   block [0x826D4368..0x826D43D8)
	// 826D4368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D436C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4374: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4378: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826D437C: 38EA0D90  addi r7, r10, 0xd90
	ctx.r[7].s64 = ctx.r[10].s64 + 3472;
	// 826D4380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4388: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826D438C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4390: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4394: 396B4520  addi r11, r11, 0x4520
	ctx.r[11].s64 = ctx.r[11].s64 + 17696;
	// 826D4398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D439C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D43A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D43A4: 386ABC24  addi r3, r10, -0x43dc
	ctx.r[3].s64 = ctx.r[10].s64 + -17372;
	// 826D43A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D43AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D43B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D43B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D43B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D43BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D43C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D43C4: 4BD92A5D  bl 0x82466e20
	ctx.lr = 0x826D43C8;
	sub_82466E20(ctx, base);
	// 826D43C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D43CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D43D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D43D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D43D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D43D8 size=112
    let mut pc: u32 = 0x826D43D8;
    'dispatch: loop {
        match pc {
            0x826D43D8 => {
    //   block [0x826D43D8..0x826D4448)
	// 826D43D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D43DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D43E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D43E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D43E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D43EC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D43F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D43F4: 390B0EC8  addi r8, r11, 0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + 3784;
	// 826D43F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D43FC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826D4400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D440C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4410: 386ABC54  addi r3, r10, -0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17324;
	// 826D4414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D441C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D442C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4434: 4BD929ED  bl 0x82466e20
	ctx.lr = 0x826D4438;
	sub_82466E20(ctx, base);
	// 826D4438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D443C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4448 size=108
    let mut pc: u32 = 0x826D4448;
    'dispatch: loop {
        match pc {
            0x826D4448 => {
    //   block [0x826D4448..0x826D44B4)
	// 826D4448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4454: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D445C: 38EB0EF8  addi r7, r11, 0xef8
	ctx.r[7].s64 = ctx.r[11].s64 + 3832;
	// 826D4460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D4464: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826D4468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D446C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4478: 386ABC84  addi r3, r10, -0x437c
	ctx.r[3].s64 = ctx.r[10].s64 + -17276;
	// 826D447C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D448C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D449C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D44A0: 4BD92981  bl 0x82466e20
	ctx.lr = 0x826D44A4;
	sub_82466E20(ctx, base);
	// 826D44A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D44A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D44AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D44B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D44B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D44B8 size=112
    let mut pc: u32 = 0x826D44B8;
    'dispatch: loop {
        match pc {
            0x826D44B8 => {
    //   block [0x826D44B8..0x826D4528)
	// 826D44B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D44BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D44C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D44C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D44C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D44CC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D44D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D44D4: 390B0F28  addi r8, r11, 0xf28
	ctx.r[8].s64 = ctx.r[11].s64 + 3880;
	// 826D44D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D44DC: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826D44E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D44E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D44E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D44EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D44F0: 386ABCB4  addi r3, r10, -0x434c
	ctx.r[3].s64 = ctx.r[10].s64 + -17228;
	// 826D44F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D44F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D44FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D450C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4514: 4BD9290D  bl 0x82466e20
	ctx.lr = 0x826D4518;
	sub_82466E20(ctx, base);
	// 826D4518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D451C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4528 size=112
    let mut pc: u32 = 0x826D4528;
    'dispatch: loop {
        match pc {
            0x826D4528 => {
    //   block [0x826D4528..0x826D4598)
	// 826D4528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4534: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4538: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D453C: 38EA0F40  addi r7, r10, 0xf40
	ctx.r[7].s64 = ctx.r[10].s64 + 3904;
	// 826D4540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4544: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4548: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826D454C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4550: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4554: 396B4574  addi r11, r11, 0x4574
	ctx.r[11].s64 = ctx.r[11].s64 + 17780;
	// 826D4558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D455C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4564: 386ABCE4  addi r3, r10, -0x431c
	ctx.r[3].s64 = ctx.r[10].s64 + -17180;
	// 826D4568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D456C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4574: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4578: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D457C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4580: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4584: 4BD9289D  bl 0x82466e20
	ctx.lr = 0x826D4588;
	sub_82466E20(ctx, base);
	// 826D4588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D458C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4598 size=108
    let mut pc: u32 = 0x826D4598;
    'dispatch: loop {
        match pc {
            0x826D4598 => {
    //   block [0x826D4598..0x826D4604)
	// 826D4598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D45A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D45A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D45A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D45AC: 38EB1018  addi r7, r11, 0x1018
	ctx.r[7].s64 = ctx.r[11].s64 + 4120;
	// 826D45B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D45B4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826D45B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D45BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D45C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D45C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D45C8: 386ABD14  addi r3, r10, -0x42ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17132;
	// 826D45CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D45D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D45D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D45D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D45DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D45E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D45E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D45E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D45EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D45F0: 4BD92831  bl 0x82466e20
	ctx.lr = 0x826D45F4;
	sub_82466E20(ctx, base);
	// 826D45F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D45F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D45FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4608 size=108
    let mut pc: u32 = 0x826D4608;
    'dispatch: loop {
        match pc {
            0x826D4608 => {
    //   block [0x826D4608..0x826D4674)
	// 826D4608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4614: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D461C: 38EB1030  addi r7, r11, 0x1030
	ctx.r[7].s64 = ctx.r[11].s64 + 4144;
	// 826D4620: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826D4624: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826D4628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D462C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4638: 386ABD44  addi r3, r10, -0x42bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17084;
	// 826D463C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D464C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D465C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4660: 4BD927C1  bl 0x82466e20
	ctx.lr = 0x826D4664;
	sub_82466E20(ctx, base);
	// 826D4664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D466C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4678 size=108
    let mut pc: u32 = 0x826D4678;
    'dispatch: loop {
        match pc {
            0x826D4678 => {
    //   block [0x826D4678..0x826D46E4)
	// 826D4678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4684: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D468C: 38EB1138  addi r7, r11, 0x1138
	ctx.r[7].s64 = ctx.r[11].s64 + 4408;
	// 826D4690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D4694: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826D4698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D469C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D46A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D46A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D46A8: 386ABD74  addi r3, r10, -0x428c
	ctx.r[3].s64 = ctx.r[10].s64 + -17036;
	// 826D46AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D46B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D46B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D46B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D46BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D46C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D46C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D46C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D46CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D46D0: 4BD92751  bl 0x82466e20
	ctx.lr = 0x826D46D4;
	sub_82466E20(ctx, base);
	// 826D46D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D46D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D46DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D46E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D46E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D46E8 size=112
    let mut pc: u32 = 0x826D46E8;
    'dispatch: loop {
        match pc {
            0x826D46E8 => {
    //   block [0x826D46E8..0x826D4758)
	// 826D46E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D46EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D46F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D46F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D46F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D46FC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4704: 390B1198  addi r8, r11, 0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + 4504;
	// 826D4708: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826D470C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826D4710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4720: 386ABDA4  addi r3, r10, -0x425c
	ctx.r[3].s64 = ctx.r[10].s64 + -16988;
	// 826D4724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D472C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D473C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4744: 4BD926DD  bl 0x82466e20
	ctx.lr = 0x826D4748;
	sub_82466E20(ctx, base);
	// 826D4748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D474C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4758 size=112
    let mut pc: u32 = 0x826D4758;
    'dispatch: loop {
        match pc {
            0x826D4758 => {
    //   block [0x826D4758..0x826D47C8)
	// 826D4758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4768: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D476C: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4774: 390B12B8  addi r8, r11, 0x12b8
	ctx.r[8].s64 = ctx.r[11].s64 + 4792;
	// 826D4778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D477C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826D4780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4790: 386ABDD4  addi r3, r10, -0x422c
	ctx.r[3].s64 = ctx.r[10].s64 + -16940;
	// 826D4794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D479C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D47A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D47A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D47A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D47AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D47B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D47B4: 4BD9266D  bl 0x82466e20
	ctx.lr = 0x826D47B8;
	sub_82466E20(ctx, base);
	// 826D47B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D47BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D47C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D47C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D47C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D47C8 size=116
    let mut pc: u32 = 0x826D47C8;
    'dispatch: loop {
        match pc {
            0x826D47C8 => {
    //   block [0x826D47C8..0x826D483C)
	// 826D47C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D47CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D47D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D47D4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D47D8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D47DC: 390A12D0  addi r8, r10, 0x12d0
	ctx.r[8].s64 = ctx.r[10].s64 + 4816;
	// 826D47E0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D47E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D47E8: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D47EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D47F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D47F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D47F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D47FC: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826D4800: 396B45A4  addi r11, r11, 0x45a4
	ctx.r[11].s64 = ctx.r[11].s64 + 17828;
	// 826D4804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D480C: 386ABE04  addi r3, r10, -0x41fc
	ctx.r[3].s64 = ctx.r[10].s64 + -16892;
	// 826D4810: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4818: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D481C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4828: 4BD925F9  bl 0x82466e20
	ctx.lr = 0x826D482C;
	sub_82466E20(ctx, base);
	// 826D482C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4840 size=108
    let mut pc: u32 = 0x826D4840;
    'dispatch: loop {
        match pc {
            0x826D4840 => {
    //   block [0x826D4840..0x826D48AC)
	// 826D4840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D484C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D4854: 38EB1330  addi r7, r11, 0x1330
	ctx.r[7].s64 = ctx.r[11].s64 + 4912;
	// 826D4858: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D485C: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826D4860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D486C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4870: 386ABE34  addi r3, r10, -0x41cc
	ctx.r[3].s64 = ctx.r[10].s64 + -16844;
	// 826D4874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D487C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D488C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4898: 4BD92589  bl 0x82466e20
	ctx.lr = 0x826D489C;
	sub_82466E20(ctx, base);
	// 826D489C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D48A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D48A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D48A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D48B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D48B0 size=112
    let mut pc: u32 = 0x826D48B0;
    'dispatch: loop {
        match pc {
            0x826D48B0 => {
    //   block [0x826D48B0..0x826D4920)
	// 826D48B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D48B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D48B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D48BC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D48C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D48C4: 38EA13D8  addi r7, r10, 0x13d8
	ctx.r[7].s64 = ctx.r[10].s64 + 5080;
	// 826D48C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D48CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D48D0: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826D48D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D48D8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D48DC: 396B45B8  addi r11, r11, 0x45b8
	ctx.r[11].s64 = ctx.r[11].s64 + 17848;
	// 826D48E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D48E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D48E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D48EC: 386ABE64  addi r3, r10, -0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + -16796;
	// 826D48F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D48F4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D48F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D48FC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4904: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D490C: 4BD92515  bl 0x82466e20
	ctx.lr = 0x826D4910;
	sub_82466E20(ctx, base);
	// 826D4910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D491C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4920 size=112
    let mut pc: u32 = 0x826D4920;
    'dispatch: loop {
        match pc {
            0x826D4920 => {
    //   block [0x826D4920..0x826D4990)
	// 826D4920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D492C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4930: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4934: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D493C: 390B1450  addi r8, r11, 0x1450
	ctx.r[8].s64 = ctx.r[11].s64 + 5200;
	// 826D4940: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D4944: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826D4948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D494C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4958: 386ABE94  addi r3, r10, -0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + -16748;
	// 826D495C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D496C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D497C: 4BD924A5  bl 0x82466e20
	ctx.lr = 0x826D4980;
	sub_82466E20(ctx, base);
	// 826D4980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D498C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4990 size=112
    let mut pc: u32 = 0x826D4990;
    'dispatch: loop {
        match pc {
            0x826D4990 => {
    //   block [0x826D4990..0x826D4A00)
	// 826D4990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D499C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D49A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D49A4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D49A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D49AC: 390B1498  addi r8, r11, 0x1498
	ctx.r[8].s64 = ctx.r[11].s64 + 5272;
	// 826D49B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826D49B4: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826D49B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D49BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D49C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D49C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D49C8: 386ABEC4  addi r3, r10, -0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + -16700;
	// 826D49CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D49D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D49D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D49D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D49DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D49E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D49E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D49E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D49EC: 4BD92435  bl 0x82466e20
	ctx.lr = 0x826D49F0;
	sub_82466E20(ctx, base);
	// 826D49F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D49F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D49F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D49FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4A00 size=112
    let mut pc: u32 = 0x826D4A00;
    'dispatch: loop {
        match pc {
            0x826D4A00 => {
    //   block [0x826D4A00..0x826D4A70)
	// 826D4A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4A0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4A14: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4A1C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826D4A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4A24: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826D4A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4A38: 386ABEF4  addi r3, r10, -0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + -16652;
	// 826D4A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4A5C: 4BD923C5  bl 0x82466e20
	ctx.lr = 0x826D4A60;
	sub_82466E20(ctx, base);
	// 826D4A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4A70 size=112
    let mut pc: u32 = 0x826D4A70;
    'dispatch: loop {
        match pc {
            0x826D4A70 => {
    //   block [0x826D4A70..0x826D4AE0)
	// 826D4A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4A7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4A80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4A84: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D4A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4A8C: 390B15B8  addi r8, r11, 0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5560;
	// 826D4A90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D4A94: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826D4A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4A9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4AA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4AA8: 386ABF24  addi r3, r10, -0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + -16604;
	// 826D4AAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4ACC: 4BD92355  bl 0x82466e20
	ctx.lr = 0x826D4AD0;
	sub_82466E20(ctx, base);
	// 826D4AD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4AE0 size=112
    let mut pc: u32 = 0x826D4AE0;
    'dispatch: loop {
        match pc {
            0x826D4AE0 => {
    //   block [0x826D4AE0..0x826D4B50)
	// 826D4AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4AEC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4AF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D4AF4: 38EA15E8  addi r7, r10, 0x15e8
	ctx.r[7].s64 = ctx.r[10].s64 + 5608;
	// 826D4AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4AFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4B00: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826D4B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4B08: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4B0C: 396B45D8  addi r11, r11, 0x45d8
	ctx.r[11].s64 = ctx.r[11].s64 + 17880;
	// 826D4B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4B14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4B1C: 386ABF54  addi r3, r10, -0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + -16556;
	// 826D4B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4B24: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4B28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4B2C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4B30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4B34: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4B38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4B3C: 4BD922E5  bl 0x82466e20
	ctx.lr = 0x826D4B40;
	sub_82466E20(ctx, base);
	// 826D4B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D4B50 size=24
    let mut pc: u32 = 0x826D4B50;
    'dispatch: loop {
        match pc {
            0x826D4B50 => {
    //   block [0x826D4B50..0x826D4B68)
	// 826D4B50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4B54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D4B58: 394A9878  addi r10, r10, -0x6788
	ctx.r[10].s64 = ctx.r[10].s64 + -26504;
	// 826D4B5C: 816B1660  lwz r11, 0x1660(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5728 as u32) ) } as u64;
	// 826D4B60: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D4B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4B68 size=116
    let mut pc: u32 = 0x826D4B68;
    'dispatch: loop {
        match pc {
            0x826D4B68 => {
    //   block [0x826D4B68..0x826D4BDC)
	// 826D4B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4B74: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D4B78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D4B7C: 390B9878  addi r8, r11, -0x6788
	ctx.r[8].s64 = ctx.r[11].s64 + -26504;
	// 826D4B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4B84: 392A4618  addi r9, r10, 0x4618
	ctx.r[9].s64 = ctx.r[10].s64 + 17944;
	// 826D4B88: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4B8C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D4B90: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4B94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4B9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4BAC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D4BB0: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826D4BB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4BB8: 386BBF84  addi r3, r11, -0x407c
	ctx.r[3].s64 = ctx.r[11].s64 + -16508;
	// 826D4BBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D4BC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4BC8: 4BD92259  bl 0x82466e20
	ctx.lr = 0x826D4BCC;
	sub_82466E20(ctx, base);
	// 826D4BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4BE0 size=112
    let mut pc: u32 = 0x826D4BE0;
    'dispatch: loop {
        match pc {
            0x826D4BE0 => {
    //   block [0x826D4BE0..0x826D4C50)
	// 826D4BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4BF0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4BF4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4BFC: 390B1664  addi r8, r11, 0x1664
	ctx.r[8].s64 = ctx.r[11].s64 + 5732;
	// 826D4C00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D4C04: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826D4C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4C0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4C18: 386ABFB4  addi r3, r10, -0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + -16460;
	// 826D4C1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4C3C: 4BD921E5  bl 0x82466e20
	ctx.lr = 0x826D4C40;
	sub_82466E20(ctx, base);
	// 826D4C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4C50 size=116
    let mut pc: u32 = 0x826D4C50;
    'dispatch: loop {
        match pc {
            0x826D4C50 => {
    //   block [0x826D4C50..0x826D4CC4)
	// 826D4C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4C5C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4C60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D4C64: 390A1698  addi r8, r10, 0x1698
	ctx.r[8].s64 = ctx.r[10].s64 + 5784;
	// 826D4C68: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4C70: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4C74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4C78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4C84: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826D4C88: 396B462C  addi r11, r11, 0x462c
	ctx.r[11].s64 = ctx.r[11].s64 + 17964;
	// 826D4C8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4C90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4C94: 386ABFE4  addi r3, r10, -0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + -16412;
	// 826D4C98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4CA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4CB0: 4BD92171  bl 0x82466e20
	ctx.lr = 0x826D4CB4;
	sub_82466E20(ctx, base);
	// 826D4CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4CC8 size=112
    let mut pc: u32 = 0x826D4CC8;
    'dispatch: loop {
        match pc {
            0x826D4CC8 => {
    //   block [0x826D4CC8..0x826D4D38)
	// 826D4CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4CD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4CDC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4CE4: 390B1758  addi r8, r11, 0x1758
	ctx.r[8].s64 = ctx.r[11].s64 + 5976;
	// 826D4CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4CEC: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826D4CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4D00: 386AC014  addi r3, r10, -0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + -16364;
	// 826D4D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4D24: 4BD920FD  bl 0x82466e20
	ctx.lr = 0x826D4D28;
	sub_82466E20(ctx, base);
	// 826D4D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4D38 size=112
    let mut pc: u32 = 0x826D4D38;
    'dispatch: loop {
        match pc {
            0x826D4D38 => {
    //   block [0x826D4D38..0x826D4DA8)
	// 826D4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4D44: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4D48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D4D4C: 38EA1770  addi r7, r10, 0x1770
	ctx.r[7].s64 = ctx.r[10].s64 + 6000;
	// 826D4D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4D54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4D58: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826D4D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4D60: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D4D64: 396B4650  addi r11, r11, 0x4650
	ctx.r[11].s64 = ctx.r[11].s64 + 18000;
	// 826D4D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D4D6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4D70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4D74: 386AC044  addi r3, r10, -0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -16316;
	// 826D4D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4D7C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4D84: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4D88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4D8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4D90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D4D94: 4BD9208D  bl 0x82466e20
	ctx.lr = 0x826D4D98;
	sub_82466E20(ctx, base);
	// 826D4D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4DA8 size=116
    let mut pc: u32 = 0x826D4DA8;
    'dispatch: loop {
        match pc {
            0x826D4DA8 => {
    //   block [0x826D4DA8..0x826D4E1C)
	// 826D4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4DB4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D4DB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D4DBC: 390A17A0  addi r8, r10, 0x17a0
	ctx.r[8].s64 = ctx.r[10].s64 + 6048;
	// 826D4DC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D4DC8: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D4DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4DDC: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826D4DE0: 396B465C  addi r11, r11, 0x465c
	ctx.r[11].s64 = ctx.r[11].s64 + 18012;
	// 826D4DE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4DEC: 386AC074  addi r3, r10, -0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16268;
	// 826D4DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D4DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D4DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4E08: 4BD92019  bl 0x82466e20
	ctx.lr = 0x826D4E0C;
	sub_82466E20(ctx, base);
	// 826D4E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4E20 size=112
    let mut pc: u32 = 0x826D4E20;
    'dispatch: loop {
        match pc {
            0x826D4E20 => {
    //   block [0x826D4E20..0x826D4E90)
	// 826D4E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4E30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4E34: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4E3C: 390B1848  addi r8, r11, 0x1848
	ctx.r[8].s64 = ctx.r[11].s64 + 6216;
	// 826D4E40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4E44: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826D4E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4E58: 386AC0A4  addi r3, r10, -0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -16220;
	// 826D4E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4E7C: 4BD91FA5  bl 0x82466e20
	ctx.lr = 0x826D4E80;
	sub_82466E20(ctx, base);
	// 826D4E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4E90 size=112
    let mut pc: u32 = 0x826D4E90;
    'dispatch: loop {
        match pc {
            0x826D4E90 => {
    //   block [0x826D4E90..0x826D4F00)
	// 826D4E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4E9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4EA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4EA4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4EAC: 390B1860  addi r8, r11, 0x1860
	ctx.r[8].s64 = ctx.r[11].s64 + 6240;
	// 826D4EB0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826D4EB4: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826D4EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4EBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4EC8: 386AC0D4  addi r3, r10, -0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -16172;
	// 826D4ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4EEC: 4BD91F35  bl 0x82466e20
	ctx.lr = 0x826D4EF0;
	sub_82466E20(ctx, base);
	// 826D4EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4F00 size=112
    let mut pc: u32 = 0x826D4F00;
    'dispatch: loop {
        match pc {
            0x826D4F00 => {
    //   block [0x826D4F00..0x826D4F70)
	// 826D4F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4F0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4F14: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D4F1C: 390B1998  addi r8, r11, 0x1998
	ctx.r[8].s64 = ctx.r[11].s64 + 6552;
	// 826D4F20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D4F24: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826D4F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4F38: 386AC104  addi r3, r10, -0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + -16124;
	// 826D4F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4F5C: 4BD91EC5  bl 0x82466e20
	ctx.lr = 0x826D4F60;
	sub_82466E20(ctx, base);
	// 826D4F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4F70 size=112
    let mut pc: u32 = 0x826D4F70;
    'dispatch: loop {
        match pc {
            0x826D4F70 => {
    //   block [0x826D4F70..0x826D4FE0)
	// 826D4F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4F7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4F80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4F84: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D4F88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D4F8C: 390B19B0  addi r8, r11, 0x19b0
	ctx.r[8].s64 = ctx.r[11].s64 + 6576;
	// 826D4F90: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D4F94: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826D4F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4F9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D4FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D4FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D4FA8: 386AC134  addi r3, r10, -0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -16076;
	// 826D4FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D4FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D4FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D4FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D4FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D4FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D4FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D4FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D4FCC: 4BD91E55  bl 0x82466e20
	ctx.lr = 0x826D4FD0;
	sub_82466E20(ctx, base);
	// 826D4FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D4FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D4FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D4FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D4FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D4FE0 size=116
    let mut pc: u32 = 0x826D4FE0;
    'dispatch: loop {
        match pc {
            0x826D4FE0 => {
    //   block [0x826D4FE0..0x826D5054)
	// 826D4FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D4FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D4FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D4FEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D4FF0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D4FF4: 390B1A40  addi r8, r11, 0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + 6720;
	// 826D4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D4FFC: 392A4694  addi r9, r10, 0x4694
	ctx.r[9].s64 = ctx.r[10].s64 + 18068;
	// 826D5000: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5004: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5008: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D500C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5024: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5028: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826D502C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5030: 386BC164  addi r3, r11, -0x3e9c
	ctx.r[3].s64 = ctx.r[11].s64 + -16028;
	// 826D5034: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5038: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D503C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5040: 4BD91DE1  bl 0x82466e20
	ctx.lr = 0x826D5044;
	sub_82466E20(ctx, base);
	// 826D5044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D504C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5058 size=100
    let mut pc: u32 = 0x826D5058;
    'dispatch: loop {
        match pc {
            0x826D5058 => {
    //   block [0x826D5058..0x826D50BC)
	// 826D5058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D505C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D506C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5078: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826D507C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D508C: 386AC194  addi r3, r10, -0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15980;
	// 826D5090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D509C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D50A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D50A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D50A8: 4BD91D79  bl 0x82466e20
	ctx.lr = 0x826D50AC;
	sub_82466E20(ctx, base);
	// 826D50AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D50B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D50B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D50B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D50C0 size=112
    let mut pc: u32 = 0x826D50C0;
    'dispatch: loop {
        match pc {
            0x826D50C0 => {
    //   block [0x826D50C0..0x826D5130)
	// 826D50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D50C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D50C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D50CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D50D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D50D4: 38AAC194  addi r5, r10, -0x3e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -15980;
	// 826D50D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D50DC: 390B1A70  addi r8, r11, 0x1a70
	ctx.r[8].s64 = ctx.r[11].s64 + 6768;
	// 826D50E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D50E4: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826D50E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D50EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D50F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D50F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D50F8: 386AC1C4  addi r3, r10, -0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15932;
	// 826D50FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D510C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D511C: 4BD91D05  bl 0x82466e20
	ctx.lr = 0x826D5120;
	sub_82466E20(ctx, base);
	// 826D5120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5130 size=112
    let mut pc: u32 = 0x826D5130;
    'dispatch: loop {
        match pc {
            0x826D5130 => {
    //   block [0x826D5130..0x826D51A0)
	// 826D5130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D513C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5140: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5144: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D514C: 390B1A88  addi r8, r11, 0x1a88
	ctx.r[8].s64 = ctx.r[11].s64 + 6792;
	// 826D5150: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D5154: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826D5158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D515C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5168: 386AC1F4  addi r3, r10, -0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15884;
	// 826D516C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D517C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D518C: 4BD91C95  bl 0x82466e20
	ctx.lr = 0x826D5190;
	sub_82466E20(ctx, base);
	// 826D5190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D519C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D51A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D51A0 size=112
    let mut pc: u32 = 0x826D51A0;
    'dispatch: loop {
        match pc {
            0x826D51A0 => {
    //   block [0x826D51A0..0x826D5210)
	// 826D51A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D51A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D51A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D51AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D51B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D51B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D51B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D51BC: 390B1B30  addi r8, r11, 0x1b30
	ctx.r[8].s64 = ctx.r[11].s64 + 6960;
	// 826D51C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D51C4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826D51C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D51CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D51D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D51D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D51D8: 386AC224  addi r3, r10, -0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -15836;
	// 826D51DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D51E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D51E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D51E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D51EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D51F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D51F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D51F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D51FC: 4BD91C25  bl 0x82466e20
	ctx.lr = 0x826D5200;
	sub_82466E20(ctx, base);
	// 826D5200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D520C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5210 size=112
    let mut pc: u32 = 0x826D5210;
    'dispatch: loop {
        match pc {
            0x826D5210 => {
    //   block [0x826D5210..0x826D5280)
	// 826D5210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D521C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5220: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5224: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D522C: 390B1B78  addi r8, r11, 0x1b78
	ctx.r[8].s64 = ctx.r[11].s64 + 7032;
	// 826D5230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D5234: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826D5238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D523C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5248: 386AC254  addi r3, r10, -0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + -15788;
	// 826D524C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D525C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D526C: 4BD91BB5  bl 0x82466e20
	ctx.lr = 0x826D5270;
	sub_82466E20(ctx, base);
	// 826D5270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D527C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5280 size=116
    let mut pc: u32 = 0x826D5280;
    'dispatch: loop {
        match pc {
            0x826D5280 => {
    //   block [0x826D5280..0x826D52F4)
	// 826D5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D528C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D5290: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D5294: 390A1BA8  addi r8, r10, 0x1ba8
	ctx.r[8].s64 = ctx.r[10].s64 + 7080;
	// 826D5298: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D529C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D52A0: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D52A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D52A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D52AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D52B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D52B4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826D52B8: 396B46A8  addi r11, r11, 0x46a8
	ctx.r[11].s64 = ctx.r[11].s64 + 18088;
	// 826D52BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D52C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D52C4: 386AC284  addi r3, r10, -0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15740;
	// 826D52C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D52CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D52D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D52D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D52D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D52DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D52E0: 4BD91B41  bl 0x82466e20
	ctx.lr = 0x826D52E4;
	sub_82466E20(ctx, base);
	// 826D52E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D52E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D52EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D52F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D52F8 size=100
    let mut pc: u32 = 0x826D52F8;
    'dispatch: loop {
        match pc {
            0x826D52F8 => {
    //   block [0x826D52F8..0x826D535C)
	// 826D52F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D52FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D530C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5318: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826D531C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D532C: 386AC2B4  addi r3, r10, -0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15692;
	// 826D5330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5338: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D533C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5340: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D5344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5348: 4BD91AD9  bl 0x82466e20
	ctx.lr = 0x826D534C;
	sub_82466E20(ctx, base);
	// 826D534C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5360 size=108
    let mut pc: u32 = 0x826D5360;
    'dispatch: loop {
        match pc {
            0x826D5360 => {
    //   block [0x826D5360..0x826D53CC)
	// 826D5360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D536C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5374: 38EB1C68  addi r7, r11, 0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + 7272;
	// 826D5378: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D537C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826D5380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5390: 386AC2E4  addi r3, r10, -0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15644;
	// 826D5394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D53A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D53A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D53A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D53AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D53B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D53B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D53B8: 4BD91A69  bl 0x82466e20
	ctx.lr = 0x826D53BC;
	sub_82466E20(ctx, base);
	// 826D53BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D53C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D53C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D53C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D53D0 size=112
    let mut pc: u32 = 0x826D53D0;
    'dispatch: loop {
        match pc {
            0x826D53D0 => {
    //   block [0x826D53D0..0x826D5440)
	// 826D53D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D53D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D53D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D53DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D53E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D53E4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D53E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D53EC: 390B1C98  addi r8, r11, 0x1c98
	ctx.r[8].s64 = ctx.r[11].s64 + 7320;
	// 826D53F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D53F4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826D53F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D53FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5408: 386AC314  addi r3, r10, -0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + -15596;
	// 826D540C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D541C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D542C: 4BD919F5  bl 0x82466e20
	ctx.lr = 0x826D5430;
	sub_82466E20(ctx, base);
	// 826D5430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D543C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5440 size=108
    let mut pc: u32 = 0x826D5440;
    'dispatch: loop {
        match pc {
            0x826D5440 => {
    //   block [0x826D5440..0x826D54AC)
	// 826D5440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D544C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5454: 38EB1CC8  addi r7, r11, 0x1cc8
	ctx.r[7].s64 = ctx.r[11].s64 + 7368;
	// 826D5458: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D545C: 388AA93C  addi r4, r10, -0x56c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22212;
	// 826D5460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D546C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5470: 386AC344  addi r3, r10, -0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15548;
	// 826D5474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D547C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D548C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5498: 4BD91989  bl 0x82466e20
	ctx.lr = 0x826D549C;
	sub_82466E20(ctx, base);
	// 826D549C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D54A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D54A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D54A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D54B0 size=112
    let mut pc: u32 = 0x826D54B0;
    'dispatch: loop {
        match pc {
            0x826D54B0 => {
    //   block [0x826D54B0..0x826D5520)
	// 826D54B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D54B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D54B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D54BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D54C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D54C4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D54C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D54CC: 390B1CF8  addi r8, r11, 0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 7416;
	// 826D54D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D54D4: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826D54D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D54DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D54E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D54E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D54E8: 386AC374  addi r3, r10, -0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15500;
	// 826D54EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D54F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D54F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D54F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D54FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D550C: 4BD91915  bl 0x82466e20
	ctx.lr = 0x826D5510;
	sub_82466E20(ctx, base);
	// 826D5510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D551C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5520 size=108
    let mut pc: u32 = 0x826D5520;
    'dispatch: loop {
        match pc {
            0x826D5520 => {
    //   block [0x826D5520..0x826D558C)
	// 826D5520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D552C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5534: 38EB1D40  addi r7, r11, 0x1d40
	ctx.r[7].s64 = ctx.r[11].s64 + 7488;
	// 826D5538: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D553C: 388AA960  addi r4, r10, -0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22176;
	// 826D5540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D554C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5550: 386AC3A4  addi r3, r10, -0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15452;
	// 826D5554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D555C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D556C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5578: 4BD918A9  bl 0x82466e20
	ctx.lr = 0x826D557C;
	sub_82466E20(ctx, base);
	// 826D557C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5590 size=112
    let mut pc: u32 = 0x826D5590;
    'dispatch: loop {
        match pc {
            0x826D5590 => {
    //   block [0x826D5590..0x826D5600)
	// 826D5590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D559C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D55A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D55A4: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D55A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D55AC: 390B1D70  addi r8, r11, 0x1d70
	ctx.r[8].s64 = ctx.r[11].s64 + 7536;
	// 826D55B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D55B4: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826D55B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D55BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D55C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D55C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D55C8: 386AC3D4  addi r3, r10, -0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15404;
	// 826D55CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D55D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D55D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D55D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D55DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D55E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D55E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D55E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D55EC: 4BD91835  bl 0x82466e20
	ctx.lr = 0x826D55F0;
	sub_82466E20(ctx, base);
	// 826D55F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D55F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D55F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D55FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5600 size=108
    let mut pc: u32 = 0x826D5600;
    'dispatch: loop {
        match pc {
            0x826D5600 => {
    //   block [0x826D5600..0x826D566C)
	// 826D5600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D560C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5610: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5614: 38EB1DB8  addi r7, r11, 0x1db8
	ctx.r[7].s64 = ctx.r[11].s64 + 7608;
	// 826D5618: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D561C: 388AA984  addi r4, r10, -0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + -22140;
	// 826D5620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5628: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D562C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5630: 386AC404  addi r3, r10, -0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15356;
	// 826D5634: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D563C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D564C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5658: 4BD917C9  bl 0x82466e20
	ctx.lr = 0x826D565C;
	sub_82466E20(ctx, base);
	// 826D565C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5670 size=112
    let mut pc: u32 = 0x826D5670;
    'dispatch: loop {
        match pc {
            0x826D5670 => {
    //   block [0x826D5670..0x826D56E0)
	// 826D5670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D567C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5680: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5684: 38AAC2B4  addi r5, r10, -0x3d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -15692;
	// 826D5688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D568C: 390B1DE8  addi r8, r11, 0x1de8
	ctx.r[8].s64 = ctx.r[11].s64 + 7656;
	// 826D5690: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D5694: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826D5698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D569C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D56A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D56A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D56A8: 386AC434  addi r3, r10, -0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -15308;
	// 826D56AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D56B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D56B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D56B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D56BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D56C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D56C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D56C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D56CC: 4BD91755  bl 0x82466e20
	ctx.lr = 0x826D56D0;
	sub_82466E20(ctx, base);
	// 826D56D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D56D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D56D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D56DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D56E0 size=108
    let mut pc: u32 = 0x826D56E0;
    'dispatch: loop {
        match pc {
            0x826D56E0 => {
    //   block [0x826D56E0..0x826D574C)
	// 826D56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D56E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D56E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D56EC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D56F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D56F4: 38EB1E38  addi r7, r11, 0x1e38
	ctx.r[7].s64 = ctx.r[11].s64 + 7736;
	// 826D56F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D56FC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826D5700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5704: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D570C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5710: 386AC464  addi r3, r10, -0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15260;
	// 826D5714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D571C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5738: 4BD916E9  bl 0x82466e20
	ctx.lr = 0x826D573C;
	sub_82466E20(ctx, base);
	// 826D573C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5750 size=24
    let mut pc: u32 = 0x826D5750;
    'dispatch: loop {
        match pc {
            0x826D5750 => {
    //   block [0x826D5750..0x826D5768)
	// 826D5750: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5754: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5758: 394A9980  addi r10, r10, -0x6680
	ctx.r[10].s64 = ctx.r[10].s64 + -26240;
	// 826D575C: 816B1E34  lwz r11, 0x1e34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7732 as u32) ) } as u64;
	// 826D5760: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826D5764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5768 size=112
    let mut pc: u32 = 0x826D5768;
    'dispatch: loop {
        match pc {
            0x826D5768 => {
    //   block [0x826D5768..0x826D57D8)
	// 826D5768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5774: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5778: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D577C: 392A4790  addi r9, r10, 0x4790
	ctx.r[9].s64 = ctx.r[10].s64 + 18320;
	// 826D5780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5784: 390B9980  addi r8, r11, -0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + -26240;
	// 826D5788: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D578C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826D5790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D579C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D57A0: 386AC494  addi r3, r10, -0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15212;
	// 826D57A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D57A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D57AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D57B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D57B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D57B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D57BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D57C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D57C4: 4BD9165D  bl 0x82466e20
	ctx.lr = 0x826D57C8;
	sub_82466E20(ctx, base);
	// 826D57C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D57CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D57D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D57D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D57D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D57D8 size=108
    let mut pc: u32 = 0x826D57D8;
    'dispatch: loop {
        match pc {
            0x826D57D8 => {
    //   block [0x826D57D8..0x826D5844)
	// 826D57D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D57DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D57E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D57E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D57E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D57EC: 38EB1EA0  addi r7, r11, 0x1ea0
	ctx.r[7].s64 = ctx.r[11].s64 + 7840;
	// 826D57F0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D57F4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826D57F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D57FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5808: 386AC4C4  addi r3, r10, -0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15164;
	// 826D580C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D582C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5830: 4BD915F1  bl 0x82466e20
	ctx.lr = 0x826D5834;
	sub_82466E20(ctx, base);
	// 826D5834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D583C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5848 size=108
    let mut pc: u32 = 0x826D5848;
    'dispatch: loop {
        match pc {
            0x826D5848 => {
    //   block [0x826D5848..0x826D58B4)
	// 826D5848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D584C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5854: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D585C: 38EB1F18  addi r7, r11, 0x1f18
	ctx.r[7].s64 = ctx.r[11].s64 + 7960;
	// 826D5860: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D5864: 388AA9B8  addi r4, r10, -0x5648
	ctx.r[4].s64 = ctx.r[10].s64 + -22088;
	// 826D5868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D586C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5878: 386AC4F4  addi r3, r10, -0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15116;
	// 826D587C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D588C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D589C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D58A0: 4BD91581  bl 0x82466e20
	ctx.lr = 0x826D58A4;
	sub_82466E20(ctx, base);
	// 826D58A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D58A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D58AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D58B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D58B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D58B8 size=108
    let mut pc: u32 = 0x826D58B8;
    'dispatch: loop {
        match pc {
            0x826D58B8 => {
    //   block [0x826D58B8..0x826D5924)
	// 826D58B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D58BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D58C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D58C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D58C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D58CC: 38EB1F78  addi r7, r11, 0x1f78
	ctx.r[7].s64 = ctx.r[11].s64 + 8056;
	// 826D58D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D58D4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826D58D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D58DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D58E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D58E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D58E8: 386AC524  addi r3, r10, -0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + -15068;
	// 826D58EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D58F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D58F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D58F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D58FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D590C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5910: 4BD91511  bl 0x82466e20
	ctx.lr = 0x826D5914;
	sub_82466E20(ctx, base);
	// 826D5914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5928 size=24
    let mut pc: u32 = 0x826D5928;
    'dispatch: loop {
        match pc {
            0x826D5928 => {
    //   block [0x826D5928..0x826D5940)
	// 826D5928: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D592C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5930: 394A9A88  addi r10, r10, -0x6578
	ctx.r[10].s64 = ctx.r[10].s64 + -25976;
	// 826D5934: 816B1E30  lwz r11, 0x1e30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7728 as u32) ) } as u64;
	// 826D5938: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826D593C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5940 size=116
    let mut pc: u32 = 0x826D5940;
    'dispatch: loop {
        match pc {
            0x826D5940 => {
    //   block [0x826D5940..0x826D59B4)
	// 826D5940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D594C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D5950: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5954: 392B46F0  addi r9, r11, 0x46f0
	ctx.r[9].s64 = ctx.r[11].s64 + 18160;
	// 826D5958: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D595C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5960: 38E900C8  addi r7, r9, 0xc8
	ctx.r[7].s64 = ctx.r[9].s64 + 200;
	// 826D5964: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 826D5968: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D596C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826D5970: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5974: 396B9A88  addi r11, r11, -0x6578
	ctx.r[11].s64 = ctx.r[11].s64 + -25976;
	// 826D5978: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D597C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5980: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D5984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5988: 386AC554  addi r3, r10, -0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + -15020;
	// 826D598C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5990: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D5994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5998: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D599C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D59A0: 4BD91481  bl 0x82466e20
	ctx.lr = 0x826D59A4;
	sub_82466E20(ctx, base);
	// 826D59A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D59A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D59AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D59B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D59B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D59B8 size=36
    let mut pc: u32 = 0x826D59B8;
    'dispatch: loop {
        match pc {
            0x826D59B8 => {
    //   block [0x826D59B8..0x826D59DC)
	// 826D59B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D59BC: 814B1E9C  lwz r10, 0x1e9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7836 as u32) ) } as u64;
	// 826D59C0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D59C4: 396B9D58  addi r11, r11, -0x62a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25256;
	// 826D59C8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D59CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D59D0: 814A2020  lwz r10, 0x2020(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8224 as u32) ) } as u64;
	// 826D59D4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D59D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D59E0 size=116
    let mut pc: u32 = 0x826D59E0;
    'dispatch: loop {
        match pc {
            0x826D59E0 => {
    //   block [0x826D59E0..0x826D5A54)
	// 826D59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D59EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D59F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D59F4: 390B9D58  addi r8, r11, -0x62a8
	ctx.r[8].s64 = ctx.r[11].s64 + -25256;
	// 826D59F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D59FC: 392A4878  addi r9, r10, 0x4878
	ctx.r[9].s64 = ctx.r[10].s64 + 18552;
	// 826D5A00: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5A04: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5A08: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D5A0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5A14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5A24: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5A28: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826D5A2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5A30: 386BC584  addi r3, r11, -0x3a7c
	ctx.r[3].s64 = ctx.r[11].s64 + -14972;
	// 826D5A34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D5A38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5A40: 4BD913E1  bl 0x82466e20
	ctx.lr = 0x826D5A44;
	sub_82466E20(ctx, base);
	// 826D5A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5A58 size=24
    let mut pc: u32 = 0x826D5A58;
    'dispatch: loop {
        match pc {
            0x826D5A58 => {
    //   block [0x826D5A58..0x826D5A70)
	// 826D5A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5A5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5A60: 394A9D88  addi r10, r10, -0x6278
	ctx.r[10].s64 = ctx.r[10].s64 + -25208;
	// 826D5A64: 816B2028  lwz r11, 0x2028(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8232 as u32) ) } as u64;
	// 826D5A68: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D5A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5A70 size=116
    let mut pc: u32 = 0x826D5A70;
    'dispatch: loop {
        match pc {
            0x826D5A70 => {
    //   block [0x826D5A70..0x826D5AE4)
	// 826D5A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5A7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D5A80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5A84: 390B9D88  addi r8, r11, -0x6278
	ctx.r[8].s64 = ctx.r[11].s64 + -25208;
	// 826D5A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5A8C: 392A48D0  addi r9, r10, 0x48d0
	ctx.r[9].s64 = ctx.r[10].s64 + 18640;
	// 826D5A90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5A94: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826D5A98: 38AAC584  addi r5, r10, -0x3a7c
	ctx.r[5].s64 = ctx.r[10].s64 + -14972;
	// 826D5A9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5AA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5AB4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D5AB8: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826D5ABC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5AC0: 386BC5B4  addi r3, r11, -0x3a4c
	ctx.r[3].s64 = ctx.r[11].s64 + -14924;
	// 826D5AC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D5AC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5AD0: 4BD91351  bl 0x82466e20
	ctx.lr = 0x826D5AD4;
	sub_82466E20(ctx, base);
	// 826D5AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5AE8 size=112
    let mut pc: u32 = 0x826D5AE8;
    'dispatch: loop {
        match pc {
            0x826D5AE8 => {
    //   block [0x826D5AE8..0x826D5B58)
	// 826D5AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5AF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5AFC: 38AAC584  addi r5, r10, -0x3a7c
	ctx.r[5].s64 = ctx.r[10].s64 + -14972;
	// 826D5B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D5B04: 390B2030  addi r8, r11, 0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + 8240;
	// 826D5B08: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D5B0C: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826D5B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5B14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5B20: 386AC5E4  addi r3, r10, -0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -14876;
	// 826D5B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5B44: 4BD912DD  bl 0x82466e20
	ctx.lr = 0x826D5B48;
	sub_82466E20(ctx, base);
	// 826D5B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5B58 size=112
    let mut pc: u32 = 0x826D5B58;
    'dispatch: loop {
        match pc {
            0x826D5B58 => {
    //   block [0x826D5B58..0x826D5BC8)
	// 826D5B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5B64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5B6C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5B74: 390B2120  addi r8, r11, 0x2120
	ctx.r[8].s64 = ctx.r[11].s64 + 8480;
	// 826D5B78: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826D5B7C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826D5B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5B90: 386AC614  addi r3, r10, -0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14828;
	// 826D5B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5BB4: 4BD9126D  bl 0x82466e20
	ctx.lr = 0x826D5BB8;
	sub_82466E20(ctx, base);
	// 826D5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5BC8 size=108
    let mut pc: u32 = 0x826D5BC8;
    'dispatch: loop {
        match pc {
            0x826D5BC8 => {
    //   block [0x826D5BC8..0x826D5C34)
	// 826D5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5BD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5BDC: 38EB21F8  addi r7, r11, 0x21f8
	ctx.r[7].s64 = ctx.r[11].s64 + 8696;
	// 826D5BE0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D5BE4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826D5BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5BF8: 386AC644  addi r3, r10, -0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14780;
	// 826D5BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5C20: 4BD91201  bl 0x82466e20
	ctx.lr = 0x826D5C24;
	sub_82466E20(ctx, base);
	// 826D5C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5C38 size=108
    let mut pc: u32 = 0x826D5C38;
    'dispatch: loop {
        match pc {
            0x826D5C38 => {
    //   block [0x826D5C38..0x826D5CA4)
	// 826D5C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5C44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5C4C: 38EB2270  addi r7, r11, 0x2270
	ctx.r[7].s64 = ctx.r[11].s64 + 8816;
	// 826D5C50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D5C54: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826D5C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5C60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5C68: 386AC674  addi r3, r10, -0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + -14732;
	// 826D5C6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5C90: 4BD91191  bl 0x82466e20
	ctx.lr = 0x826D5C94;
	sub_82466E20(ctx, base);
	// 826D5C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5CA8 size=112
    let mut pc: u32 = 0x826D5CA8;
    'dispatch: loop {
        match pc {
            0x826D5CA8 => {
    //   block [0x826D5CA8..0x826D5D18)
	// 826D5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5CB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5CB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5CBC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5CC4: 390B22B8  addi r8, r11, 0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8888;
	// 826D5CC8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 826D5CCC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826D5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5CE0: 386AC6A4  addi r3, r10, -0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + -14684;
	// 826D5CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5D04: 4BD9111D  bl 0x82466e20
	ctx.lr = 0x826D5D08;
	sub_82466E20(ctx, base);
	// 826D5D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5D18 size=108
    let mut pc: u32 = 0x826D5D18;
    'dispatch: loop {
        match pc {
            0x826D5D18 => {
    //   block [0x826D5D18..0x826D5D84)
	// 826D5D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5D24: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5D2C: 38EB2498  addi r7, r11, 0x2498
	ctx.r[7].s64 = ctx.r[11].s64 + 9368;
	// 826D5D30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D5D34: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826D5D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5D3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5D48: 386AC6D4  addi r3, r10, -0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + -14636;
	// 826D5D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5D70: 4BD910B1  bl 0x82466e20
	ctx.lr = 0x826D5D74;
	sub_82466E20(ctx, base);
	// 826D5D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D5D88 size=24
    let mut pc: u32 = 0x826D5D88;
    'dispatch: loop {
        match pc {
            0x826D5D88 => {
    //   block [0x826D5D88..0x826D5DA0)
	// 826D5D88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5D8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D5D90: 394A9EA8  addi r10, r10, -0x6158
	ctx.r[10].s64 = ctx.r[10].s64 + -24920;
	// 826D5D94: 816B202C  lwz r11, 0x202c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8236 as u32) ) } as u64;
	// 826D5D98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D5D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5DA0 size=112
    let mut pc: u32 = 0x826D5DA0;
    'dispatch: loop {
        match pc {
            0x826D5DA0 => {
    //   block [0x826D5DA0..0x826D5E10)
	// 826D5DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5DAC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5DB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D5DB4: 392A4928  addi r9, r10, 0x4928
	ctx.r[9].s64 = ctx.r[10].s64 + 18728;
	// 826D5DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5DBC: 390B9EA8  addi r8, r11, -0x6158
	ctx.r[8].s64 = ctx.r[11].s64 + -24920;
	// 826D5DC0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D5DC4: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826D5DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5DD8: 386AC704  addi r3, r10, -0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14588;
	// 826D5DDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D5DE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D5DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5DFC: 4BD91025  bl 0x82466e20
	ctx.lr = 0x826D5E00;
	sub_82466E20(ctx, base);
	// 826D5E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5E10 size=112
    let mut pc: u32 = 0x826D5E10;
    'dispatch: loop {
        match pc {
            0x826D5E10 => {
    //   block [0x826D5E10..0x826D5E80)
	// 826D5E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5E1C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D5E20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D5E24: 38EA24B0  addi r7, r10, 0x24b0
	ctx.r[7].s64 = ctx.r[10].s64 + 9392;
	// 826D5E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5E2C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D5E30: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826D5E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5E38: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5E3C: 396B493C  addi r11, r11, 0x493c
	ctx.r[11].s64 = ctx.r[11].s64 + 18748;
	// 826D5E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5E4C: 386AC734  addi r3, r10, -0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14540;
	// 826D5E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5E54: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D5E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5E5C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D5E60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5E64: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5E68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5E6C: 4BD90FB5  bl 0x82466e20
	ctx.lr = 0x826D5E70;
	sub_82466E20(ctx, base);
	// 826D5E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5E80 size=112
    let mut pc: u32 = 0x826D5E80;
    'dispatch: loop {
        match pc {
            0x826D5E80 => {
    //   block [0x826D5E80..0x826D5EF0)
	// 826D5E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5E8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5E90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5E94: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5E9C: 390B2540  addi r8, r11, 0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + 9536;
	// 826D5EA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D5EA4: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826D5EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D5EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5EB8: 386AC764  addi r3, r10, -0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + -14492;
	// 826D5EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D5EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5EDC: 4BD90F45  bl 0x82466e20
	ctx.lr = 0x826D5EE0;
	sub_82466E20(ctx, base);
	// 826D5EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5EF0 size=108
    let mut pc: u32 = 0x826D5EF0;
    'dispatch: loop {
        match pc {
            0x826D5EF0 => {
    //   block [0x826D5EF0..0x826D5F5C)
	// 826D5EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5EFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5F04: 38EB2560  addi r7, r11, 0x2560
	ctx.r[7].s64 = ctx.r[11].s64 + 9568;
	// 826D5F08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D5F0C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826D5F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5F20: 386AC794  addi r3, r10, -0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + -14444;
	// 826D5F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5F48: 4BD90ED9  bl 0x82466e20
	ctx.lr = 0x826D5F4C;
	sub_82466E20(ctx, base);
	// 826D5F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5F60 size=108
    let mut pc: u32 = 0x826D5F60;
    'dispatch: loop {
        match pc {
            0x826D5F60 => {
    //   block [0x826D5F60..0x826D5FCC)
	// 826D5F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5F6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D5F74: 38EB25C0  addi r7, r11, 0x25c0
	ctx.r[7].s64 = ctx.r[11].s64 + 9664;
	// 826D5F78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D5F7C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826D5F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D5F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D5F90: 386AC7C4  addi r3, r10, -0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + -14396;
	// 826D5F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D5F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D5F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D5FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D5FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D5FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D5FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D5FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D5FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D5FB8: 4BD90E69  bl 0x82466e20
	ctx.lr = 0x826D5FBC;
	sub_82466E20(ctx, base);
	// 826D5FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D5FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D5FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D5FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D5FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D5FD0 size=116
    let mut pc: u32 = 0x826D5FD0;
    'dispatch: loop {
        match pc {
            0x826D5FD0 => {
    //   block [0x826D5FD0..0x826D6044)
	// 826D5FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D5FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D5FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D5FDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D5FE0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D5FE4: 390B25F0  addi r8, r11, 0x25f0
	ctx.r[8].s64 = ctx.r[11].s64 + 9712;
	// 826D5FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D5FEC: 392A4970  addi r9, r10, 0x4970
	ctx.r[9].s64 = ctx.r[10].s64 + 18800;
	// 826D5FF0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D5FF4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D5FF8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D5FFC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6014: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6018: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826D601C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6020: 386BC7F4  addi r3, r11, -0x380c
	ctx.r[3].s64 = ctx.r[11].s64 + -14348;
	// 826D6024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6030: 4BD90DF1  bl 0x82466e20
	ctx.lr = 0x826D6034;
	sub_82466E20(ctx, base);
	// 826D6034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6048 size=76
    let mut pc: u32 = 0x826D6048;
    'dispatch: loop {
        match pc {
            0x826D6048 => {
    //   block [0x826D6048..0x826D6094)
	// 826D6048: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D604C: 814B255C  lwz r10, 0x255c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9564 as u32) ) } as u64;
	// 826D6050: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6054: 396B9ED8  addi r11, r11, -0x6128
	ctx.r[11].s64 = ctx.r[11].s64 + -24872;
	// 826D6058: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826D605C: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826D6060: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D6064: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6068: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826D606C: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826D6070: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6074: 814A2608  lwz r10, 0x2608(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9736 as u32) ) } as u64;
	// 826D6078: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826D607C: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826D6080: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826D6084: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826D6088: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826D608C: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826D6090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6098 size=108
    let mut pc: u32 = 0x826D6098;
    'dispatch: loop {
        match pc {
            0x826D6098 => {
    //   block [0x826D6098..0x826D6104)
	// 826D6098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D60A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D60A4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D60A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D60AC: 38EB9ED8  addi r7, r11, -0x6128
	ctx.r[7].s64 = ctx.r[11].s64 + -24872;
	// 826D60B0: 39000019  li r8, 0x19
	ctx.r[8].s64 = 25;
	// 826D60B4: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826D60B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D60BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D60C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D60C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D60C8: 386AC824  addi r3, r10, -0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14300;
	// 826D60CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D60D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D60D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D60D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D60DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D60E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D60E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D60E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D60EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D60F0: 4BD90D31  bl 0x82466e20
	ctx.lr = 0x826D60F4;
	sub_82466E20(ctx, base);
	// 826D60F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D60F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D60FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6108 size=76
    let mut pc: u32 = 0x826D6108;
    'dispatch: loop {
        match pc {
            0x826D6108 => {
    //   block [0x826D6108..0x826D6154)
	// 826D6108: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D610C: 814B255C  lwz r10, 0x255c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9564 as u32) ) } as u64;
	// 826D6110: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6114: 396BA130  addi r11, r11, -0x5ed0
	ctx.r[11].s64 = ctx.r[11].s64 + -24272;
	// 826D6118: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826D611C: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826D6120: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D6124: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6128: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826D612C: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826D6130: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6134: 814A2608  lwz r10, 0x2608(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9736 as u32) ) } as u64;
	// 826D6138: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826D613C: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826D6140: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826D6144: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826D6148: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826D614C: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826D6150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6158 size=116
    let mut pc: u32 = 0x826D6158;
    'dispatch: loop {
        match pc {
            0x826D6158 => {
    //   block [0x826D6158..0x826D61CC)
	// 826D6158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D615C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6164: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6168: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D616C: 390BA130  addi r8, r11, -0x5ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -24272;
	// 826D6170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6174: 392A49BC  addi r9, r10, 0x49bc
	ctx.r[9].s64 = ctx.r[10].s64 + 18876;
	// 826D6178: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D617C: 38E00026  li r7, 0x26
	ctx.r[7].s64 = 38;
	// 826D6180: 38AABBC4  addi r5, r10, -0x443c
	ctx.r[5].s64 = ctx.r[10].s64 + -17468;
	// 826D6184: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D618C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D619C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D61A0: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826D61A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D61A8: 386BC854  addi r3, r11, -0x37ac
	ctx.r[3].s64 = ctx.r[11].s64 + -14252;
	// 826D61AC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D61B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D61B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D61B8: 4BD90C69  bl 0x82466e20
	ctx.lr = 0x826D61BC;
	sub_82466E20(ctx, base);
	// 826D61BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D61C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D61C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D61C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D61D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D61D0 size=112
    let mut pc: u32 = 0x826D61D0;
    'dispatch: loop {
        match pc {
            0x826D61D0 => {
    //   block [0x826D61D0..0x826D6240)
	// 826D61D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D61D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D61D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D61DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D61E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D61E4: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D61E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D61EC: 390B2610  addi r8, r11, 0x2610
	ctx.r[8].s64 = ctx.r[11].s64 + 9744;
	// 826D61F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D61F4: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 826D61F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D61FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6208: 386AC884  addi r3, r10, -0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + -14204;
	// 826D620C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D621C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D622C: 4BD90BF5  bl 0x82466e20
	ctx.lr = 0x826D6230;
	sub_82466E20(ctx, base);
	// 826D6230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6240 size=108
    let mut pc: u32 = 0x826D6240;
    'dispatch: loop {
        match pc {
            0x826D6240 => {
    //   block [0x826D6240..0x826D62AC)
	// 826D6240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D624C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6254: 38EB2658  addi r7, r11, 0x2658
	ctx.r[7].s64 = ctx.r[11].s64 + 9816;
	// 826D6258: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D625C: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 826D6260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6264: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D626C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6270: 386AC8B4  addi r3, r10, -0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + -14156;
	// 826D6274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D627C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D628C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6298: 4BD90B89  bl 0x82466e20
	ctx.lr = 0x826D629C;
	sub_82466E20(ctx, base);
	// 826D629C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D62A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D62A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D62A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D62B0 size=108
    let mut pc: u32 = 0x826D62B0;
    'dispatch: loop {
        match pc {
            0x826D62B0 => {
    //   block [0x826D62B0..0x826D631C)
	// 826D62B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D62B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D62B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D62BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D62C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D62C4: 38EB26A0  addi r7, r11, 0x26a0
	ctx.r[7].s64 = ctx.r[11].s64 + 9888;
	// 826D62C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D62CC: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 826D62D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D62D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D62D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D62DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D62E0: 386AC8E4  addi r3, r10, -0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + -14108;
	// 826D62E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D62E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D62EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D62F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D62F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D62F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6308: 4BD90B19  bl 0x82466e20
	ctx.lr = 0x826D630C;
	sub_82466E20(ctx, base);
	// 826D630C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6320 size=116
    let mut pc: u32 = 0x826D6320;
    'dispatch: loop {
        match pc {
            0x826D6320 => {
    //   block [0x826D6320..0x826D6394)
	// 826D6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D632C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D6334: 390A26E8  addi r8, r10, 0x26e8
	ctx.r[8].s64 = ctx.r[10].s64 + 9960;
	// 826D6338: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D633C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6340: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D6344: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6348: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6354: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 826D6358: 396B49E4  addi r11, r11, 0x49e4
	ctx.r[11].s64 = ctx.r[11].s64 + 18916;
	// 826D635C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6364: 386AC914  addi r3, r10, -0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14060;
	// 826D6368: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D636C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6370: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D6374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D637C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6380: 4BD90AA1  bl 0x82466e20
	ctx.lr = 0x826D6384;
	sub_82466E20(ctx, base);
	// 826D6384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6398 size=116
    let mut pc: u32 = 0x826D6398;
    'dispatch: loop {
        match pc {
            0x826D6398 => {
    //   block [0x826D6398..0x826D640C)
	// 826D6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D63A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D63A4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D63A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D63AC: 390A2790  addi r8, r10, 0x2790
	ctx.r[8].s64 = ctx.r[10].s64 + 10128;
	// 826D63B0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D63B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D63B8: 38AAC914  addi r5, r10, -0x36ec
	ctx.r[5].s64 = ctx.r[10].s64 + -14060;
	// 826D63BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D63C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D63C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D63C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D63CC: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 826D63D0: 396B4A08  addi r11, r11, 0x4a08
	ctx.r[11].s64 = ctx.r[11].s64 + 18952;
	// 826D63D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D63D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D63DC: 386AC944  addi r3, r10, -0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14012;
	// 826D63E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D63E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D63E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D63EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D63F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D63F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D63F8: 4BD90A29  bl 0x82466e20
	ctx.lr = 0x826D63FC;
	sub_82466E20(ctx, base);
	// 826D63FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6410 size=36
    let mut pc: u32 = 0x826D6410;
    'dispatch: loop {
        match pc {
            0x826D6410 => {
    //   block [0x826D6410..0x826D6434)
	// 826D6410: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6414: 814B260C  lwz r10, 0x260c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9740 as u32) ) } as u64;
	// 826D6418: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D641C: 396BA4C0  addi r11, r11, -0x5b40
	ctx.r[11].s64 = ctx.r[11].s64 + -23360;
	// 826D6420: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D6424: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6428: 814A27F0  lwz r10, 0x27f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10224 as u32) ) } as u64;
	// 826D642C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D6430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6438 size=116
    let mut pc: u32 = 0x826D6438;
    'dispatch: loop {
        match pc {
            0x826D6438 => {
    //   block [0x826D6438..0x826D64AC)
	// 826D6438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6444: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6448: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D644C: 390BA4C0  addi r8, r11, -0x5b40
	ctx.r[8].s64 = ctx.r[11].s64 + -23360;
	// 826D6450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6454: 392A4A4C  addi r9, r10, 0x4a4c
	ctx.r[9].s64 = ctx.r[10].s64 + 19020;
	// 826D6458: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D645C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826D6460: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6464: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D646C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D647C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6480: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 826D6484: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6488: 386BC974  addi r3, r11, -0x368c
	ctx.r[3].s64 = ctx.r[11].s64 + -13964;
	// 826D648C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D6490: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6498: 4BD90989  bl 0x82466e20
	ctx.lr = 0x826D649C;
	sub_82466E20(ctx, base);
	// 826D649C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D64A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D64A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D64A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D64B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D64B0 size=112
    let mut pc: u32 = 0x826D64B0;
    'dispatch: loop {
        match pc {
            0x826D64B0 => {
    //   block [0x826D64B0..0x826D6520)
	// 826D64B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D64B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D64B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D64BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D64C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D64C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D64C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D64CC: 390B27F8  addi r8, r11, 0x27f8
	ctx.r[8].s64 = ctx.r[11].s64 + 10232;
	// 826D64D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D64D4: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 826D64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D64DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D64E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D64E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D64E8: 386AC9A4  addi r3, r10, -0x365c
	ctx.r[3].s64 = ctx.r[10].s64 + -13916;
	// 826D64EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D64F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D64F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D64F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D64FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D650C: 4BD90915  bl 0x82466e20
	ctx.lr = 0x826D6510;
	sub_82466E20(ctx, base);
	// 826D6510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6520 size=108
    let mut pc: u32 = 0x826D6520;
    'dispatch: loop {
        match pc {
            0x826D6520 => {
    //   block [0x826D6520..0x826D658C)
	// 826D6520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D652C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6534: 38EB28B8  addi r7, r11, 0x28b8
	ctx.r[7].s64 = ctx.r[11].s64 + 10424;
	// 826D6538: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D653C: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 826D6540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D654C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6550: 386AC9D4  addi r3, r10, -0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + -13868;
	// 826D6554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D655C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D656C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6578: 4BD908A9  bl 0x82466e20
	ctx.lr = 0x826D657C;
	sub_82466E20(ctx, base);
	// 826D657C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6590 size=112
    let mut pc: u32 = 0x826D6590;
    'dispatch: loop {
        match pc {
            0x826D6590 => {
    //   block [0x826D6590..0x826D6600)
	// 826D6590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D659C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D65A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D65A4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D65A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D65AC: 390B28E8  addi r8, r11, 0x28e8
	ctx.r[8].s64 = ctx.r[11].s64 + 10472;
	// 826D65B0: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 826D65B4: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 826D65B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D65BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D65C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D65C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D65C8: 386ACA04  addi r3, r10, -0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13820;
	// 826D65CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D65D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D65D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D65D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D65DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D65E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D65E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D65E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D65EC: 4BD90835  bl 0x82466e20
	ctx.lr = 0x826D65F0;
	sub_82466E20(ctx, base);
	// 826D65F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D65F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D65F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D65FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6600 size=112
    let mut pc: u32 = 0x826D6600;
    'dispatch: loop {
        match pc {
            0x826D6600 => {
    //   block [0x826D6600..0x826D6670)
	// 826D6600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D660C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6610: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6614: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D661C: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 826D6620: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826D6624: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 826D6628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D662C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6638: 386ACA34  addi r3, r10, -0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13772;
	// 826D663C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D664C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D665C: 4BD907C5  bl 0x82466e20
	ctx.lr = 0x826D6660;
	sub_82466E20(ctx, base);
	// 826D6660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D666C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6670 size=116
    let mut pc: u32 = 0x826D6670;
    'dispatch: loop {
        match pc {
            0x826D6670 => {
    //   block [0x826D6670..0x826D66E4)
	// 826D6670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D667C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6680: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6684: 390B2B88  addi r8, r11, 0x2b88
	ctx.r[8].s64 = ctx.r[11].s64 + 11144;
	// 826D6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D668C: 392A4A84  addi r9, r10, 0x4a84
	ctx.r[9].s64 = ctx.r[10].s64 + 19076;
	// 826D6690: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6694: 38E0001D  li r7, 0x1d
	ctx.r[7].s64 = 29;
	// 826D6698: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D669C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D66A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D66A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D66A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D66AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D66B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D66B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D66B8: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 826D66BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D66C0: 386BCA64  addi r3, r11, -0x359c
	ctx.r[3].s64 = ctx.r[11].s64 + -13724;
	// 826D66C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D66C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D66CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D66D0: 4BD90751  bl 0x82466e20
	ctx.lr = 0x826D66D4;
	sub_82466E20(ctx, base);
	// 826D66D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D66D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D66DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D66E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D66E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D66E8 size=112
    let mut pc: u32 = 0x826D66E8;
    'dispatch: loop {
        match pc {
            0x826D66E8 => {
    //   block [0x826D66E8..0x826D6758)
	// 826D66E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D66EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D66F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D66F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D66F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D66FC: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6704: 390B2E40  addi r8, r11, 0x2e40
	ctx.r[8].s64 = ctx.r[11].s64 + 11840;
	// 826D6708: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D670C: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 826D6710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D671C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6720: 386ACA94  addi r3, r10, -0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + -13676;
	// 826D6724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D672C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D673C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6744: 4BD906DD  bl 0x82466e20
	ctx.lr = 0x826D6748;
	sub_82466E20(ctx, base);
	// 826D6748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D674C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6758 size=116
    let mut pc: u32 = 0x826D6758;
    'dispatch: loop {
        match pc {
            0x826D6758 => {
    //   block [0x826D6758..0x826D67CC)
	// 826D6758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D675C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6764: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6768: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D676C: 390A2E88  addi r8, r10, 0x2e88
	ctx.r[8].s64 = ctx.r[10].s64 + 11912;
	// 826D6770: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6774: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6778: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D677C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6780: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D678C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826D6790: 396B4A98  addi r11, r11, 0x4a98
	ctx.r[11].s64 = ctx.r[11].s64 + 19096;
	// 826D6794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6798: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D679C: 386ACAC4  addi r3, r10, -0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + -13628;
	// 826D67A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D67A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D67A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D67AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D67B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D67B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D67B8: 4BD90669  bl 0x82466e20
	ctx.lr = 0x826D67BC;
	sub_82466E20(ctx, base);
	// 826D67BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D67C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D67C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D67C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D67D0 size=112
    let mut pc: u32 = 0x826D67D0;
    'dispatch: loop {
        match pc {
            0x826D67D0 => {
    //   block [0x826D67D0..0x826D6840)
	// 826D67D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D67D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D67D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D67DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D67E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D67E4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D67E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D67EC: 390B2ED0  addi r8, r11, 0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 11984;
	// 826D67F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D67F4: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 826D67F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D67FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6808: 386ACAF4  addi r3, r10, -0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + -13580;
	// 826D680C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D681C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D682C: 4BD905F5  bl 0x82466e20
	ctx.lr = 0x826D6830;
	sub_82466E20(ctx, base);
	// 826D6830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6840 size=112
    let mut pc: u32 = 0x826D6840;
    'dispatch: loop {
        match pc {
            0x826D6840 => {
    //   block [0x826D6840..0x826D68B0)
	// 826D6840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D684C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6850: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6854: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6858: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D685C: 390B2F60  addi r8, r11, 0x2f60
	ctx.r[8].s64 = ctx.r[11].s64 + 12128;
	// 826D6860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D6864: 388AE324  addi r4, r10, -0x1cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -7388;
	// 826D6868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D686C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6878: 386ACB24  addi r3, r10, -0x34dc
	ctx.r[3].s64 = ctx.r[10].s64 + -13532;
	// 826D687C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D688C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D689C: 4BD90585  bl 0x82466e20
	ctx.lr = 0x826D68A0;
	sub_82466E20(ctx, base);
	// 826D68A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D68A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D68A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D68AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D68B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D68B0 size=112
    let mut pc: u32 = 0x826D68B0;
    'dispatch: loop {
        match pc {
            0x826D68B0 => {
    //   block [0x826D68B0..0x826D6920)
	// 826D68B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D68B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D68B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D68BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D68C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D68C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D68C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D68CC: 390B2FD8  addi r8, r11, 0x2fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 12248;
	// 826D68D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D68D4: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 826D68D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D68DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D68E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D68E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D68E8: 386ACB54  addi r3, r10, -0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13484;
	// 826D68EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D68F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D68F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D68F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D68FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D690C: 4BD90515  bl 0x82466e20
	ctx.lr = 0x826D6910;
	sub_82466E20(ctx, base);
	// 826D6910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D691C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6920 size=24
    let mut pc: u32 = 0x826D6920;
    'dispatch: loop {
        match pc {
            0x826D6920 => {
    //   block [0x826D6920..0x826D6938)
	// 826D6920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6928: 394AA5C8  addi r10, r10, -0x5a38
	ctx.r[10].s64 = ctx.r[10].s64 + -23096;
	// 826D692C: 816B3050  lwz r11, 0x3050(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12368 as u32) ) } as u64;
	// 826D6930: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826D6934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6938 size=116
    let mut pc: u32 = 0x826D6938;
    'dispatch: loop {
        match pc {
            0x826D6938 => {
    //   block [0x826D6938..0x826D69AC)
	// 826D6938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6944: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6948: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D694C: 392B4AC4  addi r9, r11, 0x4ac4
	ctx.r[9].s64 = ctx.r[11].s64 + 19140;
	// 826D6950: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6954: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6958: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D695C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D6960: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6964: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826D6968: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D696C: 396BA5C8  addi r11, r11, -0x5a38
	ctx.r[11].s64 = ctx.r[11].s64 + -23096;
	// 826D6970: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D6974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6978: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D697C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6980: 386ACB84  addi r3, r10, -0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + -13436;
	// 826D6984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6988: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D698C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6990: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D6994: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D6998: 4BD90489  bl 0x82466e20
	ctx.lr = 0x826D699C;
	sub_82466E20(ctx, base);
	// 826D699C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D69A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D69A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D69A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D69B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D69B0 size=112
    let mut pc: u32 = 0x826D69B0;
    'dispatch: loop {
        match pc {
            0x826D69B0 => {
    //   block [0x826D69B0..0x826D6A20)
	// 826D69B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D69B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D69B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D69BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D69C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D69C4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D69C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D69CC: 390B3058  addi r8, r11, 0x3058
	ctx.r[8].s64 = ctx.r[11].s64 + 12376;
	// 826D69D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D69D4: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 826D69D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D69DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D69E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D69E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D69E8: 386ACBB4  addi r3, r10, -0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + -13388;
	// 826D69EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D69F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D69F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D69F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D69FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6A0C: 4BD90415  bl 0x82466e20
	ctx.lr = 0x826D6A10;
	sub_82466E20(ctx, base);
	// 826D6A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6A20 size=24
    let mut pc: u32 = 0x826D6A20;
    'dispatch: loop {
        match pc {
            0x826D6A20 => {
    //   block [0x826D6A20..0x826D6A38)
	// 826D6A20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6A24: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6A28: 394AA670  addi r10, r10, -0x5990
	ctx.r[10].s64 = ctx.r[10].s64 + -22928;
	// 826D6A2C: 816B3054  lwz r11, 0x3054(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12372 as u32) ) } as u64;
	// 826D6A30: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D6A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6A38 size=116
    let mut pc: u32 = 0x826D6A38;
    'dispatch: loop {
        match pc {
            0x826D6A38 => {
    //   block [0x826D6A38..0x826D6AAC)
	// 826D6A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6A44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6A48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6A4C: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 826D6A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6A54: 392A4B2C  addi r9, r10, 0x4b2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19244;
	// 826D6A58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6A5C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826D6A60: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6A64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6A6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6A7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6A80: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826D6A84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6A88: 386BCBE4  addi r3, r11, -0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + -13340;
	// 826D6A8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6A90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6A98: 4BD90389  bl 0x82466e20
	ctx.lr = 0x826D6A9C;
	sub_82466E20(ctx, base);
	// 826D6A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6AB0 size=112
    let mut pc: u32 = 0x826D6AB0;
    'dispatch: loop {
        match pc {
            0x826D6AB0 => {
    //   block [0x826D6AB0..0x826D6B20)
	// 826D6AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6ABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6AC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6AC4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6ACC: 390B30D0  addi r8, r11, 0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + 12496;
	// 826D6AD0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D6AD4: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 826D6AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6ADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6AE8: 386ACC14  addi r3, r10, -0x33ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13292;
	// 826D6AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6B0C: 4BD90315  bl 0x82466e20
	ctx.lr = 0x826D6B10;
	sub_82466E20(ctx, base);
	// 826D6B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6B20 size=112
    let mut pc: u32 = 0x826D6B20;
    'dispatch: loop {
        match pc {
            0x826D6B20 => {
    //   block [0x826D6B20..0x826D6B90)
	// 826D6B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6B2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6B30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6B34: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6B3C: 390B31C0  addi r8, r11, 0x31c0
	ctx.r[8].s64 = ctx.r[11].s64 + 12736;
	// 826D6B40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6B44: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 826D6B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6B58: 386ACC44  addi r3, r10, -0x33bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13244;
	// 826D6B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6B7C: 4BD902A5  bl 0x82466e20
	ctx.lr = 0x826D6B80;
	sub_82466E20(ctx, base);
	// 826D6B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6B90 size=24
    let mut pc: u32 = 0x826D6B90;
    'dispatch: loop {
        match pc {
            0x826D6B90 => {
    //   block [0x826D6B90..0x826D6BA8)
	// 826D6B90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6B94: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D6B98: 394AA718  addi r10, r10, -0x58e8
	ctx.r[10].s64 = ctx.r[10].s64 + -22760;
	// 826D6B9C: 816B3220  lwz r11, 0x3220(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12832 as u32) ) } as u64;
	// 826D6BA0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D6BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6BA8 size=116
    let mut pc: u32 = 0x826D6BA8;
    'dispatch: loop {
        match pc {
            0x826D6BA8 => {
    //   block [0x826D6BA8..0x826D6C1C)
	// 826D6BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6BB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6BB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6BBC: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 826D6BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6BC4: 392A4B78  addi r9, r10, 0x4b78
	ctx.r[9].s64 = ctx.r[10].s64 + 19320;
	// 826D6BC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6BCC: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 826D6BD0: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6BD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6BDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6BEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6BF0: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826D6BF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6BF8: 386BCC74  addi r3, r11, -0x338c
	ctx.r[3].s64 = ctx.r[11].s64 + -13196;
	// 826D6BFC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D6C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6C08: 4BD90219  bl 0x82466e20
	ctx.lr = 0x826D6C0C;
	sub_82466E20(ctx, base);
	// 826D6C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6C20 size=116
    let mut pc: u32 = 0x826D6C20;
    'dispatch: loop {
        match pc {
            0x826D6C20 => {
    //   block [0x826D6C20..0x826D6C94)
	// 826D6C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6C2C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6C30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6C34: 392B4BB0  addi r9, r11, 0x4bb0
	ctx.r[9].s64 = ctx.r[11].s64 + 19376;
	// 826D6C38: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6C3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6C40: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826D6C44: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 826D6C48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6C4C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 826D6C50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6C54: 396B3230  addi r11, r11, 0x3230
	ctx.r[11].s64 = ctx.r[11].s64 + 12848;
	// 826D6C58: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D6C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6C60: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D6C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6C68: 386ACCA4  addi r3, r10, -0x335c
	ctx.r[3].s64 = ctx.r[10].s64 + -13148;
	// 826D6C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D6C70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D6C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6C78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D6C7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D6C80: 4BD901A1  bl 0x82466e20
	ctx.lr = 0x826D6C84;
	sub_82466E20(ctx, base);
	// 826D6C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6C98 size=112
    let mut pc: u32 = 0x826D6C98;
    'dispatch: loop {
        match pc {
            0x826D6C98 => {
    //   block [0x826D6C98..0x826D6D08)
	// 826D6C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6CA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6CAC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D6CB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6CB4: 390B33F8  addi r8, r11, 0x33f8
	ctx.r[8].s64 = ctx.r[11].s64 + 13304;
	// 826D6CB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6CBC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 826D6CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6CC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6CD0: 386ACCD4  addi r3, r10, -0x332c
	ctx.r[3].s64 = ctx.r[10].s64 + -13100;
	// 826D6CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6CF4: 4BD9012D  bl 0x82466e20
	ctx.lr = 0x826D6CF8;
	sub_82466E20(ctx, base);
	// 826D6CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D6D08 size=48
    let mut pc: u32 = 0x826D6D08;
    'dispatch: loop {
        match pc {
            0x826D6D08 => {
    //   block [0x826D6D08..0x826D6D38)
	// 826D6D08: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6D0C: 814B3458  lwz r10, 0x3458(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13400 as u32) ) } as u64;
	// 826D6D10: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6D14: 396BA970  addi r11, r11, -0x5690
	ctx.r[11].s64 = ctx.r[11].s64 + -22160;
	// 826D6D18: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D6D1C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6D20: 814A345C  lwz r10, 0x345c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(13404 as u32) ) } as u64;
	// 826D6D24: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826D6D28: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6D2C: 814A322C  lwz r10, 0x322c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12844 as u32) ) } as u64;
	// 826D6D30: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 826D6D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6D38 size=116
    let mut pc: u32 = 0x826D6D38;
    'dispatch: loop {
        match pc {
            0x826D6D38 => {
    //   block [0x826D6D38..0x826D6DAC)
	// 826D6D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6D44: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D6D48: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D6D4C: 390BA970  addi r8, r11, -0x5690
	ctx.r[8].s64 = ctx.r[11].s64 + -22160;
	// 826D6D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6D54: 392A4C70  addi r9, r10, 0x4c70
	ctx.r[9].s64 = ctx.r[10].s64 + 19568;
	// 826D6D58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6D5C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826D6D60: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6D64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6D6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6D7C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D6D80: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826D6D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6D88: 386BCD04  addi r3, r11, -0x32fc
	ctx.r[3].s64 = ctx.r[11].s64 + -13052;
	// 826D6D8C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826D6D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6D98: 4BD90089  bl 0x82466e20
	ctx.lr = 0x826D6D9C;
	sub_82466E20(ctx, base);
	// 826D6D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6DB0 size=112
    let mut pc: u32 = 0x826D6DB0;
    'dispatch: loop {
        match pc {
            0x826D6DB0 => {
    //   block [0x826D6DB0..0x826D6E20)
	// 826D6DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6DC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6DC4: 38AABBF4  addi r5, r10, -0x440c
	ctx.r[5].s64 = ctx.r[10].s64 + -17420;
	// 826D6DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6DCC: 390B3460  addi r8, r11, 0x3460
	ctx.r[8].s64 = ctx.r[11].s64 + 13408;
	// 826D6DD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D6DD4: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 826D6DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6DE8: 386ACD34  addi r3, r10, -0x32cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13004;
	// 826D6DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6E0C: 4BD90015  bl 0x82466e20
	ctx.lr = 0x826D6E10;
	sub_82466E20(ctx, base);
	// 826D6E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6E20 size=112
    let mut pc: u32 = 0x826D6E20;
    'dispatch: loop {
        match pc {
            0x826D6E20 => {
    //   block [0x826D6E20..0x826D6E90)
	// 826D6E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6E2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6E30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6E34: 38AAC194  addi r5, r10, -0x3e6c
	ctx.r[5].s64 = ctx.r[10].s64 + -15980;
	// 826D6E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6E3C: 390B34A8  addi r8, r11, 0x34a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13480;
	// 826D6E40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D6E44: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826D6E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6E58: 386ACD64  addi r3, r10, -0x329c
	ctx.r[3].s64 = ctx.r[10].s64 + -12956;
	// 826D6E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D6E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6E7C: 4BD8FFA5  bl 0x82466e20
	ctx.lr = 0x826D6E80;
	sub_82466E20(ctx, base);
	// 826D6E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6E90 size=116
    let mut pc: u32 = 0x826D6E90;
    'dispatch: loop {
        match pc {
            0x826D6E90 => {
    //   block [0x826D6E90..0x826D6F04)
	// 826D6E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6E9C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D6EA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D6EA4: 390A3508  addi r8, r10, 0x3508
	ctx.r[8].s64 = ctx.r[10].s64 + 13576;
	// 826D6EA8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6EAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D6EB0: 38AAC5B4  addi r5, r10, -0x3a4c
	ctx.r[5].s64 = ctx.r[10].s64 + -14924;
	// 826D6EB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6EB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D6EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D6EC4: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 826D6EC8: 396B4CAC  addi r11, r11, 0x4cac
	ctx.r[11].s64 = ctx.r[11].s64 + 19628;
	// 826D6ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6ED4: 386ACD94  addi r3, r10, -0x326c
	ctx.r[3].s64 = ctx.r[10].s64 + -12908;
	// 826D6ED8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D6EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6EE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D6EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6EF0: 4BD8FF31  bl 0x82466e20
	ctx.lr = 0x826D6EF4;
	sub_82466E20(ctx, base);
	// 826D6EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6F08 size=108
    let mut pc: u32 = 0x826D6F08;
    'dispatch: loop {
        match pc {
            0x826D6F08 => {
    //   block [0x826D6F08..0x826D6F74)
	// 826D6F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6F14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6F18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6F1C: 38EB3580  addi r7, r11, 0x3580
	ctx.r[7].s64 = ctx.r[11].s64 + 13696;
	// 826D6F20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D6F24: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 826D6F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6F2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D6F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6F38: 386ACDC4  addi r3, r10, -0x323c
	ctx.r[3].s64 = ctx.r[10].s64 + -12860;
	// 826D6F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6F60: 4BD8FEC1  bl 0x82466e20
	ctx.lr = 0x826D6F64;
	sub_82466E20(ctx, base);
	// 826D6F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6F78 size=108
    let mut pc: u32 = 0x826D6F78;
    'dispatch: loop {
        match pc {
            0x826D6F78 => {
    //   block [0x826D6F78..0x826D6FE4)
	// 826D6F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6F84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6F88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6F8C: 38EB35C8  addi r7, r11, 0x35c8
	ctx.r[7].s64 = ctx.r[11].s64 + 13768;
	// 826D6F90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D6F94: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826D6F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D6F9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D6FA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D6FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D6FA8: 386ACDF4  addi r3, r10, -0x320c
	ctx.r[3].s64 = ctx.r[10].s64 + -12812;
	// 826D6FAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D6FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D6FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D6FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D6FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D6FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D6FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D6FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D6FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D6FD0: 4BD8FE51  bl 0x82466e20
	ctx.lr = 0x826D6FD4;
	sub_82466E20(ctx, base);
	// 826D6FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D6FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D6FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D6FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D6FE8 size=108
    let mut pc: u32 = 0x826D6FE8;
    'dispatch: loop {
        match pc {
            0x826D6FE8 => {
    //   block [0x826D6FE8..0x826D7054)
	// 826D6FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D6FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D6FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D6FF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D6FF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D6FFC: 38EB3610  addi r7, r11, 0x3610
	ctx.r[7].s64 = ctx.r[11].s64 + 13840;
	// 826D7000: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D7004: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 826D7008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D700C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7018: 386ACE24  addi r3, r10, -0x31dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12764;
	// 826D701C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D702C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D703C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7040: 4BD8FDE1  bl 0x82466e20
	ctx.lr = 0x826D7044;
	sub_82466E20(ctx, base);
	// 826D7044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D704C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7058 size=108
    let mut pc: u32 = 0x826D7058;
    'dispatch: loop {
        match pc {
            0x826D7058 => {
    //   block [0x826D7058..0x826D70C4)
	// 826D7058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D705C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7064: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D706C: 38EB3658  addi r7, r11, 0x3658
	ctx.r[7].s64 = ctx.r[11].s64 + 13912;
	// 826D7070: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D7074: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826D7078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D707C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7088: 386ACE54  addi r3, r10, -0x31ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12716;
	// 826D708C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D709C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D70A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D70A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D70A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D70AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D70B0: 4BD8FD71  bl 0x82466e20
	ctx.lr = 0x826D70B4;
	sub_82466E20(ctx, base);
	// 826D70B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D70B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D70BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D70C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D70C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D70C8 size=108
    let mut pc: u32 = 0x826D70C8;
    'dispatch: loop {
        match pc {
            0x826D70C8 => {
    //   block [0x826D70C8..0x826D7134)
	// 826D70C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D70CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D70D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D70D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D70D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D70DC: 38EB3700  addi r7, r11, 0x3700
	ctx.r[7].s64 = ctx.r[11].s64 + 14080;
	// 826D70E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D70E4: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826D70E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D70EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D70F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D70F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D70F8: 386ACE84  addi r3, r10, -0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + -12668;
	// 826D70FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D710C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D711C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7120: 4BD8FD01  bl 0x82466e20
	ctx.lr = 0x826D7124;
	sub_82466E20(ctx, base);
	// 826D7124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D712C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7138 size=112
    let mut pc: u32 = 0x826D7138;
    'dispatch: loop {
        match pc {
            0x826D7138 => {
    //   block [0x826D7138..0x826D71A8)
	// 826D7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7144: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D714C: 392A4CEC  addi r9, r10, 0x4cec
	ctx.r[9].s64 = ctx.r[10].s64 + 19692;
	// 826D7150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7154: 390B3738  addi r8, r11, 0x3738
	ctx.r[8].s64 = ctx.r[11].s64 + 14136;
	// 826D7158: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D715C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826D7160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D716C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7170: 386ACEB4  addi r3, r10, -0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + -12620;
	// 826D7174: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7178: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D718C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7194: 4BD8FC8D  bl 0x82466e20
	ctx.lr = 0x826D7198;
	sub_82466E20(ctx, base);
	// 826D7198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D719C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D71A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D71A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D71A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D71A8 size=108
    let mut pc: u32 = 0x826D71A8;
    'dispatch: loop {
        match pc {
            0x826D71A8 => {
    //   block [0x826D71A8..0x826D7214)
	// 826D71A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D71AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D71B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D71B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D71B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D71BC: 38EB3780  addi r7, r11, 0x3780
	ctx.r[7].s64 = ctx.r[11].s64 + 14208;
	// 826D71C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D71C4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826D71C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D71CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D71D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D71D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D71D8: 386ACEE4  addi r3, r10, -0x311c
	ctx.r[3].s64 = ctx.r[10].s64 + -12572;
	// 826D71DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D71E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D71E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D71E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D71EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D71F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D71F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D71F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D71FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7200: 4BD8FC21  bl 0x82466e20
	ctx.lr = 0x826D7204;
	sub_82466E20(ctx, base);
	// 826D7204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7218 size=108
    let mut pc: u32 = 0x826D7218;
    'dispatch: loop {
        match pc {
            0x826D7218 => {
    //   block [0x826D7218..0x826D7284)
	// 826D7218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D721C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7224: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7228: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D722C: 38EB37F8  addi r7, r11, 0x37f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14328;
	// 826D7230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7234: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826D7238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D723C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7248: 386ACF14  addi r3, r10, -0x30ec
	ctx.r[3].s64 = ctx.r[10].s64 + -12524;
	// 826D724C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D725C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D726C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7270: 4BD8FBB1  bl 0x82466e20
	ctx.lr = 0x826D7274;
	sub_82466E20(ctx, base);
	// 826D7274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D727C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7288 size=108
    let mut pc: u32 = 0x826D7288;
    'dispatch: loop {
        match pc {
            0x826D7288 => {
    //   block [0x826D7288..0x826D72F4)
	// 826D7288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D728C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7294: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7298: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D729C: 38EB3828  addi r7, r11, 0x3828
	ctx.r[7].s64 = ctx.r[11].s64 + 14376;
	// 826D72A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D72A4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826D72A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D72AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D72B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D72B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D72B8: 386ACF44  addi r3, r10, -0x30bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12476;
	// 826D72BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D72C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D72C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D72C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D72CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D72D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D72D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D72D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D72DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D72E0: 4BD8FB41  bl 0x82466e20
	ctx.lr = 0x826D72E4;
	sub_82466E20(ctx, base);
	// 826D72E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D72E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D72EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D72F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D72F8 size=24
    let mut pc: u32 = 0x826D72F8;
    'dispatch: loop {
        match pc {
            0x826D72F8 => {
    //   block [0x826D72F8..0x826D7310)
	// 826D72F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D72FC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7300: 394AAB80  addi r10, r10, -0x5480
	ctx.r[10].s64 = ctx.r[10].s64 + -21632;
	// 826D7304: 816B3840  lwz r11, 0x3840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14400 as u32) ) } as u64;
	// 826D7308: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7310 size=112
    let mut pc: u32 = 0x826D7310;
    'dispatch: loop {
        match pc {
            0x826D7310 => {
    //   block [0x826D7310..0x826D7380)
	// 826D7310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D731C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7320: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7324: 392A4D2C  addi r9, r10, 0x4d2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19756;
	// 826D7328: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D732C: 390BAB80  addi r8, r11, -0x5480
	ctx.r[8].s64 = ctx.r[11].s64 + -21632;
	// 826D7330: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D7334: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826D7338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D733C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7348: 386ACF74  addi r3, r10, -0x308c
	ctx.r[3].s64 = ctx.r[10].s64 + -12428;
	// 826D734C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D7354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D736C: 4BD8FAB5  bl 0x82466e20
	ctx.lr = 0x826D7370;
	sub_82466E20(ctx, base);
	// 826D7370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7380 size=96
    let mut pc: u32 = 0x826D7380;
    'dispatch: loop {
        match pc {
            0x826D7380 => {
    //   block [0x826D7380..0x826D73E0)
	// 826D7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D738C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7394: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826D7398: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D73A0: 386ACFA4  addi r3, r10, -0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + -12380;
	// 826D73A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D73A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D73AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D73B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D73B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D73B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D73BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D73C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D73C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D73C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D73CC: 4BD8FA55  bl 0x82466e20
	ctx.lr = 0x826D73D0;
	sub_82466E20(ctx, base);
	// 826D73D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D73D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D73D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D73DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D73E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D73E0 size=112
    let mut pc: u32 = 0x826D73E0;
    'dispatch: loop {
        match pc {
            0x826D73E0 => {
    //   block [0x826D73E0..0x826D7450)
	// 826D73E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D73E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D73E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D73EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D73F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D73F4: 38AACFA4  addi r5, r10, -0x305c
	ctx.r[5].s64 = ctx.r[10].s64 + -12380;
	// 826D73F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D73FC: 390B3844  addi r8, r11, 0x3844
	ctx.r[8].s64 = ctx.r[11].s64 + 14404;
	// 826D7400: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D7404: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826D7408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D740C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7418: 386ACFD4  addi r3, r10, -0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + -12332;
	// 826D741C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D7420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D742C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D743C: 4BD8F9E5  bl 0x82466e20
	ctx.lr = 0x826D7440;
	sub_82466E20(ctx, base);
	// 826D7440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D744C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7450 size=24
    let mut pc: u32 = 0x826D7450;
    'dispatch: loop {
        match pc {
            0x826D7450 => {
    //   block [0x826D7450..0x826D7468)
	// 826D7450: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7454: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7458: 394AAC58  addi r10, r10, -0x53a8
	ctx.r[10].s64 = ctx.r[10].s64 + -21416;
	// 826D745C: 816B3878  lwz r11, 0x3878(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14456 as u32) ) } as u64;
	// 826D7460: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826D7464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7468 size=112
    let mut pc: u32 = 0x826D7468;
    'dispatch: loop {
        match pc {
            0x826D7468 => {
    //   block [0x826D7468..0x826D74D8)
	// 826D7468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7474: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7478: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D747C: 392A4D50  addi r9, r10, 0x4d50
	ctx.r[9].s64 = ctx.r[10].s64 + 19792;
	// 826D7480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7484: 390BAC58  addi r8, r11, -0x53a8
	ctx.r[8].s64 = ctx.r[11].s64 + -21416;
	// 826D7488: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D748C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826D7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D74A0: 386AD004  addi r3, r10, -0x2ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -12284;
	// 826D74A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D74A8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D74AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D74B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D74BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D74C4: 4BD8F95D  bl 0x82466e20
	ctx.lr = 0x826D74C8;
	sub_82466E20(ctx, base);
	// 826D74C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D74CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D74D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D74D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D74D8 size=108
    let mut pc: u32 = 0x826D74D8;
    'dispatch: loop {
        match pc {
            0x826D74D8 => {
    //   block [0x826D74D8..0x826D7544)
	// 826D74D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D74DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D74E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D74E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D74E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D74EC: 38EB3880  addi r7, r11, 0x3880
	ctx.r[7].s64 = ctx.r[11].s64 + 14464;
	// 826D74F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D74F4: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826D74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D74FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7508: 386AD034  addi r3, r10, -0x2fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -12236;
	// 826D750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7530: 4BD8F8F1  bl 0x82466e20
	ctx.lr = 0x826D7534;
	sub_82466E20(ctx, base);
	// 826D7534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7548 size=24
    let mut pc: u32 = 0x826D7548;
    'dispatch: loop {
        match pc {
            0x826D7548 => {
    //   block [0x826D7548..0x826D7560)
	// 826D7548: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D754C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7550: 394AAD48  addi r10, r10, -0x52b8
	ctx.r[10].s64 = ctx.r[10].s64 + -21176;
	// 826D7554: 816B387C  lwz r11, 0x387c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14460 as u32) ) } as u64;
	// 826D7558: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826D755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7560 size=112
    let mut pc: u32 = 0x826D7560;
    'dispatch: loop {
        match pc {
            0x826D7560 => {
    //   block [0x826D7560..0x826D75D0)
	// 826D7560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D756C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7570: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7574: 392A4D80  addi r9, r10, 0x4d80
	ctx.r[9].s64 = ctx.r[10].s64 + 19840;
	// 826D7578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D757C: 390BAD48  addi r8, r11, -0x52b8
	ctx.r[8].s64 = ctx.r[11].s64 + -21176;
	// 826D7580: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826D7584: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826D7588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D758C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7598: 386AD064  addi r3, r10, -0x2f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -12188;
	// 826D759C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D75A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D75A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D75A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D75AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D75B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D75B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D75B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D75BC: 4BD8F865  bl 0x82466e20
	ctx.lr = 0x826D75C0;
	sub_82466E20(ctx, base);
	// 826D75C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D75C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D75C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D75CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D75D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D75D0 size=40
    let mut pc: u32 = 0x826D75D0;
    'dispatch: loop {
        match pc {
            0x826D75D0 => {
    //   block [0x826D75D0..0x826D75F8)
	// 826D75D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D75D4: 814B38B0  lwz r10, 0x38b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14512 as u32) ) } as u64;
	// 826D75D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D75DC: 396BADA8  addi r11, r11, -0x5258
	ctx.r[11].s64 = ctx.r[11].s64 + -21080;
	// 826D75E0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826D75E4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 826D75E8: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D75EC: 814A38B4  lwz r10, 0x38b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(14516 as u32) ) } as u64;
	// 826D75F0: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826D75F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D75F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D75F8 size=112
    let mut pc: u32 = 0x826D75F8;
    'dispatch: loop {
        match pc {
            0x826D75F8 => {
    //   block [0x826D75F8..0x826D7668)
	// 826D75F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D75FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7604: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7608: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D760C: 392A4F30  addi r9, r10, 0x4f30
	ctx.r[9].s64 = ctx.r[10].s64 + 20272;
	// 826D7610: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D7614: 390BADA8  addi r8, r11, -0x5258
	ctx.r[8].s64 = ctx.r[11].s64 + -21080;
	// 826D7618: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D761C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826D7620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7630: 386AD094  addi r3, r10, -0x2f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -12140;
	// 826D7634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7638: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826D763C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D764C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7654: 4BD8F7CD  bl 0x82466e20
	ctx.lr = 0x826D7658;
	sub_82466E20(ctx, base);
	// 826D7658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D765C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7668 size=108
    let mut pc: u32 = 0x826D7668;
    'dispatch: loop {
        match pc {
            0x826D7668 => {
    //   block [0x826D7668..0x826D76D4)
	// 826D7668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D766C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7674: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D767C: 38EB38C0  addi r7, r11, 0x38c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14528;
	// 826D7680: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7684: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 826D7688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D768C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7698: 386AD0C4  addi r3, r10, -0x2f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -12092;
	// 826D769C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D76A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D76A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D76A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D76AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D76B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D76B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D76B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D76BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D76C0: 4BD8F761  bl 0x82466e20
	ctx.lr = 0x826D76C4;
	sub_82466E20(ctx, base);
	// 826D76C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D76C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D76CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D76D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D76D8 size=108
    let mut pc: u32 = 0x826D76D8;
    'dispatch: loop {
        match pc {
            0x826D76D8 => {
    //   block [0x826D76D8..0x826D7744)
	// 826D76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D76E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D76E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D76EC: 38EB38F0  addi r7, r11, 0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + 14576;
	// 826D76F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D76F4: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 826D76F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D76FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7708: 386AD0F4  addi r3, r10, -0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -12044;
	// 826D770C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D771C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D772C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7730: 4BD8F6F1  bl 0x82466e20
	ctx.lr = 0x826D7734;
	sub_82466E20(ctx, base);
	// 826D7734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D773C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7748 size=108
    let mut pc: u32 = 0x826D7748;
    'dispatch: loop {
        match pc {
            0x826D7748 => {
    //   block [0x826D7748..0x826D77B4)
	// 826D7748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7754: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7758: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D775C: 38EB3908  addi r7, r11, 0x3908
	ctx.r[7].s64 = ctx.r[11].s64 + 14600;
	// 826D7760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7764: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826D7768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D776C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7778: 386AD124  addi r3, r10, -0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + -11996;
	// 826D777C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D778C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D779C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D77A0: 4BD8F681  bl 0x82466e20
	ctx.lr = 0x826D77A4;
	sub_82466E20(ctx, base);
	// 826D77A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D77A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D77AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D77B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


