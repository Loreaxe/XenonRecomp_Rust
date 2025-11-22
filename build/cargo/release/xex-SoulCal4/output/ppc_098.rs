pub fn sub_826295B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826295B0 size=100
    let mut pc: u32 = 0x826295B0;
    'dispatch: loop {
        match pc {
            0x826295B0 => {
    //   block [0x826295B0..0x82629614)
	// 826295B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826295B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826295B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826295BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826295C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826295C4: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826295C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826295CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826295D0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826295D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826295D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826295DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826295E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826295E4: 386A3A3C  addi r3, r10, 0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 14908;
	// 826295E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826295EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826295F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826295F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826295F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826295FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629600: 4BE3D821  bl 0x82466e20
	ctx.lr = 0x82629604;
	sub_82466E20(ctx, base);
	// 82629604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262960C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629618 size=112
    let mut pc: u32 = 0x82629618;
    'dispatch: loop {
        match pc {
            0x82629618 => {
    //   block [0x82629618..0x82629688)
	// 82629618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629628: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262962C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629634: 390B7960  addi r8, r11, 0x7960
	ctx.r[8].s64 = ctx.r[11].s64 + 31072;
	// 82629638: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262963C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82629640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262964C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629650: 386A3A6C  addi r3, r10, 0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 14956;
	// 82629654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262966C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629674: 4BE3D7AD  bl 0x82466e20
	ctx.lr = 0x82629678;
	sub_82466E20(ctx, base);
	// 82629678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629688 size=112
    let mut pc: u32 = 0x82629688;
    'dispatch: loop {
        match pc {
            0x82629688 => {
    //   block [0x82629688..0x826296F8)
	// 82629688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629698: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262969C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826296A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826296A4: 390B7978  addi r8, r11, 0x7978
	ctx.r[8].s64 = ctx.r[11].s64 + 31096;
	// 826296A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826296AC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826296B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826296B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826296B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826296BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826296C0: 386A3A9C  addi r3, r10, 0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15004;
	// 826296C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826296C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826296CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826296D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826296D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826296D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826296DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826296E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826296E4: 4BE3D73D  bl 0x82466e20
	ctx.lr = 0x826296E8;
	sub_82466E20(ctx, base);
	// 826296E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826296EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826296F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826296F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826296F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826296F8 size=112
    let mut pc: u32 = 0x826296F8;
    'dispatch: loop {
        match pc {
            0x826296F8 => {
    //   block [0x826296F8..0x82629768)
	// 826296F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826296FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629708: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262970C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629714: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 82629718: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262971C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82629720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262972C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629730: 386A3ACC  addi r3, r10, 0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + 15052;
	// 82629734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262973C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262974C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629754: 4BE3D6CD  bl 0x82466e20
	ctx.lr = 0x82629758;
	sub_82466E20(ctx, base);
	// 82629758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262975C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629768 size=112
    let mut pc: u32 = 0x82629768;
    'dispatch: loop {
        match pc {
            0x82629768 => {
    //   block [0x82629768..0x826297D8)
	// 82629768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262976C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629778: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262977C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629784: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82629788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262978C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82629790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262979C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826297A0: 386A3AFC  addi r3, r10, 0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + 15100;
	// 826297A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826297A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826297AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826297B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826297B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826297B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826297BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826297C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826297C4: 4BE3D65D  bl 0x82466e20
	ctx.lr = 0x826297C8;
	sub_82466E20(ctx, base);
	// 826297C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826297CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826297D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826297D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826297D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826297D8 size=112
    let mut pc: u32 = 0x826297D8;
    'dispatch: loop {
        match pc {
            0x826297D8 => {
    //   block [0x826297D8..0x82629848)
	// 826297D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826297DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826297E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826297E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826297E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826297EC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826297F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826297F4: 390B7A08  addi r8, r11, 0x7a08
	ctx.r[8].s64 = ctx.r[11].s64 + 31240;
	// 826297F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826297FC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82629800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629810: 386A3B2C  addi r3, r10, 0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15148;
	// 82629814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262981C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262982C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629834: 4BE3D5ED  bl 0x82466e20
	ctx.lr = 0x82629838;
	sub_82466E20(ctx, base);
	// 82629838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262983C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629848 size=112
    let mut pc: u32 = 0x82629848;
    'dispatch: loop {
        match pc {
            0x82629848 => {
    //   block [0x82629848..0x826298B8)
	// 82629848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262984C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629858: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262985C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629864: 390B7A20  addi r8, r11, 0x7a20
	ctx.r[8].s64 = ctx.r[11].s64 + 31264;
	// 82629868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262986C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82629870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262987C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629880: 386A3B5C  addi r3, r10, 0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15196;
	// 82629884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262988C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262989C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826298A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826298A4: 4BE3D57D  bl 0x82466e20
	ctx.lr = 0x826298A8;
	sub_82466E20(ctx, base);
	// 826298A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826298AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826298B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826298B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826298B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826298B8 size=112
    let mut pc: u32 = 0x826298B8;
    'dispatch: loop {
        match pc {
            0x826298B8 => {
    //   block [0x826298B8..0x82629928)
	// 826298B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826298BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826298C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826298C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826298C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826298CC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826298D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826298D4: 390B7A38  addi r8, r11, 0x7a38
	ctx.r[8].s64 = ctx.r[11].s64 + 31288;
	// 826298D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826298DC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826298E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826298E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826298E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826298EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826298F0: 386A3B8C  addi r3, r10, 0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15244;
	// 826298F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826298F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826298FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262990C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629914: 4BE3D50D  bl 0x82466e20
	ctx.lr = 0x82629918;
	sub_82466E20(ctx, base);
	// 82629918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629928 size=112
    let mut pc: u32 = 0x82629928;
    'dispatch: loop {
        match pc {
            0x82629928 => {
    //   block [0x82629928..0x82629998)
	// 82629928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262992C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629938: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262993C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629944: 390B7A80  addi r8, r11, 0x7a80
	ctx.r[8].s64 = ctx.r[11].s64 + 31360;
	// 82629948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262994C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82629950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262995C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629960: 386A3BBC  addi r3, r10, 0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15292;
	// 82629964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262996C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262997C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629984: 4BE3D49D  bl 0x82466e20
	ctx.lr = 0x82629988;
	sub_82466E20(ctx, base);
	// 82629988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262998C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629998 size=112
    let mut pc: u32 = 0x82629998;
    'dispatch: loop {
        match pc {
            0x82629998 => {
    //   block [0x82629998..0x82629A08)
	// 82629998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826299A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826299A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826299A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826299AC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826299B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826299B4: 390B7AC8  addi r8, r11, 0x7ac8
	ctx.r[8].s64 = ctx.r[11].s64 + 31432;
	// 826299B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826299BC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826299C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826299C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826299C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826299CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826299D0: 386A3BEC  addi r3, r10, 0x3bec
	ctx.r[3].s64 = ctx.r[10].s64 + 15340;
	// 826299D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826299D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826299DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826299E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826299E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826299E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826299EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826299F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826299F4: 4BE3D42D  bl 0x82466e20
	ctx.lr = 0x826299F8;
	sub_82466E20(ctx, base);
	// 826299F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826299FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629A08 size=112
    let mut pc: u32 = 0x82629A08;
    'dispatch: loop {
        match pc {
            0x82629A08 => {
    //   block [0x82629A08..0x82629A78)
	// 82629A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629A14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629A1C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629A24: 390B7AE0  addi r8, r11, 0x7ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 31456;
	// 82629A28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629A2C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82629A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629A40: 386A3C1C  addi r3, r10, 0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15388;
	// 82629A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629A64: 4BE3D3BD  bl 0x82466e20
	ctx.lr = 0x82629A68;
	sub_82466E20(ctx, base);
	// 82629A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629A78 size=116
    let mut pc: u32 = 0x82629A78;
    'dispatch: loop {
        match pc {
            0x82629A78 => {
    //   block [0x82629A78..0x82629AEC)
	// 82629A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629A84: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82629A88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82629A8C: 390A7B10  addi r8, r10, 0x7b10
	ctx.r[8].s64 = ctx.r[10].s64 + 31504;
	// 82629A90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629A94: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629A98: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629A9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629AA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82629AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629AAC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82629AB0: 396B36C8  addi r11, r11, 0x36c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14024;
	// 82629AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629AB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629ABC: 386A3C4C  addi r3, r10, 0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15436;
	// 82629AC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82629AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629AC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82629ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629AD8: 4BE3D349  bl 0x82466e20
	ctx.lr = 0x82629ADC;
	sub_82466E20(ctx, base);
	// 82629ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629AF0 size=116
    let mut pc: u32 = 0x82629AF0;
    'dispatch: loop {
        match pc {
            0x82629AF0 => {
    //   block [0x82629AF0..0x82629B64)
	// 82629AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629AFC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82629B00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82629B04: 390A7B88  addi r8, r10, 0x7b88
	ctx.r[8].s64 = ctx.r[10].s64 + 31624;
	// 82629B08: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629B10: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629B14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629B18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82629B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629B24: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 82629B28: 396B36E0  addi r11, r11, 0x36e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14048;
	// 82629B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629B34: 386A3C7C  addi r3, r10, 0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15484;
	// 82629B38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82629B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629B40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82629B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629B50: 4BE3D2D1  bl 0x82466e20
	ctx.lr = 0x82629B54;
	sub_82466E20(ctx, base);
	// 82629B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82629B68 size=24
    let mut pc: u32 = 0x82629B68;
    'dispatch: loop {
        match pc {
            0x82629B68 => {
    //   block [0x82629B68..0x82629B80)
	// 82629B68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629B6C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82629B70: 394AB410  addi r10, r10, -0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + -19440;
	// 82629B74: 816B7C18  lwz r11, 0x7c18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31768 as u32) ) } as u64;
	// 82629B78: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82629B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629B80 size=116
    let mut pc: u32 = 0x82629B80;
    'dispatch: loop {
        match pc {
            0x82629B80 => {
    //   block [0x82629B80..0x82629BF4)
	// 82629B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629B8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82629B90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629B94: 392B370C  addi r9, r11, 0x370c
	ctx.r[9].s64 = ctx.r[11].s64 + 14092;
	// 82629B98: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629B9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629BA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82629BA4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82629BA8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82629BAC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82629BB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629BB4: 396BB410  addi r11, r11, -0x4bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -19440;
	// 82629BB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82629BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629BC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82629BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629BC8: 386A3CAC  addi r3, r10, 0x3cac
	ctx.r[3].s64 = ctx.r[10].s64 + 15532;
	// 82629BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82629BD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82629BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629BD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82629BDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82629BE0: 4BE3D241  bl 0x82466e20
	ctx.lr = 0x82629BE4;
	sub_82466E20(ctx, base);
	// 82629BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629BF8 size=112
    let mut pc: u32 = 0x82629BF8;
    'dispatch: loop {
        match pc {
            0x82629BF8 => {
    //   block [0x82629BF8..0x82629C68)
	// 82629BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629C0C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629C14: 390B7C20  addi r8, r11, 0x7c20
	ctx.r[8].s64 = ctx.r[11].s64 + 31776;
	// 82629C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82629C1C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 82629C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629C30: 386A3CDC  addi r3, r10, 0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 15580;
	// 82629C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629C54: 4BE3D1CD  bl 0x82466e20
	ctx.lr = 0x82629C58;
	sub_82466E20(ctx, base);
	// 82629C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629C68 size=112
    let mut pc: u32 = 0x82629C68;
    'dispatch: loop {
        match pc {
            0x82629C68 => {
    //   block [0x82629C68..0x82629CD8)
	// 82629C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629C74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629C7C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629C84: 390B7C80  addi r8, r11, 0x7c80
	ctx.r[8].s64 = ctx.r[11].s64 + 31872;
	// 82629C88: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82629C8C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82629C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629C94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629CA0: 386A3D0C  addi r3, r10, 0x3d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15628;
	// 82629CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629CC4: 4BE3D15D  bl 0x82466e20
	ctx.lr = 0x82629CC8;
	sub_82466E20(ctx, base);
	// 82629CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629CD8 size=112
    let mut pc: u32 = 0x82629CD8;
    'dispatch: loop {
        match pc {
            0x82629CD8 => {
    //   block [0x82629CD8..0x82629D48)
	// 82629CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629CE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629CE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629CEC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629CF4: 390B7D28  addi r8, r11, 0x7d28
	ctx.r[8].s64 = ctx.r[11].s64 + 32040;
	// 82629CF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82629CFC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 82629D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629D04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629D10: 386A3D3C  addi r3, r10, 0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15676;
	// 82629D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629D34: 4BE3D0ED  bl 0x82466e20
	ctx.lr = 0x82629D38;
	sub_82466E20(ctx, base);
	// 82629D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629D48 size=112
    let mut pc: u32 = 0x82629D48;
    'dispatch: loop {
        match pc {
            0x82629D48 => {
    //   block [0x82629D48..0x82629DB8)
	// 82629D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629D5C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629D64: 390B7DA0  addi r8, r11, 0x7da0
	ctx.r[8].s64 = ctx.r[11].s64 + 32160;
	// 82629D68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82629D6C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82629D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629D80: 386A3D6C  addi r3, r10, 0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15724;
	// 82629D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629DA4: 4BE3D07D  bl 0x82466e20
	ctx.lr = 0x82629DA8;
	sub_82466E20(ctx, base);
	// 82629DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629DB8 size=112
    let mut pc: u32 = 0x82629DB8;
    'dispatch: loop {
        match pc {
            0x82629DB8 => {
    //   block [0x82629DB8..0x82629E28)
	// 82629DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629DC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629DCC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629DD4: 390B7DE8  addi r8, r11, 0x7de8
	ctx.r[8].s64 = ctx.r[11].s64 + 32232;
	// 82629DD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82629DDC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 82629DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629DF0: 386A3D9C  addi r3, r10, 0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15772;
	// 82629DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629E14: 4BE3D00D  bl 0x82466e20
	ctx.lr = 0x82629E18;
	sub_82466E20(ctx, base);
	// 82629E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629E28 size=112
    let mut pc: u32 = 0x82629E28;
    'dispatch: loop {
        match pc {
            0x82629E28 => {
    //   block [0x82629E28..0x82629E98)
	// 82629E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629E34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629E38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629E3C: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629E44: 390B7E78  addi r8, r11, 0x7e78
	ctx.r[8].s64 = ctx.r[11].s64 + 32376;
	// 82629E48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82629E4C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82629E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629E60: 386A3DCC  addi r3, r10, 0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15820;
	// 82629E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629E84: 4BE3CF9D  bl 0x82466e20
	ctx.lr = 0x82629E88;
	sub_82466E20(ctx, base);
	// 82629E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629E98 size=112
    let mut pc: u32 = 0x82629E98;
    'dispatch: loop {
        match pc {
            0x82629E98 => {
    //   block [0x82629E98..0x82629F08)
	// 82629E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629EA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629EAC: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 82629EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629EB4: 390B7ED8  addi r8, r11, 0x7ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 32472;
	// 82629EB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82629EBC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82629EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629EC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629ED0: 386A3DFC  addi r3, r10, 0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15868;
	// 82629ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629EF4: 4BE3CF2D  bl 0x82466e20
	ctx.lr = 0x82629EF8;
	sub_82466E20(ctx, base);
	// 82629EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629F08 size=112
    let mut pc: u32 = 0x82629F08;
    'dispatch: loop {
        match pc {
            0x82629F08 => {
    //   block [0x82629F08..0x82629F78)
	// 82629F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629F1C: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 82629F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629F24: 390B7F20  addi r8, r11, 0x7f20
	ctx.r[8].s64 = ctx.r[11].s64 + 32544;
	// 82629F28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629F2C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 82629F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629F40: 386A3E2C  addi r3, r10, 0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15916;
	// 82629F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629F64: 4BE3CEBD  bl 0x82466e20
	ctx.lr = 0x82629F68;
	sub_82466E20(ctx, base);
	// 82629F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629F78 size=112
    let mut pc: u32 = 0x82629F78;
    'dispatch: loop {
        match pc {
            0x82629F78 => {
    //   block [0x82629F78..0x82629FE8)
	// 82629F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629F88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629F8C: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 82629F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629F94: 390B7F50  addi r8, r11, 0x7f50
	ctx.r[8].s64 = ctx.r[11].s64 + 32592;
	// 82629F98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82629F9C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82629FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629FB0: 386A3E5C  addi r3, r10, 0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15964;
	// 82629FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629FD4: 4BE3CE4D  bl 0x82466e20
	ctx.lr = 0x82629FD8;
	sub_82466E20(ctx, base);
	// 82629FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629FE8 size=100
    let mut pc: u32 = 0x82629FE8;
    'dispatch: loop {
        match pc {
            0x82629FE8 => {
    //   block [0x82629FE8..0x8262A04C)
	// 82629FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629FFC: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 8262A000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A008: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8262A00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A01C: 386A3E8C  addi r3, r10, 0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16012;
	// 8262A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262A02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262A034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A038: 4BE3CDE9  bl 0x82466e20
	ctx.lr = 0x8262A03C;
	sub_82466E20(ctx, base);
	// 8262A03C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A050 size=112
    let mut pc: u32 = 0x8262A050;
    'dispatch: loop {
        match pc {
            0x8262A050 => {
    //   block [0x8262A050..0x8262A0C0)
	// 8262A050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A05C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A060: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A064: 38AA3DFC  addi r5, r10, 0x3dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15868;
	// 8262A068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A06C: 390B7F80  addi r8, r11, 0x7f80
	ctx.r[8].s64 = ctx.r[11].s64 + 32640;
	// 8262A070: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A074: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8262A078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A07C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A088: 386A3EBC  addi r3, r10, 0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 16060;
	// 8262A08C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A0AC: 4BE3CD75  bl 0x82466e20
	ctx.lr = 0x8262A0B0;
	sub_82466E20(ctx, base);
	// 8262A0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A0C0 size=112
    let mut pc: u32 = 0x8262A0C0;
    'dispatch: loop {
        match pc {
            0x8262A0C0 => {
    //   block [0x8262A0C0..0x8262A130)
	// 8262A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A0C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A0CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A0D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A0D4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A0DC: 390B7F98  addi r8, r11, 0x7f98
	ctx.r[8].s64 = ctx.r[11].s64 + 32664;
	// 8262A0E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262A0E4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8262A0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A0F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A0F8: 386A3EEC  addi r3, r10, 0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + 16108;
	// 8262A0FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A11C: 4BE3CD05  bl 0x82466e20
	ctx.lr = 0x8262A120;
	sub_82466E20(ctx, base);
	// 8262A120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A130 size=112
    let mut pc: u32 = 0x8262A130;
    'dispatch: loop {
        match pc {
            0x8262A130 => {
    //   block [0x8262A130..0x8262A1A0)
	// 8262A130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A13C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A140: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A144: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A14C: 390B7FF8  addi r8, r11, 0x7ff8
	ctx.r[8].s64 = ctx.r[11].s64 + 32760;
	// 8262A150: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A154: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8262A158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A15C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A168: 386A3F1C  addi r3, r10, 0x3f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 16156;
	// 8262A16C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A18C: 4BE3CC95  bl 0x82466e20
	ctx.lr = 0x8262A190;
	sub_82466E20(ctx, base);
	// 8262A190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A1A0 size=112
    let mut pc: u32 = 0x8262A1A0;
    'dispatch: loop {
        match pc {
            0x8262A1A0 => {
    //   block [0x8262A1A0..0x8262A210)
	// 8262A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A1AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A1B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A1B4: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A1BC: 390B8010  addi r8, r11, -0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -32752;
	// 8262A1C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262A1C4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8262A1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A1D8: 386A3F4C  addi r3, r10, 0x3f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 16204;
	// 8262A1DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A1FC: 4BE3CC25  bl 0x82466e20
	ctx.lr = 0x8262A200;
	sub_82466E20(ctx, base);
	// 8262A200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A210 size=112
    let mut pc: u32 = 0x8262A210;
    'dispatch: loop {
        match pc {
            0x8262A210 => {
    //   block [0x8262A210..0x8262A280)
	// 8262A210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A21C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A220: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A224: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262A228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A22C: 390B8040  addi r8, r11, -0x7fc0
	ctx.r[8].s64 = ctx.r[11].s64 + -32704;
	// 8262A230: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A234: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8262A238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A248: 386A3F7C  addi r3, r10, 0x3f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 16252;
	// 8262A24C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A26C: 4BE3CBB5  bl 0x82466e20
	ctx.lr = 0x8262A270;
	sub_82466E20(ctx, base);
	// 8262A270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A280 size=24
    let mut pc: u32 = 0x8262A280;
    'dispatch: loop {
        match pc {
            0x8262A280 => {
    //   block [0x8262A280..0x8262A298)
	// 8262A280: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262A284: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A288: 394AB4B8  addi r10, r10, -0x4b48
	ctx.r[10].s64 = ctx.r[10].s64 + -19272;
	// 8262A28C: 816B7C1C  lwz r11, 0x7c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31772 as u32) ) } as u64;
	// 8262A290: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A298 size=112
    let mut pc: u32 = 0x8262A298;
    'dispatch: loop {
        match pc {
            0x8262A298 => {
    //   block [0x8262A298..0x8262A308)
	// 8262A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A2A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A2A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A2AC: 392A3768  addi r9, r10, 0x3768
	ctx.r[9].s64 = ctx.r[10].s64 + 14184;
	// 8262A2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A2B4: 390BB4B8  addi r8, r11, -0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + -19272;
	// 8262A2B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262A2BC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8262A2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A2D0: 386A3FAC  addi r3, r10, 0x3fac
	ctx.r[3].s64 = ctx.r[10].s64 + 16300;
	// 8262A2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A2F4: 4BE3CB2D  bl 0x82466e20
	ctx.lr = 0x8262A2F8;
	sub_82466E20(ctx, base);
	// 8262A2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A308 size=108
    let mut pc: u32 = 0x8262A308;
    'dispatch: loop {
        match pc {
            0x8262A308 => {
    //   block [0x8262A308..0x8262A374)
	// 8262A308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A314: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A31C: 38EB8058  addi r7, r11, -0x7fa8
	ctx.r[7].s64 = ctx.r[11].s64 + -32680;
	// 8262A320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262A324: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8262A328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A32C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A338: 386A3FDC  addi r3, r10, 0x3fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 16348;
	// 8262A33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A360: 4BE3CAC1  bl 0x82466e20
	ctx.lr = 0x8262A364;
	sub_82466E20(ctx, base);
	// 8262A364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A378 size=108
    let mut pc: u32 = 0x8262A378;
    'dispatch: loop {
        match pc {
            0x8262A378 => {
    //   block [0x8262A378..0x8262A3E4)
	// 8262A378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A384: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A38C: 38EB8070  addi r7, r11, -0x7f90
	ctx.r[7].s64 = ctx.r[11].s64 + -32656;
	// 8262A390: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262A394: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8262A398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A39C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A3A8: 386A400C  addi r3, r10, 0x400c
	ctx.r[3].s64 = ctx.r[10].s64 + 16396;
	// 8262A3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A3D0: 4BE3CA51  bl 0x82466e20
	ctx.lr = 0x8262A3D4;
	sub_82466E20(ctx, base);
	// 8262A3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A3E8 size=116
    let mut pc: u32 = 0x8262A3E8;
    'dispatch: loop {
        match pc {
            0x8262A3E8 => {
    //   block [0x8262A3E8..0x8262A45C)
	// 8262A3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A3F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A3F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A3FC: 390B80BC  addi r8, r11, -0x7f44
	ctx.r[8].s64 = ctx.r[11].s64 + -32580;
	// 8262A400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A404: 392A3820  addi r9, r10, 0x3820
	ctx.r[9].s64 = ctx.r[10].s64 + 14368;
	// 8262A408: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A40C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262A410: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A414: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A41C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A42C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A430: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8262A434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A438: 386B403C  addi r3, r11, 0x403c
	ctx.r[3].s64 = ctx.r[11].s64 + 16444;
	// 8262A43C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A440: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A448: 4BE3C9D9  bl 0x82466e20
	ctx.lr = 0x8262A44C;
	sub_82466E20(ctx, base);
	// 8262A44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A460 size=108
    let mut pc: u32 = 0x8262A460;
    'dispatch: loop {
        match pc {
            0x8262A460 => {
    //   block [0x8262A460..0x8262A4CC)
	// 8262A460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A46C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A474: 38EB80D8  addi r7, r11, -0x7f28
	ctx.r[7].s64 = ctx.r[11].s64 + -32552;
	// 8262A478: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262A47C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 8262A480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A490: 386A406C  addi r3, r10, 0x406c
	ctx.r[3].s64 = ctx.r[10].s64 + 16492;
	// 8262A494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A4B8: 4BE3C969  bl 0x82466e20
	ctx.lr = 0x8262A4BC;
	sub_82466E20(ctx, base);
	// 8262A4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A4D0 size=24
    let mut pc: u32 = 0x8262A4D0;
    'dispatch: loop {
        match pc {
            0x8262A4D0 => {
    //   block [0x8262A4D0..0x8262A4E8)
	// 8262A4D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A4D4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A4D8: 394AB500  addi r10, r10, -0x4b00
	ctx.r[10].s64 = ctx.r[10].s64 + -19200;
	// 8262A4DC: 816B80D4  lwz r11, -0x7f2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32556 as u32) ) } as u64;
	// 8262A4E0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262A4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A4E8 size=116
    let mut pc: u32 = 0x8262A4E8;
    'dispatch: loop {
        match pc {
            0x8262A4E8 => {
    //   block [0x8262A4E8..0x8262A55C)
	// 8262A4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A4F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A4FC: 390BB500  addi r8, r11, -0x4b00
	ctx.r[8].s64 = ctx.r[11].s64 + -19200;
	// 8262A500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A504: 392A387C  addi r9, r10, 0x387c
	ctx.r[9].s64 = ctx.r[10].s64 + 14460;
	// 8262A508: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A50C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8262A510: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A514: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A51C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A52C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A530: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8262A534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A538: 386B409C  addi r3, r11, 0x409c
	ctx.r[3].s64 = ctx.r[11].s64 + 16540;
	// 8262A53C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8262A540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A548: 4BE3C8D9  bl 0x82466e20
	ctx.lr = 0x8262A54C;
	sub_82466E20(ctx, base);
	// 8262A54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A560 size=108
    let mut pc: u32 = 0x8262A560;
    'dispatch: loop {
        match pc {
            0x8262A560 => {
    //   block [0x8262A560..0x8262A5CC)
	// 8262A560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A56C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A574: 38EB8140  addi r7, r11, -0x7ec0
	ctx.r[7].s64 = ctx.r[11].s64 + -32448;
	// 8262A578: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262A57C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8262A580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A590: 386A40CC  addi r3, r10, 0x40cc
	ctx.r[3].s64 = ctx.r[10].s64 + 16588;
	// 8262A594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A5B8: 4BE3C869  bl 0x82466e20
	ctx.lr = 0x8262A5BC;
	sub_82466E20(ctx, base);
	// 8262A5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A5D0 size=112
    let mut pc: u32 = 0x8262A5D0;
    'dispatch: loop {
        match pc {
            0x8262A5D0 => {
    //   block [0x8262A5D0..0x8262A640)
	// 8262A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A5E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A5E4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A5EC: 390B8170  addi r8, r11, -0x7e90
	ctx.r[8].s64 = ctx.r[11].s64 + -32400;
	// 8262A5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A5F4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8262A5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A5FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A608: 386A40FC  addi r3, r10, 0x40fc
	ctx.r[3].s64 = ctx.r[10].s64 + 16636;
	// 8262A60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A62C: 4BE3C7F5  bl 0x82466e20
	ctx.lr = 0x8262A630;
	sub_82466E20(ctx, base);
	// 8262A630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A640 size=112
    let mut pc: u32 = 0x8262A640;
    'dispatch: loop {
        match pc {
            0x8262A640 => {
    //   block [0x8262A640..0x8262A6B0)
	// 8262A640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A64C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A650: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A654: 392A38C0  addi r9, r10, 0x38c0
	ctx.r[9].s64 = ctx.r[10].s64 + 14528;
	// 8262A658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A65C: 390B8190  addi r8, r11, -0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + -32368;
	// 8262A660: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262A664: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8262A668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A66C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A678: 386A412C  addi r3, r10, 0x412c
	ctx.r[3].s64 = ctx.r[10].s64 + 16684;
	// 8262A67C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A680: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A69C: 4BE3C785  bl 0x82466e20
	ctx.lr = 0x8262A6A0;
	sub_82466E20(ctx, base);
	// 8262A6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A6B0 size=112
    let mut pc: u32 = 0x8262A6B0;
    'dispatch: loop {
        match pc {
            0x8262A6B0 => {
    //   block [0x8262A6B0..0x8262A720)
	// 8262A6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A6BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A6C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A6C4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A6CC: 390B81D8  addi r8, r11, -0x7e28
	ctx.r[8].s64 = ctx.r[11].s64 + -32296;
	// 8262A6D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A6D4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8262A6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A6DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A6E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A6E8: 386A415C  addi r3, r10, 0x415c
	ctx.r[3].s64 = ctx.r[10].s64 + 16732;
	// 8262A6EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A70C: 4BE3C715  bl 0x82466e20
	ctx.lr = 0x8262A710;
	sub_82466E20(ctx, base);
	// 8262A710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A720 size=112
    let mut pc: u32 = 0x8262A720;
    'dispatch: loop {
        match pc {
            0x8262A720 => {
    //   block [0x8262A720..0x8262A790)
	// 8262A720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A72C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A730: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A734: 392A38EC  addi r9, r10, 0x38ec
	ctx.r[9].s64 = ctx.r[10].s64 + 14572;
	// 8262A738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A73C: 390B81F0  addi r8, r11, -0x7e10
	ctx.r[8].s64 = ctx.r[11].s64 + -32272;
	// 8262A740: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262A744: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8262A748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A74C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A758: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 8262A75C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A760: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A77C: 4BE3C6A5  bl 0x82466e20
	ctx.lr = 0x8262A780;
	sub_82466E20(ctx, base);
	// 8262A780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A790 size=112
    let mut pc: u32 = 0x8262A790;
    'dispatch: loop {
        match pc {
            0x8262A790 => {
    //   block [0x8262A790..0x8262A800)
	// 8262A790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A7A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A7A4: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A7AC: 390B8280  addi r8, r11, -0x7d80
	ctx.r[8].s64 = ctx.r[11].s64 + -32128;
	// 8262A7B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A7B4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8262A7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A7BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A7C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A7C8: 386A41BC  addi r3, r10, 0x41bc
	ctx.r[3].s64 = ctx.r[10].s64 + 16828;
	// 8262A7CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A7EC: 4BE3C635  bl 0x82466e20
	ctx.lr = 0x8262A7F0;
	sub_82466E20(ctx, base);
	// 8262A7F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A800 size=112
    let mut pc: u32 = 0x8262A800;
    'dispatch: loop {
        match pc {
            0x8262A800 => {
    //   block [0x8262A800..0x8262A870)
	// 8262A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A814: 38AA421C  addi r5, r10, 0x421c
	ctx.r[5].s64 = ctx.r[10].s64 + 16924;
	// 8262A818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A81C: 390B8298  addi r8, r11, -0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + -32104;
	// 8262A820: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262A824: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8262A828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A82C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A838: 386A41EC  addi r3, r10, 0x41ec
	ctx.r[3].s64 = ctx.r[10].s64 + 16876;
	// 8262A83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262A840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A85C: 4BE3C5C5  bl 0x82466e20
	ctx.lr = 0x8262A860;
	sub_82466E20(ctx, base);
	// 8262A860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A870 size=100
    let mut pc: u32 = 0x8262A870;
    'dispatch: loop {
        match pc {
            0x8262A870 => {
    //   block [0x8262A870..0x8262A8D4)
	// 8262A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A87C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A884: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262A888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A890: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8262A894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A8A4: 386A421C  addi r3, r10, 0x421c
	ctx.r[3].s64 = ctx.r[10].s64 + 16924;
	// 8262A8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A8AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A8B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262A8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A8B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262A8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A8C0: 4BE3C561  bl 0x82466e20
	ctx.lr = 0x8262A8C4;
	sub_82466E20(ctx, base);
	// 8262A8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262A8D8 size=24
    let mut pc: u32 = 0x8262A8D8;
    'dispatch: loop {
        match pc {
            0x8262A8D8 => {
    //   block [0x8262A8D8..0x8262A8F0)
	// 8262A8D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A8DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262A8E0: 394AB5D8  addi r10, r10, -0x4a28
	ctx.r[10].s64 = ctx.r[10].s64 + -18984;
	// 8262A8E4: 816B8310  lwz r11, -0x7cf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31984 as u32) ) } as u64;
	// 8262A8E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262A8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A8F0 size=116
    let mut pc: u32 = 0x8262A8F0;
    'dispatch: loop {
        match pc {
            0x8262A8F0 => {
    //   block [0x8262A8F0..0x8262A964)
	// 8262A8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A8FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262A904: 390BB5D8  addi r8, r11, -0x4a28
	ctx.r[8].s64 = ctx.r[11].s64 + -18984;
	// 8262A908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A90C: 392A3928  addi r9, r10, 0x3928
	ctx.r[9].s64 = ctx.r[10].s64 + 14632;
	// 8262A910: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A914: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8262A918: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A91C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262A920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A924: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A934: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262A938: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8262A93C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262A940: 386B424C  addi r3, r11, 0x424c
	ctx.r[3].s64 = ctx.r[11].s64 + 16972;
	// 8262A944: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262A948: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A950: 4BE3C4D1  bl 0x82466e20
	ctx.lr = 0x8262A954;
	sub_82466E20(ctx, base);
	// 8262A954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A968 size=108
    let mut pc: u32 = 0x8262A968;
    'dispatch: loop {
        match pc {
            0x8262A968 => {
    //   block [0x8262A968..0x8262A9D4)
	// 8262A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A974: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A97C: 38EB8314  addi r7, r11, -0x7cec
	ctx.r[7].s64 = ctx.r[11].s64 + -31980;
	// 8262A980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262A984: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8262A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262A98C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262A994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262A998: 386A427C  addi r3, r10, 0x427c
	ctx.r[3].s64 = ctx.r[10].s64 + 17020;
	// 8262A99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262A9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262A9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262A9C0: 4BE3C461  bl 0x82466e20
	ctx.lr = 0x8262A9C4;
	sub_82466E20(ctx, base);
	// 8262A9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262A9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262A9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262A9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262A9D8 size=112
    let mut pc: u32 = 0x8262A9D8;
    'dispatch: loop {
        match pc {
            0x8262A9D8 => {
    //   block [0x8262A9D8..0x8262AA48)
	// 8262A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262A9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262A9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262A9E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262A9E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262A9EC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262A9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262A9F4: 390B8344  addi r8, r11, -0x7cbc
	ctx.r[8].s64 = ctx.r[11].s64 + -31932;
	// 8262A9F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262A9FC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8262AA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AA04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AA10: 386A42AC  addi r3, r10, 0x42ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17068;
	// 8262AA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AA34: 4BE3C3ED  bl 0x82466e20
	ctx.lr = 0x8262AA38;
	sub_82466E20(ctx, base);
	// 8262AA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AA48 size=112
    let mut pc: u32 = 0x8262AA48;
    'dispatch: loop {
        match pc {
            0x8262AA48 => {
    //   block [0x8262AA48..0x8262AAB8)
	// 8262AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AA54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AA58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AA5C: 392A394C  addi r9, r10, 0x394c
	ctx.r[9].s64 = ctx.r[10].s64 + 14668;
	// 8262AA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AA64: 390B8360  addi r8, r11, -0x7ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -31904;
	// 8262AA68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262AA6C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8262AA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AA80: 386A42DC  addi r3, r10, 0x42dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17116;
	// 8262AA84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AA88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AAA4: 4BE3C37D  bl 0x82466e20
	ctx.lr = 0x8262AAA8;
	sub_82466E20(ctx, base);
	// 8262AAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AAB8 size=112
    let mut pc: u32 = 0x8262AAB8;
    'dispatch: loop {
        match pc {
            0x8262AAB8 => {
    //   block [0x8262AAB8..0x8262AB28)
	// 8262AAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AAC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AAC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AACC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AAD4: 390B8408  addi r8, r11, -0x7bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -31736;
	// 8262AAD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262AADC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8262AAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AAE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AAF0: 386A430C  addi r3, r10, 0x430c
	ctx.r[3].s64 = ctx.r[10].s64 + 17164;
	// 8262AAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AB14: 4BE3C30D  bl 0x82466e20
	ctx.lr = 0x8262AB18;
	sub_82466E20(ctx, base);
	// 8262AB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AB28 size=108
    let mut pc: u32 = 0x8262AB28;
    'dispatch: loop {
        match pc {
            0x8262AB28 => {
    //   block [0x8262AB28..0x8262AB94)
	// 8262AB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AB34: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AB3C: 38EB8420  addi r7, r11, -0x7be0
	ctx.r[7].s64 = ctx.r[11].s64 + -31712;
	// 8262AB40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262AB44: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8262AB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AB4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262AB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AB58: 386A433C  addi r3, r10, 0x433c
	ctx.r[3].s64 = ctx.r[10].s64 + 17212;
	// 8262AB5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262AB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AB80: 4BE3C2A1  bl 0x82466e20
	ctx.lr = 0x8262AB84;
	sub_82466E20(ctx, base);
	// 8262AB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AB98 size=112
    let mut pc: u32 = 0x8262AB98;
    'dispatch: loop {
        match pc {
            0x8262AB98 => {
    //   block [0x8262AB98..0x8262AC08)
	// 8262AB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ABA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ABA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ABA8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ABAC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262ABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ABB4: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8262ABB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262ABBC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8262ABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ABC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ABC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ABD0: 386A436C  addi r3, r10, 0x436c
	ctx.r[3].s64 = ctx.r[10].s64 + 17260;
	// 8262ABD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262ABD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ABDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ABE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ABEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ABF4: 4BE3C22D  bl 0x82466e20
	ctx.lr = 0x8262ABF8;
	sub_82466E20(ctx, base);
	// 8262ABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AC08 size=112
    let mut pc: u32 = 0x8262AC08;
    'dispatch: loop {
        match pc {
            0x8262AC08 => {
    //   block [0x8262AC08..0x8262AC78)
	// 8262AC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AC14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AC18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AC1C: 392A3980  addi r9, r10, 0x3980
	ctx.r[9].s64 = ctx.r[10].s64 + 14720;
	// 8262AC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AC24: 390B8470  addi r8, r11, -0x7b90
	ctx.r[8].s64 = ctx.r[11].s64 + -31632;
	// 8262AC28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262AC2C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8262AC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AC34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AC40: 386A439C  addi r3, r10, 0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + 17308;
	// 8262AC44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AC48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AC64: 4BE3C1BD  bl 0x82466e20
	ctx.lr = 0x8262AC68;
	sub_82466E20(ctx, base);
	// 8262AC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AC78 size=112
    let mut pc: u32 = 0x8262AC78;
    'dispatch: loop {
        match pc {
            0x8262AC78 => {
    //   block [0x8262AC78..0x8262ACE8)
	// 8262AC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AC84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AC88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AC8C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AC94: 390B8518  addi r8, r11, -0x7ae8
	ctx.r[8].s64 = ctx.r[11].s64 + -31464;
	// 8262AC98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262AC9C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8262ACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ACA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ACA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ACAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ACB0: 386A43CC  addi r3, r10, 0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17356;
	// 8262ACB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262ACB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ACBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ACC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ACC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ACC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ACCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ACD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ACD4: 4BE3C14D  bl 0x82466e20
	ctx.lr = 0x8262ACD8;
	sub_82466E20(ctx, base);
	// 8262ACD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ACE8 size=112
    let mut pc: u32 = 0x8262ACE8;
    'dispatch: loop {
        match pc {
            0x8262ACE8 => {
    //   block [0x8262ACE8..0x8262AD58)
	// 8262ACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ACF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ACF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ACFC: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AD04: 390B8560  addi r8, r11, -0x7aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -31392;
	// 8262AD08: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8262AD0C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8262AD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AD14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AD20: 386A43FC  addi r3, r10, 0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + 17404;
	// 8262AD24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AD44: 4BE3C0DD  bl 0x82466e20
	ctx.lr = 0x8262AD48;
	sub_82466E20(ctx, base);
	// 8262AD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AD58 size=100
    let mut pc: u32 = 0x8262AD58;
    'dispatch: loop {
        match pc {
            0x8262AD58 => {
    //   block [0x8262AD58..0x8262ADBC)
	// 8262AD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AD64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AD6C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262AD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AD78: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8262AD7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AD8C: 386A442C  addi r3, r10, 0x442c
	ctx.r[3].s64 = ctx.r[10].s64 + 17452;
	// 8262AD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AD94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AD98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262AD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ADA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262ADA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ADA8: 4BE3C079  bl 0x82466e20
	ctx.lr = 0x8262ADAC;
	sub_82466E20(ctx, base);
	// 8262ADAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ADB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ADB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ADB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ADC0 size=112
    let mut pc: u32 = 0x8262ADC0;
    'dispatch: loop {
        match pc {
            0x8262ADC0 => {
    //   block [0x8262ADC0..0x8262AE30)
	// 8262ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ADC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ADCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ADD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ADD4: 38AA409C  addi r5, r10, 0x409c
	ctx.r[5].s64 = ctx.r[10].s64 + 16540;
	// 8262ADD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ADDC: 390B8638  addi r8, r11, -0x79c8
	ctx.r[8].s64 = ctx.r[11].s64 + -31176;
	// 8262ADE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262ADE4: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8262ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ADEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ADF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262ADF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ADF8: 386A445C  addi r3, r10, 0x445c
	ctx.r[3].s64 = ctx.r[10].s64 + 17500;
	// 8262ADFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AE1C: 4BE3C005  bl 0x82466e20
	ctx.lr = 0x8262AE20;
	sub_82466E20(ctx, base);
	// 8262AE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AE30 size=112
    let mut pc: u32 = 0x8262AE30;
    'dispatch: loop {
        match pc {
            0x8262AE30 => {
    //   block [0x8262AE30..0x8262AEA0)
	// 8262AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AE3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AE40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AE44: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 8262AE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AE4C: 390B8668  addi r8, r11, -0x7998
	ctx.r[8].s64 = ctx.r[11].s64 + -31128;
	// 8262AE50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262AE54: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8262AE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AE5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AE60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AE68: 386A448C  addi r3, r10, 0x448c
	ctx.r[3].s64 = ctx.r[10].s64 + 17548;
	// 8262AE6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AE8C: 4BE3BF95  bl 0x82466e20
	ctx.lr = 0x8262AE90;
	sub_82466E20(ctx, base);
	// 8262AE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AEA0 size=108
    let mut pc: u32 = 0x8262AEA0;
    'dispatch: loop {
        match pc {
            0x8262AEA0 => {
    //   block [0x8262AEA0..0x8262AF0C)
	// 8262AEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AEAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AEB4: 38EB8680  addi r7, r11, -0x7980
	ctx.r[7].s64 = ctx.r[11].s64 + -31104;
	// 8262AEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262AEBC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8262AEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AED0: 386A44BC  addi r3, r10, 0x44bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17596;
	// 8262AED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262AED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AEF8: 4BE3BF29  bl 0x82466e20
	ctx.lr = 0x8262AEFC;
	sub_82466E20(ctx, base);
	// 8262AEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AF10 size=112
    let mut pc: u32 = 0x8262AF10;
    'dispatch: loop {
        match pc {
            0x8262AF10 => {
    //   block [0x8262AF10..0x8262AF80)
	// 8262AF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AF1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AF20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AF24: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262AF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AF2C: 390B86B0  addi r8, r11, -0x7950
	ctx.r[8].s64 = ctx.r[11].s64 + -31056;
	// 8262AF30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262AF34: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8262AF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AF3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AF48: 386A44EC  addi r3, r10, 0x44ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17644;
	// 8262AF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262AF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262AF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262AF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AF6C: 4BE3BEB5  bl 0x82466e20
	ctx.lr = 0x8262AF70;
	sub_82466E20(ctx, base);
	// 8262AF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AF80 size=112
    let mut pc: u32 = 0x8262AF80;
    'dispatch: loop {
        match pc {
            0x8262AF80 => {
    //   block [0x8262AF80..0x8262AFF0)
	// 8262AF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AF8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262AF90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262AF94: 392A39AC  addi r9, r10, 0x39ac
	ctx.r[9].s64 = ctx.r[10].s64 + 14764;
	// 8262AF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262AF9C: 390B8740  addi r8, r11, -0x78c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30912;
	// 8262AFA0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262AFA4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8262AFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262AFAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262AFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262AFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262AFB8: 386A451C  addi r3, r10, 0x451c
	ctx.r[3].s64 = ctx.r[10].s64 + 17692;
	// 8262AFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262AFC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262AFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262AFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262AFDC: 4BE3BE45  bl 0x82466e20
	ctx.lr = 0x8262AFE0;
	sub_82466E20(ctx, base);
	// 8262AFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262AFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262AFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262AFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262AFF0 size=112
    let mut pc: u32 = 0x8262AFF0;
    'dispatch: loop {
        match pc {
            0x8262AFF0 => {
    //   block [0x8262AFF0..0x8262B060)
	// 8262AFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262AFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262AFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262AFFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B000: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B004: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B00C: 390B8788  addi r8, r11, -0x7878
	ctx.r[8].s64 = ctx.r[11].s64 + -30840;
	// 8262B010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262B014: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8262B018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B01C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B028: 386A454C  addi r3, r10, 0x454c
	ctx.r[3].s64 = ctx.r[10].s64 + 17740;
	// 8262B02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B04C: 4BE3BDD5  bl 0x82466e20
	ctx.lr = 0x8262B050;
	sub_82466E20(ctx, base);
	// 8262B050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B060 size=108
    let mut pc: u32 = 0x8262B060;
    'dispatch: loop {
        match pc {
            0x8262B060 => {
    //   block [0x8262B060..0x8262B0CC)
	// 8262B060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B06C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B074: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 8262B078: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262B07C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8262B080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B090: 386A457C  addi r3, r10, 0x457c
	ctx.r[3].s64 = ctx.r[10].s64 + 17788;
	// 8262B094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B0B8: 4BE3BD69  bl 0x82466e20
	ctx.lr = 0x8262B0BC;
	sub_82466E20(ctx, base);
	// 8262B0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B0D0 size=116
    let mut pc: u32 = 0x8262B0D0;
    'dispatch: loop {
        match pc {
            0x8262B0D0 => {
    //   block [0x8262B0D0..0x8262B144)
	// 8262B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B0DC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B0E0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8262B0E4: 390A8830  addi r8, r10, -0x77d0
	ctx.r[8].s64 = ctx.r[10].s64 + -30672;
	// 8262B0E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B0EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262B0F0: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262B0F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B0F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B104: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8262B108: 396B39C0  addi r11, r11, 0x39c0
	ctx.r[11].s64 = ctx.r[11].s64 + 14784;
	// 8262B10C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B110: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B114: 386A45AC  addi r3, r10, 0x45ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17836;
	// 8262B118: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262B11C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B120: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262B124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B130: 4BE3BCF1  bl 0x82466e20
	ctx.lr = 0x8262B134;
	sub_82466E20(ctx, base);
	// 8262B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B148 size=108
    let mut pc: u32 = 0x8262B148;
    'dispatch: loop {
        match pc {
            0x8262B148 => {
    //   block [0x8262B148..0x8262B1B4)
	// 8262B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B154: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B15C: 38EB8908  addi r7, r11, -0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + -30456;
	// 8262B160: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262B164: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8262B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B178: 386A45DC  addi r3, r10, 0x45dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17884;
	// 8262B17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B1A0: 4BE3BC81  bl 0x82466e20
	ctx.lr = 0x8262B1A4;
	sub_82466E20(ctx, base);
	// 8262B1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B1B8 size=112
    let mut pc: u32 = 0x8262B1B8;
    'dispatch: loop {
        match pc {
            0x8262B1B8 => {
    //   block [0x8262B1B8..0x8262B228)
	// 8262B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B1C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B1C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B1CC: 38AA442C  addi r5, r10, 0x442c
	ctx.r[5].s64 = ctx.r[10].s64 + 17452;
	// 8262B1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B1D4: 390B8950  addi r8, r11, -0x76b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30384;
	// 8262B1D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262B1DC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8262B1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B1E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B1F0: 386A460C  addi r3, r10, 0x460c
	ctx.r[3].s64 = ctx.r[10].s64 + 17932;
	// 8262B1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B214: 4BE3BC0D  bl 0x82466e20
	ctx.lr = 0x8262B218;
	sub_82466E20(ctx, base);
	// 8262B218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B228 size=112
    let mut pc: u32 = 0x8262B228;
    'dispatch: loop {
        match pc {
            0x8262B228 => {
    //   block [0x8262B228..0x8262B298)
	// 8262B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B238: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B23C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B244: 390B89C8  addi r8, r11, -0x7638
	ctx.r[8].s64 = ctx.r[11].s64 + -30264;
	// 8262B248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B24C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8262B250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B260: 386A463C  addi r3, r10, 0x463c
	ctx.r[3].s64 = ctx.r[10].s64 + 17980;
	// 8262B264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B284: 4BE3BB9D  bl 0x82466e20
	ctx.lr = 0x8262B288;
	sub_82466E20(ctx, base);
	// 8262B288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B298 size=108
    let mut pc: u32 = 0x8262B298;
    'dispatch: loop {
        match pc {
            0x8262B298 => {
    //   block [0x8262B298..0x8262B304)
	// 8262B298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B2A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B2AC: 38EB89F8  addi r7, r11, -0x7608
	ctx.r[7].s64 = ctx.r[11].s64 + -30216;
	// 8262B2B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262B2B4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8262B2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B2BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B2C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B2C8: 386A466C  addi r3, r10, 0x466c
	ctx.r[3].s64 = ctx.r[10].s64 + 18028;
	// 8262B2CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B2F0: 4BE3BB31  bl 0x82466e20
	ctx.lr = 0x8262B2F4;
	sub_82466E20(ctx, base);
	// 8262B2F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B308 size=112
    let mut pc: u32 = 0x8262B308;
    'dispatch: loop {
        match pc {
            0x8262B308 => {
    //   block [0x8262B308..0x8262B378)
	// 8262B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B318: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B31C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B324: 390B8A70  addi r8, r11, -0x7590
	ctx.r[8].s64 = ctx.r[11].s64 + -30096;
	// 8262B328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262B32C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8262B330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B340: 386A469C  addi r3, r10, 0x469c
	ctx.r[3].s64 = ctx.r[10].s64 + 18076;
	// 8262B344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B364: 4BE3BABD  bl 0x82466e20
	ctx.lr = 0x8262B368;
	sub_82466E20(ctx, base);
	// 8262B368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262B378 size=24
    let mut pc: u32 = 0x8262B378;
    'dispatch: loop {
        match pc {
            0x8262B378 => {
    //   block [0x8262B378..0x8262B390)
	// 8262B378: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B37C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B380: 394AB650  addi r10, r10, -0x49b0
	ctx.r[10].s64 = ctx.r[10].s64 + -18864;
	// 8262B384: 816B8AB8  lwz r11, -0x7548(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30024 as u32) ) } as u64;
	// 8262B388: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262B38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B390 size=116
    let mut pc: u32 = 0x8262B390;
    'dispatch: loop {
        match pc {
            0x8262B390 => {
    //   block [0x8262B390..0x8262B404)
	// 8262B390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B39C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B3A4: 390BB650  addi r8, r11, -0x49b0
	ctx.r[8].s64 = ctx.r[11].s64 + -18864;
	// 8262B3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B3AC: 392A3A1C  addi r9, r10, 0x3a1c
	ctx.r[9].s64 = ctx.r[10].s64 + 14876;
	// 8262B3B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B3B4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262B3B8: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B3BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B3C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B3D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262B3D8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8262B3DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B3E0: 386B46CC  addi r3, r11, 0x46cc
	ctx.r[3].s64 = ctx.r[11].s64 + 18124;
	// 8262B3E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262B3E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B3F0: 4BE3BA31  bl 0x82466e20
	ctx.lr = 0x8262B3F4;
	sub_82466E20(ctx, base);
	// 8262B3F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B408 size=112
    let mut pc: u32 = 0x8262B408;
    'dispatch: loop {
        match pc {
            0x8262B408 => {
    //   block [0x8262B408..0x8262B478)
	// 8262B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B414: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B418: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B41C: 38AA46CC  addi r5, r10, 0x46cc
	ctx.r[5].s64 = ctx.r[10].s64 + 18124;
	// 8262B420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B424: 390B8ABC  addi r8, r11, -0x7544
	ctx.r[8].s64 = ctx.r[11].s64 + -30020;
	// 8262B428: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B42C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8262B430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B440: 386A46FC  addi r3, r10, 0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18172;
	// 8262B444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B464: 4BE3B9BD  bl 0x82466e20
	ctx.lr = 0x8262B468;
	sub_82466E20(ctx, base);
	// 8262B468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B478 size=112
    let mut pc: u32 = 0x8262B478;
    'dispatch: loop {
        match pc {
            0x8262B478 => {
    //   block [0x8262B478..0x8262B4E8)
	// 8262B478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B488: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B48C: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B494: 390B8AF0  addi r8, r11, -0x7510
	ctx.r[8].s64 = ctx.r[11].s64 + -29968;
	// 8262B498: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262B49C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8262B4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B4A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B4B0: 386A472C  addi r3, r10, 0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + 18220;
	// 8262B4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B4D4: 4BE3B94D  bl 0x82466e20
	ctx.lr = 0x8262B4D8;
	sub_82466E20(ctx, base);
	// 8262B4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B4E8 size=112
    let mut pc: u32 = 0x8262B4E8;
    'dispatch: loop {
        match pc {
            0x8262B4E8 => {
    //   block [0x8262B4E8..0x8262B558)
	// 8262B4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B4F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B4F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B4FC: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B504: 390B8B50  addi r8, r11, -0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + -29872;
	// 8262B508: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262B50C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8262B510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B514: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B520: 386A475C  addi r3, r10, 0x475c
	ctx.r[3].s64 = ctx.r[10].s64 + 18268;
	// 8262B524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B544: 4BE3B8DD  bl 0x82466e20
	ctx.lr = 0x8262B548;
	sub_82466E20(ctx, base);
	// 8262B548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B558 size=112
    let mut pc: u32 = 0x8262B558;
    'dispatch: loop {
        match pc {
            0x8262B558 => {
    //   block [0x8262B558..0x8262B5C8)
	// 8262B558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B568: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B56C: 38AA46FC  addi r5, r10, 0x46fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18172;
	// 8262B570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B574: 390B8B80  addi r8, r11, -0x7480
	ctx.r[8].s64 = ctx.r[11].s64 + -29824;
	// 8262B578: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262B57C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8262B580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B590: 386A478C  addi r3, r10, 0x478c
	ctx.r[3].s64 = ctx.r[10].s64 + 18316;
	// 8262B594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B5B4: 4BE3B86D  bl 0x82466e20
	ctx.lr = 0x8262B5B8;
	sub_82466E20(ctx, base);
	// 8262B5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B5C8 size=108
    let mut pc: u32 = 0x8262B5C8;
    'dispatch: loop {
        match pc {
            0x8262B5C8 => {
    //   block [0x8262B5C8..0x8262B634)
	// 8262B5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B5D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B5DC: 38EB8BC8  addi r7, r11, -0x7438
	ctx.r[7].s64 = ctx.r[11].s64 + -29752;
	// 8262B5E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262B5E4: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8262B5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B5EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B5F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B5F8: 386A47BC  addi r3, r10, 0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + 18364;
	// 8262B5FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B620: 4BE3B801  bl 0x82466e20
	ctx.lr = 0x8262B624;
	sub_82466E20(ctx, base);
	// 8262B624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B638 size=112
    let mut pc: u32 = 0x8262B638;
    'dispatch: loop {
        match pc {
            0x8262B638 => {
    //   block [0x8262B638..0x8262B6A8)
	// 8262B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B648: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B64C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262B650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B654: 390B8BF8  addi r8, r11, -0x7408
	ctx.r[8].s64 = ctx.r[11].s64 + -29704;
	// 8262B658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262B65C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8262B660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B670: 386A47EC  addi r3, r10, 0x47ec
	ctx.r[3].s64 = ctx.r[10].s64 + 18412;
	// 8262B674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B694: 4BE3B78D  bl 0x82466e20
	ctx.lr = 0x8262B698;
	sub_82466E20(ctx, base);
	// 8262B698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B6A8 size=116
    let mut pc: u32 = 0x8262B6A8;
    'dispatch: loop {
        match pc {
            0x8262B6A8 => {
    //   block [0x8262B6A8..0x8262B71C)
	// 8262B6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B6B4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B6B8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8262B6BC: 390A8C10  addi r8, r10, -0x73f0
	ctx.r[8].s64 = ctx.r[10].s64 + -29680;
	// 8262B6C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B6C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262B6C8: 38AA4C6C  addi r5, r10, 0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + 19564;
	// 8262B6CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B6D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262B6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B6DC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8262B6E0: 396B3A30  addi r11, r11, 0x3a30
	ctx.r[11].s64 = ctx.r[11].s64 + 14896;
	// 8262B6E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B6E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B6EC: 386A481C  addi r3, r10, 0x481c
	ctx.r[3].s64 = ctx.r[10].s64 + 18460;
	// 8262B6F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262B6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B6F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262B6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B708: 4BE3B719  bl 0x82466e20
	ctx.lr = 0x8262B70C;
	sub_82466E20(ctx, base);
	// 8262B70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B720 size=100
    let mut pc: u32 = 0x8262B720;
    'dispatch: loop {
        match pc {
            0x8262B720 => {
    //   block [0x8262B720..0x8262B784)
	// 8262B720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B734: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B740: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8262B744: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B754: 386A484C  addi r3, r10, 0x484c
	ctx.r[3].s64 = ctx.r[10].s64 + 18508;
	// 8262B758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B770: 4BE3B6B1  bl 0x82466e20
	ctx.lr = 0x8262B774;
	sub_82466E20(ctx, base);
	// 8262B774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B788 size=100
    let mut pc: u32 = 0x8262B788;
    'dispatch: loop {
        match pc {
            0x8262B788 => {
    //   block [0x8262B788..0x8262B7EC)
	// 8262B788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B79C: 38AA48DC  addi r5, r10, 0x48dc
	ctx.r[5].s64 = ctx.r[10].s64 + 18652;
	// 8262B7A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B7A8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8262B7AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B7BC: 386A487C  addi r3, r10, 0x487c
	ctx.r[3].s64 = ctx.r[10].s64 + 18556;
	// 8262B7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B7C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B7D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B7D8: 4BE3B649  bl 0x82466e20
	ctx.lr = 0x8262B7DC;
	sub_82466E20(ctx, base);
	// 8262B7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B7F0 size=100
    let mut pc: u32 = 0x8262B7F0;
    'dispatch: loop {
        match pc {
            0x8262B7F0 => {
    //   block [0x8262B7F0..0x8262B854)
	// 8262B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B7FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B804: 38AA481C  addi r5, r10, 0x481c
	ctx.r[5].s64 = ctx.r[10].s64 + 18460;
	// 8262B808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B810: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8262B814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B824: 386A48AC  addi r3, r10, 0x48ac
	ctx.r[3].s64 = ctx.r[10].s64 + 18604;
	// 8262B828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B82C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B830: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B838: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B840: 4BE3B5E1  bl 0x82466e20
	ctx.lr = 0x8262B844;
	sub_82466E20(ctx, base);
	// 8262B844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B858 size=104
    let mut pc: u32 = 0x8262B858;
    'dispatch: loop {
        match pc {
            0x8262B858 => {
    //   block [0x8262B858..0x8262B8C0)
	// 8262B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B864: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B86C: 392A3A94  addi r9, r10, 0x3a94
	ctx.r[9].s64 = ctx.r[10].s64 + 14996;
	// 8262B870: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B878: 38AA484C  addi r5, r10, 0x484c
	ctx.r[5].s64 = ctx.r[10].s64 + 18508;
	// 8262B87C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B88C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8262B890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262B89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B8A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262B8A4: 386A48DC  addi r3, r10, 0x48dc
	ctx.r[3].s64 = ctx.r[10].s64 + 18652;
	// 8262B8A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262B8AC: 4BE3B575  bl 0x82466e20
	ctx.lr = 0x8262B8B0;
	sub_82466E20(ctx, base);
	// 8262B8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B8C0 size=108
    let mut pc: u32 = 0x8262B8C0;
    'dispatch: loop {
        match pc {
            0x8262B8C0 => {
    //   block [0x8262B8C0..0x8262B92C)
	// 8262B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B8CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B8D4: 38EB8D90  addi r7, r11, -0x7270
	ctx.r[7].s64 = ctx.r[11].s64 + -29296;
	// 8262B8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262B8DC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8262B8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B8E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262B8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B8F0: 386A490C  addi r3, r10, 0x490c
	ctx.r[3].s64 = ctx.r[10].s64 + 18700;
	// 8262B8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262B8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262B918: 4BE3B509  bl 0x82466e20
	ctx.lr = 0x8262B91C;
	sub_82466E20(ctx, base);
	// 8262B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B930 size=112
    let mut pc: u32 = 0x8262B930;
    'dispatch: loop {
        match pc {
            0x8262B930 => {
    //   block [0x8262B930..0x8262B9A0)
	// 8262B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B93C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B940: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B944: 38AA48DC  addi r5, r10, 0x48dc
	ctx.r[5].s64 = ctx.r[10].s64 + 18652;
	// 8262B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B94C: 390B8DC0  addi r8, r11, -0x7240
	ctx.r[8].s64 = ctx.r[11].s64 + -29248;
	// 8262B950: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262B954: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8262B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B95C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262B968: 386A493C  addi r3, r10, 0x493c
	ctx.r[3].s64 = ctx.r[10].s64 + 18748;
	// 8262B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B98C: 4BE3B495  bl 0x82466e20
	ctx.lr = 0x8262B990;
	sub_82466E20(ctx, base);
	// 8262B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262B9A0 size=24
    let mut pc: u32 = 0x8262B9A0;
    'dispatch: loop {
        match pc {
            0x8262B9A0 => {
    //   block [0x8262B9A0..0x8262B9B8)
	// 8262B9A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B9A4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262B9A8: 394AB668  addi r10, r10, -0x4998
	ctx.r[10].s64 = ctx.r[10].s64 + -18840;
	// 8262B9AC: 816B8E68  lwz r11, -0x7198(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29080 as u32) ) } as u64;
	// 8262B9B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262B9B8 size=116
    let mut pc: u32 = 0x8262B9B8;
    'dispatch: loop {
        match pc {
            0x8262B9B8 => {
    //   block [0x8262B9B8..0x8262BA2C)
	// 8262B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262B9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262B9C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262B9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262B9CC: 390BB668  addi r8, r11, -0x4998
	ctx.r[8].s64 = ctx.r[11].s64 + -18840;
	// 8262B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262B9D4: 392A3AF8  addi r9, r10, 0x3af8
	ctx.r[9].s64 = ctx.r[10].s64 + 15096;
	// 8262B9D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262B9DC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262B9E0: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262B9E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262B9EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262B9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262B9FC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262BA00: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8262BA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262BA08: 386B496C  addi r3, r11, 0x496c
	ctx.r[3].s64 = ctx.r[11].s64 + 18796;
	// 8262BA0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262BA10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BA18: 4BE3B409  bl 0x82466e20
	ctx.lr = 0x8262BA1C;
	sub_82466E20(ctx, base);
	// 8262BA1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BA30 size=100
    let mut pc: u32 = 0x8262BA30;
    'dispatch: loop {
        match pc {
            0x8262BA30 => {
    //   block [0x8262BA30..0x8262BA94)
	// 8262BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BA3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BA44: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BA50: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8262BA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BA64: 386A499C  addi r3, r10, 0x499c
	ctx.r[3].s64 = ctx.r[10].s64 + 18844;
	// 8262BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BA6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BA70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BA78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BA80: 4BE3B3A1  bl 0x82466e20
	ctx.lr = 0x8262BA84;
	sub_82466E20(ctx, base);
	// 8262BA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BA98 size=100
    let mut pc: u32 = 0x8262BA98;
    'dispatch: loop {
        match pc {
            0x8262BA98 => {
    //   block [0x8262BA98..0x8262BAFC)
	// 8262BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BAA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BAAC: 38AA49FC  addi r5, r10, 0x49fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18940;
	// 8262BAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BAB8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8262BABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BACC: 386A49CC  addi r3, r10, 0x49cc
	ctx.r[3].s64 = ctx.r[10].s64 + 18892;
	// 8262BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BAE8: 4BE3B339  bl 0x82466e20
	ctx.lr = 0x8262BAEC;
	sub_82466E20(ctx, base);
	// 8262BAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BB00 size=112
    let mut pc: u32 = 0x8262BB00;
    'dispatch: loop {
        match pc {
            0x8262BB00 => {
    //   block [0x8262BB00..0x8262BB70)
	// 8262BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BB14: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BB1C: 390B8E6C  addi r8, r11, -0x7194
	ctx.r[8].s64 = ctx.r[11].s64 + -29076;
	// 8262BB20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BB24: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8262BB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BB2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BB38: 386A49FC  addi r3, r10, 0x49fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18940;
	// 8262BB3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BB5C: 4BE3B2C5  bl 0x82466e20
	ctx.lr = 0x8262BB60;
	sub_82466E20(ctx, base);
	// 8262BB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BB70 size=100
    let mut pc: u32 = 0x8262BB70;
    'dispatch: loop {
        match pc {
            0x8262BB70 => {
    //   block [0x8262BB70..0x8262BBD4)
	// 8262BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BB7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BB84: 38AA49FC  addi r5, r10, 0x49fc
	ctx.r[5].s64 = ctx.r[10].s64 + 18940;
	// 8262BB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BB90: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8262BB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BBA4: 386A4A2C  addi r3, r10, 0x4a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 18988;
	// 8262BBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BBB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BBB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BBC0: 4BE3B261  bl 0x82466e20
	ctx.lr = 0x8262BBC4;
	sub_82466E20(ctx, base);
	// 8262BBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BBD8 size=100
    let mut pc: u32 = 0x8262BBD8;
    'dispatch: loop {
        match pc {
            0x8262BBD8 => {
    //   block [0x8262BBD8..0x8262BC3C)
	// 8262BBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BBE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BBEC: 38AA496C  addi r5, r10, 0x496c
	ctx.r[5].s64 = ctx.r[10].s64 + 18796;
	// 8262BBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BBF8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8262BBFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BC0C: 386A4A5C  addi r3, r10, 0x4a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 19036;
	// 8262BC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BC14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BC18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BC20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BC28: 4BE3B1F9  bl 0x82466e20
	ctx.lr = 0x8262BC2C;
	sub_82466E20(ctx, base);
	// 8262BC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BC40 size=100
    let mut pc: u32 = 0x8262BC40;
    'dispatch: loop {
        match pc {
            0x8262BC40 => {
    //   block [0x8262BC40..0x8262BCA4)
	// 8262BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BC4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BC54: 38AA499C  addi r5, r10, 0x499c
	ctx.r[5].s64 = ctx.r[10].s64 + 18844;
	// 8262BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BC60: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8262BC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BC74: 386A4A8C  addi r3, r10, 0x4a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19084;
	// 8262BC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BC7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BC80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BC88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BC90: 4BE3B191  bl 0x82466e20
	ctx.lr = 0x8262BC94;
	sub_82466E20(ctx, base);
	// 8262BC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BCA8 size=100
    let mut pc: u32 = 0x8262BCA8;
    'dispatch: loop {
        match pc {
            0x8262BCA8 => {
    //   block [0x8262BCA8..0x8262BD0C)
	// 8262BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BCB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BCBC: 38AA4A5C  addi r5, r10, 0x4a5c
	ctx.r[5].s64 = ctx.r[10].s64 + 19036;
	// 8262BCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BCC8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8262BCCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BCDC: 386A4ABC  addi r3, r10, 0x4abc
	ctx.r[3].s64 = ctx.r[10].s64 + 19132;
	// 8262BCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BCE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BCE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BCF8: 4BE3B129  bl 0x82466e20
	ctx.lr = 0x8262BCFC;
	sub_82466E20(ctx, base);
	// 8262BCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BD10 size=100
    let mut pc: u32 = 0x8262BD10;
    'dispatch: loop {
        match pc {
            0x8262BD10 => {
    //   block [0x8262BD10..0x8262BD74)
	// 8262BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BD1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BD24: 38AA499C  addi r5, r10, 0x499c
	ctx.r[5].s64 = ctx.r[10].s64 + 18844;
	// 8262BD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BD30: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8262BD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BD44: 386A4AEC  addi r3, r10, 0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + 19180;
	// 8262BD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BD50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BD58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262BD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BD60: 4BE3B0C1  bl 0x82466e20
	ctx.lr = 0x8262BD64;
	sub_82466E20(ctx, base);
	// 8262BD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BD78 size=112
    let mut pc: u32 = 0x8262BD78;
    'dispatch: loop {
        match pc {
            0x8262BD78 => {
    //   block [0x8262BD78..0x8262BDE8)
	// 8262BD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BD88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BD8C: 38AA4B7C  addi r5, r10, 0x4b7c
	ctx.r[5].s64 = ctx.r[10].s64 + 19324;
	// 8262BD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BD94: 390B8E9C  addi r8, r11, -0x7164
	ctx.r[8].s64 = ctx.r[11].s64 + -29028;
	// 8262BD98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BD9C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8262BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BDA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BDB0: 386A4B1C  addi r3, r10, 0x4b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19228;
	// 8262BDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BDD4: 4BE3B04D  bl 0x82466e20
	ctx.lr = 0x8262BDD8;
	sub_82466E20(ctx, base);
	// 8262BDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BDE8 size=112
    let mut pc: u32 = 0x8262BDE8;
    'dispatch: loop {
        match pc {
            0x8262BDE8 => {
    //   block [0x8262BDE8..0x8262BE58)
	// 8262BDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BDF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BDF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BDFC: 38AA4BAC  addi r5, r10, 0x4bac
	ctx.r[5].s64 = ctx.r[10].s64 + 19372;
	// 8262BE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BE04: 390B8ECC  addi r8, r11, -0x7134
	ctx.r[8].s64 = ctx.r[11].s64 + -28980;
	// 8262BE08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BE0C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8262BE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BE14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BE20: 386A4B4C  addi r3, r10, 0x4b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 19276;
	// 8262BE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BE44: 4BE3AFDD  bl 0x82466e20
	ctx.lr = 0x8262BE48;
	sub_82466E20(ctx, base);
	// 8262BE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BE58 size=112
    let mut pc: u32 = 0x8262BE58;
    'dispatch: loop {
        match pc {
            0x8262BE58 => {
    //   block [0x8262BE58..0x8262BEC8)
	// 8262BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BE64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BE6C: 38AA4C6C  addi r5, r10, 0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + 19564;
	// 8262BE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BE74: 390B8EE4  addi r8, r11, -0x711c
	ctx.r[8].s64 = ctx.r[11].s64 + -28956;
	// 8262BE78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262BE7C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8262BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BE90: 386A4B7C  addi r3, r10, 0x4b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 19324;
	// 8262BE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BEB4: 4BE3AF6D  bl 0x82466e20
	ctx.lr = 0x8262BEB8;
	sub_82466E20(ctx, base);
	// 8262BEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BEC8 size=112
    let mut pc: u32 = 0x8262BEC8;
    'dispatch: loop {
        match pc {
            0x8262BEC8 => {
    //   block [0x8262BEC8..0x8262BF38)
	// 8262BEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BED8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BEDC: 38AA4B7C  addi r5, r10, 0x4b7c
	ctx.r[5].s64 = ctx.r[10].s64 + 19324;
	// 8262BEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BEE4: 390B8F14  addi r8, r11, -0x70ec
	ctx.r[8].s64 = ctx.r[11].s64 + -28908;
	// 8262BEE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BEEC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8262BEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BF00: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 8262BF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BF24: 4BE3AEFD  bl 0x82466e20
	ctx.lr = 0x8262BF28;
	sub_82466E20(ctx, base);
	// 8262BF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BF38 size=112
    let mut pc: u32 = 0x8262BF38;
    'dispatch: loop {
        match pc {
            0x8262BF38 => {
    //   block [0x8262BF38..0x8262BFA8)
	// 8262BF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BF48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BF4C: 38AA4BAC  addi r5, r10, 0x4bac
	ctx.r[5].s64 = ctx.r[10].s64 + 19372;
	// 8262BF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BF54: 390B8F2C  addi r8, r11, -0x70d4
	ctx.r[8].s64 = ctx.r[11].s64 + -28884;
	// 8262BF58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BF5C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8262BF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BF64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BF68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BF70: 386A4BDC  addi r3, r10, 0x4bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 19420;
	// 8262BF74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BF94: 4BE3AE8D  bl 0x82466e20
	ctx.lr = 0x8262BF98;
	sub_82466E20(ctx, base);
	// 8262BF98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BFA8 size=116
    let mut pc: u32 = 0x8262BFA8;
    'dispatch: loop {
        match pc {
            0x8262BFA8 => {
    //   block [0x8262BFA8..0x8262C01C)
	// 8262BFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BFB4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262BFB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262BFBC: 390A8F48  addi r8, r10, -0x70b8
	ctx.r[8].s64 = ctx.r[10].s64 + -28856;
	// 8262BFC0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BFC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262BFC8: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262BFCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BFD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262BFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BFDC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8262BFE0: 396B3B0C  addi r11, r11, 0x3b0c
	ctx.r[11].s64 = ctx.r[11].s64 + 15116;
	// 8262BFE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BFE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BFEC: 386A4C0C  addi r3, r10, 0x4c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 19468;
	// 8262BFF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262BFF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BFF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262BFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C008: 4BE3AE19  bl 0x82466e20
	ctx.lr = 0x8262C00C;
	sub_82466E20(ctx, base);
	// 8262C00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262C020 size=48
    let mut pc: u32 = 0x8262C020;
    'dispatch: loop {
        match pc {
            0x8262C020 => {
    //   block [0x8262C020..0x8262C050)
	// 8262C020: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C024: 814B8FF8  lwz r10, -0x7008(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28680 as u32) ) } as u64;
	// 8262C028: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C02C: 396BB728  addi r11, r11, -0x48d8
	ctx.r[11].s64 = ctx.r[11].s64 + -18648;
	// 8262C030: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8262C034: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C038: 814A8FF4  lwz r10, -0x700c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28684 as u32) ) } as u64;
	// 8262C03C: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8262C040: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C044: 814A8FF0  lwz r10, -0x7010(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28688 as u32) ) } as u64;
	// 8262C048: 914B0278  stw r10, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 8262C04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C050 size=116
    let mut pc: u32 = 0x8262C050;
    'dispatch: loop {
        match pc {
            0x8262C050 => {
    //   block [0x8262C050..0x8262C0C4)
	// 8262C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C05C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262C060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C064: 392B3BE0  addi r9, r11, 0x3be0
	ctx.r[9].s64 = ctx.r[11].s64 + 15328;
	// 8262C068: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C06C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C070: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8262C074: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 8262C078: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C07C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8262C080: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C084: 396BB728  addi r11, r11, -0x48d8
	ctx.r[11].s64 = ctx.r[11].s64 + -18648;
	// 8262C088: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262C08C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C090: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262C094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C098: 386A4C3C  addi r3, r10, 0x4c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 19516;
	// 8262C09C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8262C0A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262C0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C0A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262C0AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262C0B0: 4BE3AD71  bl 0x82466e20
	ctx.lr = 0x8262C0B4;
	sub_82466E20(ctx, base);
	// 8262C0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C0C8 size=116
    let mut pc: u32 = 0x8262C0C8;
    'dispatch: loop {
        match pc {
            0x8262C0C8 => {
    //   block [0x8262C0C8..0x8262C13C)
	// 8262C0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C0D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C0D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C0DC: 390B9000  addi r8, r11, -0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + -28672;
	// 8262C0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C0E4: 392A3D2C  addi r9, r10, 0x3d2c
	ctx.r[9].s64 = ctx.r[10].s64 + 15660;
	// 8262C0E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C0EC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262C0F0: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C0F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C0FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C10C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262C110: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8262C114: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C118: 386B4C6C  addi r3, r11, 0x4c6c
	ctx.r[3].s64 = ctx.r[11].s64 + 19564;
	// 8262C11C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C120: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C128: 4BE3ACF9  bl 0x82466e20
	ctx.lr = 0x8262C12C;
	sub_82466E20(ctx, base);
	// 8262C12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C140 size=112
    let mut pc: u32 = 0x8262C140;
    'dispatch: loop {
        match pc {
            0x8262C140 => {
    //   block [0x8262C140..0x8262C1B0)
	// 8262C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C14C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C150: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C154: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C15C: 390B9090  addi r8, r11, -0x6f70
	ctx.r[8].s64 = ctx.r[11].s64 + -28528;
	// 8262C160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C164: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8262C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C178: 386A4C9C  addi r3, r10, 0x4c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 19612;
	// 8262C17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C19C: 4BE3AC85  bl 0x82466e20
	ctx.lr = 0x8262C1A0;
	sub_82466E20(ctx, base);
	// 8262C1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C1B0 size=112
    let mut pc: u32 = 0x8262C1B0;
    'dispatch: loop {
        match pc {
            0x8262C1B0 => {
    //   block [0x8262C1B0..0x8262C220)
	// 8262C1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C1BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C1C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C1C4: 38AA2ECC  addi r5, r10, 0x2ecc
	ctx.r[5].s64 = ctx.r[10].s64 + 11980;
	// 8262C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C1CC: 390B90A8  addi r8, r11, -0x6f58
	ctx.r[8].s64 = ctx.r[11].s64 + -28504;
	// 8262C1D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C1D4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8262C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C1DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C1E8: 386A4CCC  addi r3, r10, 0x4ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 19660;
	// 8262C1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C20C: 4BE3AC15  bl 0x82466e20
	ctx.lr = 0x8262C210;
	sub_82466E20(ctx, base);
	// 8262C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C220 size=108
    let mut pc: u32 = 0x8262C220;
    'dispatch: loop {
        match pc {
            0x8262C220 => {
    //   block [0x8262C220..0x8262C28C)
	// 8262C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C22C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C234: 38EB90C0  addi r7, r11, -0x6f40
	ctx.r[7].s64 = ctx.r[11].s64 + -28480;
	// 8262C238: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C23C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8262C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C250: 386A4CFC  addi r3, r10, 0x4cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 19708;
	// 8262C254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C278: 4BE3ABA9  bl 0x82466e20
	ctx.lr = 0x8262C27C;
	sub_82466E20(ctx, base);
	// 8262C27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C290 size=112
    let mut pc: u32 = 0x8262C290;
    'dispatch: loop {
        match pc {
            0x8262C290 => {
    //   block [0x8262C290..0x8262C300)
	// 8262C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C29C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C2A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C2A4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C2AC: 390B90D8  addi r8, r11, -0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + -28456;
	// 8262C2B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262C2B4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8262C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C2BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C2C8: 386A4D2C  addi r3, r10, 0x4d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 19756;
	// 8262C2CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C2EC: 4BE3AB35  bl 0x82466e20
	ctx.lr = 0x8262C2F0;
	sub_82466E20(ctx, base);
	// 8262C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C300 size=108
    let mut pc: u32 = 0x8262C300;
    'dispatch: loop {
        match pc {
            0x8262C300 => {
    //   block [0x8262C300..0x8262C36C)
	// 8262C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C30C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C314: 38EB9120  addi r7, r11, -0x6ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -28384;
	// 8262C318: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C31C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8262C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C330: 386A4D5C  addi r3, r10, 0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 19804;
	// 8262C334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C358: 4BE3AAC9  bl 0x82466e20
	ctx.lr = 0x8262C35C;
	sub_82466E20(ctx, base);
	// 8262C35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C370 size=112
    let mut pc: u32 = 0x8262C370;
    'dispatch: loop {
        match pc {
            0x8262C370 => {
    //   block [0x8262C370..0x8262C3E0)
	// 8262C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C37C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C380: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C384: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C38C: 390B9138  addi r8, r11, -0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -28360;
	// 8262C390: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262C394: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8262C398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C39C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C3A8: 386A4D8C  addi r3, r10, 0x4d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19852;
	// 8262C3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C3CC: 4BE3AA55  bl 0x82466e20
	ctx.lr = 0x8262C3D0;
	sub_82466E20(ctx, base);
	// 8262C3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C3E0 size=112
    let mut pc: u32 = 0x8262C3E0;
    'dispatch: loop {
        match pc {
            0x8262C3E0 => {
    //   block [0x8262C3E0..0x8262C450)
	// 8262C3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C3EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C3F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C3F4: 392A3D84  addi r9, r10, 0x3d84
	ctx.r[9].s64 = ctx.r[10].s64 + 15748;
	// 8262C3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C3FC: 390B9170  addi r8, r11, -0x6e90
	ctx.r[8].s64 = ctx.r[11].s64 + -28304;
	// 8262C400: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262C404: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8262C408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C418: 386A4DBC  addi r3, r10, 0x4dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 19900;
	// 8262C41C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C43C: 4BE3A9E5  bl 0x82466e20
	ctx.lr = 0x8262C440;
	sub_82466E20(ctx, base);
	// 8262C440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C450 size=116
    let mut pc: u32 = 0x8262C450;
    'dispatch: loop {
        match pc {
            0x8262C450 => {
    //   block [0x8262C450..0x8262C4C4)
	// 8262C450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C45C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C464: 390B9218  addi r8, r11, -0x6de8
	ctx.r[8].s64 = ctx.r[11].s64 + -28136;
	// 8262C468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C46C: 392A3D58  addi r9, r10, 0x3d58
	ctx.r[9].s64 = ctx.r[10].s64 + 15704;
	// 8262C470: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C474: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262C478: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262C47C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C494: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262C498: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8262C49C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C4A0: 386B4DEC  addi r3, r11, 0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + 19948;
	// 8262C4A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C4A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C4B0: 4BE3A971  bl 0x82466e20
	ctx.lr = 0x8262C4B4;
	sub_82466E20(ctx, base);
	// 8262C4B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C4C8 size=112
    let mut pc: u32 = 0x8262C4C8;
    'dispatch: loop {
        match pc {
            0x8262C4C8 => {
    //   block [0x8262C4C8..0x8262C538)
	// 8262C4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C4D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C4D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C4DC: 392A3DB0  addi r9, r10, 0x3db0
	ctx.r[9].s64 = ctx.r[10].s64 + 15792;
	// 8262C4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C4E4: 390B9238  addi r8, r11, -0x6dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -28104;
	// 8262C4E8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262C4EC: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8262C4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C4F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C4FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C500: 386A4E1C  addi r3, r10, 0x4e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19996;
	// 8262C504: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C508: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C51C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C524: 4BE3A8FD  bl 0x82466e20
	ctx.lr = 0x8262C528;
	sub_82466E20(ctx, base);
	// 8262C528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C538 size=112
    let mut pc: u32 = 0x8262C538;
    'dispatch: loop {
        match pc {
            0x8262C538 => {
    //   block [0x8262C538..0x8262C5A8)
	// 8262C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C548: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C54C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262C550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C554: 390B9298  addi r8, r11, -0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + -28008;
	// 8262C558: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C55C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8262C560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C570: 386A4E4C  addi r3, r10, 0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 20044;
	// 8262C574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C594: 4BE3A88D  bl 0x82466e20
	ctx.lr = 0x8262C598;
	sub_82466E20(ctx, base);
	// 8262C598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C5A8 size=112
    let mut pc: u32 = 0x8262C5A8;
    'dispatch: loop {
        match pc {
            0x8262C5A8 => {
    //   block [0x8262C5A8..0x8262C618)
	// 8262C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C5B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C5BC: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C5C4: 390B92B0  addi r8, r11, -0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + -27984;
	// 8262C5C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262C5CC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8262C5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C5D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C5E0: 386A4E7C  addi r3, r10, 0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20092;
	// 8262C5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C604: 4BE3A81D  bl 0x82466e20
	ctx.lr = 0x8262C608;
	sub_82466E20(ctx, base);
	// 8262C608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C618 size=112
    let mut pc: u32 = 0x8262C618;
    'dispatch: loop {
        match pc {
            0x8262C618 => {
    //   block [0x8262C618..0x8262C688)
	// 8262C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C628: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C62C: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C634: 390B92F8  addi r8, r11, -0x6d08
	ctx.r[8].s64 = ctx.r[11].s64 + -27912;
	// 8262C638: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C63C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8262C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C650: 386A4EAC  addi r3, r10, 0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + 20140;
	// 8262C654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C674: 4BE3A7AD  bl 0x82466e20
	ctx.lr = 0x8262C678;
	sub_82466E20(ctx, base);
	// 8262C678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C688 size=112
    let mut pc: u32 = 0x8262C688;
    'dispatch: loop {
        match pc {
            0x8262C688 => {
    //   block [0x8262C688..0x8262C6F8)
	// 8262C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C69C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262C6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C6A4: 390B9358  addi r8, r11, -0x6ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -27816;
	// 8262C6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C6AC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8262C6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C6B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C6C0: 386A4EDC  addi r3, r10, 0x4edc
	ctx.r[3].s64 = ctx.r[10].s64 + 20188;
	// 8262C6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C6E4: 4BE3A73D  bl 0x82466e20
	ctx.lr = 0x8262C6E8;
	sub_82466E20(ctx, base);
	// 8262C6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C6F8 size=112
    let mut pc: u32 = 0x8262C6F8;
    'dispatch: loop {
        match pc {
            0x8262C6F8 => {
    //   block [0x8262C6F8..0x8262C768)
	// 8262C6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C708: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C70C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262C710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C714: 390B93B8  addi r8, r11, -0x6c48
	ctx.r[8].s64 = ctx.r[11].s64 + -27720;
	// 8262C718: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C71C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8262C720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C730: 386A4F0C  addi r3, r10, 0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 20236;
	// 8262C734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C754: 4BE3A6CD  bl 0x82466e20
	ctx.lr = 0x8262C758;
	sub_82466E20(ctx, base);
	// 8262C758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C768 size=112
    let mut pc: u32 = 0x8262C768;
    'dispatch: loop {
        match pc {
            0x8262C768 => {
    //   block [0x8262C768..0x8262C7D8)
	// 8262C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C778: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C77C: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C784: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 8262C788: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8262C78C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8262C790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C7A0: 386A4F3C  addi r3, r10, 0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 20284;
	// 8262C7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C7C4: 4BE3A65D  bl 0x82466e20
	ctx.lr = 0x8262C7C8;
	sub_82466E20(ctx, base);
	// 8262C7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C7D8 size=112
    let mut pc: u32 = 0x8262C7D8;
    'dispatch: loop {
        match pc {
            0x8262C7D8 => {
    //   block [0x8262C7D8..0x8262C848)
	// 8262C7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C7E4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C7E8: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8262C7EC: 38EA94D8  addi r7, r10, -0x6b28
	ctx.r[7].s64 = ctx.r[10].s64 + -27432;
	// 8262C7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C7F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262C7F8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8262C7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C800: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C804: 396B3DC8  addi r11, r11, 0x3dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 15816;
	// 8262C808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C814: 386A4F6C  addi r3, r10, 0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 20332;
	// 8262C818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C81C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262C820: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C824: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262C828: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C82C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C834: 4BE3A5ED  bl 0x82466e20
	ctx.lr = 0x8262C838;
	sub_82466E20(ctx, base);
	// 8262C838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C848 size=112
    let mut pc: u32 = 0x8262C848;
    'dispatch: loop {
        match pc {
            0x8262C848 => {
    //   block [0x8262C848..0x8262C8B8)
	// 8262C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C858: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C85C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C864: 390B96A0  addi r8, r11, -0x6960
	ctx.r[8].s64 = ctx.r[11].s64 + -26976;
	// 8262C868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C86C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8262C870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C880: 386A4F9C  addi r3, r10, 0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 20380;
	// 8262C884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C894: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C8A4: 4BE3A57D  bl 0x82466e20
	ctx.lr = 0x8262C8A8;
	sub_82466E20(ctx, base);
	// 8262C8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C8B8 size=112
    let mut pc: u32 = 0x8262C8B8;
    'dispatch: loop {
        match pc {
            0x8262C8B8 => {
    //   block [0x8262C8B8..0x8262C928)
	// 8262C8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C8C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C8C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C8CC: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C8D4: 390B96B8  addi r8, r11, -0x6948
	ctx.r[8].s64 = ctx.r[11].s64 + -26952;
	// 8262C8D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C8DC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8262C8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C8E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C8F0: 386A4FCC  addi r3, r10, 0x4fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 20428;
	// 8262C8F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C904: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C914: 4BE3A50D  bl 0x82466e20
	ctx.lr = 0x8262C918;
	sub_82466E20(ctx, base);
	// 8262C918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C928 size=112
    let mut pc: u32 = 0x8262C928;
    'dispatch: loop {
        match pc {
            0x8262C928 => {
    //   block [0x8262C928..0x8262C998)
	// 8262C928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C938: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C93C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C944: 390B96D0  addi r8, r11, -0x6930
	ctx.r[8].s64 = ctx.r[11].s64 + -26928;
	// 8262C948: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262C94C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8262C950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C960: 386A4FFC  addi r3, r10, 0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 20476;
	// 8262C964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C984: 4BE3A49D  bl 0x82466e20
	ctx.lr = 0x8262C988;
	sub_82466E20(ctx, base);
	// 8262C988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C998 size=108
    let mut pc: u32 = 0x8262C998;
    'dispatch: loop {
        match pc {
            0x8262C998 => {
    //   block [0x8262C998..0x8262CA04)
	// 8262C998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C9AC: 38EB9700  addi r7, r11, -0x6900
	ctx.r[7].s64 = ctx.r[11].s64 + -26880;
	// 8262C9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262C9B4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8262C9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C9C8: 386A502C  addi r3, r10, 0x502c
	ctx.r[3].s64 = ctx.r[10].s64 + 20524;
	// 8262C9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C9F0: 4BE3A431  bl 0x82466e20
	ctx.lr = 0x8262C9F4;
	sub_82466E20(ctx, base);
	// 8262C9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CA08 size=112
    let mut pc: u32 = 0x8262CA08;
    'dispatch: loop {
        match pc {
            0x8262CA08 => {
    //   block [0x8262CA08..0x8262CA78)
	// 8262CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CA14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CA18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CA1C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262CA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CA24: 390B9730  addi r8, r11, -0x68d0
	ctx.r[8].s64 = ctx.r[11].s64 + -26832;
	// 8262CA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CA2C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8262CA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CA34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CA40: 386A505C  addi r3, r10, 0x505c
	ctx.r[3].s64 = ctx.r[10].s64 + 20572;
	// 8262CA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CA54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262CA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CA64: 4BE3A3BD  bl 0x82466e20
	ctx.lr = 0x8262CA68;
	sub_82466E20(ctx, base);
	// 8262CA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CA78 size=108
    let mut pc: u32 = 0x8262CA78;
    'dispatch: loop {
        match pc {
            0x8262CA78 => {
    //   block [0x8262CA78..0x8262CAE4)
	// 8262CA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CA84: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CA8C: 38EB9748  addi r7, r11, -0x68b8
	ctx.r[7].s64 = ctx.r[11].s64 + -26808;
	// 8262CA90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CA94: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8262CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CA9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CAA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CAA8: 386A508C  addi r3, r10, 0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + 20620;
	// 8262CAAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CAD0: 4BE3A351  bl 0x82466e20
	ctx.lr = 0x8262CAD4;
	sub_82466E20(ctx, base);
	// 8262CAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CAE8 size=108
    let mut pc: u32 = 0x8262CAE8;
    'dispatch: loop {
        match pc {
            0x8262CAE8 => {
    //   block [0x8262CAE8..0x8262CB54)
	// 8262CAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CAF4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CAFC: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 8262CB00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262CB04: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8262CB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CB18: 386A50BC  addi r3, r10, 0x50bc
	ctx.r[3].s64 = ctx.r[10].s64 + 20668;
	// 8262CB1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CB40: 4BE3A2E1  bl 0x82466e20
	ctx.lr = 0x8262CB44;
	sub_82466E20(ctx, base);
	// 8262CB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CB58 size=112
    let mut pc: u32 = 0x8262CB58;
    'dispatch: loop {
        match pc {
            0x8262CB58 => {
    //   block [0x8262CB58..0x8262CBC8)
	// 8262CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CB64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CB6C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CB70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CB74: 390B97C0  addi r8, r11, -0x6840
	ctx.r[8].s64 = ctx.r[11].s64 + -26688;
	// 8262CB78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262CB7C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8262CB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CB84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CB90: 386A50EC  addi r3, r10, 0x50ec
	ctx.r[3].s64 = ctx.r[10].s64 + 20716;
	// 8262CB94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CB98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CBA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CBA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CBB4: 4BE3A26D  bl 0x82466e20
	ctx.lr = 0x8262CBB8;
	sub_82466E20(ctx, base);
	// 8262CBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CBC8 size=112
    let mut pc: u32 = 0x8262CBC8;
    'dispatch: loop {
        match pc {
            0x8262CBC8 => {
    //   block [0x8262CBC8..0x8262CC38)
	// 8262CBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CBD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CBD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CBDC: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262CBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CBE4: 390B9808  addi r8, r11, -0x67f8
	ctx.r[8].s64 = ctx.r[11].s64 + -26616;
	// 8262CBE8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262CBEC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8262CBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CBF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CBF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CC00: 386A511C  addi r3, r10, 0x511c
	ctx.r[3].s64 = ctx.r[10].s64 + 20764;
	// 8262CC04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CC24: 4BE3A1FD  bl 0x82466e20
	ctx.lr = 0x8262CC28;
	sub_82466E20(ctx, base);
	// 8262CC28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CC38 size=108
    let mut pc: u32 = 0x8262CC38;
    'dispatch: loop {
        match pc {
            0x8262CC38 => {
    //   block [0x8262CC38..0x8262CCA4)
	// 8262CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CC44: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CC4C: 38EB9898  addi r7, r11, -0x6768
	ctx.r[7].s64 = ctx.r[11].s64 + -26472;
	// 8262CC50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262CC54: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8262CC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CC60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CC68: 386A514C  addi r3, r10, 0x514c
	ctx.r[3].s64 = ctx.r[10].s64 + 20812;
	// 8262CC6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CC8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CC90: 4BE3A191  bl 0x82466e20
	ctx.lr = 0x8262CC94;
	sub_82466E20(ctx, base);
	// 8262CC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CCA8 size=108
    let mut pc: u32 = 0x8262CCA8;
    'dispatch: loop {
        match pc {
            0x8262CCA8 => {
    //   block [0x8262CCA8..0x8262CD14)
	// 8262CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CCB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CCBC: 38EB98E0  addi r7, r11, -0x6720
	ctx.r[7].s64 = ctx.r[11].s64 + -26400;
	// 8262CCC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CCC4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8262CCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CCCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CCD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CCD8: 386A517C  addi r3, r10, 0x517c
	ctx.r[3].s64 = ctx.r[10].s64 + 20860;
	// 8262CCDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CD00: 4BE3A121  bl 0x82466e20
	ctx.lr = 0x8262CD04;
	sub_82466E20(ctx, base);
	// 8262CD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CD18 size=108
    let mut pc: u32 = 0x8262CD18;
    'dispatch: loop {
        match pc {
            0x8262CD18 => {
    //   block [0x8262CD18..0x8262CD84)
	// 8262CD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CD24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CD2C: 38EB9910  addi r7, r11, -0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26352;
	// 8262CD30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CD34: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8262CD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CD3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CD40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CD48: 386A51AC  addi r3, r10, 0x51ac
	ctx.r[3].s64 = ctx.r[10].s64 + 20908;
	// 8262CD4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CD70: 4BE3A0B1  bl 0x82466e20
	ctx.lr = 0x8262CD74;
	sub_82466E20(ctx, base);
	// 8262CD74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CD88 size=112
    let mut pc: u32 = 0x8262CD88;
    'dispatch: loop {
        match pc {
            0x8262CD88 => {
    //   block [0x8262CD88..0x8262CDF8)
	// 8262CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CD94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CD98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CD9C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CDA4: 390B9940  addi r8, r11, -0x66c0
	ctx.r[8].s64 = ctx.r[11].s64 + -26304;
	// 8262CDA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262CDAC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8262CDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CDC0: 386A51DC  addi r3, r10, 0x51dc
	ctx.r[3].s64 = ctx.r[10].s64 + 20956;
	// 8262CDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CDE4: 4BE3A03D  bl 0x82466e20
	ctx.lr = 0x8262CDE8;
	sub_82466E20(ctx, base);
	// 8262CDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CDF8 size=112
    let mut pc: u32 = 0x8262CDF8;
    'dispatch: loop {
        match pc {
            0x8262CDF8 => {
    //   block [0x8262CDF8..0x8262CE68)
	// 8262CDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CE04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CE0C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CE10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CE14: 390B9970  addi r8, r11, -0x6690
	ctx.r[8].s64 = ctx.r[11].s64 + -26256;
	// 8262CE18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CE1C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8262CE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CE24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CE30: 386A520C  addi r3, r10, 0x520c
	ctx.r[3].s64 = ctx.r[10].s64 + 21004;
	// 8262CE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CE54: 4BE39FCD  bl 0x82466e20
	ctx.lr = 0x8262CE58;
	sub_82466E20(ctx, base);
	// 8262CE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CE68 size=112
    let mut pc: u32 = 0x8262CE68;
    'dispatch: loop {
        match pc {
            0x8262CE68 => {
    //   block [0x8262CE68..0x8262CED8)
	// 8262CE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CE74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CE7C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CE84: 390B9988  addi r8, r11, -0x6678
	ctx.r[8].s64 = ctx.r[11].s64 + -26232;
	// 8262CE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CE8C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8262CE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CEA0: 386A523C  addi r3, r10, 0x523c
	ctx.r[3].s64 = ctx.r[10].s64 + 21052;
	// 8262CEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CEC4: 4BE39F5D  bl 0x82466e20
	ctx.lr = 0x8262CEC8;
	sub_82466E20(ctx, base);
	// 8262CEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CED8 size=108
    let mut pc: u32 = 0x8262CED8;
    'dispatch: loop {
        match pc {
            0x8262CED8 => {
    //   block [0x8262CED8..0x8262CF44)
	// 8262CED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CEE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CEEC: 38EB99A0  addi r7, r11, -0x6660
	ctx.r[7].s64 = ctx.r[11].s64 + -26208;
	// 8262CEF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CEF4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8262CEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CEFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CF08: 386A526C  addi r3, r10, 0x526c
	ctx.r[3].s64 = ctx.r[10].s64 + 21100;
	// 8262CF0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CF30: 4BE39EF1  bl 0x82466e20
	ctx.lr = 0x8262CF34;
	sub_82466E20(ctx, base);
	// 8262CF34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CF48 size=112
    let mut pc: u32 = 0x8262CF48;
    'dispatch: loop {
        match pc {
            0x8262CF48 => {
    //   block [0x8262CF48..0x8262CFB8)
	// 8262CF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CF54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CF5C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CF64: 390B99D0  addi r8, r11, -0x6630
	ctx.r[8].s64 = ctx.r[11].s64 + -26160;
	// 8262CF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CF6C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8262CF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CF74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CF80: 386A529C  addi r3, r10, 0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + 21148;
	// 8262CF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CFA4: 4BE39E7D  bl 0x82466e20
	ctx.lr = 0x8262CFA8;
	sub_82466E20(ctx, base);
	// 8262CFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CFB8 size=108
    let mut pc: u32 = 0x8262CFB8;
    'dispatch: loop {
        match pc {
            0x8262CFB8 => {
    //   block [0x8262CFB8..0x8262D024)
	// 8262CFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CFC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CFCC: 38EB99E8  addi r7, r11, -0x6618
	ctx.r[7].s64 = ctx.r[11].s64 + -26136;
	// 8262CFD0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8262CFD4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8262CFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CFDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CFE8: 386A52CC  addi r3, r10, 0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21196;
	// 8262CFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D010: 4BE39E11  bl 0x82466e20
	ctx.lr = 0x8262D014;
	sub_82466E20(ctx, base);
	// 8262D014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D028 size=112
    let mut pc: u32 = 0x8262D028;
    'dispatch: loop {
        match pc {
            0x8262D028 => {
    //   block [0x8262D028..0x8262D098)
	// 8262D028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D038: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D03C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D044: 390B9AD8  addi r8, r11, -0x6528
	ctx.r[8].s64 = ctx.r[11].s64 + -25896;
	// 8262D048: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8262D04C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8262D050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D060: 386A52FC  addi r3, r10, 0x52fc
	ctx.r[3].s64 = ctx.r[10].s64 + 21244;
	// 8262D064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D084: 4BE39D9D  bl 0x82466e20
	ctx.lr = 0x8262D088;
	sub_82466E20(ctx, base);
	// 8262D088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D098 size=108
    let mut pc: u32 = 0x8262D098;
    'dispatch: loop {
        match pc {
            0x8262D098 => {
    //   block [0x8262D098..0x8262D104)
	// 8262D098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D0A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D0AC: 38EB9C88  addi r7, r11, -0x6378
	ctx.r[7].s64 = ctx.r[11].s64 + -25464;
	// 8262D0B0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8262D0B4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8262D0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D0BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D0C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D0C8: 386A532C  addi r3, r10, 0x532c
	ctx.r[3].s64 = ctx.r[10].s64 + 21292;
	// 8262D0CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D0EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D0F0: 4BE39D31  bl 0x82466e20
	ctx.lr = 0x8262D0F4;
	sub_82466E20(ctx, base);
	// 8262D0F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D0F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D0FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D108 size=112
    let mut pc: u32 = 0x8262D108;
    'dispatch: loop {
        match pc {
            0x8262D108 => {
    //   block [0x8262D108..0x8262D178)
	// 8262D108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D118: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D11C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262D120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D124: 390B9E20  addi r8, r11, -0x61e0
	ctx.r[8].s64 = ctx.r[11].s64 + -25056;
	// 8262D128: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8262D12C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8262D130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D140: 386A535C  addi r3, r10, 0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + 21340;
	// 8262D144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D164: 4BE39CBD  bl 0x82466e20
	ctx.lr = 0x8262D168;
	sub_82466E20(ctx, base);
	// 8262D168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D178 size=100
    let mut pc: u32 = 0x8262D178;
    'dispatch: loop {
        match pc {
            0x8262D178 => {
    //   block [0x8262D178..0x8262D1DC)
	// 8262D178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D18C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D198: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8262D19C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D1AC: 386A538C  addi r3, r10, 0x538c
	ctx.r[3].s64 = ctx.r[10].s64 + 21388;
	// 8262D1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D1B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D1C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D1C8: 4BE39C59  bl 0x82466e20
	ctx.lr = 0x8262D1CC;
	sub_82466E20(ctx, base);
	// 8262D1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D1E0 size=112
    let mut pc: u32 = 0x8262D1E0;
    'dispatch: loop {
        match pc {
            0x8262D1E0 => {
    //   block [0x8262D1E0..0x8262D250)
	// 8262D1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D1EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D1F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D1F4: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 8262D1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D1FC: 390BA078  addi r8, r11, -0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + -24456;
	// 8262D200: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262D204: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8262D208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D20C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D218: 386A53BC  addi r3, r10, 0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + 21436;
	// 8262D21C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D23C: 4BE39BE5  bl 0x82466e20
	ctx.lr = 0x8262D240;
	sub_82466E20(ctx, base);
	// 8262D240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D250 size=100
    let mut pc: u32 = 0x8262D250;
    'dispatch: loop {
        match pc {
            0x8262D250 => {
    //   block [0x8262D250..0x8262D2B4)
	// 8262D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D264: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D270: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8262D274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D284: 386A53EC  addi r3, r10, 0x53ec
	ctx.r[3].s64 = ctx.r[10].s64 + 21484;
	// 8262D288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D28C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D290: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D2A0: 4BE39B81  bl 0x82466e20
	ctx.lr = 0x8262D2A4;
	sub_82466E20(ctx, base);
	// 8262D2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D2B8 size=108
    let mut pc: u32 = 0x8262D2B8;
    'dispatch: loop {
        match pc {
            0x8262D2B8 => {
    //   block [0x8262D2B8..0x8262D324)
	// 8262D2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D2C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D2CC: 38EBA0F0  addi r7, r11, -0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + -24336;
	// 8262D2D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D2D4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8262D2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D2DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D2E8: 386A541C  addi r3, r10, 0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + 21532;
	// 8262D2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D310: 4BE39B11  bl 0x82466e20
	ctx.lr = 0x8262D314;
	sub_82466E20(ctx, base);
	// 8262D314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D328 size=112
    let mut pc: u32 = 0x8262D328;
    'dispatch: loop {
        match pc {
            0x8262D328 => {
    //   block [0x8262D328..0x8262D398)
	// 8262D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D338: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D33C: 38AA53EC  addi r5, r10, 0x53ec
	ctx.r[5].s64 = ctx.r[10].s64 + 21484;
	// 8262D340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D344: 390BA138  addi r8, r11, -0x5ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -24264;
	// 8262D348: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262D34C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8262D350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D360: 386A544C  addi r3, r10, 0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + 21580;
	// 8262D364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D384: 4BE39A9D  bl 0x82466e20
	ctx.lr = 0x8262D388;
	sub_82466E20(ctx, base);
	// 8262D388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D398 size=100
    let mut pc: u32 = 0x8262D398;
    'dispatch: loop {
        match pc {
            0x8262D398 => {
    //   block [0x8262D398..0x8262D3FC)
	// 8262D398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D3A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D3AC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D3B8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8262D3BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D3CC: 386A547C  addi r3, r10, 0x547c
	ctx.r[3].s64 = ctx.r[10].s64 + 21628;
	// 8262D3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D3E8: 4BE39A39  bl 0x82466e20
	ctx.lr = 0x8262D3EC;
	sub_82466E20(ctx, base);
	// 8262D3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D400 size=100
    let mut pc: u32 = 0x8262D400;
    'dispatch: loop {
        match pc {
            0x8262D400 => {
    //   block [0x8262D400..0x8262D464)
	// 8262D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D414: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D420: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8262D424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D434: 386A54AC  addi r3, r10, 0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + 21676;
	// 8262D438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D43C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D450: 4BE399D1  bl 0x82466e20
	ctx.lr = 0x8262D454;
	sub_82466E20(ctx, base);
	// 8262D454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D468 size=112
    let mut pc: u32 = 0x8262D468;
    'dispatch: loop {
        match pc {
            0x8262D468 => {
    //   block [0x8262D468..0x8262D4D8)
	// 8262D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D478: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D47C: 38AA547C  addi r5, r10, 0x547c
	ctx.r[5].s64 = ctx.r[10].s64 + 21628;
	// 8262D480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D484: 390BA168  addi r8, r11, -0x5e98
	ctx.r[8].s64 = ctx.r[11].s64 + -24216;
	// 8262D488: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262D48C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8262D490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D4A0: 386A54DC  addi r3, r10, 0x54dc
	ctx.r[3].s64 = ctx.r[10].s64 + 21724;
	// 8262D4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D4C4: 4BE3995D  bl 0x82466e20
	ctx.lr = 0x8262D4C8;
	sub_82466E20(ctx, base);
	// 8262D4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D4D8 size=112
    let mut pc: u32 = 0x8262D4D8;
    'dispatch: loop {
        match pc {
            0x8262D4D8 => {
    //   block [0x8262D4D8..0x8262D548)
	// 8262D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D4E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D4EC: 38AA54AC  addi r5, r10, 0x54ac
	ctx.r[5].s64 = ctx.r[10].s64 + 21676;
	// 8262D4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D4F4: 390BA1C8  addi r8, r11, -0x5e38
	ctx.r[8].s64 = ctx.r[11].s64 + -24120;
	// 8262D4F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262D4FC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8262D500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D510: 386A550C  addi r3, r10, 0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + 21772;
	// 8262D514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D534: 4BE398ED  bl 0x82466e20
	ctx.lr = 0x8262D538;
	sub_82466E20(ctx, base);
	// 8262D538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D548 size=100
    let mut pc: u32 = 0x8262D548;
    'dispatch: loop {
        match pc {
            0x8262D548 => {
    //   block [0x8262D548..0x8262D5AC)
	// 8262D548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D554: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D55C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D568: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8262D56C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D57C: 386A553C  addi r3, r10, 0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + 21820;
	// 8262D580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D598: 4BE39889  bl 0x82466e20
	ctx.lr = 0x8262D59C;
	sub_82466E20(ctx, base);
	// 8262D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D5B0 size=112
    let mut pc: u32 = 0x8262D5B0;
    'dispatch: loop {
        match pc {
            0x8262D5B0 => {
    //   block [0x8262D5B0..0x8262D620)
	// 8262D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D5BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D5C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D5C4: 38AA553C  addi r5, r10, 0x553c
	ctx.r[5].s64 = ctx.r[10].s64 + 21820;
	// 8262D5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D5CC: 390BA228  addi r8, r11, -0x5dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -24024;
	// 8262D5D0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8262D5D4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8262D5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D5E8: 386A556C  addi r3, r10, 0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + 21868;
	// 8262D5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D60C: 4BE39815  bl 0x82466e20
	ctx.lr = 0x8262D610;
	sub_82466E20(ctx, base);
	// 8262D610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D620 size=108
    let mut pc: u32 = 0x8262D620;
    'dispatch: loop {
        match pc {
            0x8262D620 => {
    //   block [0x8262D620..0x8262D68C)
	// 8262D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D62C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D634: 38EBA318  addi r7, r11, -0x5ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -23784;
	// 8262D638: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8262D63C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8262D640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D650: 386A559C  addi r3, r10, 0x559c
	ctx.r[3].s64 = ctx.r[10].s64 + 21916;
	// 8262D654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D678: 4BE397A9  bl 0x82466e20
	ctx.lr = 0x8262D67C;
	sub_82466E20(ctx, base);
	// 8262D67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D690 size=108
    let mut pc: u32 = 0x8262D690;
    'dispatch: loop {
        match pc {
            0x8262D690 => {
    //   block [0x8262D690..0x8262D6FC)
	// 8262D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D69C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D6A4: 38EBA408  addi r7, r11, -0x5bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -23544;
	// 8262D6A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D6AC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8262D6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D6B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D6C0: 386A55CC  addi r3, r10, 0x55cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21964;
	// 8262D6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D6E8: 4BE39739  bl 0x82466e20
	ctx.lr = 0x8262D6EC;
	sub_82466E20(ctx, base);
	// 8262D6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D700 size=108
    let mut pc: u32 = 0x8262D700;
    'dispatch: loop {
        match pc {
            0x8262D700 => {
    //   block [0x8262D700..0x8262D76C)
	// 8262D700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D70C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D714: 38EBA450  addi r7, r11, -0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -23472;
	// 8262D718: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8262D71C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8262D720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D730: 386A55FC  addi r3, r10, 0x55fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22012;
	// 8262D734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D758: 4BE396C9  bl 0x82466e20
	ctx.lr = 0x8262D75C;
	sub_82466E20(ctx, base);
	// 8262D75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D770 size=108
    let mut pc: u32 = 0x8262D770;
    'dispatch: loop {
        match pc {
            0x8262D770 => {
    //   block [0x8262D770..0x8262D7DC)
	// 8262D770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D77C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D784: 38EBA528  addi r7, r11, -0x5ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -23256;
	// 8262D788: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262D78C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8262D790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D7A0: 386A562C  addi r3, r10, 0x562c
	ctx.r[3].s64 = ctx.r[10].s64 + 22060;
	// 8262D7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D7C8: 4BE39659  bl 0x82466e20
	ctx.lr = 0x8262D7CC;
	sub_82466E20(ctx, base);
	// 8262D7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D7E0 size=100
    let mut pc: u32 = 0x8262D7E0;
    'dispatch: loop {
        match pc {
            0x8262D7E0 => {
    //   block [0x8262D7E0..0x8262D844)
	// 8262D7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D7EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D7F4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D800: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8262D804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D814: 386A565C  addi r3, r10, 0x565c
	ctx.r[3].s64 = ctx.r[10].s64 + 22108;
	// 8262D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D81C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D830: 4BE395F1  bl 0x82466e20
	ctx.lr = 0x8262D834;
	sub_82466E20(ctx, base);
	// 8262D834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D848 size=112
    let mut pc: u32 = 0x8262D848;
    'dispatch: loop {
        match pc {
            0x8262D848 => {
    //   block [0x8262D848..0x8262D8B8)
	// 8262D848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D858: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D85C: 38AA565C  addi r5, r10, 0x565c
	ctx.r[5].s64 = ctx.r[10].s64 + 22108;
	// 8262D860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D864: 390BA540  addi r8, r11, -0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + -23232;
	// 8262D868: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262D86C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8262D870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D880: 386A568C  addi r3, r10, 0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + 22156;
	// 8262D884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D8A4: 4BE3957D  bl 0x82466e20
	ctx.lr = 0x8262D8A8;
	sub_82466E20(ctx, base);
	// 8262D8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D8B8 size=108
    let mut pc: u32 = 0x8262D8B8;
    'dispatch: loop {
        match pc {
            0x8262D8B8 => {
    //   block [0x8262D8B8..0x8262D924)
	// 8262D8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D8C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D8CC: 38EBA588  addi r7, r11, -0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + -23160;
	// 8262D8D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D8D4: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8262D8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D8DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D8E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D8E8: 386A56BC  addi r3, r10, 0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22204;
	// 8262D8EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D910: 4BE39511  bl 0x82466e20
	ctx.lr = 0x8262D914;
	sub_82466E20(ctx, base);
	// 8262D914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D928 size=112
    let mut pc: u32 = 0x8262D928;
    'dispatch: loop {
        match pc {
            0x8262D928 => {
    //   block [0x8262D928..0x8262D998)
	// 8262D928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D938: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D93C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D944: 390BA5D0  addi r8, r11, -0x5a30
	ctx.r[8].s64 = ctx.r[11].s64 + -23088;
	// 8262D948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262D94C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8262D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D960: 386A56EC  addi r3, r10, 0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + 22252;
	// 8262D964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D984: 4BE3949D  bl 0x82466e20
	ctx.lr = 0x8262D988;
	sub_82466E20(ctx, base);
	// 8262D988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D998 size=108
    let mut pc: u32 = 0x8262D998;
    'dispatch: loop {
        match pc {
            0x8262D998 => {
    //   block [0x8262D998..0x8262DA04)
	// 8262D998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D9AC: 38EBA5E8  addi r7, r11, -0x5a18
	ctx.r[7].s64 = ctx.r[11].s64 + -23064;
	// 8262D9B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D9B4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8262D9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D9C8: 386A571C  addi r3, r10, 0x571c
	ctx.r[3].s64 = ctx.r[10].s64 + 22300;
	// 8262D9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D9F0: 4BE39431  bl 0x82466e20
	ctx.lr = 0x8262D9F4;
	sub_82466E20(ctx, base);
	// 8262D9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DA08 size=112
    let mut pc: u32 = 0x8262DA08;
    'dispatch: loop {
        match pc {
            0x8262DA08 => {
    //   block [0x8262DA08..0x8262DA78)
	// 8262DA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DA14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DA1C: 38AA56EC  addi r5, r10, 0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + 22252;
	// 8262DA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DA24: 390BA630  addi r8, r11, -0x59d0
	ctx.r[8].s64 = ctx.r[11].s64 + -22992;
	// 8262DA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DA2C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8262DA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DA34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DA40: 386A574C  addi r3, r10, 0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + 22348;
	// 8262DA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DA64: 4BE393BD  bl 0x82466e20
	ctx.lr = 0x8262DA68;
	sub_82466E20(ctx, base);
	// 8262DA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DA78 size=100
    let mut pc: u32 = 0x8262DA78;
    'dispatch: loop {
        match pc {
            0x8262DA78 => {
    //   block [0x8262DA78..0x8262DADC)
	// 8262DA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DA84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DA8C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DA98: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8262DA9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DAAC: 386A577C  addi r3, r10, 0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + 22396;
	// 8262DAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DAB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DAB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262DABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DAC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DAC8: 4BE39359  bl 0x82466e20
	ctx.lr = 0x8262DACC;
	sub_82466E20(ctx, base);
	// 8262DACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DAE0 size=112
    let mut pc: u32 = 0x8262DAE0;
    'dispatch: loop {
        match pc {
            0x8262DAE0 => {
    //   block [0x8262DAE0..0x8262DB50)
	// 8262DAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DAEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DAF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DAF4: 38AA577C  addi r5, r10, 0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + 22396;
	// 8262DAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DAFC: 390BA648  addi r8, r11, -0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22968;
	// 8262DB00: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262DB04: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8262DB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DB18: 386A57AC  addi r3, r10, 0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + 22444;
	// 8262DB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DB3C: 4BE392E5  bl 0x82466e20
	ctx.lr = 0x8262DB40;
	sub_82466E20(ctx, base);
	// 8262DB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DB50 size=108
    let mut pc: u32 = 0x8262DB50;
    'dispatch: loop {
        match pc {
            0x8262DB50 => {
    //   block [0x8262DB50..0x8262DBBC)
	// 8262DB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DB5C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DB64: 38EBA6F0  addi r7, r11, -0x5910
	ctx.r[7].s64 = ctx.r[11].s64 + -22800;
	// 8262DB68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262DB6C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8262DB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DB74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DB78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262DB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DB80: 386A57DC  addi r3, r10, 0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + 22492;
	// 8262DB84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262DB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DBA8: 4BE39279  bl 0x82466e20
	ctx.lr = 0x8262DBAC;
	sub_82466E20(ctx, base);
	// 8262DBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DBC0 size=112
    let mut pc: u32 = 0x8262DBC0;
    'dispatch: loop {
        match pc {
            0x8262DBC0 => {
    //   block [0x8262DBC0..0x8262DC30)
	// 8262DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DBCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DBD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DBD4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DBDC: 390BA720  addi r8, r11, -0x58e0
	ctx.r[8].s64 = ctx.r[11].s64 + -22752;
	// 8262DBE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DBE4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8262DBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DBF8: 386A580C  addi r3, r10, 0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + 22540;
	// 8262DBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DC1C: 4BE39205  bl 0x82466e20
	ctx.lr = 0x8262DC20;
	sub_82466E20(ctx, base);
	// 8262DC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DC30 size=112
    let mut pc: u32 = 0x8262DC30;
    'dispatch: loop {
        match pc {
            0x8262DC30 => {
    //   block [0x8262DC30..0x8262DCA0)
	// 8262DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DC3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DC40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DC44: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DC4C: 390BA768  addi r8, r11, -0x5898
	ctx.r[8].s64 = ctx.r[11].s64 + -22680;
	// 8262DC50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DC54: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8262DC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DC68: 386A583C  addi r3, r10, 0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + 22588;
	// 8262DC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DC8C: 4BE39195  bl 0x82466e20
	ctx.lr = 0x8262DC90;
	sub_82466E20(ctx, base);
	// 8262DC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DCA0 size=100
    let mut pc: u32 = 0x8262DCA0;
    'dispatch: loop {
        match pc {
            0x8262DCA0 => {
    //   block [0x8262DCA0..0x8262DD04)
	// 8262DCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DCAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DCB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DCB4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DCBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DCC0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8262DCC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DCD4: 386A586C  addi r3, r10, 0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + 22636;
	// 8262DCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DCDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DCE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262DCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DCE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DCF0: 4BE39131  bl 0x82466e20
	ctx.lr = 0x8262DCF4;
	sub_82466E20(ctx, base);
	// 8262DCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DD08 size=112
    let mut pc: u32 = 0x8262DD08;
    'dispatch: loop {
        match pc {
            0x8262DD08 => {
    //   block [0x8262DD08..0x8262DD78)
	// 8262DD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DD14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DD1C: 38AA586C  addi r5, r10, 0x586c
	ctx.r[5].s64 = ctx.r[10].s64 + 22636;
	// 8262DD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DD24: 390BA7B0  addi r8, r11, -0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + -22608;
	// 8262DD28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DD2C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8262DD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DD40: 386A589C  addi r3, r10, 0x589c
	ctx.r[3].s64 = ctx.r[10].s64 + 22684;
	// 8262DD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DD64: 4BE390BD  bl 0x82466e20
	ctx.lr = 0x8262DD68;
	sub_82466E20(ctx, base);
	// 8262DD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DD78 size=112
    let mut pc: u32 = 0x8262DD78;
    'dispatch: loop {
        match pc {
            0x8262DD78 => {
    //   block [0x8262DD78..0x8262DDE8)
	// 8262DD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DD8C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DD94: 390BA7F8  addi r8, r11, -0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + -22536;
	// 8262DD98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DD9C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8262DDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DDA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DDB0: 386A58CC  addi r3, r10, 0x58cc
	ctx.r[3].s64 = ctx.r[10].s64 + 22732;
	// 8262DDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DDD4: 4BE3904D  bl 0x82466e20
	ctx.lr = 0x8262DDD8;
	sub_82466E20(ctx, base);
	// 8262DDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DDE8 size=112
    let mut pc: u32 = 0x8262DDE8;
    'dispatch: loop {
        match pc {
            0x8262DDE8 => {
    //   block [0x8262DDE8..0x8262DE58)
	// 8262DDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DDF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DDF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DDFC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DE04: 390BA810  addi r8, r11, -0x57f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22512;
	// 8262DE08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DE0C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8262DE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DE14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DE20: 386A58FC  addi r3, r10, 0x58fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22780;
	// 8262DE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DE34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262DE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DE44: 4BE38FDD  bl 0x82466e20
	ctx.lr = 0x8262DE48;
	sub_82466E20(ctx, base);
	// 8262DE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DE58 size=112
    let mut pc: u32 = 0x8262DE58;
    'dispatch: loop {
        match pc {
            0x8262DE58 => {
    //   block [0x8262DE58..0x8262DEC8)
	// 8262DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DE64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DE6C: 38AA58CC  addi r5, r10, 0x58cc
	ctx.r[5].s64 = ctx.r[10].s64 + 22732;
	// 8262DE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DE74: 390BA828  addi r8, r11, -0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + -22488;
	// 8262DE78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DE7C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8262DE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DE90: 386A592C  addi r3, r10, 0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + 22828;
	// 8262DE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DEB4: 4BE38F6D  bl 0x82466e20
	ctx.lr = 0x8262DEB8;
	sub_82466E20(ctx, base);
	// 8262DEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DEC8 size=72
    let mut pc: u32 = 0x8262DEC8;
    'dispatch: loop {
        match pc {
            0x8262DEC8 => {
    //   block [0x8262DEC8..0x8262DF10)
	// 8262DEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DED4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DED8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8262DEDC: 38CB2A70  addi r6, r11, 0x2a70
	ctx.r[6].s64 = ctx.r[11].s64 + 10864;
	// 8262DEE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DEE4: 388B3E20  addi r4, r11, 0x3e20
	ctx.r[4].s64 = ctx.r[11].s64 + 15904;
	// 8262DEE8: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262DEEC: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 8262DEF0: 4BE4DB99  bl 0x8247ba88
	ctx.lr = 0x8262DEF4;
	sub_8247BA88(ctx, base);
	// 8262DEF4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8262DEF8: 386BCD68  addi r3, r11, -0x3298
	ctx.r[3].s64 = ctx.r[11].s64 + -12952;
	// 8262DEFC: 4BF04C3D  bl 0x82532b38
	ctx.lr = 0x8262DF00;
	sub_82532B38(ctx, base);
	// 8262DF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262DF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF10 size=108
    let mut pc: u32 = 0x8262DF10;
    'dispatch: loop {
        match pc {
            0x8262DF10 => {
    //   block [0x8262DF10..0x8262DF7C)
	// 8262DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DF1C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DF24: 38EBBA10  addi r7, r11, -0x45f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17904;
	// 8262DF28: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262DF2C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8262DF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DF34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DF38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262DF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DF40: 386A5974  addi r3, r10, 0x5974
	ctx.r[3].s64 = ctx.r[10].s64 + 22900;
	// 8262DF44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262DF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DF68: 4BE38EB9  bl 0x82466e20
	ctx.lr = 0x8262DF6C;
	sub_82466E20(ctx, base);
	// 8262DF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DF80 size=24
    let mut pc: u32 = 0x8262DF80;
    'dispatch: loop {
        match pc {
            0x8262DF80 => {
    //   block [0x8262DF80..0x8262DF98)
	// 8262DF80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DF84: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262DF88: 394A1930  addi r10, r10, 0x1930
	ctx.r[10].s64 = ctx.r[10].s64 + 6448;
	// 8262DF8C: 816BBA88  lwz r11, -0x4578(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17784 as u32) ) } as u64;
	// 8262DF90: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8262DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF98 size=112
    let mut pc: u32 = 0x8262DF98;
    'dispatch: loop {
        match pc {
            0x8262DF98 => {
    //   block [0x8262DF98..0x8262E008)
	// 8262DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DFA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DFAC: 392B46EC  addi r9, r11, 0x46ec
	ctx.r[9].s64 = ctx.r[11].s64 + 18156;
	// 8262DFB0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8262DFB4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262DFB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DFBC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8262DFC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DFC4: 396B1930  addi r11, r11, 0x1930
	ctx.r[11].s64 = ctx.r[11].s64 + 6448;
	// 8262DFC8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262DFCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DFD0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262DFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DFD8: 386A59A4  addi r3, r10, 0x59a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22948;
	// 8262DFDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262DFE0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262DFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DFE8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262DFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DFF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DFF4: 4BE38E2D  bl 0x82466e20
	ctx.lr = 0x8262DFF8;
	sub_82466E20(ctx, base);
	// 8262DFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E008 size=108
    let mut pc: u32 = 0x8262E008;
    'dispatch: loop {
        match pc {
            0x8262E008 => {
    //   block [0x8262E008..0x8262E074)
	// 8262E008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E014: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E01C: 38EBBA8C  addi r7, r11, -0x4574
	ctx.r[7].s64 = ctx.r[11].s64 + -17780;
	// 8262E020: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E024: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8262E028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E038: 386A59D4  addi r3, r10, 0x59d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22996;
	// 8262E03C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E060: 4BE38DC1  bl 0x82466e20
	ctx.lr = 0x8262E064;
	sub_82466E20(ctx, base);
	// 8262E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E078 size=108
    let mut pc: u32 = 0x8262E078;
    'dispatch: loop {
        match pc {
            0x8262E078 => {
    //   block [0x8262E078..0x8262E0E4)
	// 8262E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E084: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E08C: 38EBBABC  addi r7, r11, -0x4544
	ctx.r[7].s64 = ctx.r[11].s64 + -17732;
	// 8262E090: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E094: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8262E098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E09C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E0A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E0A8: 386A5A04  addi r3, r10, 0x5a04
	ctx.r[3].s64 = ctx.r[10].s64 + 23044;
	// 8262E0AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E0CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E0D0: 4BE38D51  bl 0x82466e20
	ctx.lr = 0x8262E0D4;
	sub_82466E20(ctx, base);
	// 8262E0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E0E8 size=112
    let mut pc: u32 = 0x8262E0E8;
    'dispatch: loop {
        match pc {
            0x8262E0E8 => {
    //   block [0x8262E0E8..0x8262E158)
	// 8262E0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E0F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E0FC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E104: 390BBAF0  addi r8, r11, -0x4510
	ctx.r[8].s64 = ctx.r[11].s64 + -17680;
	// 8262E108: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262E10C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8262E110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E120: 386A5A34  addi r3, r10, 0x5a34
	ctx.r[3].s64 = ctx.r[10].s64 + 23092;
	// 8262E124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E144: 4BE38CDD  bl 0x82466e20
	ctx.lr = 0x8262E148;
	sub_82466E20(ctx, base);
	// 8262E148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E158 size=108
    let mut pc: u32 = 0x8262E158;
    'dispatch: loop {
        match pc {
            0x8262E158 => {
    //   block [0x8262E158..0x8262E1C4)
	// 8262E158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E164: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E16C: 38EBBB50  addi r7, r11, -0x44b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17584;
	// 8262E170: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262E174: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8262E178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E188: 386A5A64  addi r3, r10, 0x5a64
	ctx.r[3].s64 = ctx.r[10].s64 + 23140;
	// 8262E18C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E1B0: 4BE38C71  bl 0x82466e20
	ctx.lr = 0x8262E1B4;
	sub_82466E20(ctx, base);
	// 8262E1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E1C8 size=112
    let mut pc: u32 = 0x8262E1C8;
    'dispatch: loop {
        match pc {
            0x8262E1C8 => {
    //   block [0x8262E1C8..0x8262E238)
	// 8262E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E1D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E1DC: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E1E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E1E4: 390BBBC8  addi r8, r11, -0x4438
	ctx.r[8].s64 = ctx.r[11].s64 + -17464;
	// 8262E1E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262E1EC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8262E1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E1F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E200: 386A5A94  addi r3, r10, 0x5a94
	ctx.r[3].s64 = ctx.r[10].s64 + 23188;
	// 8262E204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E224: 4BE38BFD  bl 0x82466e20
	ctx.lr = 0x8262E228;
	sub_82466E20(ctx, base);
	// 8262E228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E238 size=112
    let mut pc: u32 = 0x8262E238;
    'dispatch: loop {
        match pc {
            0x8262E238 => {
    //   block [0x8262E238..0x8262E2A8)
	// 8262E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E24C: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E254: 390BBC70  addi r8, r11, -0x4390
	ctx.r[8].s64 = ctx.r[11].s64 + -17296;
	// 8262E258: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262E25C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8262E260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E270: 386A5AC4  addi r3, r10, 0x5ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 23236;
	// 8262E274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E294: 4BE38B8D  bl 0x82466e20
	ctx.lr = 0x8262E298;
	sub_82466E20(ctx, base);
	// 8262E298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E2A8 size=108
    let mut pc: u32 = 0x8262E2A8;
    'dispatch: loop {
        match pc {
            0x8262E2A8 => {
    //   block [0x8262E2A8..0x8262E314)
	// 8262E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E2B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E2BC: 38EBBC88  addi r7, r11, -0x4378
	ctx.r[7].s64 = ctx.r[11].s64 + -17272;
	// 8262E2C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262E2C4: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8262E2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E2CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E2D8: 386A5AF4  addi r3, r10, 0x5af4
	ctx.r[3].s64 = ctx.r[10].s64 + 23284;
	// 8262E2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E300: 4BE38B21  bl 0x82466e20
	ctx.lr = 0x8262E304;
	sub_82466E20(ctx, base);
	// 8262E304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E318 size=112
    let mut pc: u32 = 0x8262E318;
    'dispatch: loop {
        match pc {
            0x8262E318 => {
    //   block [0x8262E318..0x8262E388)
	// 8262E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E328: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E32C: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E334: 390BBD00  addi r8, r11, -0x4300
	ctx.r[8].s64 = ctx.r[11].s64 + -17152;
	// 8262E338: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262E33C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8262E340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E350: 386A5B24  addi r3, r10, 0x5b24
	ctx.r[3].s64 = ctx.r[10].s64 + 23332;
	// 8262E354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E374: 4BE38AAD  bl 0x82466e20
	ctx.lr = 0x8262E378;
	sub_82466E20(ctx, base);
	// 8262E378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E388 size=108
    let mut pc: u32 = 0x8262E388;
    'dispatch: loop {
        match pc {
            0x8262E388 => {
    //   block [0x8262E388..0x8262E3F4)
	// 8262E388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E394: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E39C: 38EBBDA8  addi r7, r11, -0x4258
	ctx.r[7].s64 = ctx.r[11].s64 + -16984;
	// 8262E3A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E3A4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8262E3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E3AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E3B8: 386A5B54  addi r3, r10, 0x5b54
	ctx.r[3].s64 = ctx.r[10].s64 + 23380;
	// 8262E3BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E3E0: 4BE38A41  bl 0x82466e20
	ctx.lr = 0x8262E3E4;
	sub_82466E20(ctx, base);
	// 8262E3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E3F8 size=108
    let mut pc: u32 = 0x8262E3F8;
    'dispatch: loop {
        match pc {
            0x8262E3F8 => {
    //   block [0x8262E3F8..0x8262E464)
	// 8262E3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E404: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E40C: 38EBBDC0  addi r7, r11, -0x4240
	ctx.r[7].s64 = ctx.r[11].s64 + -16960;
	// 8262E410: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E414: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8262E418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E41C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E428: 386A5B84  addi r3, r10, 0x5b84
	ctx.r[3].s64 = ctx.r[10].s64 + 23428;
	// 8262E42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E450: 4BE389D1  bl 0x82466e20
	ctx.lr = 0x8262E454;
	sub_82466E20(ctx, base);
	// 8262E454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E468 size=116
    let mut pc: u32 = 0x8262E468;
    'dispatch: loop {
        match pc {
            0x8262E468 => {
    //   block [0x8262E468..0x8262E4DC)
	// 8262E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E474: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E478: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E47C: 390BBE20  addi r8, r11, -0x41e0
	ctx.r[8].s64 = ctx.r[11].s64 + -16864;
	// 8262E480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E484: 392A4728  addi r9, r10, 0x4728
	ctx.r[9].s64 = ctx.r[10].s64 + 18216;
	// 8262E488: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E48C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262E490: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E494: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E49C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E4AC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262E4B0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8262E4B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262E4B8: 386B5BB4  addi r3, r11, 0x5bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23476;
	// 8262E4BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262E4C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E4C8: 4BE38959  bl 0x82466e20
	ctx.lr = 0x8262E4CC;
	sub_82466E20(ctx, base);
	// 8262E4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E4E0 size=108
    let mut pc: u32 = 0x8262E4E0;
    'dispatch: loop {
        match pc {
            0x8262E4E0 => {
    //   block [0x8262E4E0..0x8262E54C)
	// 8262E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E4EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E4F4: 38EBBE38  addi r7, r11, -0x41c8
	ctx.r[7].s64 = ctx.r[11].s64 + -16840;
	// 8262E4F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262E4FC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8262E500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E510: 386A5BE4  addi r3, r10, 0x5be4
	ctx.r[3].s64 = ctx.r[10].s64 + 23524;
	// 8262E514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E538: 4BE388E9  bl 0x82466e20
	ctx.lr = 0x8262E53C;
	sub_82466E20(ctx, base);
	// 8262E53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E550 size=108
    let mut pc: u32 = 0x8262E550;
    'dispatch: loop {
        match pc {
            0x8262E550 => {
    //   block [0x8262E550..0x8262E5BC)
	// 8262E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E55C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E564: 38EBBE80  addi r7, r11, -0x4180
	ctx.r[7].s64 = ctx.r[11].s64 + -16768;
	// 8262E568: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262E56C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8262E570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E580: 386A5C14  addi r3, r10, 0x5c14
	ctx.r[3].s64 = ctx.r[10].s64 + 23572;
	// 8262E584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E5A8: 4BE38879  bl 0x82466e20
	ctx.lr = 0x8262E5AC;
	sub_82466E20(ctx, base);
	// 8262E5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E5C0 size=108
    let mut pc: u32 = 0x8262E5C0;
    'dispatch: loop {
        match pc {
            0x8262E5C0 => {
    //   block [0x8262E5C0..0x8262E62C)
	// 8262E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E5CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E5D4: 38EBBF10  addi r7, r11, -0x40f0
	ctx.r[7].s64 = ctx.r[11].s64 + -16624;
	// 8262E5D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262E5DC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8262E5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E5E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E5F0: 386A5C44  addi r3, r10, 0x5c44
	ctx.r[3].s64 = ctx.r[10].s64 + 23620;
	// 8262E5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E618: 4BE38809  bl 0x82466e20
	ctx.lr = 0x8262E61C;
	sub_82466E20(ctx, base);
	// 8262E61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E630 size=100
    let mut pc: u32 = 0x8262E630;
    'dispatch: loop {
        match pc {
            0x8262E630 => {
    //   block [0x8262E630..0x8262E694)
	// 8262E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E63C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E644: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E650: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8262E654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E664: 386A5C74  addi r3, r10, 0x5c74
	ctx.r[3].s64 = ctx.r[10].s64 + 23668;
	// 8262E668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E670: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262E674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E678: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262E67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E680: 4BE387A1  bl 0x82466e20
	ctx.lr = 0x8262E684;
	sub_82466E20(ctx, base);
	// 8262E684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E698 size=112
    let mut pc: u32 = 0x8262E698;
    'dispatch: loop {
        match pc {
            0x8262E698 => {
    //   block [0x8262E698..0x8262E708)
	// 8262E698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E6A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E6AC: 38AA5C74  addi r5, r10, 0x5c74
	ctx.r[5].s64 = ctx.r[10].s64 + 23668;
	// 8262E6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E6B4: 390BBFA0  addi r8, r11, -0x4060
	ctx.r[8].s64 = ctx.r[11].s64 + -16480;
	// 8262E6B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262E6BC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8262E6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E6D0: 386A5CA4  addi r3, r10, 0x5ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 23716;
	// 8262E6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E6F4: 4BE3872D  bl 0x82466e20
	ctx.lr = 0x8262E6F8;
	sub_82466E20(ctx, base);
	// 8262E6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E708 size=108
    let mut pc: u32 = 0x8262E708;
    'dispatch: loop {
        match pc {
            0x8262E708 => {
    //   block [0x8262E708..0x8262E774)
	// 8262E708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E714: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E71C: 38EBC000  addi r7, r11, -0x4000
	ctx.r[7].s64 = ctx.r[11].s64 + -16384;
	// 8262E720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E724: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8262E728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E738: 386A5CD4  addi r3, r10, 0x5cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 23764;
	// 8262E73C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E760: 4BE386C1  bl 0x82466e20
	ctx.lr = 0x8262E764;
	sub_82466E20(ctx, base);
	// 8262E764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E778 size=108
    let mut pc: u32 = 0x8262E778;
    'dispatch: loop {
        match pc {
            0x8262E778 => {
    //   block [0x8262E778..0x8262E7E4)
	// 8262E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E784: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E78C: 38EBC030  addi r7, r11, -0x3fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -16336;
	// 8262E790: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E794: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8262E798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E7A8: 386A5D04  addi r3, r10, 0x5d04
	ctx.r[3].s64 = ctx.r[10].s64 + 23812;
	// 8262E7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E7D0: 4BE38651  bl 0x82466e20
	ctx.lr = 0x8262E7D4;
	sub_82466E20(ctx, base);
	// 8262E7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E7E8 size=108
    let mut pc: u32 = 0x8262E7E8;
    'dispatch: loop {
        match pc {
            0x8262E7E8 => {
    //   block [0x8262E7E8..0x8262E854)
	// 8262E7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E7F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E7FC: 38EBC090  addi r7, r11, -0x3f70
	ctx.r[7].s64 = ctx.r[11].s64 + -16240;
	// 8262E800: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E804: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8262E808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E818: 386A5D34  addi r3, r10, 0x5d34
	ctx.r[3].s64 = ctx.r[10].s64 + 23860;
	// 8262E81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E840: 4BE385E1  bl 0x82466e20
	ctx.lr = 0x8262E844;
	sub_82466E20(ctx, base);
	// 8262E844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E858 size=112
    let mut pc: u32 = 0x8262E858;
    'dispatch: loop {
        match pc {
            0x8262E858 => {
    //   block [0x8262E858..0x8262E8C8)
	// 8262E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E864: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E868: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E86C: 392A475C  addi r9, r10, 0x475c
	ctx.r[9].s64 = ctx.r[10].s64 + 18268;
	// 8262E870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E874: 390BC0F8  addi r8, r11, -0x3f08
	ctx.r[8].s64 = ctx.r[11].s64 + -16136;
	// 8262E878: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8262E87C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8262E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E890: 386A5D64  addi r3, r10, 0x5d64
	ctx.r[3].s64 = ctx.r[10].s64 + 23908;
	// 8262E894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262E898: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262E89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E8B4: 4BE3856D  bl 0x82466e20
	ctx.lr = 0x8262E8B8;
	sub_82466E20(ctx, base);
	// 8262E8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E8C8 size=108
    let mut pc: u32 = 0x8262E8C8;
    'dispatch: loop {
        match pc {
            0x8262E8C8 => {
    //   block [0x8262E8C8..0x8262E934)
	// 8262E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E8D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E8DC: 38EBC200  addi r7, r11, -0x3e00
	ctx.r[7].s64 = ctx.r[11].s64 + -15872;
	// 8262E8E0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8262E8E4: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 8262E8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E8EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E8F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E8F8: 386A5D94  addi r3, r10, 0x5d94
	ctx.r[3].s64 = ctx.r[10].s64 + 23956;
	// 8262E8FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E91C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E920: 4BE38501  bl 0x82466e20
	ctx.lr = 0x8262E924;
	sub_82466E20(ctx, base);
	// 8262E924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E938 size=108
    let mut pc: u32 = 0x8262E938;
    'dispatch: loop {
        match pc {
            0x8262E938 => {
    //   block [0x8262E938..0x8262E9A4)
	// 8262E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E944: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E94C: 38EBC338  addi r7, r11, -0x3cc8
	ctx.r[7].s64 = ctx.r[11].s64 + -15560;
	// 8262E950: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E954: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 8262E958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E95C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E968: 386A5DC4  addi r3, r10, 0x5dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 24004;
	// 8262E96C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E98C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E990: 4BE38491  bl 0x82466e20
	ctx.lr = 0x8262E994;
	sub_82466E20(ctx, base);
	// 8262E994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E9A8 size=108
    let mut pc: u32 = 0x8262E9A8;
    'dispatch: loop {
        match pc {
            0x8262E9A8 => {
    //   block [0x8262E9A8..0x8262EA14)
	// 8262E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E9B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E9B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E9BC: 38EBC350  addi r7, r11, -0x3cb0
	ctx.r[7].s64 = ctx.r[11].s64 + -15536;
	// 8262E9C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E9C4: 388A5948  addi r4, r10, 0x5948
	ctx.r[4].s64 = ctx.r[10].s64 + 22856;
	// 8262E9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E9D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E9D8: 386A5DF4  addi r3, r10, 0x5df4
	ctx.r[3].s64 = ctx.r[10].s64 + 24052;
	// 8262E9DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EA00: 4BE38421  bl 0x82466e20
	ctx.lr = 0x8262EA04;
	sub_82466E20(ctx, base);
	// 8262EA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EA18 size=108
    let mut pc: u32 = 0x8262EA18;
    'dispatch: loop {
        match pc {
            0x8262EA18 => {
    //   block [0x8262EA18..0x8262EA84)
	// 8262EA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EA24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EA2C: 38EBC368  addi r7, r11, -0x3c98
	ctx.r[7].s64 = ctx.r[11].s64 + -15512;
	// 8262EA30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262EA34: 388A595C  addi r4, r10, 0x595c
	ctx.r[4].s64 = ctx.r[10].s64 + 22876;
	// 8262EA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EA3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EA40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262EA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EA48: 386A5E24  addi r3, r10, 0x5e24
	ctx.r[3].s64 = ctx.r[10].s64 + 24100;
	// 8262EA4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262EA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EA6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EA70: 4BE383B1  bl 0x82466e20
	ctx.lr = 0x8262EA74;
	sub_82466E20(ctx, base);
	// 8262EA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EA88 size=100
    let mut pc: u32 = 0x8262EA88;
    'dispatch: loop {
        match pc {
            0x8262EA88 => {
    //   block [0x8262EA88..0x8262EAEC)
	// 8262EA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EA94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EA9C: 38AA62D4  addi r5, r10, 0x62d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25300;
	// 8262EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EAA8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8262EAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EABC: 386A5E54  addi r3, r10, 0x5e54
	ctx.r[3].s64 = ctx.r[10].s64 + 24148;
	// 8262EAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EAC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EAC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EAD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EAD8: 4BE38349  bl 0x82466e20
	ctx.lr = 0x8262EADC;
	sub_82466E20(ctx, base);
	// 8262EADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EAF0 size=112
    let mut pc: u32 = 0x8262EAF0;
    'dispatch: loop {
        match pc {
            0x8262EAF0 => {
    //   block [0x8262EAF0..0x8262EB60)
	// 8262EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EAFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EB04: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EB0C: 390BC398  addi r8, r11, -0x3c68
	ctx.r[8].s64 = ctx.r[11].s64 + -15464;
	// 8262EB10: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262EB14: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 8262EB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EB1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EB28: 386A5E84  addi r3, r10, 0x5e84
	ctx.r[3].s64 = ctx.r[10].s64 + 24196;
	// 8262EB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EB4C: 4BE382D5  bl 0x82466e20
	ctx.lr = 0x8262EB50;
	sub_82466E20(ctx, base);
	// 8262EB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EB60 size=100
    let mut pc: u32 = 0x8262EB60;
    'dispatch: loop {
        match pc {
            0x8262EB60 => {
    //   block [0x8262EB60..0x8262EBC4)
	// 8262EB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EB6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EB74: 38AA5E84  addi r5, r10, 0x5e84
	ctx.r[5].s64 = ctx.r[10].s64 + 24196;
	// 8262EB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EB80: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 8262EB84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EB94: 386A5EB4  addi r3, r10, 0x5eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24244;
	// 8262EB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EB9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EBA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EBA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EBA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EBB0: 4BE38271  bl 0x82466e20
	ctx.lr = 0x8262EBB4;
	sub_82466E20(ctx, base);
	// 8262EBB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EBC8 size=108
    let mut pc: u32 = 0x8262EBC8;
    'dispatch: loop {
        match pc {
            0x8262EBC8 => {
    //   block [0x8262EBC8..0x8262EC34)
	// 8262EBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EBD4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EBDC: 38EBC428  addi r7, r11, -0x3bd8
	ctx.r[7].s64 = ctx.r[11].s64 + -15320;
	// 8262EBE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262EBE4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 8262EBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262EBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EBF8: 386A5EE4  addi r3, r10, 0x5ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 24292;
	// 8262EBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262EC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EC20: 4BE38201  bl 0x82466e20
	ctx.lr = 0x8262EC24;
	sub_82466E20(ctx, base);
	// 8262EC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EC38 size=112
    let mut pc: u32 = 0x8262EC38;
    'dispatch: loop {
        match pc {
            0x8262EC38 => {
    //   block [0x8262EC38..0x8262ECA8)
	// 8262EC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EC44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EC48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EC4C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EC54: 390BC470  addi r8, r11, -0x3b90
	ctx.r[8].s64 = ctx.r[11].s64 + -15248;
	// 8262EC58: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262EC5C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 8262EC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EC70: 386A5F14  addi r3, r10, 0x5f14
	ctx.r[3].s64 = ctx.r[10].s64 + 24340;
	// 8262EC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EC94: 4BE3818D  bl 0x82466e20
	ctx.lr = 0x8262EC98;
	sub_82466E20(ctx, base);
	// 8262EC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ECA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ECA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ECA8 size=108
    let mut pc: u32 = 0x8262ECA8;
    'dispatch: loop {
        match pc {
            0x8262ECA8 => {
    //   block [0x8262ECA8..0x8262ED14)
	// 8262ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ECB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ECB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ECBC: 38EBC4E8  addi r7, r11, -0x3b18
	ctx.r[7].s64 = ctx.r[11].s64 + -15128;
	// 8262ECC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262ECC4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 8262ECC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ECCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ECD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262ECD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ECD8: 386A5F44  addi r3, r10, 0x5f44
	ctx.r[3].s64 = ctx.r[10].s64 + 24388;
	// 8262ECDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262ECE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ECE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ECF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ECF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ECF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ECFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262ED00: 4BE38121  bl 0x82466e20
	ctx.lr = 0x8262ED04;
	sub_82466E20(ctx, base);
	// 8262ED04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ED08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ED0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ED10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262ED18 size=24
    let mut pc: u32 = 0x8262ED18;
    'dispatch: loop {
        match pc {
            0x8262ED18 => {
    //   block [0x8262ED18..0x8262ED30)
	// 8262ED18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ED1C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262ED20: 394A1978  addi r10, r10, 0x1978
	ctx.r[10].s64 = ctx.r[10].s64 + 6520;
	// 8262ED24: 816BC0F4  lwz r11, -0x3f0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16140 as u32) ) } as u64;
	// 8262ED28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262ED2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ED30 size=116
    let mut pc: u32 = 0x8262ED30;
    'dispatch: loop {
        match pc {
            0x8262ED30 => {
    //   block [0x8262ED30..0x8262EDA4)
	// 8262ED30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ED34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ED38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ED3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262ED40: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ED44: 392B4788  addi r9, r11, 0x4788
	ctx.r[9].s64 = ctx.r[11].s64 + 18312;
	// 8262ED48: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262ED4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ED50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262ED54: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8262ED58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ED5C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 8262ED60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ED64: 396B1978  addi r11, r11, 0x1978
	ctx.r[11].s64 = ctx.r[11].s64 + 6520;
	// 8262ED68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262ED6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ED70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262ED74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ED78: 386A5F74  addi r3, r10, 0x5f74
	ctx.r[3].s64 = ctx.r[10].s64 + 24436;
	// 8262ED7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262ED80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262ED84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ED88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262ED8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262ED90: 4BE38091  bl 0x82466e20
	ctx.lr = 0x8262ED94;
	sub_82466E20(ctx, base);
	// 8262ED94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ED98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ED9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EDA8 size=112
    let mut pc: u32 = 0x8262EDA8;
    'dispatch: loop {
        match pc {
            0x8262EDA8 => {
    //   block [0x8262EDA8..0x8262EE18)
	// 8262EDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EDB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EDBC: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EDC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EDC4: 390BC530  addi r8, r11, -0x3ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -15056;
	// 8262EDC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262EDCC: 388A596C  addi r4, r10, 0x596c
	ctx.r[4].s64 = ctx.r[10].s64 + 22892;
	// 8262EDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EDD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EDD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EDE0: 386A5FA4  addi r3, r10, 0x5fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 24484;
	// 8262EDE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EDE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EDFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EE04: 4BE3801D  bl 0x82466e20
	ctx.lr = 0x8262EE08;
	sub_82466E20(ctx, base);
	// 8262EE08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EE18 size=112
    let mut pc: u32 = 0x8262EE18;
    'dispatch: loop {
        match pc {
            0x8262EE18 => {
    //   block [0x8262EE18..0x8262EE88)
	// 8262EE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EE24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EE2C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EE34: 390BC560  addi r8, r11, -0x3aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -15008;
	// 8262EE38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262EE3C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8262EE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EE44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EE50: 386A5FD4  addi r3, r10, 0x5fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24532;
	// 8262EE54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EE74: 4BE37FAD  bl 0x82466e20
	ctx.lr = 0x8262EE78;
	sub_82466E20(ctx, base);
	// 8262EE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EE88 size=112
    let mut pc: u32 = 0x8262EE88;
    'dispatch: loop {
        match pc {
            0x8262EE88 => {
    //   block [0x8262EE88..0x8262EEF8)
	// 8262EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EE9C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EEA4: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 8262EEA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262EEAC: 388A5984  addi r4, r10, 0x5984
	ctx.r[4].s64 = ctx.r[10].s64 + 22916;
	// 8262EEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EEB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EEC0: 386A6004  addi r3, r10, 0x6004
	ctx.r[3].s64 = ctx.r[10].s64 + 24580;
	// 8262EEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EEE4: 4BE37F3D  bl 0x82466e20
	ctx.lr = 0x8262EEE8;
	sub_82466E20(ctx, base);
	// 8262EEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EEF8 size=100
    let mut pc: u32 = 0x8262EEF8;
    'dispatch: loop {
        match pc {
            0x8262EEF8 => {
    //   block [0x8262EEF8..0x8262EF5C)
	// 8262EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EF04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EF0C: 38AA62D4  addi r5, r10, 0x62d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25300;
	// 8262EF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EF18: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8262EF1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EF2C: 386A6034  addi r3, r10, 0x6034
	ctx.r[3].s64 = ctx.r[10].s64 + 24628;
	// 8262EF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EF34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EF38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EF40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EF48: 4BE37ED9  bl 0x82466e20
	ctx.lr = 0x8262EF4C;
	sub_82466E20(ctx, base);
	// 8262EF4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EF60 size=116
    let mut pc: u32 = 0x8262EF60;
    'dispatch: loop {
        match pc {
            0x8262EF60 => {
    //   block [0x8262EF60..0x8262EFD4)
	// 8262EF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EF6C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262EF70: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8262EF74: 390AC5A8  addi r8, r10, -0x3a58
	ctx.r[8].s64 = ctx.r[10].s64 + -14936;
	// 8262EF78: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262EF80: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262EF84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EF88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262EF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EF90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EF94: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8262EF98: 396B47D0  addi r11, r11, 0x47d0
	ctx.r[11].s64 = ctx.r[11].s64 + 18384;
	// 8262EF9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EFA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EFA4: 386A6064  addi r3, r10, 0x6064
	ctx.r[3].s64 = ctx.r[10].s64 + 24676;
	// 8262EFA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262EFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EFB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262EFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EFC0: 4BE37E61  bl 0x82466e20
	ctx.lr = 0x8262EFC4;
	sub_82466E20(ctx, base);
	// 8262EFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EFD8 size=112
    let mut pc: u32 = 0x8262EFD8;
    'dispatch: loop {
        match pc {
            0x8262EFD8 => {
    //   block [0x8262EFD8..0x8262F048)
	// 8262EFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EFE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EFE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EFEC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262EFF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EFF4: 390BC740  addi r8, r11, -0x38c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14528;
	// 8262EFF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262EFFC: 388A5998  addi r4, r10, 0x5998
	ctx.r[4].s64 = ctx.r[10].s64 + 22936;
	// 8262F000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F010: 386A6094  addi r3, r10, 0x6094
	ctx.r[3].s64 = ctx.r[10].s64 + 24724;
	// 8262F014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F034: 4BE37DED  bl 0x82466e20
	ctx.lr = 0x8262F038;
	sub_82466E20(ctx, base);
	// 8262F038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F048 size=112
    let mut pc: u32 = 0x8262F048;
    'dispatch: loop {
        match pc {
            0x8262F048 => {
    //   block [0x8262F048..0x8262F0B8)
	// 8262F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F058: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F05C: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F064: 390BC758  addi r8, r11, -0x38a8
	ctx.r[8].s64 = ctx.r[11].s64 + -14504;
	// 8262F068: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262F06C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8262F070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F080: 386A60C4  addi r3, r10, 0x60c4
	ctx.r[3].s64 = ctx.r[10].s64 + 24772;
	// 8262F084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F0A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F0A4: 4BE37D7D  bl 0x82466e20
	ctx.lr = 0x8262F0A8;
	sub_82466E20(ctx, base);
	// 8262F0A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F0B8 size=112
    let mut pc: u32 = 0x8262F0B8;
    'dispatch: loop {
        match pc {
            0x8262F0B8 => {
    //   block [0x8262F0B8..0x8262F128)
	// 8262F0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F0C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F0C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F0CC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F0D4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 8262F0D8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8262F0DC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8262F0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F0E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F0F0: 386A60F4  addi r3, r10, 0x60f4
	ctx.r[3].s64 = ctx.r[10].s64 + 24820;
	// 8262F0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F114: 4BE37D0D  bl 0x82466e20
	ctx.lr = 0x8262F118;
	sub_82466E20(ctx, base);
	// 8262F118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F128 size=112
    let mut pc: u32 = 0x8262F128;
    'dispatch: loop {
        match pc {
            0x8262F128 => {
    //   block [0x8262F128..0x8262F198)
	// 8262F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F138: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F13C: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F144: 390BC8D8  addi r8, r11, -0x3728
	ctx.r[8].s64 = ctx.r[11].s64 + -14120;
	// 8262F148: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8262F14C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8262F150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F160: 386A6124  addi r3, r10, 0x6124
	ctx.r[3].s64 = ctx.r[10].s64 + 24868;
	// 8262F164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F184: 4BE37C9D  bl 0x82466e20
	ctx.lr = 0x8262F188;
	sub_82466E20(ctx, base);
	// 8262F188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F198 size=112
    let mut pc: u32 = 0x8262F198;
    'dispatch: loop {
        match pc {
            0x8262F198 => {
    //   block [0x8262F198..0x8262F208)
	// 8262F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F1A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F1A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F1AC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F1B4: 390BC9E0  addi r8, r11, -0x3620
	ctx.r[8].s64 = ctx.r[11].s64 + -13856;
	// 8262F1B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262F1BC: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 8262F1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F1C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F1D0: 386A6154  addi r3, r10, 0x6154
	ctx.r[3].s64 = ctx.r[10].s64 + 24916;
	// 8262F1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F1F4: 4BE37C2D  bl 0x82466e20
	ctx.lr = 0x8262F1F8;
	sub_82466E20(ctx, base);
	// 8262F1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F208 size=24
    let mut pc: u32 = 0x8262F208;
    'dispatch: loop {
        match pc {
            0x8262F208 => {
    //   block [0x8262F208..0x8262F220)
	// 8262F208: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F20C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F210: 394A1A98  addi r10, r10, 0x1a98
	ctx.r[10].s64 = ctx.r[10].s64 + 6808;
	// 8262F214: 816BCA10  lwz r11, -0x35f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13808 as u32) ) } as u64;
	// 8262F218: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262F21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F220 size=116
    let mut pc: u32 = 0x8262F220;
    'dispatch: loop {
        match pc {
            0x8262F220 => {
    //   block [0x8262F220..0x8262F294)
	// 8262F220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F22C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F230: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F234: 392B482C  addi r9, r11, 0x482c
	ctx.r[9].s64 = ctx.r[11].s64 + 18476;
	// 8262F238: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F23C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F240: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262F244: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8262F248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F24C: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 8262F250: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F254: 396B1A98  addi r11, r11, 0x1a98
	ctx.r[11].s64 = ctx.r[11].s64 + 6808;
	// 8262F258: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262F25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F260: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262F264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F268: 386A6184  addi r3, r10, 0x6184
	ctx.r[3].s64 = ctx.r[10].s64 + 24964;
	// 8262F26C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F270: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262F274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F278: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262F27C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F280: 4BE37BA1  bl 0x82466e20
	ctx.lr = 0x8262F284;
	sub_82466E20(ctx, base);
	// 8262F284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F298 size=116
    let mut pc: u32 = 0x8262F298;
    'dispatch: loop {
        match pc {
            0x8262F298 => {
    //   block [0x8262F298..0x8262F30C)
	// 8262F298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F2A4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F2A8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8262F2AC: 390ACA14  addi r8, r10, -0x35ec
	ctx.r[8].s64 = ctx.r[10].s64 + -13804;
	// 8262F2B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F2B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F2B8: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F2BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F2C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F2CC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8262F2D0: 396B489C  addi r11, r11, 0x489c
	ctx.r[11].s64 = ctx.r[11].s64 + 18588;
	// 8262F2D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F2D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F2DC: 386A61B4  addi r3, r10, 0x61b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25012;
	// 8262F2E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F2E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F2E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F2F8: 4BE37B29  bl 0x82466e20
	ctx.lr = 0x8262F2FC;
	sub_82466E20(ctx, base);
	// 8262F2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F310 size=24
    let mut pc: u32 = 0x8262F310;
    'dispatch: loop {
        match pc {
            0x8262F310 => {
    //   block [0x8262F310..0x8262F328)
	// 8262F310: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F314: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F318: 394A1C30  addi r10, r10, 0x1c30
	ctx.r[10].s64 = ctx.r[10].s64 + 7216;
	// 8262F31C: 816BCA44  lwz r11, -0x35bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13756 as u32) ) } as u64;
	// 8262F320: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8262F324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F328 size=116
    let mut pc: u32 = 0x8262F328;
    'dispatch: loop {
        match pc {
            0x8262F328 => {
    //   block [0x8262F328..0x8262F39C)
	// 8262F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F334: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F338: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F33C: 392B48C0  addi r9, r11, 0x48c0
	ctx.r[9].s64 = ctx.r[11].s64 + 18624;
	// 8262F340: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F344: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F348: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8262F34C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8262F350: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F354: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8262F358: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F35C: 396B1C30  addi r11, r11, 0x1c30
	ctx.r[11].s64 = ctx.r[11].s64 + 7216;
	// 8262F360: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262F364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F368: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262F36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F370: 386A61E4  addi r3, r10, 0x61e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25060;
	// 8262F374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F378: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262F37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F380: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262F384: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F388: 4BE37A99  bl 0x82466e20
	ctx.lr = 0x8262F38C;
	sub_82466E20(ctx, base);
	// 8262F38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F3A0 size=108
    let mut pc: u32 = 0x8262F3A0;
    'dispatch: loop {
        match pc {
            0x8262F3A0 => {
    //   block [0x8262F3A0..0x8262F40C)
	// 8262F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F3AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F3B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F3B4: 38EBCA48  addi r7, r11, -0x35b8
	ctx.r[7].s64 = ctx.r[11].s64 + -13752;
	// 8262F3B8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8262F3BC: 388A59F4  addi r4, r10, 0x59f4
	ctx.r[4].s64 = ctx.r[10].s64 + 23028;
	// 8262F3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F3C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F3C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F3D0: 386A6214  addi r3, r10, 0x6214
	ctx.r[3].s64 = ctx.r[10].s64 + 25108;
	// 8262F3D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F3F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F3F8: 4BE37A29  bl 0x82466e20
	ctx.lr = 0x8262F3FC;
	sub_82466E20(ctx, base);
	// 8262F3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F410 size=116
    let mut pc: u32 = 0x8262F410;
    'dispatch: loop {
        match pc {
            0x8262F410 => {
    //   block [0x8262F410..0x8262F484)
	// 8262F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F41C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F420: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 8262F424: 390ACB68  addi r8, r10, -0x3498
	ctx.r[8].s64 = ctx.r[10].s64 + -13464;
	// 8262F428: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F42C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F430: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F434: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F438: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F444: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8262F448: 396B4940  addi r11, r11, 0x4940
	ctx.r[11].s64 = ctx.r[11].s64 + 18752;
	// 8262F44C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F450: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F454: 386A6244  addi r3, r10, 0x6244
	ctx.r[3].s64 = ctx.r[10].s64 + 25156;
	// 8262F458: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F45C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F460: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F470: 4BE379B1  bl 0x82466e20
	ctx.lr = 0x8262F474;
	sub_82466E20(ctx, base);
	// 8262F474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F488 size=112
    let mut pc: u32 = 0x8262F488;
    'dispatch: loop {
        match pc {
            0x8262F488 => {
    //   block [0x8262F488..0x8262F4F8)
	// 8262F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F498: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F49C: 38AA60F4  addi r5, r10, 0x60f4
	ctx.r[5].s64 = ctx.r[10].s64 + 24820;
	// 8262F4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F4A4: 390BCCB8  addi r8, r11, -0x3348
	ctx.r[8].s64 = ctx.r[11].s64 + -13128;
	// 8262F4A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262F4AC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8262F4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F4B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F4C0: 386A6274  addi r3, r10, 0x6274
	ctx.r[3].s64 = ctx.r[10].s64 + 25204;
	// 8262F4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F4E4: 4BE3793D  bl 0x82466e20
	ctx.lr = 0x8262F4E8;
	sub_82466E20(ctx, base);
	// 8262F4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F4F8 size=24
    let mut pc: u32 = 0x8262F4F8;
    'dispatch: loop {
        match pc {
            0x8262F4F8 => {
    //   block [0x8262F4F8..0x8262F510)
	// 8262F4F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F4FC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F500: 394A1E40  addi r10, r10, 0x1e40
	ctx.r[10].s64 = ctx.r[10].s64 + 7744;
	// 8262F504: 816BCD30  lwz r11, -0x32d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13008 as u32) ) } as u64;
	// 8262F508: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262F50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F510 size=116
    let mut pc: u32 = 0x8262F510;
    'dispatch: loop {
        match pc {
            0x8262F510 => {
    //   block [0x8262F510..0x8262F584)
	// 8262F510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F51C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F520: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F524: 390B1E40  addi r8, r11, 0x1e40
	ctx.r[8].s64 = ctx.r[11].s64 + 7744;
	// 8262F528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F52C: 392A4998  addi r9, r10, 0x4998
	ctx.r[9].s64 = ctx.r[10].s64 + 18840;
	// 8262F530: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F534: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262F538: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F53C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F544: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F554: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262F558: 388A5A2C  addi r4, r10, 0x5a2c
	ctx.r[4].s64 = ctx.r[10].s64 + 23084;
	// 8262F55C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F560: 386B62A4  addi r3, r11, 0x62a4
	ctx.r[3].s64 = ctx.r[11].s64 + 25252;
	// 8262F564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F568: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F570: 4BE378B1  bl 0x82466e20
	ctx.lr = 0x8262F574;
	sub_82466E20(ctx, base);
	// 8262F574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F588 size=112
    let mut pc: u32 = 0x8262F588;
    'dispatch: loop {
        match pc {
            0x8262F588 => {
    //   block [0x8262F588..0x8262F5F8)
	// 8262F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F598: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F59C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F5A4: 390BCD34  addi r8, r11, -0x32cc
	ctx.r[8].s64 = ctx.r[11].s64 + -13004;
	// 8262F5A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262F5AC: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8262F5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F5C0: 386A62D4  addi r3, r10, 0x62d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25300;
	// 8262F5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F5E4: 4BE3783D  bl 0x82466e20
	ctx.lr = 0x8262F5E8;
	sub_82466E20(ctx, base);
	// 8262F5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F5F8 size=108
    let mut pc: u32 = 0x8262F5F8;
    'dispatch: loop {
        match pc {
            0x8262F5F8 => {
    //   block [0x8262F5F8..0x8262F664)
	// 8262F5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F604: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F60C: 38EBCD50  addi r7, r11, -0x32b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12976;
	// 8262F610: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F614: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8262F618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F61C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F628: 386A6304  addi r3, r10, 0x6304
	ctx.r[3].s64 = ctx.r[10].s64 + 25348;
	// 8262F62C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F650: 4BE377D1  bl 0x82466e20
	ctx.lr = 0x8262F654;
	sub_82466E20(ctx, base);
	// 8262F654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F668 size=108
    let mut pc: u32 = 0x8262F668;
    'dispatch: loop {
        match pc {
            0x8262F668 => {
    //   block [0x8262F668..0x8262F6D4)
	// 8262F668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F674: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F67C: 38EBCD98  addi r7, r11, -0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + -12904;
	// 8262F680: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F684: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8262F688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F68C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F698: 386A6334  addi r3, r10, 0x6334
	ctx.r[3].s64 = ctx.r[10].s64 + 25396;
	// 8262F69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F6C0: 4BE37761  bl 0x82466e20
	ctx.lr = 0x8262F6C4;
	sub_82466E20(ctx, base);
	// 8262F6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F6D8 size=116
    let mut pc: u32 = 0x8262F6D8;
    'dispatch: loop {
        match pc {
            0x8262F6D8 => {
    //   block [0x8262F6D8..0x8262F74C)
	// 8262F6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F6E4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F6E8: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 8262F6EC: 390ACDE0  addi r8, r10, -0x3220
	ctx.r[8].s64 = ctx.r[10].s64 + -12832;
	// 8262F6F0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F6F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F6F8: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262F6FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F700: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F70C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8262F710: 396B49B0  addi r11, r11, 0x49b0
	ctx.r[11].s64 = ctx.r[11].s64 + 18864;
	// 8262F714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F718: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F71C: 386A6364  addi r3, r10, 0x6364
	ctx.r[3].s64 = ctx.r[10].s64 + 25444;
	// 8262F720: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F728: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F738: 4BE376E9  bl 0x82466e20
	ctx.lr = 0x8262F73C;
	sub_82466E20(ctx, base);
	// 8262F73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F750 size=104
    let mut pc: u32 = 0x8262F750;
    'dispatch: loop {
        match pc {
            0x8262F750 => {
    //   block [0x8262F750..0x8262F7B8)
	// 8262F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F75C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F764: 392A4A0C  addi r9, r10, 0x4a0c
	ctx.r[9].s64 = ctx.r[10].s64 + 18956;
	// 8262F768: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F770: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F774: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F784: 388A5A40  addi r4, r10, 0x5a40
	ctx.r[4].s64 = ctx.r[10].s64 + 23104;
	// 8262F788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F78C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262F794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F79C: 386A6394  addi r3, r10, 0x6394
	ctx.r[3].s64 = ctx.r[10].s64 + 25492;
	// 8262F7A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F7A4: 4BE3767D  bl 0x82466e20
	ctx.lr = 0x8262F7A8;
	sub_82466E20(ctx, base);
	// 8262F7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F7B8 size=108
    let mut pc: u32 = 0x8262F7B8;
    'dispatch: loop {
        match pc {
            0x8262F7B8 => {
    //   block [0x8262F7B8..0x8262F824)
	// 8262F7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F7C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F7C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F7CC: 38EBCF30  addi r7, r11, -0x30d0
	ctx.r[7].s64 = ctx.r[11].s64 + -12496;
	// 8262F7D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F7D4: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 8262F7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F7DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F7E8: 386A63C4  addi r3, r10, 0x63c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25540;
	// 8262F7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F810: 4BE37611  bl 0x82466e20
	ctx.lr = 0x8262F814;
	sub_82466E20(ctx, base);
	// 8262F814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F828 size=24
    let mut pc: u32 = 0x8262F828;
    'dispatch: loop {
        match pc {
            0x8262F828 => {
    //   block [0x8262F828..0x8262F840)
	// 8262F828: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F82C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F830: 394A1ED0  addi r10, r10, 0x1ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 7888;
	// 8262F834: 816BCF78  lwz r11, -0x3088(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12424 as u32) ) } as u64;
	// 8262F838: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262F83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F840 size=116
    let mut pc: u32 = 0x8262F840;
    'dispatch: loop {
        match pc {
            0x8262F840 => {
    //   block [0x8262F840..0x8262F8B4)
	// 8262F840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F84C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F854: 390B1ED0  addi r8, r11, 0x1ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 7888;
	// 8262F858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F85C: 392A4A48  addi r9, r10, 0x4a48
	ctx.r[9].s64 = ctx.r[10].s64 + 19016;
	// 8262F860: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F864: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262F868: 38AA6394  addi r5, r10, 0x6394
	ctx.r[5].s64 = ctx.r[10].s64 + 25492;
	// 8262F86C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F874: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F884: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262F888: 388A5A5C  addi r4, r10, 0x5a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 23132;
	// 8262F88C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F890: 386B63F4  addi r3, r11, 0x63f4
	ctx.r[3].s64 = ctx.r[11].s64 + 25588;
	// 8262F894: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F898: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F8A0: 4BE37581  bl 0x82466e20
	ctx.lr = 0x8262F8A4;
	sub_82466E20(ctx, base);
	// 8262F8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F8B8 size=108
    let mut pc: u32 = 0x8262F8B8;
    'dispatch: loop {
        match pc {
            0x8262F8B8 => {
    //   block [0x8262F8B8..0x8262F924)
	// 8262F8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F8C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F8CC: 38EBCF80  addi r7, r11, -0x3080
	ctx.r[7].s64 = ctx.r[11].s64 + -12416;
	// 8262F8D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F8D4: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8262F8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F8DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F8E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F8E8: 386A6424  addi r3, r10, 0x6424
	ctx.r[3].s64 = ctx.r[10].s64 + 25636;
	// 8262F8EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F910: 4BE37511  bl 0x82466e20
	ctx.lr = 0x8262F914;
	sub_82466E20(ctx, base);
	// 8262F914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F928 size=108
    let mut pc: u32 = 0x8262F928;
    'dispatch: loop {
        match pc {
            0x8262F928 => {
    //   block [0x8262F928..0x8262F994)
	// 8262F928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F934: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F93C: 38EBCFC8  addi r7, r11, -0x3038
	ctx.r[7].s64 = ctx.r[11].s64 + -12344;
	// 8262F940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262F944: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8262F948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F94C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F958: 386A6454  addi r3, r10, 0x6454
	ctx.r[3].s64 = ctx.r[10].s64 + 25684;
	// 8262F95C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F97C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F980: 4BE374A1  bl 0x82466e20
	ctx.lr = 0x8262F984;
	sub_82466E20(ctx, base);
	// 8262F984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F998 size=108
    let mut pc: u32 = 0x8262F998;
    'dispatch: loop {
        match pc {
            0x8262F998 => {
    //   block [0x8262F998..0x8262FA04)
	// 8262F998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F9AC: 38EBCFF8  addi r7, r11, -0x3008
	ctx.r[7].s64 = ctx.r[11].s64 + -12296;
	// 8262F9B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262F9B4: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8262F9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F9C8: 386A6484  addi r3, r10, 0x6484
	ctx.r[3].s64 = ctx.r[10].s64 + 25732;
	// 8262F9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F9F0: 4BE37431  bl 0x82466e20
	ctx.lr = 0x8262F9F4;
	sub_82466E20(ctx, base);
	// 8262F9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FA08 size=96
    let mut pc: u32 = 0x8262FA08;
    'dispatch: loop {
        match pc {
            0x8262FA08 => {
    //   block [0x8262FA08..0x8262FA68)
	// 8262FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FA14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FA1C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8262FA20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FA28: 386A64B4  addi r3, r10, 0x64b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25780;
	// 8262FA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FA34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FA48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262FA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FA50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262FA54: 4BE373CD  bl 0x82466e20
	ctx.lr = 0x8262FA58;
	sub_82466E20(ctx, base);
	// 8262FA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FA68 size=112
    let mut pc: u32 = 0x8262FA68;
    'dispatch: loop {
        match pc {
            0x8262FA68 => {
    //   block [0x8262FA68..0x8262FAD8)
	// 8262FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FA7C: 38AA64B4  addi r5, r10, 0x64b4
	ctx.r[5].s64 = ctx.r[10].s64 + 25780;
	// 8262FA80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FA84: 390BD010  addi r8, r11, -0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -12272;
	// 8262FA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262FA8C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8262FA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FA94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FAA0: 386A64E4  addi r3, r10, 0x64e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25828;
	// 8262FAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262FAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FAC4: 4BE3735D  bl 0x82466e20
	ctx.lr = 0x8262FAC8;
	sub_82466E20(ctx, base);
	// 8262FAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FAD8 size=112
    let mut pc: u32 = 0x8262FAD8;
    'dispatch: loop {
        match pc {
            0x8262FAD8 => {
    //   block [0x8262FAD8..0x8262FB48)
	// 8262FAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FAE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FAE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FAEC: 392A4A64  addi r9, r10, 0x4a64
	ctx.r[9].s64 = ctx.r[10].s64 + 19044;
	// 8262FAF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FAF4: 390BD040  addi r8, r11, -0x2fc0
	ctx.r[8].s64 = ctx.r[11].s64 + -12224;
	// 8262FAF8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262FAFC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8262FB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FB04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FB10: 386A6514  addi r3, r10, 0x6514
	ctx.r[3].s64 = ctx.r[10].s64 + 25876;
	// 8262FB14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FB18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FB34: 4BE372ED  bl 0x82466e20
	ctx.lr = 0x8262FB38;
	sub_82466E20(ctx, base);
	// 8262FB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FB48 size=108
    let mut pc: u32 = 0x8262FB48;
    'dispatch: loop {
        match pc {
            0x8262FB48 => {
    //   block [0x8262FB48..0x8262FBB4)
	// 8262FB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FB54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FB58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FB5C: 38EBD0E8  addi r7, r11, -0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + -12056;
	// 8262FB60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FB64: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8262FB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FB6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FB70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FB78: 386A6544  addi r3, r10, 0x6544
	ctx.r[3].s64 = ctx.r[10].s64 + 25924;
	// 8262FB7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FBA0: 4BE37281  bl 0x82466e20
	ctx.lr = 0x8262FBA4;
	sub_82466E20(ctx, base);
	// 8262FBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FBB8 size=108
    let mut pc: u32 = 0x8262FBB8;
    'dispatch: loop {
        match pc {
            0x8262FBB8 => {
    //   block [0x8262FBB8..0x8262FC24)
	// 8262FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FBC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FBC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FBCC: 38EBD118  addi r7, r11, -0x2ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -12008;
	// 8262FBD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FBD4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8262FBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FBDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FBE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FBE8: 386A6574  addi r3, r10, 0x6574
	ctx.r[3].s64 = ctx.r[10].s64 + 25972;
	// 8262FBEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FC0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FC10: 4BE37211  bl 0x82466e20
	ctx.lr = 0x8262FC14;
	sub_82466E20(ctx, base);
	// 8262FC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262FC28 size=28
    let mut pc: u32 = 0x8262FC28;
    'dispatch: loop {
        match pc {
            0x8262FC28 => {
    //   block [0x8262FC28..0x8262FC44)
	// 8262FC28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FC2C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262FC30: 394A1F60  addi r10, r10, 0x1f60
	ctx.r[10].s64 = ctx.r[10].s64 + 8032;
	// 8262FC34: 816BD148  lwz r11, -0x2eb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11960 as u32) ) } as u64;
	// 8262FC38: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262FC3C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FC48 size=112
    let mut pc: u32 = 0x8262FC48;
    'dispatch: loop {
        match pc {
            0x8262FC48 => {
    //   block [0x8262FC48..0x8262FCB8)
	// 8262FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FC54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FC58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FC5C: 392A4BD8  addi r9, r10, 0x4bd8
	ctx.r[9].s64 = ctx.r[10].s64 + 19416;
	// 8262FC60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FC64: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 8262FC68: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262FC6C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8262FC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FC80: 386A65A4  addi r3, r10, 0x65a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26020;
	// 8262FC84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FC88: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8262FC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FCA4: 4BE3717D  bl 0x82466e20
	ctx.lr = 0x8262FCA8;
	sub_82466E20(ctx, base);
	// 8262FCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FCB8 size=108
    let mut pc: u32 = 0x8262FCB8;
    'dispatch: loop {
        match pc {
            0x8262FCB8 => {
    //   block [0x8262FCB8..0x8262FD24)
	// 8262FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FCC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FCC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FCCC: 38EBD154  addi r7, r11, -0x2eac
	ctx.r[7].s64 = ctx.r[11].s64 + -11948;
	// 8262FCD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FCD4: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8262FCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FCDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FCE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FCE8: 386A65D4  addi r3, r10, 0x65d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26068;
	// 8262FCEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FD10: 4BE37111  bl 0x82466e20
	ctx.lr = 0x8262FD14;
	sub_82466E20(ctx, base);
	// 8262FD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FD28 size=108
    let mut pc: u32 = 0x8262FD28;
    'dispatch: loop {
        match pc {
            0x8262FD28 => {
    //   block [0x8262FD28..0x8262FD94)
	// 8262FD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FD34: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FD38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FD3C: 38EBD184  addi r7, r11, -0x2e7c
	ctx.r[7].s64 = ctx.r[11].s64 + -11900;
	// 8262FD40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FD44: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8262FD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FD4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FD50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FD58: 386A6604  addi r3, r10, 0x6604
	ctx.r[3].s64 = ctx.r[10].s64 + 26116;
	// 8262FD5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FD80: 4BE370A1  bl 0x82466e20
	ctx.lr = 0x8262FD84;
	sub_82466E20(ctx, base);
	// 8262FD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262FD98 size=24
    let mut pc: u32 = 0x8262FD98;
    'dispatch: loop {
        match pc {
            0x8262FD98 => {
    //   block [0x8262FD98..0x8262FDB0)
	// 8262FD98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FD9C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262FDA0: 394A2020  addi r10, r10, 0x2020
	ctx.r[10].s64 = ctx.r[10].s64 + 8224;
	// 8262FDA4: 816BD19C  lwz r11, -0x2e64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11876 as u32) ) } as u64;
	// 8262FDA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262FDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


