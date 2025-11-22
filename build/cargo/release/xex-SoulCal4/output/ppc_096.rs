pub fn sub_8261C708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C708 size=112
    let mut pc: u32 = 0x8261C708;
    'dispatch: loop {
        match pc {
            0x8261C708 => {
    //   block [0x8261C708..0x8261C778)
	// 8261C708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C718: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C71C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C724: 390BCA78  addi r8, r11, -0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + -13704;
	// 8261C728: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C72C: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8261C730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C734: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C740: 386AE2AC  addi r3, r10, -0x1d54
	ctx.r[3].s64 = ctx.r[10].s64 + -7508;
	// 8261C744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C764: 4BE4A6BD  bl 0x82466e20
	ctx.lr = 0x8261C768;
	sub_82466E20(ctx, base);
	// 8261C768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C778 size=100
    let mut pc: u32 = 0x8261C778;
    'dispatch: loop {
        match pc {
            0x8261C778 => {
    //   block [0x8261C778..0x8261C7DC)
	// 8261C778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C784: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C78C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C798: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8261C79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C7AC: 386AE2DC  addi r3, r10, -0x1d24
	ctx.r[3].s64 = ctx.r[10].s64 + -7460;
	// 8261C7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C7B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C7B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C7C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C7C8: 4BE4A659  bl 0x82466e20
	ctx.lr = 0x8261C7CC;
	sub_82466E20(ctx, base);
	// 8261C7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C7E0 size=112
    let mut pc: u32 = 0x8261C7E0;
    'dispatch: loop {
        match pc {
            0x8261C7E0 => {
    //   block [0x8261C7E0..0x8261C850)
	// 8261C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C7EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C7F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C7F4: 38AAE2DC  addi r5, r10, -0x1d24
	ctx.r[5].s64 = ctx.r[10].s64 + -7460;
	// 8261C7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C7FC: 390BCAC0  addi r8, r11, -0x3540
	ctx.r[8].s64 = ctx.r[11].s64 + -13632;
	// 8261C800: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C804: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8261C808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C818: 386AE30C  addi r3, r10, -0x1cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -7412;
	// 8261C81C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C83C: 4BE4A5E5  bl 0x82466e20
	ctx.lr = 0x8261C840;
	sub_82466E20(ctx, base);
	// 8261C840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C850 size=112
    let mut pc: u32 = 0x8261C850;
    'dispatch: loop {
        match pc {
            0x8261C850 => {
    //   block [0x8261C850..0x8261C8C0)
	// 8261C850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C85C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C860: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C864: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C86C: 390BCB08  addi r8, r11, -0x34f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13560;
	// 8261C870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C874: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8261C878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C87C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C888: 386AE33C  addi r3, r10, -0x1cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -7364;
	// 8261C88C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C8AC: 4BE4A575  bl 0x82466e20
	ctx.lr = 0x8261C8B0;
	sub_82466E20(ctx, base);
	// 8261C8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C8C0 size=112
    let mut pc: u32 = 0x8261C8C0;
    'dispatch: loop {
        match pc {
            0x8261C8C0 => {
    //   block [0x8261C8C0..0x8261C930)
	// 8261C8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C8CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C8D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C8D4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C8DC: 390BCB20  addi r8, r11, -0x34e0
	ctx.r[8].s64 = ctx.r[11].s64 + -13536;
	// 8261C8E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C8E4: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8261C8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C8EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C8F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C8F8: 386AE36C  addi r3, r10, -0x1c94
	ctx.r[3].s64 = ctx.r[10].s64 + -7316;
	// 8261C8FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C90C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261C910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C91C: 4BE4A505  bl 0x82466e20
	ctx.lr = 0x8261C920;
	sub_82466E20(ctx, base);
	// 8261C920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C930 size=112
    let mut pc: u32 = 0x8261C930;
    'dispatch: loop {
        match pc {
            0x8261C930 => {
    //   block [0x8261C930..0x8261C9A0)
	// 8261C930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C93C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C940: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C944: 38AAE33C  addi r5, r10, -0x1cc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7364;
	// 8261C948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C94C: 390BCB38  addi r8, r11, -0x34c8
	ctx.r[8].s64 = ctx.r[11].s64 + -13512;
	// 8261C950: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C954: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8261C958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C95C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C968: 386AE39C  addi r3, r10, -0x1c64
	ctx.r[3].s64 = ctx.r[10].s64 + -7268;
	// 8261C96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C98C: 4BE4A495  bl 0x82466e20
	ctx.lr = 0x8261C990;
	sub_82466E20(ctx, base);
	// 8261C990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C9A0 size=72
    let mut pc: u32 = 0x8261C9A0;
    'dispatch: loop {
        match pc {
            0x8261C9A0 => {
    //   block [0x8261C9A0..0x8261C9E8)
	// 8261C9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C9AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261C9B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8261C9B4: 38CBED78  addi r6, r11, -0x1288
	ctx.r[6].s64 = ctx.r[11].s64 + -4744;
	// 8261C9B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261C9BC: 388B0698  addi r4, r11, 0x698
	ctx.r[4].s64 = ctx.r[11].s64 + 1688;
	// 8261C9C0: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261C9C4: 386BE3CC  addi r3, r11, -0x1c34
	ctx.r[3].s64 = ctx.r[11].s64 + -7220;
	// 8261C9C8: 4BE5F0C1  bl 0x8247ba88
	ctx.lr = 0x8261C9CC;
	sub_8247BA88(ctx, base);
	// 8261C9CC: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8261C9D0: 386BCD38  addi r3, r11, -0x32c8
	ctx.r[3].s64 = ctx.r[11].s64 + -13000;
	// 8261C9D4: 4BF16165  bl 0x82532b38
	ctx.lr = 0x8261C9D8;
	sub_82532B38(ctx, base);
	// 8261C9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8261C9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C9E8 size=108
    let mut pc: u32 = 0x8261C9E8;
    'dispatch: loop {
        match pc {
            0x8261C9E8 => {
    //   block [0x8261C9E8..0x8261CA54)
	// 8261C9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C9F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C9FC: 38EBEAA0  addi r7, r11, -0x1560
	ctx.r[7].s64 = ctx.r[11].s64 + -5472;
	// 8261CA00: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261CA04: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8261CA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CA0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CA10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CA18: 386AE3E4  addi r3, r10, -0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7196;
	// 8261CA1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CA3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CA40: 4BE4A3E1  bl 0x82466e20
	ctx.lr = 0x8261CA44;
	sub_82466E20(ctx, base);
	// 8261CA44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261CA58 size=24
    let mut pc: u32 = 0x8261CA58;
    'dispatch: loop {
        match pc {
            0x8261CA58 => {
    //   block [0x8261CA58..0x8261CA70)
	// 8261CA58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CA5C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261CA60: 394A3990  addi r10, r10, 0x3990
	ctx.r[10].s64 = ctx.r[10].s64 + 14736;
	// 8261CA64: 816BEB18  lwz r11, -0x14e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5352 as u32) ) } as u64;
	// 8261CA68: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8261CA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CA70 size=112
    let mut pc: u32 = 0x8261CA70;
    'dispatch: loop {
        match pc {
            0x8261CA70 => {
    //   block [0x8261CA70..0x8261CAE0)
	// 8261CA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CA7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261CA80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CA84: 392A1444  addi r9, r10, 0x1444
	ctx.r[9].s64 = ctx.r[10].s64 + 5188;
	// 8261CA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CA8C: 390B3990  addi r8, r11, 0x3990
	ctx.r[8].s64 = ctx.r[11].s64 + 14736;
	// 8261CA90: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261CA94: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8261CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CA9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CAA8: 386AE414  addi r3, r10, -0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + -7148;
	// 8261CAAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261CAB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CAC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CACC: 4BE4A355  bl 0x82466e20
	ctx.lr = 0x8261CAD0;
	sub_82466E20(ctx, base);
	// 8261CAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CAE0 size=108
    let mut pc: u32 = 0x8261CAE0;
    'dispatch: loop {
        match pc {
            0x8261CAE0 => {
    //   block [0x8261CAE0..0x8261CB4C)
	// 8261CAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CAEC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CAF4: 38EBEB1C  addi r7, r11, -0x14e4
	ctx.r[7].s64 = ctx.r[11].s64 + -5348;
	// 8261CAF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261CAFC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8261CB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CB04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CB08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CB10: 386AE444  addi r3, r10, -0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7100;
	// 8261CB14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CB38: 4BE4A2E9  bl 0x82466e20
	ctx.lr = 0x8261CB3C;
	sub_82466E20(ctx, base);
	// 8261CB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CB50 size=108
    let mut pc: u32 = 0x8261CB50;
    'dispatch: loop {
        match pc {
            0x8261CB50 => {
    //   block [0x8261CB50..0x8261CBBC)
	// 8261CB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CB5C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CB64: 38EBEB4C  addi r7, r11, -0x14b4
	ctx.r[7].s64 = ctx.r[11].s64 + -5300;
	// 8261CB68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261CB6C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8261CB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CB74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CB78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CB80: 386AE474  addi r3, r10, -0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7052;
	// 8261CB84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CBA8: 4BE4A279  bl 0x82466e20
	ctx.lr = 0x8261CBAC;
	sub_82466E20(ctx, base);
	// 8261CBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CBC0 size=112
    let mut pc: u32 = 0x8261CBC0;
    'dispatch: loop {
        match pc {
            0x8261CBC0 => {
    //   block [0x8261CBC0..0x8261CC30)
	// 8261CBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CBCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CBD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CBD4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261CBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CBDC: 390BEB80  addi r8, r11, -0x1480
	ctx.r[8].s64 = ctx.r[11].s64 + -5248;
	// 8261CBE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261CBE4: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8261CBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CBF8: 386AE4A4  addi r3, r10, -0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7004;
	// 8261CBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261CC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CC1C: 4BE4A205  bl 0x82466e20
	ctx.lr = 0x8261CC20;
	sub_82466E20(ctx, base);
	// 8261CC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CC30 size=108
    let mut pc: u32 = 0x8261CC30;
    'dispatch: loop {
        match pc {
            0x8261CC30 => {
    //   block [0x8261CC30..0x8261CC9C)
	// 8261CC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CC3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CC44: 38EBEBE0  addi r7, r11, -0x1420
	ctx.r[7].s64 = ctx.r[11].s64 + -5152;
	// 8261CC48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261CC4C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8261CC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CC54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CC58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CC60: 386AE4D4  addi r3, r10, -0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6956;
	// 8261CC64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CC84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CC88: 4BE4A199  bl 0x82466e20
	ctx.lr = 0x8261CC8C;
	sub_82466E20(ctx, base);
	// 8261CC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CCA0 size=112
    let mut pc: u32 = 0x8261CCA0;
    'dispatch: loop {
        match pc {
            0x8261CCA0 => {
    //   block [0x8261CCA0..0x8261CD10)
	// 8261CCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CCAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CCB0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CCB4: 38AAE4A4  addi r5, r10, -0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7004;
	// 8261CCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CCBC: 390BEC40  addi r8, r11, -0x13c0
	ctx.r[8].s64 = ctx.r[11].s64 + -5056;
	// 8261CCC0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8261CCC4: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8261CCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CCCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CCD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CCD8: 386AE504  addi r3, r10, -0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + -6908;
	// 8261CCDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261CCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CCFC: 4BE4A125  bl 0x82466e20
	ctx.lr = 0x8261CD00;
	sub_82466E20(ctx, base);
	// 8261CD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CD10 size=112
    let mut pc: u32 = 0x8261CD10;
    'dispatch: loop {
        match pc {
            0x8261CD10 => {
    //   block [0x8261CD10..0x8261CD80)
	// 8261CD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CD1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CD20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CD24: 38AAE4A4  addi r5, r10, -0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7004;
	// 8261CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CD2C: 390BECD0  addi r8, r11, -0x1330
	ctx.r[8].s64 = ctx.r[11].s64 + -4912;
	// 8261CD30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261CD34: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8261CD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CD3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CD48: 386AE534  addi r3, r10, -0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + -6860;
	// 8261CD4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261CD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CD6C: 4BE4A0B5  bl 0x82466e20
	ctx.lr = 0x8261CD70;
	sub_82466E20(ctx, base);
	// 8261CD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CD80 size=108
    let mut pc: u32 = 0x8261CD80;
    'dispatch: loop {
        match pc {
            0x8261CD80 => {
    //   block [0x8261CD80..0x8261CDEC)
	// 8261CD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CD8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CD94: 38EBECE8  addi r7, r11, -0x1318
	ctx.r[7].s64 = ctx.r[11].s64 + -4888;
	// 8261CD98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261CD9C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8261CDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CDA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CDB0: 386AE564  addi r3, r10, -0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -6812;
	// 8261CDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CDD8: 4BE4A049  bl 0x82466e20
	ctx.lr = 0x8261CDDC;
	sub_82466E20(ctx, base);
	// 8261CDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CDF0 size=112
    let mut pc: u32 = 0x8261CDF0;
    'dispatch: loop {
        match pc {
            0x8261CDF0 => {
    //   block [0x8261CDF0..0x8261CE60)
	// 8261CDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CDFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CE00: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CE04: 38AAE4A4  addi r5, r10, -0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7004;
	// 8261CE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CE0C: 390BED48  addi r8, r11, -0x12b8
	ctx.r[8].s64 = ctx.r[11].s64 + -4792;
	// 8261CE10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261CE14: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8261CE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CE1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CE28: 386AE594  addi r3, r10, -0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -6764;
	// 8261CE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261CE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CE4C: 4BE49FD5  bl 0x82466e20
	ctx.lr = 0x8261CE50;
	sub_82466E20(ctx, base);
	// 8261CE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CE60 size=108
    let mut pc: u32 = 0x8261CE60;
    'dispatch: loop {
        match pc {
            0x8261CE60 => {
    //   block [0x8261CE60..0x8261CECC)
	// 8261CE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CE6C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CE74: 38EBEDF0  addi r7, r11, -0x1210
	ctx.r[7].s64 = ctx.r[11].s64 + -4624;
	// 8261CE78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261CE7C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8261CE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CE90: 386AE5C4  addi r3, r10, -0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -6716;
	// 8261CE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CEB8: 4BE49F69  bl 0x82466e20
	ctx.lr = 0x8261CEBC;
	sub_82466E20(ctx, base);
	// 8261CEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CED0 size=108
    let mut pc: u32 = 0x8261CED0;
    'dispatch: loop {
        match pc {
            0x8261CED0 => {
    //   block [0x8261CED0..0x8261CF3C)
	// 8261CED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CEDC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CEE4: 38EBEE08  addi r7, r11, -0x11f8
	ctx.r[7].s64 = ctx.r[11].s64 + -4600;
	// 8261CEE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261CEEC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8261CEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CEF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CF00: 386AE5F4  addi r3, r10, -0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -6668;
	// 8261CF04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CF14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CF24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261CF28: 4BE49EF9  bl 0x82466e20
	ctx.lr = 0x8261CF2C;
	sub_82466E20(ctx, base);
	// 8261CF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CF40 size=112
    let mut pc: u32 = 0x8261CF40;
    'dispatch: loop {
        match pc {
            0x8261CF40 => {
    //   block [0x8261CF40..0x8261CFB0)
	// 8261CF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CF4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CF50: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CF54: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261CF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CF5C: 390BEE68  addi r8, r11, -0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + -4504;
	// 8261CF60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261CF64: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8261CF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CF6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CF70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261CF74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CF78: 386AE624  addi r3, r10, -0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6620;
	// 8261CF7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261CF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261CF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261CF9C: 4BE49E85  bl 0x82466e20
	ctx.lr = 0x8261CFA0;
	sub_82466E20(ctx, base);
	// 8261CFA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261CFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261CFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261CFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261CFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261CFB0 size=108
    let mut pc: u32 = 0x8261CFB0;
    'dispatch: loop {
        match pc {
            0x8261CFB0 => {
    //   block [0x8261CFB0..0x8261D01C)
	// 8261CFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261CFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261CFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261CFBC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261CFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261CFC4: 38EBEE80  addi r7, r11, -0x1180
	ctx.r[7].s64 = ctx.r[11].s64 + -4480;
	// 8261CFC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261CFCC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8261CFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261CFD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261CFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261CFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261CFE0: 386AE654  addi r3, r10, -0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6572;
	// 8261CFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261CFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261CFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261CFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261CFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261CFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261CFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D008: 4BE49E19  bl 0x82466e20
	ctx.lr = 0x8261D00C;
	sub_82466E20(ctx, base);
	// 8261D00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D020 size=108
    let mut pc: u32 = 0x8261D020;
    'dispatch: loop {
        match pc {
            0x8261D020 => {
    //   block [0x8261D020..0x8261D08C)
	// 8261D020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D02C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D034: 38EBEEC8  addi r7, r11, -0x1138
	ctx.r[7].s64 = ctx.r[11].s64 + -4408;
	// 8261D038: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261D03C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8261D040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D050: 386AE684  addi r3, r10, -0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + -6524;
	// 8261D054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D078: 4BE49DA9  bl 0x82466e20
	ctx.lr = 0x8261D07C;
	sub_82466E20(ctx, base);
	// 8261D07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D090 size=108
    let mut pc: u32 = 0x8261D090;
    'dispatch: loop {
        match pc {
            0x8261D090 => {
    //   block [0x8261D090..0x8261D0FC)
	// 8261D090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D09C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D0A4: 38EBEF58  addi r7, r11, -0x10a8
	ctx.r[7].s64 = ctx.r[11].s64 + -4264;
	// 8261D0A8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261D0AC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8261D0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D0B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D0B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D0C0: 386AE6B4  addi r3, r10, -0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + -6476;
	// 8261D0C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D0E8: 4BE49D39  bl 0x82466e20
	ctx.lr = 0x8261D0EC;
	sub_82466E20(ctx, base);
	// 8261D0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D100 size=100
    let mut pc: u32 = 0x8261D100;
    'dispatch: loop {
        match pc {
            0x8261D100 => {
    //   block [0x8261D100..0x8261D164)
	// 8261D100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D10C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D114: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261D118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D120: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8261D124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D134: 386AE6E4  addi r3, r10, -0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + -6428;
	// 8261D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D13C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D140: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261D144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D148: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261D14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D150: 4BE49CD1  bl 0x82466e20
	ctx.lr = 0x8261D154;
	sub_82466E20(ctx, base);
	// 8261D154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D168 size=112
    let mut pc: u32 = 0x8261D168;
    'dispatch: loop {
        match pc {
            0x8261D168 => {
    //   block [0x8261D168..0x8261D1D8)
	// 8261D168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D174: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D178: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D17C: 38AAE6E4  addi r5, r10, -0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + -6428;
	// 8261D180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D184: 390BEFE8  addi r8, r11, -0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + -4120;
	// 8261D188: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261D18C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8261D190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D1A0: 386AE714  addi r3, r10, -0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6380;
	// 8261D1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261D1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D1C4: 4BE49C5D  bl 0x82466e20
	ctx.lr = 0x8261D1C8;
	sub_82466E20(ctx, base);
	// 8261D1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D1D8 size=108
    let mut pc: u32 = 0x8261D1D8;
    'dispatch: loop {
        match pc {
            0x8261D1D8 => {
    //   block [0x8261D1D8..0x8261D244)
	// 8261D1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D1E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D1EC: 38EBF048  addi r7, r11, -0xfb8
	ctx.r[7].s64 = ctx.r[11].s64 + -4024;
	// 8261D1F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D1F4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8261D1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D1FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D208: 386AE744  addi r3, r10, -0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6332;
	// 8261D20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D230: 4BE49BF1  bl 0x82466e20
	ctx.lr = 0x8261D234;
	sub_82466E20(ctx, base);
	// 8261D234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D248 size=108
    let mut pc: u32 = 0x8261D248;
    'dispatch: loop {
        match pc {
            0x8261D248 => {
    //   block [0x8261D248..0x8261D2B4)
	// 8261D248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D254: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D25C: 38EBF078  addi r7, r11, -0xf88
	ctx.r[7].s64 = ctx.r[11].s64 + -3976;
	// 8261D260: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261D264: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8261D268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D26C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D278: 386AE774  addi r3, r10, -0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + -6284;
	// 8261D27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D2A0: 4BE49B81  bl 0x82466e20
	ctx.lr = 0x8261D2A4;
	sub_82466E20(ctx, base);
	// 8261D2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D2B8 size=108
    let mut pc: u32 = 0x8261D2B8;
    'dispatch: loop {
        match pc {
            0x8261D2B8 => {
    //   block [0x8261D2B8..0x8261D324)
	// 8261D2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D2C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D2CC: 38EBF0C0  addi r7, r11, -0xf40
	ctx.r[7].s64 = ctx.r[11].s64 + -3904;
	// 8261D2D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261D2D4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8261D2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D2DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D2E8: 386AE7A4  addi r3, r10, -0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + -6236;
	// 8261D2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D310: 4BE49B11  bl 0x82466e20
	ctx.lr = 0x8261D314;
	sub_82466E20(ctx, base);
	// 8261D314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D328 size=96
    let mut pc: u32 = 0x8261D328;
    'dispatch: loop {
        match pc {
            0x8261D328 => {
    //   block [0x8261D328..0x8261D388)
	// 8261D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D334: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D33C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8261D340: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D348: 386AE7D4  addi r3, r10, -0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + -6188;
	// 8261D34C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D354: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261D358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D368: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261D36C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D370: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261D374: 4BE49AAD  bl 0x82466e20
	ctx.lr = 0x8261D378;
	sub_82466E20(ctx, base);
	// 8261D378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D388 size=112
    let mut pc: u32 = 0x8261D388;
    'dispatch: loop {
        match pc {
            0x8261D388 => {
    //   block [0x8261D388..0x8261D3F8)
	// 8261D388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D398: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D39C: 38AAE7D4  addi r5, r10, -0x182c
	ctx.r[5].s64 = ctx.r[10].s64 + -6188;
	// 8261D3A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D3A4: 390BF120  addi r8, r11, -0xee0
	ctx.r[8].s64 = ctx.r[11].s64 + -3808;
	// 8261D3A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261D3AC: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8261D3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D3B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D3C0: 386AE804  addi r3, r10, -0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6140;
	// 8261D3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261D3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D3E4: 4BE49A3D  bl 0x82466e20
	ctx.lr = 0x8261D3E8;
	sub_82466E20(ctx, base);
	// 8261D3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D3F8 size=112
    let mut pc: u32 = 0x8261D3F8;
    'dispatch: loop {
        match pc {
            0x8261D3F8 => {
    //   block [0x8261D3F8..0x8261D468)
	// 8261D3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D404: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261D408: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D40C: 392A1460  addi r9, r10, 0x1460
	ctx.r[9].s64 = ctx.r[10].s64 + 5216;
	// 8261D410: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D414: 390BF150  addi r8, r11, -0xeb0
	ctx.r[8].s64 = ctx.r[11].s64 + -3760;
	// 8261D418: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261D41C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8261D420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D430: 386AE834  addi r3, r10, -0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6092;
	// 8261D434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261D438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261D43C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D454: 4BE499CD  bl 0x82466e20
	ctx.lr = 0x8261D458;
	sub_82466E20(ctx, base);
	// 8261D458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D468 size=108
    let mut pc: u32 = 0x8261D468;
    'dispatch: loop {
        match pc {
            0x8261D468 => {
    //   block [0x8261D468..0x8261D4D4)
	// 8261D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D474: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D478: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D47C: 38EBF1F8  addi r7, r11, -0xe08
	ctx.r[7].s64 = ctx.r[11].s64 + -3592;
	// 8261D480: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D484: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8261D488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D48C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D498: 386AE864  addi r3, r10, -0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + -6044;
	// 8261D49C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D4C0: 4BE49961  bl 0x82466e20
	ctx.lr = 0x8261D4C4;
	sub_82466E20(ctx, base);
	// 8261D4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D4D8 size=108
    let mut pc: u32 = 0x8261D4D8;
    'dispatch: loop {
        match pc {
            0x8261D4D8 => {
    //   block [0x8261D4D8..0x8261D544)
	// 8261D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D4E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D4E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D4EC: 38EBF228  addi r7, r11, -0xdd8
	ctx.r[7].s64 = ctx.r[11].s64 + -3544;
	// 8261D4F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D4F4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8261D4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D4FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D508: 386AE894  addi r3, r10, -0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + -5996;
	// 8261D50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D530: 4BE498F1  bl 0x82466e20
	ctx.lr = 0x8261D534;
	sub_82466E20(ctx, base);
	// 8261D534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261D548 size=28
    let mut pc: u32 = 0x8261D548;
    'dispatch: loop {
        match pc {
            0x8261D548 => {
    //   block [0x8261D548..0x8261D564)
	// 8261D548: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D54C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261D550: 394A39D8  addi r10, r10, 0x39d8
	ctx.r[10].s64 = ctx.r[10].s64 + 14808;
	// 8261D554: 816BF258  lwz r11, -0xda8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3496 as u32) ) } as u64;
	// 8261D558: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8261D55C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261D560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D568 size=112
    let mut pc: u32 = 0x8261D568;
    'dispatch: loop {
        match pc {
            0x8261D568 => {
    //   block [0x8261D568..0x8261D5D8)
	// 8261D568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261D578: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D57C: 392A15D0  addi r9, r10, 0x15d0
	ctx.r[9].s64 = ctx.r[10].s64 + 5584;
	// 8261D580: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D584: 390B39D8  addi r8, r11, 0x39d8
	ctx.r[8].s64 = ctx.r[11].s64 + 14808;
	// 8261D588: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261D58C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8261D590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D5A0: 386AE8C4  addi r3, r10, -0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + -5948;
	// 8261D5A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261D5A8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8261D5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D5B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D5BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D5C4: 4BE4985D  bl 0x82466e20
	ctx.lr = 0x8261D5C8;
	sub_82466E20(ctx, base);
	// 8261D5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D5D8 size=108
    let mut pc: u32 = 0x8261D5D8;
    'dispatch: loop {
        match pc {
            0x8261D5D8 => {
    //   block [0x8261D5D8..0x8261D644)
	// 8261D5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D5E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D5E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D5EC: 38EBF264  addi r7, r11, -0xd9c
	ctx.r[7].s64 = ctx.r[11].s64 + -3484;
	// 8261D5F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D5F4: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8261D5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D5FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D608: 386AE8F4  addi r3, r10, -0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + -5900;
	// 8261D60C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D630: 4BE497F1  bl 0x82466e20
	ctx.lr = 0x8261D634;
	sub_82466E20(ctx, base);
	// 8261D634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D648 size=108
    let mut pc: u32 = 0x8261D648;
    'dispatch: loop {
        match pc {
            0x8261D648 => {
    //   block [0x8261D648..0x8261D6B4)
	// 8261D648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D654: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D658: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D65C: 38EBF294  addi r7, r11, -0xd6c
	ctx.r[7].s64 = ctx.r[11].s64 + -3436;
	// 8261D660: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261D664: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8261D668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D66C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D678: 386AE924  addi r3, r10, -0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5852;
	// 8261D67C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D6A0: 4BE49781  bl 0x82466e20
	ctx.lr = 0x8261D6A4;
	sub_82466E20(ctx, base);
	// 8261D6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261D6B8 size=24
    let mut pc: u32 = 0x8261D6B8;
    'dispatch: loop {
        match pc {
            0x8261D6B8 => {
    //   block [0x8261D6B8..0x8261D6D0)
	// 8261D6B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D6BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261D6C0: 394A3A98  addi r10, r10, 0x3a98
	ctx.r[10].s64 = ctx.r[10].s64 + 15000;
	// 8261D6C4: 816BF2AC  lwz r11, -0xd54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3412 as u32) ) } as u64;
	// 8261D6C8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8261D6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D6D0 size=112
    let mut pc: u32 = 0x8261D6D0;
    'dispatch: loop {
        match pc {
            0x8261D6D0 => {
    //   block [0x8261D6D0..0x8261D740)
	// 8261D6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D6DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261D6E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D6E4: 392A1624  addi r9, r10, 0x1624
	ctx.r[9].s64 = ctx.r[10].s64 + 5668;
	// 8261D6E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D6EC: 390B3A98  addi r8, r11, 0x3a98
	ctx.r[8].s64 = ctx.r[11].s64 + 15000;
	// 8261D6F0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8261D6F4: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8261D6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D6FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D708: 386AE954  addi r3, r10, -0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5804;
	// 8261D70C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261D710: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261D714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D72C: 4BE496F5  bl 0x82466e20
	ctx.lr = 0x8261D730;
	sub_82466E20(ctx, base);
	// 8261D730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D740 size=108
    let mut pc: u32 = 0x8261D740;
    'dispatch: loop {
        match pc {
            0x8261D740 => {
    //   block [0x8261D740..0x8261D7AC)
	// 8261D740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D74C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D750: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D754: 38EBF2B0  addi r7, r11, -0xd50
	ctx.r[7].s64 = ctx.r[11].s64 + -3408;
	// 8261D758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D75C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8261D760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D770: 386AE984  addi r3, r10, -0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + -5756;
	// 8261D774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D798: 4BE49689  bl 0x82466e20
	ctx.lr = 0x8261D79C;
	sub_82466E20(ctx, base);
	// 8261D79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D7B0 size=108
    let mut pc: u32 = 0x8261D7B0;
    'dispatch: loop {
        match pc {
            0x8261D7B0 => {
    //   block [0x8261D7B0..0x8261D81C)
	// 8261D7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D7BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D7C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D7C4: 38EBF2E0  addi r7, r11, -0xd20
	ctx.r[7].s64 = ctx.r[11].s64 + -3360;
	// 8261D7C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261D7CC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8261D7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D7D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D7E0: 386AE9B4  addi r3, r10, -0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + -5708;
	// 8261D7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D808: 4BE49619  bl 0x82466e20
	ctx.lr = 0x8261D80C;
	sub_82466E20(ctx, base);
	// 8261D80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D820 size=112
    let mut pc: u32 = 0x8261D820;
    'dispatch: loop {
        match pc {
            0x8261D820 => {
    //   block [0x8261D820..0x8261D890)
	// 8261D820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D82C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261D830: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D834: 392A1648  addi r9, r10, 0x1648
	ctx.r[9].s64 = ctx.r[10].s64 + 5704;
	// 8261D838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D83C: 390BF318  addi r8, r11, -0xce8
	ctx.r[8].s64 = ctx.r[11].s64 + -3304;
	// 8261D840: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8261D844: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8261D848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D84C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261D854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D858: 386AE9E4  addi r3, r10, -0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + -5660;
	// 8261D85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261D860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261D864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D87C: 4BE495A5  bl 0x82466e20
	ctx.lr = 0x8261D880;
	sub_82466E20(ctx, base);
	// 8261D880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D890 size=108
    let mut pc: u32 = 0x8261D890;
    'dispatch: loop {
        match pc {
            0x8261D890 => {
    //   block [0x8261D890..0x8261D8FC)
	// 8261D890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D89C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D8A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D8A4: 38EBF378  addi r7, r11, -0xc88
	ctx.r[7].s64 = ctx.r[11].s64 + -3208;
	// 8261D8A8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8261D8AC: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8261D8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D8B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D8C0: 386AEA14  addi r3, r10, -0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5612;
	// 8261D8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D8DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D8E8: 4BE49539  bl 0x82466e20
	ctx.lr = 0x8261D8EC;
	sub_82466E20(ctx, base);
	// 8261D8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D900 size=108
    let mut pc: u32 = 0x8261D900;
    'dispatch: loop {
        match pc {
            0x8261D900 => {
    //   block [0x8261D900..0x8261D96C)
	// 8261D900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D90C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D910: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261D914: 38EBF438  addi r7, r11, -0xbc8
	ctx.r[7].s64 = ctx.r[11].s64 + -3016;
	// 8261D918: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261D91C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8261D920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D924: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D930: 386AEA44  addi r3, r10, -0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5564;
	// 8261D934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D958: 4BE494C9  bl 0x82466e20
	ctx.lr = 0x8261D95C;
	sub_82466E20(ctx, base);
	// 8261D95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D970 size=108
    let mut pc: u32 = 0x8261D970;
    'dispatch: loop {
        match pc {
            0x8261D970 => {
    //   block [0x8261D970..0x8261D9DC)
	// 8261D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261D978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261D97C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261D984: 38EBF450  addi r7, r11, -0xbb0
	ctx.r[7].s64 = ctx.r[11].s64 + -2992;
	// 8261D988: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261D98C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8261D990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261D994: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261D998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261D99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261D9A0: 386AEA74  addi r3, r10, -0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + -5516;
	// 8261D9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261D9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261D9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261D9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261D9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261D9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261D9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261D9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261D9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261D9C8: 4BE49459  bl 0x82466e20
	ctx.lr = 0x8261D9CC;
	sub_82466E20(ctx, base);
	// 8261D9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261D9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261D9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261D9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261D9E0 size=24
    let mut pc: u32 = 0x8261D9E0;
    'dispatch: loop {
        match pc {
            0x8261D9E0 => {
    //   block [0x8261D9E0..0x8261D9F8)
	// 8261D9E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261D9E4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261D9E8: 394A3B28  addi r10, r10, 0x3b28
	ctx.r[10].s64 = ctx.r[10].s64 + 15144;
	// 8261D9EC: 816BF314  lwz r11, -0xcec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3308 as u32) ) } as u64;
	// 8261D9F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8261D9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261D9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261D9F8 size=108
    let mut pc: u32 = 0x8261D9F8;
    'dispatch: loop {
        match pc {
            0x8261D9F8 => {
    //   block [0x8261D9F8..0x8261DA64)
	// 8261D9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261D9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DA04: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DA0C: 38EB3B28  addi r7, r11, 0x3b28
	ctx.r[7].s64 = ctx.r[11].s64 + 15144;
	// 8261DA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DA14: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8261DA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DA1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DA28: 386AEAA4  addi r3, r10, -0x155c
	ctx.r[3].s64 = ctx.r[10].s64 + -5468;
	// 8261DA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DA50: 4BE493D1  bl 0x82466e20
	ctx.lr = 0x8261DA54;
	sub_82466E20(ctx, base);
	// 8261DA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261DA68 size=24
    let mut pc: u32 = 0x8261DA68;
    'dispatch: loop {
        match pc {
            0x8261DA68 => {
    //   block [0x8261DA68..0x8261DA80)
	// 8261DA68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DA6C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261DA70: 394A3B58  addi r10, r10, 0x3b58
	ctx.r[10].s64 = ctx.r[10].s64 + 15192;
	// 8261DA74: 816BF314  lwz r11, -0xcec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3308 as u32) ) } as u64;
	// 8261DA78: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8261DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DA80 size=108
    let mut pc: u32 = 0x8261DA80;
    'dispatch: loop {
        match pc {
            0x8261DA80 => {
    //   block [0x8261DA80..0x8261DAEC)
	// 8261DA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DA8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DA94: 38EB3B58  addi r7, r11, 0x3b58
	ctx.r[7].s64 = ctx.r[11].s64 + 15192;
	// 8261DA98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DA9C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8261DAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DAA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DAB0: 386AEAD4  addi r3, r10, -0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + -5420;
	// 8261DAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DAD8: 4BE49349  bl 0x82466e20
	ctx.lr = 0x8261DADC;
	sub_82466E20(ctx, base);
	// 8261DADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DAF0 size=108
    let mut pc: u32 = 0x8261DAF0;
    'dispatch: loop {
        match pc {
            0x8261DAF0 => {
    //   block [0x8261DAF0..0x8261DB5C)
	// 8261DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DAFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DB04: 38EBF4C8  addi r7, r11, -0xb38
	ctx.r[7].s64 = ctx.r[11].s64 + -2872;
	// 8261DB08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261DB0C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8261DB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DB14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DB20: 386AEB04  addi r3, r10, -0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5372;
	// 8261DB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DB48: 4BE492D9  bl 0x82466e20
	ctx.lr = 0x8261DB4C;
	sub_82466E20(ctx, base);
	// 8261DB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261DB60 size=24
    let mut pc: u32 = 0x8261DB60;
    'dispatch: loop {
        match pc {
            0x8261DB60 => {
    //   block [0x8261DB60..0x8261DB78)
	// 8261DB60: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DB64: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261DB68: 394A3B88  addi r10, r10, 0x3b88
	ctx.r[10].s64 = ctx.r[10].s64 + 15240;
	// 8261DB6C: 816BF314  lwz r11, -0xcec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3308 as u32) ) } as u64;
	// 8261DB70: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8261DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DB78 size=108
    let mut pc: u32 = 0x8261DB78;
    'dispatch: loop {
        match pc {
            0x8261DB78 => {
    //   block [0x8261DB78..0x8261DBE4)
	// 8261DB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DB84: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DB8C: 38EB3B88  addi r7, r11, 0x3b88
	ctx.r[7].s64 = ctx.r[11].s64 + 15240;
	// 8261DB90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DB94: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8261DB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DB9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DBA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DBA8: 386AEB34  addi r3, r10, -0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5324;
	// 8261DBAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DBCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DBD0: 4BE49251  bl 0x82466e20
	ctx.lr = 0x8261DBD4;
	sub_82466E20(ctx, base);
	// 8261DBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DBE8 size=112
    let mut pc: u32 = 0x8261DBE8;
    'dispatch: loop {
        match pc {
            0x8261DBE8 => {
    //   block [0x8261DBE8..0x8261DC58)
	// 8261DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DBF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261DBF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DBFC: 392A168C  addi r9, r10, 0x168c
	ctx.r[9].s64 = ctx.r[10].s64 + 5772;
	// 8261DC00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DC04: 390BF4E0  addi r8, r11, -0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + -2848;
	// 8261DC08: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8261DC0C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8261DC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DC14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DC18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261DC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DC20: 386AEB64  addi r3, r10, -0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + -5276;
	// 8261DC24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261DC28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261DC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DC30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DC34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DC38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DC3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DC40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DC44: 4BE491DD  bl 0x82466e20
	ctx.lr = 0x8261DC48;
	sub_82466E20(ctx, base);
	// 8261DC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DC58 size=108
    let mut pc: u32 = 0x8261DC58;
    'dispatch: loop {
        match pc {
            0x8261DC58 => {
    //   block [0x8261DC58..0x8261DCC4)
	// 8261DC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DC64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DC6C: 38EBF510  addi r7, r11, -0xaf0
	ctx.r[7].s64 = ctx.r[11].s64 + -2800;
	// 8261DC70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DC74: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8261DC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DC7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DC80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DC88: 386AEB94  addi r3, r10, -0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + -5228;
	// 8261DC8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DCA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DCAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DCB0: 4BE49171  bl 0x82466e20
	ctx.lr = 0x8261DCB4;
	sub_82466E20(ctx, base);
	// 8261DCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DCC8 size=108
    let mut pc: u32 = 0x8261DCC8;
    'dispatch: loop {
        match pc {
            0x8261DCC8 => {
    //   block [0x8261DCC8..0x8261DD34)
	// 8261DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DCD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DCDC: 38EBF540  addi r7, r11, -0xac0
	ctx.r[7].s64 = ctx.r[11].s64 + -2752;
	// 8261DCE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DCE4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8261DCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DCEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DCF8: 386AEBC4  addi r3, r10, -0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + -5180;
	// 8261DCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DD20: 4BE49101  bl 0x82466e20
	ctx.lr = 0x8261DD24;
	sub_82466E20(ctx, base);
	// 8261DD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DD38 size=112
    let mut pc: u32 = 0x8261DD38;
    'dispatch: loop {
        match pc {
            0x8261DD38 => {
    //   block [0x8261DD38..0x8261DDA8)
	// 8261DD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DD44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DD48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DD4C: 38AAEC24  addi r5, r10, -0x13dc
	ctx.r[5].s64 = ctx.r[10].s64 + -5084;
	// 8261DD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DD54: 390BF570  addi r8, r11, -0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + -2704;
	// 8261DD58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261DD5C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8261DD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DD64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DD68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261DD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DD70: 386AEBF4  addi r3, r10, -0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + -5132;
	// 8261DD74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261DD78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DD8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DD94: 4BE4908D  bl 0x82466e20
	ctx.lr = 0x8261DD98;
	sub_82466E20(ctx, base);
	// 8261DD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DDA8 size=108
    let mut pc: u32 = 0x8261DDA8;
    'dispatch: loop {
        match pc {
            0x8261DDA8 => {
    //   block [0x8261DDA8..0x8261DE14)
	// 8261DDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DDB4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DDB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DDBC: 38EBF588  addi r7, r11, -0xa78
	ctx.r[7].s64 = ctx.r[11].s64 + -2680;
	// 8261DDC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DDC4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8261DDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DDCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DDD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DDD8: 386AEC24  addi r3, r10, -0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5084;
	// 8261DDDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DDE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DDE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DDF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DDF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DDFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DE00: 4BE49021  bl 0x82466e20
	ctx.lr = 0x8261DE04;
	sub_82466E20(ctx, base);
	// 8261DE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DE18 size=108
    let mut pc: u32 = 0x8261DE18;
    'dispatch: loop {
        match pc {
            0x8261DE18 => {
    //   block [0x8261DE18..0x8261DE84)
	// 8261DE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DE24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DE2C: 38EBF5B8  addi r7, r11, -0xa48
	ctx.r[7].s64 = ctx.r[11].s64 + -2632;
	// 8261DE30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261DE34: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8261DE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DE3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DE48: 386AEC54  addi r3, r10, -0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5036;
	// 8261DE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DE70: 4BE48FB1  bl 0x82466e20
	ctx.lr = 0x8261DE74;
	sub_82466E20(ctx, base);
	// 8261DE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DE88 size=108
    let mut pc: u32 = 0x8261DE88;
    'dispatch: loop {
        match pc {
            0x8261DE88 => {
    //   block [0x8261DE88..0x8261DEF4)
	// 8261DE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DE94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DE98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DE9C: 38EBF5D0  addi r7, r11, -0xa30
	ctx.r[7].s64 = ctx.r[11].s64 + -2608;
	// 8261DEA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DEA4: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8261DEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DEAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DEB8: 386AEC84  addi r3, r10, -0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + -4988;
	// 8261DEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DEE0: 4BE48F41  bl 0x82466e20
	ctx.lr = 0x8261DEE4;
	sub_82466E20(ctx, base);
	// 8261DEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DEF8 size=108
    let mut pc: u32 = 0x8261DEF8;
    'dispatch: loop {
        match pc {
            0x8261DEF8 => {
    //   block [0x8261DEF8..0x8261DF64)
	// 8261DEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DF04: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DF0C: 38EBF600  addi r7, r11, -0xa00
	ctx.r[7].s64 = ctx.r[11].s64 + -2560;
	// 8261DF10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8261DF14: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8261DF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DF1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DF28: 386AECB4  addi r3, r10, -0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + -4940;
	// 8261DF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DF50: 4BE48ED1  bl 0x82466e20
	ctx.lr = 0x8261DF54;
	sub_82466E20(ctx, base);
	// 8261DF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DF68 size=108
    let mut pc: u32 = 0x8261DF68;
    'dispatch: loop {
        match pc {
            0x8261DF68 => {
    //   block [0x8261DF68..0x8261DFD4)
	// 8261DF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DF74: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DF7C: 38EBF6A8  addi r7, r11, -0x958
	ctx.r[7].s64 = ctx.r[11].s64 + -2392;
	// 8261DF80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261DF84: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8261DF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DF8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261DF90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261DF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261DF98: 386AECE4  addi r3, r10, -0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + -4892;
	// 8261DF9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261DFA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261DFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261DFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261DFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261DFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261DFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261DFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261DFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261DFC0: 4BE48E61  bl 0x82466e20
	ctx.lr = 0x8261DFC4;
	sub_82466E20(ctx, base);
	// 8261DFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261DFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261DFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261DFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261DFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261DFD8 size=108
    let mut pc: u32 = 0x8261DFD8;
    'dispatch: loop {
        match pc {
            0x8261DFD8 => {
    //   block [0x8261DFD8..0x8261E044)
	// 8261DFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261DFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261DFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261DFE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261DFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261DFEC: 38EBF6D8  addi r7, r11, -0x928
	ctx.r[7].s64 = ctx.r[11].s64 + -2344;
	// 8261DFF0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8261DFF4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8261DFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261DFFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E008: 386AED14  addi r3, r10, -0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4844;
	// 8261E00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E030: 4BE48DF1  bl 0x82466e20
	ctx.lr = 0x8261E034;
	sub_82466E20(ctx, base);
	// 8261E034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261E048 size=24
    let mut pc: u32 = 0x8261E048;
    'dispatch: loop {
        match pc {
            0x8261E048 => {
    //   block [0x8261E048..0x8261E060)
	// 8261E048: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E04C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261E050: 394A3BB8  addi r10, r10, 0x3bb8
	ctx.r[10].s64 = ctx.r[10].s64 + 15288;
	// 8261E054: 816BF798  lwz r11, -0x868(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2152 as u32) ) } as u64;
	// 8261E058: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261E05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E060 size=112
    let mut pc: u32 = 0x8261E060;
    'dispatch: loop {
        match pc {
            0x8261E060 => {
    //   block [0x8261E060..0x8261E0D0)
	// 8261E060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E06C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261E070: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E074: 392A16B8  addi r9, r10, 0x16b8
	ctx.r[9].s64 = ctx.r[10].s64 + 5816;
	// 8261E078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E07C: 390B3BB8  addi r8, r11, 0x3bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 15288;
	// 8261E080: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261E084: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8261E088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E08C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261E094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E098: 386AED44  addi r3, r10, -0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + -4796;
	// 8261E09C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261E0A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261E0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E0BC: 4BE48D65  bl 0x82466e20
	ctx.lr = 0x8261E0C0;
	sub_82466E20(ctx, base);
	// 8261E0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E0D0 size=108
    let mut pc: u32 = 0x8261E0D0;
    'dispatch: loop {
        match pc {
            0x8261E0D0 => {
    //   block [0x8261E0D0..0x8261E13C)
	// 8261E0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E0DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E0E4: 38EBF7A0  addi r7, r11, -0x860
	ctx.r[7].s64 = ctx.r[11].s64 + -2144;
	// 8261E0E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261E0EC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8261E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E100: 386AED74  addi r3, r10, -0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + -4748;
	// 8261E104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E128: 4BE48CF9  bl 0x82466e20
	ctx.lr = 0x8261E12C;
	sub_82466E20(ctx, base);
	// 8261E12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E140 size=112
    let mut pc: u32 = 0x8261E140;
    'dispatch: loop {
        match pc {
            0x8261E140 => {
    //   block [0x8261E140..0x8261E1B0)
	// 8261E140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E14C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261E150: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E154: 392A16FC  addi r9, r10, 0x16fc
	ctx.r[9].s64 = ctx.r[10].s64 + 5884;
	// 8261E158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E15C: 390BF7D0  addi r8, r11, -0x830
	ctx.r[8].s64 = ctx.r[11].s64 + -2096;
	// 8261E160: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261E164: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8261E168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261E174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E178: 386AEDA4  addi r3, r10, -0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + -4700;
	// 8261E17C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261E180: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261E184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E19C: 4BE48C85  bl 0x82466e20
	ctx.lr = 0x8261E1A0;
	sub_82466E20(ctx, base);
	// 8261E1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261E1B0 size=24
    let mut pc: u32 = 0x8261E1B0;
    'dispatch: loop {
        match pc {
            0x8261E1B0 => {
    //   block [0x8261E1B0..0x8261E1C8)
	// 8261E1B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E1B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261E1B8: 394A3C30  addi r10, r10, 0x3c30
	ctx.r[10].s64 = ctx.r[10].s64 + 15408;
	// 8261E1BC: 816BF890  lwz r11, -0x770(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1904 as u32) ) } as u64;
	// 8261E1C0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8261E1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E1C8 size=112
    let mut pc: u32 = 0x8261E1C8;
    'dispatch: loop {
        match pc {
            0x8261E1C8 => {
    //   block [0x8261E1C8..0x8261E238)
	// 8261E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E1D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261E1D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E1DC: 392A1738  addi r9, r10, 0x1738
	ctx.r[9].s64 = ctx.r[10].s64 + 5944;
	// 8261E1E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E1E4: 390B3C30  addi r8, r11, 0x3c30
	ctx.r[8].s64 = ctx.r[11].s64 + 15408;
	// 8261E1E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261E1EC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8261E1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E1F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E200: 386AEDD4  addi r3, r10, -0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + -4652;
	// 8261E204: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261E208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261E20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E224: 4BE48BFD  bl 0x82466e20
	ctx.lr = 0x8261E228;
	sub_82466E20(ctx, base);
	// 8261E228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E238 size=108
    let mut pc: u32 = 0x8261E238;
    'dispatch: loop {
        match pc {
            0x8261E238 => {
    //   block [0x8261E238..0x8261E2A4)
	// 8261E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E244: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E24C: 38EBF894  addi r7, r11, -0x76c
	ctx.r[7].s64 = ctx.r[11].s64 + -1900;
	// 8261E250: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261E254: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8261E258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E268: 386AEE04  addi r3, r10, -0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4604;
	// 8261E26C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E28C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E290: 4BE48B91  bl 0x82466e20
	ctx.lr = 0x8261E294;
	sub_82466E20(ctx, base);
	// 8261E294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E2A8 size=108
    let mut pc: u32 = 0x8261E2A8;
    'dispatch: loop {
        match pc {
            0x8261E2A8 => {
    //   block [0x8261E2A8..0x8261E314)
	// 8261E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E2B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E2BC: 38EBF8AC  addi r7, r11, -0x754
	ctx.r[7].s64 = ctx.r[11].s64 + -1876;
	// 8261E2C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261E2C4: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8261E2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E2CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E2D8: 386AEE34  addi r3, r10, -0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4556;
	// 8261E2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E300: 4BE48B21  bl 0x82466e20
	ctx.lr = 0x8261E304;
	sub_82466E20(ctx, base);
	// 8261E304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261E318 size=24
    let mut pc: u32 = 0x8261E318;
    'dispatch: loop {
        match pc {
            0x8261E318 => {
    //   block [0x8261E318..0x8261E330)
	// 8261E318: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E31C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261E320: 394A3C78  addi r10, r10, 0x3c78
	ctx.r[10].s64 = ctx.r[10].s64 + 15480;
	// 8261E324: 816BF8DC  lwz r11, -0x724(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1828 as u32) ) } as u64;
	// 8261E328: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261E32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E330 size=112
    let mut pc: u32 = 0x8261E330;
    'dispatch: loop {
        match pc {
            0x8261E330 => {
    //   block [0x8261E330..0x8261E3A0)
	// 8261E330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E33C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261E340: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E344: 392A1774  addi r9, r10, 0x1774
	ctx.r[9].s64 = ctx.r[10].s64 + 6004;
	// 8261E348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E34C: 390B3C78  addi r8, r11, 0x3c78
	ctx.r[8].s64 = ctx.r[11].s64 + 15480;
	// 8261E350: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261E354: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8261E358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E35C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261E364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E368: 386AEE64  addi r3, r10, -0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + -4508;
	// 8261E36C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261E370: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261E374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E38C: 4BE48A95  bl 0x82466e20
	ctx.lr = 0x8261E390;
	sub_82466E20(ctx, base);
	// 8261E390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E3A0 size=108
    let mut pc: u32 = 0x8261E3A0;
    'dispatch: loop {
        match pc {
            0x8261E3A0 => {
    //   block [0x8261E3A0..0x8261E40C)
	// 8261E3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E3AC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E3B4: 38EBF8E0  addi r7, r11, -0x720
	ctx.r[7].s64 = ctx.r[11].s64 + -1824;
	// 8261E3B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261E3BC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8261E3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E3C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E3C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E3D0: 386AEE94  addi r3, r10, -0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + -4460;
	// 8261E3D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E3F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E3F8: 4BE48A29  bl 0x82466e20
	ctx.lr = 0x8261E3FC;
	sub_82466E20(ctx, base);
	// 8261E3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E410 size=108
    let mut pc: u32 = 0x8261E410;
    'dispatch: loop {
        match pc {
            0x8261E410 => {
    //   block [0x8261E410..0x8261E47C)
	// 8261E410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E41C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E424: 38EBF8F8  addi r7, r11, -0x708
	ctx.r[7].s64 = ctx.r[11].s64 + -1800;
	// 8261E428: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261E42C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8261E430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E440: 386AEEC4  addi r3, r10, -0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + -4412;
	// 8261E444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E468: 4BE489B9  bl 0x82466e20
	ctx.lr = 0x8261E46C;
	sub_82466E20(ctx, base);
	// 8261E46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E480 size=108
    let mut pc: u32 = 0x8261E480;
    'dispatch: loop {
        match pc {
            0x8261E480 => {
    //   block [0x8261E480..0x8261E4EC)
	// 8261E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E48C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E494: 38EBF940  addi r7, r11, -0x6c0
	ctx.r[7].s64 = ctx.r[11].s64 + -1728;
	// 8261E498: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261E49C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8261E4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E4A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E4A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E4B0: 386AEEF4  addi r3, r10, -0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + -4364;
	// 8261E4B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E4D8: 4BE48949  bl 0x82466e20
	ctx.lr = 0x8261E4DC;
	sub_82466E20(ctx, base);
	// 8261E4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E4F0 size=108
    let mut pc: u32 = 0x8261E4F0;
    'dispatch: loop {
        match pc {
            0x8261E4F0 => {
    //   block [0x8261E4F0..0x8261E55C)
	// 8261E4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E4FC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E504: 38EBF970  addi r7, r11, -0x690
	ctx.r[7].s64 = ctx.r[11].s64 + -1680;
	// 8261E508: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8261E50C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8261E510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E514: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E520: 386AEF24  addi r3, r10, -0x10dc
	ctx.r[3].s64 = ctx.r[10].s64 + -4316;
	// 8261E524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E548: 4BE488D9  bl 0x82466e20
	ctx.lr = 0x8261E54C;
	sub_82466E20(ctx, base);
	// 8261E54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E560 size=108
    let mut pc: u32 = 0x8261E560;
    'dispatch: loop {
        match pc {
            0x8261E560 => {
    //   block [0x8261E560..0x8261E5CC)
	// 8261E560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E56C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E574: 38EBFA90  addi r7, r11, -0x570
	ctx.r[7].s64 = ctx.r[11].s64 + -1392;
	// 8261E578: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261E57C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8261E580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E590: 386AEF54  addi r3, r10, -0x10ac
	ctx.r[3].s64 = ctx.r[10].s64 + -4268;
	// 8261E594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E5B8: 4BE48869  bl 0x82466e20
	ctx.lr = 0x8261E5BC;
	sub_82466E20(ctx, base);
	// 8261E5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E5D0 size=108
    let mut pc: u32 = 0x8261E5D0;
    'dispatch: loop {
        match pc {
            0x8261E5D0 => {
    //   block [0x8261E5D0..0x8261E63C)
	// 8261E5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E5DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E5E4: 38EBFB20  addi r7, r11, -0x4e0
	ctx.r[7].s64 = ctx.r[11].s64 + -1248;
	// 8261E5E8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8261E5EC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8261E5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E5F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E5F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E600: 386AEF84  addi r3, r10, -0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + -4220;
	// 8261E604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E628: 4BE487F9  bl 0x82466e20
	ctx.lr = 0x8261E62C;
	sub_82466E20(ctx, base);
	// 8261E62C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E640 size=108
    let mut pc: u32 = 0x8261E640;
    'dispatch: loop {
        match pc {
            0x8261E640 => {
    //   block [0x8261E640..0x8261E6AC)
	// 8261E640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E64C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E654: 38EBFBE0  addi r7, r11, -0x420
	ctx.r[7].s64 = ctx.r[11].s64 + -1056;
	// 8261E658: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8261E65C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8261E660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E670: 386AEFB4  addi r3, r10, -0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + -4172;
	// 8261E674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E698: 4BE48789  bl 0x82466e20
	ctx.lr = 0x8261E69C;
	sub_82466E20(ctx, base);
	// 8261E69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E6B0 size=108
    let mut pc: u32 = 0x8261E6B0;
    'dispatch: loop {
        match pc {
            0x8261E6B0 => {
    //   block [0x8261E6B0..0x8261E71C)
	// 8261E6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E6BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E6C4: 38EBFCB8  addi r7, r11, -0x348
	ctx.r[7].s64 = ctx.r[11].s64 + -840;
	// 8261E6C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8261E6CC: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8261E6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E6D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E6E0: 386AEFE4  addi r3, r10, -0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + -4124;
	// 8261E6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E708: 4BE48719  bl 0x82466e20
	ctx.lr = 0x8261E70C;
	sub_82466E20(ctx, base);
	// 8261E70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E720 size=108
    let mut pc: u32 = 0x8261E720;
    'dispatch: loop {
        match pc {
            0x8261E720 => {
    //   block [0x8261E720..0x8261E78C)
	// 8261E720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E72C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E734: 38EBFD78  addi r7, r11, -0x288
	ctx.r[7].s64 = ctx.r[11].s64 + -648;
	// 8261E738: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8261E73C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8261E740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E744: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E750: 386AF014  addi r3, r10, -0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + -4076;
	// 8261E754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E778: 4BE486A9  bl 0x82466e20
	ctx.lr = 0x8261E77C;
	sub_82466E20(ctx, base);
	// 8261E77C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E790 size=112
    let mut pc: u32 = 0x8261E790;
    'dispatch: loop {
        match pc {
            0x8261E790 => {
    //   block [0x8261E790..0x8261E800)
	// 8261E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E79C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261E7A0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8261E7A4: 38EAFE20  addi r7, r10, -0x1e0
	ctx.r[7].s64 = ctx.r[10].s64 + -480;
	// 8261E7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E7AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261E7B0: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8261E7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E7B8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E7BC: 396B1788  addi r11, r11, 0x1788
	ctx.r[11].s64 = ctx.r[11].s64 + 6024;
	// 8261E7C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E7C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E7C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E7CC: 386AF044  addi r3, r10, -0xfbc
	ctx.r[3].s64 = ctx.r[10].s64 + -4028;
	// 8261E7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E7D4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261E7D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E7DC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261E7E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E7E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E7E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E7EC: 4BE48635  bl 0x82466e20
	ctx.lr = 0x8261E7F0;
	sub_82466E20(ctx, base);
	// 8261E7F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E800 size=108
    let mut pc: u32 = 0x8261E800;
    'dispatch: loop {
        match pc {
            0x8261E800 => {
    //   block [0x8261E800..0x8261E86C)
	// 8261E800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E80C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E814: 38EBFF40  addi r7, r11, -0xc0
	ctx.r[7].s64 = ctx.r[11].s64 + -192;
	// 8261E818: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261E81C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8261E820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E824: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E830: 386AF074  addi r3, r10, -0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3980;
	// 8261E834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E858: 4BE485C9  bl 0x82466e20
	ctx.lr = 0x8261E85C;
	sub_82466E20(ctx, base);
	// 8261E85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E870 size=108
    let mut pc: u32 = 0x8261E870;
    'dispatch: loop {
        match pc {
            0x8261E870 => {
    //   block [0x8261E870..0x8261E8DC)
	// 8261E870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E87C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E884: 38EBFFA0  addi r7, r11, -0x60
	ctx.r[7].s64 = ctx.r[11].s64 + -96;
	// 8261E888: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8261E88C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8261E890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E8A0: 386AF0A4  addi r3, r10, -0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3932;
	// 8261E8A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E8C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E8C8: 4BE48559  bl 0x82466e20
	ctx.lr = 0x8261E8CC;
	sub_82466E20(ctx, base);
	// 8261E8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E8E0 size=108
    let mut pc: u32 = 0x8261E8E0;
    'dispatch: loop {
        match pc {
            0x8261E8E0 => {
    //   block [0x8261E8E0..0x8261E94C)
	// 8261E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E8EC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E8F4: 38EB00A8  addi r7, r11, 0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + 168;
	// 8261E8F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8261E8FC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8261E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E904: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E910: 386AF0D4  addi r3, r10, -0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3884;
	// 8261E914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E938: 4BE484E9  bl 0x82466e20
	ctx.lr = 0x8261E93C;
	sub_82466E20(ctx, base);
	// 8261E93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E950 size=108
    let mut pc: u32 = 0x8261E950;
    'dispatch: loop {
        match pc {
            0x8261E950 => {
    //   block [0x8261E950..0x8261E9BC)
	// 8261E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E95C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E964: 38EB0180  addi r7, r11, 0x180
	ctx.r[7].s64 = ctx.r[11].s64 + 384;
	// 8261E968: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261E96C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8261E970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E974: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E980: 386AF104  addi r3, r10, -0xefc
	ctx.r[3].s64 = ctx.r[10].s64 + -3836;
	// 8261E984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261E994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261E99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261E9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261E9A8: 4BE48479  bl 0x82466e20
	ctx.lr = 0x8261E9AC;
	sub_82466E20(ctx, base);
	// 8261E9AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261E9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261E9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261E9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261E9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261E9C0 size=108
    let mut pc: u32 = 0x8261E9C0;
    'dispatch: loop {
        match pc {
            0x8261E9C0 => {
    //   block [0x8261E9C0..0x8261EA2C)
	// 8261E9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261E9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261E9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261E9CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261E9D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261E9D4: 38EB01B0  addi r7, r11, 0x1b0
	ctx.r[7].s64 = ctx.r[11].s64 + 432;
	// 8261E9D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261E9DC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8261E9E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261E9E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261E9E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261E9EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261E9F0: 386AF134  addi r3, r10, -0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + -3788;
	// 8261E9F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261E9F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261E9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EA00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EA10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EA14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EA18: 4BE48409  bl 0x82466e20
	ctx.lr = 0x8261EA1C;
	sub_82466E20(ctx, base);
	// 8261EA1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EA30 size=108
    let mut pc: u32 = 0x8261EA30;
    'dispatch: loop {
        match pc {
            0x8261EA30 => {
    //   block [0x8261EA30..0x8261EA9C)
	// 8261EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EA3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EA44: 38EB01C8  addi r7, r11, 0x1c8
	ctx.r[7].s64 = ctx.r[11].s64 + 456;
	// 8261EA48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261EA4C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8261EA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261EA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EA60: 386AF164  addi r3, r10, -0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3740;
	// 8261EA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261EA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EA88: 4BE48399  bl 0x82466e20
	ctx.lr = 0x8261EA8C;
	sub_82466E20(ctx, base);
	// 8261EA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EAA0 size=108
    let mut pc: u32 = 0x8261EAA0;
    'dispatch: loop {
        match pc {
            0x8261EAA0 => {
    //   block [0x8261EAA0..0x8261EB0C)
	// 8261EAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EAAC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EAB4: 38EB0210  addi r7, r11, 0x210
	ctx.r[7].s64 = ctx.r[11].s64 + 528;
	// 8261EAB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261EABC: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8261EAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EAC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EAC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261EACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EAD0: 386AF194  addi r3, r10, -0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3692;
	// 8261EAD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261EAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EAF8: 4BE48329  bl 0x82466e20
	ctx.lr = 0x8261EAFC;
	sub_82466E20(ctx, base);
	// 8261EAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EB10 size=112
    let mut pc: u32 = 0x8261EB10;
    'dispatch: loop {
        match pc {
            0x8261EB10 => {
    //   block [0x8261EB10..0x8261EB80)
	// 8261EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EB1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EB20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EB24: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261EB28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EB2C: 390B0228  addi r8, r11, 0x228
	ctx.r[8].s64 = ctx.r[11].s64 + 552;
	// 8261EB30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261EB34: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8261EB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EB3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261EB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EB48: 386AF1C4  addi r3, r10, -0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3644;
	// 8261EB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261EB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EB6C: 4BE482B5  bl 0x82466e20
	ctx.lr = 0x8261EB70;
	sub_82466E20(ctx, base);
	// 8261EB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EB80 size=108
    let mut pc: u32 = 0x8261EB80;
    'dispatch: loop {
        match pc {
            0x8261EB80 => {
    //   block [0x8261EB80..0x8261EBEC)
	// 8261EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EB8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EB90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EB94: 38EB0270  addi r7, r11, 0x270
	ctx.r[7].s64 = ctx.r[11].s64 + 624;
	// 8261EB98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261EB9C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8261EBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EBA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EBA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261EBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EBB0: 386AF1F4  addi r3, r10, -0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3596;
	// 8261EBB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261EBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EBD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EBD8: 4BE48249  bl 0x82466e20
	ctx.lr = 0x8261EBDC;
	sub_82466E20(ctx, base);
	// 8261EBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EBF0 size=112
    let mut pc: u32 = 0x8261EBF0;
    'dispatch: loop {
        match pc {
            0x8261EBF0 => {
    //   block [0x8261EBF0..0x8261EC60)
	// 8261EBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EBFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EC00: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EC04: 38AAF1F4  addi r5, r10, -0xe0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3596;
	// 8261EC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EC0C: 390B02D0  addi r8, r11, 0x2d0
	ctx.r[8].s64 = ctx.r[11].s64 + 720;
	// 8261EC10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261EC14: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8261EC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EC1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261EC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EC28: 386AF224  addi r3, r10, -0xddc
	ctx.r[3].s64 = ctx.r[10].s64 + -3548;
	// 8261EC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261EC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EC4C: 4BE481D5  bl 0x82466e20
	ctx.lr = 0x8261EC50;
	sub_82466E20(ctx, base);
	// 8261EC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EC60 size=96
    let mut pc: u32 = 0x8261EC60;
    'dispatch: loop {
        match pc {
            0x8261EC60 => {
    //   block [0x8261EC60..0x8261ECC0)
	// 8261EC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EC6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EC74: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8261EC78: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EC80: 386AF254  addi r3, r10, -0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + -3500;
	// 8261EC84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EC8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261EC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ECA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261ECA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261ECA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261ECAC: 4BE48175  bl 0x82466e20
	ctx.lr = 0x8261ECB0;
	sub_82466E20(ctx, base);
	// 8261ECB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ECB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ECB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ECC0 size=112
    let mut pc: u32 = 0x8261ECC0;
    'dispatch: loop {
        match pc {
            0x8261ECC0 => {
    //   block [0x8261ECC0..0x8261ED30)
	// 8261ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ECC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ECCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ECD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261ECD4: 38AA0AB4  addi r5, r10, 0xab4
	ctx.r[5].s64 = ctx.r[10].s64 + 2740;
	// 8261ECD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ECDC: 390B0318  addi r8, r11, 0x318
	ctx.r[8].s64 = ctx.r[11].s64 + 792;
	// 8261ECE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261ECE4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8261ECE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ECEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ECF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261ECF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ECF8: 386AF284  addi r3, r10, -0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3452;
	// 8261ECFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ED00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ED08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ED0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261ED10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ED14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ED18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ED1C: 4BE48105  bl 0x82466e20
	ctx.lr = 0x8261ED20;
	sub_82466E20(ctx, base);
	// 8261ED20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ED24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ED28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ED2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ED30 size=96
    let mut pc: u32 = 0x8261ED30;
    'dispatch: loop {
        match pc {
            0x8261ED30 => {
    //   block [0x8261ED30..0x8261ED90)
	// 8261ED30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ED34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ED38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ED3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ED40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ED44: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8261ED48: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ED4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ED50: 386AF2B4  addi r3, r10, -0xd4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3404;
	// 8261ED54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ED58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ED5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261ED60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ED64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ED68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ED6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ED70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261ED74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261ED78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261ED7C: 4BE480A5  bl 0x82466e20
	ctx.lr = 0x8261ED80;
	sub_82466E20(ctx, base);
	// 8261ED80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ED84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ED88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ED8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ED90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ED90 size=100
    let mut pc: u32 = 0x8261ED90;
    'dispatch: loop {
        match pc {
            0x8261ED90 => {
    //   block [0x8261ED90..0x8261EDF4)
	// 8261ED90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ED94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ED98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ED9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EDA4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261EDA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EDB0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8261EDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EDBC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261EDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EDC4: 386AF2E4  addi r3, r10, -0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + -3356;
	// 8261EDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EDCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EDD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EDD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EDE0: 4BE48041  bl 0x82466e20
	ctx.lr = 0x8261EDE4;
	sub_82466E20(ctx, base);
	// 8261EDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EDE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EDEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EDF8 size=96
    let mut pc: u32 = 0x8261EDF8;
    'dispatch: loop {
        match pc {
            0x8261EDF8 => {
    //   block [0x8261EDF8..0x8261EE58)
	// 8261EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EE04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EE0C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8261EE10: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EE18: 386AF314  addi r3, r10, -0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + -3308;
	// 8261EE1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EE24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261EE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EE3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EE40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EE44: 4BE47FDD  bl 0x82466e20
	ctx.lr = 0x8261EE48;
	sub_82466E20(ctx, base);
	// 8261EE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EE58 size=112
    let mut pc: u32 = 0x8261EE58;
    'dispatch: loop {
        match pc {
            0x8261EE58 => {
    //   block [0x8261EE58..0x8261EEC8)
	// 8261EE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EE64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EE68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EE6C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 8261EE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EE74: 390B0378  addi r8, r11, 0x378
	ctx.r[8].s64 = ctx.r[11].s64 + 888;
	// 8261EE78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261EE7C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8261EE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261EE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EE90: 386AF344  addi r3, r10, -0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3260;
	// 8261EE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261EE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EEB4: 4BE47F6D  bl 0x82466e20
	ctx.lr = 0x8261EEB8;
	sub_82466E20(ctx, base);
	// 8261EEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EEC8 size=112
    let mut pc: u32 = 0x8261EEC8;
    'dispatch: loop {
        match pc {
            0x8261EEC8 => {
    //   block [0x8261EEC8..0x8261EF38)
	// 8261EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EED8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EEDC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 8261EEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EEE4: 390B03A8  addi r8, r11, 0x3a8
	ctx.r[8].s64 = ctx.r[11].s64 + 936;
	// 8261EEE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261EEEC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8261EEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261EEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EF00: 386AF374  addi r3, r10, -0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3212;
	// 8261EF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261EF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EF24: 4BE47EFD  bl 0x82466e20
	ctx.lr = 0x8261EF28;
	sub_82466E20(ctx, base);
	// 8261EF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EF38 size=100
    let mut pc: u32 = 0x8261EF38;
    'dispatch: loop {
        match pc {
            0x8261EF38 => {
    //   block [0x8261EF38..0x8261EF9C)
	// 8261EF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EF4C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 8261EF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EF58: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8261EF5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EF6C: 386AF3A4  addi r3, r10, -0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3164;
	// 8261EF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EF74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EF78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EF80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EF88: 4BE47E99  bl 0x82466e20
	ctx.lr = 0x8261EF8C;
	sub_82466E20(ctx, base);
	// 8261EF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EFA0 size=96
    let mut pc: u32 = 0x8261EFA0;
    'dispatch: loop {
        match pc {
            0x8261EFA0 => {
    //   block [0x8261EFA0..0x8261F000)
	// 8261EFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EFAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EFB4: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8261EFB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EFC0: 386AF3D4  addi r3, r10, -0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3116;
	// 8261EFC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EFCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261EFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EFE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EFE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EFEC: 4BE47E35  bl 0x82466e20
	ctx.lr = 0x8261EFF0;
	sub_82466E20(ctx, base);
	// 8261EFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F000 size=112
    let mut pc: u32 = 0x8261F000;
    'dispatch: loop {
        match pc {
            0x8261F000 => {
    //   block [0x8261F000..0x8261F070)
	// 8261F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F010: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F014: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261F018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F01C: 390B03C0  addi r8, r11, 0x3c0
	ctx.r[8].s64 = ctx.r[11].s64 + 960;
	// 8261F020: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F024: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8261F028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F038: 386AF404  addi r3, r10, -0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3068;
	// 8261F03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F05C: 4BE47DC5  bl 0x82466e20
	ctx.lr = 0x8261F060;
	sub_82466E20(ctx, base);
	// 8261F060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F070 size=108
    let mut pc: u32 = 0x8261F070;
    'dispatch: loop {
        match pc {
            0x8261F070 => {
    //   block [0x8261F070..0x8261F0DC)
	// 8261F070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F07C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F084: 38EB03D8  addi r7, r11, 0x3d8
	ctx.r[7].s64 = ctx.r[11].s64 + 984;
	// 8261F088: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261F08C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8261F090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F0A0: 386AF434  addi r3, r10, -0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3020;
	// 8261F0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F0C8: 4BE47D59  bl 0x82466e20
	ctx.lr = 0x8261F0CC;
	sub_82466E20(ctx, base);
	// 8261F0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F0E0 size=112
    let mut pc: u32 = 0x8261F0E0;
    'dispatch: loop {
        match pc {
            0x8261F0E0 => {
    //   block [0x8261F0E0..0x8261F150)
	// 8261F0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F0F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F0F4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F0FC: 390B0438  addi r8, r11, 0x438
	ctx.r[8].s64 = ctx.r[11].s64 + 1080;
	// 8261F100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F104: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8261F108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F10C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F118: 386AF464  addi r3, r10, -0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + -2972;
	// 8261F11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F13C: 4BE47CE5  bl 0x82466e20
	ctx.lr = 0x8261F140;
	sub_82466E20(ctx, base);
	// 8261F140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F150 size=112
    let mut pc: u32 = 0x8261F150;
    'dispatch: loop {
        match pc {
            0x8261F150 => {
    //   block [0x8261F150..0x8261F1C0)
	// 8261F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F15C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F160: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F164: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F16C: 390B0450  addi r8, r11, 0x450
	ctx.r[8].s64 = ctx.r[11].s64 + 1104;
	// 8261F170: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F174: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8261F178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F188: 386AF494  addi r3, r10, -0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2924;
	// 8261F18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F1AC: 4BE47C75  bl 0x82466e20
	ctx.lr = 0x8261F1B0;
	sub_82466E20(ctx, base);
	// 8261F1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F1C0 size=112
    let mut pc: u32 = 0x8261F1C0;
    'dispatch: loop {
        match pc {
            0x8261F1C0 => {
    //   block [0x8261F1C0..0x8261F230)
	// 8261F1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F1D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F1D4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F1D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F1DC: 390B0480  addi r8, r11, 0x480
	ctx.r[8].s64 = ctx.r[11].s64 + 1152;
	// 8261F1E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F1E4: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8261F1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F1EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F1F8: 386AF4C4  addi r3, r10, -0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2876;
	// 8261F1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F21C: 4BE47C05  bl 0x82466e20
	ctx.lr = 0x8261F220;
	sub_82466E20(ctx, base);
	// 8261F220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F230 size=112
    let mut pc: u32 = 0x8261F230;
    'dispatch: loop {
        match pc {
            0x8261F230 => {
    //   block [0x8261F230..0x8261F2A0)
	// 8261F230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F240: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F244: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F24C: 390B0498  addi r8, r11, 0x498
	ctx.r[8].s64 = ctx.r[11].s64 + 1176;
	// 8261F250: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F254: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8261F258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F268: 386AF4F4  addi r3, r10, -0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2828;
	// 8261F26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F28C: 4BE47B95  bl 0x82466e20
	ctx.lr = 0x8261F290;
	sub_82466E20(ctx, base);
	// 8261F290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F2A0 size=112
    let mut pc: u32 = 0x8261F2A0;
    'dispatch: loop {
        match pc {
            0x8261F2A0 => {
    //   block [0x8261F2A0..0x8261F310)
	// 8261F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F2AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F2B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F2B4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F2BC: 390B04C8  addi r8, r11, 0x4c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1224;
	// 8261F2C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F2C4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8261F2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F2CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F2D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F2D8: 386AF524  addi r3, r10, -0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + -2780;
	// 8261F2DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F2EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F2FC: 4BE47B25  bl 0x82466e20
	ctx.lr = 0x8261F300;
	sub_82466E20(ctx, base);
	// 8261F300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F310 size=112
    let mut pc: u32 = 0x8261F310;
    'dispatch: loop {
        match pc {
            0x8261F310 => {
    //   block [0x8261F310..0x8261F380)
	// 8261F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F31C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F320: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F324: 38AAFA04  addi r5, r10, -0x5fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1532;
	// 8261F328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F32C: 390B04E0  addi r8, r11, 0x4e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1248;
	// 8261F330: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F334: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8261F338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F33C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F348: 386AF554  addi r3, r10, -0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + -2732;
	// 8261F34C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F36C: 4BE47AB5  bl 0x82466e20
	ctx.lr = 0x8261F370;
	sub_82466E20(ctx, base);
	// 8261F370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F380 size=112
    let mut pc: u32 = 0x8261F380;
    'dispatch: loop {
        match pc {
            0x8261F380 => {
    //   block [0x8261F380..0x8261F3F0)
	// 8261F380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F38C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F390: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F394: 38AAF764  addi r5, r10, -0x89c
	ctx.r[5].s64 = ctx.r[10].s64 + -2204;
	// 8261F398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F39C: 390B04F8  addi r8, r11, 0x4f8
	ctx.r[8].s64 = ctx.r[11].s64 + 1272;
	// 8261F3A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F3A4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8261F3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F3AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F3B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F3B8: 386AF584  addi r3, r10, -0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + -2684;
	// 8261F3BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F3DC: 4BE47A45  bl 0x82466e20
	ctx.lr = 0x8261F3E0;
	sub_82466E20(ctx, base);
	// 8261F3E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F3F0 size=112
    let mut pc: u32 = 0x8261F3F0;
    'dispatch: loop {
        match pc {
            0x8261F3F0 => {
    //   block [0x8261F3F0..0x8261F460)
	// 8261F3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F3FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F400: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F404: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F40C: 390B0510  addi r8, r11, 0x510
	ctx.r[8].s64 = ctx.r[11].s64 + 1296;
	// 8261F410: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261F414: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8261F418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F41C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F428: 386AF5B4  addi r3, r10, -0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + -2636;
	// 8261F42C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F43C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F44C: 4BE479D5  bl 0x82466e20
	ctx.lr = 0x8261F450;
	sub_82466E20(ctx, base);
	// 8261F450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F460 size=112
    let mut pc: u32 = 0x8261F460;
    'dispatch: loop {
        match pc {
            0x8261F460 => {
    //   block [0x8261F460..0x8261F4D0)
	// 8261F460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F46C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F470: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F474: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F47C: 390B0558  addi r8, r11, 0x558
	ctx.r[8].s64 = ctx.r[11].s64 + 1368;
	// 8261F480: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F484: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8261F488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F48C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F498: 386AF5E4  addi r3, r10, -0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + -2588;
	// 8261F49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F4BC: 4BE47965  bl 0x82466e20
	ctx.lr = 0x8261F4C0;
	sub_82466E20(ctx, base);
	// 8261F4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F4D0 size=112
    let mut pc: u32 = 0x8261F4D0;
    'dispatch: loop {
        match pc {
            0x8261F4D0 => {
    //   block [0x8261F4D0..0x8261F540)
	// 8261F4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F4DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F4E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F4E4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F4EC: 390B0588  addi r8, r11, 0x588
	ctx.r[8].s64 = ctx.r[11].s64 + 1416;
	// 8261F4F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F4F4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8261F4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F4FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F508: 386AF614  addi r3, r10, -0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2540;
	// 8261F50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F52C: 4BE478F5  bl 0x82466e20
	ctx.lr = 0x8261F530;
	sub_82466E20(ctx, base);
	// 8261F530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F540 size=108
    let mut pc: u32 = 0x8261F540;
    'dispatch: loop {
        match pc {
            0x8261F540 => {
    //   block [0x8261F540..0x8261F5AC)
	// 8261F540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F54C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F554: 38EB05B8  addi r7, r11, 0x5b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1464;
	// 8261F558: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261F55C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8261F560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F570: 386AF644  addi r3, r10, -0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2492;
	// 8261F574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F598: 4BE47889  bl 0x82466e20
	ctx.lr = 0x8261F59C;
	sub_82466E20(ctx, base);
	// 8261F59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F5B0 size=112
    let mut pc: u32 = 0x8261F5B0;
    'dispatch: loop {
        match pc {
            0x8261F5B0 => {
    //   block [0x8261F5B0..0x8261F620)
	// 8261F5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F5BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F5C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F5C4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F5CC: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 8261F5D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261F5D4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8261F5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F5E8: 386AF674  addi r3, r10, -0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + -2444;
	// 8261F5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F60C: 4BE47815  bl 0x82466e20
	ctx.lr = 0x8261F610;
	sub_82466E20(ctx, base);
	// 8261F610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F620 size=116
    let mut pc: u32 = 0x8261F620;
    'dispatch: loop {
        match pc {
            0x8261F620 => {
    //   block [0x8261F620..0x8261F694)
	// 8261F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F62C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261F634: 390B0680  addi r8, r11, 0x680
	ctx.r[8].s64 = ctx.r[11].s64 + 1664;
	// 8261F638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F63C: 392A1810  addi r9, r10, 0x1810
	ctx.r[9].s64 = ctx.r[10].s64 + 6160;
	// 8261F640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F644: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261F648: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261F668: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8261F66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261F670: 386BF6A4  addi r3, r11, -0x95c
	ctx.r[3].s64 = ctx.r[11].s64 + -2396;
	// 8261F674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261F678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F680: 4BE477A1  bl 0x82466e20
	ctx.lr = 0x8261F684;
	sub_82466E20(ctx, base);
	// 8261F684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F698 size=100
    let mut pc: u32 = 0x8261F698;
    'dispatch: loop {
        match pc {
            0x8261F698 => {
    //   block [0x8261F698..0x8261F6FC)
	// 8261F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F6AC: 38AAF7F4  addi r5, r10, -0x80c
	ctx.r[5].s64 = ctx.r[10].s64 + -2060;
	// 8261F6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F6B8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8261F6BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F6CC: 386AF6D4  addi r3, r10, -0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + -2348;
	// 8261F6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F6D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F6D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261F6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F6E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261F6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F6E8: 4BE47739  bl 0x82466e20
	ctx.lr = 0x8261F6EC;
	sub_82466E20(ctx, base);
	// 8261F6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F700 size=100
    let mut pc: u32 = 0x8261F700;
    'dispatch: loop {
        match pc {
            0x8261F700 => {
    //   block [0x8261F700..0x8261F764)
	// 8261F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F714: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F720: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8261F724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F734: 386AF704  addi r3, r10, -0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2300;
	// 8261F738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F73C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261F744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261F74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F750: 4BE476D1  bl 0x82466e20
	ctx.lr = 0x8261F754;
	sub_82466E20(ctx, base);
	// 8261F754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F768 size=108
    let mut pc: u32 = 0x8261F768;
    'dispatch: loop {
        match pc {
            0x8261F768 => {
    //   block [0x8261F768..0x8261F7D4)
	// 8261F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F774: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F77C: 38EB06F8  addi r7, r11, 0x6f8
	ctx.r[7].s64 = ctx.r[11].s64 + 1784;
	// 8261F780: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261F784: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8261F788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F78C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F798: 386AF734  addi r3, r10, -0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + -2252;
	// 8261F79C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F7BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F7C0: 4BE47661  bl 0x82466e20
	ctx.lr = 0x8261F7C4;
	sub_82466E20(ctx, base);
	// 8261F7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F7D8 size=112
    let mut pc: u32 = 0x8261F7D8;
    'dispatch: loop {
        match pc {
            0x8261F7D8 => {
    //   block [0x8261F7D8..0x8261F848)
	// 8261F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F7E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F7E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F7EC: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F7F4: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 8261F7F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F7FC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8261F800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F810: 386AF764  addi r3, r10, -0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + -2204;
	// 8261F814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F834: 4BE475ED  bl 0x82466e20
	ctx.lr = 0x8261F838;
	sub_82466E20(ctx, base);
	// 8261F838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F848 size=108
    let mut pc: u32 = 0x8261F848;
    'dispatch: loop {
        match pc {
            0x8261F848 => {
    //   block [0x8261F848..0x8261F8B4)
	// 8261F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F854: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F85C: 38EB0740  addi r7, r11, 0x740
	ctx.r[7].s64 = ctx.r[11].s64 + 1856;
	// 8261F860: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261F864: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8261F868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F86C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F878: 386AF794  addi r3, r10, -0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + -2156;
	// 8261F87C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F89C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F8A0: 4BE47581  bl 0x82466e20
	ctx.lr = 0x8261F8A4;
	sub_82466E20(ctx, base);
	// 8261F8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261F8B8 size=28
    let mut pc: u32 = 0x8261F8B8;
    'dispatch: loop {
        match pc {
            0x8261F8B8 => {
    //   block [0x8261F8B8..0x8261F8D4)
	// 8261F8B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F8BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261F8C0: 394A3CF0  addi r10, r10, 0x3cf0
	ctx.r[10].s64 = ctx.r[10].s64 + 15600;
	// 8261F8C4: 816B067C  lwz r11, 0x67c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1660 as u32) ) } as u64;
	// 8261F8C8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261F8CC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8261F8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F8D8 size=108
    let mut pc: u32 = 0x8261F8D8;
    'dispatch: loop {
        match pc {
            0x8261F8D8 => {
    //   block [0x8261F8D8..0x8261F944)
	// 8261F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F8E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F8E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F8EC: 38EB3CF0  addi r7, r11, 0x3cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 15600;
	// 8261F8F0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8261F8F4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8261F8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F8FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F908: 386AF7C4  addi r3, r10, -0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + -2108;
	// 8261F90C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F930: 4BE474F1  bl 0x82466e20
	ctx.lr = 0x8261F934;
	sub_82466E20(ctx, base);
	// 8261F934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F948 size=116
    let mut pc: u32 = 0x8261F948;
    'dispatch: loop {
        match pc {
            0x8261F948 => {
    //   block [0x8261F948..0x8261F9BC)
	// 8261F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F954: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261F95C: 390B0760  addi r8, r11, 0x760
	ctx.r[8].s64 = ctx.r[11].s64 + 1888;
	// 8261F960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F964: 392A1864  addi r9, r10, 0x1864
	ctx.r[9].s64 = ctx.r[10].s64 + 6244;
	// 8261F968: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F96C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8261F970: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F974: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F97C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F98C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261F990: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8261F994: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261F998: 386BF7F4  addi r3, r11, -0x80c
	ctx.r[3].s64 = ctx.r[11].s64 + -2060;
	// 8261F99C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8261F9A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F9A8: 4BE47479  bl 0x82466e20
	ctx.lr = 0x8261F9AC;
	sub_82466E20(ctx, base);
	// 8261F9AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F9C0 size=112
    let mut pc: u32 = 0x8261F9C0;
    'dispatch: loop {
        match pc {
            0x8261F9C0 => {
    //   block [0x8261F9C0..0x8261FA30)
	// 8261F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F9D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F9D4: 38AAF4C4  addi r5, r10, -0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + -2876;
	// 8261F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F9DC: 390B07C0  addi r8, r11, 0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1984;
	// 8261F9E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F9E4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8261F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F9EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F9F8: 386AF824  addi r3, r10, -0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2012;
	// 8261F9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FA1C: 4BE47405  bl 0x82466e20
	ctx.lr = 0x8261FA20;
	sub_82466E20(ctx, base);
	// 8261FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FA30 size=108
    let mut pc: u32 = 0x8261FA30;
    'dispatch: loop {
        match pc {
            0x8261FA30 => {
    //   block [0x8261FA30..0x8261FA9C)
	// 8261FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FA3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FA44: 38EB07D8  addi r7, r11, 0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2008;
	// 8261FA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261FA4C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8261FA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FA60: 386AF854  addi r3, r10, -0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1964;
	// 8261FA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FA88: 4BE47399  bl 0x82466e20
	ctx.lr = 0x8261FA8C;
	sub_82466E20(ctx, base);
	// 8261FA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FAA0 size=112
    let mut pc: u32 = 0x8261FAA0;
    'dispatch: loop {
        match pc {
            0x8261FAA0 => {
    //   block [0x8261FAA0..0x8261FB10)
	// 8261FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FAB0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FAB4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FABC: 390B0808  addi r8, r11, 0x808
	ctx.r[8].s64 = ctx.r[11].s64 + 2056;
	// 8261FAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FAC4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8261FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FAD8: 386AF884  addi r3, r10, -0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + -1916;
	// 8261FADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FAFC: 4BE47325  bl 0x82466e20
	ctx.lr = 0x8261FB00;
	sub_82466E20(ctx, base);
	// 8261FB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FB10 size=112
    let mut pc: u32 = 0x8261FB10;
    'dispatch: loop {
        match pc {
            0x8261FB10 => {
    //   block [0x8261FB10..0x8261FB80)
	// 8261FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FB1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FB24: 38AAFA04  addi r5, r10, -0x5fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1532;
	// 8261FB28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FB2C: 390B0838  addi r8, r11, 0x838
	ctx.r[8].s64 = ctx.r[11].s64 + 2104;
	// 8261FB30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FB34: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8261FB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FB3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FB48: 386AF8B4  addi r3, r10, -0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + -1868;
	// 8261FB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FB6C: 4BE472B5  bl 0x82466e20
	ctx.lr = 0x8261FB70;
	sub_82466E20(ctx, base);
	// 8261FB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FB80 size=100
    let mut pc: u32 = 0x8261FB80;
    'dispatch: loop {
        match pc {
            0x8261FB80 => {
    //   block [0x8261FB80..0x8261FBE4)
	// 8261FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FB8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FB94: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FB98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FBA0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8261FBA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FBB4: 386AF8E4  addi r3, r10, -0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + -1820;
	// 8261FBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FBBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FBC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FBC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FBD0: 4BE47251  bl 0x82466e20
	ctx.lr = 0x8261FBD4;
	sub_82466E20(ctx, base);
	// 8261FBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FBE8 size=112
    let mut pc: u32 = 0x8261FBE8;
    'dispatch: loop {
        match pc {
            0x8261FBE8 => {
    //   block [0x8261FBE8..0x8261FC58)
	// 8261FBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FBF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FBF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FBFC: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 8261FC00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FC04: 390B0868  addi r8, r11, 0x868
	ctx.r[8].s64 = ctx.r[11].s64 + 2152;
	// 8261FC08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261FC0C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8261FC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FC14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FC20: 386AF914  addi r3, r10, -0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1772;
	// 8261FC24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FC28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FC30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FC38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FC40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FC44: 4BE471DD  bl 0x82466e20
	ctx.lr = 0x8261FC48;
	sub_82466E20(ctx, base);
	// 8261FC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FC58 size=112
    let mut pc: u32 = 0x8261FC58;
    'dispatch: loop {
        match pc {
            0x8261FC58 => {
    //   block [0x8261FC58..0x8261FCC8)
	// 8261FC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FC6C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 8261FC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FC74: 390B08B0  addi r8, r11, 0x8b0
	ctx.r[8].s64 = ctx.r[11].s64 + 2224;
	// 8261FC78: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261FC7C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8261FC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FC84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FC90: 386AF944  addi r3, r10, -0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1724;
	// 8261FC94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FCB4: 4BE4716D  bl 0x82466e20
	ctx.lr = 0x8261FCB8;
	sub_82466E20(ctx, base);
	// 8261FCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FCC8 size=108
    let mut pc: u32 = 0x8261FCC8;
    'dispatch: loop {
        match pc {
            0x8261FCC8 => {
    //   block [0x8261FCC8..0x8261FD34)
	// 8261FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FCD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FCDC: 38EB0958  addi r7, r11, 0x958
	ctx.r[7].s64 = ctx.r[11].s64 + 2392;
	// 8261FCE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261FCE4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8261FCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FCEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FCF8: 386AF974  addi r3, r10, -0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + -1676;
	// 8261FCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FD20: 4BE47101  bl 0x82466e20
	ctx.lr = 0x8261FD24;
	sub_82466E20(ctx, base);
	// 8261FD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FD38 size=112
    let mut pc: u32 = 0x8261FD38;
    'dispatch: loop {
        match pc {
            0x8261FD38 => {
    //   block [0x8261FD38..0x8261FDA8)
	// 8261FD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FD44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FD48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FD4C: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261FD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FD54: 390B09A0  addi r8, r11, 0x9a0
	ctx.r[8].s64 = ctx.r[11].s64 + 2464;
	// 8261FD58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261FD5C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8261FD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FD64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FD68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FD70: 386AF9A4  addi r3, r10, -0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + -1628;
	// 8261FD74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FD78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FD8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FD94: 4BE4708D  bl 0x82466e20
	ctx.lr = 0x8261FD98;
	sub_82466E20(ctx, base);
	// 8261FD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FDA8 size=100
    let mut pc: u32 = 0x8261FDA8;
    'dispatch: loop {
        match pc {
            0x8261FDA8 => {
    //   block [0x8261FDA8..0x8261FE0C)
	// 8261FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FDBC: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261FDC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FDC8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8261FDCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FDDC: 386AF9D4  addi r3, r10, -0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + -1580;
	// 8261FDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FDE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FDE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FDF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FDF8: 4BE47029  bl 0x82466e20
	ctx.lr = 0x8261FDFC;
	sub_82466E20(ctx, base);
	// 8261FDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FE10 size=100
    let mut pc: u32 = 0x8261FE10;
    'dispatch: loop {
        match pc {
            0x8261FE10 => {
    //   block [0x8261FE10..0x8261FE74)
	// 8261FE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FE1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FE24: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FE30: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8261FE34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FE44: 386AFA04  addi r3, r10, -0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1532;
	// 8261FE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FE4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FE50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FE58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FE60: 4BE46FC1  bl 0x82466e20
	ctx.lr = 0x8261FE64;
	sub_82466E20(ctx, base);
	// 8261FE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FE78 size=108
    let mut pc: u32 = 0x8261FE78;
    'dispatch: loop {
        match pc {
            0x8261FE78 => {
    //   block [0x8261FE78..0x8261FEE4)
	// 8261FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FE84: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FE88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FE8C: 38EB0A00  addi r7, r11, 0xa00
	ctx.r[7].s64 = ctx.r[11].s64 + 2560;
	// 8261FE90: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261FE94: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8261FE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FE9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FEA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FEA8: 386AFA34  addi r3, r10, -0x5cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1484;
	// 8261FEAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FEB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FED0: 4BE46F51  bl 0x82466e20
	ctx.lr = 0x8261FED4;
	sub_82466E20(ctx, base);
	// 8261FED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FEE8 size=112
    let mut pc: u32 = 0x8261FEE8;
    'dispatch: loop {
        match pc {
            0x8261FEE8 => {
    //   block [0x8261FEE8..0x8261FF58)
	// 8261FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FEF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FEFC: 38AAF7F4  addi r5, r10, -0x80c
	ctx.r[5].s64 = ctx.r[10].s64 + -2060;
	// 8261FF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FF04: 390B0A90  addi r8, r11, 0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + 2704;
	// 8261FF08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261FF0C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8261FF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FF14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FF20: 386AFA64  addi r3, r10, -0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + -1436;
	// 8261FF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FF44: 4BE46EDD  bl 0x82466e20
	ctx.lr = 0x8261FF48;
	sub_82466E20(ctx, base);
	// 8261FF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FF58 size=112
    let mut pc: u32 = 0x8261FF58;
    'dispatch: loop {
        match pc {
            0x8261FF58 => {
    //   block [0x8261FF58..0x8261FFC8)
	// 8261FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FF64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FF6C: 38AAF944  addi r5, r10, -0x6bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1724;
	// 8261FF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FF74: 390B0AA8  addi r8, r11, 0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + 2728;
	// 8261FF78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FF7C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8261FF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FF90: 386AFA94  addi r3, r10, -0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + -1388;
	// 8261FF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FFB4: 4BE46E6D  bl 0x82466e20
	ctx.lr = 0x8261FFB8;
	sub_82466E20(ctx, base);
	// 8261FFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FFC8 size=112
    let mut pc: u32 = 0x8261FFC8;
    'dispatch: loop {
        match pc {
            0x8261FFC8 => {
    //   block [0x8261FFC8..0x82620038)
	// 8261FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FFD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FFD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FFDC: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FFE4: 390B0AD8  addi r8, r11, 0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + 2776;
	// 8261FFE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261FFEC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8261FFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FFF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620000: 386AFAC4  addi r3, r10, -0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + -1340;
	// 82620004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262000C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262001C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620024: 4BE46DFD  bl 0x82466e20
	ctx.lr = 0x82620028;
	sub_82466E20(ctx, base);
	// 82620028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262002C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620038 size=112
    let mut pc: u32 = 0x82620038;
    'dispatch: loop {
        match pc {
            0x82620038 => {
    //   block [0x82620038..0x826200A8)
	// 82620038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262003C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620048: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262004C: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 82620050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620054: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 82620058: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262005C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82620060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620064: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262006C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620070: 386AFAF4  addi r3, r10, -0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + -1292;
	// 82620074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262008C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620094: 4BE46D8D  bl 0x82466e20
	ctx.lr = 0x82620098;
	sub_82466E20(ctx, base);
	// 82620098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262009C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826200A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826200A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826200A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826200A8 size=108
    let mut pc: u32 = 0x826200A8;
    'dispatch: loop {
        match pc {
            0x826200A8 => {
    //   block [0x826200A8..0x82620114)
	// 826200A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826200AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826200B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826200B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826200B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826200BC: 38EB0B68  addi r7, r11, 0xb68
	ctx.r[7].s64 = ctx.r[11].s64 + 2920;
	// 826200C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826200C4: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 826200C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826200CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826200D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826200D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826200D8: 386AFB24  addi r3, r10, -0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1244;
	// 826200DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826200E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826200E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826200E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826200EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826200F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826200F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826200F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826200FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620100: 4BE46D21  bl 0x82466e20
	ctx.lr = 0x82620104;
	sub_82466E20(ctx, base);
	// 82620104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262010C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620118 size=112
    let mut pc: u32 = 0x82620118;
    'dispatch: loop {
        match pc {
            0x82620118 => {
    //   block [0x82620118..0x82620188)
	// 82620118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620128: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262012C: 38AAF4C4  addi r5, r10, -0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + -2876;
	// 82620130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620134: 390B0BB0  addi r8, r11, 0xbb0
	ctx.r[8].s64 = ctx.r[11].s64 + 2992;
	// 82620138: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262013C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82620140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620150: 386AFB54  addi r3, r10, -0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1196;
	// 82620154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262016C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620174: 4BE46CAD  bl 0x82466e20
	ctx.lr = 0x82620178;
	sub_82466E20(ctx, base);
	// 82620178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620188 size=112
    let mut pc: u32 = 0x82620188;
    'dispatch: loop {
        match pc {
            0x82620188 => {
    //   block [0x82620188..0x826201F8)
	// 82620188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620198: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262019C: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 826201A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826201A4: 390B0BC8  addi r8, r11, 0xbc8
	ctx.r[8].s64 = ctx.r[11].s64 + 3016;
	// 826201A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826201AC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826201B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826201B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826201B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826201BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826201C0: 386AFB84  addi r3, r10, -0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + -1148;
	// 826201C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826201C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826201CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826201D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826201D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826201D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826201DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826201E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826201E4: 4BE46C3D  bl 0x82466e20
	ctx.lr = 0x826201E8;
	sub_82466E20(ctx, base);
	// 826201E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826201EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826201F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826201F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826201F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826201F8 size=112
    let mut pc: u32 = 0x826201F8;
    'dispatch: loop {
        match pc {
            0x826201F8 => {
    //   block [0x826201F8..0x82620268)
	// 826201F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826201FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262020C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82620210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620214: 390B0BF8  addi r8, r11, 0xbf8
	ctx.r[8].s64 = ctx.r[11].s64 + 3064;
	// 82620218: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262021C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82620220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262022C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620230: 386AFBB4  addi r3, r10, -0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + -1100;
	// 82620234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262023C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262024C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620254: 4BE46BCD  bl 0x82466e20
	ctx.lr = 0x82620258;
	sub_82466E20(ctx, base);
	// 82620258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262025C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620268 size=112
    let mut pc: u32 = 0x82620268;
    'dispatch: loop {
        match pc {
            0x82620268 => {
    //   block [0x82620268..0x826202D8)
	// 82620268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262027C: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 82620280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620284: 390B0C58  addi r8, r11, 0xc58
	ctx.r[8].s64 = ctx.r[11].s64 + 3160;
	// 82620288: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262028C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82620290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826202A0: 386AFBE4  addi r3, r10, -0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + -1052;
	// 826202A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826202A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826202AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826202B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826202B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826202B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826202BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826202C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826202C4: 4BE46B5D  bl 0x82466e20
	ctx.lr = 0x826202C8;
	sub_82466E20(ctx, base);
	// 826202C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826202CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826202D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826202D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826202D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826202D8 size=112
    let mut pc: u32 = 0x826202D8;
    'dispatch: loop {
        match pc {
            0x826202D8 => {
    //   block [0x826202D8..0x82620348)
	// 826202D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826202DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826202E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826202E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826202E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826202EC: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 826202F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826202F4: 390B0C70  addi r8, r11, 0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + 3184;
	// 826202F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826202FC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82620300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620310: 386AFC14  addi r3, r10, -0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1004;
	// 82620314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262031C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262032C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620334: 4BE46AED  bl 0x82466e20
	ctx.lr = 0x82620338;
	sub_82466E20(ctx, base);
	// 82620338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620348 size=112
    let mut pc: u32 = 0x82620348;
    'dispatch: loop {
        match pc {
            0x82620348 => {
    //   block [0x82620348..0x826203B8)
	// 82620348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262035C: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 82620360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620364: 390B0CA0  addi r8, r11, 0xca0
	ctx.r[8].s64 = ctx.r[11].s64 + 3232;
	// 82620368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262036C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82620370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262037C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620380: 386AFC44  addi r3, r10, -0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + -956;
	// 82620384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262038C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262039C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826203A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826203A4: 4BE46A7D  bl 0x82466e20
	ctx.lr = 0x826203A8;
	sub_82466E20(ctx, base);
	// 826203A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826203AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826203B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826203B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826203B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826203B8 size=24
    let mut pc: u32 = 0x826203B8;
    'dispatch: loop {
        match pc {
            0x826203B8 => {
    //   block [0x826203B8..0x826203D0)
	// 826203B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826203BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826203C0: 394A3E28  addi r10, r10, 0x3e28
	ctx.r[10].s64 = ctx.r[10].s64 + 15912;
	// 826203C4: 816B075C  lwz r11, 0x75c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1884 as u32) ) } as u64;
	// 826203C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826203CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826203D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826203D0 size=112
    let mut pc: u32 = 0x826203D0;
    'dispatch: loop {
        match pc {
            0x826203D0 => {
    //   block [0x826203D0..0x82620440)
	// 826203D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826203D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826203D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826203DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826203E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826203E4: 392A18B4  addi r9, r10, 0x18b4
	ctx.r[9].s64 = ctx.r[10].s64 + 6324;
	// 826203E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826203EC: 390B3E28  addi r8, r11, 0x3e28
	ctx.r[8].s64 = ctx.r[11].s64 + 15912;
	// 826203F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826203F4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826203F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826203FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620408: 386AFC74  addi r3, r10, -0x38c
	ctx.r[3].s64 = ctx.r[10].s64 + -908;
	// 8262040C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620410: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262041C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262042C: 4BE469F5  bl 0x82466e20
	ctx.lr = 0x82620430;
	sub_82466E20(ctx, base);
	// 82620430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262043C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620440 size=108
    let mut pc: u32 = 0x82620440;
    'dispatch: loop {
        match pc {
            0x82620440 => {
    //   block [0x82620440..0x826204AC)
	// 82620440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262044C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620454: 38EB0CB8  addi r7, r11, 0xcb8
	ctx.r[7].s64 = ctx.r[11].s64 + 3256;
	// 82620458: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262045C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82620460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262046C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620470: 386AFCA4  addi r3, r10, -0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + -860;
	// 82620474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262047C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262048C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620498: 4BE46989  bl 0x82466e20
	ctx.lr = 0x8262049C;
	sub_82466E20(ctx, base);
	// 8262049C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826204A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826204A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826204A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826204B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826204B0 size=108
    let mut pc: u32 = 0x826204B0;
    'dispatch: loop {
        match pc {
            0x826204B0 => {
    //   block [0x826204B0..0x8262051C)
	// 826204B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826204B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826204B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826204BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826204C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826204C4: 38EB0CD0  addi r7, r11, 0xcd0
	ctx.r[7].s64 = ctx.r[11].s64 + 3280;
	// 826204C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826204CC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826204D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826204D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826204D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826204DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826204E0: 386AFCD4  addi r3, r10, -0x32c
	ctx.r[3].s64 = ctx.r[10].s64 + -812;
	// 826204E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826204E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826204EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826204F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826204F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826204F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826204FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620508: 4BE46919  bl 0x82466e20
	ctx.lr = 0x8262050C;
	sub_82466E20(ctx, base);
	// 8262050C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620520 size=116
    let mut pc: u32 = 0x82620520;
    'dispatch: loop {
        match pc {
            0x82620520 => {
    //   block [0x82620520..0x82620594)
	// 82620520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262052C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620534: 390B0D1C  addi r8, r11, 0xd1c
	ctx.r[8].s64 = ctx.r[11].s64 + 3356;
	// 82620538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262053C: 392A1988  addi r9, r10, 0x1988
	ctx.r[9].s64 = ctx.r[10].s64 + 6536;
	// 82620540: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620544: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82620548: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262054C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262055C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620564: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620568: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8262056C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620570: 386BFD04  addi r3, r11, -0x2fc
	ctx.r[3].s64 = ctx.r[11].s64 + -764;
	// 82620574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262057C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620580: 4BE468A1  bl 0x82466e20
	ctx.lr = 0x82620584;
	sub_82466E20(ctx, base);
	// 82620584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262058C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620598 size=108
    let mut pc: u32 = 0x82620598;
    'dispatch: loop {
        match pc {
            0x82620598 => {
    //   block [0x82620598..0x82620604)
	// 82620598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262059C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826205A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826205A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826205A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826205AC: 38EB0D38  addi r7, r11, 0xd38
	ctx.r[7].s64 = ctx.r[11].s64 + 3384;
	// 826205B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826205B4: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826205B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826205BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826205C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826205C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826205C8: 386AFD34  addi r3, r10, -0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + -716;
	// 826205CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826205D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826205D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826205D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826205DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826205E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826205E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826205E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826205EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826205F0: 4BE46831  bl 0x82466e20
	ctx.lr = 0x826205F4;
	sub_82466E20(ctx, base);
	// 826205F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826205F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826205FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82620608 size=24
    let mut pc: u32 = 0x82620608;
    'dispatch: loop {
        match pc {
            0x82620608 => {
    //   block [0x82620608..0x82620620)
	// 82620608: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262060C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82620610: 394A3E70  addi r10, r10, 0x3e70
	ctx.r[10].s64 = ctx.r[10].s64 + 15984;
	// 82620614: 816B0D34  lwz r11, 0xd34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3380 as u32) ) } as u64;
	// 82620618: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620620 size=116
    let mut pc: u32 = 0x82620620;
    'dispatch: loop {
        match pc {
            0x82620620 => {
    //   block [0x82620620..0x82620694)
	// 82620620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262062C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620634: 390B3E70  addi r8, r11, 0x3e70
	ctx.r[8].s64 = ctx.r[11].s64 + 15984;
	// 82620638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262063C: 392A19E4  addi r9, r10, 0x19e4
	ctx.r[9].s64 = ctx.r[10].s64 + 6628;
	// 82620640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620644: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82620648: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262064C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262065C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620668: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8262066C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620670: 386BFD64  addi r3, r11, -0x29c
	ctx.r[3].s64 = ctx.r[11].s64 + -668;
	// 82620674: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82620678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262067C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620680: 4BE467A1  bl 0x82466e20
	ctx.lr = 0x82620684;
	sub_82466E20(ctx, base);
	// 82620684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262068C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620698 size=112
    let mut pc: u32 = 0x82620698;
    'dispatch: loop {
        match pc {
            0x82620698 => {
    //   block [0x82620698..0x82620708)
	// 82620698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826206A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826206A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826206A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826206AC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826206B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826206B4: 390B0DA0  addi r8, r11, 0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + 3488;
	// 826206B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826206BC: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 826206C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826206C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826206C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826206CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826206D0: 386AFD94  addi r3, r10, -0x26c
	ctx.r[3].s64 = ctx.r[10].s64 + -620;
	// 826206D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826206D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826206DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826206E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826206E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826206E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826206EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826206F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826206F4: 4BE4672D  bl 0x82466e20
	ctx.lr = 0x826206F8;
	sub_82466E20(ctx, base);
	// 826206F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826206FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620708 size=112
    let mut pc: u32 = 0x82620708;
    'dispatch: loop {
        match pc {
            0x82620708 => {
    //   block [0x82620708..0x82620778)
	// 82620708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620718: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262071C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620724: 390B0DB8  addi r8, r11, 0xdb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3512;
	// 82620728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262072C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82620730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620734: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620740: 386AFDC4  addi r3, r10, -0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + -572;
	// 82620744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262074C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262075C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620764: 4BE466BD  bl 0x82466e20
	ctx.lr = 0x82620768;
	sub_82466E20(ctx, base);
	// 82620768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262076C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620778 size=108
    let mut pc: u32 = 0x82620778;
    'dispatch: loop {
        match pc {
            0x82620778 => {
    //   block [0x82620778..0x826207E4)
	// 82620778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262077C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262078C: 38EB0DE8  addi r7, r11, 0xde8
	ctx.r[7].s64 = ctx.r[11].s64 + 3560;
	// 82620790: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620794: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 82620798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262079C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826207A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826207A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826207A8: 386AFDF4  addi r3, r10, -0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + -524;
	// 826207AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826207B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826207B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826207B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826207BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826207C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826207C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826207C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826207CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826207D0: 4BE46651  bl 0x82466e20
	ctx.lr = 0x826207D4;
	sub_82466E20(ctx, base);
	// 826207D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826207D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826207DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826207E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826207E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826207E8 size=108
    let mut pc: u32 = 0x826207E8;
    'dispatch: loop {
        match pc {
            0x826207E8 => {
    //   block [0x826207E8..0x82620854)
	// 826207E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826207EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826207F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826207F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826207F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826207FC: 38EB0E30  addi r7, r11, 0xe30
	ctx.r[7].s64 = ctx.r[11].s64 + 3632;
	// 82620800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82620804: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82620808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262080C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620818: 386AFE24  addi r3, r10, -0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + -476;
	// 8262081C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262082C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262083C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620840: 4BE465E1  bl 0x82466e20
	ctx.lr = 0x82620844;
	sub_82466E20(ctx, base);
	// 82620844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262084C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620858 size=112
    let mut pc: u32 = 0x82620858;
    'dispatch: loop {
        match pc {
            0x82620858 => {
    //   block [0x82620858..0x826208C8)
	// 82620858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262085C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620868: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262086C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620874: 390B0E60  addi r8, r11, 0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + 3680;
	// 82620878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262087C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82620880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262088C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620890: 386AFE54  addi r3, r10, -0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + -428;
	// 82620894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262089C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826208A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826208A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826208A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826208AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826208B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826208B4: 4BE4656D  bl 0x82466e20
	ctx.lr = 0x826208B8;
	sub_82466E20(ctx, base);
	// 826208B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826208BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826208C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826208C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826208C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826208C8 size=108
    let mut pc: u32 = 0x826208C8;
    'dispatch: loop {
        match pc {
            0x826208C8 => {
    //   block [0x826208C8..0x82620934)
	// 826208C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826208CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826208D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826208D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826208D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826208DC: 38EB0E90  addi r7, r11, 0xe90
	ctx.r[7].s64 = ctx.r[11].s64 + 3728;
	// 826208E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826208E4: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826208E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826208EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826208F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826208F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826208F8: 386AFE84  addi r3, r10, -0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + -380;
	// 826208FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262090C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262091C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620920: 4BE46501  bl 0x82466e20
	ctx.lr = 0x82620924;
	sub_82466E20(ctx, base);
	// 82620924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262092C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620938 size=108
    let mut pc: u32 = 0x82620938;
    'dispatch: loop {
        match pc {
            0x82620938 => {
    //   block [0x82620938..0x826209A4)
	// 82620938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262093C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262094C: 38EB0EF0  addi r7, r11, 0xef0
	ctx.r[7].s64 = ctx.r[11].s64 + 3824;
	// 82620950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620954: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82620958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262095C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620968: 386AFEB4  addi r3, r10, -0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + -332;
	// 8262096C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262097C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262098C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620990: 4BE46491  bl 0x82466e20
	ctx.lr = 0x82620994;
	sub_82466E20(ctx, base);
	// 82620994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262099C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826209A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826209A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826209A8 size=116
    let mut pc: u32 = 0x826209A8;
    'dispatch: loop {
        match pc {
            0x826209A8 => {
    //   block [0x826209A8..0x82620A1C)
	// 826209A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826209AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826209B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826209B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826209B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826209BC: 390A0F38  addi r8, r10, 0xf38
	ctx.r[8].s64 = ctx.r[10].s64 + 3896;
	// 826209C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826209C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826209C8: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826209CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826209D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826209D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826209D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826209DC: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826209E0: 396B1A20  addi r11, r11, 0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + 6688;
	// 826209E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826209E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826209EC: 386AFEE4  addi r3, r10, -0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + -284;
	// 826209F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826209F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826209F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826209FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620A08: 4BE46419  bl 0x82466e20
	ctx.lr = 0x82620A0C;
	sub_82466E20(ctx, base);
	// 82620A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620A20 size=112
    let mut pc: u32 = 0x82620A20;
    'dispatch: loop {
        match pc {
            0x82620A20 => {
    //   block [0x82620A20..0x82620A90)
	// 82620A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620A30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620A34: 38AAFF44  addi r5, r10, -0xbc
	ctx.r[5].s64 = ctx.r[10].s64 + -188;
	// 82620A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620A3C: 390B0FC8  addi r8, r11, 0xfc8
	ctx.r[8].s64 = ctx.r[11].s64 + 4040;
	// 82620A40: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82620A44: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82620A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620A58: 386AFF14  addi r3, r10, -0xec
	ctx.r[3].s64 = ctx.r[10].s64 + -236;
	// 82620A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620A7C: 4BE463A5  bl 0x82466e20
	ctx.lr = 0x82620A80;
	sub_82466E20(ctx, base);
	// 82620A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620A90 size=100
    let mut pc: u32 = 0x82620A90;
    'dispatch: loop {
        match pc {
            0x82620A90 => {
    //   block [0x82620A90..0x82620AF4)
	// 82620A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620A9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620AA4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82620AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620AB0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82620AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620AC4: 386AFF44  addi r3, r10, -0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + -188;
	// 82620AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620ACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620AD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82620AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620AD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82620ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620AE0: 4BE46341  bl 0x82466e20
	ctx.lr = 0x82620AE4;
	sub_82466E20(ctx, base);
	// 82620AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82620AF8 size=24
    let mut pc: u32 = 0x82620AF8;
    'dispatch: loop {
        match pc {
            0x82620AF8 => {
    //   block [0x82620AF8..0x82620B10)
	// 82620AF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620AFC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82620B00: 394A3F30  addi r10, r10, 0x3f30
	ctx.r[10].s64 = ctx.r[10].s64 + 16176;
	// 82620B04: 816B1040  lwz r11, 0x1040(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4160 as u32) ) } as u64;
	// 82620B08: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82620B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620B10 size=116
    let mut pc: u32 = 0x82620B10;
    'dispatch: loop {
        match pc {
            0x82620B10 => {
    //   block [0x82620B10..0x82620B84)
	// 82620B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620B1C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620B20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620B24: 390B3F30  addi r8, r11, 0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + 16176;
	// 82620B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620B2C: 392A1A64  addi r9, r10, 0x1a64
	ctx.r[9].s64 = ctx.r[10].s64 + 6756;
	// 82620B30: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620B34: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82620B38: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620B3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620B44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620B54: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620B58: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82620B5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620B60: 386BFF74  addi r3, r11, -0x8c
	ctx.r[3].s64 = ctx.r[11].s64 + -140;
	// 82620B64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620B68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620B70: 4BE462B1  bl 0x82466e20
	ctx.lr = 0x82620B74;
	sub_82466E20(ctx, base);
	// 82620B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620B88 size=112
    let mut pc: u32 = 0x82620B88;
    'dispatch: loop {
        match pc {
            0x82620B88 => {
    //   block [0x82620B88..0x82620BF8)
	// 82620B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620B98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620B9C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620BA4: 390B1048  addi r8, r11, 0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + 4168;
	// 82620BA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82620BAC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82620BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620BC0: 386AFFA4  addi r3, r10, -0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + -92;
	// 82620BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620BE4: 4BE4623D  bl 0x82466e20
	ctx.lr = 0x82620BE8;
	sub_82466E20(ctx, base);
	// 82620BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620BF8 size=112
    let mut pc: u32 = 0x82620BF8;
    'dispatch: loop {
        match pc {
            0x82620BF8 => {
    //   block [0x82620BF8..0x82620C68)
	// 82620BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620C0C: 38AAFEE4  addi r5, r10, -0x11c
	ctx.r[5].s64 = ctx.r[10].s64 + -284;
	// 82620C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620C14: 390B1090  addi r8, r11, 0x1090
	ctx.r[8].s64 = ctx.r[11].s64 + 4240;
	// 82620C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82620C1C: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82620C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620C30: 386AFFD4  addi r3, r10, -0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + -44;
	// 82620C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620C54: 4BE461CD  bl 0x82466e20
	ctx.lr = 0x82620C58;
	sub_82466E20(ctx, base);
	// 82620C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620C68 size=108
    let mut pc: u32 = 0x82620C68;
    'dispatch: loop {
        match pc {
            0x82620C68 => {
    //   block [0x82620C68..0x82620CD4)
	// 82620C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620C74: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620C78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620C7C: 38EB10F0  addi r7, r11, 0x10f0
	ctx.r[7].s64 = ctx.r[11].s64 + 4336;
	// 82620C80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620C84: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82620C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620C98: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82620C9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620CC0: 4BE46161  bl 0x82466e20
	ctx.lr = 0x82620CC4;
	sub_82466E20(ctx, base);
	// 82620CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620CD8 size=108
    let mut pc: u32 = 0x82620CD8;
    'dispatch: loop {
        match pc {
            0x82620CD8 => {
    //   block [0x82620CD8..0x82620D44)
	// 82620CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620CE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620CEC: 38EB1138  addi r7, r11, 0x1138
	ctx.r[7].s64 = ctx.r[11].s64 + 4408;
	// 82620CF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620CF4: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82620CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620D08: 386A0034  addi r3, r10, 0x34
	ctx.r[3].s64 = ctx.r[10].s64 + 52;
	// 82620D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620D30: 4BE460F1  bl 0x82466e20
	ctx.lr = 0x82620D34;
	sub_82466E20(ctx, base);
	// 82620D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620D48 size=112
    let mut pc: u32 = 0x82620D48;
    'dispatch: loop {
        match pc {
            0x82620D48 => {
    //   block [0x82620D48..0x82620DB8)
	// 82620D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620D5C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620D64: 390B1180  addi r8, r11, 0x1180
	ctx.r[8].s64 = ctx.r[11].s64 + 4480;
	// 82620D68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82620D6C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82620D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620D80: 386A0064  addi r3, r10, 0x64
	ctx.r[3].s64 = ctx.r[10].s64 + 100;
	// 82620D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620DA4: 4BE4607D  bl 0x82466e20
	ctx.lr = 0x82620DA8;
	sub_82466E20(ctx, base);
	// 82620DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620DB8 size=112
    let mut pc: u32 = 0x82620DB8;
    'dispatch: loop {
        match pc {
            0x82620DB8 => {
    //   block [0x82620DB8..0x82620E28)
	// 82620DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620DC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620DCC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620DD4: 390B1228  addi r8, r11, 0x1228
	ctx.r[8].s64 = ctx.r[11].s64 + 4648;
	// 82620DD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82620DDC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82620DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620DF0: 386A0094  addi r3, r10, 0x94
	ctx.r[3].s64 = ctx.r[10].s64 + 148;
	// 82620DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620E14: 4BE4600D  bl 0x82466e20
	ctx.lr = 0x82620E18;
	sub_82466E20(ctx, base);
	// 82620E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620E28 size=108
    let mut pc: u32 = 0x82620E28;
    'dispatch: loop {
        match pc {
            0x82620E28 => {
    //   block [0x82620E28..0x82620E94)
	// 82620E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620E34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620E3C: 38EB1270  addi r7, r11, 0x1270
	ctx.r[7].s64 = ctx.r[11].s64 + 4720;
	// 82620E40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82620E44: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82620E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620E4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620E58: 386A00C4  addi r3, r10, 0xc4
	ctx.r[3].s64 = ctx.r[10].s64 + 196;
	// 82620E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620E80: 4BE45FA1  bl 0x82466e20
	ctx.lr = 0x82620E84;
	sub_82466E20(ctx, base);
	// 82620E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620E98 size=108
    let mut pc: u32 = 0x82620E98;
    'dispatch: loop {
        match pc {
            0x82620E98 => {
    //   block [0x82620E98..0x82620F04)
	// 82620E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620EA4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620EA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620EAC: 38EB12A0  addi r7, r11, 0x12a0
	ctx.r[7].s64 = ctx.r[11].s64 + 4768;
	// 82620EB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82620EB4: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 82620EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620EBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620EC8: 386A00F4  addi r3, r10, 0xf4
	ctx.r[3].s64 = ctx.r[10].s64 + 244;
	// 82620ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620EF0: 4BE45F31  bl 0x82466e20
	ctx.lr = 0x82620EF4;
	sub_82466E20(ctx, base);
	// 82620EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620F08 size=112
    let mut pc: u32 = 0x82620F08;
    'dispatch: loop {
        match pc {
            0x82620F08 => {
    //   block [0x82620F08..0x82620F78)
	// 82620F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620F1C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620F24: 390B1330  addi r8, r11, 0x1330
	ctx.r[8].s64 = ctx.r[11].s64 + 4912;
	// 82620F28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82620F2C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82620F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620F40: 386A0124  addi r3, r10, 0x124
	ctx.r[3].s64 = ctx.r[10].s64 + 292;
	// 82620F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620F64: 4BE45EBD  bl 0x82466e20
	ctx.lr = 0x82620F68;
	sub_82466E20(ctx, base);
	// 82620F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620F78 size=112
    let mut pc: u32 = 0x82620F78;
    'dispatch: loop {
        match pc {
            0x82620F78 => {
    //   block [0x82620F78..0x82620FE8)
	// 82620F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620F8C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620F94: 390B13C0  addi r8, r11, 0x13c0
	ctx.r[8].s64 = ctx.r[11].s64 + 5056;
	// 82620F98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82620F9C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82620FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620FB0: 386A0154  addi r3, r10, 0x154
	ctx.r[3].s64 = ctx.r[10].s64 + 340;
	// 82620FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620FD4: 4BE45E4D  bl 0x82466e20
	ctx.lr = 0x82620FD8;
	sub_82466E20(ctx, base);
	// 82620FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620FE8 size=100
    let mut pc: u32 = 0x82620FE8;
    'dispatch: loop {
        match pc {
            0x82620FE8 => {
    //   block [0x82620FE8..0x8262104C)
	// 82620FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620FFC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621008: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8262100C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262101C: 386A0184  addi r3, r10, 0x184
	ctx.r[3].s64 = ctx.r[10].s64 + 388;
	// 82621020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262102C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621038: 4BE45DE9  bl 0x82466e20
	ctx.lr = 0x8262103C;
	sub_82466E20(ctx, base);
	// 8262103C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621050 size=112
    let mut pc: u32 = 0x82621050;
    'dispatch: loop {
        match pc {
            0x82621050 => {
    //   block [0x82621050..0x826210C0)
	// 82621050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262105C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621060: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621064: 38AAFD64  addi r5, r10, -0x29c
	ctx.r[5].s64 = ctx.r[10].s64 + -668;
	// 82621068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262106C: 390B1480  addi r8, r11, 0x1480
	ctx.r[8].s64 = ctx.r[11].s64 + 5248;
	// 82621070: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621074: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82621078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262107C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621088: 386A01B4  addi r3, r10, 0x1b4
	ctx.r[3].s64 = ctx.r[10].s64 + 436;
	// 8262108C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262109C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826210A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826210A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826210A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826210AC: 4BE45D75  bl 0x82466e20
	ctx.lr = 0x826210B0;
	sub_82466E20(ctx, base);
	// 826210B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826210B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826210B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826210BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826210C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826210C0 size=112
    let mut pc: u32 = 0x826210C0;
    'dispatch: loop {
        match pc {
            0x826210C0 => {
    //   block [0x826210C0..0x82621130)
	// 826210C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826210C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826210C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826210CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826210D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826210D4: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 826210D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826210DC: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 826210E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826210E4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826210E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826210EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826210F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826210F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826210F8: 386A01E4  addi r3, r10, 0x1e4
	ctx.r[3].s64 = ctx.r[10].s64 + 484;
	// 826210FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262110C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262111C: 4BE45D05  bl 0x82466e20
	ctx.lr = 0x82621120;
	sub_82466E20(ctx, base);
	// 82621120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262112C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621130 size=108
    let mut pc: u32 = 0x82621130;
    'dispatch: loop {
        match pc {
            0x82621130 => {
    //   block [0x82621130..0x8262119C)
	// 82621130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262113C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621144: 38EB14C8  addi r7, r11, 0x14c8
	ctx.r[7].s64 = ctx.r[11].s64 + 5320;
	// 82621148: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262114C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82621150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621160: 386A0214  addi r3, r10, 0x214
	ctx.r[3].s64 = ctx.r[10].s64 + 532;
	// 82621164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262116C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262117C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621188: 4BE45C99  bl 0x82466e20
	ctx.lr = 0x8262118C;
	sub_82466E20(ctx, base);
	// 8262118C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826211A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826211A0 size=112
    let mut pc: u32 = 0x826211A0;
    'dispatch: loop {
        match pc {
            0x826211A0 => {
    //   block [0x826211A0..0x82621210)
	// 826211A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826211A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826211A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826211AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826211B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826211B4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826211B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826211BC: 390B14F8  addi r8, r11, 0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + 5368;
	// 826211C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826211C4: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826211C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826211CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826211D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826211D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826211D8: 386A0244  addi r3, r10, 0x244
	ctx.r[3].s64 = ctx.r[10].s64 + 580;
	// 826211DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826211E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826211E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826211E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826211EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826211F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826211F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826211F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826211FC: 4BE45C25  bl 0x82466e20
	ctx.lr = 0x82621200;
	sub_82466E20(ctx, base);
	// 82621200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621210 size=108
    let mut pc: u32 = 0x82621210;
    'dispatch: loop {
        match pc {
            0x82621210 => {
    //   block [0x82621210..0x8262127C)
	// 82621210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262121C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621224: 38EB1570  addi r7, r11, 0x1570
	ctx.r[7].s64 = ctx.r[11].s64 + 5488;
	// 82621228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262122C: 388A239C  addi r4, r10, 0x239c
	ctx.r[4].s64 = ctx.r[10].s64 + 9116;
	// 82621230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621240: 386A0274  addi r3, r10, 0x274
	ctx.r[3].s64 = ctx.r[10].s64 + 628;
	// 82621244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262124C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262125C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621268: 4BE45BB9  bl 0x82466e20
	ctx.lr = 0x8262126C;
	sub_82466E20(ctx, base);
	// 8262126C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621280 size=112
    let mut pc: u32 = 0x82621280;
    'dispatch: loop {
        match pc {
            0x82621280 => {
    //   block [0x82621280..0x826212F0)
	// 82621280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262128C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621290: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621294: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621298: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262129C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826212A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826212A4: 388A23C0  addi r4, r10, 0x23c0
	ctx.r[4].s64 = ctx.r[10].s64 + 9152;
	// 826212A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826212AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826212B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826212B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826212B8: 386A02A4  addi r3, r10, 0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + 676;
	// 826212BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826212C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826212C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826212C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826212CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826212D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826212D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826212D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826212DC: 4BE45B45  bl 0x82466e20
	ctx.lr = 0x826212E0;
	sub_82466E20(ctx, base);
	// 826212E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826212E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826212E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826212EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826212F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826212F0 size=108
    let mut pc: u32 = 0x826212F0;
    'dispatch: loop {
        match pc {
            0x826212F0 => {
    //   block [0x826212F0..0x8262135C)
	// 826212F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826212F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826212F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826212FC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621304: 38EB15E8  addi r7, r11, 0x15e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5608;
	// 82621308: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262130C: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82621310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262131C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621320: 386A02D4  addi r3, r10, 0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + 724;
	// 82621324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262132C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262133C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621348: 4BE45AD9  bl 0x82466e20
	ctx.lr = 0x8262134C;
	sub_82466E20(ctx, base);
	// 8262134C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621360 size=108
    let mut pc: u32 = 0x82621360;
    'dispatch: loop {
        match pc {
            0x82621360 => {
    //   block [0x82621360..0x826213CC)
	// 82621360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262136C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621374: 38EB1630  addi r7, r11, 0x1630
	ctx.r[7].s64 = ctx.r[11].s64 + 5680;
	// 82621378: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262137C: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82621380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262138C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621390: 386A0304  addi r3, r10, 0x304
	ctx.r[3].s64 = ctx.r[10].s64 + 772;
	// 82621394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262139C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826213A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826213A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826213A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826213AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826213B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826213B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826213B8: 4BE45A69  bl 0x82466e20
	ctx.lr = 0x826213BC;
	sub_82466E20(ctx, base);
	// 826213BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826213C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826213C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826213C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826213D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826213D0 size=112
    let mut pc: u32 = 0x826213D0;
    'dispatch: loop {
        match pc {
            0x826213D0 => {
    //   block [0x826213D0..0x82621440)
	// 826213D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826213D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826213D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826213DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826213E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826213E4: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826213E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826213EC: 390B1660  addi r8, r11, 0x1660
	ctx.r[8].s64 = ctx.r[11].s64 + 5728;
	// 826213F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826213F4: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826213F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826213FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621408: 386A0334  addi r3, r10, 0x334
	ctx.r[3].s64 = ctx.r[10].s64 + 820;
	// 8262140C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262141C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262142C: 4BE459F5  bl 0x82466e20
	ctx.lr = 0x82621430;
	sub_82466E20(ctx, base);
	// 82621430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262143C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621440 size=108
    let mut pc: u32 = 0x82621440;
    'dispatch: loop {
        match pc {
            0x82621440 => {
    //   block [0x82621440..0x826214AC)
	// 82621440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262144C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621454: 38EB16F0  addi r7, r11, 0x16f0
	ctx.r[7].s64 = ctx.r[11].s64 + 5872;
	// 82621458: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262145C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82621460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262146C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621470: 386A0364  addi r3, r10, 0x364
	ctx.r[3].s64 = ctx.r[10].s64 + 868;
	// 82621474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262147C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262148C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621498: 4BE45989  bl 0x82466e20
	ctx.lr = 0x8262149C;
	sub_82466E20(ctx, base);
	// 8262149C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826214A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826214A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826214A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826214B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826214B0 size=112
    let mut pc: u32 = 0x826214B0;
    'dispatch: loop {
        match pc {
            0x826214B0 => {
    //   block [0x826214B0..0x82621520)
	// 826214B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826214B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826214B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826214BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826214C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826214C4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826214C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826214CC: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 826214D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826214D4: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826214D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826214DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826214E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826214E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826214E8: 386A0394  addi r3, r10, 0x394
	ctx.r[3].s64 = ctx.r[10].s64 + 916;
	// 826214EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826214F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826214F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826214F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826214FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262150C: 4BE45915  bl 0x82466e20
	ctx.lr = 0x82621510;
	sub_82466E20(ctx, base);
	// 82621510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621520 size=108
    let mut pc: u32 = 0x82621520;
    'dispatch: loop {
        match pc {
            0x82621520 => {
    //   block [0x82621520..0x8262158C)
	// 82621520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262152C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621534: 38EB17F8  addi r7, r11, 0x17f8
	ctx.r[7].s64 = ctx.r[11].s64 + 6136;
	// 82621538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262153C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82621540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262154C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621550: 386A03C4  addi r3, r10, 0x3c4
	ctx.r[3].s64 = ctx.r[10].s64 + 964;
	// 82621554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262155C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262156C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621578: 4BE458A9  bl 0x82466e20
	ctx.lr = 0x8262157C;
	sub_82466E20(ctx, base);
	// 8262157C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621590 size=112
    let mut pc: u32 = 0x82621590;
    'dispatch: loop {
        match pc {
            0x82621590 => {
    //   block [0x82621590..0x82621600)
	// 82621590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262159C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826215A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826215A4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826215A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826215AC: 390B1840  addi r8, r11, 0x1840
	ctx.r[8].s64 = ctx.r[11].s64 + 6208;
	// 826215B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826215B4: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826215B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826215BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826215C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826215C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826215C8: 386A03F4  addi r3, r10, 0x3f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1012;
	// 826215CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826215D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826215D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826215D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826215DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826215E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826215E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826215E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826215EC: 4BE45835  bl 0x82466e20
	ctx.lr = 0x826215F0;
	sub_82466E20(ctx, base);
	// 826215F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826215F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826215F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826215FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621600 size=112
    let mut pc: u32 = 0x82621600;
    'dispatch: loop {
        match pc {
            0x82621600 => {
    //   block [0x82621600..0x82621670)
	// 82621600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262160C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621610: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621614: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262161C: 390B18A0  addi r8, r11, 0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6304;
	// 82621620: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82621624: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82621628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262162C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621638: 386A0424  addi r3, r10, 0x424
	ctx.r[3].s64 = ctx.r[10].s64 + 1060;
	// 8262163C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262164C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262165C: 4BE457C5  bl 0x82466e20
	ctx.lr = 0x82621660;
	sub_82466E20(ctx, base);
	// 82621660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262166C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621670 size=108
    let mut pc: u32 = 0x82621670;
    'dispatch: loop {
        match pc {
            0x82621670 => {
    //   block [0x82621670..0x826216DC)
	// 82621670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262167C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621684: 38EB18B8  addi r7, r11, 0x18b8
	ctx.r[7].s64 = ctx.r[11].s64 + 6328;
	// 82621688: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262168C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82621690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621698: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262169C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826216A0: 386A0454  addi r3, r10, 0x454
	ctx.r[3].s64 = ctx.r[10].s64 + 1108;
	// 826216A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826216A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826216AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826216B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826216B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826216B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826216BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826216C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826216C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826216C8: 4BE45759  bl 0x82466e20
	ctx.lr = 0x826216CC;
	sub_82466E20(ctx, base);
	// 826216CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826216D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826216D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826216D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826216E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826216E0 size=112
    let mut pc: u32 = 0x826216E0;
    'dispatch: loop {
        match pc {
            0x826216E0 => {
    //   block [0x826216E0..0x82621750)
	// 826216E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826216E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826216E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826216EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826216F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826216F4: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826216F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826216FC: 390B1930  addi r8, r11, 0x1930
	ctx.r[8].s64 = ctx.r[11].s64 + 6448;
	// 82621700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621704: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82621708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262170C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621718: 386A0484  addi r3, r10, 0x484
	ctx.r[3].s64 = ctx.r[10].s64 + 1156;
	// 8262171C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262172C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262173C: 4BE456E5  bl 0x82466e20
	ctx.lr = 0x82621740;
	sub_82466E20(ctx, base);
	// 82621740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262174C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621750 size=100
    let mut pc: u32 = 0x82621750;
    'dispatch: loop {
        match pc {
            0x82621750 => {
    //   block [0x82621750..0x826217B4)
	// 82621750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262175C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621764: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262176C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621770: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82621774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262177C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621784: 386A04B4  addi r3, r10, 0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1204;
	// 82621788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262178C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262179C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826217A0: 4BE45681  bl 0x82466e20
	ctx.lr = 0x826217A4;
	sub_82466E20(ctx, base);
	// 826217A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826217A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826217AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826217B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826217B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826217B8 size=112
    let mut pc: u32 = 0x826217B8;
    'dispatch: loop {
        match pc {
            0x826217B8 => {
    //   block [0x826217B8..0x82621828)
	// 826217B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826217BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826217C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826217C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826217C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826217CC: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 826217D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826217D4: 390B1960  addi r8, r11, 0x1960
	ctx.r[8].s64 = ctx.r[11].s64 + 6496;
	// 826217D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826217DC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826217E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826217E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826217E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826217EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826217F0: 386A04E4  addi r3, r10, 0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1252;
	// 826217F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826217F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826217FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262180C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621814: 4BE4560D  bl 0x82466e20
	ctx.lr = 0x82621818;
	sub_82466E20(ctx, base);
	// 82621818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621828 size=112
    let mut pc: u32 = 0x82621828;
    'dispatch: loop {
        match pc {
            0x82621828 => {
    //   block [0x82621828..0x82621898)
	// 82621828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621838: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262183C: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82621840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621844: 390B19D8  addi r8, r11, 0x19d8
	ctx.r[8].s64 = ctx.r[11].s64 + 6616;
	// 82621848: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262184C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82621850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621860: 386A0514  addi r3, r10, 0x514
	ctx.r[3].s64 = ctx.r[10].s64 + 1300;
	// 82621864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262186C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262187C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621884: 4BE4559D  bl 0x82466e20
	ctx.lr = 0x82621888;
	sub_82466E20(ctx, base);
	// 82621888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621898 size=112
    let mut pc: u32 = 0x82621898;
    'dispatch: loop {
        match pc {
            0x82621898 => {
    //   block [0x82621898..0x82621908)
	// 82621898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826218A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826218A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826218A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826218AC: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 826218B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826218B4: 390B1A38  addi r8, r11, 0x1a38
	ctx.r[8].s64 = ctx.r[11].s64 + 6712;
	// 826218B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826218BC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826218C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826218C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826218C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826218CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826218D0: 386A0544  addi r3, r10, 0x544
	ctx.r[3].s64 = ctx.r[10].s64 + 1348;
	// 826218D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826218D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826218DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826218E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826218E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826218E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826218EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826218F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826218F4: 4BE4552D  bl 0x82466e20
	ctx.lr = 0x826218F8;
	sub_82466E20(ctx, base);
	// 826218F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826218FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621908 size=112
    let mut pc: u32 = 0x82621908;
    'dispatch: loop {
        match pc {
            0x82621908 => {
    //   block [0x82621908..0x82621978)
	// 82621908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621918: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262191C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621924: 390B1AB0  addi r8, r11, 0x1ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 6832;
	// 82621928: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262192C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82621930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621940: 386A0574  addi r3, r10, 0x574
	ctx.r[3].s64 = ctx.r[10].s64 + 1396;
	// 82621944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262194C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262195C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621964: 4BE454BD  bl 0x82466e20
	ctx.lr = 0x82621968;
	sub_82466E20(ctx, base);
	// 82621968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621978 size=116
    let mut pc: u32 = 0x82621978;
    'dispatch: loop {
        match pc {
            0x82621978 => {
    //   block [0x82621978..0x826219EC)
	// 82621978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621984: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82621988: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8262198C: 390A1B40  addi r8, r10, 0x1b40
	ctx.r[8].s64 = ctx.r[10].s64 + 6976;
	// 82621990: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621994: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82621998: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 8262199C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826219A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826219A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826219A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826219AC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826219B0: 396B1A78  addi r11, r11, 0x1a78
	ctx.r[11].s64 = ctx.r[11].s64 + 6776;
	// 826219B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826219B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826219BC: 386A05A4  addi r3, r10, 0x5a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1444;
	// 826219C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826219C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826219C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826219CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826219D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826219D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826219D8: 4BE45449  bl 0x82466e20
	ctx.lr = 0x826219DC;
	sub_82466E20(ctx, base);
	// 826219DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826219E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826219E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826219E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826219F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826219F0 size=100
    let mut pc: u32 = 0x826219F0;
    'dispatch: loop {
        match pc {
            0x826219F0 => {
    //   block [0x826219F0..0x82621A54)
	// 826219F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826219F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826219F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826219FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621A04: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621A10: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82621A14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621A24: 386A05D4  addi r3, r10, 0x5d4
	ctx.r[3].s64 = ctx.r[10].s64 + 1492;
	// 82621A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621A30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621A38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621A40: 4BE453E1  bl 0x82466e20
	ctx.lr = 0x82621A44;
	sub_82466E20(ctx, base);
	// 82621A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621A58 size=100
    let mut pc: u32 = 0x82621A58;
    'dispatch: loop {
        match pc {
            0x82621A58 => {
    //   block [0x82621A58..0x82621ABC)
	// 82621A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621A64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621A6C: 38AA0664  addi r5, r10, 0x664
	ctx.r[5].s64 = ctx.r[10].s64 + 1636;
	// 82621A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621A78: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82621A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621A8C: 386A0604  addi r3, r10, 0x604
	ctx.r[3].s64 = ctx.r[10].s64 + 1540;
	// 82621A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621A94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621A98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621AA8: 4BE45379  bl 0x82466e20
	ctx.lr = 0x82621AAC;
	sub_82466E20(ctx, base);
	// 82621AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621AC0 size=100
    let mut pc: u32 = 0x82621AC0;
    'dispatch: loop {
        match pc {
            0x82621AC0 => {
    //   block [0x82621AC0..0x82621B24)
	// 82621AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621ACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621AD4: 38AA05A4  addi r5, r10, 0x5a4
	ctx.r[5].s64 = ctx.r[10].s64 + 1444;
	// 82621AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621AE0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82621AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621AF4: 386A0634  addi r3, r10, 0x634
	ctx.r[3].s64 = ctx.r[10].s64 + 1588;
	// 82621AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621AFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621B00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621B08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621B10: 4BE45311  bl 0x82466e20
	ctx.lr = 0x82621B14;
	sub_82466E20(ctx, base);
	// 82621B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621B28 size=104
    let mut pc: u32 = 0x82621B28;
    'dispatch: loop {
        match pc {
            0x82621B28 => {
    //   block [0x82621B28..0x82621B90)
	// 82621B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621B34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621B3C: 392A1AE4  addi r9, r10, 0x1ae4
	ctx.r[9].s64 = ctx.r[10].s64 + 6884;
	// 82621B40: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621B48: 38AA05D4  addi r5, r10, 0x5d4
	ctx.r[5].s64 = ctx.r[10].s64 + 1492;
	// 82621B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621B5C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82621B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621B64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621B68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621B70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621B74: 386A0664  addi r3, r10, 0x664
	ctx.r[3].s64 = ctx.r[10].s64 + 1636;
	// 82621B78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82621B7C: 4BE452A5  bl 0x82466e20
	ctx.lr = 0x82621B80;
	sub_82466E20(ctx, base);
	// 82621B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621B90 size=108
    let mut pc: u32 = 0x82621B90;
    'dispatch: loop {
        match pc {
            0x82621B90 => {
    //   block [0x82621B90..0x82621BFC)
	// 82621B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621B9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621BA4: 38EB1CF0  addi r7, r11, 0x1cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 7408;
	// 82621BA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82621BAC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82621BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82621BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621BC0: 386A0694  addi r3, r10, 0x694
	ctx.r[3].s64 = ctx.r[10].s64 + 1684;
	// 82621BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621BE8: 4BE45239  bl 0x82466e20
	ctx.lr = 0x82621BEC;
	sub_82466E20(ctx, base);
	// 82621BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621C00 size=112
    let mut pc: u32 = 0x82621C00;
    'dispatch: loop {
        match pc {
            0x82621C00 => {
    //   block [0x82621C00..0x82621C70)
	// 82621C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621C0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621C14: 38AA0664  addi r5, r10, 0x664
	ctx.r[5].s64 = ctx.r[10].s64 + 1636;
	// 82621C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621C1C: 390B1D20  addi r8, r11, 0x1d20
	ctx.r[8].s64 = ctx.r[11].s64 + 7456;
	// 82621C20: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82621C24: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82621C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621C38: 386A06C4  addi r3, r10, 0x6c4
	ctx.r[3].s64 = ctx.r[10].s64 + 1732;
	// 82621C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621C5C: 4BE451C5  bl 0x82466e20
	ctx.lr = 0x82621C60;
	sub_82466E20(ctx, base);
	// 82621C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621C70 size=116
    let mut pc: u32 = 0x82621C70;
    'dispatch: loop {
        match pc {
            0x82621C70 => {
    //   block [0x82621C70..0x82621CE4)
	// 82621C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621C7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621C84: 390B1DCC  addi r8, r11, 0x1dcc
	ctx.r[8].s64 = ctx.r[11].s64 + 7628;
	// 82621C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621C8C: 392A1B48  addi r9, r10, 0x1b48
	ctx.r[9].s64 = ctx.r[10].s64 + 6984;
	// 82621C90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C94: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82621C98: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621C9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621CA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621CB4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82621CB8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82621CBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82621CC0: 386B06F4  addi r3, r11, 0x6f4
	ctx.r[3].s64 = ctx.r[11].s64 + 1780;
	// 82621CC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82621CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621CD0: 4BE45151  bl 0x82466e20
	ctx.lr = 0x82621CD4;
	sub_82466E20(ctx, base);
	// 82621CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621CE8 size=112
    let mut pc: u32 = 0x82621CE8;
    'dispatch: loop {
        match pc {
            0x82621CE8 => {
    //   block [0x82621CE8..0x82621D58)
	// 82621CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621CF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621CF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621CFC: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621D04: 390B1DE4  addi r8, r11, 0x1de4
	ctx.r[8].s64 = ctx.r[11].s64 + 7652;
	// 82621D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82621D0C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82621D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621D20: 386A0724  addi r3, r10, 0x724
	ctx.r[3].s64 = ctx.r[10].s64 + 1828;
	// 82621D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621D44: 4BE450DD  bl 0x82466e20
	ctx.lr = 0x82621D48;
	sub_82466E20(ctx, base);
	// 82621D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621D58 size=100
    let mut pc: u32 = 0x82621D58;
    'dispatch: loop {
        match pc {
            0x82621D58 => {
    //   block [0x82621D58..0x82621DBC)
	// 82621D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621D6C: 38AA0784  addi r5, r10, 0x784
	ctx.r[5].s64 = ctx.r[10].s64 + 1924;
	// 82621D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621D78: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82621D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621D8C: 386A0754  addi r3, r10, 0x754
	ctx.r[3].s64 = ctx.r[10].s64 + 1876;
	// 82621D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621D94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621D98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621DA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621DA8: 4BE45079  bl 0x82466e20
	ctx.lr = 0x82621DAC;
	sub_82466E20(ctx, base);
	// 82621DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621DC0 size=112
    let mut pc: u32 = 0x82621DC0;
    'dispatch: loop {
        match pc {
            0x82621DC0 => {
    //   block [0x82621DC0..0x82621E30)
	// 82621DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621DD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621DD4: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621DDC: 390B1DFC  addi r8, r11, 0x1dfc
	ctx.r[8].s64 = ctx.r[11].s64 + 7676;
	// 82621DE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621DE4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82621DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621DF8: 386A0784  addi r3, r10, 0x784
	ctx.r[3].s64 = ctx.r[10].s64 + 1924;
	// 82621DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621E1C: 4BE45005  bl 0x82466e20
	ctx.lr = 0x82621E20;
	sub_82466E20(ctx, base);
	// 82621E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621E30 size=112
    let mut pc: u32 = 0x82621E30;
    'dispatch: loop {
        match pc {
            0x82621E30 => {
    //   block [0x82621E30..0x82621EA0)
	// 82621E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621E40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621E44: 38AA06F4  addi r5, r10, 0x6f4
	ctx.r[5].s64 = ctx.r[10].s64 + 1780;
	// 82621E48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621E4C: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 82621E50: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82621E54: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82621E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621E68: 386A07B4  addi r3, r10, 0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1972;
	// 82621E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621E8C: 4BE44F95  bl 0x82466e20
	ctx.lr = 0x82621E90;
	sub_82466E20(ctx, base);
	// 82621E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621EA0 size=100
    let mut pc: u32 = 0x82621EA0;
    'dispatch: loop {
        match pc {
            0x82621EA0 => {
    //   block [0x82621EA0..0x82621F04)
	// 82621EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621EB4: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621EC0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82621EC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621ED4: 386A07E4  addi r3, r10, 0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2020;
	// 82621ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621EE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621EE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621EF0: 4BE44F31  bl 0x82466e20
	ctx.lr = 0x82621EF4;
	sub_82466E20(ctx, base);
	// 82621EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621F08 size=100
    let mut pc: u32 = 0x82621F08;
    'dispatch: loop {
        match pc {
            0x82621F08 => {
    //   block [0x82621F08..0x82621F6C)
	// 82621F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621F1C: 38AA0724  addi r5, r10, 0x724
	ctx.r[5].s64 = ctx.r[10].s64 + 1828;
	// 82621F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621F28: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82621F2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621F3C: 386A0814  addi r3, r10, 0x814
	ctx.r[3].s64 = ctx.r[10].s64 + 2068;
	// 82621F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621F44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621F48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621F50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621F58: 4BE44EC9  bl 0x82466e20
	ctx.lr = 0x82621F5C;
	sub_82466E20(ctx, base);
	// 82621F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621F70 size=100
    let mut pc: u32 = 0x82621F70;
    'dispatch: loop {
        match pc {
            0x82621F70 => {
    //   block [0x82621F70..0x82621FD4)
	// 82621F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621F7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621F84: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82621F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621F90: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82621F94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621FA4: 386A0844  addi r3, r10, 0x844
	ctx.r[3].s64 = ctx.r[10].s64 + 2116;
	// 82621FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621FAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621FB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621FB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621FC0: 4BE44E61  bl 0x82466e20
	ctx.lr = 0x82621FC4;
	sub_82466E20(ctx, base);
	// 82621FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621FD8 size=100
    let mut pc: u32 = 0x82621FD8;
    'dispatch: loop {
        match pc {
            0x82621FD8 => {
    //   block [0x82621FD8..0x8262203C)
	// 82621FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621FEC: 38AA0724  addi r5, r10, 0x724
	ctx.r[5].s64 = ctx.r[10].s64 + 1828;
	// 82621FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621FF8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82621FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262200C: 386A0874  addi r3, r10, 0x874
	ctx.r[3].s64 = ctx.r[10].s64 + 2164;
	// 82622010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262201C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82622024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622028: 4BE44DF9  bl 0x82466e20
	ctx.lr = 0x8262202C;
	sub_82466E20(ctx, base);
	// 8262202C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622040 size=112
    let mut pc: u32 = 0x82622040;
    'dispatch: loop {
        match pc {
            0x82622040 => {
    //   block [0x82622040..0x826220B0)
	// 82622040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262204C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622050: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622054: 38AA0904  addi r5, r10, 0x904
	ctx.r[5].s64 = ctx.r[10].s64 + 2308;
	// 82622058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262205C: 390B1ED8  addi r8, r11, 0x1ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 7896;
	// 82622060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622064: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82622068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262206C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622078: 386A08A4  addi r3, r10, 0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + 2212;
	// 8262207C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262209C: 4BE44D85  bl 0x82466e20
	ctx.lr = 0x826220A0;
	sub_82466E20(ctx, base);
	// 826220A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826220A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826220A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826220AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826220B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826220B0 size=112
    let mut pc: u32 = 0x826220B0;
    'dispatch: loop {
        match pc {
            0x826220B0 => {
    //   block [0x826220B0..0x82622120)
	// 826220B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826220B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826220B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826220BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826220C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826220C4: 38AA0934  addi r5, r10, 0x934
	ctx.r[5].s64 = ctx.r[10].s64 + 2356;
	// 826220C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826220CC: 390B1F08  addi r8, r11, 0x1f08
	ctx.r[8].s64 = ctx.r[11].s64 + 7944;
	// 826220D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826220D4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826220D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826220DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826220E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826220E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826220E8: 386A08D4  addi r3, r10, 0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2260;
	// 826220EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826220F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826220F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826220F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826220FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262210C: 4BE44D15  bl 0x82466e20
	ctx.lr = 0x82622110;
	sub_82466E20(ctx, base);
	// 82622110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262211C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622120 size=112
    let mut pc: u32 = 0x82622120;
    'dispatch: loop {
        match pc {
            0x82622120 => {
    //   block [0x82622120..0x82622190)
	// 82622120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262212C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622130: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622134: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82622138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262213C: 390B1F20  addi r8, r11, 0x1f20
	ctx.r[8].s64 = ctx.r[11].s64 + 7968;
	// 82622140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622144: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82622148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262214C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622158: 386A0904  addi r3, r10, 0x904
	ctx.r[3].s64 = ctx.r[10].s64 + 2308;
	// 8262215C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262216C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262217C: 4BE44CA5  bl 0x82466e20
	ctx.lr = 0x82622180;
	sub_82466E20(ctx, base);
	// 82622180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262218C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622190 size=112
    let mut pc: u32 = 0x82622190;
    'dispatch: loop {
        match pc {
            0x82622190 => {
    //   block [0x82622190..0x82622200)
	// 82622190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262219C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826221A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826221A4: 38AA0904  addi r5, r10, 0x904
	ctx.r[5].s64 = ctx.r[10].s64 + 2308;
	// 826221A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826221AC: 390B1F50  addi r8, r11, 0x1f50
	ctx.r[8].s64 = ctx.r[11].s64 + 8016;
	// 826221B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826221B4: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826221B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826221BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826221C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826221C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826221C8: 386A0934  addi r3, r10, 0x934
	ctx.r[3].s64 = ctx.r[10].s64 + 2356;
	// 826221CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826221D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826221D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826221D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826221DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826221E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826221E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826221E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826221EC: 4BE44C35  bl 0x82466e20
	ctx.lr = 0x826221F0;
	sub_82466E20(ctx, base);
	// 826221F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826221F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826221F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826221FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622200 size=112
    let mut pc: u32 = 0x82622200;
    'dispatch: loop {
        match pc {
            0x82622200 => {
    //   block [0x82622200..0x82622270)
	// 82622200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262220C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622210: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622214: 38AA0934  addi r5, r10, 0x934
	ctx.r[5].s64 = ctx.r[10].s64 + 2356;
	// 82622218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262221C: 390B1F68  addi r8, r11, 0x1f68
	ctx.r[8].s64 = ctx.r[11].s64 + 8040;
	// 82622220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622224: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82622228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262222C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622238: 386A0964  addi r3, r10, 0x964
	ctx.r[3].s64 = ctx.r[10].s64 + 2404;
	// 8262223C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262224C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262225C: 4BE44BC5  bl 0x82466e20
	ctx.lr = 0x82622260;
	sub_82466E20(ctx, base);
	// 82622260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262226C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622270 size=112
    let mut pc: u32 = 0x82622270;
    'dispatch: loop {
        match pc {
            0x82622270 => {
    //   block [0x82622270..0x826222E0)
	// 82622270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262227C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622280: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622284: 392A1B74  addi r9, r10, 0x1b74
	ctx.r[9].s64 = ctx.r[10].s64 + 7028;
	// 82622288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262228C: 390B1F80  addi r8, r11, 0x1f80
	ctx.r[8].s64 = ctx.r[11].s64 + 8064;
	// 82622290: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82622294: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82622298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262229C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826222A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826222A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826222A8: 386A0994  addi r3, r10, 0x994
	ctx.r[3].s64 = ctx.r[10].s64 + 2452;
	// 826222AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826222B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826222B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826222B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826222BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826222C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826222C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826222C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826222CC: 4BE44B55  bl 0x82466e20
	ctx.lr = 0x826222D0;
	sub_82466E20(ctx, base);
	// 826222D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826222D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826222D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826222DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826222E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826222E0 size=112
    let mut pc: u32 = 0x826222E0;
    'dispatch: loop {
        match pc {
            0x826222E0 => {
    //   block [0x826222E0..0x82622350)
	// 826222E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826222E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826222E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826222EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826222F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826222F4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826222F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826222FC: 390B1FB0  addi r8, r11, 0x1fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 8112;
	// 82622300: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82622304: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82622308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262230C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622318: 386A09C4  addi r3, r10, 0x9c4
	ctx.r[3].s64 = ctx.r[10].s64 + 2500;
	// 8262231C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262232C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262233C: 4BE44AE5  bl 0x82466e20
	ctx.lr = 0x82622340;
	sub_82466E20(ctx, base);
	// 82622340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262234C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82622350 size=48
    let mut pc: u32 = 0x82622350;
    'dispatch: loop {
        match pc {
            0x82622350 => {
    //   block [0x82622350..0x82622380)
	// 82622350: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622354: 814B204C  lwz r10, 0x204c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) } as u64;
	// 82622358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262235C: 396B3F90  addi r11, r11, 0x3f90
	ctx.r[11].s64 = ctx.r[11].s64 + 16272;
	// 82622360: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82622364: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622368: 814A2048  lwz r10, 0x2048(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8264 as u32) ) } as u64;
	// 8262236C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82622370: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622374: 814A2044  lwz r10, 0x2044(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82622378: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8262237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622380 size=116
    let mut pc: u32 = 0x82622380;
    'dispatch: loop {
        match pc {
            0x82622380 => {
    //   block [0x82622380..0x826223F4)
	// 82622380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262238C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82622390: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622394: 392B1C68  addi r9, r11, 0x1c68
	ctx.r[9].s64 = ctx.r[11].s64 + 7272;
	// 82622398: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262239C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826223A0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826223A4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826223A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826223AC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826223B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826223B4: 396B3F90  addi r11, r11, 0x3f90
	ctx.r[11].s64 = ctx.r[11].s64 + 16272;
	// 826223B8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826223BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826223C0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826223C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826223C8: 386A09F4  addi r3, r10, 0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2548;
	// 826223CC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826223D0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826223D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826223D8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826223DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826223E0: 4BE44A41  bl 0x82466e20
	ctx.lr = 0x826223E4;
	sub_82466E20(ctx, base);
	// 826223E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826223E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826223EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826223F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826223F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826223F8 size=116
    let mut pc: u32 = 0x826223F8;
    'dispatch: loop {
        match pc {
            0x826223F8 => {
    //   block [0x826223F8..0x8262246C)
	// 826223F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826223FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622404: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262240C: 390B2058  addi r8, r11, 0x2058
	ctx.r[8].s64 = ctx.r[11].s64 + 8280;
	// 82622410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622414: 392A1D50  addi r9, r10, 0x1d50
	ctx.r[9].s64 = ctx.r[10].s64 + 7504;
	// 82622418: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262241C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82622420: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82622424: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262242C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262243C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82622440: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82622444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82622448: 386B0A24  addi r3, r11, 0xa24
	ctx.r[3].s64 = ctx.r[11].s64 + 2596;
	// 8262244C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82622450: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622458: 4BE449C9  bl 0x82466e20
	ctx.lr = 0x8262245C;
	sub_82466E20(ctx, base);
	// 8262245C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622470 size=112
    let mut pc: u32 = 0x82622470;
    'dispatch: loop {
        match pc {
            0x82622470 => {
    //   block [0x82622470..0x826224E0)
	// 82622470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262247C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622480: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622484: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82622488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262248C: 390B20D0  addi r8, r11, 0x20d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8400;
	// 82622490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622494: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82622498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262249C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826224A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826224A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826224A8: 386A0A54  addi r3, r10, 0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + 2644;
	// 826224AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826224B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826224B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826224B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826224BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826224C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826224C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826224C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826224CC: 4BE44955  bl 0x82466e20
	ctx.lr = 0x826224D0;
	sub_82466E20(ctx, base);
	// 826224D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826224D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826224D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826224DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826224E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826224E0 size=112
    let mut pc: u32 = 0x826224E0;
    'dispatch: loop {
        match pc {
            0x826224E0 => {
    //   block [0x826224E0..0x82622550)
	// 826224E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826224E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826224E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826224EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826224F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826224F4: 38AAF224  addi r5, r10, -0xddc
	ctx.r[5].s64 = ctx.r[10].s64 + -3548;
	// 826224F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826224FC: 390B20E8  addi r8, r11, 0x20e8
	ctx.r[8].s64 = ctx.r[11].s64 + 8424;
	// 82622500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622504: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82622508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262250C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622518: 386A0A84  addi r3, r10, 0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + 2692;
	// 8262251C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262252C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262253C: 4BE448E5  bl 0x82466e20
	ctx.lr = 0x82622540;
	sub_82466E20(ctx, base);
	// 82622540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622550 size=108
    let mut pc: u32 = 0x82622550;
    'dispatch: loop {
        match pc {
            0x82622550 => {
    //   block [0x82622550..0x826225BC)
	// 82622550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262255C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622564: 38EB2100  addi r7, r11, 0x2100
	ctx.r[7].s64 = ctx.r[11].s64 + 8448;
	// 82622568: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262256C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82622570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622580: 386A0AB4  addi r3, r10, 0xab4
	ctx.r[3].s64 = ctx.r[10].s64 + 2740;
	// 82622584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262259C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826225A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826225A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826225A8: 4BE44879  bl 0x82466e20
	ctx.lr = 0x826225AC;
	sub_82466E20(ctx, base);
	// 826225AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826225B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826225B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826225B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826225C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826225C0 size=112
    let mut pc: u32 = 0x826225C0;
    'dispatch: loop {
        match pc {
            0x826225C0 => {
    //   block [0x826225C0..0x82622630)
	// 826225C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826225C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826225C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826225CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826225D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826225D4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826225D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826225DC: 390B2118  addi r8, r11, 0x2118
	ctx.r[8].s64 = ctx.r[11].s64 + 8472;
	// 826225E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826225E4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826225E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826225EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826225F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826225F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826225F8: 386A0AE4  addi r3, r10, 0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + 2788;
	// 826225FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262260C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262261C: 4BE44805  bl 0x82466e20
	ctx.lr = 0x82622620;
	sub_82466E20(ctx, base);
	// 82622620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262262C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622630 size=108
    let mut pc: u32 = 0x82622630;
    'dispatch: loop {
        match pc {
            0x82622630 => {
    //   block [0x82622630..0x8262269C)
	// 82622630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262263C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622644: 38EB2160  addi r7, r11, 0x2160
	ctx.r[7].s64 = ctx.r[11].s64 + 8544;
	// 82622648: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262264C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82622650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622660: 386A0B14  addi r3, r10, 0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + 2836;
	// 82622664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262267C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622688: 4BE44799  bl 0x82466e20
	ctx.lr = 0x8262268C;
	sub_82466E20(ctx, base);
	// 8262268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826226A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826226A0 size=112
    let mut pc: u32 = 0x826226A0;
    'dispatch: loop {
        match pc {
            0x826226A0 => {
    //   block [0x826226A0..0x82622710)
	// 826226A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826226A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826226A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826226AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826226B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826226B4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826226B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826226BC: 390B2178  addi r8, r11, 0x2178
	ctx.r[8].s64 = ctx.r[11].s64 + 8568;
	// 826226C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826226C4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826226C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826226CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826226D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826226D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826226D8: 386A0B44  addi r3, r10, 0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + 2884;
	// 826226DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826226E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826226E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826226E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826226EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826226F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826226F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826226F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826226FC: 4BE44725  bl 0x82466e20
	ctx.lr = 0x82622700;
	sub_82466E20(ctx, base);
	// 82622700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262270C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622710 size=112
    let mut pc: u32 = 0x82622710;
    'dispatch: loop {
        match pc {
            0x82622710 => {
    //   block [0x82622710..0x82622780)
	// 82622710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262271C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622720: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622724: 38AA0C04  addi r5, r10, 0xc04
	ctx.r[5].s64 = ctx.r[10].s64 + 3076;
	// 82622728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262272C: 390B21A8  addi r8, r11, 0x21a8
	ctx.r[8].s64 = ctx.r[11].s64 + 8616;
	// 82622730: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82622734: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82622738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262273C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622748: 386A0B74  addi r3, r10, 0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + 2932;
	// 8262274C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262275C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262276C: 4BE446B5  bl 0x82466e20
	ctx.lr = 0x82622770;
	sub_82466E20(ctx, base);
	// 82622770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622780 size=108
    let mut pc: u32 = 0x82622780;
    'dispatch: loop {
        match pc {
            0x82622780 => {
    //   block [0x82622780..0x826227EC)
	// 82622780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262278C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622794: 38EB2220  addi r7, r11, 0x2220
	ctx.r[7].s64 = ctx.r[11].s64 + 8736;
	// 82622798: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262279C: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 826227A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826227A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826227A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826227AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826227B0: 386A0BA4  addi r3, r10, 0xba4
	ctx.r[3].s64 = ctx.r[10].s64 + 2980;
	// 826227B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826227B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826227BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826227C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826227C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826227C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826227CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826227D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826227D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826227D8: 4BE44649  bl 0x82466e20
	ctx.lr = 0x826227DC;
	sub_82466E20(ctx, base);
	// 826227DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826227E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826227E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826227E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826227F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826227F0 size=108
    let mut pc: u32 = 0x826227F0;
    'dispatch: loop {
        match pc {
            0x826227F0 => {
    //   block [0x826227F0..0x8262285C)
	// 826227F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826227F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826227F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826227FC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622804: 38EB2268  addi r7, r11, 0x2268
	ctx.r[7].s64 = ctx.r[11].s64 + 8808;
	// 82622808: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262280C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82622810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262281C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622820: 386A0BD4  addi r3, r10, 0xbd4
	ctx.r[3].s64 = ctx.r[10].s64 + 3028;
	// 82622824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262282C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262283C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622848: 4BE445D9  bl 0x82466e20
	ctx.lr = 0x8262284C;
	sub_82466E20(ctx, base);
	// 8262284C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622860 size=116
    let mut pc: u32 = 0x82622860;
    'dispatch: loop {
        match pc {
            0x82622860 => {
    //   block [0x82622860..0x826228D4)
	// 82622860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262286C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622870: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82622874: 390A22B0  addi r8, r10, 0x22b0
	ctx.r[8].s64 = ctx.r[10].s64 + 8880;
	// 82622878: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262287C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82622880: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82622884: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622888: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262288C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622894: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82622898: 396B1D64  addi r11, r11, 0x1d64
	ctx.r[11].s64 = ctx.r[11].s64 + 7524;
	// 8262289C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826228A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826228A4: 386A0C04  addi r3, r10, 0xc04
	ctx.r[3].s64 = ctx.r[10].s64 + 3076;
	// 826228A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826228AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826228B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826228B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826228B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826228BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826228C0: 4BE44561  bl 0x82466e20
	ctx.lr = 0x826228C4;
	sub_82466E20(ctx, base);
	// 826228C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826228C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826228CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826228D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826228D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826228D8 size=108
    let mut pc: u32 = 0x826228D8;
    'dispatch: loop {
        match pc {
            0x826228D8 => {
    //   block [0x826228D8..0x82622944)
	// 826228D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826228DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826228E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826228E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826228E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826228EC: 38EB2388  addi r7, r11, 0x2388
	ctx.r[7].s64 = ctx.r[11].s64 + 9096;
	// 826228F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826228F4: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 826228F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826228FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622908: 386A0C34  addi r3, r10, 0xc34
	ctx.r[3].s64 = ctx.r[10].s64 + 3124;
	// 8262290C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262291C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262292C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622930: 4BE444F1  bl 0x82466e20
	ctx.lr = 0x82622934;
	sub_82466E20(ctx, base);
	// 82622934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622948 size=108
    let mut pc: u32 = 0x82622948;
    'dispatch: loop {
        match pc {
            0x82622948 => {
    //   block [0x82622948..0x826229B4)
	// 82622948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622954: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262295C: 38EB23B8  addi r7, r11, 0x23b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9144;
	// 82622960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82622964: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 82622968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262296C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622978: 386A0C64  addi r3, r10, 0xc64
	ctx.r[3].s64 = ctx.r[10].s64 + 3172;
	// 8262297C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262298C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262299C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826229A0: 4BE44481  bl 0x82466e20
	ctx.lr = 0x826229A4;
	sub_82466E20(ctx, base);
	// 826229A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826229A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826229AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826229B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826229B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826229B8 size=112
    let mut pc: u32 = 0x826229B8;
    'dispatch: loop {
        match pc {
            0x826229B8 => {
    //   block [0x826229B8..0x82622A28)
	// 826229B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826229BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826229C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826229C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826229C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826229CC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826229D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826229D4: 390B23E8  addi r8, r11, 0x23e8
	ctx.r[8].s64 = ctx.r[11].s64 + 9192;
	// 826229D8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826229DC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826229E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826229E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826229E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826229EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826229F0: 386A0C94  addi r3, r10, 0xc94
	ctx.r[3].s64 = ctx.r[10].s64 + 3220;
	// 826229F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826229F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826229FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622A14: 4BE4440D  bl 0x82466e20
	ctx.lr = 0x82622A18;
	sub_82466E20(ctx, base);
	// 82622A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622A28 size=112
    let mut pc: u32 = 0x82622A28;
    'dispatch: loop {
        match pc {
            0x82622A28 => {
    //   block [0x82622A28..0x82622A98)
	// 82622A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622A38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622A3C: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622A44: 390B24C0  addi r8, r11, 0x24c0
	ctx.r[8].s64 = ctx.r[11].s64 + 9408;
	// 82622A48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82622A4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82622A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622A54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622A60: 386A0CC4  addi r3, r10, 0xcc4
	ctx.r[3].s64 = ctx.r[10].s64 + 3268;
	// 82622A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622A84: 4BE4439D  bl 0x82466e20
	ctx.lr = 0x82622A88;
	sub_82466E20(ctx, base);
	// 82622A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622A98 size=112
    let mut pc: u32 = 0x82622A98;
    'dispatch: loop {
        match pc {
            0x82622A98 => {
    //   block [0x82622A98..0x82622B08)
	// 82622A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622AAC: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622AB4: 390B2508  addi r8, r11, 0x2508
	ctx.r[8].s64 = ctx.r[11].s64 + 9480;
	// 82622AB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622ABC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82622AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622AD0: 386A0CF4  addi r3, r10, 0xcf4
	ctx.r[3].s64 = ctx.r[10].s64 + 3316;
	// 82622AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622AF4: 4BE4432D  bl 0x82466e20
	ctx.lr = 0x82622AF8;
	sub_82466E20(ctx, base);
	// 82622AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622B08 size=112
    let mut pc: u32 = 0x82622B08;
    'dispatch: loop {
        match pc {
            0x82622B08 => {
    //   block [0x82622B08..0x82622B78)
	// 82622B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622B14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622B1C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82622B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622B24: 390B2568  addi r8, r11, 0x2568
	ctx.r[8].s64 = ctx.r[11].s64 + 9576;
	// 82622B28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622B2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82622B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622B34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622B40: 386A0D24  addi r3, r10, 0xd24
	ctx.r[3].s64 = ctx.r[10].s64 + 3364;
	// 82622B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622B64: 4BE442BD  bl 0x82466e20
	ctx.lr = 0x82622B68;
	sub_82466E20(ctx, base);
	// 82622B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622B78 size=112
    let mut pc: u32 = 0x82622B78;
    'dispatch: loop {
        match pc {
            0x82622B78 => {
    //   block [0x82622B78..0x82622BE8)
	// 82622B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622B84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622B8C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82622B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622B94: 390B25C8  addi r8, r11, 0x25c8
	ctx.r[8].s64 = ctx.r[11].s64 + 9672;
	// 82622B98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622B9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82622BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622BB0: 386A0D54  addi r3, r10, 0xd54
	ctx.r[3].s64 = ctx.r[10].s64 + 3412;
	// 82622BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622BD4: 4BE4424D  bl 0x82466e20
	ctx.lr = 0x82622BD8;
	sub_82466E20(ctx, base);
	// 82622BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622BE8 size=112
    let mut pc: u32 = 0x82622BE8;
    'dispatch: loop {
        match pc {
            0x82622BE8 => {
    //   block [0x82622BE8..0x82622C58)
	// 82622BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622BF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622BFC: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622C04: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 82622C08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82622C0C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82622C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622C20: 386A0D84  addi r3, r10, 0xd84
	ctx.r[3].s64 = ctx.r[10].s64 + 3460;
	// 82622C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622C44: 4BE441DD  bl 0x82466e20
	ctx.lr = 0x82622C48;
	sub_82466E20(ctx, base);
	// 82622C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622C58 size=108
    let mut pc: u32 = 0x82622C58;
    'dispatch: loop {
        match pc {
            0x82622C58 => {
    //   block [0x82622C58..0x82622CC4)
	// 82622C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622C64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622C6C: 38EB26E8  addi r7, r11, 0x26e8
	ctx.r[7].s64 = ctx.r[11].s64 + 9960;
	// 82622C70: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82622C74: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82622C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622C88: 386A0DB4  addi r3, r10, 0xdb4
	ctx.r[3].s64 = ctx.r[10].s64 + 3508;
	// 82622C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622CB0: 4BE44171  bl 0x82466e20
	ctx.lr = 0x82622CB4;
	sub_82466E20(ctx, base);
	// 82622CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622CC8 size=112
    let mut pc: u32 = 0x82622CC8;
    'dispatch: loop {
        match pc {
            0x82622CC8 => {
    //   block [0x82622CC8..0x82622D38)
	// 82622CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622CD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622CDC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622CE4: 390B2880  addi r8, r11, 0x2880
	ctx.r[8].s64 = ctx.r[11].s64 + 10368;
	// 82622CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622CEC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82622CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622CF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622D00: 386A0DE4  addi r3, r10, 0xde4
	ctx.r[3].s64 = ctx.r[10].s64 + 3556;
	// 82622D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622D24: 4BE440FD  bl 0x82466e20
	ctx.lr = 0x82622D28;
	sub_82466E20(ctx, base);
	// 82622D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622D38 size=112
    let mut pc: u32 = 0x82622D38;
    'dispatch: loop {
        match pc {
            0x82622D38 => {
    //   block [0x82622D38..0x82622DA8)
	// 82622D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622D48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622D4C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622D54: 390B2898  addi r8, r11, 0x2898
	ctx.r[8].s64 = ctx.r[11].s64 + 10392;
	// 82622D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622D5C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82622D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622D70: 386A0E14  addi r3, r10, 0xe14
	ctx.r[3].s64 = ctx.r[10].s64 + 3604;
	// 82622D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622D84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82622D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622D94: 4BE4408D  bl 0x82466e20
	ctx.lr = 0x82622D98;
	sub_82466E20(ctx, base);
	// 82622D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622DA8 size=112
    let mut pc: u32 = 0x82622DA8;
    'dispatch: loop {
        match pc {
            0x82622DA8 => {
    //   block [0x82622DA8..0x82622E18)
	// 82622DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622DB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622DBC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622DC4: 390B28B0  addi r8, r11, 0x28b0
	ctx.r[8].s64 = ctx.r[11].s64 + 10416;
	// 82622DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622DCC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82622DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622DD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622DE0: 386A0E44  addi r3, r10, 0xe44
	ctx.r[3].s64 = ctx.r[10].s64 + 3652;
	// 82622DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622E04: 4BE4401D  bl 0x82466e20
	ctx.lr = 0x82622E08;
	sub_82466E20(ctx, base);
	// 82622E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622E18 size=108
    let mut pc: u32 = 0x82622E18;
    'dispatch: loop {
        match pc {
            0x82622E18 => {
    //   block [0x82622E18..0x82622E84)
	// 82622E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622E24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622E2C: 38EB28E0  addi r7, r11, 0x28e0
	ctx.r[7].s64 = ctx.r[11].s64 + 10464;
	// 82622E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82622E34: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82622E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622E48: 386A0E74  addi r3, r10, 0xe74
	ctx.r[3].s64 = ctx.r[10].s64 + 3700;
	// 82622E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622E70: 4BE43FB1  bl 0x82466e20
	ctx.lr = 0x82622E74;
	sub_82466E20(ctx, base);
	// 82622E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622E88 size=112
    let mut pc: u32 = 0x82622E88;
    'dispatch: loop {
        match pc {
            0x82622E88 => {
    //   block [0x82622E88..0x82622EF8)
	// 82622E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622E98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622E9C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622EA4: 390B2910  addi r8, r11, 0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + 10512;
	// 82622EA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622EAC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82622EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622EB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622EC0: 386A0EA4  addi r3, r10, 0xea4
	ctx.r[3].s64 = ctx.r[10].s64 + 3748;
	// 82622EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622ED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82622ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622EE4: 4BE43F3D  bl 0x82466e20
	ctx.lr = 0x82622EE8;
	sub_82466E20(ctx, base);
	// 82622EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


