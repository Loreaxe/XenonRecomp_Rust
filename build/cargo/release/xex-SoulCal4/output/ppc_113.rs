pub fn sub_8268A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A490 size=112
    let mut pc: u32 = 0x8268A490;
    'dispatch: loop {
        match pc {
            0x8268A490 => {
    //   block [0x8268A490..0x8268A500)
	// 8268A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A49C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A4A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A4A4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A4AC: 390B3270  addi r8, r11, 0x3270
	ctx.r[8].s64 = ctx.r[11].s64 + 12912;
	// 8268A4B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268A4B4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8268A4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A4BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A4C8: 386ACA50  addi r3, r10, -0x35b0
	ctx.r[3].s64 = ctx.r[10].s64 + -13744;
	// 8268A4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A4EC: 4BDDC935  bl 0x82466e20
	ctx.lr = 0x8268A4F0;
	sub_82466E20(ctx, base);
	// 8268A4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A500 size=112
    let mut pc: u32 = 0x8268A500;
    'dispatch: loop {
        match pc {
            0x8268A500 => {
    //   block [0x8268A500..0x8268A570)
	// 8268A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A50C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A510: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A514: 38AAB250  addi r5, r10, -0x4db0
	ctx.r[5].s64 = ctx.r[10].s64 + -19888;
	// 8268A518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A51C: 390B3288  addi r8, r11, 0x3288
	ctx.r[8].s64 = ctx.r[11].s64 + 12936;
	// 8268A520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268A524: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8268A528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A52C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A538: 386ACA80  addi r3, r10, -0x3580
	ctx.r[3].s64 = ctx.r[10].s64 + -13696;
	// 8268A53C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A55C: 4BDDC8C5  bl 0x82466e20
	ctx.lr = 0x8268A560;
	sub_82466E20(ctx, base);
	// 8268A560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A570 size=108
    let mut pc: u32 = 0x8268A570;
    'dispatch: loop {
        match pc {
            0x8268A570 => {
    //   block [0x8268A570..0x8268A5DC)
	// 8268A570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A57C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A584: 38EB32A0  addi r7, r11, 0x32a0
	ctx.r[7].s64 = ctx.r[11].s64 + 12960;
	// 8268A588: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268A58C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8268A590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A5A0: 386ACAB0  addi r3, r10, -0x3550
	ctx.r[3].s64 = ctx.r[10].s64 + -13648;
	// 8268A5A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A5B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A5C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A5C8: 4BDDC859  bl 0x82466e20
	ctx.lr = 0x8268A5CC;
	sub_82466E20(ctx, base);
	// 8268A5CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A5E0 size=112
    let mut pc: u32 = 0x8268A5E0;
    'dispatch: loop {
        match pc {
            0x8268A5E0 => {
    //   block [0x8268A5E0..0x8268A650)
	// 8268A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A5EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A5F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A5F4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A5FC: 390B32B8  addi r8, r11, 0x32b8
	ctx.r[8].s64 = ctx.r[11].s64 + 12984;
	// 8268A600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268A604: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8268A608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A60C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A618: 386ACAE0  addi r3, r10, -0x3520
	ctx.r[3].s64 = ctx.r[10].s64 + -13600;
	// 8268A61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A63C: 4BDDC7E5  bl 0x82466e20
	ctx.lr = 0x8268A640;
	sub_82466E20(ctx, base);
	// 8268A640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A650 size=108
    let mut pc: u32 = 0x8268A650;
    'dispatch: loop {
        match pc {
            0x8268A650 => {
    //   block [0x8268A650..0x8268A6BC)
	// 8268A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A65C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A664: 38EB3300  addi r7, r11, 0x3300
	ctx.r[7].s64 = ctx.r[11].s64 + 13056;
	// 8268A668: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268A66C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8268A670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A680: 386ACB10  addi r3, r10, -0x34f0
	ctx.r[3].s64 = ctx.r[10].s64 + -13552;
	// 8268A684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A6A8: 4BDDC779  bl 0x82466e20
	ctx.lr = 0x8268A6AC;
	sub_82466E20(ctx, base);
	// 8268A6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A6C0 size=112
    let mut pc: u32 = 0x8268A6C0;
    'dispatch: loop {
        match pc {
            0x8268A6C0 => {
    //   block [0x8268A6C0..0x8268A730)
	// 8268A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A6CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A6D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A6D4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A6DC: 390B3318  addi r8, r11, 0x3318
	ctx.r[8].s64 = ctx.r[11].s64 + 13080;
	// 8268A6E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268A6E4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8268A6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A6EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A6F8: 386ACB40  addi r3, r10, -0x34c0
	ctx.r[3].s64 = ctx.r[10].s64 + -13504;
	// 8268A6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A71C: 4BDDC705  bl 0x82466e20
	ctx.lr = 0x8268A720;
	sub_82466E20(ctx, base);
	// 8268A720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A730 size=112
    let mut pc: u32 = 0x8268A730;
    'dispatch: loop {
        match pc {
            0x8268A730 => {
    //   block [0x8268A730..0x8268A7A0)
	// 8268A730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A73C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A740: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A744: 38AACC00  addi r5, r10, -0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + -13312;
	// 8268A748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268A74C: 390B3348  addi r8, r11, 0x3348
	ctx.r[8].s64 = ctx.r[11].s64 + 13128;
	// 8268A750: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8268A754: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 8268A758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A75C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A768: 386ACB70  addi r3, r10, -0x3490
	ctx.r[3].s64 = ctx.r[10].s64 + -13456;
	// 8268A76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A78C: 4BDDC695  bl 0x82466e20
	ctx.lr = 0x8268A790;
	sub_82466E20(ctx, base);
	// 8268A790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A7A0 size=108
    let mut pc: u32 = 0x8268A7A0;
    'dispatch: loop {
        match pc {
            0x8268A7A0 => {
    //   block [0x8268A7A0..0x8268A80C)
	// 8268A7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A7AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A7B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268A7B4: 38EB33C0  addi r7, r11, 0x33c0
	ctx.r[7].s64 = ctx.r[11].s64 + 13248;
	// 8268A7B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268A7BC: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 8268A7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A7D0: 386ACBA0  addi r3, r10, -0x3460
	ctx.r[3].s64 = ctx.r[10].s64 + -13408;
	// 8268A7D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A7F8: 4BDDC629  bl 0x82466e20
	ctx.lr = 0x8268A7FC;
	sub_82466E20(ctx, base);
	// 8268A7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A810 size=108
    let mut pc: u32 = 0x8268A810;
    'dispatch: loop {
        match pc {
            0x8268A810 => {
    //   block [0x8268A810..0x8268A87C)
	// 8268A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A81C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A820: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268A824: 38EB3408  addi r7, r11, 0x3408
	ctx.r[7].s64 = ctx.r[11].s64 + 13320;
	// 8268A828: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268A82C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 8268A830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A834: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A840: 386ACBD0  addi r3, r10, -0x3430
	ctx.r[3].s64 = ctx.r[10].s64 + -13360;
	// 8268A844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A868: 4BDDC5B9  bl 0x82466e20
	ctx.lr = 0x8268A86C;
	sub_82466E20(ctx, base);
	// 8268A86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A880 size=116
    let mut pc: u32 = 0x8268A880;
    'dispatch: loop {
        match pc {
            0x8268A880 => {
    //   block [0x8268A880..0x8268A8F4)
	// 8268A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A88C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8268A890: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8268A894: 390A3450  addi r8, r10, 0x3450
	ctx.r[8].s64 = ctx.r[10].s64 + 13392;
	// 8268A898: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A89C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8268A8A0: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 8268A8A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A8A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268A8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A8B4: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8268A8B8: 396B63D4  addi r11, r11, 0x63d4
	ctx.r[11].s64 = ctx.r[11].s64 + 25556;
	// 8268A8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A8C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A8C4: 386ACC00  addi r3, r10, -0x3400
	ctx.r[3].s64 = ctx.r[10].s64 + -13312;
	// 8268A8C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8268A8CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A8D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8268A8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A8DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A8E0: 4BDDC541  bl 0x82466e20
	ctx.lr = 0x8268A8E4;
	sub_82466E20(ctx, base);
	// 8268A8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A8F8 size=108
    let mut pc: u32 = 0x8268A8F8;
    'dispatch: loop {
        match pc {
            0x8268A8F8 => {
    //   block [0x8268A8F8..0x8268A964)
	// 8268A8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A904: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268A90C: 38EB3528  addi r7, r11, 0x3528
	ctx.r[7].s64 = ctx.r[11].s64 + 13608;
	// 8268A910: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268A914: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 8268A918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A91C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A928: 386ACC30  addi r3, r10, -0x33d0
	ctx.r[3].s64 = ctx.r[10].s64 + -13264;
	// 8268A92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A950: 4BDDC4D1  bl 0x82466e20
	ctx.lr = 0x8268A954;
	sub_82466E20(ctx, base);
	// 8268A954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A968 size=108
    let mut pc: u32 = 0x8268A968;
    'dispatch: loop {
        match pc {
            0x8268A968 => {
    //   block [0x8268A968..0x8268A9D4)
	// 8268A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A974: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268A97C: 38EB3558  addi r7, r11, 0x3558
	ctx.r[7].s64 = ctx.r[11].s64 + 13656;
	// 8268A980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268A984: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 8268A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A98C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268A994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A998: 386ACC60  addi r3, r10, -0x33a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13216;
	// 8268A99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A9C0: 4BDDC461  bl 0x82466e20
	ctx.lr = 0x8268A9C4;
	sub_82466E20(ctx, base);
	// 8268A9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A9D8 size=112
    let mut pc: u32 = 0x8268A9D8;
    'dispatch: loop {
        match pc {
            0x8268A9D8 => {
    //   block [0x8268A9D8..0x8268AA48)
	// 8268A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A9E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A9E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A9EC: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 8268A9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A9F4: 390B3588  addi r8, r11, 0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + 13704;
	// 8268A9F8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8268A9FC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8268AA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AA04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AA10: 386ACC90  addi r3, r10, -0x3370
	ctx.r[3].s64 = ctx.r[10].s64 + -13168;
	// 8268AA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AA34: 4BDDC3ED  bl 0x82466e20
	ctx.lr = 0x8268AA38;
	sub_82466E20(ctx, base);
	// 8268AA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AA48 size=112
    let mut pc: u32 = 0x8268AA48;
    'dispatch: loop {
        match pc {
            0x8268AA48 => {
    //   block [0x8268AA48..0x8268AAB8)
	// 8268AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AA54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AA58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AA5C: 38AABC40  addi r5, r10, -0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + -17344;
	// 8268AA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AA64: 390B3660  addi r8, r11, 0x3660
	ctx.r[8].s64 = ctx.r[11].s64 + 13920;
	// 8268AA68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268AA6C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8268AA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AA74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AA80: 386ACCC0  addi r3, r10, -0x3340
	ctx.r[3].s64 = ctx.r[10].s64 + -13120;
	// 8268AA84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AAA4: 4BDDC37D  bl 0x82466e20
	ctx.lr = 0x8268AAA8;
	sub_82466E20(ctx, base);
	// 8268AAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AAB8 size=112
    let mut pc: u32 = 0x8268AAB8;
    'dispatch: loop {
        match pc {
            0x8268AAB8 => {
    //   block [0x8268AAB8..0x8268AB28)
	// 8268AAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AAC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AAC8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AACC: 38AABC40  addi r5, r10, -0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + -17344;
	// 8268AAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AAD4: 390B36A8  addi r8, r11, 0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13992;
	// 8268AAD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268AADC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8268AAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AAE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AAF0: 386ACCF0  addi r3, r10, -0x3310
	ctx.r[3].s64 = ctx.r[10].s64 + -13072;
	// 8268AAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AB14: 4BDDC30D  bl 0x82466e20
	ctx.lr = 0x8268AB18;
	sub_82466E20(ctx, base);
	// 8268AB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AB28 size=112
    let mut pc: u32 = 0x8268AB28;
    'dispatch: loop {
        match pc {
            0x8268AB28 => {
    //   block [0x8268AB28..0x8268AB98)
	// 8268AB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AB34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AB38: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AB3C: 38AABC70  addi r5, r10, -0x4390
	ctx.r[5].s64 = ctx.r[10].s64 + -17296;
	// 8268AB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AB44: 390B3708  addi r8, r11, 0x3708
	ctx.r[8].s64 = ctx.r[11].s64 + 14088;
	// 8268AB48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268AB4C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8268AB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AB54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AB60: 386ACD20  addi r3, r10, -0x32e0
	ctx.r[3].s64 = ctx.r[10].s64 + -13024;
	// 8268AB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AB84: 4BDDC29D  bl 0x82466e20
	ctx.lr = 0x8268AB88;
	sub_82466E20(ctx, base);
	// 8268AB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AB98 size=112
    let mut pc: u32 = 0x8268AB98;
    'dispatch: loop {
        match pc {
            0x8268AB98 => {
    //   block [0x8268AB98..0x8268AC08)
	// 8268AB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268ABA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268ABA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ABA8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268ABAC: 38AABC70  addi r5, r10, -0x4390
	ctx.r[5].s64 = ctx.r[10].s64 + -17296;
	// 8268ABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268ABB4: 390B3768  addi r8, r11, 0x3768
	ctx.r[8].s64 = ctx.r[11].s64 + 14184;
	// 8268ABB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268ABBC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8268ABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268ABC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ABC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268ABD0: 386ACD50  addi r3, r10, -0x32b0
	ctx.r[3].s64 = ctx.r[10].s64 + -12976;
	// 8268ABD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268ABD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268ABDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ABE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268ABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ABEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268ABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268ABF4: 4BDDC22D  bl 0x82466e20
	ctx.lr = 0x8268ABF8;
	sub_82466E20(ctx, base);
	// 8268ABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268ABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AC08 size=112
    let mut pc: u32 = 0x8268AC08;
    'dispatch: loop {
        match pc {
            0x8268AC08 => {
    //   block [0x8268AC08..0x8268AC78)
	// 8268AC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AC14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AC18: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AC1C: 38AABC40  addi r5, r10, -0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + -17344;
	// 8268AC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AC24: 390B37C8  addi r8, r11, 0x37c8
	ctx.r[8].s64 = ctx.r[11].s64 + 14280;
	// 8268AC28: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8268AC2C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8268AC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AC34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AC40: 386ACD80  addi r3, r10, -0x3280
	ctx.r[3].s64 = ctx.r[10].s64 + -12928;
	// 8268AC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AC64: 4BDDC1BD  bl 0x82466e20
	ctx.lr = 0x8268AC68;
	sub_82466E20(ctx, base);
	// 8268AC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AC78 size=108
    let mut pc: u32 = 0x8268AC78;
    'dispatch: loop {
        match pc {
            0x8268AC78 => {
    //   block [0x8268AC78..0x8268ACE4)
	// 8268AC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AC84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AC8C: 38EB3888  addi r7, r11, 0x3888
	ctx.r[7].s64 = ctx.r[11].s64 + 14472;
	// 8268AC90: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8268AC94: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8268AC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AC9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ACA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268ACA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ACA8: 386ACDB0  addi r3, r10, -0x3250
	ctx.r[3].s64 = ctx.r[10].s64 + -12880;
	// 8268ACAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268ACB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268ACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268ACB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ACBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268ACC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ACC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268ACC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268ACCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268ACD0: 4BDDC151  bl 0x82466e20
	ctx.lr = 0x8268ACD4;
	sub_82466E20(ctx, base);
	// 8268ACD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268ACD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268ACDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268ACE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268ACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268ACE8 size=112
    let mut pc: u32 = 0x8268ACE8;
    'dispatch: loop {
        match pc {
            0x8268ACE8 => {
    //   block [0x8268ACE8..0x8268AD58)
	// 8268ACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268ACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268ACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268ACF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ACF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268ACFC: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 8268AD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AD04: 390B3A20  addi r8, r11, 0x3a20
	ctx.r[8].s64 = ctx.r[11].s64 + 14880;
	// 8268AD08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268AD0C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8268AD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AD14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AD20: 386ACDE0  addi r3, r10, -0x3220
	ctx.r[3].s64 = ctx.r[10].s64 + -12832;
	// 8268AD24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AD44: 4BDDC0DD  bl 0x82466e20
	ctx.lr = 0x8268AD48;
	sub_82466E20(ctx, base);
	// 8268AD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AD58 size=112
    let mut pc: u32 = 0x8268AD58;
    'dispatch: loop {
        match pc {
            0x8268AD58 => {
    //   block [0x8268AD58..0x8268ADC8)
	// 8268AD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AD64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AD68: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AD6C: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 8268AD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AD74: 390B3A38  addi r8, r11, 0x3a38
	ctx.r[8].s64 = ctx.r[11].s64 + 14904;
	// 8268AD78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268AD7C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8268AD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AD84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AD90: 386ACE10  addi r3, r10, -0x31f0
	ctx.r[3].s64 = ctx.r[10].s64 + -12784;
	// 8268AD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ADA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ADA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268ADA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ADAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268ADB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268ADB4: 4BDDC06D  bl 0x82466e20
	ctx.lr = 0x8268ADB8;
	sub_82466E20(ctx, base);
	// 8268ADB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268ADBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268ADC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268ADC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268ADC8 size=112
    let mut pc: u32 = 0x8268ADC8;
    'dispatch: loop {
        match pc {
            0x8268ADC8 => {
    //   block [0x8268ADC8..0x8268AE38)
	// 8268ADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268ADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268ADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268ADD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ADD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268ADDC: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 8268ADE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268ADE4: 390B3A50  addi r8, r11, 0x3a50
	ctx.r[8].s64 = ctx.r[11].s64 + 14928;
	// 8268ADE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268ADEC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8268ADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268ADF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ADF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268ADFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AE00: 386ACE40  addi r3, r10, -0x31c0
	ctx.r[3].s64 = ctx.r[10].s64 + -12736;
	// 8268AE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AE24: 4BDDBFFD  bl 0x82466e20
	ctx.lr = 0x8268AE28;
	sub_82466E20(ctx, base);
	// 8268AE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AE38 size=108
    let mut pc: u32 = 0x8268AE38;
    'dispatch: loop {
        match pc {
            0x8268AE38 => {
    //   block [0x8268AE38..0x8268AEA4)
	// 8268AE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AE44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AE4C: 38EB3A80  addi r7, r11, 0x3a80
	ctx.r[7].s64 = ctx.r[11].s64 + 14976;
	// 8268AE50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268AE54: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8268AE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268AE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AE68: 386ACE70  addi r3, r10, -0x3190
	ctx.r[3].s64 = ctx.r[10].s64 + -12688;
	// 8268AE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268AE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268AE90: 4BDDBF91  bl 0x82466e20
	ctx.lr = 0x8268AE94;
	sub_82466E20(ctx, base);
	// 8268AE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AEA8 size=112
    let mut pc: u32 = 0x8268AEA8;
    'dispatch: loop {
        match pc {
            0x8268AEA8 => {
    //   block [0x8268AEA8..0x8268AF18)
	// 8268AEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AEB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AEB8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AEBC: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 8268AEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AEC4: 390B3AB0  addi r8, r11, 0x3ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 15024;
	// 8268AEC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268AECC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8268AED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AED4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268AEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AEE0: 386ACEA0  addi r3, r10, -0x3160
	ctx.r[3].s64 = ctx.r[10].s64 + -12640;
	// 8268AEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268AEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AEF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268AEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AF04: 4BDDBF1D  bl 0x82466e20
	ctx.lr = 0x8268AF08;
	sub_82466E20(ctx, base);
	// 8268AF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AF18 size=108
    let mut pc: u32 = 0x8268AF18;
    'dispatch: loop {
        match pc {
            0x8268AF18 => {
    //   block [0x8268AF18..0x8268AF84)
	// 8268AF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AF24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AF2C: 38EB3AC8  addi r7, r11, 0x3ac8
	ctx.r[7].s64 = ctx.r[11].s64 + 15048;
	// 8268AF30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268AF34: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8268AF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AF3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268AF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AF48: 386ACED0  addi r3, r10, -0x3130
	ctx.r[3].s64 = ctx.r[10].s64 + -12592;
	// 8268AF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268AF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268AF70: 4BDDBEB1  bl 0x82466e20
	ctx.lr = 0x8268AF74;
	sub_82466E20(ctx, base);
	// 8268AF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AF88 size=108
    let mut pc: u32 = 0x8268AF88;
    'dispatch: loop {
        match pc {
            0x8268AF88 => {
    //   block [0x8268AF88..0x8268AFF4)
	// 8268AF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268AF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268AF94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268AF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268AF9C: 38EB3AF8  addi r7, r11, 0x3af8
	ctx.r[7].s64 = ctx.r[11].s64 + 15096;
	// 8268AFA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268AFA4: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8268AFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268AFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268AFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268AFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268AFB8: 386ACF00  addi r3, r10, -0x3100
	ctx.r[3].s64 = ctx.r[10].s64 + -12544;
	// 8268AFBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268AFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268AFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268AFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268AFDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268AFE0: 4BDDBE41  bl 0x82466e20
	ctx.lr = 0x8268AFE4;
	sub_82466E20(ctx, base);
	// 8268AFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268AFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268AFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268AFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268AFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268AFF8 size=112
    let mut pc: u32 = 0x8268AFF8;
    'dispatch: loop {
        match pc {
            0x8268AFF8 => {
    //   block [0x8268AFF8..0x8268B068)
	// 8268AFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268AFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B008: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B00C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B014: 390B3B40  addi r8, r11, 0x3b40
	ctx.r[8].s64 = ctx.r[11].s64 + 15168;
	// 8268B018: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268B01C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8268B020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B024: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B030: 386ACF30  addi r3, r10, -0x30d0
	ctx.r[3].s64 = ctx.r[10].s64 + -12496;
	// 8268B034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B054: 4BDDBDCD  bl 0x82466e20
	ctx.lr = 0x8268B058;
	sub_82466E20(ctx, base);
	// 8268B058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B068 size=112
    let mut pc: u32 = 0x8268B068;
    'dispatch: loop {
        match pc {
            0x8268B068 => {
    //   block [0x8268B068..0x8268B0D8)
	// 8268B068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B074: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B078: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B07C: 38AABC70  addi r5, r10, -0x4390
	ctx.r[5].s64 = ctx.r[10].s64 + -17296;
	// 8268B080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B084: 390B3B88  addi r8, r11, 0x3b88
	ctx.r[8].s64 = ctx.r[11].s64 + 15240;
	// 8268B088: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8268B08C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8268B090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B094: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B0A0: 386ACF60  addi r3, r10, -0x30a0
	ctx.r[3].s64 = ctx.r[10].s64 + -12448;
	// 8268B0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B0C4: 4BDDBD5D  bl 0x82466e20
	ctx.lr = 0x8268B0C8;
	sub_82466E20(ctx, base);
	// 8268B0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B0D8 size=108
    let mut pc: u32 = 0x8268B0D8;
    'dispatch: loop {
        match pc {
            0x8268B0D8 => {
    //   block [0x8268B0D8..0x8268B144)
	// 8268B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B0E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B0EC: 38EB3C18  addi r7, r11, 0x3c18
	ctx.r[7].s64 = ctx.r[11].s64 + 15384;
	// 8268B0F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268B0F4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8268B0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B0FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B108: 386ACF90  addi r3, r10, -0x3070
	ctx.r[3].s64 = ctx.r[10].s64 + -12400;
	// 8268B10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B130: 4BDDBCF1  bl 0x82466e20
	ctx.lr = 0x8268B134;
	sub_82466E20(ctx, base);
	// 8268B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B148 size=108
    let mut pc: u32 = 0x8268B148;
    'dispatch: loop {
        match pc {
            0x8268B148 => {
    //   block [0x8268B148..0x8268B1B4)
	// 8268B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B154: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B15C: 38EB3C60  addi r7, r11, 0x3c60
	ctx.r[7].s64 = ctx.r[11].s64 + 15456;
	// 8268B160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268B164: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8268B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B16C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B178: 386ACFC0  addi r3, r10, -0x3040
	ctx.r[3].s64 = ctx.r[10].s64 + -12352;
	// 8268B17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B1A0: 4BDDBC81  bl 0x82466e20
	ctx.lr = 0x8268B1A4;
	sub_82466E20(ctx, base);
	// 8268B1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B1B8 size=108
    let mut pc: u32 = 0x8268B1B8;
    'dispatch: loop {
        match pc {
            0x8268B1B8 => {
    //   block [0x8268B1B8..0x8268B224)
	// 8268B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B1C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B1CC: 38EB3C90  addi r7, r11, 0x3c90
	ctx.r[7].s64 = ctx.r[11].s64 + 15504;
	// 8268B1D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268B1D4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8268B1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B1DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B1E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B1E8: 386ACFF0  addi r3, r10, -0x3010
	ctx.r[3].s64 = ctx.r[10].s64 + -12304;
	// 8268B1EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B20C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B210: 4BDDBC11  bl 0x82466e20
	ctx.lr = 0x8268B214;
	sub_82466E20(ctx, base);
	// 8268B214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B228 size=112
    let mut pc: u32 = 0x8268B228;
    'dispatch: loop {
        match pc {
            0x8268B228 => {
    //   block [0x8268B228..0x8268B298)
	// 8268B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B234: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B238: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B23C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B244: 390B3CC0  addi r8, r11, 0x3cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 15552;
	// 8268B248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268B24C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8268B250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B254: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B260: 386AD020  addi r3, r10, -0x2fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -12256;
	// 8268B264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B284: 4BDDBB9D  bl 0x82466e20
	ctx.lr = 0x8268B288;
	sub_82466E20(ctx, base);
	// 8268B288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B298 size=112
    let mut pc: u32 = 0x8268B298;
    'dispatch: loop {
        match pc {
            0x8268B298 => {
    //   block [0x8268B298..0x8268B308)
	// 8268B298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B2A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B2A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B2AC: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B2B4: 390B3CF0  addi r8, r11, 0x3cf0
	ctx.r[8].s64 = ctx.r[11].s64 + 15600;
	// 8268B2B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268B2BC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8268B2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B2D0: 386AD050  addi r3, r10, -0x2fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -12208;
	// 8268B2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B2F4: 4BDDBB2D  bl 0x82466e20
	ctx.lr = 0x8268B2F8;
	sub_82466E20(ctx, base);
	// 8268B2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B308 size=112
    let mut pc: u32 = 0x8268B308;
    'dispatch: loop {
        match pc {
            0x8268B308 => {
    //   block [0x8268B308..0x8268B378)
	// 8268B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B314: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B318: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B31C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B324: 390B3D08  addi r8, r11, 0x3d08
	ctx.r[8].s64 = ctx.r[11].s64 + 15624;
	// 8268B328: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268B32C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8268B330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B334: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B340: 386AD080  addi r3, r10, -0x2f80
	ctx.r[3].s64 = ctx.r[10].s64 + -12160;
	// 8268B344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B364: 4BDDBABD  bl 0x82466e20
	ctx.lr = 0x8268B368;
	sub_82466E20(ctx, base);
	// 8268B368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B378 size=108
    let mut pc: u32 = 0x8268B378;
    'dispatch: loop {
        match pc {
            0x8268B378 => {
    //   block [0x8268B378..0x8268B3E4)
	// 8268B378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B384: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B38C: 38EB3D20  addi r7, r11, 0x3d20
	ctx.r[7].s64 = ctx.r[11].s64 + 15648;
	// 8268B390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268B394: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8268B398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B39C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B3A8: 386AD0B0  addi r3, r10, -0x2f50
	ctx.r[3].s64 = ctx.r[10].s64 + -12112;
	// 8268B3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B3D0: 4BDDBA51  bl 0x82466e20
	ctx.lr = 0x8268B3D4;
	sub_82466E20(ctx, base);
	// 8268B3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B3E8 size=112
    let mut pc: u32 = 0x8268B3E8;
    'dispatch: loop {
        match pc {
            0x8268B3E8 => {
    //   block [0x8268B3E8..0x8268B458)
	// 8268B3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B3F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B3F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B3FC: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B404: 390B3D50  addi r8, r11, 0x3d50
	ctx.r[8].s64 = ctx.r[11].s64 + 15696;
	// 8268B408: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268B40C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8268B410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B420: 386AD0E0  addi r3, r10, -0x2f20
	ctx.r[3].s64 = ctx.r[10].s64 + -12064;
	// 8268B424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B444: 4BDDB9DD  bl 0x82466e20
	ctx.lr = 0x8268B448;
	sub_82466E20(ctx, base);
	// 8268B448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B458 size=108
    let mut pc: u32 = 0x8268B458;
    'dispatch: loop {
        match pc {
            0x8268B458 => {
    //   block [0x8268B458..0x8268B4C4)
	// 8268B458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B464: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B46C: 38EB3D68  addi r7, r11, 0x3d68
	ctx.r[7].s64 = ctx.r[11].s64 + 15720;
	// 8268B470: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8268B474: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8268B478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B47C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B488: 386AD110  addi r3, r10, -0x2ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -12016;
	// 8268B48C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B4B0: 4BDDB971  bl 0x82466e20
	ctx.lr = 0x8268B4B4;
	sub_82466E20(ctx, base);
	// 8268B4B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B4C8 size=112
    let mut pc: u32 = 0x8268B4C8;
    'dispatch: loop {
        match pc {
            0x8268B4C8 => {
    //   block [0x8268B4C8..0x8268B538)
	// 8268B4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B4D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B4D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B4DC: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B4E4: 390B3E40  addi r8, r11, 0x3e40
	ctx.r[8].s64 = ctx.r[11].s64 + 15936;
	// 8268B4E8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8268B4EC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8268B4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B4F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B500: 386AD140  addi r3, r10, -0x2ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -11968;
	// 8268B504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B524: 4BDDB8FD  bl 0x82466e20
	ctx.lr = 0x8268B528;
	sub_82466E20(ctx, base);
	// 8268B528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B538 size=108
    let mut pc: u32 = 0x8268B538;
    'dispatch: loop {
        match pc {
            0x8268B538 => {
    //   block [0x8268B538..0x8268B5A4)
	// 8268B538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B544: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B54C: 38EB3FF0  addi r7, r11, 0x3ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 16368;
	// 8268B550: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8268B554: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8268B558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B55C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B568: 386AD170  addi r3, r10, -0x2e90
	ctx.r[3].s64 = ctx.r[10].s64 + -11920;
	// 8268B56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B590: 4BDDB891  bl 0x82466e20
	ctx.lr = 0x8268B594;
	sub_82466E20(ctx, base);
	// 8268B594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B5A8 size=112
    let mut pc: u32 = 0x8268B5A8;
    'dispatch: loop {
        match pc {
            0x8268B5A8 => {
    //   block [0x8268B5A8..0x8268B618)
	// 8268B5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B5B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B5B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B5BC: 38AABC70  addi r5, r10, -0x4390
	ctx.r[5].s64 = ctx.r[10].s64 + -17296;
	// 8268B5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B5C4: 390B4188  addi r8, r11, 0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + 16776;
	// 8268B5C8: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8268B5CC: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8268B5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B5D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B5E0: 386AD1A0  addi r3, r10, -0x2e60
	ctx.r[3].s64 = ctx.r[10].s64 + -11872;
	// 8268B5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B604: 4BDDB81D  bl 0x82466e20
	ctx.lr = 0x8268B608;
	sub_82466E20(ctx, base);
	// 8268B608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B618 size=100
    let mut pc: u32 = 0x8268B618;
    'dispatch: loop {
        match pc {
            0x8268B618 => {
    //   block [0x8268B618..0x8268B67C)
	// 8268B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B624: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B62C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B638: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8268B63C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B64C: 386AD1D0  addi r3, r10, -0x2e30
	ctx.r[3].s64 = ctx.r[10].s64 + -11824;
	// 8268B650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268B65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268B664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B668: 4BDDB7B9  bl 0x82466e20
	ctx.lr = 0x8268B66C;
	sub_82466E20(ctx, base);
	// 8268B66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B680 size=112
    let mut pc: u32 = 0x8268B680;
    'dispatch: loop {
        match pc {
            0x8268B680 => {
    //   block [0x8268B680..0x8268B6F0)
	// 8268B680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B68C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B690: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B694: 38AAD1D0  addi r5, r10, -0x2e30
	ctx.r[5].s64 = ctx.r[10].s64 + -11824;
	// 8268B698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B69C: 390B43F8  addi r8, r11, 0x43f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17400;
	// 8268B6A0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8268B6A4: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8268B6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B6B8: 386AD200  addi r3, r10, -0x2e00
	ctx.r[3].s64 = ctx.r[10].s64 + -11776;
	// 8268B6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B6DC: 4BDDB745  bl 0x82466e20
	ctx.lr = 0x8268B6E0;
	sub_82466E20(ctx, base);
	// 8268B6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B6F0 size=100
    let mut pc: u32 = 0x8268B6F0;
    'dispatch: loop {
        match pc {
            0x8268B6F0 => {
    //   block [0x8268B6F0..0x8268B754)
	// 8268B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B704: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B710: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8268B714: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B724: 386AD230  addi r3, r10, -0x2dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -11728;
	// 8268B728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B72C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268B734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268B73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B740: 4BDDB6E1  bl 0x82466e20
	ctx.lr = 0x8268B744;
	sub_82466E20(ctx, base);
	// 8268B744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B758 size=108
    let mut pc: u32 = 0x8268B758;
    'dispatch: loop {
        match pc {
            0x8268B758 => {
    //   block [0x8268B758..0x8268B7C4)
	// 8268B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B764: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B76C: 38EB4470  addi r7, r11, 0x4470
	ctx.r[7].s64 = ctx.r[11].s64 + 17520;
	// 8268B770: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268B774: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8268B778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B77C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268B784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B788: 386AD260  addi r3, r10, -0x2da0
	ctx.r[3].s64 = ctx.r[10].s64 + -11680;
	// 8268B78C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268B790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B7AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268B7B0: 4BDDB671  bl 0x82466e20
	ctx.lr = 0x8268B7B4;
	sub_82466E20(ctx, base);
	// 8268B7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B7C8 size=112
    let mut pc: u32 = 0x8268B7C8;
    'dispatch: loop {
        match pc {
            0x8268B7C8 => {
    //   block [0x8268B7C8..0x8268B838)
	// 8268B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B7D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B7D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B7DC: 38AAD230  addi r5, r10, -0x2dd0
	ctx.r[5].s64 = ctx.r[10].s64 + -11728;
	// 8268B7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B7E4: 390B44B8  addi r8, r11, 0x44b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17592;
	// 8268B7E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268B7EC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8268B7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B7F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B800: 386AD290  addi r3, r10, -0x2d70
	ctx.r[3].s64 = ctx.r[10].s64 + -11632;
	// 8268B804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B824: 4BDDB5FD  bl 0x82466e20
	ctx.lr = 0x8268B828;
	sub_82466E20(ctx, base);
	// 8268B828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B838 size=100
    let mut pc: u32 = 0x8268B838;
    'dispatch: loop {
        match pc {
            0x8268B838 => {
    //   block [0x8268B838..0x8268B89C)
	// 8268B838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B844: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B84C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B858: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8268B85C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B86C: 386AD2C0  addi r3, r10, -0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	// 8268B870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268B87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268B884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B888: 4BDDB599  bl 0x82466e20
	ctx.lr = 0x8268B88C;
	sub_82466E20(ctx, base);
	// 8268B88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B8A0 size=100
    let mut pc: u32 = 0x8268B8A0;
    'dispatch: loop {
        match pc {
            0x8268B8A0 => {
    //   block [0x8268B8A0..0x8268B904)
	// 8268B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B8AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B8B4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268B8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B8C0: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8268B8C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B8D4: 386AD2F0  addi r3, r10, -0x2d10
	ctx.r[3].s64 = ctx.r[10].s64 + -11536;
	// 8268B8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B8DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B8E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268B8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B8E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268B8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B8F0: 4BDDB531  bl 0x82466e20
	ctx.lr = 0x8268B8F4;
	sub_82466E20(ctx, base);
	// 8268B8F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B908 size=112
    let mut pc: u32 = 0x8268B908;
    'dispatch: loop {
        match pc {
            0x8268B908 => {
    //   block [0x8268B908..0x8268B978)
	// 8268B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B914: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B918: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B91C: 38AAD2C0  addi r5, r10, -0x2d40
	ctx.r[5].s64 = ctx.r[10].s64 + -11584;
	// 8268B920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B924: 390B44E8  addi r8, r11, 0x44e8
	ctx.r[8].s64 = ctx.r[11].s64 + 17640;
	// 8268B928: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268B92C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8268B930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B940: 386AD320  addi r3, r10, -0x2ce0
	ctx.r[3].s64 = ctx.r[10].s64 + -11488;
	// 8268B944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B964: 4BDDB4BD  bl 0x82466e20
	ctx.lr = 0x8268B968;
	sub_82466E20(ctx, base);
	// 8268B968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B978 size=112
    let mut pc: u32 = 0x8268B978;
    'dispatch: loop {
        match pc {
            0x8268B978 => {
    //   block [0x8268B978..0x8268B9E8)
	// 8268B978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B984: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B988: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268B98C: 38AAD2F0  addi r5, r10, -0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + -11536;
	// 8268B990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268B994: 390B4548  addi r8, r11, 0x4548
	ctx.r[8].s64 = ctx.r[11].s64 + 17736;
	// 8268B998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268B99C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8268B9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B9A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B9A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268B9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268B9B0: 386AD350  addi r3, r10, -0x2cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11440;
	// 8268B9B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268B9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268B9BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268B9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268B9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268B9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268B9CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268B9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268B9D4: 4BDDB44D  bl 0x82466e20
	ctx.lr = 0x8268B9D8;
	sub_82466E20(ctx, base);
	// 8268B9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268B9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268B9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268B9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268B9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268B9E8 size=100
    let mut pc: u32 = 0x8268B9E8;
    'dispatch: loop {
        match pc {
            0x8268B9E8 => {
    //   block [0x8268B9E8..0x8268BA4C)
	// 8268B9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268B9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268B9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268B9F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268B9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268B9FC: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268BA00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BA08: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8268BA0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BA1C: 386AD380  addi r3, r10, -0x2c80
	ctx.r[3].s64 = ctx.r[10].s64 + -11392;
	// 8268BA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BA24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BA28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268BA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BA30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268BA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BA38: 4BDDB3E9  bl 0x82466e20
	ctx.lr = 0x8268BA3C;
	sub_82466E20(ctx, base);
	// 8268BA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BA50 size=112
    let mut pc: u32 = 0x8268BA50;
    'dispatch: loop {
        match pc {
            0x8268BA50 => {
    //   block [0x8268BA50..0x8268BAC0)
	// 8268BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BA60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BA64: 38AAD380  addi r5, r10, -0x2c80
	ctx.r[5].s64 = ctx.r[10].s64 + -11392;
	// 8268BA68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BA6C: 390B45A8  addi r8, r11, 0x45a8
	ctx.r[8].s64 = ctx.r[11].s64 + 17832;
	// 8268BA70: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8268BA74: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8268BA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BA7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BA80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268BA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BA88: 386AD3B0  addi r3, r10, -0x2c50
	ctx.r[3].s64 = ctx.r[10].s64 + -11344;
	// 8268BA8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268BA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BAAC: 4BDDB375  bl 0x82466e20
	ctx.lr = 0x8268BAB0;
	sub_82466E20(ctx, base);
	// 8268BAB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BAC0 size=108
    let mut pc: u32 = 0x8268BAC0;
    'dispatch: loop {
        match pc {
            0x8268BAC0 => {
    //   block [0x8268BAC0..0x8268BB2C)
	// 8268BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BACC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BAD4: 38EB4698  addi r7, r11, 0x4698
	ctx.r[7].s64 = ctx.r[11].s64 + 18072;
	// 8268BAD8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8268BADC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8268BAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BAE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BAE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BAF0: 386AD3E0  addi r3, r10, -0x2c20
	ctx.r[3].s64 = ctx.r[10].s64 + -11296;
	// 8268BAF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BB18: 4BDDB309  bl 0x82466e20
	ctx.lr = 0x8268BB1C;
	sub_82466E20(ctx, base);
	// 8268BB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BB30 size=108
    let mut pc: u32 = 0x8268BB30;
    'dispatch: loop {
        match pc {
            0x8268BB30 => {
    //   block [0x8268BB30..0x8268BB9C)
	// 8268BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BB3C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BB44: 38EB4788  addi r7, r11, 0x4788
	ctx.r[7].s64 = ctx.r[11].s64 + 18312;
	// 8268BB48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268BB4C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8268BB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BB54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BB58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BB60: 386AD410  addi r3, r10, -0x2bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -11248;
	// 8268BB64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BB88: 4BDDB299  bl 0x82466e20
	ctx.lr = 0x8268BB8C;
	sub_82466E20(ctx, base);
	// 8268BB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BB90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BB94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BBA0 size=108
    let mut pc: u32 = 0x8268BBA0;
    'dispatch: loop {
        match pc {
            0x8268BBA0 => {
    //   block [0x8268BBA0..0x8268BC0C)
	// 8268BBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BBAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BBB4: 38EB47D0  addi r7, r11, 0x47d0
	ctx.r[7].s64 = ctx.r[11].s64 + 18384;
	// 8268BBB8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8268BBBC: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8268BBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BBC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BBC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BBD0: 386AD440  addi r3, r10, -0x2bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11200;
	// 8268BBD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BBF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BBF8: 4BDDB229  bl 0x82466e20
	ctx.lr = 0x8268BBFC;
	sub_82466E20(ctx, base);
	// 8268BBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BC10 size=108
    let mut pc: u32 = 0x8268BC10;
    'dispatch: loop {
        match pc {
            0x8268BC10 => {
    //   block [0x8268BC10..0x8268BC7C)
	// 8268BC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BC1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BC24: 38EB48A8  addi r7, r11, 0x48a8
	ctx.r[7].s64 = ctx.r[11].s64 + 18600;
	// 8268BC28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268BC2C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8268BC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BC34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BC40: 386AD470  addi r3, r10, -0x2b90
	ctx.r[3].s64 = ctx.r[10].s64 + -11152;
	// 8268BC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BC68: 4BDDB1B9  bl 0x82466e20
	ctx.lr = 0x8268BC6C;
	sub_82466E20(ctx, base);
	// 8268BC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BC80 size=100
    let mut pc: u32 = 0x8268BC80;
    'dispatch: loop {
        match pc {
            0x8268BC80 => {
    //   block [0x8268BC80..0x8268BCE4)
	// 8268BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BC8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BC94: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268BC98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BCA0: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8268BCA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BCB4: 386AD4A0  addi r3, r10, -0x2b60
	ctx.r[3].s64 = ctx.r[10].s64 + -11104;
	// 8268BCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BCBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BCC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268BCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BCC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268BCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BCD0: 4BDDB151  bl 0x82466e20
	ctx.lr = 0x8268BCD4;
	sub_82466E20(ctx, base);
	// 8268BCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BCE8 size=112
    let mut pc: u32 = 0x8268BCE8;
    'dispatch: loop {
        match pc {
            0x8268BCE8 => {
    //   block [0x8268BCE8..0x8268BD58)
	// 8268BCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BCF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BCF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BCF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BCFC: 38AAD4A0  addi r5, r10, -0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + -11104;
	// 8268BD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BD04: 390B48C0  addi r8, r11, 0x48c0
	ctx.r[8].s64 = ctx.r[11].s64 + 18624;
	// 8268BD08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268BD0C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8268BD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BD14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268BD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BD20: 386AD4D0  addi r3, r10, -0x2b30
	ctx.r[3].s64 = ctx.r[10].s64 + -11056;
	// 8268BD24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268BD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BD44: 4BDDB0DD  bl 0x82466e20
	ctx.lr = 0x8268BD48;
	sub_82466E20(ctx, base);
	// 8268BD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BD58 size=108
    let mut pc: u32 = 0x8268BD58;
    'dispatch: loop {
        match pc {
            0x8268BD58 => {
    //   block [0x8268BD58..0x8268BDC4)
	// 8268BD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BD64: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BD6C: 38EB4908  addi r7, r11, 0x4908
	ctx.r[7].s64 = ctx.r[11].s64 + 18696;
	// 8268BD70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268BD74: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8268BD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BD80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BD88: 386AD500  addi r3, r10, -0x2b00
	ctx.r[3].s64 = ctx.r[10].s64 + -11008;
	// 8268BD8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BDAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BDB0: 4BDDB071  bl 0x82466e20
	ctx.lr = 0x8268BDB4;
	sub_82466E20(ctx, base);
	// 8268BDB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BDB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BDBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BDC8 size=112
    let mut pc: u32 = 0x8268BDC8;
    'dispatch: loop {
        match pc {
            0x8268BDC8 => {
    //   block [0x8268BDC8..0x8268BE38)
	// 8268BDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BDD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BDD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BDDC: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268BDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BDE4: 390B4950  addi r8, r11, 0x4950
	ctx.r[8].s64 = ctx.r[11].s64 + 18768;
	// 8268BDE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268BDEC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8268BDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BDF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BDF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268BDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BE00: 386AD530  addi r3, r10, -0x2ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -10960;
	// 8268BE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268BE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BE24: 4BDDAFFD  bl 0x82466e20
	ctx.lr = 0x8268BE28;
	sub_82466E20(ctx, base);
	// 8268BE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BE38 size=108
    let mut pc: u32 = 0x8268BE38;
    'dispatch: loop {
        match pc {
            0x8268BE38 => {
    //   block [0x8268BE38..0x8268BEA4)
	// 8268BE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BE44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BE4C: 38EB4968  addi r7, r11, 0x4968
	ctx.r[7].s64 = ctx.r[11].s64 + 18792;
	// 8268BE50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268BE54: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8268BE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268BE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BE68: 386AD560  addi r3, r10, -0x2aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -10912;
	// 8268BE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268BE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268BE90: 4BDDAF91  bl 0x82466e20
	ctx.lr = 0x8268BE94;
	sub_82466E20(ctx, base);
	// 8268BE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BEA8 size=112
    let mut pc: u32 = 0x8268BEA8;
    'dispatch: loop {
        match pc {
            0x8268BEA8 => {
    //   block [0x8268BEA8..0x8268BF18)
	// 8268BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BEB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BEB8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BEBC: 38AAD530  addi r5, r10, -0x2ad0
	ctx.r[5].s64 = ctx.r[10].s64 + -10960;
	// 8268BEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BEC4: 390B49B0  addi r8, r11, 0x49b0
	ctx.r[8].s64 = ctx.r[11].s64 + 18864;
	// 8268BEC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268BECC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8268BED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BED4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268BEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BEE0: 386AD590  addi r3, r10, -0x2a70
	ctx.r[3].s64 = ctx.r[10].s64 + -10864;
	// 8268BEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268BEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BF04: 4BDDAF1D  bl 0x82466e20
	ctx.lr = 0x8268BF08;
	sub_82466E20(ctx, base);
	// 8268BF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BF18 size=100
    let mut pc: u32 = 0x8268BF18;
    'dispatch: loop {
        match pc {
            0x8268BF18 => {
    //   block [0x8268BF18..0x8268BF7C)
	// 8268BF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BF24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BF2C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268BF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BF38: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8268BF3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BF4C: 386AD5C0  addi r3, r10, -0x2a40
	ctx.r[3].s64 = ctx.r[10].s64 + -10816;
	// 8268BF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BF54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BF58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268BF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BF60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268BF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BF68: 4BDDAEB9  bl 0x82466e20
	ctx.lr = 0x8268BF6C;
	sub_82466E20(ctx, base);
	// 8268BF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BF80 size=112
    let mut pc: u32 = 0x8268BF80;
    'dispatch: loop {
        match pc {
            0x8268BF80 => {
    //   block [0x8268BF80..0x8268BFF0)
	// 8268BF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BF8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BF90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268BF94: 38AAD5C0  addi r5, r10, -0x2a40
	ctx.r[5].s64 = ctx.r[10].s64 + -10816;
	// 8268BF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268BF9C: 390B49C8  addi r8, r11, 0x49c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18888;
	// 8268BFA0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268BFA4: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8268BFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268BFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268BFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268BFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268BFB8: 386AD5F0  addi r3, r10, -0x2a10
	ctx.r[3].s64 = ctx.r[10].s64 + -10768;
	// 8268BFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268BFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268BFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268BFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268BFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268BFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268BFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268BFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268BFDC: 4BDDAE45  bl 0x82466e20
	ctx.lr = 0x8268BFE0;
	sub_82466E20(ctx, base);
	// 8268BFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268BFF0 size=108
    let mut pc: u32 = 0x8268BFF0;
    'dispatch: loop {
        match pc {
            0x8268BFF0 => {
    //   block [0x8268BFF0..0x8268C05C)
	// 8268BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268BFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268BFFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C004: 38EB4A70  addi r7, r11, 0x4a70
	ctx.r[7].s64 = ctx.r[11].s64 + 19056;
	// 8268C008: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268C00C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8268C010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C020: 386AD620  addi r3, r10, -0x29e0
	ctx.r[3].s64 = ctx.r[10].s64 + -10720;
	// 8268C024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C048: 4BDDADD9  bl 0x82466e20
	ctx.lr = 0x8268C04C;
	sub_82466E20(ctx, base);
	// 8268C04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C060 size=112
    let mut pc: u32 = 0x8268C060;
    'dispatch: loop {
        match pc {
            0x8268C060 => {
    //   block [0x8268C060..0x8268C0D0)
	// 8268C060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C06C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C070: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C074: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268C078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C07C: 390B4AA0  addi r8, r11, 0x4aa0
	ctx.r[8].s64 = ctx.r[11].s64 + 19104;
	// 8268C080: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268C084: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8268C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C08C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C098: 386AD650  addi r3, r10, -0x29b0
	ctx.r[3].s64 = ctx.r[10].s64 + -10672;
	// 8268C09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C0BC: 4BDDAD65  bl 0x82466e20
	ctx.lr = 0x8268C0C0;
	sub_82466E20(ctx, base);
	// 8268C0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C0D0 size=112
    let mut pc: u32 = 0x8268C0D0;
    'dispatch: loop {
        match pc {
            0x8268C0D0 => {
    //   block [0x8268C0D0..0x8268C140)
	// 8268C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C0DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C0E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C0E4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C0EC: 390B4AE8  addi r8, r11, 0x4ae8
	ctx.r[8].s64 = ctx.r[11].s64 + 19176;
	// 8268C0F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268C0F4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8268C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C0FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C108: 386AD680  addi r3, r10, -0x2980
	ctx.r[3].s64 = ctx.r[10].s64 + -10624;
	// 8268C10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C12C: 4BDDACF5  bl 0x82466e20
	ctx.lr = 0x8268C130;
	sub_82466E20(ctx, base);
	// 8268C130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C140 size=100
    let mut pc: u32 = 0x8268C140;
    'dispatch: loop {
        match pc {
            0x8268C140 => {
    //   block [0x8268C140..0x8268C1A4)
	// 8268C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C14C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C154: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C160: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8268C164: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C174: 386AD6B0  addi r3, r10, -0x2950
	ctx.r[3].s64 = ctx.r[10].s64 + -10576;
	// 8268C178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268C184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268C18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C190: 4BDDAC91  bl 0x82466e20
	ctx.lr = 0x8268C194;
	sub_82466E20(ctx, base);
	// 8268C194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C1A8 size=112
    let mut pc: u32 = 0x8268C1A8;
    'dispatch: loop {
        match pc {
            0x8268C1A8 => {
    //   block [0x8268C1A8..0x8268C218)
	// 8268C1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C1B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C1BC: 38AAD6B0  addi r5, r10, -0x2950
	ctx.r[5].s64 = ctx.r[10].s64 + -10576;
	// 8268C1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C1C4: 390B4B30  addi r8, r11, 0x4b30
	ctx.r[8].s64 = ctx.r[11].s64 + 19248;
	// 8268C1C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268C1CC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8268C1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C1D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C1E0: 386AD6E0  addi r3, r10, -0x2920
	ctx.r[3].s64 = ctx.r[10].s64 + -10528;
	// 8268C1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C204: 4BDDAC1D  bl 0x82466e20
	ctx.lr = 0x8268C208;
	sub_82466E20(ctx, base);
	// 8268C208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C218 size=112
    let mut pc: u32 = 0x8268C218;
    'dispatch: loop {
        match pc {
            0x8268C218 => {
    //   block [0x8268C218..0x8268C288)
	// 8268C218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C228: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C22C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268C230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C234: 390B4B78  addi r8, r11, 0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + 19320;
	// 8268C238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268C23C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8268C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C250: 386AD710  addi r3, r10, -0x28f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	// 8268C254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C274: 4BDDABAD  bl 0x82466e20
	ctx.lr = 0x8268C278;
	sub_82466E20(ctx, base);
	// 8268C278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C288 size=112
    let mut pc: u32 = 0x8268C288;
    'dispatch: loop {
        match pc {
            0x8268C288 => {
    //   block [0x8268C288..0x8268C2F8)
	// 8268C288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C298: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C29C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268C2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C2A4: 390B4B90  addi r8, r11, 0x4b90
	ctx.r[8].s64 = ctx.r[11].s64 + 19344;
	// 8268C2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268C2AC: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8268C2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C2B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C2C0: 386AD740  addi r3, r10, -0x28c0
	ctx.r[3].s64 = ctx.r[10].s64 + -10432;
	// 8268C2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C2D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268C2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C2E4: 4BDDAB3D  bl 0x82466e20
	ctx.lr = 0x8268C2E8;
	sub_82466E20(ctx, base);
	// 8268C2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C2F8 size=112
    let mut pc: u32 = 0x8268C2F8;
    'dispatch: loop {
        match pc {
            0x8268C2F8 => {
    //   block [0x8268C2F8..0x8268C368)
	// 8268C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C308: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C30C: 38AAD710  addi r5, r10, -0x28f0
	ctx.r[5].s64 = ctx.r[10].s64 + -10480;
	// 8268C310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C314: 390B4BA8  addi r8, r11, 0x4ba8
	ctx.r[8].s64 = ctx.r[11].s64 + 19368;
	// 8268C318: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268C31C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8268C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C330: 386AD770  addi r3, r10, -0x2890
	ctx.r[3].s64 = ctx.r[10].s64 + -10384;
	// 8268C334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C354: 4BDDAACD  bl 0x82466e20
	ctx.lr = 0x8268C358;
	sub_82466E20(ctx, base);
	// 8268C358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C368 size=72
    let mut pc: u32 = 0x8268C368;
    'dispatch: loop {
        match pc {
            0x8268C368 => {
    //   block [0x8268C368..0x8268C3B0)
	// 8268C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C374: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8268C378: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8268C37C: 38CB5A48  addi r6, r11, 0x5a48
	ctx.r[6].s64 = ctx.r[11].s64 + 23112;
	// 8268C380: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8268C384: 388B6400  addi r4, r11, 0x6400
	ctx.r[4].s64 = ctx.r[11].s64 + 25600;
	// 8268C388: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268C38C: 386BD7A0  addi r3, r11, -0x2860
	ctx.r[3].s64 = ctx.r[11].s64 + -10336;
	// 8268C390: 4BDEF6F9  bl 0x8247ba88
	ctx.lr = 0x8268C394;
	sub_8247BA88(ctx, base);
	// 8268C394: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8268C398: 386BCE58  addi r3, r11, -0x31a8
	ctx.r[3].s64 = ctx.r[11].s64 + -12712;
	// 8268C39C: 4BEA679D  bl 0x82532b38
	ctx.lr = 0x8268C3A0;
	sub_82532B38(ctx, base);
	// 8268C3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8268C3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C3B0 size=108
    let mut pc: u32 = 0x8268C3B0;
    'dispatch: loop {
        match pc {
            0x8268C3B0 => {
    //   block [0x8268C3B0..0x8268C41C)
	// 8268C3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C3BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C3C4: 38EB5478  addi r7, r11, 0x5478
	ctx.r[7].s64 = ctx.r[11].s64 + 21624;
	// 8268C3C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8268C3CC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8268C3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C3D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C3E0: 386AD7B8  addi r3, r10, -0x2848
	ctx.r[3].s64 = ctx.r[10].s64 + -10312;
	// 8268C3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C408: 4BDDAA19  bl 0x82466e20
	ctx.lr = 0x8268C40C;
	sub_82466E20(ctx, base);
	// 8268C40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268C420 size=24
    let mut pc: u32 = 0x8268C420;
    'dispatch: loop {
        match pc {
            0x8268C420 => {
    //   block [0x8268C420..0x8268C438)
	// 8268C420: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C424: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268C428: 394AA320  addi r10, r10, -0x5ce0
	ctx.r[10].s64 = ctx.r[10].s64 + -23776;
	// 8268C42C: 816B54F0  lwz r11, 0x54f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21744 as u32) ) } as u64;
	// 8268C430: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8268C434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C438 size=112
    let mut pc: u32 = 0x8268C438;
    'dispatch: loop {
        match pc {
            0x8268C438 => {
    //   block [0x8268C438..0x8268C4A8)
	// 8268C438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C444: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268C448: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268C44C: 392A6874  addi r9, r10, 0x6874
	ctx.r[9].s64 = ctx.r[10].s64 + 26740;
	// 8268C450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C454: 390BA320  addi r8, r11, -0x5ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -23776;
	// 8268C458: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8268C45C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8268C460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C464: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C470: 386AD7E8  addi r3, r10, -0x2818
	ctx.r[3].s64 = ctx.r[10].s64 + -10264;
	// 8268C474: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268C478: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268C47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C48C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C494: 4BDDA98D  bl 0x82466e20
	ctx.lr = 0x8268C498;
	sub_82466E20(ctx, base);
	// 8268C498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C4A8 size=108
    let mut pc: u32 = 0x8268C4A8;
    'dispatch: loop {
        match pc {
            0x8268C4A8 => {
    //   block [0x8268C4A8..0x8268C514)
	// 8268C4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C4B4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C4B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C4BC: 38EB54F4  addi r7, r11, 0x54f4
	ctx.r[7].s64 = ctx.r[11].s64 + 21748;
	// 8268C4C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268C4C4: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8268C4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C4CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C4D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C4D8: 386AD818  addi r3, r10, -0x27e8
	ctx.r[3].s64 = ctx.r[10].s64 + -10216;
	// 8268C4DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C500: 4BDDA921  bl 0x82466e20
	ctx.lr = 0x8268C504;
	sub_82466E20(ctx, base);
	// 8268C504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C518 size=108
    let mut pc: u32 = 0x8268C518;
    'dispatch: loop {
        match pc {
            0x8268C518 => {
    //   block [0x8268C518..0x8268C584)
	// 8268C518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C524: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C52C: 38EB5524  addi r7, r11, 0x5524
	ctx.r[7].s64 = ctx.r[11].s64 + 21796;
	// 8268C530: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268C534: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8268C538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C53C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C548: 386AD848  addi r3, r10, -0x27b8
	ctx.r[3].s64 = ctx.r[10].s64 + -10168;
	// 8268C54C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C570: 4BDDA8B1  bl 0x82466e20
	ctx.lr = 0x8268C574;
	sub_82466E20(ctx, base);
	// 8268C574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C588 size=112
    let mut pc: u32 = 0x8268C588;
    'dispatch: loop {
        match pc {
            0x8268C588 => {
    //   block [0x8268C588..0x8268C5F8)
	// 8268C588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C598: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C59C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268C5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C5A4: 390B5558  addi r8, r11, 0x5558
	ctx.r[8].s64 = ctx.r[11].s64 + 21848;
	// 8268C5A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268C5AC: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8268C5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C5B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C5C0: 386AD878  addi r3, r10, -0x2788
	ctx.r[3].s64 = ctx.r[10].s64 + -10120;
	// 8268C5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C5E4: 4BDDA83D  bl 0x82466e20
	ctx.lr = 0x8268C5E8;
	sub_82466E20(ctx, base);
	// 8268C5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C5F8 size=108
    let mut pc: u32 = 0x8268C5F8;
    'dispatch: loop {
        match pc {
            0x8268C5F8 => {
    //   block [0x8268C5F8..0x8268C664)
	// 8268C5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C604: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C60C: 38EB55B8  addi r7, r11, 0x55b8
	ctx.r[7].s64 = ctx.r[11].s64 + 21944;
	// 8268C610: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268C614: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8268C618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C61C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C628: 386AD8A8  addi r3, r10, -0x2758
	ctx.r[3].s64 = ctx.r[10].s64 + -10072;
	// 8268C62C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C650: 4BDDA7D1  bl 0x82466e20
	ctx.lr = 0x8268C654;
	sub_82466E20(ctx, base);
	// 8268C654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C668 size=112
    let mut pc: u32 = 0x8268C668;
    'dispatch: loop {
        match pc {
            0x8268C668 => {
    //   block [0x8268C668..0x8268C6D8)
	// 8268C668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C678: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C67C: 38AAD878  addi r5, r10, -0x2788
	ctx.r[5].s64 = ctx.r[10].s64 + -10120;
	// 8268C680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C684: 390B5618  addi r8, r11, 0x5618
	ctx.r[8].s64 = ctx.r[11].s64 + 22040;
	// 8268C688: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8268C68C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8268C690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C694: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C6A0: 386AD8D8  addi r3, r10, -0x2728
	ctx.r[3].s64 = ctx.r[10].s64 + -10024;
	// 8268C6A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C6A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C6B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C6B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C6BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C6C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C6C4: 4BDDA75D  bl 0x82466e20
	ctx.lr = 0x8268C6C8;
	sub_82466E20(ctx, base);
	// 8268C6C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C6D8 size=112
    let mut pc: u32 = 0x8268C6D8;
    'dispatch: loop {
        match pc {
            0x8268C6D8 => {
    //   block [0x8268C6D8..0x8268C748)
	// 8268C6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C6E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C6E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C6EC: 38AAD878  addi r5, r10, -0x2788
	ctx.r[5].s64 = ctx.r[10].s64 + -10120;
	// 8268C6F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C6F4: 390B56A8  addi r8, r11, 0x56a8
	ctx.r[8].s64 = ctx.r[11].s64 + 22184;
	// 8268C6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268C6FC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8268C700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C704: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C710: 386AD908  addi r3, r10, -0x26f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9976;
	// 8268C714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C72C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C734: 4BDDA6ED  bl 0x82466e20
	ctx.lr = 0x8268C738;
	sub_82466E20(ctx, base);
	// 8268C738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C748 size=108
    let mut pc: u32 = 0x8268C748;
    'dispatch: loop {
        match pc {
            0x8268C748 => {
    //   block [0x8268C748..0x8268C7B4)
	// 8268C748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C754: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C75C: 38EB56C0  addi r7, r11, 0x56c0
	ctx.r[7].s64 = ctx.r[11].s64 + 22208;
	// 8268C760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268C764: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8268C768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C76C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C778: 386AD938  addi r3, r10, -0x26c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	// 8268C77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C7A0: 4BDDA681  bl 0x82466e20
	ctx.lr = 0x8268C7A4;
	sub_82466E20(ctx, base);
	// 8268C7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C7B8 size=112
    let mut pc: u32 = 0x8268C7B8;
    'dispatch: loop {
        match pc {
            0x8268C7B8 => {
    //   block [0x8268C7B8..0x8268C828)
	// 8268C7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C7C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C7CC: 38AAD878  addi r5, r10, -0x2788
	ctx.r[5].s64 = ctx.r[10].s64 + -10120;
	// 8268C7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C7D4: 390B5720  addi r8, r11, 0x5720
	ctx.r[8].s64 = ctx.r[11].s64 + 22304;
	// 8268C7D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268C7DC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8268C7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C7E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C7F0: 386AD968  addi r3, r10, -0x2698
	ctx.r[3].s64 = ctx.r[10].s64 + -9880;
	// 8268C7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C814: 4BDDA60D  bl 0x82466e20
	ctx.lr = 0x8268C818;
	sub_82466E20(ctx, base);
	// 8268C818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C828 size=108
    let mut pc: u32 = 0x8268C828;
    'dispatch: loop {
        match pc {
            0x8268C828 => {
    //   block [0x8268C828..0x8268C894)
	// 8268C828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C834: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C83C: 38EB57C8  addi r7, r11, 0x57c8
	ctx.r[7].s64 = ctx.r[11].s64 + 22472;
	// 8268C840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268C844: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8268C848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C858: 386AD998  addi r3, r10, -0x2668
	ctx.r[3].s64 = ctx.r[10].s64 + -9832;
	// 8268C85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C880: 4BDDA5A1  bl 0x82466e20
	ctx.lr = 0x8268C884;
	sub_82466E20(ctx, base);
	// 8268C884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C898 size=108
    let mut pc: u32 = 0x8268C898;
    'dispatch: loop {
        match pc {
            0x8268C898 => {
    //   block [0x8268C898..0x8268C904)
	// 8268C898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C8A4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C8AC: 38EB57E0  addi r7, r11, 0x57e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22496;
	// 8268C8B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268C8B4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8268C8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C8C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C8C8: 386AD9C8  addi r3, r10, -0x2638
	ctx.r[3].s64 = ctx.r[10].s64 + -9784;
	// 8268C8CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C8EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C8F0: 4BDDA531  bl 0x82466e20
	ctx.lr = 0x8268C8F4;
	sub_82466E20(ctx, base);
	// 8268C8F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C908 size=112
    let mut pc: u32 = 0x8268C908;
    'dispatch: loop {
        match pc {
            0x8268C908 => {
    //   block [0x8268C908..0x8268C978)
	// 8268C908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C914: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C918: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C91C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268C920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C924: 390B5840  addi r8, r11, 0x5840
	ctx.r[8].s64 = ctx.r[11].s64 + 22592;
	// 8268C928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268C92C: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8268C930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268C93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C940: 386AD9F8  addi r3, r10, -0x2608
	ctx.r[3].s64 = ctx.r[10].s64 + -9736;
	// 8268C944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268C948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C964: 4BDDA4BD  bl 0x82466e20
	ctx.lr = 0x8268C968;
	sub_82466E20(ctx, base);
	// 8268C968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C978 size=108
    let mut pc: u32 = 0x8268C978;
    'dispatch: loop {
        match pc {
            0x8268C978 => {
    //   block [0x8268C978..0x8268C9E4)
	// 8268C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C984: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C98C: 38EB5858  addi r7, r11, 0x5858
	ctx.r[7].s64 = ctx.r[11].s64 + 22616;
	// 8268C990: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268C994: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8268C998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268C99C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268C9A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268C9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268C9A8: 386ADA28  addi r3, r10, -0x25d8
	ctx.r[3].s64 = ctx.r[10].s64 + -9688;
	// 8268C9AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268C9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268C9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268C9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268C9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268C9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268C9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268C9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268C9CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268C9D0: 4BDDA451  bl 0x82466e20
	ctx.lr = 0x8268C9D4;
	sub_82466E20(ctx, base);
	// 8268C9D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268C9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268C9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268C9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268C9E8 size=108
    let mut pc: u32 = 0x8268C9E8;
    'dispatch: loop {
        match pc {
            0x8268C9E8 => {
    //   block [0x8268C9E8..0x8268CA54)
	// 8268C9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268C9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268C9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268C9F4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268C9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268C9FC: 38EB58A0  addi r7, r11, 0x58a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22688;
	// 8268CA00: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268CA04: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8268CA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CA0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CA10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CA18: 386ADA58  addi r3, r10, -0x25a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9640;
	// 8268CA1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CA3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CA40: 4BDDA3E1  bl 0x82466e20
	ctx.lr = 0x8268CA44;
	sub_82466E20(ctx, base);
	// 8268CA44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CA58 size=108
    let mut pc: u32 = 0x8268CA58;
    'dispatch: loop {
        match pc {
            0x8268CA58 => {
    //   block [0x8268CA58..0x8268CAC4)
	// 8268CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CA64: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CA68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CA6C: 38EB5930  addi r7, r11, 0x5930
	ctx.r[7].s64 = ctx.r[11].s64 + 22832;
	// 8268CA70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268CA74: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8268CA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CA7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CA80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CA88: 386ADA88  addi r3, r10, -0x2578
	ctx.r[3].s64 = ctx.r[10].s64 + -9592;
	// 8268CA8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CAAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CAB0: 4BDDA371  bl 0x82466e20
	ctx.lr = 0x8268CAB4;
	sub_82466E20(ctx, base);
	// 8268CAB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CAC8 size=100
    let mut pc: u32 = 0x8268CAC8;
    'dispatch: loop {
        match pc {
            0x8268CAC8 => {
    //   block [0x8268CAC8..0x8268CB2C)
	// 8268CAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CADC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268CAE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CAE8: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8268CAEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CAFC: 386ADAB8  addi r3, r10, -0x2548
	ctx.r[3].s64 = ctx.r[10].s64 + -9544;
	// 8268CB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CB04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CB08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268CB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CB10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268CB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CB18: 4BDDA309  bl 0x82466e20
	ctx.lr = 0x8268CB1C;
	sub_82466E20(ctx, base);
	// 8268CB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CB30 size=112
    let mut pc: u32 = 0x8268CB30;
    'dispatch: loop {
        match pc {
            0x8268CB30 => {
    //   block [0x8268CB30..0x8268CBA0)
	// 8268CB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CB3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CB40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CB44: 38AADAB8  addi r5, r10, -0x2548
	ctx.r[5].s64 = ctx.r[10].s64 + -9544;
	// 8268CB48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CB4C: 390B59C0  addi r8, r11, 0x59c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22976;
	// 8268CB50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268CB54: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8268CB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CB5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CB60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268CB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CB68: 386ADAE8  addi r3, r10, -0x2518
	ctx.r[3].s64 = ctx.r[10].s64 + -9496;
	// 8268CB6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268CB70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CB84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CB8C: 4BDDA295  bl 0x82466e20
	ctx.lr = 0x8268CB90;
	sub_82466E20(ctx, base);
	// 8268CB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CBA0 size=108
    let mut pc: u32 = 0x8268CBA0;
    'dispatch: loop {
        match pc {
            0x8268CBA0 => {
    //   block [0x8268CBA0..0x8268CC0C)
	// 8268CBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CBAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CBB4: 38EB5A20  addi r7, r11, 0x5a20
	ctx.r[7].s64 = ctx.r[11].s64 + 23072;
	// 8268CBB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268CBBC: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8268CBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CBC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CBC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CBD0: 386ADB18  addi r3, r10, -0x24e8
	ctx.r[3].s64 = ctx.r[10].s64 + -9448;
	// 8268CBD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CBF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CBF8: 4BDDA229  bl 0x82466e20
	ctx.lr = 0x8268CBFC;
	sub_82466E20(ctx, base);
	// 8268CBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CC10 size=108
    let mut pc: u32 = 0x8268CC10;
    'dispatch: loop {
        match pc {
            0x8268CC10 => {
    //   block [0x8268CC10..0x8268CC7C)
	// 8268CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CC1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CC24: 38EB5A50  addi r7, r11, 0x5a50
	ctx.r[7].s64 = ctx.r[11].s64 + 23120;
	// 8268CC28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268CC2C: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8268CC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CC34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CC40: 386ADB48  addi r3, r10, -0x24b8
	ctx.r[3].s64 = ctx.r[10].s64 + -9400;
	// 8268CC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CC68: 4BDDA1B9  bl 0x82466e20
	ctx.lr = 0x8268CC6C;
	sub_82466E20(ctx, base);
	// 8268CC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CC80 size=108
    let mut pc: u32 = 0x8268CC80;
    'dispatch: loop {
        match pc {
            0x8268CC80 => {
    //   block [0x8268CC80..0x8268CCEC)
	// 8268CC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CC8C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268CC94: 38EB5A98  addi r7, r11, 0x5a98
	ctx.r[7].s64 = ctx.r[11].s64 + 23192;
	// 8268CC98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268CC9C: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8268CCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CCA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CCB0: 386ADB78  addi r3, r10, -0x2488
	ctx.r[3].s64 = ctx.r[10].s64 + -9352;
	// 8268CCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CCD8: 4BDDA149  bl 0x82466e20
	ctx.lr = 0x8268CCDC;
	sub_82466E20(ctx, base);
	// 8268CCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CCF0 size=96
    let mut pc: u32 = 0x8268CCF0;
    'dispatch: loop {
        match pc {
            0x8268CCF0 => {
    //   block [0x8268CCF0..0x8268CD50)
	// 8268CCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CCFC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CD04: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8268CD08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CD10: 386ADBA8  addi r3, r10, -0x2458
	ctx.r[3].s64 = ctx.r[10].s64 + -9304;
	// 8268CD14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CD1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268CD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CD30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268CD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CD38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268CD3C: 4BDDA0E5  bl 0x82466e20
	ctx.lr = 0x8268CD40;
	sub_82466E20(ctx, base);
	// 8268CD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CD50 size=112
    let mut pc: u32 = 0x8268CD50;
    'dispatch: loop {
        match pc {
            0x8268CD50 => {
    //   block [0x8268CD50..0x8268CDC0)
	// 8268CD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CD5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CD60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CD64: 38AADBA8  addi r5, r10, -0x2458
	ctx.r[5].s64 = ctx.r[10].s64 + -9304;
	// 8268CD68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CD6C: 390B5AF8  addi r8, r11, 0x5af8
	ctx.r[8].s64 = ctx.r[11].s64 + 23288;
	// 8268CD70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268CD74: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8268CD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268CD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CD88: 386ADBD8  addi r3, r10, -0x2428
	ctx.r[3].s64 = ctx.r[10].s64 + -9256;
	// 8268CD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268CD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CDAC: 4BDDA075  bl 0x82466e20
	ctx.lr = 0x8268CDB0;
	sub_82466E20(ctx, base);
	// 8268CDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CDC0 size=112
    let mut pc: u32 = 0x8268CDC0;
    'dispatch: loop {
        match pc {
            0x8268CDC0 => {
    //   block [0x8268CDC0..0x8268CE30)
	// 8268CDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CDCC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268CDD0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CDD4: 392A6890  addi r9, r10, 0x6890
	ctx.r[9].s64 = ctx.r[10].s64 + 26768;
	// 8268CDD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CDDC: 390B5B28  addi r8, r11, 0x5b28
	ctx.r[8].s64 = ctx.r[11].s64 + 23336;
	// 8268CDE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8268CDE4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8268CDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268CDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CDF8: 386ADC08  addi r3, r10, -0x23f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9208;
	// 8268CDFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268CE00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268CE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CE1C: 4BDDA005  bl 0x82466e20
	ctx.lr = 0x8268CE20;
	sub_82466E20(ctx, base);
	// 8268CE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CE30 size=108
    let mut pc: u32 = 0x8268CE30;
    'dispatch: loop {
        match pc {
            0x8268CE30 => {
    //   block [0x8268CE30..0x8268CE9C)
	// 8268CE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CE3C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CE40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CE44: 38EB5BD0  addi r7, r11, 0x5bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 23504;
	// 8268CE48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268CE4C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8268CE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CE54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CE58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CE60: 386ADC38  addi r3, r10, -0x23c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9160;
	// 8268CE64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CE88: 4BDD9F99  bl 0x82466e20
	ctx.lr = 0x8268CE8C;
	sub_82466E20(ctx, base);
	// 8268CE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CEA0 size=108
    let mut pc: u32 = 0x8268CEA0;
    'dispatch: loop {
        match pc {
            0x8268CEA0 => {
    //   block [0x8268CEA0..0x8268CF0C)
	// 8268CEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CEAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CEB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CEB4: 38EB5C00  addi r7, r11, 0x5c00
	ctx.r[7].s64 = ctx.r[11].s64 + 23552;
	// 8268CEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268CEBC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8268CEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CEC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CED0: 386ADC68  addi r3, r10, -0x2398
	ctx.r[3].s64 = ctx.r[10].s64 + -9112;
	// 8268CED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CEF8: 4BDD9F29  bl 0x82466e20
	ctx.lr = 0x8268CEFC;
	sub_82466E20(ctx, base);
	// 8268CEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268CF10 size=28
    let mut pc: u32 = 0x8268CF10;
    'dispatch: loop {
        match pc {
            0x8268CF10 => {
    //   block [0x8268CF10..0x8268CF2C)
	// 8268CF10: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CF14: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268CF18: 394AA368  addi r10, r10, -0x5c98
	ctx.r[10].s64 = ctx.r[10].s64 + -23704;
	// 8268CF1C: 816B5C30  lwz r11, 0x5c30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23600 as u32) ) } as u64;
	// 8268CF20: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268CF24: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8268CF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CF30 size=112
    let mut pc: u32 = 0x8268CF30;
    'dispatch: loop {
        match pc {
            0x8268CF30 => {
    //   block [0x8268CF30..0x8268CFA0)
	// 8268CF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CF3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268CF40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268CF44: 392A6A00  addi r9, r10, 0x6a00
	ctx.r[9].s64 = ctx.r[10].s64 + 27136;
	// 8268CF48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CF4C: 390BA368  addi r8, r11, -0x5c98
	ctx.r[8].s64 = ctx.r[11].s64 + -23704;
	// 8268CF50: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8268CF54: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8268CF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CF5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268CF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CF68: 386ADC98  addi r3, r10, -0x2368
	ctx.r[3].s64 = ctx.r[10].s64 + -9064;
	// 8268CF6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268CF70: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8268CF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CF84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CF8C: 4BDD9E95  bl 0x82466e20
	ctx.lr = 0x8268CF90;
	sub_82466E20(ctx, base);
	// 8268CF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268CF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268CF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268CFA0 size=108
    let mut pc: u32 = 0x8268CFA0;
    'dispatch: loop {
        match pc {
            0x8268CFA0 => {
    //   block [0x8268CFA0..0x8268D00C)
	// 8268CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268CFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268CFAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268CFB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268CFB4: 38EB5C3C  addi r7, r11, 0x5c3c
	ctx.r[7].s64 = ctx.r[11].s64 + 23612;
	// 8268CFB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268CFBC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8268CFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268CFC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268CFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268CFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268CFD0: 386ADCC8  addi r3, r10, -0x2338
	ctx.r[3].s64 = ctx.r[10].s64 + -9016;
	// 8268CFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268CFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268CFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268CFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268CFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268CFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268CFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268CFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268CFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268CFF8: 4BDD9E29  bl 0x82466e20
	ctx.lr = 0x8268CFFC;
	sub_82466E20(ctx, base);
	// 8268CFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D010 size=108
    let mut pc: u32 = 0x8268D010;
    'dispatch: loop {
        match pc {
            0x8268D010 => {
    //   block [0x8268D010..0x8268D07C)
	// 8268D010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D01C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D020: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D024: 38EB5C6C  addi r7, r11, 0x5c6c
	ctx.r[7].s64 = ctx.r[11].s64 + 23660;
	// 8268D028: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268D02C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8268D030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D040: 386ADCF8  addi r3, r10, -0x2308
	ctx.r[3].s64 = ctx.r[10].s64 + -8968;
	// 8268D044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D068: 4BDD9DB9  bl 0x82466e20
	ctx.lr = 0x8268D06C;
	sub_82466E20(ctx, base);
	// 8268D06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268D080 size=24
    let mut pc: u32 = 0x8268D080;
    'dispatch: loop {
        match pc {
            0x8268D080 => {
    //   block [0x8268D080..0x8268D098)
	// 8268D080: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D084: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268D088: 394AA428  addi r10, r10, -0x5bd8
	ctx.r[10].s64 = ctx.r[10].s64 + -23512;
	// 8268D08C: 816B5C84  lwz r11, 0x5c84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23684 as u32) ) } as u64;
	// 8268D090: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268D094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D098 size=112
    let mut pc: u32 = 0x8268D098;
    'dispatch: loop {
        match pc {
            0x8268D098 => {
    //   block [0x8268D098..0x8268D108)
	// 8268D098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D0A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268D0A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268D0AC: 392A6A54  addi r9, r10, 0x6a54
	ctx.r[9].s64 = ctx.r[10].s64 + 27220;
	// 8268D0B0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D0B4: 390BA428  addi r8, r11, -0x5bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -23512;
	// 8268D0B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8268D0BC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8268D0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D0C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268D0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D0D0: 386ADD28  addi r3, r10, -0x22d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8920;
	// 8268D0D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268D0D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268D0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D0EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D0F4: 4BDD9D2D  bl 0x82466e20
	ctx.lr = 0x8268D0F8;
	sub_82466E20(ctx, base);
	// 8268D0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D108 size=108
    let mut pc: u32 = 0x8268D108;
    'dispatch: loop {
        match pc {
            0x8268D108 => {
    //   block [0x8268D108..0x8268D174)
	// 8268D108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D114: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D118: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D11C: 38EB5C88  addi r7, r11, 0x5c88
	ctx.r[7].s64 = ctx.r[11].s64 + 23688;
	// 8268D120: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D124: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8268D128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D12C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D138: 386ADD58  addi r3, r10, -0x22a8
	ctx.r[3].s64 = ctx.r[10].s64 + -8872;
	// 8268D13C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D160: 4BDD9CC1  bl 0x82466e20
	ctx.lr = 0x8268D164;
	sub_82466E20(ctx, base);
	// 8268D164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D178 size=108
    let mut pc: u32 = 0x8268D178;
    'dispatch: loop {
        match pc {
            0x8268D178 => {
    //   block [0x8268D178..0x8268D1E4)
	// 8268D178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D184: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D188: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D18C: 38EB5CB8  addi r7, r11, 0x5cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 23736;
	// 8268D190: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D194: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8268D198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D19C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D1A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D1A8: 386ADD88  addi r3, r10, -0x2278
	ctx.r[3].s64 = ctx.r[10].s64 + -8824;
	// 8268D1AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D1B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D1CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D1D0: 4BDD9C51  bl 0x82466e20
	ctx.lr = 0x8268D1D4;
	sub_82466E20(ctx, base);
	// 8268D1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D1E8 size=112
    let mut pc: u32 = 0x8268D1E8;
    'dispatch: loop {
        match pc {
            0x8268D1E8 => {
    //   block [0x8268D1E8..0x8268D258)
	// 8268D1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D1F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268D1F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D1FC: 392A6A78  addi r9, r10, 0x6a78
	ctx.r[9].s64 = ctx.r[10].s64 + 27256;
	// 8268D200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D204: 390B5CF0  addi r8, r11, 0x5cf0
	ctx.r[8].s64 = ctx.r[11].s64 + 23792;
	// 8268D208: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8268D20C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8268D210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D214: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268D21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D220: 386ADDB8  addi r3, r10, -0x2248
	ctx.r[3].s64 = ctx.r[10].s64 + -8776;
	// 8268D224: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268D228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268D22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D244: 4BDD9BDD  bl 0x82466e20
	ctx.lr = 0x8268D248;
	sub_82466E20(ctx, base);
	// 8268D248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D258 size=108
    let mut pc: u32 = 0x8268D258;
    'dispatch: loop {
        match pc {
            0x8268D258 => {
    //   block [0x8268D258..0x8268D2C4)
	// 8268D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D264: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D268: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D26C: 38EB5D50  addi r7, r11, 0x5d50
	ctx.r[7].s64 = ctx.r[11].s64 + 23888;
	// 8268D270: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268D274: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8268D278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D27C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D288: 386ADDE8  addi r3, r10, -0x2218
	ctx.r[3].s64 = ctx.r[10].s64 + -8728;
	// 8268D28C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D2A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D2AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D2B0: 4BDD9B71  bl 0x82466e20
	ctx.lr = 0x8268D2B4;
	sub_82466E20(ctx, base);
	// 8268D2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D2C8 size=108
    let mut pc: u32 = 0x8268D2C8;
    'dispatch: loop {
        match pc {
            0x8268D2C8 => {
    //   block [0x8268D2C8..0x8268D334)
	// 8268D2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D2D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D2D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268D2DC: 38EB5E10  addi r7, r11, 0x5e10
	ctx.r[7].s64 = ctx.r[11].s64 + 24080;
	// 8268D2E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268D2E4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8268D2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D2EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D2F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D2F8: 386ADE18  addi r3, r10, -0x21e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8680;
	// 8268D2FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D31C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D320: 4BDD9B01  bl 0x82466e20
	ctx.lr = 0x8268D324;
	sub_82466E20(ctx, base);
	// 8268D324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D338 size=108
    let mut pc: u32 = 0x8268D338;
    'dispatch: loop {
        match pc {
            0x8268D338 => {
    //   block [0x8268D338..0x8268D3A4)
	// 8268D338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D344: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D34C: 38EB5E28  addi r7, r11, 0x5e28
	ctx.r[7].s64 = ctx.r[11].s64 + 24104;
	// 8268D350: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8268D354: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8268D358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D35C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D368: 386ADE48  addi r3, r10, -0x21b8
	ctx.r[3].s64 = ctx.r[10].s64 + -8632;
	// 8268D36C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D390: 4BDD9A91  bl 0x82466e20
	ctx.lr = 0x8268D394;
	sub_82466E20(ctx, base);
	// 8268D394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268D3A8 size=24
    let mut pc: u32 = 0x8268D3A8;
    'dispatch: loop {
        match pc {
            0x8268D3A8 => {
    //   block [0x8268D3A8..0x8268D3C0)
	// 8268D3A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D3AC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268D3B0: 394AA4B8  addi r10, r10, -0x5b48
	ctx.r[10].s64 = ctx.r[10].s64 + -23368;
	// 8268D3B4: 816B5CEC  lwz r11, 0x5cec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23788 as u32) ) } as u64;
	// 8268D3B8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8268D3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D3C0 size=108
    let mut pc: u32 = 0x8268D3C0;
    'dispatch: loop {
        match pc {
            0x8268D3C0 => {
    //   block [0x8268D3C0..0x8268D42C)
	// 8268D3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D3CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268D3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D3D4: 38EBA4B8  addi r7, r11, -0x5b48
	ctx.r[7].s64 = ctx.r[11].s64 + -23368;
	// 8268D3D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D3DC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8268D3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D3E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D3E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D3F0: 386ADE78  addi r3, r10, -0x2188
	ctx.r[3].s64 = ctx.r[10].s64 + -8584;
	// 8268D3F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D418: 4BDD9A09  bl 0x82466e20
	ctx.lr = 0x8268D41C;
	sub_82466E20(ctx, base);
	// 8268D41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268D430 size=24
    let mut pc: u32 = 0x8268D430;
    'dispatch: loop {
        match pc {
            0x8268D430 => {
    //   block [0x8268D430..0x8268D448)
	// 8268D430: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D434: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268D438: 394AA4E8  addi r10, r10, -0x5b18
	ctx.r[10].s64 = ctx.r[10].s64 + -23320;
	// 8268D43C: 816B5CEC  lwz r11, 0x5cec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23788 as u32) ) } as u64;
	// 8268D440: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8268D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D448 size=108
    let mut pc: u32 = 0x8268D448;
    'dispatch: loop {
        match pc {
            0x8268D448 => {
    //   block [0x8268D448..0x8268D4B4)
	// 8268D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D454: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268D458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D45C: 38EBA4E8  addi r7, r11, -0x5b18
	ctx.r[7].s64 = ctx.r[11].s64 + -23320;
	// 8268D460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D464: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8268D468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D46C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D478: 386ADEA8  addi r3, r10, -0x2158
	ctx.r[3].s64 = ctx.r[10].s64 + -8536;
	// 8268D47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D4A0: 4BDD9981  bl 0x82466e20
	ctx.lr = 0x8268D4A4;
	sub_82466E20(ctx, base);
	// 8268D4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D4B8 size=108
    let mut pc: u32 = 0x8268D4B8;
    'dispatch: loop {
        match pc {
            0x8268D4B8 => {
    //   block [0x8268D4B8..0x8268D524)
	// 8268D4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D4C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D4CC: 38EB5EA0  addi r7, r11, 0x5ea0
	ctx.r[7].s64 = ctx.r[11].s64 + 24224;
	// 8268D4D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268D4D4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8268D4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D4DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D4E8: 386ADED8  addi r3, r10, -0x2128
	ctx.r[3].s64 = ctx.r[10].s64 + -8488;
	// 8268D4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D510: 4BDD9911  bl 0x82466e20
	ctx.lr = 0x8268D514;
	sub_82466E20(ctx, base);
	// 8268D514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268D528 size=24
    let mut pc: u32 = 0x8268D528;
    'dispatch: loop {
        match pc {
            0x8268D528 => {
    //   block [0x8268D528..0x8268D540)
	// 8268D528: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D52C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268D530: 394AA518  addi r10, r10, -0x5ae8
	ctx.r[10].s64 = ctx.r[10].s64 + -23272;
	// 8268D534: 816B5CEC  lwz r11, 0x5cec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23788 as u32) ) } as u64;
	// 8268D538: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8268D53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D540 size=108
    let mut pc: u32 = 0x8268D540;
    'dispatch: loop {
        match pc {
            0x8268D540 => {
    //   block [0x8268D540..0x8268D5AC)
	// 8268D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D54C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D554: 38EBA518  addi r7, r11, -0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + -23272;
	// 8268D558: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D55C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8268D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D564: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D570: 386ADF08  addi r3, r10, -0x20f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8440;
	// 8268D574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D598: 4BDD9889  bl 0x82466e20
	ctx.lr = 0x8268D59C;
	sub_82466E20(ctx, base);
	// 8268D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D5B0 size=112
    let mut pc: u32 = 0x8268D5B0;
    'dispatch: loop {
        match pc {
            0x8268D5B0 => {
    //   block [0x8268D5B0..0x8268D620)
	// 8268D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D5BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268D5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D5C4: 392A6ABC  addi r9, r10, 0x6abc
	ctx.r[9].s64 = ctx.r[10].s64 + 27324;
	// 8268D5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D5CC: 390B5EB8  addi r8, r11, 0x5eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24248;
	// 8268D5D0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8268D5D4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8268D5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D5DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268D5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D5E8: 386ADF38  addi r3, r10, -0x20c8
	ctx.r[3].s64 = ctx.r[10].s64 + -8392;
	// 8268D5EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268D5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268D5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D60C: 4BDD9815  bl 0x82466e20
	ctx.lr = 0x8268D610;
	sub_82466E20(ctx, base);
	// 8268D610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D620 size=108
    let mut pc: u32 = 0x8268D620;
    'dispatch: loop {
        match pc {
            0x8268D620 => {
    //   block [0x8268D620..0x8268D68C)
	// 8268D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D62C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D634: 38EB5EE8  addi r7, r11, 0x5ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 24296;
	// 8268D638: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D63C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8268D640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D650: 386ADF68  addi r3, r10, -0x2098
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	// 8268D654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D678: 4BDD97A9  bl 0x82466e20
	ctx.lr = 0x8268D67C;
	sub_82466E20(ctx, base);
	// 8268D67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D690 size=108
    let mut pc: u32 = 0x8268D690;
    'dispatch: loop {
        match pc {
            0x8268D690 => {
    //   block [0x8268D690..0x8268D6FC)
	// 8268D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D69C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D6A4: 38EB5F18  addi r7, r11, 0x5f18
	ctx.r[7].s64 = ctx.r[11].s64 + 24344;
	// 8268D6A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D6AC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8268D6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D6B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D6C0: 386ADF98  addi r3, r10, -0x2068
	ctx.r[3].s64 = ctx.r[10].s64 + -8296;
	// 8268D6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D6E8: 4BDD9739  bl 0x82466e20
	ctx.lr = 0x8268D6EC;
	sub_82466E20(ctx, base);
	// 8268D6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D700 size=112
    let mut pc: u32 = 0x8268D700;
    'dispatch: loop {
        match pc {
            0x8268D700 => {
    //   block [0x8268D700..0x8268D770)
	// 8268D700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D70C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D714: 38AADFF8  addi r5, r10, -0x2008
	ctx.r[5].s64 = ctx.r[10].s64 + -8200;
	// 8268D718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D71C: 390B5F48  addi r8, r11, 0x5f48
	ctx.r[8].s64 = ctx.r[11].s64 + 24392;
	// 8268D720: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268D724: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8268D728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D72C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268D734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D738: 386ADFC8  addi r3, r10, -0x2038
	ctx.r[3].s64 = ctx.r[10].s64 + -8248;
	// 8268D73C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268D740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D75C: 4BDD96C5  bl 0x82466e20
	ctx.lr = 0x8268D760;
	sub_82466E20(ctx, base);
	// 8268D760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D770 size=108
    let mut pc: u32 = 0x8268D770;
    'dispatch: loop {
        match pc {
            0x8268D770 => {
    //   block [0x8268D770..0x8268D7DC)
	// 8268D770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D77C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D784: 38EB5F60  addi r7, r11, 0x5f60
	ctx.r[7].s64 = ctx.r[11].s64 + 24416;
	// 8268D788: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D78C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8268D790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D794: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D7A0: 386ADFF8  addi r3, r10, -0x2008
	ctx.r[3].s64 = ctx.r[10].s64 + -8200;
	// 8268D7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D7C8: 4BDD9659  bl 0x82466e20
	ctx.lr = 0x8268D7CC;
	sub_82466E20(ctx, base);
	// 8268D7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D7E0 size=108
    let mut pc: u32 = 0x8268D7E0;
    'dispatch: loop {
        match pc {
            0x8268D7E0 => {
    //   block [0x8268D7E0..0x8268D84C)
	// 8268D7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D7EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D7F4: 38EB5F90  addi r7, r11, 0x5f90
	ctx.r[7].s64 = ctx.r[11].s64 + 24464;
	// 8268D7F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268D7FC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8268D800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D810: 386AE028  addi r3, r10, -0x1fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -8152;
	// 8268D814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D838: 4BDD95E9  bl 0x82466e20
	ctx.lr = 0x8268D83C;
	sub_82466E20(ctx, base);
	// 8268D83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D850 size=108
    let mut pc: u32 = 0x8268D850;
    'dispatch: loop {
        match pc {
            0x8268D850 => {
    //   block [0x8268D850..0x8268D8BC)
	// 8268D850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D85C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D864: 38EB5FA8  addi r7, r11, 0x5fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 24488;
	// 8268D868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D86C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8268D870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D880: 386AE058  addi r3, r10, -0x1fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -8104;
	// 8268D884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D8A8: 4BDD9579  bl 0x82466e20
	ctx.lr = 0x8268D8AC;
	sub_82466E20(ctx, base);
	// 8268D8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D8C0 size=108
    let mut pc: u32 = 0x8268D8C0;
    'dispatch: loop {
        match pc {
            0x8268D8C0 => {
    //   block [0x8268D8C0..0x8268D92C)
	// 8268D8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D8CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D8D4: 38EB5FD8  addi r7, r11, 0x5fd8
	ctx.r[7].s64 = ctx.r[11].s64 + 24536;
	// 8268D8D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8268D8DC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8268D8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D8E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D8F0: 386AE088  addi r3, r10, -0x1f78
	ctx.r[3].s64 = ctx.r[10].s64 + -8056;
	// 8268D8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D918: 4BDD9509  bl 0x82466e20
	ctx.lr = 0x8268D91C;
	sub_82466E20(ctx, base);
	// 8268D91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D930 size=108
    let mut pc: u32 = 0x8268D930;
    'dispatch: loop {
        match pc {
            0x8268D930 => {
    //   block [0x8268D930..0x8268D99C)
	// 8268D930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D93C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D944: 38EB6080  addi r7, r11, 0x6080
	ctx.r[7].s64 = ctx.r[11].s64 + 24704;
	// 8268D948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268D94C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8268D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D960: 386AE0B8  addi r3, r10, -0x1f48
	ctx.r[3].s64 = ctx.r[10].s64 + -8008;
	// 8268D964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D988: 4BDD9499  bl 0x82466e20
	ctx.lr = 0x8268D98C;
	sub_82466E20(ctx, base);
	// 8268D98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268D990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268D994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268D998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268D9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268D9A0 size=108
    let mut pc: u32 = 0x8268D9A0;
    'dispatch: loop {
        match pc {
            0x8268D9A0 => {
    //   block [0x8268D9A0..0x8268DA0C)
	// 8268D9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268D9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268D9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268D9AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268D9B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268D9B4: 38EB60B0  addi r7, r11, 0x60b0
	ctx.r[7].s64 = ctx.r[11].s64 + 24752;
	// 8268D9B8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268D9BC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8268D9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268D9C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268D9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268D9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268D9D0: 386AE0E8  addi r3, r10, -0x1f18
	ctx.r[3].s64 = ctx.r[10].s64 + -7960;
	// 8268D9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268D9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268D9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268D9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268D9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268D9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268D9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268D9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268D9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268D9F8: 4BDD9429  bl 0x82466e20
	ctx.lr = 0x8268D9FC;
	sub_82466E20(ctx, base);
	// 8268D9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268DA10 size=24
    let mut pc: u32 = 0x8268DA10;
    'dispatch: loop {
        match pc {
            0x8268DA10 => {
    //   block [0x8268DA10..0x8268DA28)
	// 8268DA10: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DA14: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268DA18: 394AA548  addi r10, r10, -0x5ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -23224;
	// 8268DA1C: 816B6170  lwz r11, 0x6170(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24944 as u32) ) } as u64;
	// 8268DA20: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8268DA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DA28 size=112
    let mut pc: u32 = 0x8268DA28;
    'dispatch: loop {
        match pc {
            0x8268DA28 => {
    //   block [0x8268DA28..0x8268DA98)
	// 8268DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DA34: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268DA38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268DA3C: 392A6AE8  addi r9, r10, 0x6ae8
	ctx.r[9].s64 = ctx.r[10].s64 + 27368;
	// 8268DA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DA44: 390BA548  addi r8, r11, -0x5ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -23224;
	// 8268DA48: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8268DA4C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8268DA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DA54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268DA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DA60: 386AE118  addi r3, r10, -0x1ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -7912;
	// 8268DA64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268DA68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268DA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DA84: 4BDD939D  bl 0x82466e20
	ctx.lr = 0x8268DA88;
	sub_82466E20(ctx, base);
	// 8268DA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DA98 size=108
    let mut pc: u32 = 0x8268DA98;
    'dispatch: loop {
        match pc {
            0x8268DA98 => {
    //   block [0x8268DA98..0x8268DB04)
	// 8268DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DAA4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DAAC: 38EB6178  addi r7, r11, 0x6178
	ctx.r[7].s64 = ctx.r[11].s64 + 24952;
	// 8268DAB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268DAB4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8268DAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DAC8: 386AE148  addi r3, r10, -0x1eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -7864;
	// 8268DACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DAF0: 4BDD9331  bl 0x82466e20
	ctx.lr = 0x8268DAF4;
	sub_82466E20(ctx, base);
	// 8268DAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DB08 size=112
    let mut pc: u32 = 0x8268DB08;
    'dispatch: loop {
        match pc {
            0x8268DB08 => {
    //   block [0x8268DB08..0x8268DB78)
	// 8268DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DB14: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268DB18: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DB1C: 392A6B2C  addi r9, r10, 0x6b2c
	ctx.r[9].s64 = ctx.r[10].s64 + 27436;
	// 8268DB20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DB24: 390B61A8  addi r8, r11, 0x61a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25000;
	// 8268DB28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8268DB2C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8268DB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DB34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DB38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268DB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DB40: 386AE178  addi r3, r10, -0x1e88
	ctx.r[3].s64 = ctx.r[10].s64 + -7816;
	// 8268DB44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268DB48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268DB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DB64: 4BDD92BD  bl 0x82466e20
	ctx.lr = 0x8268DB68;
	sub_82466E20(ctx, base);
	// 8268DB68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268DB78 size=24
    let mut pc: u32 = 0x8268DB78;
    'dispatch: loop {
        match pc {
            0x8268DB78 => {
    //   block [0x8268DB78..0x8268DB90)
	// 8268DB78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DB7C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268DB80: 394AA5C0  addi r10, r10, -0x5a40
	ctx.r[10].s64 = ctx.r[10].s64 + -23104;
	// 8268DB84: 816B6268  lwz r11, 0x6268(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25192 as u32) ) } as u64;
	// 8268DB88: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8268DB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DB90 size=112
    let mut pc: u32 = 0x8268DB90;
    'dispatch: loop {
        match pc {
            0x8268DB90 => {
    //   block [0x8268DB90..0x8268DC00)
	// 8268DB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DB9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268DBA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268DBA4: 392A6B68  addi r9, r10, 0x6b68
	ctx.r[9].s64 = ctx.r[10].s64 + 27496;
	// 8268DBA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DBAC: 390BA5C0  addi r8, r11, -0x5a40
	ctx.r[8].s64 = ctx.r[11].s64 + -23104;
	// 8268DBB0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8268DBB4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8268DBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DBBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DBC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268DBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DBC8: 386AE1A8  addi r3, r10, -0x1e58
	ctx.r[3].s64 = ctx.r[10].s64 + -7768;
	// 8268DBCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268DBD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268DBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DBEC: 4BDD9235  bl 0x82466e20
	ctx.lr = 0x8268DBF0;
	sub_82466E20(ctx, base);
	// 8268DBF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DC00 size=108
    let mut pc: u32 = 0x8268DC00;
    'dispatch: loop {
        match pc {
            0x8268DC00 => {
    //   block [0x8268DC00..0x8268DC6C)
	// 8268DC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DC0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DC14: 38EB626C  addi r7, r11, 0x626c
	ctx.r[7].s64 = ctx.r[11].s64 + 25196;
	// 8268DC18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268DC1C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8268DC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DC30: 386AE1D8  addi r3, r10, -0x1e28
	ctx.r[3].s64 = ctx.r[10].s64 + -7720;
	// 8268DC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DC58: 4BDD91C9  bl 0x82466e20
	ctx.lr = 0x8268DC5C;
	sub_82466E20(ctx, base);
	// 8268DC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DC70 size=108
    let mut pc: u32 = 0x8268DC70;
    'dispatch: loop {
        match pc {
            0x8268DC70 => {
    //   block [0x8268DC70..0x8268DCDC)
	// 8268DC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DC7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DC84: 38EB6284  addi r7, r11, 0x6284
	ctx.r[7].s64 = ctx.r[11].s64 + 25220;
	// 8268DC88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268DC8C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8268DC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DC94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DCA0: 386AE208  addi r3, r10, -0x1df8
	ctx.r[3].s64 = ctx.r[10].s64 + -7672;
	// 8268DCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DCC8: 4BDD9159  bl 0x82466e20
	ctx.lr = 0x8268DCCC;
	sub_82466E20(ctx, base);
	// 8268DCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268DCE0 size=24
    let mut pc: u32 = 0x8268DCE0;
    'dispatch: loop {
        match pc {
            0x8268DCE0 => {
    //   block [0x8268DCE0..0x8268DCF8)
	// 8268DCE0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DCE4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268DCE8: 394AA608  addi r10, r10, -0x59f8
	ctx.r[10].s64 = ctx.r[10].s64 + -23032;
	// 8268DCEC: 816B62B4  lwz r11, 0x62b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25268 as u32) ) } as u64;
	// 8268DCF0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8268DCF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DCF8 size=112
    let mut pc: u32 = 0x8268DCF8;
    'dispatch: loop {
        match pc {
            0x8268DCF8 => {
    //   block [0x8268DCF8..0x8268DD68)
	// 8268DCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DD04: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268DD08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268DD0C: 392A6BA4  addi r9, r10, 0x6ba4
	ctx.r[9].s64 = ctx.r[10].s64 + 27556;
	// 8268DD10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DD14: 390BA608  addi r8, r11, -0x59f8
	ctx.r[8].s64 = ctx.r[11].s64 + -23032;
	// 8268DD18: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8268DD1C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8268DD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DD24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DD28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268DD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DD30: 386AE238  addi r3, r10, -0x1dc8
	ctx.r[3].s64 = ctx.r[10].s64 + -7624;
	// 8268DD34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268DD38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268DD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DD4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DD54: 4BDD90CD  bl 0x82466e20
	ctx.lr = 0x8268DD58;
	sub_82466E20(ctx, base);
	// 8268DD58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DD68 size=108
    let mut pc: u32 = 0x8268DD68;
    'dispatch: loop {
        match pc {
            0x8268DD68 => {
    //   block [0x8268DD68..0x8268DDD4)
	// 8268DD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DD74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DD78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DD7C: 38EB62B8  addi r7, r11, 0x62b8
	ctx.r[7].s64 = ctx.r[11].s64 + 25272;
	// 8268DD80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268DD84: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8268DD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DD8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DD90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DD98: 386AE268  addi r3, r10, -0x1d98
	ctx.r[3].s64 = ctx.r[10].s64 + -7576;
	// 8268DD9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DDBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DDC0: 4BDD9061  bl 0x82466e20
	ctx.lr = 0x8268DDC4;
	sub_82466E20(ctx, base);
	// 8268DDC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DDD8 size=108
    let mut pc: u32 = 0x8268DDD8;
    'dispatch: loop {
        match pc {
            0x8268DDD8 => {
    //   block [0x8268DDD8..0x8268DE44)
	// 8268DDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DDE4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DDEC: 38EB62D0  addi r7, r11, 0x62d0
	ctx.r[7].s64 = ctx.r[11].s64 + 25296;
	// 8268DDF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268DDF4: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8268DDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DDFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DE00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DE08: 386AE298  addi r3, r10, -0x1d68
	ctx.r[3].s64 = ctx.r[10].s64 + -7528;
	// 8268DE0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DE2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DE30: 4BDD8FF1  bl 0x82466e20
	ctx.lr = 0x8268DE34;
	sub_82466E20(ctx, base);
	// 8268DE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DE48 size=108
    let mut pc: u32 = 0x8268DE48;
    'dispatch: loop {
        match pc {
            0x8268DE48 => {
    //   block [0x8268DE48..0x8268DEB4)
	// 8268DE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DE54: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DE5C: 38EB6318  addi r7, r11, 0x6318
	ctx.r[7].s64 = ctx.r[11].s64 + 25368;
	// 8268DE60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268DE64: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8268DE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DE6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DE70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DE78: 386AE2C8  addi r3, r10, -0x1d38
	ctx.r[3].s64 = ctx.r[10].s64 + -7480;
	// 8268DE7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DE9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DEA0: 4BDD8F81  bl 0x82466e20
	ctx.lr = 0x8268DEA4;
	sub_82466E20(ctx, base);
	// 8268DEA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DEB8 size=108
    let mut pc: u32 = 0x8268DEB8;
    'dispatch: loop {
        match pc {
            0x8268DEB8 => {
    //   block [0x8268DEB8..0x8268DF24)
	// 8268DEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DEC4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DECC: 38EB6348  addi r7, r11, 0x6348
	ctx.r[7].s64 = ctx.r[11].s64 + 25416;
	// 8268DED0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8268DED4: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8268DED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DEDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DEE8: 386AE2F8  addi r3, r10, -0x1d08
	ctx.r[3].s64 = ctx.r[10].s64 + -7432;
	// 8268DEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DF10: 4BDD8F11  bl 0x82466e20
	ctx.lr = 0x8268DF14;
	sub_82466E20(ctx, base);
	// 8268DF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DF28 size=108
    let mut pc: u32 = 0x8268DF28;
    'dispatch: loop {
        match pc {
            0x8268DF28 => {
    //   block [0x8268DF28..0x8268DF94)
	// 8268DF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DF34: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DF3C: 38EB6468  addi r7, r11, 0x6468
	ctx.r[7].s64 = ctx.r[11].s64 + 25704;
	// 8268DF40: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268DF44: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8268DF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DF4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DF50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DF58: 386AE328  addi r3, r10, -0x1cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -7384;
	// 8268DF5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DF7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DF80: 4BDD8EA1  bl 0x82466e20
	ctx.lr = 0x8268DF84;
	sub_82466E20(ctx, base);
	// 8268DF84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268DF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268DF98 size=108
    let mut pc: u32 = 0x8268DF98;
    'dispatch: loop {
        match pc {
            0x8268DF98 => {
    //   block [0x8268DF98..0x8268E004)
	// 8268DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268DFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268DFA4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268DFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268DFAC: 38EB64F8  addi r7, r11, 0x64f8
	ctx.r[7].s64 = ctx.r[11].s64 + 25848;
	// 8268DFB0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268DFB4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8268DFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268DFBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268DFC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268DFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268DFC8: 386AE358  addi r3, r10, -0x1ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -7336;
	// 8268DFCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268DFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268DFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268DFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268DFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268DFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268DFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268DFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268DFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268DFF0: 4BDD8E31  bl 0x82466e20
	ctx.lr = 0x8268DFF4;
	sub_82466E20(ctx, base);
	// 8268DFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268DFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268DFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E008 size=108
    let mut pc: u32 = 0x8268E008;
    'dispatch: loop {
        match pc {
            0x8268E008 => {
    //   block [0x8268E008..0x8268E074)
	// 8268E008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E014: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E01C: 38EB65B8  addi r7, r11, 0x65b8
	ctx.r[7].s64 = ctx.r[11].s64 + 26040;
	// 8268E020: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8268E024: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8268E028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E02C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E038: 386AE388  addi r3, r10, -0x1c78
	ctx.r[3].s64 = ctx.r[10].s64 + -7288;
	// 8268E03C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E060: 4BDD8DC1  bl 0x82466e20
	ctx.lr = 0x8268E064;
	sub_82466E20(ctx, base);
	// 8268E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E078 size=108
    let mut pc: u32 = 0x8268E078;
    'dispatch: loop {
        match pc {
            0x8268E078 => {
    //   block [0x8268E078..0x8268E0E4)
	// 8268E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E084: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E08C: 38EB6690  addi r7, r11, 0x6690
	ctx.r[7].s64 = ctx.r[11].s64 + 26256;
	// 8268E090: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268E094: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8268E098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E09C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E0A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E0A8: 386AE3B8  addi r3, r10, -0x1c48
	ctx.r[3].s64 = ctx.r[10].s64 + -7240;
	// 8268E0AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E0CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E0D0: 4BDD8D51  bl 0x82466e20
	ctx.lr = 0x8268E0D4;
	sub_82466E20(ctx, base);
	// 8268E0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E0E8 size=108
    let mut pc: u32 = 0x8268E0E8;
    'dispatch: loop {
        match pc {
            0x8268E0E8 => {
    //   block [0x8268E0E8..0x8268E154)
	// 8268E0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E0F4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E0FC: 38EB6750  addi r7, r11, 0x6750
	ctx.r[7].s64 = ctx.r[11].s64 + 26448;
	// 8268E100: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8268E104: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8268E108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E10C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E118: 386AE3E8  addi r3, r10, -0x1c18
	ctx.r[3].s64 = ctx.r[10].s64 + -7192;
	// 8268E11C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E140: 4BDD8CE1  bl 0x82466e20
	ctx.lr = 0x8268E144;
	sub_82466E20(ctx, base);
	// 8268E144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E158 size=112
    let mut pc: u32 = 0x8268E158;
    'dispatch: loop {
        match pc {
            0x8268E158 => {
    //   block [0x8268E158..0x8268E1C8)
	// 8268E158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E164: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8268E168: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8268E16C: 38EA67F8  addi r7, r10, 0x67f8
	ctx.r[7].s64 = ctx.r[10].s64 + 26616;
	// 8268E170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E174: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8268E178: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8268E17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E180: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E184: 396B6BB8  addi r11, r11, 0x6bb8
	ctx.r[11].s64 = ctx.r[11].s64 + 27576;
	// 8268E188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E18C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E190: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E194: 386AE418  addi r3, r10, -0x1be8
	ctx.r[3].s64 = ctx.r[10].s64 + -7144;
	// 8268E198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E19C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8268E1A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E1A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8268E1A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E1AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E1B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E1B4: 4BDD8C6D  bl 0x82466e20
	ctx.lr = 0x8268E1B8;
	sub_82466E20(ctx, base);
	// 8268E1B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E1C8 size=108
    let mut pc: u32 = 0x8268E1C8;
    'dispatch: loop {
        match pc {
            0x8268E1C8 => {
    //   block [0x8268E1C8..0x8268E234)
	// 8268E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E1D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E1D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E1DC: 38EB6918  addi r7, r11, 0x6918
	ctx.r[7].s64 = ctx.r[11].s64 + 26904;
	// 8268E1E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268E1E4: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8268E1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E1EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E1F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E1F8: 386AE448  addi r3, r10, -0x1bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -7096;
	// 8268E1FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E220: 4BDD8C01  bl 0x82466e20
	ctx.lr = 0x8268E224;
	sub_82466E20(ctx, base);
	// 8268E224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E238 size=108
    let mut pc: u32 = 0x8268E238;
    'dispatch: loop {
        match pc {
            0x8268E238 => {
    //   block [0x8268E238..0x8268E2A4)
	// 8268E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E244: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E24C: 38EB6978  addi r7, r11, 0x6978
	ctx.r[7].s64 = ctx.r[11].s64 + 27000;
	// 8268E250: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8268E254: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8268E258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E25C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E268: 386AE478  addi r3, r10, -0x1b88
	ctx.r[3].s64 = ctx.r[10].s64 + -7048;
	// 8268E26C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E28C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E290: 4BDD8B91  bl 0x82466e20
	ctx.lr = 0x8268E294;
	sub_82466E20(ctx, base);
	// 8268E294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E2A8 size=108
    let mut pc: u32 = 0x8268E2A8;
    'dispatch: loop {
        match pc {
            0x8268E2A8 => {
    //   block [0x8268E2A8..0x8268E314)
	// 8268E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E2B4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E2BC: 38EB6A80  addi r7, r11, 0x6a80
	ctx.r[7].s64 = ctx.r[11].s64 + 27264;
	// 8268E2C0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8268E2C4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8268E2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E2CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E2D8: 386AE4A8  addi r3, r10, -0x1b58
	ctx.r[3].s64 = ctx.r[10].s64 + -7000;
	// 8268E2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E300: 4BDD8B21  bl 0x82466e20
	ctx.lr = 0x8268E304;
	sub_82466E20(ctx, base);
	// 8268E304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E318 size=108
    let mut pc: u32 = 0x8268E318;
    'dispatch: loop {
        match pc {
            0x8268E318 => {
    //   block [0x8268E318..0x8268E384)
	// 8268E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E324: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E32C: 38EB6B58  addi r7, r11, 0x6b58
	ctx.r[7].s64 = ctx.r[11].s64 + 27480;
	// 8268E330: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268E334: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8268E338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E33C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E348: 386AE4D8  addi r3, r10, -0x1b28
	ctx.r[3].s64 = ctx.r[10].s64 + -6952;
	// 8268E34C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E36C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E370: 4BDD8AB1  bl 0x82466e20
	ctx.lr = 0x8268E374;
	sub_82466E20(ctx, base);
	// 8268E374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E388 size=108
    let mut pc: u32 = 0x8268E388;
    'dispatch: loop {
        match pc {
            0x8268E388 => {
    //   block [0x8268E388..0x8268E3F4)
	// 8268E388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E394: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E39C: 38EB6B88  addi r7, r11, 0x6b88
	ctx.r[7].s64 = ctx.r[11].s64 + 27528;
	// 8268E3A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E3A4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8268E3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E3AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E3B8: 386AE508  addi r3, r10, -0x1af8
	ctx.r[3].s64 = ctx.r[10].s64 + -6904;
	// 8268E3BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E3E0: 4BDD8A41  bl 0x82466e20
	ctx.lr = 0x8268E3E4;
	sub_82466E20(ctx, base);
	// 8268E3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E3F8 size=108
    let mut pc: u32 = 0x8268E3F8;
    'dispatch: loop {
        match pc {
            0x8268E3F8 => {
    //   block [0x8268E3F8..0x8268E464)
	// 8268E3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E404: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E40C: 38EB6BA0  addi r7, r11, 0x6ba0
	ctx.r[7].s64 = ctx.r[11].s64 + 27552;
	// 8268E410: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268E414: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8268E418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E41C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E428: 386AE538  addi r3, r10, -0x1ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -6856;
	// 8268E42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E450: 4BDD89D1  bl 0x82466e20
	ctx.lr = 0x8268E454;
	sub_82466E20(ctx, base);
	// 8268E454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E468 size=108
    let mut pc: u32 = 0x8268E468;
    'dispatch: loop {
        match pc {
            0x8268E468 => {
    //   block [0x8268E468..0x8268E4D4)
	// 8268E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E474: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E47C: 38EB6BE8  addi r7, r11, 0x6be8
	ctx.r[7].s64 = ctx.r[11].s64 + 27624;
	// 8268E480: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E484: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8268E488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E48C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E498: 386AE568  addi r3, r10, -0x1a98
	ctx.r[3].s64 = ctx.r[10].s64 + -6808;
	// 8268E49C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E4C0: 4BDD8961  bl 0x82466e20
	ctx.lr = 0x8268E4C4;
	sub_82466E20(ctx, base);
	// 8268E4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E4D8 size=112
    let mut pc: u32 = 0x8268E4D8;
    'dispatch: loop {
        match pc {
            0x8268E4D8 => {
    //   block [0x8268E4D8..0x8268E548)
	// 8268E4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E4E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E4E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E4EC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268E4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E4F4: 390B6C00  addi r8, r11, 0x6c00
	ctx.r[8].s64 = ctx.r[11].s64 + 27648;
	// 8268E4F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268E4FC: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8268E500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E504: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E510: 386AE598  addi r3, r10, -0x1a68
	ctx.r[3].s64 = ctx.r[10].s64 + -6760;
	// 8268E514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268E518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E534: 4BDD88ED  bl 0x82466e20
	ctx.lr = 0x8268E538;
	sub_82466E20(ctx, base);
	// 8268E538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E548 size=108
    let mut pc: u32 = 0x8268E548;
    'dispatch: loop {
        match pc {
            0x8268E548 => {
    //   block [0x8268E548..0x8268E5B4)
	// 8268E548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E554: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E55C: 38EB6C48  addi r7, r11, 0x6c48
	ctx.r[7].s64 = ctx.r[11].s64 + 27720;
	// 8268E560: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268E564: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8268E568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E56C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268E574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E578: 386AE5C8  addi r3, r10, -0x1a38
	ctx.r[3].s64 = ctx.r[10].s64 + -6712;
	// 8268E57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268E580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E5A0: 4BDD8881  bl 0x82466e20
	ctx.lr = 0x8268E5A4;
	sub_82466E20(ctx, base);
	// 8268E5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E5B8 size=112
    let mut pc: u32 = 0x8268E5B8;
    'dispatch: loop {
        match pc {
            0x8268E5B8 => {
    //   block [0x8268E5B8..0x8268E628)
	// 8268E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E5C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E5C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E5CC: 38AAE5C8  addi r5, r10, -0x1a38
	ctx.r[5].s64 = ctx.r[10].s64 + -6712;
	// 8268E5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E5D4: 390B6CA8  addi r8, r11, 0x6ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 27816;
	// 8268E5D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268E5DC: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8268E5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E5E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E5F0: 386AE5F8  addi r3, r10, -0x1a08
	ctx.r[3].s64 = ctx.r[10].s64 + -6664;
	// 8268E5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268E5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E614: 4BDD880D  bl 0x82466e20
	ctx.lr = 0x8268E618;
	sub_82466E20(ctx, base);
	// 8268E618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E628 size=96
    let mut pc: u32 = 0x8268E628;
    'dispatch: loop {
        match pc {
            0x8268E628 => {
    //   block [0x8268E628..0x8268E688)
	// 8268E628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E634: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E63C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8268E640: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E648: 386AE628  addi r3, r10, -0x19d8
	ctx.r[3].s64 = ctx.r[10].s64 + -6616;
	// 8268E64C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E654: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E668: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E670: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E674: 4BDD87AD  bl 0x82466e20
	ctx.lr = 0x8268E678;
	sub_82466E20(ctx, base);
	// 8268E678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E688 size=112
    let mut pc: u32 = 0x8268E688;
    'dispatch: loop {
        match pc {
            0x8268E688 => {
    //   block [0x8268E688..0x8268E6F8)
	// 8268E688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E694: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E698: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E69C: 38AAFE28  addi r5, r10, -0x1d8
	ctx.r[5].s64 = ctx.r[10].s64 + -472;
	// 8268E6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E6A4: 390B6CF0  addi r8, r11, 0x6cf0
	ctx.r[8].s64 = ctx.r[11].s64 + 27888;
	// 8268E6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268E6AC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8268E6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E6B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E6C0: 386AE658  addi r3, r10, -0x19a8
	ctx.r[3].s64 = ctx.r[10].s64 + -6568;
	// 8268E6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268E6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E6E4: 4BDD873D  bl 0x82466e20
	ctx.lr = 0x8268E6E8;
	sub_82466E20(ctx, base);
	// 8268E6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E6F8 size=96
    let mut pc: u32 = 0x8268E6F8;
    'dispatch: loop {
        match pc {
            0x8268E6F8 => {
    //   block [0x8268E6F8..0x8268E758)
	// 8268E6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E704: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E70C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8268E710: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E718: 386AE688  addi r3, r10, -0x1978
	ctx.r[3].s64 = ctx.r[10].s64 + -6520;
	// 8268E71C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E724: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E744: 4BDD86DD  bl 0x82466e20
	ctx.lr = 0x8268E748;
	sub_82466E20(ctx, base);
	// 8268E748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E758 size=100
    let mut pc: u32 = 0x8268E758;
    'dispatch: loop {
        match pc {
            0x8268E758 => {
    //   block [0x8268E758..0x8268E7BC)
	// 8268E758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E76C: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268E770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E778: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8268E77C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E784: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268E788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E78C: 386AE6B8  addi r3, r10, -0x1948
	ctx.r[3].s64 = ctx.r[10].s64 + -6472;
	// 8268E790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E794: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E7A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E7A8: 4BDD8679  bl 0x82466e20
	ctx.lr = 0x8268E7AC;
	sub_82466E20(ctx, base);
	// 8268E7AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E7C0 size=96
    let mut pc: u32 = 0x8268E7C0;
    'dispatch: loop {
        match pc {
            0x8268E7C0 => {
    //   block [0x8268E7C0..0x8268E820)
	// 8268E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E7CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E7D4: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8268E7D8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E7E0: 386AE6E8  addi r3, r10, -0x1918
	ctx.r[3].s64 = ctx.r[10].s64 + -6424;
	// 8268E7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E7EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E80C: 4BDD8615  bl 0x82466e20
	ctx.lr = 0x8268E810;
	sub_82466E20(ctx, base);
	// 8268E810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E820 size=112
    let mut pc: u32 = 0x8268E820;
    'dispatch: loop {
        match pc {
            0x8268E820 => {
    //   block [0x8268E820..0x8268E890)
	// 8268E820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E82C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E830: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E834: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 8268E838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E83C: 390B6D50  addi r8, r11, 0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + 27984;
	// 8268E840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268E844: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8268E848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E858: 386AE718  addi r3, r10, -0x18e8
	ctx.r[3].s64 = ctx.r[10].s64 + -6376;
	// 8268E85C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268E860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E87C: 4BDD85A5  bl 0x82466e20
	ctx.lr = 0x8268E880;
	sub_82466E20(ctx, base);
	// 8268E880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E890 size=112
    let mut pc: u32 = 0x8268E890;
    'dispatch: loop {
        match pc {
            0x8268E890 => {
    //   block [0x8268E890..0x8268E900)
	// 8268E890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E89C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E8A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E8A4: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 8268E8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E8AC: 390B6D80  addi r8, r11, 0x6d80
	ctx.r[8].s64 = ctx.r[11].s64 + 28032;
	// 8268E8B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268E8B4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8268E8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E8C8: 386AE748  addi r3, r10, -0x18b8
	ctx.r[3].s64 = ctx.r[10].s64 + -6328;
	// 8268E8CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268E8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E8EC: 4BDD8535  bl 0x82466e20
	ctx.lr = 0x8268E8F0;
	sub_82466E20(ctx, base);
	// 8268E8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E900 size=100
    let mut pc: u32 = 0x8268E900;
    'dispatch: loop {
        match pc {
            0x8268E900 => {
    //   block [0x8268E900..0x8268E964)
	// 8268E900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E90C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E914: 38AAE6B8  addi r5, r10, -0x1948
	ctx.r[5].s64 = ctx.r[10].s64 + -6472;
	// 8268E918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E920: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8268E924: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268E930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E934: 386AE778  addi r3, r10, -0x1888
	ctx.r[3].s64 = ctx.r[10].s64 + -6280;
	// 8268E938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E93C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E940: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E948: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E950: 4BDD84D1  bl 0x82466e20
	ctx.lr = 0x8268E954;
	sub_82466E20(ctx, base);
	// 8268E954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E968 size=96
    let mut pc: u32 = 0x8268E968;
    'dispatch: loop {
        match pc {
            0x8268E968 => {
    //   block [0x8268E968..0x8268E9C8)
	// 8268E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E974: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E97C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8268E980: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268E988: 386AE7A8  addi r3, r10, -0x1858
	ctx.r[3].s64 = ctx.r[10].s64 + -6232;
	// 8268E98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268E994: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268E99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268E9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268E9A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268E9AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268E9B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268E9B4: 4BDD846D  bl 0x82466e20
	ctx.lr = 0x8268E9B8;
	sub_82466E20(ctx, base);
	// 8268E9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268E9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268E9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268E9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268E9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268E9C8 size=112
    let mut pc: u32 = 0x8268E9C8;
    'dispatch: loop {
        match pc {
            0x8268E9C8 => {
    //   block [0x8268E9C8..0x8268EA38)
	// 8268E9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268E9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268E9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268E9D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E9D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268E9DC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268E9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268E9E4: 390B6D98  addi r8, r11, 0x6d98
	ctx.r[8].s64 = ctx.r[11].s64 + 28056;
	// 8268E9E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268E9EC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8268E9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268E9F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268E9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268E9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EA00: 386AE7D8  addi r3, r10, -0x1828
	ctx.r[3].s64 = ctx.r[10].s64 + -6184;
	// 8268EA04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EA08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EA24: 4BDD83FD  bl 0x82466e20
	ctx.lr = 0x8268EA28;
	sub_82466E20(ctx, base);
	// 8268EA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EA38 size=108
    let mut pc: u32 = 0x8268EA38;
    'dispatch: loop {
        match pc {
            0x8268EA38 => {
    //   block [0x8268EA38..0x8268EAA4)
	// 8268EA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EA44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EA4C: 38EB6DB0  addi r7, r11, 0x6db0
	ctx.r[7].s64 = ctx.r[11].s64 + 28080;
	// 8268EA50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268EA54: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8268EA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268EA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EA68: 386AE808  addi r3, r10, -0x17f8
	ctx.r[3].s64 = ctx.r[10].s64 + -6136;
	// 8268EA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268EA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268EA90: 4BDD8391  bl 0x82466e20
	ctx.lr = 0x8268EA94;
	sub_82466E20(ctx, base);
	// 8268EA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EAA8 size=112
    let mut pc: u32 = 0x8268EAA8;
    'dispatch: loop {
        match pc {
            0x8268EAA8 => {
    //   block [0x8268EAA8..0x8268EB18)
	// 8268EAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EAB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EAB8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EABC: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268EAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EAC4: 390B6E10  addi r8, r11, 0x6e10
	ctx.r[8].s64 = ctx.r[11].s64 + 28176;
	// 8268EAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268EACC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8268EAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EAE0: 386AE838  addi r3, r10, -0x17c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6088;
	// 8268EAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EB04: 4BDD831D  bl 0x82466e20
	ctx.lr = 0x8268EB08;
	sub_82466E20(ctx, base);
	// 8268EB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EB18 size=112
    let mut pc: u32 = 0x8268EB18;
    'dispatch: loop {
        match pc {
            0x8268EB18 => {
    //   block [0x8268EB18..0x8268EB88)
	// 8268EB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EB24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EB28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EB2C: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268EB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EB34: 390B6E28  addi r8, r11, 0x6e28
	ctx.r[8].s64 = ctx.r[11].s64 + 28200;
	// 8268EB38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268EB3C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8268EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EB50: 386AE868  addi r3, r10, -0x1798
	ctx.r[3].s64 = ctx.r[10].s64 + -6040;
	// 8268EB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EB74: 4BDD82AD  bl 0x82466e20
	ctx.lr = 0x8268EB78;
	sub_82466E20(ctx, base);
	// 8268EB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EB88 size=112
    let mut pc: u32 = 0x8268EB88;
    'dispatch: loop {
        match pc {
            0x8268EB88 => {
    //   block [0x8268EB88..0x8268EBF8)
	// 8268EB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EB98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EB9C: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268EBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EBA4: 390B6E58  addi r8, r11, 0x6e58
	ctx.r[8].s64 = ctx.r[11].s64 + 28248;
	// 8268EBA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268EBAC: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8268EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EBC0: 386AE898  addi r3, r10, -0x1768
	ctx.r[3].s64 = ctx.r[10].s64 + -5992;
	// 8268EBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EBE4: 4BDD823D  bl 0x82466e20
	ctx.lr = 0x8268EBE8;
	sub_82466E20(ctx, base);
	// 8268EBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EBF8 size=112
    let mut pc: u32 = 0x8268EBF8;
    'dispatch: loop {
        match pc {
            0x8268EBF8 => {
    //   block [0x8268EBF8..0x8268EC68)
	// 8268EBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EC04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EC08: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EC0C: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268EC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EC14: 390B6E70  addi r8, r11, 0x6e70
	ctx.r[8].s64 = ctx.r[11].s64 + 28272;
	// 8268EC18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268EC1C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8268EC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EC30: 386AE8C8  addi r3, r10, -0x1738
	ctx.r[3].s64 = ctx.r[10].s64 + -5944;
	// 8268EC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EC54: 4BDD81CD  bl 0x82466e20
	ctx.lr = 0x8268EC58;
	sub_82466E20(ctx, base);
	// 8268EC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EC68 size=112
    let mut pc: u32 = 0x8268EC68;
    'dispatch: loop {
        match pc {
            0x8268EC68 => {
    //   block [0x8268EC68..0x8268ECD8)
	// 8268EC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EC74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EC78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EC7C: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268EC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EC84: 390B6EA0  addi r8, r11, 0x6ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 28320;
	// 8268EC88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268EC8C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8268EC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EC94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268ECA0: 386AE8F8  addi r3, r10, -0x1708
	ctx.r[3].s64 = ctx.r[10].s64 + -5896;
	// 8268ECA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268ECA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268ECAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ECB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268ECB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ECBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268ECC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268ECC4: 4BDD815D  bl 0x82466e20
	ctx.lr = 0x8268ECC8;
	sub_82466E20(ctx, base);
	// 8268ECC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268ECD8 size=112
    let mut pc: u32 = 0x8268ECD8;
    'dispatch: loop {
        match pc {
            0x8268ECD8 => {
    //   block [0x8268ECD8..0x8268ED48)
	// 8268ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268ECE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268ECE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ECE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268ECEC: 38AAEDD8  addi r5, r10, -0x1228
	ctx.r[5].s64 = ctx.r[10].s64 + -4648;
	// 8268ECF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268ECF4: 390B6EB8  addi r8, r11, 0x6eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 28344;
	// 8268ECF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268ECFC: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8268ED00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268ED04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ED08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268ED0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268ED10: 386AE928  addi r3, r10, -0x16d8
	ctx.r[3].s64 = ctx.r[10].s64 + -5848;
	// 8268ED14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268ED18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268ED1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ED20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ED24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268ED28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ED2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268ED30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268ED34: 4BDD80ED  bl 0x82466e20
	ctx.lr = 0x8268ED38;
	sub_82466E20(ctx, base);
	// 8268ED38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268ED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268ED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268ED48 size=112
    let mut pc: u32 = 0x8268ED48;
    'dispatch: loop {
        match pc {
            0x8268ED48 => {
    //   block [0x8268ED48..0x8268EDB8)
	// 8268ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268ED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268ED54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ED58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268ED5C: 38AAEB38  addi r5, r10, -0x14c8
	ctx.r[5].s64 = ctx.r[10].s64 + -5320;
	// 8268ED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268ED64: 390B6ED0  addi r8, r11, 0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 28368;
	// 8268ED68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268ED6C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8268ED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268ED74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268ED78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268ED7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268ED80: 386AE958  addi r3, r10, -0x16a8
	ctx.r[3].s64 = ctx.r[10].s64 + -5800;
	// 8268ED84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268ED88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268ED8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268ED90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268ED94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268ED98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268ED9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EDA4: 4BDD807D  bl 0x82466e20
	ctx.lr = 0x8268EDA8;
	sub_82466E20(ctx, base);
	// 8268EDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EDB8 size=112
    let mut pc: u32 = 0x8268EDB8;
    'dispatch: loop {
        match pc {
            0x8268EDB8 => {
    //   block [0x8268EDB8..0x8268EE28)
	// 8268EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EDC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EDC8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EDCC: 38AAE8F8  addi r5, r10, -0x1708
	ctx.r[5].s64 = ctx.r[10].s64 + -5896;
	// 8268EDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EDD4: 390B6EE8  addi r8, r11, 0x6ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 28392;
	// 8268EDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268EDDC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8268EDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EDE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EDF0: 386AE988  addi r3, r10, -0x1678
	ctx.r[3].s64 = ctx.r[10].s64 + -5752;
	// 8268EDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EE14: 4BDD800D  bl 0x82466e20
	ctx.lr = 0x8268EE18;
	sub_82466E20(ctx, base);
	// 8268EE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EE28 size=112
    let mut pc: u32 = 0x8268EE28;
    'dispatch: loop {
        match pc {
            0x8268EE28 => {
    //   block [0x8268EE28..0x8268EE98)
	// 8268EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EE34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EE38: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EE3C: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268EE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EE44: 390B6F30  addi r8, r11, 0x6f30
	ctx.r[8].s64 = ctx.r[11].s64 + 28464;
	// 8268EE48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268EE4C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8268EE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EE54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EE60: 386AE9B8  addi r3, r10, -0x1648
	ctx.r[3].s64 = ctx.r[10].s64 + -5704;
	// 8268EE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EE84: 4BDD7F9D  bl 0x82466e20
	ctx.lr = 0x8268EE88;
	sub_82466E20(ctx, base);
	// 8268EE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EE98 size=112
    let mut pc: u32 = 0x8268EE98;
    'dispatch: loop {
        match pc {
            0x8268EE98 => {
    //   block [0x8268EE98..0x8268EF08)
	// 8268EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EEA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EEA8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EEAC: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268EEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EEB4: 390B6F60  addi r8, r11, 0x6f60
	ctx.r[8].s64 = ctx.r[11].s64 + 28512;
	// 8268EEB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268EEBC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8268EEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EEC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EED0: 386AE9E8  addi r3, r10, -0x1618
	ctx.r[3].s64 = ctx.r[10].s64 + -5656;
	// 8268EED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EEF4: 4BDD7F2D  bl 0x82466e20
	ctx.lr = 0x8268EEF8;
	sub_82466E20(ctx, base);
	// 8268EEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EF08 size=108
    let mut pc: u32 = 0x8268EF08;
    'dispatch: loop {
        match pc {
            0x8268EF08 => {
    //   block [0x8268EF08..0x8268EF74)
	// 8268EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EF14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EF1C: 38EB6F90  addi r7, r11, 0x6f90
	ctx.r[7].s64 = ctx.r[11].s64 + 28560;
	// 8268EF20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268EF24: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8268EF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EF2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268EF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EF38: 386AEA18  addi r3, r10, -0x15e8
	ctx.r[3].s64 = ctx.r[10].s64 + -5608;
	// 8268EF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268EF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268EF60: 4BDD7EC1  bl 0x82466e20
	ctx.lr = 0x8268EF64;
	sub_82466E20(ctx, base);
	// 8268EF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EF78 size=112
    let mut pc: u32 = 0x8268EF78;
    'dispatch: loop {
        match pc {
            0x8268EF78 => {
    //   block [0x8268EF78..0x8268EFE8)
	// 8268EF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EF84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EF88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EF8C: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268EF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268EF94: 390B6FD8  addi r8, r11, 0x6fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 28632;
	// 8268EF98: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8268EF9C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8268EFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268EFA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268EFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268EFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268EFB0: 386AEA48  addi r3, r10, -0x15b8
	ctx.r[3].s64 = ctx.r[10].s64 + -5560;
	// 8268EFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268EFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268EFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268EFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268EFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268EFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268EFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268EFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268EFD4: 4BDD7E4D  bl 0x82466e20
	ctx.lr = 0x8268EFD8;
	sub_82466E20(ctx, base);
	// 8268EFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268EFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268EFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268EFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268EFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268EFE8 size=116
    let mut pc: u32 = 0x8268EFE8;
    'dispatch: loop {
        match pc {
            0x8268EFE8 => {
    //   block [0x8268EFE8..0x8268F05C)
	// 8268EFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268EFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268EFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268EFF4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268EFF8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268EFFC: 390B7058  addi r8, r11, 0x7058
	ctx.r[8].s64 = ctx.r[11].s64 + 28760;
	// 8268F000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F004: 392A6C40  addi r9, r10, 0x6c40
	ctx.r[9].s64 = ctx.r[10].s64 + 27712;
	// 8268F008: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F00C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8268F010: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268F014: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F01C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F02C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268F030: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8268F034: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268F038: 386BEA78  addi r3, r11, -0x1588
	ctx.r[3].s64 = ctx.r[11].s64 + -5512;
	// 8268F03C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268F040: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F048: 4BDD7DD9  bl 0x82466e20
	ctx.lr = 0x8268F04C;
	sub_82466E20(ctx, base);
	// 8268F04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F060 size=100
    let mut pc: u32 = 0x8268F060;
    'dispatch: loop {
        match pc {
            0x8268F060 => {
    //   block [0x8268F060..0x8268F0C4)
	// 8268F060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F06C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F074: 38AAEBC8  addi r5, r10, -0x1438
	ctx.r[5].s64 = ctx.r[10].s64 + -5176;
	// 8268F078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F080: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8268F084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F094: 386AEAA8  addi r3, r10, -0x1558
	ctx.r[3].s64 = ctx.r[10].s64 + -5464;
	// 8268F098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268F0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268F0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F0B0: 4BDD7D71  bl 0x82466e20
	ctx.lr = 0x8268F0B4;
	sub_82466E20(ctx, base);
	// 8268F0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F0C8 size=100
    let mut pc: u32 = 0x8268F0C8;
    'dispatch: loop {
        match pc {
            0x8268F0C8 => {
    //   block [0x8268F0C8..0x8268F12C)
	// 8268F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F0DC: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268F0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F0E8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8268F0EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F0FC: 386AEAD8  addi r3, r10, -0x1528
	ctx.r[3].s64 = ctx.r[10].s64 + -5416;
	// 8268F100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F108: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268F10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F110: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268F114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F118: 4BDD7D09  bl 0x82466e20
	ctx.lr = 0x8268F11C;
	sub_82466E20(ctx, base);
	// 8268F11C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F130 size=108
    let mut pc: u32 = 0x8268F130;
    'dispatch: loop {
        match pc {
            0x8268F130 => {
    //   block [0x8268F130..0x8268F19C)
	// 8268F130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F13C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F144: 38EB70D0  addi r7, r11, 0x70d0
	ctx.r[7].s64 = ctx.r[11].s64 + 28880;
	// 8268F148: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268F14C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8268F150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F154: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F160: 386AEB08  addi r3, r10, -0x14f8
	ctx.r[3].s64 = ctx.r[10].s64 + -5368;
	// 8268F164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F188: 4BDD7C99  bl 0x82466e20
	ctx.lr = 0x8268F18C;
	sub_82466E20(ctx, base);
	// 8268F18C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F1A0 size=112
    let mut pc: u32 = 0x8268F1A0;
    'dispatch: loop {
        match pc {
            0x8268F1A0 => {
    //   block [0x8268F1A0..0x8268F210)
	// 8268F1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F1AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F1B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F1B4: 38AAE8F8  addi r5, r10, -0x1708
	ctx.r[5].s64 = ctx.r[10].s64 + -5896;
	// 8268F1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F1BC: 390B7100  addi r8, r11, 0x7100
	ctx.r[8].s64 = ctx.r[11].s64 + 28928;
	// 8268F1C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268F1C4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8268F1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F1CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F1D8: 386AEB38  addi r3, r10, -0x14c8
	ctx.r[3].s64 = ctx.r[10].s64 + -5320;
	// 8268F1DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F1FC: 4BDD7C25  bl 0x82466e20
	ctx.lr = 0x8268F200;
	sub_82466E20(ctx, base);
	// 8268F200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F210 size=108
    let mut pc: u32 = 0x8268F210;
    'dispatch: loop {
        match pc {
            0x8268F210 => {
    //   block [0x8268F210..0x8268F27C)
	// 8268F210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F21C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F224: 38EB7118  addi r7, r11, 0x7118
	ctx.r[7].s64 = ctx.r[11].s64 + 28952;
	// 8268F228: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268F22C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8268F230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F234: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F240: 386AEB68  addi r3, r10, -0x1498
	ctx.r[3].s64 = ctx.r[10].s64 + -5272;
	// 8268F244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F268: 4BDD7BB9  bl 0x82466e20
	ctx.lr = 0x8268F26C;
	sub_82466E20(ctx, base);
	// 8268F26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268F280 size=28
    let mut pc: u32 = 0x8268F280;
    'dispatch: loop {
        match pc {
            0x8268F280 => {
    //   block [0x8268F280..0x8268F29C)
	// 8268F280: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F284: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268F288: 394AA680  addi r10, r10, -0x5980
	ctx.r[10].s64 = ctx.r[10].s64 + -22912;
	// 8268F28C: 816B7054  lwz r11, 0x7054(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28756 as u32) ) } as u64;
	// 8268F290: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8268F294: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8268F298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F2A0 size=108
    let mut pc: u32 = 0x8268F2A0;
    'dispatch: loop {
        match pc {
            0x8268F2A0 => {
    //   block [0x8268F2A0..0x8268F30C)
	// 8268F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F2AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268F2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F2B4: 38EBA680  addi r7, r11, -0x5980
	ctx.r[7].s64 = ctx.r[11].s64 + -22912;
	// 8268F2B8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8268F2BC: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8268F2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F2D0: 386AEB98  addi r3, r10, -0x1468
	ctx.r[3].s64 = ctx.r[10].s64 + -5224;
	// 8268F2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F2F8: 4BDD7B29  bl 0x82466e20
	ctx.lr = 0x8268F2FC;
	sub_82466E20(ctx, base);
	// 8268F2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F310 size=116
    let mut pc: u32 = 0x8268F310;
    'dispatch: loop {
        match pc {
            0x8268F310 => {
    //   block [0x8268F310..0x8268F384)
	// 8268F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F31C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F320: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268F324: 390B7138  addi r8, r11, 0x7138
	ctx.r[8].s64 = ctx.r[11].s64 + 28984;
	// 8268F328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F32C: 392A6C94  addi r9, r10, 0x6c94
	ctx.r[9].s64 = ctx.r[10].s64 + 27796;
	// 8268F330: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F334: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8268F338: 38AAE8F8  addi r5, r10, -0x1708
	ctx.r[5].s64 = ctx.r[10].s64 + -5896;
	// 8268F33C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F344: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F354: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268F358: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8268F35C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268F360: 386BEBC8  addi r3, r11, -0x1438
	ctx.r[3].s64 = ctx.r[11].s64 + -5176;
	// 8268F364: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8268F368: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F370: 4BDD7AB1  bl 0x82466e20
	ctx.lr = 0x8268F374;
	sub_82466E20(ctx, base);
	// 8268F374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F388 size=112
    let mut pc: u32 = 0x8268F388;
    'dispatch: loop {
        match pc {
            0x8268F388 => {
    //   block [0x8268F388..0x8268F3F8)
	// 8268F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F394: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F398: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F39C: 38AAE898  addi r5, r10, -0x1768
	ctx.r[5].s64 = ctx.r[10].s64 + -5992;
	// 8268F3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F3A4: 390B7198  addi r8, r11, 0x7198
	ctx.r[8].s64 = ctx.r[11].s64 + 29080;
	// 8268F3A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268F3AC: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8268F3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F3B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F3C0: 386AEBF8  addi r3, r10, -0x1408
	ctx.r[3].s64 = ctx.r[10].s64 + -5128;
	// 8268F3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F3E4: 4BDD7A3D  bl 0x82466e20
	ctx.lr = 0x8268F3E8;
	sub_82466E20(ctx, base);
	// 8268F3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F3F8 size=108
    let mut pc: u32 = 0x8268F3F8;
    'dispatch: loop {
        match pc {
            0x8268F3F8 => {
    //   block [0x8268F3F8..0x8268F464)
	// 8268F3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F404: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F40C: 38EB71B0  addi r7, r11, 0x71b0
	ctx.r[7].s64 = ctx.r[11].s64 + 29104;
	// 8268F410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268F414: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8268F418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F41C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F428: 386AEC28  addi r3, r10, -0x13d8
	ctx.r[3].s64 = ctx.r[10].s64 + -5080;
	// 8268F42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F450: 4BDD79D1  bl 0x82466e20
	ctx.lr = 0x8268F454;
	sub_82466E20(ctx, base);
	// 8268F454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F468 size=112
    let mut pc: u32 = 0x8268F468;
    'dispatch: loop {
        match pc {
            0x8268F468 => {
    //   block [0x8268F468..0x8268F4D8)
	// 8268F468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F474: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F478: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F47C: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268F480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F484: 390B71E0  addi r8, r11, 0x71e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29152;
	// 8268F488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268F48C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8268F490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F494: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F4A0: 386AEC58  addi r3, r10, -0x13a8
	ctx.r[3].s64 = ctx.r[10].s64 + -5032;
	// 8268F4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F4C4: 4BDD795D  bl 0x82466e20
	ctx.lr = 0x8268F4C8;
	sub_82466E20(ctx, base);
	// 8268F4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F4D8 size=112
    let mut pc: u32 = 0x8268F4D8;
    'dispatch: loop {
        match pc {
            0x8268F4D8 => {
    //   block [0x8268F4D8..0x8268F548)
	// 8268F4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F4E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F4E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F4EC: 38AAEDD8  addi r5, r10, -0x1228
	ctx.r[5].s64 = ctx.r[10].s64 + -4648;
	// 8268F4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F4F4: 390B7210  addi r8, r11, 0x7210
	ctx.r[8].s64 = ctx.r[11].s64 + 29200;
	// 8268F4F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268F4FC: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8268F500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F504: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F510: 386AEC88  addi r3, r10, -0x1378
	ctx.r[3].s64 = ctx.r[10].s64 + -4984;
	// 8268F514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F534: 4BDD78ED  bl 0x82466e20
	ctx.lr = 0x8268F538;
	sub_82466E20(ctx, base);
	// 8268F538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F548 size=100
    let mut pc: u32 = 0x8268F548;
    'dispatch: loop {
        match pc {
            0x8268F548 => {
    //   block [0x8268F548..0x8268F5AC)
	// 8268F548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F55C: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268F560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F568: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8268F56C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F57C: 386AECB8  addi r3, r10, -0x1348
	ctx.r[3].s64 = ctx.r[10].s64 + -4936;
	// 8268F580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268F58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268F594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F598: 4BDD7889  bl 0x82466e20
	ctx.lr = 0x8268F59C;
	sub_82466E20(ctx, base);
	// 8268F59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F5B0 size=112
    let mut pc: u32 = 0x8268F5B0;
    'dispatch: loop {
        match pc {
            0x8268F5B0 => {
    //   block [0x8268F5B0..0x8268F620)
	// 8268F5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F5BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F5C4: 38AAEAD8  addi r5, r10, -0x1528
	ctx.r[5].s64 = ctx.r[10].s64 + -5416;
	// 8268F5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F5CC: 390B7240  addi r8, r11, 0x7240
	ctx.r[8].s64 = ctx.r[11].s64 + 29248;
	// 8268F5D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268F5D4: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8268F5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F5DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F5E8: 386AECE8  addi r3, r10, -0x1318
	ctx.r[3].s64 = ctx.r[10].s64 + -4888;
	// 8268F5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F60C: 4BDD7815  bl 0x82466e20
	ctx.lr = 0x8268F610;
	sub_82466E20(ctx, base);
	// 8268F610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F620 size=112
    let mut pc: u32 = 0x8268F620;
    'dispatch: loop {
        match pc {
            0x8268F620 => {
    //   block [0x8268F620..0x8268F690)
	// 8268F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F62C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F630: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F634: 38AAEAD8  addi r5, r10, -0x1528
	ctx.r[5].s64 = ctx.r[10].s64 + -5416;
	// 8268F638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F63C: 390B7288  addi r8, r11, 0x7288
	ctx.r[8].s64 = ctx.r[11].s64 + 29320;
	// 8268F640: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268F644: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8268F648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F64C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F658: 386AED18  addi r3, r10, -0x12e8
	ctx.r[3].s64 = ctx.r[10].s64 + -4840;
	// 8268F65C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F67C: 4BDD77A5  bl 0x82466e20
	ctx.lr = 0x8268F680;
	sub_82466E20(ctx, base);
	// 8268F680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F690 size=108
    let mut pc: u32 = 0x8268F690;
    'dispatch: loop {
        match pc {
            0x8268F690 => {
    //   block [0x8268F690..0x8268F6FC)
	// 8268F690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F69C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F6A4: 38EB7330  addi r7, r11, 0x7330
	ctx.r[7].s64 = ctx.r[11].s64 + 29488;
	// 8268F6A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268F6AC: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8268F6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F6B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F6C0: 386AED48  addi r3, r10, -0x12b8
	ctx.r[3].s64 = ctx.r[10].s64 + -4792;
	// 8268F6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F6E8: 4BDD7739  bl 0x82466e20
	ctx.lr = 0x8268F6EC;
	sub_82466E20(ctx, base);
	// 8268F6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F700 size=112
    let mut pc: u32 = 0x8268F700;
    'dispatch: loop {
        match pc {
            0x8268F700 => {
    //   block [0x8268F700..0x8268F770)
	// 8268F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F70C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F714: 38AAE8F8  addi r5, r10, -0x1708
	ctx.r[5].s64 = ctx.r[10].s64 + -5896;
	// 8268F718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F71C: 390B7378  addi r8, r11, 0x7378
	ctx.r[8].s64 = ctx.r[11].s64 + 29560;
	// 8268F720: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268F724: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8268F728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F72C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F738: 386AED78  addi r3, r10, -0x1288
	ctx.r[3].s64 = ctx.r[10].s64 + -4744;
	// 8268F73C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F75C: 4BDD76C5  bl 0x82466e20
	ctx.lr = 0x8268F760;
	sub_82466E20(ctx, base);
	// 8268F760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F770 size=100
    let mut pc: u32 = 0x8268F770;
    'dispatch: loop {
        match pc {
            0x8268F770 => {
    //   block [0x8268F770..0x8268F7D4)
	// 8268F770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F77C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F784: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268F788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F790: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8268F794: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F7A4: 386AEDA8  addi r3, r10, -0x1258
	ctx.r[3].s64 = ctx.r[10].s64 + -4696;
	// 8268F7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F7B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268F7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F7B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268F7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F7C0: 4BDD7661  bl 0x82466e20
	ctx.lr = 0x8268F7C4;
	sub_82466E20(ctx, base);
	// 8268F7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F7D8 size=100
    let mut pc: u32 = 0x8268F7D8;
    'dispatch: loop {
        match pc {
            0x8268F7D8 => {
    //   block [0x8268F7D8..0x8268F83C)
	// 8268F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F7E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F7EC: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268F7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F7F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F7F8: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8268F7FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F80C: 386AEDD8  addi r3, r10, -0x1228
	ctx.r[3].s64 = ctx.r[10].s64 + -4648;
	// 8268F810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F818: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268F81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268F824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F828: 4BDD75F9  bl 0x82466e20
	ctx.lr = 0x8268F82C;
	sub_82466E20(ctx, base);
	// 8268F82C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F840 size=108
    let mut pc: u32 = 0x8268F840;
    'dispatch: loop {
        match pc {
            0x8268F840 => {
    //   block [0x8268F840..0x8268F8AC)
	// 8268F840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F84C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F854: 38EB73D8  addi r7, r11, 0x73d8
	ctx.r[7].s64 = ctx.r[11].s64 + 29656;
	// 8268F858: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268F85C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8268F860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F864: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268F86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F870: 386AEE08  addi r3, r10, -0x11f8
	ctx.r[3].s64 = ctx.r[10].s64 + -4600;
	// 8268F874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268F878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268F898: 4BDD7589  bl 0x82466e20
	ctx.lr = 0x8268F89C;
	sub_82466E20(ctx, base);
	// 8268F89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F8B0 size=112
    let mut pc: u32 = 0x8268F8B0;
    'dispatch: loop {
        match pc {
            0x8268F8B0 => {
    //   block [0x8268F8B0..0x8268F920)
	// 8268F8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F8C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F8C4: 38AAEBC8  addi r5, r10, -0x1438
	ctx.r[5].s64 = ctx.r[10].s64 + -5176;
	// 8268F8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F8CC: 390B7468  addi r8, r11, 0x7468
	ctx.r[8].s64 = ctx.r[11].s64 + 29800;
	// 8268F8D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268F8D4: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8268F8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F8DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F8E8: 386AEE38  addi r3, r10, -0x11c8
	ctx.r[3].s64 = ctx.r[10].s64 + -4552;
	// 8268F8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F90C: 4BDD7515  bl 0x82466e20
	ctx.lr = 0x8268F910;
	sub_82466E20(ctx, base);
	// 8268F910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F920 size=112
    let mut pc: u32 = 0x8268F920;
    'dispatch: loop {
        match pc {
            0x8268F920 => {
    //   block [0x8268F920..0x8268F990)
	// 8268F920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F92C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F930: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F934: 38AAED18  addi r5, r10, -0x12e8
	ctx.r[5].s64 = ctx.r[10].s64 + -4840;
	// 8268F938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F93C: 390B7480  addi r8, r11, 0x7480
	ctx.r[8].s64 = ctx.r[11].s64 + 29824;
	// 8268F940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268F944: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8268F948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F94C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F958: 386AEE68  addi r3, r10, -0x1198
	ctx.r[3].s64 = ctx.r[10].s64 + -4504;
	// 8268F95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F97C: 4BDD74A5  bl 0x82466e20
	ctx.lr = 0x8268F980;
	sub_82466E20(ctx, base);
	// 8268F980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268F990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268F990 size=112
    let mut pc: u32 = 0x8268F990;
    'dispatch: loop {
        match pc {
            0x8268F990 => {
    //   block [0x8268F990..0x8268FA00)
	// 8268F990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268F994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268F998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268F99C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F9A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268F9A4: 38AAE7D8  addi r5, r10, -0x1828
	ctx.r[5].s64 = ctx.r[10].s64 + -6184;
	// 8268F9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268F9AC: 390B74B0  addi r8, r11, 0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29872;
	// 8268F9B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268F9B4: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8268F9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268F9BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268F9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268F9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268F9C8: 386AEE98  addi r3, r10, -0x1168
	ctx.r[3].s64 = ctx.r[10].s64 + -4456;
	// 8268F9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268F9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268F9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268F9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268F9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268F9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268F9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268F9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268F9EC: 4BDD7435  bl 0x82466e20
	ctx.lr = 0x8268F9F0;
	sub_82466E20(ctx, base);
	// 8268F9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268F9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268F9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268F9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FA00 size=112
    let mut pc: u32 = 0x8268FA00;
    'dispatch: loop {
        match pc {
            0x8268FA00 => {
    //   block [0x8268FA00..0x8268FA70)
	// 8268FA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FA0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FA10: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FA14: 38AAE928  addi r5, r10, -0x16d8
	ctx.r[5].s64 = ctx.r[10].s64 + -5848;
	// 8268FA18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FA1C: 390B74F8  addi r8, r11, 0x74f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29944;
	// 8268FA20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268FA24: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8268FA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FA2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FA38: 386AEEC8  addi r3, r10, -0x1138
	ctx.r[3].s64 = ctx.r[10].s64 + -4408;
	// 8268FA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FA5C: 4BDD73C5  bl 0x82466e20
	ctx.lr = 0x8268FA60;
	sub_82466E20(ctx, base);
	// 8268FA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FA70 size=112
    let mut pc: u32 = 0x8268FA70;
    'dispatch: loop {
        match pc {
            0x8268FA70 => {
    //   block [0x8268FA70..0x8268FAE0)
	// 8268FA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FA7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FA80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FA84: 38AAE898  addi r5, r10, -0x1768
	ctx.r[5].s64 = ctx.r[10].s64 + -5992;
	// 8268FA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FA8C: 390B7540  addi r8, r11, 0x7540
	ctx.r[8].s64 = ctx.r[11].s64 + 30016;
	// 8268FA90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268FA94: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8268FA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FA9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FAA8: 386AEEF8  addi r3, r10, -0x1108
	ctx.r[3].s64 = ctx.r[10].s64 + -4360;
	// 8268FAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FACC: 4BDD7355  bl 0x82466e20
	ctx.lr = 0x8268FAD0;
	sub_82466E20(ctx, base);
	// 8268FAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FAE0 size=112
    let mut pc: u32 = 0x8268FAE0;
    'dispatch: loop {
        match pc {
            0x8268FAE0 => {
    //   block [0x8268FAE0..0x8268FB50)
	// 8268FAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FAEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FAF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FAF4: 38AAE8F8  addi r5, r10, -0x1708
	ctx.r[5].s64 = ctx.r[10].s64 + -5896;
	// 8268FAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FAFC: 390B7558  addi r8, r11, 0x7558
	ctx.r[8].s64 = ctx.r[11].s64 + 30040;
	// 8268FB00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268FB04: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8268FB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FB0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FB18: 386AEF28  addi r3, r10, -0x10d8
	ctx.r[3].s64 = ctx.r[10].s64 + -4312;
	// 8268FB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FB3C: 4BDD72E5  bl 0x82466e20
	ctx.lr = 0x8268FB40;
	sub_82466E20(ctx, base);
	// 8268FB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FB50 size=112
    let mut pc: u32 = 0x8268FB50;
    'dispatch: loop {
        match pc {
            0x8268FB50 => {
    //   block [0x8268FB50..0x8268FBC0)
	// 8268FB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FB5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FB60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FB64: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268FB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FB6C: 390B7588  addi r8, r11, 0x7588
	ctx.r[8].s64 = ctx.r[11].s64 + 30088;
	// 8268FB70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268FB74: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8268FB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FB7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FB88: 386AEF58  addi r3, r10, -0x10a8
	ctx.r[3].s64 = ctx.r[10].s64 + -4264;
	// 8268FB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FBAC: 4BDD7275  bl 0x82466e20
	ctx.lr = 0x8268FBB0;
	sub_82466E20(ctx, base);
	// 8268FBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FBC0 size=112
    let mut pc: u32 = 0x8268FBC0;
    'dispatch: loop {
        match pc {
            0x8268FBC0 => {
    //   block [0x8268FBC0..0x8268FC30)
	// 8268FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FBCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FBD0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FBD4: 38AAEF58  addi r5, r10, -0x10a8
	ctx.r[5].s64 = ctx.r[10].s64 + -4264;
	// 8268FBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FBDC: 390B75E8  addi r8, r11, 0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30184;
	// 8268FBE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268FBE4: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8268FBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FBEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FBF8: 386AEF88  addi r3, r10, -0x1078
	ctx.r[3].s64 = ctx.r[10].s64 + -4216;
	// 8268FBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FC1C: 4BDD7205  bl 0x82466e20
	ctx.lr = 0x8268FC20;
	sub_82466E20(ctx, base);
	// 8268FC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FC30 size=112
    let mut pc: u32 = 0x8268FC30;
    'dispatch: loop {
        match pc {
            0x8268FC30 => {
    //   block [0x8268FC30..0x8268FCA0)
	// 8268FC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FC3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FC40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FC44: 38AAEF58  addi r5, r10, -0x10a8
	ctx.r[5].s64 = ctx.r[10].s64 + -4264;
	// 8268FC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FC4C: 390B7600  addi r8, r11, 0x7600
	ctx.r[8].s64 = ctx.r[11].s64 + 30208;
	// 8268FC50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268FC54: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8268FC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FC5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FC68: 386AEFB8  addi r3, r10, -0x1048
	ctx.r[3].s64 = ctx.r[10].s64 + -4168;
	// 8268FC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FC8C: 4BDD7195  bl 0x82466e20
	ctx.lr = 0x8268FC90;
	sub_82466E20(ctx, base);
	// 8268FC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FCA0 size=112
    let mut pc: u32 = 0x8268FCA0;
    'dispatch: loop {
        match pc {
            0x8268FCA0 => {
    //   block [0x8268FCA0..0x8268FD10)
	// 8268FCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FCAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FCB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FCB4: 38AAEF58  addi r5, r10, -0x10a8
	ctx.r[5].s64 = ctx.r[10].s64 + -4264;
	// 8268FCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FCBC: 390B7630  addi r8, r11, 0x7630
	ctx.r[8].s64 = ctx.r[11].s64 + 30256;
	// 8268FCC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268FCC4: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8268FCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FCCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FCD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FCD8: 386AEFE8  addi r3, r10, -0x1018
	ctx.r[3].s64 = ctx.r[10].s64 + -4120;
	// 8268FCDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268FCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FCFC: 4BDD7125  bl 0x82466e20
	ctx.lr = 0x8268FD00;
	sub_82466E20(ctx, base);
	// 8268FD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268FD10 size=24
    let mut pc: u32 = 0x8268FD10;
    'dispatch: loop {
        match pc {
            0x8268FD10 => {
    //   block [0x8268FD10..0x8268FD28)
	// 8268FD10: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FD14: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268FD18: 394AA7B8  addi r10, r10, -0x5848
	ctx.r[10].s64 = ctx.r[10].s64 + -22600;
	// 8268FD1C: 816B7134  lwz r11, 0x7134(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28980 as u32) ) } as u64;
	// 8268FD20: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8268FD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FD28 size=112
    let mut pc: u32 = 0x8268FD28;
    'dispatch: loop {
        match pc {
            0x8268FD28 => {
    //   block [0x8268FD28..0x8268FD98)
	// 8268FD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FD34: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268FD38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268FD3C: 392A6CE4  addi r9, r10, 0x6ce4
	ctx.r[9].s64 = ctx.r[10].s64 + 27876;
	// 8268FD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FD44: 390BA7B8  addi r8, r11, -0x5848
	ctx.r[8].s64 = ctx.r[11].s64 + -22600;
	// 8268FD48: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8268FD4C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8268FD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FD54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FD58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FD60: 386AF018  addi r3, r10, -0xfe8
	ctx.r[3].s64 = ctx.r[10].s64 + -4072;
	// 8268FD64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268FD68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268FD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268FD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FD84: 4BDD709D  bl 0x82466e20
	ctx.lr = 0x8268FD88;
	sub_82466E20(ctx, base);
	// 8268FD88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FD98 size=108
    let mut pc: u32 = 0x8268FD98;
    'dispatch: loop {
        match pc {
            0x8268FD98 => {
    //   block [0x8268FD98..0x8268FE04)
	// 8268FD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FDA4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FDA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FDAC: 38EB7648  addi r7, r11, 0x7648
	ctx.r[7].s64 = ctx.r[11].s64 + 30280;
	// 8268FDB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268FDB4: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8268FDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FDBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FDC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268FDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FDC8: 386AF048  addi r3, r10, -0xfb8
	ctx.r[3].s64 = ctx.r[10].s64 + -4024;
	// 8268FDCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268FDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268FDF0: 4BDD7031  bl 0x82466e20
	ctx.lr = 0x8268FDF4;
	sub_82466E20(ctx, base);
	// 8268FDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FE08 size=108
    let mut pc: u32 = 0x8268FE08;
    'dispatch: loop {
        match pc {
            0x8268FE08 => {
    //   block [0x8268FE08..0x8268FE74)
	// 8268FE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FE14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FE1C: 38EB7660  addi r7, r11, 0x7660
	ctx.r[7].s64 = ctx.r[11].s64 + 30304;
	// 8268FE20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268FE24: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8268FE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FE2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268FE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FE38: 386AF078  addi r3, r10, -0xf88
	ctx.r[3].s64 = ctx.r[10].s64 + -3976;
	// 8268FE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268FE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268FE60: 4BDD6FC1  bl 0x82466e20
	ctx.lr = 0x8268FE64;
	sub_82466E20(ctx, base);
	// 8268FE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FE78 size=116
    let mut pc: u32 = 0x8268FE78;
    'dispatch: loop {
        match pc {
            0x8268FE78 => {
    //   block [0x8268FE78..0x8268FEEC)
	// 8268FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FE84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FE88: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268FE8C: 390B76AC  addi r8, r11, 0x76ac
	ctx.r[8].s64 = ctx.r[11].s64 + 30380;
	// 8268FE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FE94: 392A6DB0  addi r9, r10, 0x6db0
	ctx.r[9].s64 = ctx.r[10].s64 + 28080;
	// 8268FE98: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FE9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8268FEA0: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268FEA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FEAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FEBC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268FEC0: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8268FEC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268FEC8: 386BF0A8  addi r3, r11, -0xf58
	ctx.r[3].s64 = ctx.r[11].s64 + -3928;
	// 8268FECC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268FED0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FED8: 4BDD6F49  bl 0x82466e20
	ctx.lr = 0x8268FEDC;
	sub_82466E20(ctx, base);
	// 8268FEDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FEF0 size=108
    let mut pc: u32 = 0x8268FEF0;
    'dispatch: loop {
        match pc {
            0x8268FEF0 => {
    //   block [0x8268FEF0..0x8268FF5C)
	// 8268FEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FEFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FF00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268FF04: 38EB76C8  addi r7, r11, 0x76c8
	ctx.r[7].s64 = ctx.r[11].s64 + 30408;
	// 8268FF08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268FF0C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 8268FF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FF14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FF18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268FF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268FF20: 386AF0D8  addi r3, r10, -0xf28
	ctx.r[3].s64 = ctx.r[10].s64 + -3880;
	// 8268FF24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268FF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268FF2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FF44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268FF48: 4BDD6ED9  bl 0x82466e20
	ctx.lr = 0x8268FF4C;
	sub_82466E20(ctx, base);
	// 8268FF4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268FF60 size=24
    let mut pc: u32 = 0x8268FF60;
    'dispatch: loop {
        match pc {
            0x8268FF60 => {
    //   block [0x8268FF60..0x8268FF78)
	// 8268FF60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268FF64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8268FF68: 394AA800  addi r10, r10, -0x5800
	ctx.r[10].s64 = ctx.r[10].s64 + -22528;
	// 8268FF6C: 816B76C4  lwz r11, 0x76c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30404 as u32) ) } as u64;
	// 8268FF70: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268FF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FF78 size=116
    let mut pc: u32 = 0x8268FF78;
    'dispatch: loop {
        match pc {
            0x8268FF78 => {
    //   block [0x8268FF78..0x8268FFEC)
	// 8268FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FF84: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8268FF88: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268FF8C: 390BA800  addi r8, r11, -0x5800
	ctx.r[8].s64 = ctx.r[11].s64 + -22528;
	// 8268FF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268FF94: 392A6E0C  addi r9, r10, 0x6e0c
	ctx.r[9].s64 = ctx.r[10].s64 + 28172;
	// 8268FF98: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268FF9C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8268FFA0: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 8268FFA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268FFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268FFAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268FFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268FFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268FFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268FFBC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268FFC0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8268FFC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268FFC8: 386BF108  addi r3, r11, -0xef8
	ctx.r[3].s64 = ctx.r[11].s64 + -3832;
	// 8268FFCC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8268FFD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268FFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268FFD8: 4BDD6E49  bl 0x82466e20
	ctx.lr = 0x8268FFDC;
	sub_82466E20(ctx, base);
	// 8268FFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268FFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268FFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268FFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268FFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268FFF0 size=112
    let mut pc: u32 = 0x8268FFF0;
    'dispatch: loop {
        match pc {
            0x8268FFF0 => {
    //   block [0x8268FFF0..0x82690060)
	// 8268FFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268FFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268FFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268FFFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690000: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690004: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269000C: 390B7730  addi r8, r11, 0x7730
	ctx.r[8].s64 = ctx.r[11].s64 + 30512;
	// 82690010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82690014: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 82690018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269001C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690028: 386AF138  addi r3, r10, -0xec8
	ctx.r[3].s64 = ctx.r[10].s64 + -3784;
	// 8269002C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269003C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269004C: 4BDD6DD5  bl 0x82466e20
	ctx.lr = 0x82690050;
	sub_82466E20(ctx, base);
	// 82690050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269005C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690060 size=112
    let mut pc: u32 = 0x82690060;
    'dispatch: loop {
        match pc {
            0x82690060 => {
    //   block [0x82690060..0x826900D0)
	// 82690060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269006C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690070: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690074: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269007C: 390B7748  addi r8, r11, 0x7748
	ctx.r[8].s64 = ctx.r[11].s64 + 30536;
	// 82690080: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82690084: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82690088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269008C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690098: 386AF168  addi r3, r10, -0xe98
	ctx.r[3].s64 = ctx.r[10].s64 + -3736;
	// 8269009C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826900A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826900A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826900A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826900AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826900B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826900B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826900B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826900BC: 4BDD6D65  bl 0x82466e20
	ctx.lr = 0x826900C0;
	sub_82466E20(ctx, base);
	// 826900C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826900C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826900C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826900CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826900D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826900D0 size=108
    let mut pc: u32 = 0x826900D0;
    'dispatch: loop {
        match pc {
            0x826900D0 => {
    //   block [0x826900D0..0x8269013C)
	// 826900D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826900D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826900D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826900DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826900E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826900E4: 38EB7778  addi r7, r11, 0x7778
	ctx.r[7].s64 = ctx.r[11].s64 + 30584;
	// 826900E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826900EC: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826900F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826900F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826900F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826900FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690100: 386AF198  addi r3, r10, -0xe68
	ctx.r[3].s64 = ctx.r[10].s64 + -3688;
	// 82690104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269010C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269011C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690128: 4BDD6CF9  bl 0x82466e20
	ctx.lr = 0x8269012C;
	sub_82466E20(ctx, base);
	// 8269012C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690140 size=108
    let mut pc: u32 = 0x82690140;
    'dispatch: loop {
        match pc {
            0x82690140 => {
    //   block [0x82690140..0x826901AC)
	// 82690140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269014C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690154: 38EB77C0  addi r7, r11, 0x77c0
	ctx.r[7].s64 = ctx.r[11].s64 + 30656;
	// 82690158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269015C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82690160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690164: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269016C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690170: 386AF1C8  addi r3, r10, -0xe38
	ctx.r[3].s64 = ctx.r[10].s64 + -3640;
	// 82690174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269017C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269018C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690198: 4BDD6C89  bl 0x82466e20
	ctx.lr = 0x8269019C;
	sub_82466E20(ctx, base);
	// 8269019C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826901A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826901A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826901A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826901B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826901B0 size=112
    let mut pc: u32 = 0x826901B0;
    'dispatch: loop {
        match pc {
            0x826901B0 => {
    //   block [0x826901B0..0x82690220)
	// 826901B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826901B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826901B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826901BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826901C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826901C4: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 826901C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826901CC: 390B77F0  addi r8, r11, 0x77f0
	ctx.r[8].s64 = ctx.r[11].s64 + 30704;
	// 826901D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826901D4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826901D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826901DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826901E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826901E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826901E8: 386AF1F8  addi r3, r10, -0xe08
	ctx.r[3].s64 = ctx.r[10].s64 + -3592;
	// 826901EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826901F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826901F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826901F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826901FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269020C: 4BDD6C15  bl 0x82466e20
	ctx.lr = 0x82690210;
	sub_82466E20(ctx, base);
	// 82690210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269021C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690220 size=108
    let mut pc: u32 = 0x82690220;
    'dispatch: loop {
        match pc {
            0x82690220 => {
    //   block [0x82690220..0x8269028C)
	// 82690220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269022C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690234: 38EB7820  addi r7, r11, 0x7820
	ctx.r[7].s64 = ctx.r[11].s64 + 30752;
	// 82690238: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269023C: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82690240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269024C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690250: 386AF228  addi r3, r10, -0xdd8
	ctx.r[3].s64 = ctx.r[10].s64 + -3544;
	// 82690254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269025C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269026C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690278: 4BDD6BA9  bl 0x82466e20
	ctx.lr = 0x8269027C;
	sub_82466E20(ctx, base);
	// 8269027C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690290 size=108
    let mut pc: u32 = 0x82690290;
    'dispatch: loop {
        match pc {
            0x82690290 => {
    //   block [0x82690290..0x826902FC)
	// 82690290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269029C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826902A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826902A4: 38EB7880  addi r7, r11, 0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + 30848;
	// 826902A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826902AC: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826902B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826902B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826902B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826902BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826902C0: 386AF258  addi r3, r10, -0xda8
	ctx.r[3].s64 = ctx.r[10].s64 + -3496;
	// 826902C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826902C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826902CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826902D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826902D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826902D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826902DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826902E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826902E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826902E8: 4BDD6B39  bl 0x82466e20
	ctx.lr = 0x826902EC;
	sub_82466E20(ctx, base);
	// 826902EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826902F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826902F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826902F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690300 size=116
    let mut pc: u32 = 0x82690300;
    'dispatch: loop {
        match pc {
            0x82690300 => {
    //   block [0x82690300..0x82690374)
	// 82690300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269030C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82690310: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82690314: 390A78C8  addi r8, r10, 0x78c8
	ctx.r[8].s64 = ctx.r[10].s64 + 30920;
	// 82690318: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269031C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82690320: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690324: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690328: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269032C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690334: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82690338: 396B6E48  addi r11, r11, 0x6e48
	ctx.r[11].s64 = ctx.r[11].s64 + 28232;
	// 8269033C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690344: 386AF288  addi r3, r10, -0xd78
	ctx.r[3].s64 = ctx.r[10].s64 + -3448;
	// 82690348: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269034C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690350: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82690354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269035C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690360: 4BDD6AC1  bl 0x82466e20
	ctx.lr = 0x82690364;
	sub_82466E20(ctx, base);
	// 82690364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269036C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690378 size=112
    let mut pc: u32 = 0x82690378;
    'dispatch: loop {
        match pc {
            0x82690378 => {
    //   block [0x82690378..0x826903E8)
	// 82690378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269037C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690384: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690388: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8269038C: 38AAF2E8  addi r5, r10, -0xd18
	ctx.r[5].s64 = ctx.r[10].s64 + -3352;
	// 82690390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690394: 390B7958  addi r8, r11, 0x7958
	ctx.r[8].s64 = ctx.r[11].s64 + 31064;
	// 82690398: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269039C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826903A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826903A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826903A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826903AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826903B0: 386AF2B8  addi r3, r10, -0xd48
	ctx.r[3].s64 = ctx.r[10].s64 + -3400;
	// 826903B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826903B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826903BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826903C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826903C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826903C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826903CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826903D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826903D4: 4BDD6A4D  bl 0x82466e20
	ctx.lr = 0x826903D8;
	sub_82466E20(ctx, base);
	// 826903D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826903DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826903E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826903E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826903E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826903E8 size=100
    let mut pc: u32 = 0x826903E8;
    'dispatch: loop {
        match pc {
            0x826903E8 => {
    //   block [0x826903E8..0x8269044C)
	// 826903E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826903EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826903F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826903F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826903F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826903FC: 38AADBD8  addi r5, r10, -0x2428
	ctx.r[5].s64 = ctx.r[10].s64 + -9256;
	// 82690400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690408: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8269040C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269041C: 386AF2E8  addi r3, r10, -0xd18
	ctx.r[3].s64 = ctx.r[10].s64 + -3352;
	// 82690420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690428: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269042C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82690434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690438: 4BDD69E9  bl 0x82466e20
	ctx.lr = 0x8269043C;
	sub_82466E20(ctx, base);
	// 8269043C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82690450 size=24
    let mut pc: u32 = 0x82690450;
    'dispatch: loop {
        match pc {
            0x82690450 => {
    //   block [0x82690450..0x82690468)
	// 82690450: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690454: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82690458: 394AA8C0  addi r10, r10, -0x5740
	ctx.r[10].s64 = ctx.r[10].s64 + -22336;
	// 8269045C: 816B79D0  lwz r11, 0x79d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31184 as u32) ) } as u64;
	// 82690460: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82690464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690468 size=116
    let mut pc: u32 = 0x82690468;
    'dispatch: loop {
        match pc {
            0x82690468 => {
    //   block [0x82690468..0x826904DC)
	// 82690468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269046C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690474: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82690478: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8269047C: 390BA8C0  addi r8, r11, -0x5740
	ctx.r[8].s64 = ctx.r[11].s64 + -22336;
	// 82690480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690484: 392A6E8C  addi r9, r10, 0x6e8c
	ctx.r[9].s64 = ctx.r[10].s64 + 28300;
	// 82690488: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269048C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82690490: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690494: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269049C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826904A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826904A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826904A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826904AC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826904B0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826904B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826904B8: 386BF318  addi r3, r11, -0xce8
	ctx.r[3].s64 = ctx.r[11].s64 + -3304;
	// 826904BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826904C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826904C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826904C8: 4BDD6959  bl 0x82466e20
	ctx.lr = 0x826904CC;
	sub_82466E20(ctx, base);
	// 826904CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826904D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826904D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826904D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826904E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826904E0 size=112
    let mut pc: u32 = 0x826904E0;
    'dispatch: loop {
        match pc {
            0x826904E0 => {
    //   block [0x826904E0..0x82690550)
	// 826904E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826904E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826904E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826904EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826904F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826904F4: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 826904F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826904FC: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82690500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82690504: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82690508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269050C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690518: 386AF348  addi r3, r10, -0xcb8
	ctx.r[3].s64 = ctx.r[10].s64 + -3256;
	// 8269051C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269052C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269053C: 4BDD68E5  bl 0x82466e20
	ctx.lr = 0x82690540;
	sub_82466E20(ctx, base);
	// 82690540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269054C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690550 size=112
    let mut pc: u32 = 0x82690550;
    'dispatch: loop {
        match pc {
            0x82690550 => {
    //   block [0x82690550..0x826905C0)
	// 82690550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269055C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690560: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690564: 38AAF288  addi r5, r10, -0xd78
	ctx.r[5].s64 = ctx.r[10].s64 + -3448;
	// 82690568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269056C: 390B7A20  addi r8, r11, 0x7a20
	ctx.r[8].s64 = ctx.r[11].s64 + 31264;
	// 82690570: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82690574: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82690578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269057C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690588: 386AF378  addi r3, r10, -0xc88
	ctx.r[3].s64 = ctx.r[10].s64 + -3208;
	// 8269058C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269059C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826905A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826905A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826905A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826905AC: 4BDD6875  bl 0x82466e20
	ctx.lr = 0x826905B0;
	sub_82466E20(ctx, base);
	// 826905B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826905B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826905B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826905BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826905C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826905C0 size=108
    let mut pc: u32 = 0x826905C0;
    'dispatch: loop {
        match pc {
            0x826905C0 => {
    //   block [0x826905C0..0x8269062C)
	// 826905C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826905C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826905C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826905CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826905D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826905D4: 38EB7A80  addi r7, r11, 0x7a80
	ctx.r[7].s64 = ctx.r[11].s64 + 31360;
	// 826905D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826905DC: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 826905E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826905E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826905E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826905EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826905F0: 386AF3A8  addi r3, r10, -0xc58
	ctx.r[3].s64 = ctx.r[10].s64 + -3160;
	// 826905F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826905F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826905FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269060C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690618: 4BDD6809  bl 0x82466e20
	ctx.lr = 0x8269061C;
	sub_82466E20(ctx, base);
	// 8269061C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690630 size=108
    let mut pc: u32 = 0x82690630;
    'dispatch: loop {
        match pc {
            0x82690630 => {
    //   block [0x82690630..0x8269069C)
	// 82690630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269063C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690644: 38EB7AC8  addi r7, r11, 0x7ac8
	ctx.r[7].s64 = ctx.r[11].s64 + 31432;
	// 82690648: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269064C: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82690650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269065C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690660: 386AF3D8  addi r3, r10, -0xc28
	ctx.r[3].s64 = ctx.r[10].s64 + -3112;
	// 82690664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269066C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269067C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690688: 4BDD6799  bl 0x82466e20
	ctx.lr = 0x8269068C;
	sub_82466E20(ctx, base);
	// 8269068C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826906A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826906A0 size=112
    let mut pc: u32 = 0x826906A0;
    'dispatch: loop {
        match pc {
            0x826906A0 => {
    //   block [0x826906A0..0x82690710)
	// 826906A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826906A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826906A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826906AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826906B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826906B4: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 826906B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826906BC: 390B7B10  addi r8, r11, 0x7b10
	ctx.r[8].s64 = ctx.r[11].s64 + 31504;
	// 826906C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826906C4: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826906C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826906CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826906D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826906D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826906D8: 386AF408  addi r3, r10, -0xbf8
	ctx.r[3].s64 = ctx.r[10].s64 + -3064;
	// 826906DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826906E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826906E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826906E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826906EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826906F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826906F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826906F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826906FC: 4BDD6725  bl 0x82466e20
	ctx.lr = 0x82690700;
	sub_82466E20(ctx, base);
	// 82690700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269070C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690710 size=112
    let mut pc: u32 = 0x82690710;
    'dispatch: loop {
        match pc {
            0x82690710 => {
    //   block [0x82690710..0x82690780)
	// 82690710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269071C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690720: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690724: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269072C: 390B7BB8  addi r8, r11, 0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 31672;
	// 82690730: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82690734: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82690738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269073C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690748: 386AF438  addi r3, r10, -0xbc8
	ctx.r[3].s64 = ctx.r[10].s64 + -3016;
	// 8269074C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269075C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269076C: 4BDD66B5  bl 0x82466e20
	ctx.lr = 0x82690770;
	sub_82466E20(ctx, base);
	// 82690770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269077C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690780 size=108
    let mut pc: u32 = 0x82690780;
    'dispatch: loop {
        match pc {
            0x82690780 => {
    //   block [0x82690780..0x826907EC)
	// 82690780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269078C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690794: 38EB7C00  addi r7, r11, 0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + 31744;
	// 82690798: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269079C: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 826907A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826907A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826907A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826907AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826907B0: 386AF468  addi r3, r10, -0xb98
	ctx.r[3].s64 = ctx.r[10].s64 + -2968;
	// 826907B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826907B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826907BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826907C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826907C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826907C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826907CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826907D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826907D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826907D8: 4BDD6649  bl 0x82466e20
	ctx.lr = 0x826907DC;
	sub_82466E20(ctx, base);
	// 826907DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826907E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826907E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826907E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826907F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826907F0 size=108
    let mut pc: u32 = 0x826907F0;
    'dispatch: loop {
        match pc {
            0x826907F0 => {
    //   block [0x826907F0..0x8269085C)
	// 826907F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826907F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826907F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826907FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690804: 38EB7C30  addi r7, r11, 0x7c30
	ctx.r[7].s64 = ctx.r[11].s64 + 31792;
	// 82690808: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269080C: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 82690810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690814: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269081C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690820: 386AF498  addi r3, r10, -0xb68
	ctx.r[3].s64 = ctx.r[10].s64 + -2920;
	// 82690824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269082C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269083C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690848: 4BDD65D9  bl 0x82466e20
	ctx.lr = 0x8269084C;
	sub_82466E20(ctx, base);
	// 8269084C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690860 size=112
    let mut pc: u32 = 0x82690860;
    'dispatch: loop {
        match pc {
            0x82690860 => {
    //   block [0x82690860..0x826908D0)
	// 82690860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269086C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690870: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690874: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269087C: 390B7CC0  addi r8, r11, 0x7cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 31936;
	// 82690880: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82690884: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82690888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269088C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690898: 386AF4C8  addi r3, r10, -0xb38
	ctx.r[3].s64 = ctx.r[10].s64 + -2872;
	// 8269089C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826908A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826908A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826908A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826908AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826908B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826908B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826908B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826908BC: 4BDD6565  bl 0x82466e20
	ctx.lr = 0x826908C0;
	sub_82466E20(ctx, base);
	// 826908C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826908C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826908C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826908CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826908D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826908D0 size=112
    let mut pc: u32 = 0x826908D0;
    'dispatch: loop {
        match pc {
            0x826908D0 => {
    //   block [0x826908D0..0x82690940)
	// 826908D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826908D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826908D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826908DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826908E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826908E4: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 826908E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826908EC: 390B7D50  addi r8, r11, 0x7d50
	ctx.r[8].s64 = ctx.r[11].s64 + 32080;
	// 826908F0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826908F4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826908F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826908FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690908: 386AF4F8  addi r3, r10, -0xb08
	ctx.r[3].s64 = ctx.r[10].s64 + -2824;
	// 8269090C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269091C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269092C: 4BDD64F5  bl 0x82466e20
	ctx.lr = 0x82690930;
	sub_82466E20(ctx, base);
	// 82690930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269093C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690940 size=100
    let mut pc: u32 = 0x82690940;
    'dispatch: loop {
        match pc {
            0x82690940 => {
    //   block [0x82690940..0x826909A4)
	// 82690940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269094C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690954: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269095C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690960: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82690964: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269096C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690974: 386AF528  addi r3, r10, -0xad8
	ctx.r[3].s64 = ctx.r[10].s64 + -2776;
	// 82690978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269097C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82690984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269098C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690990: 4BDD6491  bl 0x82466e20
	ctx.lr = 0x82690994;
	sub_82466E20(ctx, base);
	// 82690994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269099C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826909A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826909A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826909A8 size=112
    let mut pc: u32 = 0x826909A8;
    'dispatch: loop {
        match pc {
            0x826909A8 => {
    //   block [0x826909A8..0x82690A18)
	// 826909A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826909AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826909B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826909B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826909B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826909BC: 38AAF108  addi r5, r10, -0xef8
	ctx.r[5].s64 = ctx.r[10].s64 + -3832;
	// 826909C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826909C4: 390B7E10  addi r8, r11, 0x7e10
	ctx.r[8].s64 = ctx.r[11].s64 + 32272;
	// 826909C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826909CC: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826909D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826909D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826909D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826909DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826909E0: 386AF558  addi r3, r10, -0xaa8
	ctx.r[3].s64 = ctx.r[10].s64 + -2728;
	// 826909E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826909E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826909EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826909F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826909F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826909F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826909FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690A04: 4BDD641D  bl 0x82466e20
	ctx.lr = 0x82690A08;
	sub_82466E20(ctx, base);
	// 82690A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690A18 size=112
    let mut pc: u32 = 0x82690A18;
    'dispatch: loop {
        match pc {
            0x82690A18 => {
    //   block [0x82690A18..0x82690A88)
	// 82690A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690A24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690A28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690A2C: 38AAEF58  addi r5, r10, -0x10a8
	ctx.r[5].s64 = ctx.r[10].s64 + -4264;
	// 82690A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690A34: 390B7E40  addi r8, r11, 0x7e40
	ctx.r[8].s64 = ctx.r[11].s64 + 32320;
	// 82690A38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82690A3C: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82690A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690A44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690A50: 386AF588  addi r3, r10, -0xa78
	ctx.r[3].s64 = ctx.r[10].s64 + -2680;
	// 82690A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690A74: 4BDD63AD  bl 0x82466e20
	ctx.lr = 0x82690A78;
	sub_82466E20(ctx, base);
	// 82690A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690A88 size=108
    let mut pc: u32 = 0x82690A88;
    'dispatch: loop {
        match pc {
            0x82690A88 => {
    //   block [0x82690A88..0x82690AF4)
	// 82690A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690A94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690A9C: 38EB7E58  addi r7, r11, 0x7e58
	ctx.r[7].s64 = ctx.r[11].s64 + 32344;
	// 82690AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82690AA4: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82690AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690AAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690AB8: 386AF5B8  addi r3, r10, -0xa48
	ctx.r[3].s64 = ctx.r[10].s64 + -2632;
	// 82690ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690AE0: 4BDD6341  bl 0x82466e20
	ctx.lr = 0x82690AE4;
	sub_82466E20(ctx, base);
	// 82690AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690AF8 size=112
    let mut pc: u32 = 0x82690AF8;
    'dispatch: loop {
        match pc {
            0x82690AF8 => {
    //   block [0x82690AF8..0x82690B68)
	// 82690AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690B04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690B08: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690B0C: 38AAF528  addi r5, r10, -0xad8
	ctx.r[5].s64 = ctx.r[10].s64 + -2776;
	// 82690B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690B14: 390B7E88  addi r8, r11, 0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + 32392;
	// 82690B18: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82690B1C: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 82690B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690B24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690B30: 386AF5E8  addi r3, r10, -0xa18
	ctx.r[3].s64 = ctx.r[10].s64 + -2584;
	// 82690B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690B54: 4BDD62CD  bl 0x82466e20
	ctx.lr = 0x82690B58;
	sub_82466E20(ctx, base);
	// 82690B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690B68 size=108
    let mut pc: u32 = 0x82690B68;
    'dispatch: loop {
        match pc {
            0x82690B68 => {
    //   block [0x82690B68..0x82690BD4)
	// 82690B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690B74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690B78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690B7C: 38EB7F00  addi r7, r11, 0x7f00
	ctx.r[7].s64 = ctx.r[11].s64 + 32512;
	// 82690B80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82690B84: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82690B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690B8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690B98: 386AF618  addi r3, r10, -0x9e8
	ctx.r[3].s64 = ctx.r[10].s64 + -2536;
	// 82690B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690BC0: 4BDD6261  bl 0x82466e20
	ctx.lr = 0x82690BC4;
	sub_82466E20(ctx, base);
	// 82690BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690BD8 size=108
    let mut pc: u32 = 0x82690BD8;
    'dispatch: loop {
        match pc {
            0x82690BD8 => {
    //   block [0x82690BD8..0x82690C44)
	// 82690BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690BE4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690BE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82690BEC: 38EB7F48  addi r7, r11, 0x7f48
	ctx.r[7].s64 = ctx.r[11].s64 + 32584;
	// 82690BF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82690BF4: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82690BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82690C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690C08: 386AF648  addi r3, r10, -0x9b8
	ctx.r[3].s64 = ctx.r[10].s64 + -2488;
	// 82690C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82690C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82690C30: 4BDD61F1  bl 0x82466e20
	ctx.lr = 0x82690C34;
	sub_82466E20(ctx, base);
	// 82690C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82690C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82690C48 size=112
    let mut pc: u32 = 0x82690C48;
    'dispatch: loop {
        match pc {
            0x82690C48 => {
    //   block [0x82690C48..0x82690CB8)
	// 82690C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82690C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82690C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82690C54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690C58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82690C5C: 38AAF0A8  addi r5, r10, -0xf58
	ctx.r[5].s64 = ctx.r[10].s64 + -3928;
	// 82690C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82690C64: 390B7F78  addi r8, r11, 0x7f78
	ctx.r[8].s64 = ctx.r[11].s64 + 32632;
	// 82690C68: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82690C6C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 82690C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82690C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82690C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82690C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82690C80: 386AF678  addi r3, r10, -0x988
	ctx.r[3].s64 = ctx.r[10].s64 + -2440;
	// 82690C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82690C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82690C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82690C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82690C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82690C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82690C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82690CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82690CA4: 4BDD617D  bl 0x82466e20
	ctx.lr = 0x82690CA8;
	sub_82466E20(ctx, base);
	// 82690CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82690CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82690CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82690CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


