pub fn sub_826187D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826187D0 size=112
    let mut pc: u32 = 0x826187D0;
    'dispatch: loop {
        match pc {
            0x826187D0 => {
    //   block [0x826187D0..0x82618840)
	// 826187D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826187D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826187D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826187DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826187E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826187E4: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 826187E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826187EC: 390BA0C8  addi r8, r11, -0x5f38
	ctx.r[8].s64 = ctx.r[11].s64 + -24376;
	// 826187F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826187F4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826187F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826187FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618808: 386AC7DC  addi r3, r10, -0x3824
	ctx.r[3].s64 = ctx.r[10].s64 + -14372;
	// 8261880C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261881C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261882C: 4BE4E5F5  bl 0x82466e20
	ctx.lr = 0x82618830;
	sub_82466E20(ctx, base);
	// 82618830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618840 size=112
    let mut pc: u32 = 0x82618840;
    'dispatch: loop {
        match pc {
            0x82618840 => {
    //   block [0x82618840..0x826188B0)
	// 82618840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261884C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618850: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618854: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 82618858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261885C: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 82618860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618864: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82618868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261886C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618878: 386AC80C  addi r3, r10, -0x37f4
	ctx.r[3].s64 = ctx.r[10].s64 + -14324;
	// 8261887C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261888C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261889C: 4BE4E585  bl 0x82466e20
	ctx.lr = 0x826188A0;
	sub_82466E20(ctx, base);
	// 826188A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826188A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826188A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826188AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826188B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826188B0 size=24
    let mut pc: u32 = 0x826188B0;
    'dispatch: loop {
        match pc {
            0x826188B0 => {
    //   block [0x826188B0..0x826188C8)
	// 826188B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826188B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826188B8: 394AE2C0  addi r10, r10, -0x1d40
	ctx.r[10].s64 = ctx.r[10].s64 + -7488;
	// 826188BC: 816BA110  lwz r11, -0x5ef0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24304 as u32) ) } as u64;
	// 826188C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826188C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826188C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826188C8 size=112
    let mut pc: u32 = 0x826188C8;
    'dispatch: loop {
        match pc {
            0x826188C8 => {
    //   block [0x826188C8..0x82618938)
	// 826188C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826188CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826188D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826188D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826188D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826188DC: 392AFE38  addi r9, r10, -0x1c8
	ctx.r[9].s64 = ctx.r[10].s64 + -456;
	// 826188E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826188E4: 390BE2C0  addi r8, r11, -0x1d40
	ctx.r[8].s64 = ctx.r[11].s64 + -7488;
	// 826188E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826188EC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826188F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826188F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826188F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826188FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618900: 386AC83C  addi r3, r10, -0x37c4
	ctx.r[3].s64 = ctx.r[10].s64 + -14276;
	// 82618904: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618908: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261890C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261891C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618924: 4BE4E4FD  bl 0x82466e20
	ctx.lr = 0x82618928;
	sub_82466E20(ctx, base);
	// 82618928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261892C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618938 size=108
    let mut pc: u32 = 0x82618938;
    'dispatch: loop {
        match pc {
            0x82618938 => {
    //   block [0x82618938..0x826189A4)
	// 82618938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261893C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261894C: 38EBA114  addi r7, r11, -0x5eec
	ctx.r[7].s64 = ctx.r[11].s64 + -24300;
	// 82618950: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82618954: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82618958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261895C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618968: 386AC86C  addi r3, r10, -0x3794
	ctx.r[3].s64 = ctx.r[10].s64 + -14228;
	// 8261896C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261897C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261898C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618990: 4BE4E491  bl 0x82466e20
	ctx.lr = 0x82618994;
	sub_82466E20(ctx, base);
	// 82618994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261899C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826189A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826189A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826189A8 size=108
    let mut pc: u32 = 0x826189A8;
    'dispatch: loop {
        match pc {
            0x826189A8 => {
    //   block [0x826189A8..0x82618A14)
	// 826189A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826189AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826189B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826189B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826189B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826189BC: 38EBA130  addi r7, r11, -0x5ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -24272;
	// 826189C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826189C4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826189C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826189CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826189D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826189D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826189D8: 386AC89C  addi r3, r10, -0x3764
	ctx.r[3].s64 = ctx.r[10].s64 + -14180;
	// 826189DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826189E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826189E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826189E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826189EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826189F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826189F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826189F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826189FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618A00: 4BE4E421  bl 0x82466e20
	ctx.lr = 0x82618A04;
	sub_82466E20(ctx, base);
	// 82618A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618A18 size=116
    let mut pc: u32 = 0x82618A18;
    'dispatch: loop {
        match pc {
            0x82618A18 => {
    //   block [0x82618A18..0x82618A8C)
	// 82618A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618A24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618A2C: 390BA178  addi r8, r11, -0x5e88
	ctx.r[8].s64 = ctx.r[11].s64 + -24200;
	// 82618A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618A34: 392AFEF0  addi r9, r10, -0x110
	ctx.r[9].s64 = ctx.r[10].s64 + -272;
	// 82618A38: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618A3C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82618A40: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618A44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618A4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618A5C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618A60: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82618A64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618A68: 386BC8CC  addi r3, r11, -0x3734
	ctx.r[3].s64 = ctx.r[11].s64 + -14132;
	// 82618A6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618A70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618A78: 4BE4E3A9  bl 0x82466e20
	ctx.lr = 0x82618A7C;
	sub_82466E20(ctx, base);
	// 82618A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618A90 size=24
    let mut pc: u32 = 0x82618A90;
    'dispatch: loop {
        match pc {
            0x82618A90 => {
    //   block [0x82618A90..0x82618AA8)
	// 82618A90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618A94: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618A98: 394AE308  addi r10, r10, -0x1cf8
	ctx.r[10].s64 = ctx.r[10].s64 + -7416;
	// 82618A9C: 816BA190  lwz r11, -0x5e70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24176 as u32) ) } as u64;
	// 82618AA0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82618AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618AA8 size=116
    let mut pc: u32 = 0x82618AA8;
    'dispatch: loop {
        match pc {
            0x82618AA8 => {
    //   block [0x82618AA8..0x82618B1C)
	// 82618AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618AB4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618ABC: 390BE308  addi r8, r11, -0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -7416;
	// 82618AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618AC4: 392AFF60  addi r9, r10, -0xa0
	ctx.r[9].s64 = ctx.r[10].s64 + -160;
	// 82618AC8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618ACC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82618AD0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618AD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618ADC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618AEC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618AF0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 82618AF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618AF8: 386BC8FC  addi r3, r11, -0x3704
	ctx.r[3].s64 = ctx.r[11].s64 + -14084;
	// 82618AFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82618B00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618B08: 4BE4E319  bl 0x82466e20
	ctx.lr = 0x82618B0C;
	sub_82466E20(ctx, base);
	// 82618B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618B20 size=108
    let mut pc: u32 = 0x82618B20;
    'dispatch: loop {
        match pc {
            0x82618B20 => {
    //   block [0x82618B20..0x82618B8C)
	// 82618B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618B2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618B34: 38EBA1A0  addi r7, r11, -0x5e60
	ctx.r[7].s64 = ctx.r[11].s64 + -24160;
	// 82618B38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82618B3C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 82618B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618B44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618B50: 386AC92C  addi r3, r10, -0x36d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14036;
	// 82618B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618B78: 4BE4E2A9  bl 0x82466e20
	ctx.lr = 0x82618B7C;
	sub_82466E20(ctx, base);
	// 82618B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618B90 size=112
    let mut pc: u32 = 0x82618B90;
    'dispatch: loop {
        match pc {
            0x82618B90 => {
    //   block [0x82618B90..0x82618C00)
	// 82618B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618BA0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618BA4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618BAC: 390BA1D0  addi r8, r11, -0x5e30
	ctx.r[8].s64 = ctx.r[11].s64 + -24112;
	// 82618BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618BB4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82618BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618BC8: 386AC95C  addi r3, r10, -0x36a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13988;
	// 82618BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618BEC: 4BE4E235  bl 0x82466e20
	ctx.lr = 0x82618BF0;
	sub_82466E20(ctx, base);
	// 82618BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618C00 size=112
    let mut pc: u32 = 0x82618C00;
    'dispatch: loop {
        match pc {
            0x82618C00 => {
    //   block [0x82618C00..0x82618C70)
	// 82618C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618C0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618C14: 392AFFB8  addi r9, r10, -0x48
	ctx.r[9].s64 = ctx.r[10].s64 + -72;
	// 82618C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618C1C: 390BA1F0  addi r8, r11, -0x5e10
	ctx.r[8].s64 = ctx.r[11].s64 + -24080;
	// 82618C20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82618C24: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 82618C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618C38: 386AC98C  addi r3, r10, -0x3674
	ctx.r[3].s64 = ctx.r[10].s64 + -13940;
	// 82618C3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618C40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618C5C: 4BE4E1C5  bl 0x82466e20
	ctx.lr = 0x82618C60;
	sub_82466E20(ctx, base);
	// 82618C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618C70 size=112
    let mut pc: u32 = 0x82618C70;
    'dispatch: loop {
        match pc {
            0x82618C70 => {
    //   block [0x82618C70..0x82618CE0)
	// 82618C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618C80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618C84: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618C8C: 390BA238  addi r8, r11, -0x5dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24008;
	// 82618C90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618C94: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82618C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618C9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618CA8: 386AC9BC  addi r3, r10, -0x3644
	ctx.r[3].s64 = ctx.r[10].s64 + -13892;
	// 82618CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618CCC: 4BE4E155  bl 0x82466e20
	ctx.lr = 0x82618CD0;
	sub_82466E20(ctx, base);
	// 82618CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618CE0 size=112
    let mut pc: u32 = 0x82618CE0;
    'dispatch: loop {
        match pc {
            0x82618CE0 => {
    //   block [0x82618CE0..0x82618D50)
	// 82618CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618CEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618CF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618CF4: 392AFFE4  addi r9, r10, -0x1c
	ctx.r[9].s64 = ctx.r[10].s64 + -28;
	// 82618CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618CFC: 390BA250  addi r8, r11, -0x5db0
	ctx.r[8].s64 = ctx.r[11].s64 + -23984;
	// 82618D00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82618D04: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 82618D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618D18: 386AC9EC  addi r3, r10, -0x3614
	ctx.r[3].s64 = ctx.r[10].s64 + -13844;
	// 82618D1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618D20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618D3C: 4BE4E0E5  bl 0x82466e20
	ctx.lr = 0x82618D40;
	sub_82466E20(ctx, base);
	// 82618D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618D50 size=112
    let mut pc: u32 = 0x82618D50;
    'dispatch: loop {
        match pc {
            0x82618D50 => {
    //   block [0x82618D50..0x82618DC0)
	// 82618D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D60: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618D64: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618D6C: 390BA2E0  addi r8, r11, -0x5d20
	ctx.r[8].s64 = ctx.r[11].s64 + -23840;
	// 82618D70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618D74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82618D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618D88: 386ACA1C  addi r3, r10, -0x35e4
	ctx.r[3].s64 = ctx.r[10].s64 + -13796;
	// 82618D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618DAC: 4BE4E075  bl 0x82466e20
	ctx.lr = 0x82618DB0;
	sub_82466E20(ctx, base);
	// 82618DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618DC0 size=112
    let mut pc: u32 = 0x82618DC0;
    'dispatch: loop {
        match pc {
            0x82618DC0 => {
    //   block [0x82618DC0..0x82618E30)
	// 82618DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618DD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618DD4: 38AACA7C  addi r5, r10, -0x3584
	ctx.r[5].s64 = ctx.r[10].s64 + -13700;
	// 82618DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618DDC: 390BA2F8  addi r8, r11, -0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + -23816;
	// 82618DE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82618DE4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82618DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618DF8: 386ACA4C  addi r3, r10, -0x35b4
	ctx.r[3].s64 = ctx.r[10].s64 + -13748;
	// 82618DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618E1C: 4BE4E005  bl 0x82466e20
	ctx.lr = 0x82618E20;
	sub_82466E20(ctx, base);
	// 82618E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618E30 size=100
    let mut pc: u32 = 0x82618E30;
    'dispatch: loop {
        match pc {
            0x82618E30 => {
    //   block [0x82618E30..0x82618E94)
	// 82618E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618E44: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618E50: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82618E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618E64: 386ACA7C  addi r3, r10, -0x3584
	ctx.r[3].s64 = ctx.r[10].s64 + -13700;
	// 82618E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618E6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618E70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82618E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618E78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82618E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618E80: 4BE4DFA1  bl 0x82466e20
	ctx.lr = 0x82618E84;
	sub_82466E20(ctx, base);
	// 82618E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618E98 size=24
    let mut pc: u32 = 0x82618E98;
    'dispatch: loop {
        match pc {
            0x82618E98 => {
    //   block [0x82618E98..0x82618EB0)
	// 82618E98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618E9C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618EA0: 394AE3E0  addi r10, r10, -0x1c20
	ctx.r[10].s64 = ctx.r[10].s64 + -7200;
	// 82618EA4: 816BA370  lwz r11, -0x5c90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23696 as u32) ) } as u64;
	// 82618EA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82618EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618EB0 size=116
    let mut pc: u32 = 0x82618EB0;
    'dispatch: loop {
        match pc {
            0x82618EB0 => {
    //   block [0x82618EB0..0x82618F24)
	// 82618EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618EBC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618EC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618EC4: 390BE3E0  addi r8, r11, -0x1c20
	ctx.r[8].s64 = ctx.r[11].s64 + -7200;
	// 82618EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618ECC: 392A0020  addi r9, r10, 0x20
	ctx.r[9].s64 = ctx.r[10].s64 + 32;
	// 82618ED0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618ED4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82618ED8: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618EDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618EE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618EF4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618EF8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82618EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618F00: 386BCAAC  addi r3, r11, -0x3554
	ctx.r[3].s64 = ctx.r[11].s64 + -13652;
	// 82618F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618F08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618F10: 4BE4DF11  bl 0x82466e20
	ctx.lr = 0x82618F14;
	sub_82466E20(ctx, base);
	// 82618F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618F28 size=108
    let mut pc: u32 = 0x82618F28;
    'dispatch: loop {
        match pc {
            0x82618F28 => {
    //   block [0x82618F28..0x82618F94)
	// 82618F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618F34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618F3C: 38EBA374  addi r7, r11, -0x5c8c
	ctx.r[7].s64 = ctx.r[11].s64 + -23692;
	// 82618F40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82618F44: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 82618F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618F4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618F58: 386ACADC  addi r3, r10, -0x3524
	ctx.r[3].s64 = ctx.r[10].s64 + -13604;
	// 82618F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618F80: 4BE4DEA1  bl 0x82466e20
	ctx.lr = 0x82618F84;
	sub_82466E20(ctx, base);
	// 82618F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618F98 size=112
    let mut pc: u32 = 0x82618F98;
    'dispatch: loop {
        match pc {
            0x82618F98 => {
    //   block [0x82618F98..0x82619008)
	// 82618F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618FA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618FAC: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618FB4: 390BA3A4  addi r8, r11, -0x5c5c
	ctx.r[8].s64 = ctx.r[11].s64 + -23644;
	// 82618FB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618FBC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82618FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618FD0: 386ACB0C  addi r3, r10, -0x34f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13556;
	// 82618FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618FF4: 4BE4DE2D  bl 0x82466e20
	ctx.lr = 0x82618FF8;
	sub_82466E20(ctx, base);
	// 82618FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619008 size=112
    let mut pc: u32 = 0x82619008;
    'dispatch: loop {
        match pc {
            0x82619008 => {
    //   block [0x82619008..0x82619078)
	// 82619008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261900C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619014: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619018: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261901C: 392A0044  addi r9, r10, 0x44
	ctx.r[9].s64 = ctx.r[10].s64 + 68;
	// 82619020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619024: 390BA3C0  addi r8, r11, -0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + -23616;
	// 82619028: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261902C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 82619030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261903C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619040: 386ACB3C  addi r3, r10, -0x34c4
	ctx.r[3].s64 = ctx.r[10].s64 + -13508;
	// 82619044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261904C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261905C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619064: 4BE4DDBD  bl 0x82466e20
	ctx.lr = 0x82619068;
	sub_82466E20(ctx, base);
	// 82619068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261906C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619078 size=112
    let mut pc: u32 = 0x82619078;
    'dispatch: loop {
        match pc {
            0x82619078 => {
    //   block [0x82619078..0x826190E8)
	// 82619078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619088: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261908C: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619094: 390BA468  addi r8, r11, -0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + -23448;
	// 82619098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261909C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826190A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826190A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826190A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826190AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826190B0: 386ACB6C  addi r3, r10, -0x3494
	ctx.r[3].s64 = ctx.r[10].s64 + -13460;
	// 826190B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826190B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826190BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826190C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826190C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826190C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826190CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826190D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826190D4: 4BE4DD4D  bl 0x82466e20
	ctx.lr = 0x826190D8;
	sub_82466E20(ctx, base);
	// 826190D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826190DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826190E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826190E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826190E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826190E8 size=112
    let mut pc: u32 = 0x826190E8;
    'dispatch: loop {
        match pc {
            0x826190E8 => {
    //   block [0x826190E8..0x82619158)
	// 826190E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826190EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826190F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826190F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826190F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826190FC: 392A009C  addi r9, r10, 0x9c
	ctx.r[9].s64 = ctx.r[10].s64 + 156;
	// 82619100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619104: 390BA488  addi r8, r11, -0x5b78
	ctx.r[8].s64 = ctx.r[11].s64 + -23416;
	// 82619108: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261910C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82619110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261911C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619120: 386ACB9C  addi r3, r10, -0x3464
	ctx.r[3].s64 = ctx.r[10].s64 + -13412;
	// 82619124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261912C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261913C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619144: 4BE4DCDD  bl 0x82466e20
	ctx.lr = 0x82619148;
	sub_82466E20(ctx, base);
	// 82619148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261914C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619158 size=116
    let mut pc: u32 = 0x82619158;
    'dispatch: loop {
        match pc {
            0x82619158 => {
    //   block [0x82619158..0x826191CC)
	// 82619158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261915C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619164: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261916C: 390BA530  addi r8, r11, -0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -23248;
	// 82619170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619174: 392A0070  addi r9, r10, 0x70
	ctx.r[9].s64 = ctx.r[10].s64 + 112;
	// 82619178: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261917C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82619180: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619184: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261918C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261919C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826191A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826191A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826191A8: 386BCBCC  addi r3, r11, -0x3434
	ctx.r[3].s64 = ctx.r[11].s64 + -13364;
	// 826191AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826191B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826191B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826191B8: 4BE4DC69  bl 0x82466e20
	ctx.lr = 0x826191BC;
	sub_82466E20(ctx, base);
	// 826191BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826191C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826191C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826191C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826191D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826191D0 size=108
    let mut pc: u32 = 0x826191D0;
    'dispatch: loop {
        match pc {
            0x826191D0 => {
    //   block [0x826191D0..0x8261923C)
	// 826191D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826191D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826191D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826191DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826191E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826191E4: 38EBA548  addi r7, r11, -0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -23224;
	// 826191E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826191EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826191F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826191F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826191F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826191FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619200: 386ACBFC  addi r3, r10, -0x3404
	ctx.r[3].s64 = ctx.r[10].s64 + -13316;
	// 82619204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261920C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261921C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619228: 4BE4DBF9  bl 0x82466e20
	ctx.lr = 0x8261922C;
	sub_82466E20(ctx, base);
	// 8261922C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619240 size=112
    let mut pc: u32 = 0x82619240;
    'dispatch: loop {
        match pc {
            0x82619240 => {
    //   block [0x82619240..0x826192B0)
	// 82619240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261924C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619250: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619254: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261925C: 390BA578  addi r8, r11, -0x5a88
	ctx.r[8].s64 = ctx.r[11].s64 + -23176;
	// 82619260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619264: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82619268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261926C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619278: 386ACC2C  addi r3, r10, -0x33d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13268;
	// 8261927C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261928C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261929C: 4BE4DB85  bl 0x82466e20
	ctx.lr = 0x826192A0;
	sub_82466E20(ctx, base);
	// 826192A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826192A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826192A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826192AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826192B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826192B0 size=112
    let mut pc: u32 = 0x826192B0;
    'dispatch: loop {
        match pc {
            0x826192B0 => {
    //   block [0x826192B0..0x82619320)
	// 826192B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826192B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826192B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826192BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826192C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826192C4: 392A00D0  addi r9, r10, 0xd0
	ctx.r[9].s64 = ctx.r[10].s64 + 208;
	// 826192C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826192CC: 390BA598  addi r8, r11, -0x5a68
	ctx.r[8].s64 = ctx.r[11].s64 + -23144;
	// 826192D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826192D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826192D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826192DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826192E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826192E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826192E8: 386ACC5C  addi r3, r10, -0x33a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13220;
	// 826192EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826192F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826192F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826192F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826192FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261930C: 4BE4DB15  bl 0x82466e20
	ctx.lr = 0x82619310;
	sub_82466E20(ctx, base);
	// 82619310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261931C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619320 size=112
    let mut pc: u32 = 0x82619320;
    'dispatch: loop {
        match pc {
            0x82619320 => {
    //   block [0x82619320..0x82619390)
	// 82619320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261932C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619334: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261933C: 390BA640  addi r8, r11, -0x59c0
	ctx.r[8].s64 = ctx.r[11].s64 + -22976;
	// 82619340: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619344: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82619348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261934C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619358: 386ACC8C  addi r3, r10, -0x3374
	ctx.r[3].s64 = ctx.r[10].s64 + -13172;
	// 8261935C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261936C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261937C: 4BE4DAA5  bl 0x82466e20
	ctx.lr = 0x82619380;
	sub_82466E20(ctx, base);
	// 82619380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261938C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619390 size=112
    let mut pc: u32 = 0x82619390;
    'dispatch: loop {
        match pc {
            0x82619390 => {
    //   block [0x82619390..0x82619400)
	// 82619390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261939C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826193A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826193A4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826193A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826193AC: 390BA688  addi r8, r11, -0x5978
	ctx.r[8].s64 = ctx.r[11].s64 + -22904;
	// 826193B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826193B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826193B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826193BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826193C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826193C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826193C8: 386ACCBC  addi r3, r10, -0x3344
	ctx.r[3].s64 = ctx.r[10].s64 + -13124;
	// 826193CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826193D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826193D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826193D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826193DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826193E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826193E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826193E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826193EC: 4BE4DA35  bl 0x82466e20
	ctx.lr = 0x826193F0;
	sub_82466E20(ctx, base);
	// 826193F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826193F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826193F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826193FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619400 size=100
    let mut pc: u32 = 0x82619400;
    'dispatch: loop {
        match pc {
            0x82619400 => {
    //   block [0x82619400..0x82619464)
	// 82619400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261940C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619414: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619420: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82619424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261942C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619434: 386ACCEC  addi r3, r10, -0x3314
	ctx.r[3].s64 = ctx.r[10].s64 + -13076;
	// 82619438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261943C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82619444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261944C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619450: 4BE4D9D1  bl 0x82466e20
	ctx.lr = 0x82619454;
	sub_82466E20(ctx, base);
	// 82619454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261945C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619468 size=112
    let mut pc: u32 = 0x82619468;
    'dispatch: loop {
        match pc {
            0x82619468 => {
    //   block [0x82619468..0x826194D8)
	// 82619468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619478: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261947C: 38AAC8FC  addi r5, r10, -0x3704
	ctx.r[5].s64 = ctx.r[10].s64 + -14084;
	// 82619480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619484: 390BA748  addi r8, r11, -0x58b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22712;
	// 82619488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261948C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82619490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261949C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826194A0: 386ACD1C  addi r3, r10, -0x32e4
	ctx.r[3].s64 = ctx.r[10].s64 + -13028;
	// 826194A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826194A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826194AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826194B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826194B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826194B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826194BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826194C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826194C4: 4BE4D95D  bl 0x82466e20
	ctx.lr = 0x826194C8;
	sub_82466E20(ctx, base);
	// 826194C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826194CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826194D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826194D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826194D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826194D8 size=112
    let mut pc: u32 = 0x826194D8;
    'dispatch: loop {
        match pc {
            0x826194D8 => {
    //   block [0x826194D8..0x82619548)
	// 826194D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826194DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826194E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826194E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826194E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826194EC: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 826194F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826194F4: 390BA778  addi r8, r11, -0x5888
	ctx.r[8].s64 = ctx.r[11].s64 + -22664;
	// 826194F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826194FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82619500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261950C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619510: 386ACD4C  addi r3, r10, -0x32b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12980;
	// 82619514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261952C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619534: 4BE4D8ED  bl 0x82466e20
	ctx.lr = 0x82619538;
	sub_82466E20(ctx, base);
	// 82619538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261953C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619548 size=108
    let mut pc: u32 = 0x82619548;
    'dispatch: loop {
        match pc {
            0x82619548 => {
    //   block [0x82619548..0x826195B4)
	// 82619548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261954C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619554: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261955C: 38EBA790  addi r7, r11, -0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + -22640;
	// 82619560: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82619564: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82619568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261956C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619578: 386ACD7C  addi r3, r10, -0x3284
	ctx.r[3].s64 = ctx.r[10].s64 + -12932;
	// 8261957C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261958C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261959C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826195A0: 4BE4D881  bl 0x82466e20
	ctx.lr = 0x826195A4;
	sub_82466E20(ctx, base);
	// 826195A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826195A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826195AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826195B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826195B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826195B8 size=112
    let mut pc: u32 = 0x826195B8;
    'dispatch: loop {
        match pc {
            0x826195B8 => {
    //   block [0x826195B8..0x82619628)
	// 826195B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826195BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826195C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826195C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826195C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826195CC: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 826195D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826195D4: 390BA7C0  addi r8, r11, -0x5840
	ctx.r[8].s64 = ctx.r[11].s64 + -22592;
	// 826195D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826195DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826195E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826195E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826195E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826195EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826195F0: 386ACDAC  addi r3, r10, -0x3254
	ctx.r[3].s64 = ctx.r[10].s64 + -12884;
	// 826195F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826195F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826195FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261960C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619614: 4BE4D80D  bl 0x82466e20
	ctx.lr = 0x82619618;
	sub_82466E20(ctx, base);
	// 82619618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261961C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619628 size=112
    let mut pc: u32 = 0x82619628;
    'dispatch: loop {
        match pc {
            0x82619628 => {
    //   block [0x82619628..0x82619698)
	// 82619628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619634: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619638: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261963C: 392A00FC  addi r9, r10, 0xfc
	ctx.r[9].s64 = ctx.r[10].s64 + 252;
	// 82619640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619644: 390BA850  addi r8, r11, -0x57b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22448;
	// 82619648: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261964C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 82619650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261965C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619660: 386ACDDC  addi r3, r10, -0x3224
	ctx.r[3].s64 = ctx.r[10].s64 + -12836;
	// 82619664: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619668: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261966C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261967C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619684: 4BE4D79D  bl 0x82466e20
	ctx.lr = 0x82619688;
	sub_82466E20(ctx, base);
	// 82619688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261968C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619698 size=112
    let mut pc: u32 = 0x82619698;
    'dispatch: loop {
        match pc {
            0x82619698 => {
    //   block [0x82619698..0x82619708)
	// 82619698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261969C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826196A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826196A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826196A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826196AC: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826196B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826196B4: 390BA898  addi r8, r11, -0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + -22376;
	// 826196B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826196BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826196C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826196C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826196C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826196CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826196D0: 386ACE0C  addi r3, r10, -0x31f4
	ctx.r[3].s64 = ctx.r[10].s64 + -12788;
	// 826196D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826196D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826196DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826196E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826196E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826196E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826196EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826196F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826196F4: 4BE4D72D  bl 0x82466e20
	ctx.lr = 0x826196F8;
	sub_82466E20(ctx, base);
	// 826196F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826196FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619708 size=108
    let mut pc: u32 = 0x82619708;
    'dispatch: loop {
        match pc {
            0x82619708 => {
    //   block [0x82619708..0x82619774)
	// 82619708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261971C: 38EBA8B0  addi r7, r11, -0x5750
	ctx.r[7].s64 = ctx.r[11].s64 + -22352;
	// 82619720: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82619724: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82619728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261972C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619738: 386ACE3C  addi r3, r10, -0x31c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12740;
	// 8261973C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261975C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619760: 4BE4D6C1  bl 0x82466e20
	ctx.lr = 0x82619764;
	sub_82466E20(ctx, base);
	// 82619764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261976C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619778 size=116
    let mut pc: u32 = 0x82619778;
    'dispatch: loop {
        match pc {
            0x82619778 => {
    //   block [0x82619778..0x826197EC)
	// 82619778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261977C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619784: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619788: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8261978C: 390AA940  addi r8, r10, -0x56c0
	ctx.r[8].s64 = ctx.r[10].s64 + -22208;
	// 82619790: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619794: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82619798: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 8261979C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826197A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826197A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826197A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826197AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826197B0: 396B0110  addi r11, r11, 0x110
	ctx.r[11].s64 = ctx.r[11].s64 + 272;
	// 826197B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826197B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826197BC: 386ACE6C  addi r3, r10, -0x3194
	ctx.r[3].s64 = ctx.r[10].s64 + -12692;
	// 826197C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826197C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826197C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826197CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826197D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826197D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826197D8: 4BE4D649  bl 0x82466e20
	ctx.lr = 0x826197DC;
	sub_82466E20(ctx, base);
	// 826197DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826197E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826197E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826197E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826197F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826197F0 size=112
    let mut pc: u32 = 0x826197F0;
    'dispatch: loop {
        match pc {
            0x826197F0 => {
    //   block [0x826197F0..0x82619860)
	// 826197F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826197F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826197F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826197FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619800: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619804: 392A015C  addi r9, r10, 0x15c
	ctx.r[9].s64 = ctx.r[10].s64 + 348;
	// 82619808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261980C: 390BAA20  addi r8, r11, -0x55e0
	ctx.r[8].s64 = ctx.r[11].s64 + -21984;
	// 82619810: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82619814: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82619818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261981C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619828: 386ACE9C  addi r3, r10, -0x3164
	ctx.r[3].s64 = ctx.r[10].s64 + -12644;
	// 8261982C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619830: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261983C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261984C: 4BE4D5D5  bl 0x82466e20
	ctx.lr = 0x82619850;
	sub_82466E20(ctx, base);
	// 82619850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261985C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619860 size=112
    let mut pc: u32 = 0x82619860;
    'dispatch: loop {
        match pc {
            0x82619860 => {
    //   block [0x82619860..0x826198D0)
	// 82619860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261986C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619870: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619874: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261987C: 390BAA80  addi r8, r11, -0x5580
	ctx.r[8].s64 = ctx.r[11].s64 + -21888;
	// 82619880: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619884: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82619888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261988C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619898: 386ACECC  addi r3, r10, -0x3134
	ctx.r[3].s64 = ctx.r[10].s64 + -12596;
	// 8261989C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826198A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826198A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826198A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826198AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826198B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826198B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826198B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826198BC: 4BE4D565  bl 0x82466e20
	ctx.lr = 0x826198C0;
	sub_82466E20(ctx, base);
	// 826198C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826198C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826198C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826198CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826198D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826198D0 size=108
    let mut pc: u32 = 0x826198D0;
    'dispatch: loop {
        match pc {
            0x826198D0 => {
    //   block [0x826198D0..0x8261993C)
	// 826198D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826198D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826198D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826198DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826198E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826198E4: 38EBAA98  addi r7, r11, -0x5568
	ctx.r[7].s64 = ctx.r[11].s64 + -21864;
	// 826198E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826198EC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826198F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826198F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826198F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826198FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619900: 386ACEFC  addi r3, r10, -0x3104
	ctx.r[3].s64 = ctx.r[10].s64 + -12548;
	// 82619904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261990C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619928: 4BE4D4F9  bl 0x82466e20
	ctx.lr = 0x8261992C;
	sub_82466E20(ctx, base);
	// 8261992C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619940 size=112
    let mut pc: u32 = 0x82619940;
    'dispatch: loop {
        match pc {
            0x82619940 => {
    //   block [0x82619940..0x826199B0)
	// 82619940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261994C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619950: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619954: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 82619958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261995C: 390BAAE0  addi r8, r11, -0x5520
	ctx.r[8].s64 = ctx.r[11].s64 + -21792;
	// 82619960: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82619964: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82619968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261996C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619978: 386ACF2C  addi r3, r10, -0x30d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12500;
	// 8261997C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261999C: 4BE4D485  bl 0x82466e20
	ctx.lr = 0x826199A0;
	sub_82466E20(ctx, base);
	// 826199A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826199A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826199A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826199AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826199B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826199B0 size=112
    let mut pc: u32 = 0x826199B0;
    'dispatch: loop {
        match pc {
            0x826199B0 => {
    //   block [0x826199B0..0x82619A20)
	// 826199B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826199B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826199B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826199BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826199C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826199C4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826199C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826199CC: 390BAB58  addi r8, r11, -0x54a8
	ctx.r[8].s64 = ctx.r[11].s64 + -21672;
	// 826199D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826199D4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826199D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826199DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826199E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826199E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826199E8: 386ACF5C  addi r3, r10, -0x30a4
	ctx.r[3].s64 = ctx.r[10].s64 + -12452;
	// 826199EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826199F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826199F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826199F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826199FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619A0C: 4BE4D415  bl 0x82466e20
	ctx.lr = 0x82619A10;
	sub_82466E20(ctx, base);
	// 82619A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619A20 size=108
    let mut pc: u32 = 0x82619A20;
    'dispatch: loop {
        match pc {
            0x82619A20 => {
    //   block [0x82619A20..0x82619A8C)
	// 82619A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619A2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619A34: 38EBAB88  addi r7, r11, -0x5478
	ctx.r[7].s64 = ctx.r[11].s64 + -21624;
	// 82619A38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82619A3C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82619A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619A50: 386ACF8C  addi r3, r10, -0x3074
	ctx.r[3].s64 = ctx.r[10].s64 + -12404;
	// 82619A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619A78: 4BE4D3A9  bl 0x82466e20
	ctx.lr = 0x82619A7C;
	sub_82466E20(ctx, base);
	// 82619A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619A90 size=108
    let mut pc: u32 = 0x82619A90;
    'dispatch: loop {
        match pc {
            0x82619A90 => {
    //   block [0x82619A90..0x82619AFC)
	// 82619A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619A9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619AA4: 38EBABE8  addi r7, r11, -0x5418
	ctx.r[7].s64 = ctx.r[11].s64 + -21528;
	// 82619AA8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82619AAC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82619AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619AC0: 386ACFBC  addi r3, r10, -0x3044
	ctx.r[3].s64 = ctx.r[10].s64 + -12356;
	// 82619AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619AE8: 4BE4D339  bl 0x82466e20
	ctx.lr = 0x82619AEC;
	sub_82466E20(ctx, base);
	// 82619AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619B00 size=112
    let mut pc: u32 = 0x82619B00;
    'dispatch: loop {
        match pc {
            0x82619B00 => {
    //   block [0x82619B00..0x82619B70)
	// 82619B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619B10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B14: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619B1C: 390BAC60  addi r8, r11, -0x53a0
	ctx.r[8].s64 = ctx.r[11].s64 + -21408;
	// 82619B20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619B24: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82619B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619B38: 386ACFEC  addi r3, r10, -0x3014
	ctx.r[3].s64 = ctx.r[10].s64 + -12308;
	// 82619B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619B5C: 4BE4D2C5  bl 0x82466e20
	ctx.lr = 0x82619B60;
	sub_82466E20(ctx, base);
	// 82619B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82619B70 size=24
    let mut pc: u32 = 0x82619B70;
    'dispatch: loop {
        match pc {
            0x82619B70 => {
    //   block [0x82619B70..0x82619B88)
	// 82619B70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B74: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619B78: 394AE458  addi r10, r10, -0x1ba8
	ctx.r[10].s64 = ctx.r[10].s64 + -7080;
	// 82619B7C: 816BAA1C  lwz r11, -0x55e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21988 as u32) ) } as u64;
	// 82619B80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82619B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619B88 size=116
    let mut pc: u32 = 0x82619B88;
    'dispatch: loop {
        match pc {
            0x82619B88 => {
    //   block [0x82619B88..0x82619BFC)
	// 82619B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619B94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619B9C: 390BE458  addi r8, r11, -0x1ba8
	ctx.r[8].s64 = ctx.r[11].s64 + -7080;
	// 82619BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619BA4: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82619BA8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619BAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82619BB0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82619BB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619BBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619BCC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82619BD0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82619BD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619BD8: 386BD01C  addi r3, r11, -0x2fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -12260;
	// 82619BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619BE8: 4BE4D239  bl 0x82466e20
	ctx.lr = 0x82619BEC;
	sub_82466E20(ctx, base);
	// 82619BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619C00 size=112
    let mut pc: u32 = 0x82619C00;
    'dispatch: loop {
        match pc {
            0x82619C00 => {
    //   block [0x82619C00..0x82619C70)
	// 82619C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619C0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C14: 38AAD01C  addi r5, r10, -0x2fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -12260;
	// 82619C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619C1C: 390BACA8  addi r8, r11, -0x5358
	ctx.r[8].s64 = ctx.r[11].s64 + -21336;
	// 82619C20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82619C24: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82619C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619C38: 386AD04C  addi r3, r10, -0x2fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -12212;
	// 82619C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619C5C: 4BE4D1C5  bl 0x82466e20
	ctx.lr = 0x82619C60;
	sub_82466E20(ctx, base);
	// 82619C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82619C70 size=24
    let mut pc: u32 = 0x82619C70;
    'dispatch: loop {
        match pc {
            0x82619C70 => {
    //   block [0x82619C70..0x82619C88)
	// 82619C70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C74: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619C78: 394AE470  addi r10, r10, -0x1b90
	ctx.r[10].s64 = ctx.r[10].s64 + -7056;
	// 82619C7C: 816BACD8  lwz r11, -0x5328(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21288 as u32) ) } as u64;
	// 82619C80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82619C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619C88 size=116
    let mut pc: u32 = 0x82619C88;
    'dispatch: loop {
        match pc {
            0x82619C88 => {
    //   block [0x82619C88..0x82619CFC)
	// 82619C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619C94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619C9C: 390BE470  addi r8, r11, -0x1b90
	ctx.r[8].s64 = ctx.r[11].s64 + -7056;
	// 82619CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619CA4: 392A01DC  addi r9, r10, 0x1dc
	ctx.r[9].s64 = ctx.r[10].s64 + 476;
	// 82619CA8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619CAC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82619CB0: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619CB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619CBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619CCC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82619CD0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 82619CD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619CD8: 386BD07C  addi r3, r11, -0x2f84
	ctx.r[3].s64 = ctx.r[11].s64 + -12164;
	// 82619CDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619CE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619CE8: 4BE4D139  bl 0x82466e20
	ctx.lr = 0x82619CEC;
	sub_82466E20(ctx, base);
	// 82619CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619D00 size=112
    let mut pc: u32 = 0x82619D00;
    'dispatch: loop {
        match pc {
            0x82619D00 => {
    //   block [0x82619D00..0x82619D70)
	// 82619D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619D14: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619D1C: 390BACE0  addi r8, r11, -0x5320
	ctx.r[8].s64 = ctx.r[11].s64 + -21280;
	// 82619D20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82619D24: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82619D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619D2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619D38: 386AD0AC  addi r3, r10, -0x2f54
	ctx.r[3].s64 = ctx.r[10].s64 + -12116;
	// 82619D3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619D5C: 4BE4D0C5  bl 0x82466e20
	ctx.lr = 0x82619D60;
	sub_82466E20(ctx, base);
	// 82619D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619D70 size=112
    let mut pc: u32 = 0x82619D70;
    'dispatch: loop {
        match pc {
            0x82619D70 => {
    //   block [0x82619D70..0x82619DE0)
	// 82619D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619D84: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619D8C: 390BAD40  addi r8, r11, -0x52c0
	ctx.r[8].s64 = ctx.r[11].s64 + -21184;
	// 82619D90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82619D94: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82619D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619D9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619DA8: 386AD0DC  addi r3, r10, -0x2f24
	ctx.r[3].s64 = ctx.r[10].s64 + -12068;
	// 82619DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619DCC: 4BE4D055  bl 0x82466e20
	ctx.lr = 0x82619DD0;
	sub_82466E20(ctx, base);
	// 82619DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619DE0 size=112
    let mut pc: u32 = 0x82619DE0;
    'dispatch: loop {
        match pc {
            0x82619DE0 => {
    //   block [0x82619DE0..0x82619E50)
	// 82619DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619DF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619DF4: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619DFC: 390BAD70  addi r8, r11, -0x5290
	ctx.r[8].s64 = ctx.r[11].s64 + -21136;
	// 82619E00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619E04: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82619E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619E0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619E18: 386AD10C  addi r3, r10, -0x2ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -12020;
	// 82619E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619E3C: 4BE4CFE5  bl 0x82466e20
	ctx.lr = 0x82619E40;
	sub_82466E20(ctx, base);
	// 82619E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619E50 size=108
    let mut pc: u32 = 0x82619E50;
    'dispatch: loop {
        match pc {
            0x82619E50 => {
    //   block [0x82619E50..0x82619EBC)
	// 82619E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619E5C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619E64: 38EBADB8  addi r7, r11, -0x5248
	ctx.r[7].s64 = ctx.r[11].s64 + -21064;
	// 82619E68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82619E6C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82619E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619E80: 386AD13C  addi r3, r10, -0x2ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -11972;
	// 82619E84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619EA8: 4BE4CF79  bl 0x82466e20
	ctx.lr = 0x82619EAC;
	sub_82466E20(ctx, base);
	// 82619EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619EC0 size=112
    let mut pc: u32 = 0x82619EC0;
    'dispatch: loop {
        match pc {
            0x82619EC0 => {
    //   block [0x82619EC0..0x82619F30)
	// 82619EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619ED0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619ED4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619EDC: 390BADE8  addi r8, r11, -0x5218
	ctx.r[8].s64 = ctx.r[11].s64 + -21016;
	// 82619EE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619EE4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82619EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619EEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619EF8: 386AD16C  addi r3, r10, -0x2e94
	ctx.r[3].s64 = ctx.r[10].s64 + -11924;
	// 82619EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619F1C: 4BE4CF05  bl 0x82466e20
	ctx.lr = 0x82619F20;
	sub_82466E20(ctx, base);
	// 82619F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619F30 size=108
    let mut pc: u32 = 0x82619F30;
    'dispatch: loop {
        match pc {
            0x82619F30 => {
    //   block [0x82619F30..0x82619F9C)
	// 82619F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619F3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619F44: 38EBAE00  addi r7, r11, -0x5200
	ctx.r[7].s64 = ctx.r[11].s64 + -20992;
	// 82619F48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82619F4C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82619F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619F54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619F60: 386AD19C  addi r3, r10, -0x2e64
	ctx.r[3].s64 = ctx.r[10].s64 + -11876;
	// 82619F64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619F88: 4BE4CE99  bl 0x82466e20
	ctx.lr = 0x82619F8C;
	sub_82466E20(ctx, base);
	// 82619F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619FA0 size=108
    let mut pc: u32 = 0x82619FA0;
    'dispatch: loop {
        match pc {
            0x82619FA0 => {
    //   block [0x82619FA0..0x8261A00C)
	// 82619FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619FAC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619FB4: 38EBAE48  addi r7, r11, -0x51b8
	ctx.r[7].s64 = ctx.r[11].s64 + -20920;
	// 82619FB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82619FBC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82619FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619FD0: 386AD1CC  addi r3, r10, -0x2e34
	ctx.r[3].s64 = ctx.r[10].s64 + -11828;
	// 82619FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619FF8: 4BE4CE29  bl 0x82466e20
	ctx.lr = 0x82619FFC;
	sub_82466E20(ctx, base);
	// 82619FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A010 size=116
    let mut pc: u32 = 0x8261A010;
    'dispatch: loop {
        match pc {
            0x8261A010 => {
    //   block [0x8261A010..0x8261A084)
	// 8261A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A01C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A020: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A024: 392B0210  addi r9, r11, 0x210
	ctx.r[9].s64 = ctx.r[11].s64 + 528;
	// 8261A028: 38AAD64C  addi r5, r10, -0x29b4
	ctx.r[5].s64 = ctx.r[10].s64 + -10676;
	// 8261A02C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A030: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8261A034: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8261A038: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A03C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8261A040: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A044: 396BAEA8  addi r11, r11, -0x5158
	ctx.r[11].s64 = ctx.r[11].s64 + -20824;
	// 8261A048: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261A04C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A050: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261A054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A058: 386AD1FC  addi r3, r10, -0x2e04
	ctx.r[3].s64 = ctx.r[10].s64 + -11780;
	// 8261A05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A060: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261A064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A068: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261A06C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A070: 4BE4CDB1  bl 0x82466e20
	ctx.lr = 0x8261A074;
	sub_82466E20(ctx, base);
	// 8261A074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A088 size=100
    let mut pc: u32 = 0x8261A088;
    'dispatch: loop {
        match pc {
            0x8261A088 => {
    //   block [0x8261A088..0x8261A0EC)
	// 8261A088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A09C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A0A8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8261A0AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A0BC: 386AD22C  addi r3, r10, -0x2dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -11732;
	// 8261A0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A0C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A0C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A0D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A0D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A0D8: 4BE4CD49  bl 0x82466e20
	ctx.lr = 0x8261A0DC;
	sub_82466E20(ctx, base);
	// 8261A0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A0F0 size=100
    let mut pc: u32 = 0x8261A0F0;
    'dispatch: loop {
        match pc {
            0x8261A0F0 => {
    //   block [0x8261A0F0..0x8261A154)
	// 8261A0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A0FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A104: 38AAD2BC  addi r5, r10, -0x2d44
	ctx.r[5].s64 = ctx.r[10].s64 + -11588;
	// 8261A108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A110: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8261A114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A124: 386AD25C  addi r3, r10, -0x2da4
	ctx.r[3].s64 = ctx.r[10].s64 + -11684;
	// 8261A128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A12C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A130: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A140: 4BE4CCE1  bl 0x82466e20
	ctx.lr = 0x8261A144;
	sub_82466E20(ctx, base);
	// 8261A144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A158 size=100
    let mut pc: u32 = 0x8261A158;
    'dispatch: loop {
        match pc {
            0x8261A158 => {
    //   block [0x8261A158..0x8261A1BC)
	// 8261A158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A16C: 38AAD1FC  addi r5, r10, -0x2e04
	ctx.r[5].s64 = ctx.r[10].s64 + -11780;
	// 8261A170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A178: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8261A17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A18C: 386AD28C  addi r3, r10, -0x2d74
	ctx.r[3].s64 = ctx.r[10].s64 + -11636;
	// 8261A190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A198: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A1A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A1A8: 4BE4CC79  bl 0x82466e20
	ctx.lr = 0x8261A1AC;
	sub_82466E20(ctx, base);
	// 8261A1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A1C0 size=104
    let mut pc: u32 = 0x8261A1C0;
    'dispatch: loop {
        match pc {
            0x8261A1C0 => {
    //   block [0x8261A1C0..0x8261A228)
	// 8261A1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A1CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261A1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A1D4: 392A0294  addi r9, r10, 0x294
	ctx.r[9].s64 = ctx.r[10].s64 + 660;
	// 8261A1D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A1E0: 38AAD22C  addi r5, r10, -0x2dd4
	ctx.r[5].s64 = ctx.r[10].s64 + -11732;
	// 8261A1E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A1F4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8261A1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A1FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A20C: 386AD2BC  addi r3, r10, -0x2d44
	ctx.r[3].s64 = ctx.r[10].s64 + -11588;
	// 8261A210: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A214: 4BE4CC0D  bl 0x82466e20
	ctx.lr = 0x8261A218;
	sub_82466E20(ctx, base);
	// 8261A218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A228 size=108
    let mut pc: u32 = 0x8261A228;
    'dispatch: loop {
        match pc {
            0x8261A228 => {
    //   block [0x8261A228..0x8261A294)
	// 8261A228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A234: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A23C: 38EBB05C  addi r7, r11, -0x4fa4
	ctx.r[7].s64 = ctx.r[11].s64 + -20388;
	// 8261A240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261A244: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8261A248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A24C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261A254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A258: 386AD2EC  addi r3, r10, -0x2d14
	ctx.r[3].s64 = ctx.r[10].s64 + -11540;
	// 8261A25C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261A260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261A280: 4BE4CBA1  bl 0x82466e20
	ctx.lr = 0x8261A284;
	sub_82466E20(ctx, base);
	// 8261A284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A298 size=112
    let mut pc: u32 = 0x8261A298;
    'dispatch: loop {
        match pc {
            0x8261A298 => {
    //   block [0x8261A298..0x8261A308)
	// 8261A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A2A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A2A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A2AC: 38AAD2BC  addi r5, r10, -0x2d44
	ctx.r[5].s64 = ctx.r[10].s64 + -11588;
	// 8261A2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A2B4: 390BB090  addi r8, r11, -0x4f70
	ctx.r[8].s64 = ctx.r[11].s64 + -20336;
	// 8261A2B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261A2BC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8261A2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A2D0: 386AD31C  addi r3, r10, -0x2ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -11492;
	// 8261A2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A2F4: 4BE4CB2D  bl 0x82466e20
	ctx.lr = 0x8261A2F8;
	sub_82466E20(ctx, base);
	// 8261A2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261A308 size=24
    let mut pc: u32 = 0x8261A308;
    'dispatch: loop {
        match pc {
            0x8261A308 => {
    //   block [0x8261A308..0x8261A320)
	// 8261A308: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A30C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A310: 394AE4E8  addi r10, r10, -0x1b18
	ctx.r[10].s64 = ctx.r[10].s64 + -6936;
	// 8261A314: 816BB08C  lwz r11, -0x4f74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20340 as u32) ) } as u64;
	// 8261A318: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261A31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A320 size=116
    let mut pc: u32 = 0x8261A320;
    'dispatch: loop {
        match pc {
            0x8261A320 => {
    //   block [0x8261A320..0x8261A394)
	// 8261A320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A32C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261A334: 390BE4E8  addi r8, r11, -0x1b18
	ctx.r[8].s64 = ctx.r[11].s64 + -6936;
	// 8261A338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A33C: 392A02F8  addi r9, r10, 0x2f8
	ctx.r[9].s64 = ctx.r[10].s64 + 760;
	// 8261A340: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A344: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261A348: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A34C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A354: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A364: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261A368: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8261A36C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261A370: 386BD34C  addi r3, r11, -0x2cb4
	ctx.r[3].s64 = ctx.r[11].s64 + -11444;
	// 8261A374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A378: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A380: 4BE4CAA1  bl 0x82466e20
	ctx.lr = 0x8261A384;
	sub_82466E20(ctx, base);
	// 8261A384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A398 size=100
    let mut pc: u32 = 0x8261A398;
    'dispatch: loop {
        match pc {
            0x8261A398 => {
    //   block [0x8261A398..0x8261A3FC)
	// 8261A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A3A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A3AC: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A3B8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8261A3BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A3CC: 386AD37C  addi r3, r10, -0x2c84
	ctx.r[3].s64 = ctx.r[10].s64 + -11396;
	// 8261A3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A3E8: 4BE4CA39  bl 0x82466e20
	ctx.lr = 0x8261A3EC;
	sub_82466E20(ctx, base);
	// 8261A3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A400 size=100
    let mut pc: u32 = 0x8261A400;
    'dispatch: loop {
        match pc {
            0x8261A400 => {
    //   block [0x8261A400..0x8261A464)
	// 8261A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A414: 38AAD3DC  addi r5, r10, -0x2c24
	ctx.r[5].s64 = ctx.r[10].s64 + -11300;
	// 8261A418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A420: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8261A424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A434: 386AD3AC  addi r3, r10, -0x2c54
	ctx.r[3].s64 = ctx.r[10].s64 + -11348;
	// 8261A438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A43C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A450: 4BE4C9D1  bl 0x82466e20
	ctx.lr = 0x8261A454;
	sub_82466E20(ctx, base);
	// 8261A454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A468 size=112
    let mut pc: u32 = 0x8261A468;
    'dispatch: loop {
        match pc {
            0x8261A468 => {
    //   block [0x8261A468..0x8261A4D8)
	// 8261A468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A478: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A47C: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A484: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8261A488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A48C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8261A490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A4A0: 386AD3DC  addi r3, r10, -0x2c24
	ctx.r[3].s64 = ctx.r[10].s64 + -11300;
	// 8261A4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A4C4: 4BE4C95D  bl 0x82466e20
	ctx.lr = 0x8261A4C8;
	sub_82466E20(ctx, base);
	// 8261A4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A4D8 size=100
    let mut pc: u32 = 0x8261A4D8;
    'dispatch: loop {
        match pc {
            0x8261A4D8 => {
    //   block [0x8261A4D8..0x8261A53C)
	// 8261A4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A4EC: 38AAD3DC  addi r5, r10, -0x2c24
	ctx.r[5].s64 = ctx.r[10].s64 + -11300;
	// 8261A4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A4F8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8261A4FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A50C: 386AD40C  addi r3, r10, -0x2bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -11252;
	// 8261A510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A528: 4BE4C8F9  bl 0x82466e20
	ctx.lr = 0x8261A52C;
	sub_82466E20(ctx, base);
	// 8261A52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A540 size=100
    let mut pc: u32 = 0x8261A540;
    'dispatch: loop {
        match pc {
            0x8261A540 => {
    //   block [0x8261A540..0x8261A5A4)
	// 8261A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A554: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A560: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8261A564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A574: 386AD43C  addi r3, r10, -0x2bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11204;
	// 8261A578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A57C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A580: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A588: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A590: 4BE4C891  bl 0x82466e20
	ctx.lr = 0x8261A594;
	sub_82466E20(ctx, base);
	// 8261A594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A5A8 size=100
    let mut pc: u32 = 0x8261A5A8;
    'dispatch: loop {
        match pc {
            0x8261A5A8 => {
    //   block [0x8261A5A8..0x8261A60C)
	// 8261A5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A5BC: 38AAD37C  addi r5, r10, -0x2c84
	ctx.r[5].s64 = ctx.r[10].s64 + -11396;
	// 8261A5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A5C8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8261A5CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A5DC: 386AD46C  addi r3, r10, -0x2b94
	ctx.r[3].s64 = ctx.r[10].s64 + -11156;
	// 8261A5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A5E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A5E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A5F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A5F8: 4BE4C829  bl 0x82466e20
	ctx.lr = 0x8261A5FC;
	sub_82466E20(ctx, base);
	// 8261A5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A610 size=100
    let mut pc: u32 = 0x8261A610;
    'dispatch: loop {
        match pc {
            0x8261A610 => {
    //   block [0x8261A610..0x8261A674)
	// 8261A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A61C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A624: 38AAD43C  addi r5, r10, -0x2bc4
	ctx.r[5].s64 = ctx.r[10].s64 + -11204;
	// 8261A628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A630: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8261A634: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A63C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A644: 386AD49C  addi r3, r10, -0x2b64
	ctx.r[3].s64 = ctx.r[10].s64 + -11108;
	// 8261A648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A64C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A650: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A658: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A660: 4BE4C7C1  bl 0x82466e20
	ctx.lr = 0x8261A664;
	sub_82466E20(ctx, base);
	// 8261A664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A678 size=100
    let mut pc: u32 = 0x8261A678;
    'dispatch: loop {
        match pc {
            0x8261A678 => {
    //   block [0x8261A678..0x8261A6DC)
	// 8261A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A684: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A68C: 38AAD37C  addi r5, r10, -0x2c84
	ctx.r[5].s64 = ctx.r[10].s64 + -11396;
	// 8261A690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A698: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8261A69C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A6AC: 386AD4CC  addi r3, r10, -0x2b34
	ctx.r[3].s64 = ctx.r[10].s64 + -11060;
	// 8261A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A6B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A6C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A6C8: 4BE4C759  bl 0x82466e20
	ctx.lr = 0x8261A6CC;
	sub_82466E20(ctx, base);
	// 8261A6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A6E0 size=112
    let mut pc: u32 = 0x8261A6E0;
    'dispatch: loop {
        match pc {
            0x8261A6E0 => {
    //   block [0x8261A6E0..0x8261A750)
	// 8261A6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A6EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A6F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A6F4: 38AAD55C  addi r5, r10, -0x2aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -10916;
	// 8261A6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A6FC: 390BB168  addi r8, r11, -0x4e98
	ctx.r[8].s64 = ctx.r[11].s64 + -20120;
	// 8261A700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A704: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8261A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A718: 386AD4FC  addi r3, r10, -0x2b04
	ctx.r[3].s64 = ctx.r[10].s64 + -11012;
	// 8261A71C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A73C: 4BE4C6E5  bl 0x82466e20
	ctx.lr = 0x8261A740;
	sub_82466E20(ctx, base);
	// 8261A740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A750 size=112
    let mut pc: u32 = 0x8261A750;
    'dispatch: loop {
        match pc {
            0x8261A750 => {
    //   block [0x8261A750..0x8261A7C0)
	// 8261A750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A760: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A764: 38AAD58C  addi r5, r10, -0x2a74
	ctx.r[5].s64 = ctx.r[10].s64 + -10868;
	// 8261A768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A76C: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 8261A770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A774: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8261A778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A77C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A788: 386AD52C  addi r3, r10, -0x2ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -10964;
	// 8261A78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A7AC: 4BE4C675  bl 0x82466e20
	ctx.lr = 0x8261A7B0;
	sub_82466E20(ctx, base);
	// 8261A7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A7C0 size=112
    let mut pc: u32 = 0x8261A7C0;
    'dispatch: loop {
        match pc {
            0x8261A7C0 => {
    //   block [0x8261A7C0..0x8261A830)
	// 8261A7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A7CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A7D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A7D4: 38AAD64C  addi r5, r10, -0x29b4
	ctx.r[5].s64 = ctx.r[10].s64 + -10676;
	// 8261A7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A7DC: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 8261A7E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A7E4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8261A7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A7EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A7F8: 386AD55C  addi r3, r10, -0x2aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -10916;
	// 8261A7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A81C: 4BE4C605  bl 0x82466e20
	ctx.lr = 0x8261A820;
	sub_82466E20(ctx, base);
	// 8261A820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A830 size=112
    let mut pc: u32 = 0x8261A830;
    'dispatch: loop {
        match pc {
            0x8261A830 => {
    //   block [0x8261A830..0x8261A8A0)
	// 8261A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A83C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A840: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A844: 38AAD55C  addi r5, r10, -0x2aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -10916;
	// 8261A848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A84C: 390BB1E0  addi r8, r11, -0x4e20
	ctx.r[8].s64 = ctx.r[11].s64 + -20000;
	// 8261A850: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A854: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8261A858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A85C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A868: 386AD58C  addi r3, r10, -0x2a74
	ctx.r[3].s64 = ctx.r[10].s64 + -10868;
	// 8261A86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A88C: 4BE4C595  bl 0x82466e20
	ctx.lr = 0x8261A890;
	sub_82466E20(ctx, base);
	// 8261A890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A8A0 size=112
    let mut pc: u32 = 0x8261A8A0;
    'dispatch: loop {
        match pc {
            0x8261A8A0 => {
    //   block [0x8261A8A0..0x8261A910)
	// 8261A8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A8AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A8B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A8B4: 38AAD58C  addi r5, r10, -0x2a74
	ctx.r[5].s64 = ctx.r[10].s64 + -10868;
	// 8261A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A8BC: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8261A8C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A8C4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8261A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A8CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A8D8: 386AD5BC  addi r3, r10, -0x2a44
	ctx.r[3].s64 = ctx.r[10].s64 + -10820;
	// 8261A8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A8FC: 4BE4C525  bl 0x82466e20
	ctx.lr = 0x8261A900;
	sub_82466E20(ctx, base);
	// 8261A900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A910 size=116
    let mut pc: u32 = 0x8261A910;
    'dispatch: loop {
        match pc {
            0x8261A910 => {
    //   block [0x8261A910..0x8261A984)
	// 8261A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A91C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A920: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261A924: 390AB210  addi r8, r10, -0x4df0
	ctx.r[8].s64 = ctx.r[10].s64 + -19952;
	// 8261A928: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A92C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A930: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A934: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A938: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261A93C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A944: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8261A948: 396B030C  addi r11, r11, 0x30c
	ctx.r[11].s64 = ctx.r[11].s64 + 780;
	// 8261A94C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A950: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A954: 386AD5EC  addi r3, r10, -0x2a14
	ctx.r[3].s64 = ctx.r[10].s64 + -10772;
	// 8261A958: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261A95C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A960: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261A964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A970: 4BE4C4B1  bl 0x82466e20
	ctx.lr = 0x8261A974;
	sub_82466E20(ctx, base);
	// 8261A974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A97C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261A988 size=48
    let mut pc: u32 = 0x8261A988;
    'dispatch: loop {
        match pc {
            0x8261A988 => {
    //   block [0x8261A988..0x8261A9B8)
	// 8261A988: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A98C: 814BB2C4  lwz r10, -0x4d3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19772 as u32) ) } as u64;
	// 8261A990: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A994: 396BE5A8  addi r11, r11, -0x1a58
	ctx.r[11].s64 = ctx.r[11].s64 + -6744;
	// 8261A998: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8261A99C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A9A0: 814AB2C0  lwz r10, -0x4d40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19776 as u32) ) } as u64;
	// 8261A9A4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8261A9A8: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A9AC: 814AB2BC  lwz r10, -0x4d44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19780 as u32) ) } as u64;
	// 8261A9B0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 8261A9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A9B8 size=116
    let mut pc: u32 = 0x8261A9B8;
    'dispatch: loop {
        match pc {
            0x8261A9B8 => {
    //   block [0x8261A9B8..0x8261AA2C)
	// 8261A9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A9C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A9C8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A9CC: 392B03E0  addi r9, r11, 0x3e0
	ctx.r[9].s64 = ctx.r[11].s64 + 992;
	// 8261A9D0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A9D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A9D8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8261A9DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8261A9E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A9E4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8261A9E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A9EC: 396BE5A8  addi r11, r11, -0x1a58
	ctx.r[11].s64 = ctx.r[11].s64 + -6744;
	// 8261A9F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261A9F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A9F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261A9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AA00: 386AD61C  addi r3, r10, -0x29e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10724;
	// 8261AA04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8261AA08: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261AA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AA10: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261AA14: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261AA18: 4BE4C409  bl 0x82466e20
	ctx.lr = 0x8261AA1C;
	sub_82466E20(ctx, base);
	// 8261AA1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AA30 size=116
    let mut pc: u32 = 0x8261AA30;
    'dispatch: loop {
        match pc {
            0x8261AA30 => {
    //   block [0x8261AA30..0x8261AAA4)
	// 8261AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AA3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261AA44: 390BB2D0  addi r8, r11, -0x4d30
	ctx.r[8].s64 = ctx.r[11].s64 + -19760;
	// 8261AA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AA4C: 392A0580  addi r9, r10, 0x580
	ctx.r[9].s64 = ctx.r[10].s64 + 1408;
	// 8261AA50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AA54: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8261AA58: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AA5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AA64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AA74: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261AA78: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8261AA7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261AA80: 386BD64C  addi r3, r11, -0x29b4
	ctx.r[3].s64 = ctx.r[11].s64 + -10676;
	// 8261AA84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261AA88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AA90: 4BE4C391  bl 0x82466e20
	ctx.lr = 0x8261AA94;
	sub_82466E20(ctx, base);
	// 8261AA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AAA8 size=112
    let mut pc: u32 = 0x8261AAA8;
    'dispatch: loop {
        match pc {
            0x8261AAA8 => {
    //   block [0x8261AAA8..0x8261AB18)
	// 8261AAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AAB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AAB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AABC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AAC4: 390BB360  addi r8, r11, -0x4ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -19616;
	// 8261AAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261AACC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8261AAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AAD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AAE0: 386AD67C  addi r3, r10, -0x2984
	ctx.r[3].s64 = ctx.r[10].s64 + -10628;
	// 8261AAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AB04: 4BE4C31D  bl 0x82466e20
	ctx.lr = 0x8261AB08;
	sub_82466E20(ctx, base);
	// 8261AB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AB18 size=108
    let mut pc: u32 = 0x8261AB18;
    'dispatch: loop {
        match pc {
            0x8261AB18 => {
    //   block [0x8261AB18..0x8261AB84)
	// 8261AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AB24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261AB2C: 38EBB378  addi r7, r11, -0x4c88
	ctx.r[7].s64 = ctx.r[11].s64 + -19592;
	// 8261AB30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261AB34: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 8261AB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AB3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AB40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AB48: 386AD6AC  addi r3, r10, -0x2954
	ctx.r[3].s64 = ctx.r[10].s64 + -10580;
	// 8261AB4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AB70: 4BE4C2B1  bl 0x82466e20
	ctx.lr = 0x8261AB74;
	sub_82466E20(ctx, base);
	// 8261AB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AB88 size=112
    let mut pc: u32 = 0x8261AB88;
    'dispatch: loop {
        match pc {
            0x8261AB88 => {
    //   block [0x8261AB88..0x8261ABF8)
	// 8261AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AB98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AB9C: 38AAB54C  addi r5, r10, -0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + -19124;
	// 8261ABA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ABA4: 390BB3F0  addi r8, r11, -0x4c10
	ctx.r[8].s64 = ctx.r[11].s64 + -19472;
	// 8261ABA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261ABAC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8261ABB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ABB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ABB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261ABBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ABC0: 386AD6DC  addi r3, r10, -0x2924
	ctx.r[3].s64 = ctx.r[10].s64 + -10532;
	// 8261ABC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ABC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ABCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ABD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ABD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261ABD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ABDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ABE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ABE4: 4BE4C23D  bl 0x82466e20
	ctx.lr = 0x8261ABE8;
	sub_82466E20(ctx, base);
	// 8261ABE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ABEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ABF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ABF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ABF8 size=108
    let mut pc: u32 = 0x8261ABF8;
    'dispatch: loop {
        match pc {
            0x8261ABF8 => {
    //   block [0x8261ABF8..0x8261AC64)
	// 8261ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AC04: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AC0C: 38EBB408  addi r7, r11, -0x4bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -19448;
	// 8261AC10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261AC14: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8261AC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AC1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AC28: 386AD70C  addi r3, r10, -0x28f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10484;
	// 8261AC2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AC50: 4BE4C1D1  bl 0x82466e20
	ctx.lr = 0x8261AC54;
	sub_82466E20(ctx, base);
	// 8261AC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AC68 size=112
    let mut pc: u32 = 0x8261AC68;
    'dispatch: loop {
        match pc {
            0x8261AC68 => {
    //   block [0x8261AC68..0x8261ACD8)
	// 8261AC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AC7C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AC84: 390BB420  addi r8, r11, -0x4be0
	ctx.r[8].s64 = ctx.r[11].s64 + -19424;
	// 8261AC88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261AC8C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8261AC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ACA0: 386AD73C  addi r3, r10, -0x28c4
	ctx.r[3].s64 = ctx.r[10].s64 + -10436;
	// 8261ACA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ACA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ACB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261ACB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ACBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ACC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ACC4: 4BE4C15D  bl 0x82466e20
	ctx.lr = 0x8261ACC8;
	sub_82466E20(ctx, base);
	// 8261ACC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ACCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ACD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ACD8 size=108
    let mut pc: u32 = 0x8261ACD8;
    'dispatch: loop {
        match pc {
            0x8261ACD8 => {
    //   block [0x8261ACD8..0x8261AD44)
	// 8261ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ACE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261ACE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ACEC: 38EBB468  addi r7, r11, -0x4b98
	ctx.r[7].s64 = ctx.r[11].s64 + -19352;
	// 8261ACF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261ACF4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8261ACF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ACFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AD00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AD08: 386AD76C  addi r3, r10, -0x2894
	ctx.r[3].s64 = ctx.r[10].s64 + -10388;
	// 8261AD0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AD30: 4BE4C0F1  bl 0x82466e20
	ctx.lr = 0x8261AD34;
	sub_82466E20(ctx, base);
	// 8261AD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AD48 size=108
    let mut pc: u32 = 0x8261AD48;
    'dispatch: loop {
        match pc {
            0x8261AD48 => {
    //   block [0x8261AD48..0x8261ADB4)
	// 8261AD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AD54: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AD58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AD5C: 38EBB498  addi r7, r11, -0x4b68
	ctx.r[7].s64 = ctx.r[11].s64 + -19304;
	// 8261AD60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261AD64: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8261AD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AD6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AD70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AD78: 386AD79C  addi r3, r10, -0x2864
	ctx.r[3].s64 = ctx.r[10].s64 + -10340;
	// 8261AD7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AD9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261ADA0: 4BE4C081  bl 0x82466e20
	ctx.lr = 0x8261ADA4;
	sub_82466E20(ctx, base);
	// 8261ADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ADB8 size=112
    let mut pc: u32 = 0x8261ADB8;
    'dispatch: loop {
        match pc {
            0x8261ADB8 => {
    //   block [0x8261ADB8..0x8261AE28)
	// 8261ADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ADC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ADC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ADC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261ADCC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261ADD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ADD4: 390BB4B0  addi r8, r11, -0x4b50
	ctx.r[8].s64 = ctx.r[11].s64 + -19280;
	// 8261ADD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261ADDC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8261ADE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ADE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ADE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ADF0: 386AD7CC  addi r3, r10, -0x2834
	ctx.r[3].s64 = ctx.r[10].s64 + -10292;
	// 8261ADF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ADF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ADFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AE14: 4BE4C00D  bl 0x82466e20
	ctx.lr = 0x8261AE18;
	sub_82466E20(ctx, base);
	// 8261AE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AE28 size=112
    let mut pc: u32 = 0x8261AE28;
    'dispatch: loop {
        match pc {
            0x8261AE28 => {
    //   block [0x8261AE28..0x8261AE98)
	// 8261AE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AE34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AE38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AE3C: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261AE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AE44: 390BB4E0  addi r8, r11, -0x4b20
	ctx.r[8].s64 = ctx.r[11].s64 + -19232;
	// 8261AE48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261AE4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8261AE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AE54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AE60: 386AD7FC  addi r3, r10, -0x2804
	ctx.r[3].s64 = ctx.r[10].s64 + -10244;
	// 8261AE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AE84: 4BE4BF9D  bl 0x82466e20
	ctx.lr = 0x8261AE88;
	sub_82466E20(ctx, base);
	// 8261AE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AE98 size=112
    let mut pc: u32 = 0x8261AE98;
    'dispatch: loop {
        match pc {
            0x8261AE98 => {
    //   block [0x8261AE98..0x8261AF08)
	// 8261AE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AEA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AEA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AEAC: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261AEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AEB4: 390BB528  addi r8, r11, -0x4ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -19160;
	// 8261AEB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AEBC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8261AEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AED0: 386AD82C  addi r3, r10, -0x27d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10196;
	// 8261AED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AEF4: 4BE4BF2D  bl 0x82466e20
	ctx.lr = 0x8261AEF8;
	sub_82466E20(ctx, base);
	// 8261AEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AF08 size=112
    let mut pc: u32 = 0x8261AF08;
    'dispatch: loop {
        match pc {
            0x8261AF08 => {
    //   block [0x8261AF08..0x8261AF78)
	// 8261AF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AF14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AF1C: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261AF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AF24: 390BB588  addi r8, r11, -0x4a78
	ctx.r[8].s64 = ctx.r[11].s64 + -19064;
	// 8261AF28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AF2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8261AF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AF34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AF40: 386AD85C  addi r3, r10, -0x27a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10148;
	// 8261AF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AF64: 4BE4BEBD  bl 0x82466e20
	ctx.lr = 0x8261AF68;
	sub_82466E20(ctx, base);
	// 8261AF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AF78 size=112
    let mut pc: u32 = 0x8261AF78;
    'dispatch: loop {
        match pc {
            0x8261AF78 => {
    //   block [0x8261AF78..0x8261AFE8)
	// 8261AF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AF8C: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261AF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AF94: 390BB5E8  addi r8, r11, -0x4a18
	ctx.r[8].s64 = ctx.r[11].s64 + -18968;
	// 8261AF98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AF9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8261AFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AFB0: 386AD88C  addi r3, r10, -0x2774
	ctx.r[3].s64 = ctx.r[10].s64 + -10100;
	// 8261AFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AFD4: 4BE4BE4D  bl 0x82466e20
	ctx.lr = 0x8261AFD8;
	sub_82466E20(ctx, base);
	// 8261AFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AFE8 size=112
    let mut pc: u32 = 0x8261AFE8;
    'dispatch: loop {
        match pc {
            0x8261AFE8 => {
    //   block [0x8261AFE8..0x8261B058)
	// 8261AFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AFF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AFF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AFFC: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261B000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B004: 390BB648  addi r8, r11, -0x49b8
	ctx.r[8].s64 = ctx.r[11].s64 + -18872;
	// 8261B008: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261B00C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8261B010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B014: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B020: 386AD8BC  addi r3, r10, -0x2744
	ctx.r[3].s64 = ctx.r[10].s64 + -10052;
	// 8261B024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B044: 4BE4BDDD  bl 0x82466e20
	ctx.lr = 0x8261B048;
	sub_82466E20(ctx, base);
	// 8261B048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B058 size=112
    let mut pc: u32 = 0x8261B058;
    'dispatch: loop {
        match pc {
            0x8261B058 => {
    //   block [0x8261B058..0x8261B0C8)
	// 8261B058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B064: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261B068: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8261B06C: 38EAB708  addi r7, r10, -0x48f8
	ctx.r[7].s64 = ctx.r[10].s64 + -18680;
	// 8261B070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B074: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261B078: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8261B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B080: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B084: 396B0598  addi r11, r11, 0x598
	ctx.r[11].s64 = ctx.r[11].s64 + 1432;
	// 8261B088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B08C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B090: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B094: 386AD8EC  addi r3, r10, -0x2714
	ctx.r[3].s64 = ctx.r[10].s64 + -10004;
	// 8261B098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B09C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261B0A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B0A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261B0A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B0AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B0B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B0B4: 4BE4BD6D  bl 0x82466e20
	ctx.lr = 0x8261B0B8;
	sub_82466E20(ctx, base);
	// 8261B0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B0C8 size=112
    let mut pc: u32 = 0x8261B0C8;
    'dispatch: loop {
        match pc {
            0x8261B0C8 => {
    //   block [0x8261B0C8..0x8261B138)
	// 8261B0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B0D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B0D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B0DC: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B0E4: 390BB8D0  addi r8, r11, -0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + -18224;
	// 8261B0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B0EC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8261B0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B100: 386AD91C  addi r3, r10, -0x26e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9956;
	// 8261B104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B124: 4BE4BCFD  bl 0x82466e20
	ctx.lr = 0x8261B128;
	sub_82466E20(ctx, base);
	// 8261B128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B138 size=112
    let mut pc: u32 = 0x8261B138;
    'dispatch: loop {
        match pc {
            0x8261B138 => {
    //   block [0x8261B138..0x8261B1A8)
	// 8261B138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B148: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B14C: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B154: 390BB8E8  addi r8, r11, -0x4718
	ctx.r[8].s64 = ctx.r[11].s64 + -18200;
	// 8261B158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B15C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8261B160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B170: 386AD94C  addi r3, r10, -0x26b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9908;
	// 8261B174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B184: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B194: 4BE4BC8D  bl 0x82466e20
	ctx.lr = 0x8261B198;
	sub_82466E20(ctx, base);
	// 8261B198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B1A8 size=112
    let mut pc: u32 = 0x8261B1A8;
    'dispatch: loop {
        match pc {
            0x8261B1A8 => {
    //   block [0x8261B1A8..0x8261B218)
	// 8261B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B1B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B1B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B1BC: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B1C4: 390BB900  addi r8, r11, -0x4700
	ctx.r[8].s64 = ctx.r[11].s64 + -18176;
	// 8261B1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B1CC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8261B1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B1E0: 386AD97C  addi r3, r10, -0x2684
	ctx.r[3].s64 = ctx.r[10].s64 + -9860;
	// 8261B1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B204: 4BE4BC1D  bl 0x82466e20
	ctx.lr = 0x8261B208;
	sub_82466E20(ctx, base);
	// 8261B208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B218 size=108
    let mut pc: u32 = 0x8261B218;
    'dispatch: loop {
        match pc {
            0x8261B218 => {
    //   block [0x8261B218..0x8261B284)
	// 8261B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B224: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B22C: 38EBB930  addi r7, r11, -0x46d0
	ctx.r[7].s64 = ctx.r[11].s64 + -18128;
	// 8261B230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B234: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8261B238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B248: 386AD9AC  addi r3, r10, -0x2654
	ctx.r[3].s64 = ctx.r[10].s64 + -9812;
	// 8261B24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B270: 4BE4BBB1  bl 0x82466e20
	ctx.lr = 0x8261B274;
	sub_82466E20(ctx, base);
	// 8261B274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B288 size=112
    let mut pc: u32 = 0x8261B288;
    'dispatch: loop {
        match pc {
            0x8261B288 => {
    //   block [0x8261B288..0x8261B2F8)
	// 8261B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B298: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B29C: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B2A4: 390BB960  addi r8, r11, -0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18080;
	// 8261B2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B2AC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8261B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B2B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B2C0: 386AD9DC  addi r3, r10, -0x2624
	ctx.r[3].s64 = ctx.r[10].s64 + -9764;
	// 8261B2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B2D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B2E4: 4BE4BB3D  bl 0x82466e20
	ctx.lr = 0x8261B2E8;
	sub_82466E20(ctx, base);
	// 8261B2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B2F8 size=108
    let mut pc: u32 = 0x8261B2F8;
    'dispatch: loop {
        match pc {
            0x8261B2F8 => {
    //   block [0x8261B2F8..0x8261B364)
	// 8261B2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B304: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B30C: 38EBB978  addi r7, r11, -0x4688
	ctx.r[7].s64 = ctx.r[11].s64 + -18056;
	// 8261B310: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B314: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8261B318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B31C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B328: 386ADA0C  addi r3, r10, -0x25f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9716;
	// 8261B32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B350: 4BE4BAD1  bl 0x82466e20
	ctx.lr = 0x8261B354;
	sub_82466E20(ctx, base);
	// 8261B354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B368 size=108
    let mut pc: u32 = 0x8261B368;
    'dispatch: loop {
        match pc {
            0x8261B368 => {
    //   block [0x8261B368..0x8261B3D4)
	// 8261B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B374: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B37C: 38EBB9A8  addi r7, r11, -0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + -18008;
	// 8261B380: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B384: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8261B388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B38C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B398: 386ADA3C  addi r3, r10, -0x25c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9668;
	// 8261B39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B3C0: 4BE4BA61  bl 0x82466e20
	ctx.lr = 0x8261B3C4;
	sub_82466E20(ctx, base);
	// 8261B3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B3D8 size=112
    let mut pc: u32 = 0x8261B3D8;
    'dispatch: loop {
        match pc {
            0x8261B3D8 => {
    //   block [0x8261B3D8..0x8261B448)
	// 8261B3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B3E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B3E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B3EC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B3F4: 390BB9F0  addi r8, r11, -0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + -17936;
	// 8261B3F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261B3FC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8261B400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B410: 386ADA6C  addi r3, r10, -0x2594
	ctx.r[3].s64 = ctx.r[10].s64 + -9620;
	// 8261B414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B434: 4BE4B9ED  bl 0x82466e20
	ctx.lr = 0x8261B438;
	sub_82466E20(ctx, base);
	// 8261B438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B448 size=112
    let mut pc: u32 = 0x8261B448;
    'dispatch: loop {
        match pc {
            0x8261B448 => {
    //   block [0x8261B448..0x8261B4B8)
	// 8261B448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B458: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B45C: 38AAD5EC  addi r5, r10, -0x2a14
	ctx.r[5].s64 = ctx.r[10].s64 + -10772;
	// 8261B460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B464: 390BBA38  addi r8, r11, -0x45c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17864;
	// 8261B468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B46C: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8261B470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B480: 386ADA9C  addi r3, r10, -0x2564
	ctx.r[3].s64 = ctx.r[10].s64 + -9572;
	// 8261B484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B4A4: 4BE4B97D  bl 0x82466e20
	ctx.lr = 0x8261B4A8;
	sub_82466E20(ctx, base);
	// 8261B4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B4B8 size=108
    let mut pc: u32 = 0x8261B4B8;
    'dispatch: loop {
        match pc {
            0x8261B4B8 => {
    //   block [0x8261B4B8..0x8261B524)
	// 8261B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B4C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B4CC: 38EBBA50  addi r7, r11, -0x45b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17840;
	// 8261B4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B4D4: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8261B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B4DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B4E8: 386ADACC  addi r3, r10, -0x2534
	ctx.r[3].s64 = ctx.r[10].s64 + -9524;
	// 8261B4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B510: 4BE4B911  bl 0x82466e20
	ctx.lr = 0x8261B514;
	sub_82466E20(ctx, base);
	// 8261B514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B528 size=108
    let mut pc: u32 = 0x8261B528;
    'dispatch: loop {
        match pc {
            0x8261B528 => {
    //   block [0x8261B528..0x8261B594)
	// 8261B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B534: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B53C: 38EBBA98  addi r7, r11, -0x4568
	ctx.r[7].s64 = ctx.r[11].s64 + -17768;
	// 8261B540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B544: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8261B548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B558: 386ADAFC  addi r3, r10, -0x2504
	ctx.r[3].s64 = ctx.r[10].s64 + -9476;
	// 8261B55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B580: 4BE4B8A1  bl 0x82466e20
	ctx.lr = 0x8261B584;
	sub_82466E20(ctx, base);
	// 8261B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B598 size=112
    let mut pc: u32 = 0x8261B598;
    'dispatch: loop {
        match pc {
            0x8261B598 => {
    //   block [0x8261B598..0x8261B608)
	// 8261B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B5A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B5A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B5AC: 38AADAFC  addi r5, r10, -0x2504
	ctx.r[5].s64 = ctx.r[10].s64 + -9476;
	// 8261B5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B5B4: 390BBAC8  addi r8, r11, -0x4538
	ctx.r[8].s64 = ctx.r[11].s64 + -17720;
	// 8261B5B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B5BC: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8261B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B5D0: 386ADB2C  addi r3, r10, -0x24d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9428;
	// 8261B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B5F4: 4BE4B82D  bl 0x82466e20
	ctx.lr = 0x8261B5F8;
	sub_82466E20(ctx, base);
	// 8261B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261B608 size=24
    let mut pc: u32 = 0x8261B608;
    'dispatch: loop {
        match pc {
            0x8261B608 => {
    //   block [0x8261B608..0x8261B620)
	// 8261B608: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B60C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261B610: 394AE968  addi r10, r10, -0x1698
	ctx.r[10].s64 = ctx.r[10].s64 + -5784;
	// 8261B614: 816BBAF8  lwz r11, -0x4508(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17672 as u32) ) } as u64;
	// 8261B618: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8261B61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B620 size=116
    let mut pc: u32 = 0x8261B620;
    'dispatch: loop {
        match pc {
            0x8261B620 => {
    //   block [0x8261B620..0x8261B694)
	// 8261B620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B62C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B634: 390BE968  addi r8, r11, -0x1698
	ctx.r[8].s64 = ctx.r[11].s64 + -5784;
	// 8261B638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B63C: 392A0630  addi r9, r10, 0x630
	ctx.r[9].s64 = ctx.r[10].s64 + 1584;
	// 8261B640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B644: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8261B648: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261B668: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8261B66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261B670: 386BDB5C  addi r3, r11, -0x24a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9380;
	// 8261B674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261B678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B680: 4BE4B7A1  bl 0x82466e20
	ctx.lr = 0x8261B684;
	sub_82466E20(ctx, base);
	// 8261B684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B698 size=112
    let mut pc: u32 = 0x8261B698;
    'dispatch: loop {
        match pc {
            0x8261B698 => {
    //   block [0x8261B698..0x8261B708)
	// 8261B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B6A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B6AC: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261B6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B6B4: 390BBB00  addi r8, r11, -0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + -17664;
	// 8261B6B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8261B6BC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8261B6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B6D0: 386ADB8C  addi r3, r10, -0x2474
	ctx.r[3].s64 = ctx.r[10].s64 + -9332;
	// 8261B6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B6F4: 4BE4B72D  bl 0x82466e20
	ctx.lr = 0x8261B6F8;
	sub_82466E20(ctx, base);
	// 8261B6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B708 size=108
    let mut pc: u32 = 0x8261B708;
    'dispatch: loop {
        match pc {
            0x8261B708 => {
    //   block [0x8261B708..0x8261B774)
	// 8261B708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B71C: 38EBBB90  addi r7, r11, -0x4470
	ctx.r[7].s64 = ctx.r[11].s64 + -17520;
	// 8261B720: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B724: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8261B728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B738: 386ADBBC  addi r3, r10, -0x2444
	ctx.r[3].s64 = ctx.r[10].s64 + -9284;
	// 8261B73C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B760: 4BE4B6C1  bl 0x82466e20
	ctx.lr = 0x8261B764;
	sub_82466E20(ctx, base);
	// 8261B764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B778 size=108
    let mut pc: u32 = 0x8261B778;
    'dispatch: loop {
        match pc {
            0x8261B778 => {
    //   block [0x8261B778..0x8261B7E4)
	// 8261B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B78C: 38EBBBD8  addi r7, r11, -0x4428
	ctx.r[7].s64 = ctx.r[11].s64 + -17448;
	// 8261B790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B794: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8261B798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B7A8: 386ADBEC  addi r3, r10, -0x2414
	ctx.r[3].s64 = ctx.r[10].s64 + -9236;
	// 8261B7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B7D0: 4BE4B651  bl 0x82466e20
	ctx.lr = 0x8261B7D4;
	sub_82466E20(ctx, base);
	// 8261B7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B7E8 size=108
    let mut pc: u32 = 0x8261B7E8;
    'dispatch: loop {
        match pc {
            0x8261B7E8 => {
    //   block [0x8261B7E8..0x8261B854)
	// 8261B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B7F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B7FC: 38EBBC08  addi r7, r11, -0x43f8
	ctx.r[7].s64 = ctx.r[11].s64 + -17400;
	// 8261B800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B804: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8261B808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B818: 386ADC1C  addi r3, r10, -0x23e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9188;
	// 8261B81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B840: 4BE4B5E1  bl 0x82466e20
	ctx.lr = 0x8261B844;
	sub_82466E20(ctx, base);
	// 8261B844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B858 size=112
    let mut pc: u32 = 0x8261B858;
    'dispatch: loop {
        match pc {
            0x8261B858 => {
    //   block [0x8261B858..0x8261B8C8)
	// 8261B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B868: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B86C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B874: 390BBC38  addi r8, r11, -0x43c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17352;
	// 8261B878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B87C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8261B880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B890: 386ADC4C  addi r3, r10, -0x23b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9140;
	// 8261B894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B8B4: 4BE4B56D  bl 0x82466e20
	ctx.lr = 0x8261B8B8;
	sub_82466E20(ctx, base);
	// 8261B8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B8C8 size=112
    let mut pc: u32 = 0x8261B8C8;
    'dispatch: loop {
        match pc {
            0x8261B8C8 => {
    //   block [0x8261B8C8..0x8261B938)
	// 8261B8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B8D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B8D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B8DC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B8E4: 390BBC68  addi r8, r11, -0x4398
	ctx.r[8].s64 = ctx.r[11].s64 + -17304;
	// 8261B8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B8EC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8261B8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B8F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B900: 386ADC7C  addi r3, r10, -0x2384
	ctx.r[3].s64 = ctx.r[10].s64 + -9092;
	// 8261B904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B924: 4BE4B4FD  bl 0x82466e20
	ctx.lr = 0x8261B928;
	sub_82466E20(ctx, base);
	// 8261B928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B938 size=112
    let mut pc: u32 = 0x8261B938;
    'dispatch: loop {
        match pc {
            0x8261B938 => {
    //   block [0x8261B938..0x8261B9A8)
	// 8261B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B948: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B94C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B954: 390BBC80  addi r8, r11, -0x4380
	ctx.r[8].s64 = ctx.r[11].s64 + -17280;
	// 8261B958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B95C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8261B960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B964: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B970: 386ADCAC  addi r3, r10, -0x2354
	ctx.r[3].s64 = ctx.r[10].s64 + -9044;
	// 8261B974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B994: 4BE4B48D  bl 0x82466e20
	ctx.lr = 0x8261B998;
	sub_82466E20(ctx, base);
	// 8261B998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B9A8 size=108
    let mut pc: u32 = 0x8261B9A8;
    'dispatch: loop {
        match pc {
            0x8261B9A8 => {
    //   block [0x8261B9A8..0x8261BA14)
	// 8261B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B9B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B9BC: 38EBBC98  addi r7, r11, -0x4368
	ctx.r[7].s64 = ctx.r[11].s64 + -17256;
	// 8261B9C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B9C4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8261B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B9D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B9D8: 386ADCDC  addi r3, r10, -0x2324
	ctx.r[3].s64 = ctx.r[10].s64 + -8996;
	// 8261B9DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BA00: 4BE4B421  bl 0x82466e20
	ctx.lr = 0x8261BA04;
	sub_82466E20(ctx, base);
	// 8261BA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BA18 size=112
    let mut pc: u32 = 0x8261BA18;
    'dispatch: loop {
        match pc {
            0x8261BA18 => {
    //   block [0x8261BA18..0x8261BA88)
	// 8261BA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BA24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BA28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BA2C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BA34: 390BBCC8  addi r8, r11, -0x4338
	ctx.r[8].s64 = ctx.r[11].s64 + -17208;
	// 8261BA38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261BA3C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8261BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BA44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BA50: 386ADD0C  addi r3, r10, -0x22f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8948;
	// 8261BA54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BA74: 4BE4B3AD  bl 0x82466e20
	ctx.lr = 0x8261BA78;
	sub_82466E20(ctx, base);
	// 8261BA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BA88 size=108
    let mut pc: u32 = 0x8261BA88;
    'dispatch: loop {
        match pc {
            0x8261BA88 => {
    //   block [0x8261BA88..0x8261BAF4)
	// 8261BA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BA94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BA9C: 38EBBCE0  addi r7, r11, -0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + -17184;
	// 8261BAA0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8261BAA4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8261BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BAB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BAB8: 386ADD3C  addi r3, r10, -0x22c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8900;
	// 8261BABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BAE0: 4BE4B341  bl 0x82466e20
	ctx.lr = 0x8261BAE4;
	sub_82466E20(ctx, base);
	// 8261BAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BAF8 size=116
    let mut pc: u32 = 0x8261BAF8;
    'dispatch: loop {
        match pc {
            0x8261BAF8 => {
    //   block [0x8261BAF8..0x8261BB6C)
	// 8261BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BB04: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261BB08: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8261BB0C: 390ABDD0  addi r8, r10, -0x4230
	ctx.r[8].s64 = ctx.r[10].s64 + -16944;
	// 8261BB10: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261BB18: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BB1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BB20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261BB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BB2C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8261BB30: 396B0648  addi r11, r11, 0x648
	ctx.r[11].s64 = ctx.r[11].s64 + 1608;
	// 8261BB34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BB3C: 386ADD6C  addi r3, r10, -0x2294
	ctx.r[3].s64 = ctx.r[10].s64 + -8852;
	// 8261BB40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261BB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BB48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261BB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BB58: 4BE4B2C9  bl 0x82466e20
	ctx.lr = 0x8261BB5C;
	sub_82466E20(ctx, base);
	// 8261BB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BB70 size=108
    let mut pc: u32 = 0x8261BB70;
    'dispatch: loop {
        match pc {
            0x8261BB70 => {
    //   block [0x8261BB70..0x8261BBDC)
	// 8261BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BB7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BB84: 38EBBF98  addi r7, r11, -0x4068
	ctx.r[7].s64 = ctx.r[11].s64 + -16488;
	// 8261BB88: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8261BB8C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8261BB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BBA0: 386ADD9C  addi r3, r10, -0x2264
	ctx.r[3].s64 = ctx.r[10].s64 + -8804;
	// 8261BBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BBC8: 4BE4B259  bl 0x82466e20
	ctx.lr = 0x8261BBCC;
	sub_82466E20(ctx, base);
	// 8261BBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BBE0 size=112
    let mut pc: u32 = 0x8261BBE0;
    'dispatch: loop {
        match pc {
            0x8261BBE0 => {
    //   block [0x8261BBE0..0x8261BC50)
	// 8261BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BBF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BBF4: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261BBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BBFC: 390BC130  addi r8, r11, -0x3ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -16080;
	// 8261BC00: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8261BC04: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8261BC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BC0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BC18: 386ADDCC  addi r3, r10, -0x2234
	ctx.r[3].s64 = ctx.r[10].s64 + -8756;
	// 8261BC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BC3C: 4BE4B1E5  bl 0x82466e20
	ctx.lr = 0x8261BC40;
	sub_82466E20(ctx, base);
	// 8261BC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BC50 size=100
    let mut pc: u32 = 0x8261BC50;
    'dispatch: loop {
        match pc {
            0x8261BC50 => {
    //   block [0x8261BC50..0x8261BCB4)
	// 8261BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BC64: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BC70: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8261BC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BC84: 386ADDFC  addi r3, r10, -0x2204
	ctx.r[3].s64 = ctx.r[10].s64 + -8708;
	// 8261BC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BCA0: 4BE4B181  bl 0x82466e20
	ctx.lr = 0x8261BCA4;
	sub_82466E20(ctx, base);
	// 8261BCA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BCB8 size=112
    let mut pc: u32 = 0x8261BCB8;
    'dispatch: loop {
        match pc {
            0x8261BCB8 => {
    //   block [0x8261BCB8..0x8261BD28)
	// 8261BCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BCC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BCC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BCCC: 38AADDFC  addi r5, r10, -0x2204
	ctx.r[5].s64 = ctx.r[10].s64 + -8708;
	// 8261BCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BCD4: 390BC388  addi r8, r11, -0x3c78
	ctx.r[8].s64 = ctx.r[11].s64 + -15480;
	// 8261BCD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261BCDC: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8261BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BCE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BCF0: 386ADE2C  addi r3, r10, -0x21d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8660;
	// 8261BCF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BD14: 4BE4B10D  bl 0x82466e20
	ctx.lr = 0x8261BD18;
	sub_82466E20(ctx, base);
	// 8261BD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BD28 size=100
    let mut pc: u32 = 0x8261BD28;
    'dispatch: loop {
        match pc {
            0x8261BD28 => {
    //   block [0x8261BD28..0x8261BD8C)
	// 8261BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BD3C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BD48: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8261BD4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BD5C: 386ADE5C  addi r3, r10, -0x21a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8612;
	// 8261BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BD64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BD78: 4BE4B0A9  bl 0x82466e20
	ctx.lr = 0x8261BD7C;
	sub_82466E20(ctx, base);
	// 8261BD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BD90 size=108
    let mut pc: u32 = 0x8261BD90;
    'dispatch: loop {
        match pc {
            0x8261BD90 => {
    //   block [0x8261BD90..0x8261BDFC)
	// 8261BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BD9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BDA4: 38EBC400  addi r7, r11, -0x3c00
	ctx.r[7].s64 = ctx.r[11].s64 + -15360;
	// 8261BDA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261BDAC: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8261BDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BDB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BDC0: 386ADE8C  addi r3, r10, -0x2174
	ctx.r[3].s64 = ctx.r[10].s64 + -8564;
	// 8261BDC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BDE8: 4BE4B039  bl 0x82466e20
	ctx.lr = 0x8261BDEC;
	sub_82466E20(ctx, base);
	// 8261BDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BE00 size=112
    let mut pc: u32 = 0x8261BE00;
    'dispatch: loop {
        match pc {
            0x8261BE00 => {
    //   block [0x8261BE00..0x8261BE70)
	// 8261BE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BE0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BE14: 38AADE5C  addi r5, r10, -0x21a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8612;
	// 8261BE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BE1C: 390BC448  addi r8, r11, -0x3bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -15288;
	// 8261BE20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261BE24: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8261BE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BE2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BE38: 386ADEBC  addi r3, r10, -0x2144
	ctx.r[3].s64 = ctx.r[10].s64 + -8516;
	// 8261BE3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BE5C: 4BE4AFC5  bl 0x82466e20
	ctx.lr = 0x8261BE60;
	sub_82466E20(ctx, base);
	// 8261BE60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BE70 size=100
    let mut pc: u32 = 0x8261BE70;
    'dispatch: loop {
        match pc {
            0x8261BE70 => {
    //   block [0x8261BE70..0x8261BED4)
	// 8261BE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BE7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BE84: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BE88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BE90: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8261BE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BEA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BEA4: 386ADEEC  addi r3, r10, -0x2114
	ctx.r[3].s64 = ctx.r[10].s64 + -8468;
	// 8261BEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BEAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BEB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BEB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BEC0: 4BE4AF61  bl 0x82466e20
	ctx.lr = 0x8261BEC4;
	sub_82466E20(ctx, base);
	// 8261BEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BED8 size=100
    let mut pc: u32 = 0x8261BED8;
    'dispatch: loop {
        match pc {
            0x8261BED8 => {
    //   block [0x8261BED8..0x8261BF3C)
	// 8261BED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BEE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BEEC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BEF8: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8261BEFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BF0C: 386ADF1C  addi r3, r10, -0x20e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8420;
	// 8261BF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BF14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BF18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BF20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BF28: 4BE4AEF9  bl 0x82466e20
	ctx.lr = 0x8261BF2C;
	sub_82466E20(ctx, base);
	// 8261BF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BF40 size=112
    let mut pc: u32 = 0x8261BF40;
    'dispatch: loop {
        match pc {
            0x8261BF40 => {
    //   block [0x8261BF40..0x8261BFB0)
	// 8261BF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BF4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF50: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BF54: 38AADEEC  addi r5, r10, -0x2114
	ctx.r[5].s64 = ctx.r[10].s64 + -8468;
	// 8261BF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BF5C: 390BC478  addi r8, r11, -0x3b88
	ctx.r[8].s64 = ctx.r[11].s64 + -15240;
	// 8261BF60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261BF64: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8261BF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BF6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BF74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BF78: 386ADF4C  addi r3, r10, -0x20b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8372;
	// 8261BF7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BF9C: 4BE4AE85  bl 0x82466e20
	ctx.lr = 0x8261BFA0;
	sub_82466E20(ctx, base);
	// 8261BFA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BFB0 size=112
    let mut pc: u32 = 0x8261BFB0;
    'dispatch: loop {
        match pc {
            0x8261BFB0 => {
    //   block [0x8261BFB0..0x8261C020)
	// 8261BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BFBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BFC0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BFC4: 38AADF1C  addi r5, r10, -0x20e4
	ctx.r[5].s64 = ctx.r[10].s64 + -8420;
	// 8261BFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BFCC: 390BC4D8  addi r8, r11, -0x3b28
	ctx.r[8].s64 = ctx.r[11].s64 + -15144;
	// 8261BFD0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261BFD4: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8261BFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BFDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BFE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BFE8: 386ADF7C  addi r3, r10, -0x2084
	ctx.r[3].s64 = ctx.r[10].s64 + -8324;
	// 8261BFEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C00C: 4BE4AE15  bl 0x82466e20
	ctx.lr = 0x8261C010;
	sub_82466E20(ctx, base);
	// 8261C010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C020 size=100
    let mut pc: u32 = 0x8261C020;
    'dispatch: loop {
        match pc {
            0x8261C020 => {
    //   block [0x8261C020..0x8261C084)
	// 8261C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C034: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C040: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8261C044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C054: 386ADFAC  addi r3, r10, -0x2054
	ctx.r[3].s64 = ctx.r[10].s64 + -8276;
	// 8261C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C05C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C060: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C068: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C070: 4BE4ADB1  bl 0x82466e20
	ctx.lr = 0x8261C074;
	sub_82466E20(ctx, base);
	// 8261C074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C088 size=112
    let mut pc: u32 = 0x8261C088;
    'dispatch: loop {
        match pc {
            0x8261C088 => {
    //   block [0x8261C088..0x8261C0F8)
	// 8261C088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C098: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C09C: 38AADFAC  addi r5, r10, -0x2054
	ctx.r[5].s64 = ctx.r[10].s64 + -8276;
	// 8261C0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C0A4: 390BC538  addi r8, r11, -0x3ac8
	ctx.r[8].s64 = ctx.r[11].s64 + -15048;
	// 8261C0A8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8261C0AC: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8261C0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C0B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C0C0: 386ADFDC  addi r3, r10, -0x2024
	ctx.r[3].s64 = ctx.r[10].s64 + -8228;
	// 8261C0C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C0E4: 4BE4AD3D  bl 0x82466e20
	ctx.lr = 0x8261C0E8;
	sub_82466E20(ctx, base);
	// 8261C0E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C0F8 size=108
    let mut pc: u32 = 0x8261C0F8;
    'dispatch: loop {
        match pc {
            0x8261C0F8 => {
    //   block [0x8261C0F8..0x8261C164)
	// 8261C0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C104: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C10C: 38EBC628  addi r7, r11, -0x39d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14808;
	// 8261C110: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8261C114: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8261C118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C11C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C120: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C128: 386AE00C  addi r3, r10, -0x1ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -8180;
	// 8261C12C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C14C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C150: 4BE4ACD1  bl 0x82466e20
	ctx.lr = 0x8261C154;
	sub_82466E20(ctx, base);
	// 8261C154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C168 size=108
    let mut pc: u32 = 0x8261C168;
    'dispatch: loop {
        match pc {
            0x8261C168 => {
    //   block [0x8261C168..0x8261C1D4)
	// 8261C168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C174: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C17C: 38EBC718  addi r7, r11, -0x38e8
	ctx.r[7].s64 = ctx.r[11].s64 + -14568;
	// 8261C180: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C184: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8261C188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C18C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C190: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C198: 386AE03C  addi r3, r10, -0x1fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -8132;
	// 8261C19C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C1A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C1B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C1C0: 4BE4AC61  bl 0x82466e20
	ctx.lr = 0x8261C1C4;
	sub_82466E20(ctx, base);
	// 8261C1C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C1D8 size=108
    let mut pc: u32 = 0x8261C1D8;
    'dispatch: loop {
        match pc {
            0x8261C1D8 => {
    //   block [0x8261C1D8..0x8261C244)
	// 8261C1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C1E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C1EC: 38EBC760  addi r7, r11, -0x38a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14496;
	// 8261C1F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8261C1F4: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8261C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C1FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C208: 386AE06C  addi r3, r10, -0x1f94
	ctx.r[3].s64 = ctx.r[10].s64 + -8084;
	// 8261C20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C230: 4BE4ABF1  bl 0x82466e20
	ctx.lr = 0x8261C234;
	sub_82466E20(ctx, base);
	// 8261C234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C248 size=108
    let mut pc: u32 = 0x8261C248;
    'dispatch: loop {
        match pc {
            0x8261C248 => {
    //   block [0x8261C248..0x8261C2B4)
	// 8261C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C254: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C25C: 38EBC838  addi r7, r11, -0x37c8
	ctx.r[7].s64 = ctx.r[11].s64 + -14280;
	// 8261C260: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261C264: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8261C268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C26C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C278: 386AE09C  addi r3, r10, -0x1f64
	ctx.r[3].s64 = ctx.r[10].s64 + -8036;
	// 8261C27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C2A0: 4BE4AB81  bl 0x82466e20
	ctx.lr = 0x8261C2A4;
	sub_82466E20(ctx, base);
	// 8261C2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C2B8 size=100
    let mut pc: u32 = 0x8261C2B8;
    'dispatch: loop {
        match pc {
            0x8261C2B8 => {
    //   block [0x8261C2B8..0x8261C31C)
	// 8261C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C2CC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C2D8: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8261C2DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C2EC: 386AE0CC  addi r3, r10, -0x1f34
	ctx.r[3].s64 = ctx.r[10].s64 + -7988;
	// 8261C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C2F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C308: 4BE4AB19  bl 0x82466e20
	ctx.lr = 0x8261C30C;
	sub_82466E20(ctx, base);
	// 8261C30C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C320 size=112
    let mut pc: u32 = 0x8261C320;
    'dispatch: loop {
        match pc {
            0x8261C320 => {
    //   block [0x8261C320..0x8261C390)
	// 8261C320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C32C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C334: 38AAE0CC  addi r5, r10, -0x1f34
	ctx.r[5].s64 = ctx.r[10].s64 + -7988;
	// 8261C338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C33C: 390BC850  addi r8, r11, -0x37b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14256;
	// 8261C340: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C344: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8261C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C34C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C358: 386AE0FC  addi r3, r10, -0x1f04
	ctx.r[3].s64 = ctx.r[10].s64 + -7940;
	// 8261C35C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C37C: 4BE4AAA5  bl 0x82466e20
	ctx.lr = 0x8261C380;
	sub_82466E20(ctx, base);
	// 8261C380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C390 size=108
    let mut pc: u32 = 0x8261C390;
    'dispatch: loop {
        match pc {
            0x8261C390 => {
    //   block [0x8261C390..0x8261C3FC)
	// 8261C390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C39C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C3A4: 38EBC898  addi r7, r11, -0x3768
	ctx.r[7].s64 = ctx.r[11].s64 + -14184;
	// 8261C3A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C3AC: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8261C3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C3B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C3C0: 386AE12C  addi r3, r10, -0x1ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -7892;
	// 8261C3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C3E8: 4BE4AA39  bl 0x82466e20
	ctx.lr = 0x8261C3EC;
	sub_82466E20(ctx, base);
	// 8261C3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C400 size=112
    let mut pc: u32 = 0x8261C400;
    'dispatch: loop {
        match pc {
            0x8261C400 => {
    //   block [0x8261C400..0x8261C470)
	// 8261C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C410: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C414: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C41C: 390BC8E0  addi r8, r11, -0x3720
	ctx.r[8].s64 = ctx.r[11].s64 + -14112;
	// 8261C420: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C424: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8261C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C42C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C438: 386AE15C  addi r3, r10, -0x1ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -7844;
	// 8261C43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C45C: 4BE4A9C5  bl 0x82466e20
	ctx.lr = 0x8261C460;
	sub_82466E20(ctx, base);
	// 8261C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C470 size=108
    let mut pc: u32 = 0x8261C470;
    'dispatch: loop {
        match pc {
            0x8261C470 => {
    //   block [0x8261C470..0x8261C4DC)
	// 8261C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C47C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C484: 38EBC8F8  addi r7, r11, -0x3708
	ctx.r[7].s64 = ctx.r[11].s64 + -14088;
	// 8261C488: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C48C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8261C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C4A0: 386AE18C  addi r3, r10, -0x1e74
	ctx.r[3].s64 = ctx.r[10].s64 + -7796;
	// 8261C4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C4C8: 4BE4A959  bl 0x82466e20
	ctx.lr = 0x8261C4CC;
	sub_82466E20(ctx, base);
	// 8261C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C4E0 size=112
    let mut pc: u32 = 0x8261C4E0;
    'dispatch: loop {
        match pc {
            0x8261C4E0 => {
    //   block [0x8261C4E0..0x8261C550)
	// 8261C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C4EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C4F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C4F4: 38AAE15C  addi r5, r10, -0x1ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -7844;
	// 8261C4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C4FC: 390BC940  addi r8, r11, -0x36c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14016;
	// 8261C500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C504: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8261C508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C50C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C518: 386AE1BC  addi r3, r10, -0x1e44
	ctx.r[3].s64 = ctx.r[10].s64 + -7748;
	// 8261C51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C53C: 4BE4A8E5  bl 0x82466e20
	ctx.lr = 0x8261C540;
	sub_82466E20(ctx, base);
	// 8261C540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C550 size=100
    let mut pc: u32 = 0x8261C550;
    'dispatch: loop {
        match pc {
            0x8261C550 => {
    //   block [0x8261C550..0x8261C5B4)
	// 8261C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C55C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C564: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C570: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8261C574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C584: 386AE1EC  addi r3, r10, -0x1e14
	ctx.r[3].s64 = ctx.r[10].s64 + -7700;
	// 8261C588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C5A0: 4BE4A881  bl 0x82466e20
	ctx.lr = 0x8261C5A4;
	sub_82466E20(ctx, base);
	// 8261C5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C5B8 size=112
    let mut pc: u32 = 0x8261C5B8;
    'dispatch: loop {
        match pc {
            0x8261C5B8 => {
    //   block [0x8261C5B8..0x8261C628)
	// 8261C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C5C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C5CC: 38AAE1EC  addi r5, r10, -0x1e14
	ctx.r[5].s64 = ctx.r[10].s64 + -7700;
	// 8261C5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C5D4: 390BC958  addi r8, r11, -0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + -13992;
	// 8261C5D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261C5DC: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8261C5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C5E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C5F0: 386AE21C  addi r3, r10, -0x1de4
	ctx.r[3].s64 = ctx.r[10].s64 + -7652;
	// 8261C5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C614: 4BE4A80D  bl 0x82466e20
	ctx.lr = 0x8261C618;
	sub_82466E20(ctx, base);
	// 8261C618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C628 size=108
    let mut pc: u32 = 0x8261C628;
    'dispatch: loop {
        match pc {
            0x8261C628 => {
    //   block [0x8261C628..0x8261C694)
	// 8261C628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C634: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C63C: 38EBCA00  addi r7, r11, -0x3600
	ctx.r[7].s64 = ctx.r[11].s64 + -13824;
	// 8261C640: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261C644: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8261C648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C64C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C658: 386AE24C  addi r3, r10, -0x1db4
	ctx.r[3].s64 = ctx.r[10].s64 + -7604;
	// 8261C65C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C67C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C680: 4BE4A7A1  bl 0x82466e20
	ctx.lr = 0x8261C684;
	sub_82466E20(ctx, base);
	// 8261C684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C698 size=112
    let mut pc: u32 = 0x8261C698;
    'dispatch: loop {
        match pc {
            0x8261C698 => {
    //   block [0x8261C698..0x8261C708)
	// 8261C698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C6A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C6AC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C6B4: 390BCA30  addi r8, r11, -0x35d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13776;
	// 8261C6B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C6BC: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8261C6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C6D0: 386AE27C  addi r3, r10, -0x1d84
	ctx.r[3].s64 = ctx.r[10].s64 + -7556;
	// 8261C6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C6F4: 4BE4A72D  bl 0x82466e20
	ctx.lr = 0x8261C6F8;
	sub_82466E20(ctx, base);
	// 8261C6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


