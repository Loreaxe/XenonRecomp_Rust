pub fn sub_826F74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F74E0 size=116
    let mut pc: u32 = 0x826F74E0;
    'dispatch: loop {
        match pc {
            0x826F74E0 => {
    //   block [0x826F74E0..0x826F7554)
	// 826F74E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F74E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F74E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F74EC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F74F0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F74F4: 390B25A0  addi r8, r11, 0x25a0
	ctx.r[8].s64 = ctx.r[11].s64 + 9632;
	// 826F74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F74FC: 392AA850  addi r9, r10, -0x57b0
	ctx.r[9].s64 = ctx.r[10].s64 + -22448;
	// 826F7500: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7504: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F7508: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F750C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7514: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F751C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7524: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F7528: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826F752C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7530: 386BA654  addi r3, r11, -0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + -22956;
	// 826F7534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7540: 4BD6F8E1  bl 0x82466e20
	ctx.lr = 0x826F7544;
	sub_82466E20(ctx, base);
	// 826F7544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F754C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7558 size=100
    let mut pc: u32 = 0x826F7558;
    'dispatch: loop {
        match pc {
            0x826F7558 => {
    //   block [0x826F7558..0x826F75BC)
	// 826F7558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F755C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7564: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F756C: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7578: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826F757C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F758C: 386AA684  addi r3, r10, -0x597c
	ctx.r[3].s64 = ctx.r[10].s64 + -22908;
	// 826F7590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F75A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F75A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F75A8: 4BD6F879  bl 0x82466e20
	ctx.lr = 0x826F75AC;
	sub_82466E20(ctx, base);
	// 826F75AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F75B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F75B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F75B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F75C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F75C0 size=100
    let mut pc: u32 = 0x826F75C0;
    'dispatch: loop {
        match pc {
            0x826F75C0 => {
    //   block [0x826F75C0..0x826F7624)
	// 826F75C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F75C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F75C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F75CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F75D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F75D4: 38AAA6E4  addi r5, r10, -0x591c
	ctx.r[5].s64 = ctx.r[10].s64 + -22812;
	// 826F75D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F75DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F75E0: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826F75E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F75E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F75EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F75F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F75F4: 386AA6B4  addi r3, r10, -0x594c
	ctx.r[3].s64 = ctx.r[10].s64 + -22860;
	// 826F75F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F75FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F760C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7610: 4BD6F811  bl 0x82466e20
	ctx.lr = 0x826F7614;
	sub_82466E20(ctx, base);
	// 826F7614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7628 size=112
    let mut pc: u32 = 0x826F7628;
    'dispatch: loop {
        match pc {
            0x826F7628 => {
    //   block [0x826F7628..0x826F7698)
	// 826F7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7634: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7638: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F763C: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7644: 390BF308  addi r8, r11, -0xcf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3320;
	// 826F7648: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F764C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826F7650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7654: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F765C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7660: 386AA6E4  addi r3, r10, -0x591c
	ctx.r[3].s64 = ctx.r[10].s64 + -22812;
	// 826F7664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7684: 4BD6F79D  bl 0x82466e20
	ctx.lr = 0x826F7688;
	sub_82466E20(ctx, base);
	// 826F7688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F768C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7698 size=100
    let mut pc: u32 = 0x826F7698;
    'dispatch: loop {
        match pc {
            0x826F7698 => {
    //   block [0x826F7698..0x826F76FC)
	// 826F7698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F76A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F76A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F76A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F76AC: 38AAA6E4  addi r5, r10, -0x591c
	ctx.r[5].s64 = ctx.r[10].s64 + -22812;
	// 826F76B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F76B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F76B8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826F76BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F76C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F76C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F76C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F76CC: 386AA714  addi r3, r10, -0x58ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22764;
	// 826F76D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F76D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F76D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F76DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F76E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F76E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F76E8: 4BD6F739  bl 0x82466e20
	ctx.lr = 0x826F76EC;
	sub_82466E20(ctx, base);
	// 826F76EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F76F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F76F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F76F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7700 size=100
    let mut pc: u32 = 0x826F7700;
    'dispatch: loop {
        match pc {
            0x826F7700 => {
    //   block [0x826F7700..0x826F7764)
	// 826F7700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F770C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7714: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F771C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7720: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826F7724: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F772C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7734: 386AA744  addi r3, r10, -0x58bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22716;
	// 826F7738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F773C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F774C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7750: 4BD6F6D1  bl 0x82466e20
	ctx.lr = 0x826F7754;
	sub_82466E20(ctx, base);
	// 826F7754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F775C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7768 size=100
    let mut pc: u32 = 0x826F7768;
    'dispatch: loop {
        match pc {
            0x826F7768 => {
    //   block [0x826F7768..0x826F77CC)
	// 826F7768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F776C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F777C: 38AAA684  addi r5, r10, -0x597c
	ctx.r[5].s64 = ctx.r[10].s64 + -22908;
	// 826F7780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7788: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826F778C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F779C: 386AA774  addi r3, r10, -0x588c
	ctx.r[3].s64 = ctx.r[10].s64 + -22668;
	// 826F77A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F77A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F77A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F77AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F77B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F77B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F77B8: 4BD6F669  bl 0x82466e20
	ctx.lr = 0x826F77BC;
	sub_82466E20(ctx, base);
	// 826F77BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F77C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F77C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F77C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F77D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F77D0 size=100
    let mut pc: u32 = 0x826F77D0;
    'dispatch: loop {
        match pc {
            0x826F77D0 => {
    //   block [0x826F77D0..0x826F7834)
	// 826F77D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F77D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F77D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F77DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F77E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F77E4: 38AAA744  addi r5, r10, -0x58bc
	ctx.r[5].s64 = ctx.r[10].s64 + -22716;
	// 826F77E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F77EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F77F0: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826F77F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F77FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7804: 386AA7A4  addi r3, r10, -0x585c
	ctx.r[3].s64 = ctx.r[10].s64 + -22620;
	// 826F7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F780C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F781C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7820: 4BD6F601  bl 0x82466e20
	ctx.lr = 0x826F7824;
	sub_82466E20(ctx, base);
	// 826F7824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F782C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7838 size=100
    let mut pc: u32 = 0x826F7838;
    'dispatch: loop {
        match pc {
            0x826F7838 => {
    //   block [0x826F7838..0x826F789C)
	// 826F7838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7844: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F784C: 38AAA684  addi r5, r10, -0x597c
	ctx.r[5].s64 = ctx.r[10].s64 + -22908;
	// 826F7850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7858: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826F785C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F786C: 386AA7D4  addi r3, r10, -0x582c
	ctx.r[3].s64 = ctx.r[10].s64 + -22572;
	// 826F7870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F787C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7888: 4BD6F599  bl 0x82466e20
	ctx.lr = 0x826F788C;
	sub_82466E20(ctx, base);
	// 826F788C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F78A0 size=112
    let mut pc: u32 = 0x826F78A0;
    'dispatch: loop {
        match pc {
            0x826F78A0 => {
    //   block [0x826F78A0..0x826F7910)
	// 826F78A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F78A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F78A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F78AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F78B0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F78B4: 38AAA864  addi r5, r10, -0x579c
	ctx.r[5].s64 = ctx.r[10].s64 + -22428;
	// 826F78B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F78BC: 390BF338  addi r8, r11, -0xcc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3272;
	// 826F78C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F78C4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826F78C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F78CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F78D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F78D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F78D8: 386AA804  addi r3, r10, -0x57fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22524;
	// 826F78DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F78E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F78E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F78E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F78EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F78F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F78F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F78F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F78FC: 4BD6F525  bl 0x82466e20
	ctx.lr = 0x826F7900;
	sub_82466E20(ctx, base);
	// 826F7900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F790C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7910 size=112
    let mut pc: u32 = 0x826F7910;
    'dispatch: loop {
        match pc {
            0x826F7910 => {
    //   block [0x826F7910..0x826F7980)
	// 826F7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F791C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7920: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7924: 38AAA894  addi r5, r10, -0x576c
	ctx.r[5].s64 = ctx.r[10].s64 + -22380;
	// 826F7928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F792C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 826F7930: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7934: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826F7938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F793C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7948: 386AA834  addi r3, r10, -0x57cc
	ctx.r[3].s64 = ctx.r[10].s64 + -22476;
	// 826F794C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F795C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F796C: 4BD6F4B5  bl 0x82466e20
	ctx.lr = 0x826F7970;
	sub_82466E20(ctx, base);
	// 826F7970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F797C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7980 size=112
    let mut pc: u32 = 0x826F7980;
    'dispatch: loop {
        match pc {
            0x826F7980 => {
    //   block [0x826F7980..0x826F79F0)
	// 826F7980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F798C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7994: 38AAA954  addi r5, r10, -0x56ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22188;
	// 826F7998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F799C: 390BF380  addi r8, r11, -0xc80
	ctx.r[8].s64 = ctx.r[11].s64 + -3200;
	// 826F79A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F79A4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826F79A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F79AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F79B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F79B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F79B8: 386AA864  addi r3, r10, -0x579c
	ctx.r[3].s64 = ctx.r[10].s64 + -22428;
	// 826F79BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F79C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F79C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F79C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F79CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F79D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F79D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F79D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F79DC: 4BD6F445  bl 0x82466e20
	ctx.lr = 0x826F79E0;
	sub_82466E20(ctx, base);
	// 826F79E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F79E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F79E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F79EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F79F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F79F0 size=112
    let mut pc: u32 = 0x826F79F0;
    'dispatch: loop {
        match pc {
            0x826F79F0 => {
    //   block [0x826F79F0..0x826F7A60)
	// 826F79F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F79F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F79F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F79FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A00: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7A04: 38AAA864  addi r5, r10, -0x579c
	ctx.r[5].s64 = ctx.r[10].s64 + -22428;
	// 826F7A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7A0C: 390BF3B0  addi r8, r11, -0xc50
	ctx.r[8].s64 = ctx.r[11].s64 + -3152;
	// 826F7A10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7A14: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826F7A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7A1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7A28: 386AA894  addi r3, r10, -0x576c
	ctx.r[3].s64 = ctx.r[10].s64 + -22380;
	// 826F7A2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7A4C: 4BD6F3D5  bl 0x82466e20
	ctx.lr = 0x826F7A50;
	sub_82466E20(ctx, base);
	// 826F7A50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7A60 size=112
    let mut pc: u32 = 0x826F7A60;
    'dispatch: loop {
        match pc {
            0x826F7A60 => {
    //   block [0x826F7A60..0x826F7AD0)
	// 826F7A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7A6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7A74: 38AAA894  addi r5, r10, -0x576c
	ctx.r[5].s64 = ctx.r[10].s64 + -22380;
	// 826F7A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7A7C: 390BF3C8  addi r8, r11, -0xc38
	ctx.r[8].s64 = ctx.r[11].s64 + -3128;
	// 826F7A80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7A84: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826F7A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7A8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7A98: 386AA8C4  addi r3, r10, -0x573c
	ctx.r[3].s64 = ctx.r[10].s64 + -22332;
	// 826F7A9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7ABC: 4BD6F365  bl 0x82466e20
	ctx.lr = 0x826F7AC0;
	sub_82466E20(ctx, base);
	// 826F7AC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7AD0 size=116
    let mut pc: u32 = 0x826F7AD0;
    'dispatch: loop {
        match pc {
            0x826F7AD0 => {
    //   block [0x826F7AD0..0x826F7B44)
	// 826F7AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7ADC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7AE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F7AE4: 390AF3E0  addi r8, r10, -0xc20
	ctx.r[8].s64 = ctx.r[10].s64 + -3104;
	// 826F7AE8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7AEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F7AF0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7AF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7AF8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7B04: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826F7B08: 396BA864  addi r11, r11, -0x579c
	ctx.r[11].s64 = ctx.r[11].s64 + -22428;
	// 826F7B0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7B10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7B14: 386AA8F4  addi r3, r10, -0x570c
	ctx.r[3].s64 = ctx.r[10].s64 + -22284;
	// 826F7B18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F7B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7B20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F7B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7B30: 4BD6F2F1  bl 0x82466e20
	ctx.lr = 0x826F7B34;
	sub_82466E20(ctx, base);
	// 826F7B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F7B48 size=48
    let mut pc: u32 = 0x826F7B48;
    'dispatch: loop {
        match pc {
            0x826F7B48 => {
    //   block [0x826F7B48..0x826F7B78)
	// 826F7B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7B4C: 814BF494  lwz r10, -0xb6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2924 as u32) ) } as u64;
	// 826F7B50: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7B54: 396B2660  addi r11, r11, 0x2660
	ctx.r[11].s64 = ctx.r[11].s64 + 9824;
	// 826F7B58: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826F7B5C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7B60: 814AF490  lwz r10, -0xb70(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2928 as u32) ) } as u64;
	// 826F7B64: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826F7B68: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7B6C: 814AF48C  lwz r10, -0xb74(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2932 as u32) ) } as u64;
	// 826F7B70: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 826F7B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7B78 size=116
    let mut pc: u32 = 0x826F7B78;
    'dispatch: loop {
        match pc {
            0x826F7B78 => {
    //   block [0x826F7B78..0x826F7BEC)
	// 826F7B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7B84: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F7B88: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7B8C: 392BA938  addi r9, r11, -0x56c8
	ctx.r[9].s64 = ctx.r[11].s64 + -22216;
	// 826F7B90: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7B94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7B98: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826F7B9C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 826F7BA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7BA4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826F7BA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7BAC: 396B2660  addi r11, r11, 0x2660
	ctx.r[11].s64 = ctx.r[11].s64 + 9824;
	// 826F7BB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F7BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7BB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F7BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7BC0: 386AA924  addi r3, r10, -0x56dc
	ctx.r[3].s64 = ctx.r[10].s64 + -22236;
	// 826F7BC4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826F7BC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F7BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7BD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F7BD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7BD8: 4BD6F249  bl 0x82466e20
	ctx.lr = 0x826F7BDC;
	sub_82466E20(ctx, base);
	// 826F7BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7BF0 size=116
    let mut pc: u32 = 0x826F7BF0;
    'dispatch: loop {
        match pc {
            0x826F7BF0 => {
    //   block [0x826F7BF0..0x826F7C64)
	// 826F7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7BFC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7C00: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7C04: 390BF4A0  addi r8, r11, -0xb60
	ctx.r[8].s64 = ctx.r[11].s64 + -2912;
	// 826F7C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7C0C: 392AAAD8  addi r9, r10, -0x5528
	ctx.r[9].s64 = ctx.r[10].s64 + -21800;
	// 826F7C10: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7C14: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F7C18: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7C1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7C24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7C34: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F7C38: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826F7C3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7C40: 386BA954  addi r3, r11, -0x56ac
	ctx.r[3].s64 = ctx.r[11].s64 + -22188;
	// 826F7C44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7C48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7C50: 4BD6F1D1  bl 0x82466e20
	ctx.lr = 0x826F7C54;
	sub_82466E20(ctx, base);
	// 826F7C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7C68 size=112
    let mut pc: u32 = 0x826F7C68;
    'dispatch: loop {
        match pc {
            0x826F7C68 => {
    //   block [0x826F7C68..0x826F7CD8)
	// 826F7C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7C74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7C78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7C7C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7C84: 390BF530  addi r8, r11, -0xad0
	ctx.r[8].s64 = ctx.r[11].s64 + -2768;
	// 826F7C88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7C8C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826F7C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7CA0: 386AA984  addi r3, r10, -0x567c
	ctx.r[3].s64 = ctx.r[10].s64 + -22140;
	// 826F7CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7CC4: 4BD6F15D  bl 0x82466e20
	ctx.lr = 0x826F7CC8;
	sub_82466E20(ctx, base);
	// 826F7CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7CD8 size=108
    let mut pc: u32 = 0x826F7CD8;
    'dispatch: loop {
        match pc {
            0x826F7CD8 => {
    //   block [0x826F7CD8..0x826F7D44)
	// 826F7CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7CE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F7CEC: 38EBF548  addi r7, r11, -0xab8
	ctx.r[7].s64 = ctx.r[11].s64 + -2744;
	// 826F7CF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F7CF4: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 826F7CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7D08: 386AA9B4  addi r3, r10, -0x564c
	ctx.r[3].s64 = ctx.r[10].s64 + -22092;
	// 826F7D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7D30: 4BD6F0F1  bl 0x82466e20
	ctx.lr = 0x826F7D34;
	sub_82466E20(ctx, base);
	// 826F7D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7D48 size=112
    let mut pc: u32 = 0x826F7D48;
    'dispatch: loop {
        match pc {
            0x826F7D48 => {
    //   block [0x826F7D48..0x826F7DB8)
	// 826F7D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7D5C: 38AA8914  addi r5, r10, -0x76ec
	ctx.r[5].s64 = ctx.r[10].s64 + -30444;
	// 826F7D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7D64: 390BF5C0  addi r8, r11, -0xa40
	ctx.r[8].s64 = ctx.r[11].s64 + -2624;
	// 826F7D68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7D6C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826F7D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7D80: 386AA9E4  addi r3, r10, -0x561c
	ctx.r[3].s64 = ctx.r[10].s64 + -22044;
	// 826F7D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7DA4: 4BD6F07D  bl 0x82466e20
	ctx.lr = 0x826F7DA8;
	sub_82466E20(ctx, base);
	// 826F7DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7DB8 size=108
    let mut pc: u32 = 0x826F7DB8;
    'dispatch: loop {
        match pc {
            0x826F7DB8 => {
    //   block [0x826F7DB8..0x826F7E24)
	// 826F7DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7DC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7DCC: 38EBF5D8  addi r7, r11, -0xa28
	ctx.r[7].s64 = ctx.r[11].s64 + -2600;
	// 826F7DD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F7DD4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826F7DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7DDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7DE8: 386AAA14  addi r3, r10, -0x55ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21996;
	// 826F7DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7E10: 4BD6F011  bl 0x82466e20
	ctx.lr = 0x826F7E14;
	sub_82466E20(ctx, base);
	// 826F7E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7E28 size=112
    let mut pc: u32 = 0x826F7E28;
    'dispatch: loop {
        match pc {
            0x826F7E28 => {
    //   block [0x826F7E28..0x826F7E98)
	// 826F7E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7E34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7E38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7E3C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7E44: 390BF5F0  addi r8, r11, -0xa10
	ctx.r[8].s64 = ctx.r[11].s64 + -2576;
	// 826F7E48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F7E4C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826F7E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7E60: 386AAA44  addi r3, r10, -0x55bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21948;
	// 826F7E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7E84: 4BD6EF9D  bl 0x82466e20
	ctx.lr = 0x826F7E88;
	sub_82466E20(ctx, base);
	// 826F7E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7E98 size=108
    let mut pc: u32 = 0x826F7E98;
    'dispatch: loop {
        match pc {
            0x826F7E98 => {
    //   block [0x826F7E98..0x826F7F04)
	// 826F7E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7EA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7EAC: 38EBF638  addi r7, r11, -0x9c8
	ctx.r[7].s64 = ctx.r[11].s64 + -2504;
	// 826F7EB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F7EB4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 826F7EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7EBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7EC8: 386AAA74  addi r3, r10, -0x558c
	ctx.r[3].s64 = ctx.r[10].s64 + -21900;
	// 826F7ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7EF0: 4BD6EF31  bl 0x82466e20
	ctx.lr = 0x826F7EF4;
	sub_82466E20(ctx, base);
	// 826F7EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7F08 size=108
    let mut pc: u32 = 0x826F7F08;
    'dispatch: loop {
        match pc {
            0x826F7F08 => {
    //   block [0x826F7F08..0x826F7F74)
	// 826F7F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7F14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7F1C: 38EBF668  addi r7, r11, -0x998
	ctx.r[7].s64 = ctx.r[11].s64 + -2456;
	// 826F7F20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F7F24: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826F7F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7F38: 386AAAA4  addi r3, r10, -0x555c
	ctx.r[3].s64 = ctx.r[10].s64 + -21852;
	// 826F7F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7F60: 4BD6EEC1  bl 0x82466e20
	ctx.lr = 0x826F7F64;
	sub_82466E20(ctx, base);
	// 826F7F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7F78 size=112
    let mut pc: u32 = 0x826F7F78;
    'dispatch: loop {
        match pc {
            0x826F7F78 => {
    //   block [0x826F7F78..0x826F7FE8)
	// 826F7F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7F88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7F8C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7F94: 390BF680  addi r8, r11, -0x980
	ctx.r[8].s64 = ctx.r[11].s64 + -2432;
	// 826F7F98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F7F9C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826F7FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7FB0: 386AAAD4  addi r3, r10, -0x552c
	ctx.r[3].s64 = ctx.r[10].s64 + -21804;
	// 826F7FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7FD4: 4BD6EE4D  bl 0x82466e20
	ctx.lr = 0x826F7FD8;
	sub_82466E20(ctx, base);
	// 826F7FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7FE8 size=112
    let mut pc: u32 = 0x826F7FE8;
    'dispatch: loop {
        match pc {
            0x826F7FE8 => {
    //   block [0x826F7FE8..0x826F8058)
	// 826F7FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7FF4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7FF8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7FFC: 392AAB30  addi r9, r10, -0x54d0
	ctx.r[9].s64 = ctx.r[10].s64 + -21712;
	// 826F8000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8004: 390BF6B8  addi r8, r11, -0x948
	ctx.r[8].s64 = ctx.r[11].s64 + -2376;
	// 826F8008: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F800C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826F8010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8014: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8020: 386AAB04  addi r3, r10, -0x54fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21756;
	// 826F8024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F802C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F803C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8044: 4BD6EDDD  bl 0x82466e20
	ctx.lr = 0x826F8048;
	sub_82466E20(ctx, base);
	// 826F8048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8058 size=116
    let mut pc: u32 = 0x826F8058;
    'dispatch: loop {
        match pc {
            0x826F8058 => {
    //   block [0x826F8058..0x826F80CC)
	// 826F8058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8064: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8068: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F806C: 390BF760  addi r8, r11, -0x8a0
	ctx.r[8].s64 = ctx.r[11].s64 + -2208;
	// 826F8070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8074: 392AAB04  addi r9, r10, -0x54fc
	ctx.r[9].s64 = ctx.r[10].s64 + -21756;
	// 826F8078: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F807C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F8080: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F8084: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F808C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F809C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F80A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826F80A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F80A8: 386BAB34  addi r3, r11, -0x54cc
	ctx.r[3].s64 = ctx.r[11].s64 + -21708;
	// 826F80AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F80B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F80B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F80B8: 4BD6ED69  bl 0x82466e20
	ctx.lr = 0x826F80BC;
	sub_82466E20(ctx, base);
	// 826F80BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F80C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F80C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F80C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F80D0 size=112
    let mut pc: u32 = 0x826F80D0;
    'dispatch: loop {
        match pc {
            0x826F80D0 => {
    //   block [0x826F80D0..0x826F8140)
	// 826F80D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F80D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F80D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F80DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F80E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F80E4: 392AAB5C  addi r9, r10, -0x54a4
	ctx.r[9].s64 = ctx.r[10].s64 + -21668;
	// 826F80E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F80EC: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 826F80F0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826F80F4: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826F80F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F80FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8108: 386AAB64  addi r3, r10, -0x549c
	ctx.r[3].s64 = ctx.r[10].s64 + -21660;
	// 826F810C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F8114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F811C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F812C: 4BD6ECF5  bl 0x82466e20
	ctx.lr = 0x826F8130;
	sub_82466E20(ctx, base);
	// 826F8130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8140 size=112
    let mut pc: u32 = 0x826F8140;
    'dispatch: loop {
        match pc {
            0x826F8140 => {
    //   block [0x826F8140..0x826F81B0)
	// 826F8140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F814C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8150: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8154: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F8158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F815C: 390BF7E0  addi r8, r11, -0x820
	ctx.r[8].s64 = ctx.r[11].s64 + -2080;
	// 826F8160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8164: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826F8168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F816C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8178: 386AAB94  addi r3, r10, -0x546c
	ctx.r[3].s64 = ctx.r[10].s64 + -21612;
	// 826F817C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F819C: 4BD6EC85  bl 0x82466e20
	ctx.lr = 0x826F81A0;
	sub_82466E20(ctx, base);
	// 826F81A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F81A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F81A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F81AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F81B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F81B0 size=112
    let mut pc: u32 = 0x826F81B0;
    'dispatch: loop {
        match pc {
            0x826F81B0 => {
    //   block [0x826F81B0..0x826F8220)
	// 826F81B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F81B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F81B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F81BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F81C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F81C4: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F81C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F81CC: 390BF7F8  addi r8, r11, -0x808
	ctx.r[8].s64 = ctx.r[11].s64 + -2056;
	// 826F81D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F81D4: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826F81D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F81DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F81E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F81E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F81E8: 386AABC4  addi r3, r10, -0x543c
	ctx.r[3].s64 = ctx.r[10].s64 + -21564;
	// 826F81EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F81F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F81F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F81FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F820C: 4BD6EC15  bl 0x82466e20
	ctx.lr = 0x826F8210;
	sub_82466E20(ctx, base);
	// 826F8210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8220 size=112
    let mut pc: u32 = 0x826F8220;
    'dispatch: loop {
        match pc {
            0x826F8220 => {
    //   block [0x826F8220..0x826F8290)
	// 826F8220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F822C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8230: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8234: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F8238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F823C: 390BF840  addi r8, r11, -0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + -1984;
	// 826F8240: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F8244: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826F8248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F824C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8258: 386AABF4  addi r3, r10, -0x540c
	ctx.r[3].s64 = ctx.r[10].s64 + -21516;
	// 826F825C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F827C: 4BD6EBA5  bl 0x82466e20
	ctx.lr = 0x826F8280;
	sub_82466E20(ctx, base);
	// 826F8280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F828C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8290 size=112
    let mut pc: u32 = 0x826F8290;
    'dispatch: loop {
        match pc {
            0x826F8290 => {
    //   block [0x826F8290..0x826F8300)
	// 826F8290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F829C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F82A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F82A4: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F82A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F82AC: 390BF8A0  addi r8, r11, -0x760
	ctx.r[8].s64 = ctx.r[11].s64 + -1888;
	// 826F82B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F82B4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826F82B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F82BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F82C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F82C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F82C8: 386AAC24  addi r3, r10, -0x53dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21468;
	// 826F82CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F82D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F82D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F82D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F82DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F82E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F82E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F82E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F82EC: 4BD6EB35  bl 0x82466e20
	ctx.lr = 0x826F82F0;
	sub_82466E20(ctx, base);
	// 826F82F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F82F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F82F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F82FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8300 size=112
    let mut pc: u32 = 0x826F8300;
    'dispatch: loop {
        match pc {
            0x826F8300 => {
    //   block [0x826F8300..0x826F8370)
	// 826F8300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F830C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8310: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8314: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F831C: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 826F8320: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F8324: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826F8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F832C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8338: 386AAC54  addi r3, r10, -0x53ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21420;
	// 826F833C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F834C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F835C: 4BD6EAC5  bl 0x82466e20
	ctx.lr = 0x826F8360;
	sub_82466E20(ctx, base);
	// 826F8360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8370 size=112
    let mut pc: u32 = 0x826F8370;
    'dispatch: loop {
        match pc {
            0x826F8370 => {
    //   block [0x826F8370..0x826F83E0)
	// 826F8370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F837C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8380: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8384: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F8388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F838C: 390BF960  addi r8, r11, -0x6a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1696;
	// 826F8390: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F8394: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826F8398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F839C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F83A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F83A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F83A8: 386AAC84  addi r3, r10, -0x537c
	ctx.r[3].s64 = ctx.r[10].s64 + -21372;
	// 826F83AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F83B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F83B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F83B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F83BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F83C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F83C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F83C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F83CC: 4BD6EA55  bl 0x82466e20
	ctx.lr = 0x826F83D0;
	sub_82466E20(ctx, base);
	// 826F83D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F83D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F83D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F83DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F83E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F83E0 size=112
    let mut pc: u32 = 0x826F83E0;
    'dispatch: loop {
        match pc {
            0x826F83E0 => {
    //   block [0x826F83E0..0x826F8450)
	// 826F83E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F83E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F83E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F83EC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F83F0: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826F83F4: 38EAFA20  addi r7, r10, -0x5e0
	ctx.r[7].s64 = ctx.r[10].s64 + -1504;
	// 826F83F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F83FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F8400: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826F8404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8408: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F840C: 396BAB70  addi r11, r11, -0x5490
	ctx.r[11].s64 = ctx.r[11].s64 + -21648;
	// 826F8410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8418: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F841C: 386AACB4  addi r3, r10, -0x534c
	ctx.r[3].s64 = ctx.r[10].s64 + -21324;
	// 826F8420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8424: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F8428: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F842C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F8430: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8434: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F843C: 4BD6E9E5  bl 0x82466e20
	ctx.lr = 0x826F8440;
	sub_82466E20(ctx, base);
	// 826F8440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F844C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8450 size=112
    let mut pc: u32 = 0x826F8450;
    'dispatch: loop {
        match pc {
            0x826F8450 => {
    //   block [0x826F8450..0x826F84C0)
	// 826F8450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F845C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8460: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8464: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F846C: 390BFBE8  addi r8, r11, -0x418
	ctx.r[8].s64 = ctx.r[11].s64 + -1048;
	// 826F8470: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8474: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826F8478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F847C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8488: 386AACE4  addi r3, r10, -0x531c
	ctx.r[3].s64 = ctx.r[10].s64 + -21276;
	// 826F848C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F849C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F84A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F84A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F84A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F84AC: 4BD6E975  bl 0x82466e20
	ctx.lr = 0x826F84B0;
	sub_82466E20(ctx, base);
	// 826F84B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F84B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F84B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F84BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F84C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F84C0 size=112
    let mut pc: u32 = 0x826F84C0;
    'dispatch: loop {
        match pc {
            0x826F84C0 => {
    //   block [0x826F84C0..0x826F8530)
	// 826F84C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F84C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F84C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F84CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F84D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F84D4: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F84D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F84DC: 390BFC00  addi r8, r11, -0x400
	ctx.r[8].s64 = ctx.r[11].s64 + -1024;
	// 826F84E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F84E4: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826F84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F84EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F84F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F84F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F84F8: 386AAD14  addi r3, r10, -0x52ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21228;
	// 826F84FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F850C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F8510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F851C: 4BD6E905  bl 0x82466e20
	ctx.lr = 0x826F8520;
	sub_82466E20(ctx, base);
	// 826F8520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8530 size=112
    let mut pc: u32 = 0x826F8530;
    'dispatch: loop {
        match pc {
            0x826F8530 => {
    //   block [0x826F8530..0x826F85A0)
	// 826F8530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F853C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8540: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8544: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F854C: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 826F8550: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8554: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826F8558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F855C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8568: 386AAD44  addi r3, r10, -0x52bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21180;
	// 826F856C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F857C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F858C: 4BD6E895  bl 0x82466e20
	ctx.lr = 0x826F8590;
	sub_82466E20(ctx, base);
	// 826F8590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F859C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F85A0 size=108
    let mut pc: u32 = 0x826F85A0;
    'dispatch: loop {
        match pc {
            0x826F85A0 => {
    //   block [0x826F85A0..0x826F860C)
	// 826F85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F85AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F85B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F85B4: 38EBFC48  addi r7, r11, -0x3b8
	ctx.r[7].s64 = ctx.r[11].s64 + -952;
	// 826F85B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F85BC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826F85C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F85C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F85C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F85CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F85D0: 386AAD74  addi r3, r10, -0x528c
	ctx.r[3].s64 = ctx.r[10].s64 + -21132;
	// 826F85D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F85D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F85DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F85E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F85E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F85E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F85EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F85F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F85F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F85F8: 4BD6E829  bl 0x82466e20
	ctx.lr = 0x826F85FC;
	sub_82466E20(ctx, base);
	// 826F85FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8610 size=112
    let mut pc: u32 = 0x826F8610;
    'dispatch: loop {
        match pc {
            0x826F8610 => {
    //   block [0x826F8610..0x826F8680)
	// 826F8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F861C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8620: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8624: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F862C: 390BFC78  addi r8, r11, -0x388
	ctx.r[8].s64 = ctx.r[11].s64 + -904;
	// 826F8630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8634: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826F8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F863C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8648: 386AADA4  addi r3, r10, -0x525c
	ctx.r[3].s64 = ctx.r[10].s64 + -21084;
	// 826F864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F865C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F866C: 4BD6E7B5  bl 0x82466e20
	ctx.lr = 0x826F8670;
	sub_82466E20(ctx, base);
	// 826F8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8680 size=108
    let mut pc: u32 = 0x826F8680;
    'dispatch: loop {
        match pc {
            0x826F8680 => {
    //   block [0x826F8680..0x826F86EC)
	// 826F8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F868C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8694: 38EBFC90  addi r7, r11, -0x370
	ctx.r[7].s64 = ctx.r[11].s64 + -880;
	// 826F8698: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F869C: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826F86A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F86A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F86A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F86AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F86B0: 386AADD4  addi r3, r10, -0x522c
	ctx.r[3].s64 = ctx.r[10].s64 + -21036;
	// 826F86B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F86B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F86BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F86C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F86C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F86CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F86D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F86D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F86D8: 4BD6E749  bl 0x82466e20
	ctx.lr = 0x826F86DC;
	sub_82466E20(ctx, base);
	// 826F86DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F86E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F86E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F86E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F86F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F86F0 size=108
    let mut pc: u32 = 0x826F86F0;
    'dispatch: loop {
        match pc {
            0x826F86F0 => {
    //   block [0x826F86F0..0x826F875C)
	// 826F86F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F86F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F86F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F86FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8704: 38EBFCC0  addi r7, r11, -0x340
	ctx.r[7].s64 = ctx.r[11].s64 + -832;
	// 826F8708: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F870C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826F8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F871C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8720: 386AAE04  addi r3, r10, -0x51fc
	ctx.r[3].s64 = ctx.r[10].s64 + -20988;
	// 826F8724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F872C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8748: 4BD6E6D9  bl 0x82466e20
	ctx.lr = 0x826F874C;
	sub_82466E20(ctx, base);
	// 826F874C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8760 size=112
    let mut pc: u32 = 0x826F8760;
    'dispatch: loop {
        match pc {
            0x826F8760 => {
    //   block [0x826F8760..0x826F87D0)
	// 826F8760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F876C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8770: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8774: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F877C: 390BFD08  addi r8, r11, -0x2f8
	ctx.r[8].s64 = ctx.r[11].s64 + -760;
	// 826F8780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F8784: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826F8788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F878C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8798: 386AAE34  addi r3, r10, -0x51cc
	ctx.r[3].s64 = ctx.r[10].s64 + -20940;
	// 826F879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F87A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F87A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F87A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F87AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F87B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F87B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F87B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F87BC: 4BD6E665  bl 0x82466e20
	ctx.lr = 0x826F87C0;
	sub_82466E20(ctx, base);
	// 826F87C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F87C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F87C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F87CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F87D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F87D0 size=112
    let mut pc: u32 = 0x826F87D0;
    'dispatch: loop {
        match pc {
            0x826F87D0 => {
    //   block [0x826F87D0..0x826F8840)
	// 826F87D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F87D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F87D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F87DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F87E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F87E4: 38AAA8F4  addi r5, r10, -0x570c
	ctx.r[5].s64 = ctx.r[10].s64 + -22284;
	// 826F87E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F87EC: 390BFD50  addi r8, r11, -0x2b0
	ctx.r[8].s64 = ctx.r[11].s64 + -688;
	// 826F87F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F87F4: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 826F87F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F87FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8808: 386AAE64  addi r3, r10, -0x519c
	ctx.r[3].s64 = ctx.r[10].s64 + -20892;
	// 826F880C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F881C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F882C: 4BD6E5F5  bl 0x82466e20
	ctx.lr = 0x826F8830;
	sub_82466E20(ctx, base);
	// 826F8830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8840 size=108
    let mut pc: u32 = 0x826F8840;
    'dispatch: loop {
        match pc {
            0x826F8840 => {
    //   block [0x826F8840..0x826F88AC)
	// 826F8840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F884C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F8854: 38EBFD68  addi r7, r11, -0x298
	ctx.r[7].s64 = ctx.r[11].s64 + -664;
	// 826F8858: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F885C: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 826F8860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F886C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8870: 386AAE94  addi r3, r10, -0x516c
	ctx.r[3].s64 = ctx.r[10].s64 + -20844;
	// 826F8874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F887C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F888C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8898: 4BD6E589  bl 0x82466e20
	ctx.lr = 0x826F889C;
	sub_82466E20(ctx, base);
	// 826F889C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F88A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F88A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F88A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F88B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F88B0 size=108
    let mut pc: u32 = 0x826F88B0;
    'dispatch: loop {
        match pc {
            0x826F88B0 => {
    //   block [0x826F88B0..0x826F891C)
	// 826F88B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F88B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F88B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F88BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F88C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F88C4: 38EBFD80  addi r7, r11, -0x280
	ctx.r[7].s64 = ctx.r[11].s64 + -640;
	// 826F88C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F88CC: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 826F88D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F88D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F88D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F88DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F88E0: 386AAEC4  addi r3, r10, -0x513c
	ctx.r[3].s64 = ctx.r[10].s64 + -20796;
	// 826F88E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F88E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F88EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F88F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F88F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F88F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F88FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8908: 4BD6E519  bl 0x82466e20
	ctx.lr = 0x826F890C;
	sub_82466E20(ctx, base);
	// 826F890C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8920 size=112
    let mut pc: u32 = 0x826F8920;
    'dispatch: loop {
        match pc {
            0x826F8920 => {
    //   block [0x826F8920..0x826F8990)
	// 826F8920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F892C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8930: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8934: 38AAAEC4  addi r5, r10, -0x513c
	ctx.r[5].s64 = ctx.r[10].s64 + -20796;
	// 826F8938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F893C: 390BFDB0  addi r8, r11, -0x250
	ctx.r[8].s64 = ctx.r[11].s64 + -592;
	// 826F8940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8944: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 826F8948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F894C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8958: 386AAEF4  addi r3, r10, -0x510c
	ctx.r[3].s64 = ctx.r[10].s64 + -20748;
	// 826F895C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F897C: 4BD6E4A5  bl 0x82466e20
	ctx.lr = 0x826F8980;
	sub_82466E20(ctx, base);
	// 826F8980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F898C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F8990 size=24
    let mut pc: u32 = 0x826F8990;
    'dispatch: loop {
        match pc {
            0x826F8990 => {
    //   block [0x826F8990..0x826F89A8)
	// 826F8990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8994: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F8998: 394A2A20  addi r10, r10, 0x2a20
	ctx.r[10].s64 = ctx.r[10].s64 + 10784;
	// 826F899C: 816BF77C  lwz r11, -0x884(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2180 as u32) ) } as u64;
	// 826F89A0: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826F89A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F89A8 size=116
    let mut pc: u32 = 0x826F89A8;
    'dispatch: loop {
        match pc {
            0x826F89A8 => {
    //   block [0x826F89A8..0x826F8A1C)
	// 826F89A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F89AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F89B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F89B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F89B8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F89BC: 390B2A20  addi r8, r11, 0x2a20
	ctx.r[8].s64 = ctx.r[11].s64 + 10784;
	// 826F89C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F89C4: 392AAC08  addi r9, r10, -0x53f8
	ctx.r[9].s64 = ctx.r[10].s64 + -21496;
	// 826F89C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F89CC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826F89D0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F89D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F89D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F89DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F89E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F89E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F89E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F89EC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F89F0: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 826F89F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F89F8: 386BAF24  addi r3, r11, -0x50dc
	ctx.r[3].s64 = ctx.r[11].s64 + -20700;
	// 826F89FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F8A00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8A08: 4BD6E419  bl 0x82466e20
	ctx.lr = 0x826F8A0C;
	sub_82466E20(ctx, base);
	// 826F8A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8A20 size=112
    let mut pc: u32 = 0x826F8A20;
    'dispatch: loop {
        match pc {
            0x826F8A20 => {
    //   block [0x826F8A20..0x826F8A90)
	// 826F8A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8A30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8A34: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8A3C: 390BFDE0  addi r8, r11, -0x220
	ctx.r[8].s64 = ctx.r[11].s64 + -544;
	// 826F8A40: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F8A44: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826F8A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8A58: 386AAF54  addi r3, r10, -0x50ac
	ctx.r[3].s64 = ctx.r[10].s64 + -20652;
	// 826F8A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8A7C: 4BD6E3A5  bl 0x82466e20
	ctx.lr = 0x826F8A80;
	sub_82466E20(ctx, base);
	// 826F8A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8A90 size=108
    let mut pc: u32 = 0x826F8A90;
    'dispatch: loop {
        match pc {
            0x826F8A90 => {
    //   block [0x826F8A90..0x826F8AFC)
	// 826F8A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8A9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8AA4: 38EBFE70  addi r7, r11, -0x190
	ctx.r[7].s64 = ctx.r[11].s64 + -400;
	// 826F8AA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F8AAC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826F8AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8AC0: 386AAF84  addi r3, r10, -0x507c
	ctx.r[3].s64 = ctx.r[10].s64 + -20604;
	// 826F8AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8AE8: 4BD6E339  bl 0x82466e20
	ctx.lr = 0x826F8AEC;
	sub_82466E20(ctx, base);
	// 826F8AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8B00 size=108
    let mut pc: u32 = 0x826F8B00;
    'dispatch: loop {
        match pc {
            0x826F8B00 => {
    //   block [0x826F8B00..0x826F8B6C)
	// 826F8B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8B0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8B14: 38EBFEB8  addi r7, r11, -0x148
	ctx.r[7].s64 = ctx.r[11].s64 + -328;
	// 826F8B18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8B1C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826F8B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8B28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8B30: 386AAFB4  addi r3, r10, -0x504c
	ctx.r[3].s64 = ctx.r[10].s64 + -20556;
	// 826F8B34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8B58: 4BD6E2C9  bl 0x82466e20
	ctx.lr = 0x826F8B5C;
	sub_82466E20(ctx, base);
	// 826F8B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8B70 size=108
    let mut pc: u32 = 0x826F8B70;
    'dispatch: loop {
        match pc {
            0x826F8B70 => {
    //   block [0x826F8B70..0x826F8BDC)
	// 826F8B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8B7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8B84: 38EBFEE8  addi r7, r11, -0x118
	ctx.r[7].s64 = ctx.r[11].s64 + -280;
	// 826F8B88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8B8C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826F8B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8BA0: 386AAFE4  addi r3, r10, -0x501c
	ctx.r[3].s64 = ctx.r[10].s64 + -20508;
	// 826F8BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8BC8: 4BD6E259  bl 0x82466e20
	ctx.lr = 0x826F8BCC;
	sub_82466E20(ctx, base);
	// 826F8BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8BE0 size=112
    let mut pc: u32 = 0x826F8BE0;
    'dispatch: loop {
        match pc {
            0x826F8BE0 => {
    //   block [0x826F8BE0..0x826F8C50)
	// 826F8BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8BF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8BF4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8BFC: 390BFF18  addi r8, r11, -0xe8
	ctx.r[8].s64 = ctx.r[11].s64 + -232;
	// 826F8C00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8C04: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826F8C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8C0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8C18: 386AB014  addi r3, r10, -0x4fec
	ctx.r[3].s64 = ctx.r[10].s64 + -20460;
	// 826F8C1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8C3C: 4BD6E1E5  bl 0x82466e20
	ctx.lr = 0x826F8C40;
	sub_82466E20(ctx, base);
	// 826F8C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8C50 size=112
    let mut pc: u32 = 0x826F8C50;
    'dispatch: loop {
        match pc {
            0x826F8C50 => {
    //   block [0x826F8C50..0x826F8CC0)
	// 826F8C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8C60: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8C64: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8C6C: 390BFF48  addi r8, r11, -0xb8
	ctx.r[8].s64 = ctx.r[11].s64 + -184;
	// 826F8C70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8C74: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826F8C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8C88: 386AB044  addi r3, r10, -0x4fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -20412;
	// 826F8C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8CAC: 4BD6E175  bl 0x82466e20
	ctx.lr = 0x826F8CB0;
	sub_82466E20(ctx, base);
	// 826F8CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8CC0 size=112
    let mut pc: u32 = 0x826F8CC0;
    'dispatch: loop {
        match pc {
            0x826F8CC0 => {
    //   block [0x826F8CC0..0x826F8D30)
	// 826F8CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8CD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8CD4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8CDC: 390BFF60  addi r8, r11, -0xa0
	ctx.r[8].s64 = ctx.r[11].s64 + -160;
	// 826F8CE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8CE4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826F8CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8CEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8CF8: 386AB074  addi r3, r10, -0x4f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -20364;
	// 826F8CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8D1C: 4BD6E105  bl 0x82466e20
	ctx.lr = 0x826F8D20;
	sub_82466E20(ctx, base);
	// 826F8D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8D30 size=108
    let mut pc: u32 = 0x826F8D30;
    'dispatch: loop {
        match pc {
            0x826F8D30 => {
    //   block [0x826F8D30..0x826F8D9C)
	// 826F8D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8D3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8D44: 38EBFF78  addi r7, r11, -0x88
	ctx.r[7].s64 = ctx.r[11].s64 + -136;
	// 826F8D48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8D4C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826F8D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8D60: 386AB0A4  addi r3, r10, -0x4f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20316;
	// 826F8D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8D88: 4BD6E099  bl 0x82466e20
	ctx.lr = 0x826F8D8C;
	sub_82466E20(ctx, base);
	// 826F8D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8DA0 size=112
    let mut pc: u32 = 0x826F8DA0;
    'dispatch: loop {
        match pc {
            0x826F8DA0 => {
    //   block [0x826F8DA0..0x826F8E10)
	// 826F8DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8DB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8DB4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8DBC: 390BFFA8  addi r8, r11, -0x58
	ctx.r[8].s64 = ctx.r[11].s64 + -88;
	// 826F8DC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8DC4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826F8DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8DCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8DD8: 386AB0D4  addi r3, r10, -0x4f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -20268;
	// 826F8DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8DFC: 4BD6E025  bl 0x82466e20
	ctx.lr = 0x826F8E00;
	sub_82466E20(ctx, base);
	// 826F8E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8E10 size=108
    let mut pc: u32 = 0x826F8E10;
    'dispatch: loop {
        match pc {
            0x826F8E10 => {
    //   block [0x826F8E10..0x826F8E7C)
	// 826F8E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8E1C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8E24: 38EBFFC0  addi r7, r11, -0x40
	ctx.r[7].s64 = ctx.r[11].s64 + -64;
	// 826F8E28: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826F8E2C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826F8E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8E34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8E40: 386AB104  addi r3, r10, -0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + -20220;
	// 826F8E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8E68: 4BD6DFB9  bl 0x82466e20
	ctx.lr = 0x826F8E6C;
	sub_82466E20(ctx, base);
	// 826F8E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8E80 size=116
    let mut pc: u32 = 0x826F8E80;
    'dispatch: loop {
        match pc {
            0x826F8E80 => {
    //   block [0x826F8E80..0x826F8EF4)
	// 826F8E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8E8C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F8E90: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826F8E94: 390A00B0  addi r8, r10, 0xb0
	ctx.r[8].s64 = ctx.r[10].s64 + 176;
	// 826F8E98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8E9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F8EA0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8EA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8EA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8EB4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826F8EB8: 396BAC20  addi r11, r11, -0x53e0
	ctx.r[11].s64 = ctx.r[11].s64 + -21472;
	// 826F8EBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8EC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8EC4: 386AB134  addi r3, r10, -0x4ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -20172;
	// 826F8EC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F8ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8ED0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F8ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8EE0: 4BD6DF41  bl 0x82466e20
	ctx.lr = 0x826F8EE4;
	sub_82466E20(ctx, base);
	// 826F8EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8EF8 size=108
    let mut pc: u32 = 0x826F8EF8;
    'dispatch: loop {
        match pc {
            0x826F8EF8 => {
    //   block [0x826F8EF8..0x826F8F64)
	// 826F8EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8F04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8F0C: 38EB0278  addi r7, r11, 0x278
	ctx.r[7].s64 = ctx.r[11].s64 + 632;
	// 826F8F10: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826F8F14: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826F8F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8F28: 386AB164  addi r3, r10, -0x4e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20124;
	// 826F8F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8F50: 4BD6DED1  bl 0x82466e20
	ctx.lr = 0x826F8F54;
	sub_82466E20(ctx, base);
	// 826F8F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8F68 size=112
    let mut pc: u32 = 0x826F8F68;
    'dispatch: loop {
        match pc {
            0x826F8F68 => {
    //   block [0x826F8F68..0x826F8FD8)
	// 826F8F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8F74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8F7C: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8F84: 390B0410  addi r8, r11, 0x410
	ctx.r[8].s64 = ctx.r[11].s64 + 1040;
	// 826F8F88: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826F8F8C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826F8F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8F94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8FA0: 386AB194  addi r3, r10, -0x4e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20076;
	// 826F8FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8FC4: 4BD6DE5D  bl 0x82466e20
	ctx.lr = 0x826F8FC8;
	sub_82466E20(ctx, base);
	// 826F8FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8FD8 size=100
    let mut pc: u32 = 0x826F8FD8;
    'dispatch: loop {
        match pc {
            0x826F8FD8 => {
    //   block [0x826F8FD8..0x826F903C)
	// 826F8FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8FE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8FEC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8FF8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826F8FFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F900C: 386AB1C4  addi r3, r10, -0x4e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20028;
	// 826F9010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F901C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9028: 4BD6DDF9  bl 0x82466e20
	ctx.lr = 0x826F902C;
	sub_82466E20(ctx, base);
	// 826F902C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9040 size=112
    let mut pc: u32 = 0x826F9040;
    'dispatch: loop {
        match pc {
            0x826F9040 => {
    //   block [0x826F9040..0x826F90B0)
	// 826F9040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F904C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9050: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9054: 38AAB1C4  addi r5, r10, -0x4e3c
	ctx.r[5].s64 = ctx.r[10].s64 + -20028;
	// 826F9058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F905C: 390B0668  addi r8, r11, 0x668
	ctx.r[8].s64 = ctx.r[11].s64 + 1640;
	// 826F9060: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F9064: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826F9068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F906C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9078: 386AB1F4  addi r3, r10, -0x4e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19980;
	// 826F907C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F908C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F909C: 4BD6DD85  bl 0x82466e20
	ctx.lr = 0x826F90A0;
	sub_82466E20(ctx, base);
	// 826F90A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F90A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F90A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F90AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F90B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F90B0 size=100
    let mut pc: u32 = 0x826F90B0;
    'dispatch: loop {
        match pc {
            0x826F90B0 => {
    //   block [0x826F90B0..0x826F9114)
	// 826F90B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F90B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F90B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F90BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F90C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F90C4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F90C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F90CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F90D0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826F90D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F90D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F90DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F90E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F90E4: 386AB224  addi r3, r10, -0x4ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -19932;
	// 826F90E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F90EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F90F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F90F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F90F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F90FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9100: 4BD6DD21  bl 0x82466e20
	ctx.lr = 0x826F9104;
	sub_82466E20(ctx, base);
	// 826F9104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9118 size=108
    let mut pc: u32 = 0x826F9118;
    'dispatch: loop {
        match pc {
            0x826F9118 => {
    //   block [0x826F9118..0x826F9184)
	// 826F9118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F911C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9124: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F912C: 38EB06E0  addi r7, r11, 0x6e0
	ctx.r[7].s64 = ctx.r[11].s64 + 1760;
	// 826F9130: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9134: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826F9138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F913C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9148: 386AB254  addi r3, r10, -0x4dac
	ctx.r[3].s64 = ctx.r[10].s64 + -19884;
	// 826F914C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F916C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9170: 4BD6DCB1  bl 0x82466e20
	ctx.lr = 0x826F9174;
	sub_82466E20(ctx, base);
	// 826F9174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F917C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9188 size=112
    let mut pc: u32 = 0x826F9188;
    'dispatch: loop {
        match pc {
            0x826F9188 => {
    //   block [0x826F9188..0x826F91F8)
	// 826F9188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9194: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9198: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F919C: 38AAB224  addi r5, r10, -0x4ddc
	ctx.r[5].s64 = ctx.r[10].s64 + -19932;
	// 826F91A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F91A4: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 826F91A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F91AC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826F91B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F91B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F91B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F91BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F91C0: 386AB284  addi r3, r10, -0x4d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19836;
	// 826F91C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F91C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F91CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F91D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F91D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F91D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F91DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F91E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F91E4: 4BD6DC3D  bl 0x82466e20
	ctx.lr = 0x826F91E8;
	sub_82466E20(ctx, base);
	// 826F91E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F91EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F91F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F91F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F91F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F91F8 size=100
    let mut pc: u32 = 0x826F91F8;
    'dispatch: loop {
        match pc {
            0x826F91F8 => {
    //   block [0x826F91F8..0x826F925C)
	// 826F91F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F91FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F920C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9218: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826F921C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F922C: 386AB2B4  addi r3, r10, -0x4d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19788;
	// 826F9230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9238: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9248: 4BD6DBD9  bl 0x82466e20
	ctx.lr = 0x826F924C;
	sub_82466E20(ctx, base);
	// 826F924C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9260 size=100
    let mut pc: u32 = 0x826F9260;
    'dispatch: loop {
        match pc {
            0x826F9260 => {
    //   block [0x826F9260..0x826F92C4)
	// 826F9260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F926C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9274: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F927C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9280: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826F9284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F928C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9294: 386AB2E4  addi r3, r10, -0x4d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19740;
	// 826F9298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F929C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F92A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F92A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F92A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F92AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F92B0: 4BD6DB71  bl 0x82466e20
	ctx.lr = 0x826F92B4;
	sub_82466E20(ctx, base);
	// 826F92B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F92B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F92BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F92C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F92C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F92C8 size=112
    let mut pc: u32 = 0x826F92C8;
    'dispatch: loop {
        match pc {
            0x826F92C8 => {
    //   block [0x826F92C8..0x826F9338)
	// 826F92C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F92CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F92D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F92D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F92D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F92DC: 38AAB2B4  addi r5, r10, -0x4d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -19788;
	// 826F92E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F92E4: 390B0758  addi r8, r11, 0x758
	ctx.r[8].s64 = ctx.r[11].s64 + 1880;
	// 826F92E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F92EC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826F92F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F92F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F92F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F92FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9300: 386AB314  addi r3, r10, -0x4cec
	ctx.r[3].s64 = ctx.r[10].s64 + -19692;
	// 826F9304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F930C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F931C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9324: 4BD6DAFD  bl 0x82466e20
	ctx.lr = 0x826F9328;
	sub_82466E20(ctx, base);
	// 826F9328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F932C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9338 size=112
    let mut pc: u32 = 0x826F9338;
    'dispatch: loop {
        match pc {
            0x826F9338 => {
    //   block [0x826F9338..0x826F93A8)
	// 826F9338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9344: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9348: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F934C: 38AAB2E4  addi r5, r10, -0x4d1c
	ctx.r[5].s64 = ctx.r[10].s64 + -19740;
	// 826F9350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9354: 390B07B8  addi r8, r11, 0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1976;
	// 826F9358: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F935C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826F9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F936C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9370: 386AB344  addi r3, r10, -0x4cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19644;
	// 826F9374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F937C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F938C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9394: 4BD6DA8D  bl 0x82466e20
	ctx.lr = 0x826F9398;
	sub_82466E20(ctx, base);
	// 826F9398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F939C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F93A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F93A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F93A8 size=100
    let mut pc: u32 = 0x826F93A8;
    'dispatch: loop {
        match pc {
            0x826F93A8 => {
    //   block [0x826F93A8..0x826F940C)
	// 826F93A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F93AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F93B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F93B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F93B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F93BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F93C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F93C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F93C8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826F93CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F93D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F93D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F93D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F93DC: 386AB374  addi r3, r10, -0x4c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19596;
	// 826F93E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F93E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F93E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F93EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F93F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F93F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F93F8: 4BD6DA29  bl 0x82466e20
	ctx.lr = 0x826F93FC;
	sub_82466E20(ctx, base);
	// 826F93FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9410 size=112
    let mut pc: u32 = 0x826F9410;
    'dispatch: loop {
        match pc {
            0x826F9410 => {
    //   block [0x826F9410..0x826F9480)
	// 826F9410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F941C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9420: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9424: 38AAB374  addi r5, r10, -0x4c8c
	ctx.r[5].s64 = ctx.r[10].s64 + -19596;
	// 826F9428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F942C: 390B0818  addi r8, r11, 0x818
	ctx.r[8].s64 = ctx.r[11].s64 + 2072;
	// 826F9430: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826F9434: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826F9438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F943C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9448: 386AB3A4  addi r3, r10, -0x4c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19548;
	// 826F944C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F945C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F946C: 4BD6D9B5  bl 0x82466e20
	ctx.lr = 0x826F9470;
	sub_82466E20(ctx, base);
	// 826F9470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9480 size=108
    let mut pc: u32 = 0x826F9480;
    'dispatch: loop {
        match pc {
            0x826F9480 => {
    //   block [0x826F9480..0x826F94EC)
	// 826F9480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F948C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9494: 38EB0908  addi r7, r11, 0x908
	ctx.r[7].s64 = ctx.r[11].s64 + 2312;
	// 826F9498: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826F949C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826F94A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F94A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F94A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F94AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F94B0: 386AB3D4  addi r3, r10, -0x4c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19500;
	// 826F94B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F94B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F94BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F94C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F94C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F94C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F94CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F94D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F94D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F94D8: 4BD6D949  bl 0x82466e20
	ctx.lr = 0x826F94DC;
	sub_82466E20(ctx, base);
	// 826F94DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F94E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F94E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F94E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F94F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F94F0 size=108
    let mut pc: u32 = 0x826F94F0;
    'dispatch: loop {
        match pc {
            0x826F94F0 => {
    //   block [0x826F94F0..0x826F955C)
	// 826F94F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F94F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F94F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F94FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9504: 38EB09F8  addi r7, r11, 0x9f8
	ctx.r[7].s64 = ctx.r[11].s64 + 2552;
	// 826F9508: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F950C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826F9510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9520: 386AB404  addi r3, r10, -0x4bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19452;
	// 826F9524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F952C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9548: 4BD6D8D9  bl 0x82466e20
	ctx.lr = 0x826F954C;
	sub_82466E20(ctx, base);
	// 826F954C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9560 size=108
    let mut pc: u32 = 0x826F9560;
    'dispatch: loop {
        match pc {
            0x826F9560 => {
    //   block [0x826F9560..0x826F95CC)
	// 826F9560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F956C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9574: 38EB0A40  addi r7, r11, 0xa40
	ctx.r[7].s64 = ctx.r[11].s64 + 2624;
	// 826F9578: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F957C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826F9580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9584: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F958C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9590: 386AB434  addi r3, r10, -0x4bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -19404;
	// 826F9594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F959C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F95A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F95A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F95AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F95B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F95B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F95B8: 4BD6D869  bl 0x82466e20
	ctx.lr = 0x826F95BC;
	sub_82466E20(ctx, base);
	// 826F95BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F95C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F95C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F95C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F95D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F95D0 size=108
    let mut pc: u32 = 0x826F95D0;
    'dispatch: loop {
        match pc {
            0x826F95D0 => {
    //   block [0x826F95D0..0x826F963C)
	// 826F95D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F95D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F95D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F95DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F95E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F95E4: 38EB0B18  addi r7, r11, 0xb18
	ctx.r[7].s64 = ctx.r[11].s64 + 2840;
	// 826F95E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F95EC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826F95F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F95F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F95F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F95FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9600: 386AB464  addi r3, r10, -0x4b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19356;
	// 826F9604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F960C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F961C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9628: 4BD6D7F9  bl 0x82466e20
	ctx.lr = 0x826F962C;
	sub_82466E20(ctx, base);
	// 826F962C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9640 size=100
    let mut pc: u32 = 0x826F9640;
    'dispatch: loop {
        match pc {
            0x826F9640 => {
    //   block [0x826F9640..0x826F96A4)
	// 826F9640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F964C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9654: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9660: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826F9664: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F966C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9674: 386AB494  addi r3, r10, -0x4b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19308;
	// 826F9678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F967C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9680: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9688: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F968C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9690: 4BD6D791  bl 0x82466e20
	ctx.lr = 0x826F9694;
	sub_82466E20(ctx, base);
	// 826F9694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F96A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F96A8 size=112
    let mut pc: u32 = 0x826F96A8;
    'dispatch: loop {
        match pc {
            0x826F96A8 => {
    //   block [0x826F96A8..0x826F9718)
	// 826F96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F96B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F96B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F96B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F96BC: 38AAB494  addi r5, r10, -0x4b6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19308;
	// 826F96C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F96C4: 390B0B30  addi r8, r11, 0xb30
	ctx.r[8].s64 = ctx.r[11].s64 + 2864;
	// 826F96C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F96CC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826F96D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F96D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F96D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F96DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F96E0: 386AB4C4  addi r3, r10, -0x4b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19260;
	// 826F96E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F96E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F96EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F96F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F96F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F96F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F96FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9704: 4BD6D71D  bl 0x82466e20
	ctx.lr = 0x826F9708;
	sub_82466E20(ctx, base);
	// 826F9708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F970C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9718 size=108
    let mut pc: u32 = 0x826F9718;
    'dispatch: loop {
        match pc {
            0x826F9718 => {
    //   block [0x826F9718..0x826F9784)
	// 826F9718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9724: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F972C: 38EB0B78  addi r7, r11, 0xb78
	ctx.r[7].s64 = ctx.r[11].s64 + 2936;
	// 826F9730: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9734: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826F9738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F973C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9740: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9748: 386AB4F4  addi r3, r10, -0x4b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19212;
	// 826F974C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F975C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F976C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9770: 4BD6D6B1  bl 0x82466e20
	ctx.lr = 0x826F9774;
	sub_82466E20(ctx, base);
	// 826F9774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F977C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9788 size=112
    let mut pc: u32 = 0x826F9788;
    'dispatch: loop {
        match pc {
            0x826F9788 => {
    //   block [0x826F9788..0x826F97F8)
	// 826F9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F978C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9798: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F979C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F97A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F97A4: 390B0BC0  addi r8, r11, 0xbc0
	ctx.r[8].s64 = ctx.r[11].s64 + 3008;
	// 826F97A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F97AC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826F97B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F97B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F97B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F97BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F97C0: 386AB524  addi r3, r10, -0x4adc
	ctx.r[3].s64 = ctx.r[10].s64 + -19164;
	// 826F97C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F97C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F97CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F97D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F97D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F97D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F97DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F97E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F97E4: 4BD6D63D  bl 0x82466e20
	ctx.lr = 0x826F97E8;
	sub_82466E20(ctx, base);
	// 826F97E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F97EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F97F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F97F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F97F8 size=108
    let mut pc: u32 = 0x826F97F8;
    'dispatch: loop {
        match pc {
            0x826F97F8 => {
    //   block [0x826F97F8..0x826F9864)
	// 826F97F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F97FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9804: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F980C: 38EB0BD8  addi r7, r11, 0xbd8
	ctx.r[7].s64 = ctx.r[11].s64 + 3032;
	// 826F9810: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9814: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826F9818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F981C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9828: 386AB554  addi r3, r10, -0x4aac
	ctx.r[3].s64 = ctx.r[10].s64 + -19116;
	// 826F982C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F983C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F984C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9850: 4BD6D5D1  bl 0x82466e20
	ctx.lr = 0x826F9854;
	sub_82466E20(ctx, base);
	// 826F9854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F985C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9868 size=112
    let mut pc: u32 = 0x826F9868;
    'dispatch: loop {
        match pc {
            0x826F9868 => {
    //   block [0x826F9868..0x826F98D8)
	// 826F9868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9874: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9878: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F987C: 38AAB524  addi r5, r10, -0x4adc
	ctx.r[5].s64 = ctx.r[10].s64 + -19164;
	// 826F9880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9884: 390B0C20  addi r8, r11, 0xc20
	ctx.r[8].s64 = ctx.r[11].s64 + 3104;
	// 826F9888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F988C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826F9890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F989C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F98A0: 386AB584  addi r3, r10, -0x4a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19068;
	// 826F98A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F98A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F98AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F98B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F98B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F98B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F98BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F98C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F98C4: 4BD6D55D  bl 0x82466e20
	ctx.lr = 0x826F98C8;
	sub_82466E20(ctx, base);
	// 826F98C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F98CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F98D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F98D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F98D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F98D8 size=100
    let mut pc: u32 = 0x826F98D8;
    'dispatch: loop {
        match pc {
            0x826F98D8 => {
    //   block [0x826F98D8..0x826F993C)
	// 826F98D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F98DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F98E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F98E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F98E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F98EC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F98F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F98F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F98F8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826F98FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F990C: 386AB5B4  addi r3, r10, -0x4a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19020;
	// 826F9910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F991C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9928: 4BD6D4F9  bl 0x82466e20
	ctx.lr = 0x826F992C;
	sub_82466E20(ctx, base);
	// 826F992C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9940 size=112
    let mut pc: u32 = 0x826F9940;
    'dispatch: loop {
        match pc {
            0x826F9940 => {
    //   block [0x826F9940..0x826F99B0)
	// 826F9940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F994C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9950: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9954: 38AAB5B4  addi r5, r10, -0x4a4c
	ctx.r[5].s64 = ctx.r[10].s64 + -19020;
	// 826F9958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F995C: 390B0C38  addi r8, r11, 0xc38
	ctx.r[8].s64 = ctx.r[11].s64 + 3128;
	// 826F9960: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F9964: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826F9968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F996C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9978: 386AB5E4  addi r3, r10, -0x4a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -18972;
	// 826F997C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F999C: 4BD6D485  bl 0x82466e20
	ctx.lr = 0x826F99A0;
	sub_82466E20(ctx, base);
	// 826F99A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F99A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F99A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F99AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F99B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F99B0 size=108
    let mut pc: u32 = 0x826F99B0;
    'dispatch: loop {
        match pc {
            0x826F99B0 => {
    //   block [0x826F99B0..0x826F9A1C)
	// 826F99B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F99B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F99B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F99BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F99C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F99C4: 38EB0CE0  addi r7, r11, 0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + 3296;
	// 826F99C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F99CC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826F99D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F99D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F99D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F99DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F99E0: 386AB614  addi r3, r10, -0x49ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18924;
	// 826F99E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F99E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F99EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F99F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F99FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9A08: 4BD6D419  bl 0x82466e20
	ctx.lr = 0x826F9A0C;
	sub_82466E20(ctx, base);
	// 826F9A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9A20 size=112
    let mut pc: u32 = 0x826F9A20;
    'dispatch: loop {
        match pc {
            0x826F9A20 => {
    //   block [0x826F9A20..0x826F9A90)
	// 826F9A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9A30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9A34: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9A3C: 390B0D10  addi r8, r11, 0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + 3344;
	// 826F9A40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9A44: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826F9A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9A58: 386AB644  addi r3, r10, -0x49bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18876;
	// 826F9A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9A7C: 4BD6D3A5  bl 0x82466e20
	ctx.lr = 0x826F9A80;
	sub_82466E20(ctx, base);
	// 826F9A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9A90 size=112
    let mut pc: u32 = 0x826F9A90;
    'dispatch: loop {
        match pc {
            0x826F9A90 => {
    //   block [0x826F9A90..0x826F9B00)
	// 826F9A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9A9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9AA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9AA4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9AAC: 390B0D58  addi r8, r11, 0xd58
	ctx.r[8].s64 = ctx.r[11].s64 + 3416;
	// 826F9AB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9AB4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826F9AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9ABC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9AC8: 386AB674  addi r3, r10, -0x498c
	ctx.r[3].s64 = ctx.r[10].s64 + -18828;
	// 826F9ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9AEC: 4BD6D335  bl 0x82466e20
	ctx.lr = 0x826F9AF0;
	sub_82466E20(ctx, base);
	// 826F9AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9B00 size=100
    let mut pc: u32 = 0x826F9B00;
    'dispatch: loop {
        match pc {
            0x826F9B00 => {
    //   block [0x826F9B00..0x826F9B64)
	// 826F9B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9B0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9B14: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9B20: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826F9B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9B34: 386AB6A4  addi r3, r10, -0x495c
	ctx.r[3].s64 = ctx.r[10].s64 + -18780;
	// 826F9B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9B40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F9B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9B48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9B50: 4BD6D2D1  bl 0x82466e20
	ctx.lr = 0x826F9B54;
	sub_82466E20(ctx, base);
	// 826F9B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9B68 size=112
    let mut pc: u32 = 0x826F9B68;
    'dispatch: loop {
        match pc {
            0x826F9B68 => {
    //   block [0x826F9B68..0x826F9BD8)
	// 826F9B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9B74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9B7C: 38AAB6A4  addi r5, r10, -0x495c
	ctx.r[5].s64 = ctx.r[10].s64 + -18780;
	// 826F9B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9B84: 390B0DA0  addi r8, r11, 0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + 3488;
	// 826F9B88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9B8C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826F9B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9BA0: 386AB6D4  addi r3, r10, -0x492c
	ctx.r[3].s64 = ctx.r[10].s64 + -18732;
	// 826F9BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9BC4: 4BD6D25D  bl 0x82466e20
	ctx.lr = 0x826F9BC8;
	sub_82466E20(ctx, base);
	// 826F9BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9BD8 size=112
    let mut pc: u32 = 0x826F9BD8;
    'dispatch: loop {
        match pc {
            0x826F9BD8 => {
    //   block [0x826F9BD8..0x826F9C48)
	// 826F9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9BE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9BEC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9BF4: 390B0DE8  addi r8, r11, 0xde8
	ctx.r[8].s64 = ctx.r[11].s64 + 3560;
	// 826F9BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F9BFC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826F9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9C10: 386AB704  addi r3, r10, -0x48fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18684;
	// 826F9C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9C34: 4BD6D1ED  bl 0x82466e20
	ctx.lr = 0x826F9C38;
	sub_82466E20(ctx, base);
	// 826F9C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9C48 size=112
    let mut pc: u32 = 0x826F9C48;
    'dispatch: loop {
        match pc {
            0x826F9C48 => {
    //   block [0x826F9C48..0x826F9CB8)
	// 826F9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9C54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9C58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9C5C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9C64: 390B0E00  addi r8, r11, 0xe00
	ctx.r[8].s64 = ctx.r[11].s64 + 3584;
	// 826F9C68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F9C6C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826F9C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9C80: 386AB734  addi r3, r10, -0x48cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18636;
	// 826F9C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9C94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F9C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9CA4: 4BD6D17D  bl 0x82466e20
	ctx.lr = 0x826F9CA8;
	sub_82466E20(ctx, base);
	// 826F9CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9CB8 size=112
    let mut pc: u32 = 0x826F9CB8;
    'dispatch: loop {
        match pc {
            0x826F9CB8 => {
    //   block [0x826F9CB8..0x826F9D28)
	// 826F9CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9CC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9CC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9CCC: 38AAB704  addi r5, r10, -0x48fc
	ctx.r[5].s64 = ctx.r[10].s64 + -18684;
	// 826F9CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9CD4: 390B0E18  addi r8, r11, 0xe18
	ctx.r[8].s64 = ctx.r[11].s64 + 3608;
	// 826F9CD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9CDC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826F9CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9CF0: 386AB764  addi r3, r10, -0x489c
	ctx.r[3].s64 = ctx.r[10].s64 + -18588;
	// 826F9CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9D14: 4BD6D10D  bl 0x82466e20
	ctx.lr = 0x826F9D18;
	sub_82466E20(ctx, base);
	// 826F9D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9D28 size=72
    let mut pc: u32 = 0x826F9D28;
    'dispatch: loop {
        match pc {
            0x826F9D28 => {
    //   block [0x826F9D28..0x826F9D70)
	// 826F9D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9D34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9D38: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826F9D3C: 38CB93F0  addi r6, r11, -0x6c10
	ctx.r[6].s64 = ctx.r[11].s64 + -27664;
	// 826F9D40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9D44: 388BAC70  addi r4, r11, -0x5390
	ctx.r[4].s64 = ctx.r[11].s64 + -21392;
	// 826F9D48: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F9D4C: 386BB794  addi r3, r11, -0x486c
	ctx.r[3].s64 = ctx.r[11].s64 + -18540;
	// 826F9D50: 4BD81D39  bl 0x8247ba88
	ctx.lr = 0x826F9D54;
	sub_8247BA88(ctx, base);
	// 826F9D54: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826F9D58: 386BCF48  addi r3, r11, -0x30b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12472;
	// 826F9D5C: 4BE38DDD  bl 0x82532b38
	ctx.lr = 0x826F9D60;
	sub_82532B38(ctx, base);
	// 826F9D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826F9D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9D70 size=108
    let mut pc: u32 = 0x826F9D70;
    'dispatch: loop {
        match pc {
            0x826F9D70 => {
    //   block [0x826F9D70..0x826F9DDC)
	// 826F9D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9D7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9D84: 38EB2B58  addi r7, r11, 0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + 11096;
	// 826F9D88: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F9D8C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826F9D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9DA0: 386AB7AC  addi r3, r10, -0x4854
	ctx.r[3].s64 = ctx.r[10].s64 + -18516;
	// 826F9DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9DC8: 4BD6D059  bl 0x82466e20
	ctx.lr = 0x826F9DCC;
	sub_82466E20(ctx, base);
	// 826F9DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F9DE0 size=24
    let mut pc: u32 = 0x826F9DE0;
    'dispatch: loop {
        match pc {
            0x826F9DE0 => {
    //   block [0x826F9DE0..0x826F9DF8)
	// 826F9DE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9DE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826F9DE8: 394A8CF8  addi r10, r10, -0x7308
	ctx.r[10].s64 = ctx.r[10].s64 + -29448;
	// 826F9DEC: 816B2BD0  lwz r11, 0x2bd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11216 as u32) ) } as u64;
	// 826F9DF0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826F9DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9DF8 size=112
    let mut pc: u32 = 0x826F9DF8;
    'dispatch: loop {
        match pc {
            0x826F9DF8 => {
    //   block [0x826F9DF8..0x826F9E68)
	// 826F9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9E04: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9E0C: 392BB384  addi r9, r11, -0x4c7c
	ctx.r[9].s64 = ctx.r[11].s64 + -19580;
	// 826F9E10: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826F9E14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F9E18: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826F9E1C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826F9E20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9E24: 396B8CF8  addi r11, r11, -0x7308
	ctx.r[11].s64 = ctx.r[11].s64 + -29448;
	// 826F9E28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F9E2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9E30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F9E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9E38: 386AB7DC  addi r3, r10, -0x4824
	ctx.r[3].s64 = ctx.r[10].s64 + -18468;
	// 826F9E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F9E40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9E48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F9E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9E50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9E54: 4BD6CFCD  bl 0x82466e20
	ctx.lr = 0x826F9E58;
	sub_82466E20(ctx, base);
	// 826F9E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9E68 size=108
    let mut pc: u32 = 0x826F9E68;
    'dispatch: loop {
        match pc {
            0x826F9E68 => {
    //   block [0x826F9E68..0x826F9ED4)
	// 826F9E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9E74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9E7C: 38EB2BD4  addi r7, r11, 0x2bd4
	ctx.r[7].s64 = ctx.r[11].s64 + 11220;
	// 826F9E80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F9E84: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826F9E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9E8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9E98: 386AB80C  addi r3, r10, -0x47f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18420;
	// 826F9E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9EC0: 4BD6CF61  bl 0x82466e20
	ctx.lr = 0x826F9EC4;
	sub_82466E20(ctx, base);
	// 826F9EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9ED8 size=108
    let mut pc: u32 = 0x826F9ED8;
    'dispatch: loop {
        match pc {
            0x826F9ED8 => {
    //   block [0x826F9ED8..0x826F9F44)
	// 826F9ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9EE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9EEC: 38EB2C04  addi r7, r11, 0x2c04
	ctx.r[7].s64 = ctx.r[11].s64 + 11268;
	// 826F9EF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F9EF4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826F9EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9EFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9F00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9F08: 386AB83C  addi r3, r10, -0x47c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18372;
	// 826F9F0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9F2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9F30: 4BD6CEF1  bl 0x82466e20
	ctx.lr = 0x826F9F34;
	sub_82466E20(ctx, base);
	// 826F9F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9F48 size=112
    let mut pc: u32 = 0x826F9F48;
    'dispatch: loop {
        match pc {
            0x826F9F48 => {
    //   block [0x826F9F48..0x826F9FB8)
	// 826F9F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9F54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9F58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9F5C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826F9F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9F64: 390B2C38  addi r8, r11, 0x2c38
	ctx.r[8].s64 = ctx.r[11].s64 + 11320;
	// 826F9F68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F9F6C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826F9F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9F74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9F80: 386AB86C  addi r3, r10, -0x4794
	ctx.r[3].s64 = ctx.r[10].s64 + -18324;
	// 826F9F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9FA4: 4BD6CE7D  bl 0x82466e20
	ctx.lr = 0x826F9FA8;
	sub_82466E20(ctx, base);
	// 826F9FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9FB8 size=108
    let mut pc: u32 = 0x826F9FB8;
    'dispatch: loop {
        match pc {
            0x826F9FB8 => {
    //   block [0x826F9FB8..0x826FA024)
	// 826F9FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9FC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9FCC: 38EB2C98  addi r7, r11, 0x2c98
	ctx.r[7].s64 = ctx.r[11].s64 + 11416;
	// 826F9FD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F9FD4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826F9FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9FDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9FE8: 386AB89C  addi r3, r10, -0x4764
	ctx.r[3].s64 = ctx.r[10].s64 + -18276;
	// 826F9FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA010: 4BD6CE11  bl 0x82466e20
	ctx.lr = 0x826FA014;
	sub_82466E20(ctx, base);
	// 826FA014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA028 size=112
    let mut pc: u32 = 0x826FA028;
    'dispatch: loop {
        match pc {
            0x826FA028 => {
    //   block [0x826FA028..0x826FA098)
	// 826FA028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA038: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA03C: 38AAB86C  addi r5, r10, -0x4794
	ctx.r[5].s64 = ctx.r[10].s64 + -18324;
	// 826FA040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA044: 390B2D10  addi r8, r11, 0x2d10
	ctx.r[8].s64 = ctx.r[11].s64 + 11536;
	// 826FA048: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826FA04C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826FA050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA054: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA060: 386AB8CC  addi r3, r10, -0x4734
	ctx.r[3].s64 = ctx.r[10].s64 + -18228;
	// 826FA064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FA068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA084: 4BD6CD9D  bl 0x82466e20
	ctx.lr = 0x826FA088;
	sub_82466E20(ctx, base);
	// 826FA088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA098 size=112
    let mut pc: u32 = 0x826FA098;
    'dispatch: loop {
        match pc {
            0x826FA098 => {
    //   block [0x826FA098..0x826FA108)
	// 826FA098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA0A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA0A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA0AC: 38AAB86C  addi r5, r10, -0x4794
	ctx.r[5].s64 = ctx.r[10].s64 + -18324;
	// 826FA0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA0B4: 390B2DB8  addi r8, r11, 0x2db8
	ctx.r[8].s64 = ctx.r[11].s64 + 11704;
	// 826FA0B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FA0BC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826FA0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA0C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA0D0: 386AB8FC  addi r3, r10, -0x4704
	ctx.r[3].s64 = ctx.r[10].s64 + -18180;
	// 826FA0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FA0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA0F4: 4BD6CD2D  bl 0x82466e20
	ctx.lr = 0x826FA0F8;
	sub_82466E20(ctx, base);
	// 826FA0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA108 size=108
    let mut pc: u32 = 0x826FA108;
    'dispatch: loop {
        match pc {
            0x826FA108 => {
    //   block [0x826FA108..0x826FA174)
	// 826FA108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA114: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA11C: 38EB2DD0  addi r7, r11, 0x2dd0
	ctx.r[7].s64 = ctx.r[11].s64 + 11728;
	// 826FA120: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826FA124: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826FA128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA12C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA138: 386AB92C  addi r3, r10, -0x46d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18132;
	// 826FA13C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA160: 4BD6CCC1  bl 0x82466e20
	ctx.lr = 0x826FA164;
	sub_82466E20(ctx, base);
	// 826FA164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA178 size=112
    let mut pc: u32 = 0x826FA178;
    'dispatch: loop {
        match pc {
            0x826FA178 => {
    //   block [0x826FA178..0x826FA1E8)
	// 826FA178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA188: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA18C: 38AAB86C  addi r5, r10, -0x4794
	ctx.r[5].s64 = ctx.r[10].s64 + -18324;
	// 826FA190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA194: 390B2E48  addi r8, r11, 0x2e48
	ctx.r[8].s64 = ctx.r[11].s64 + 11848;
	// 826FA198: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826FA19C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826FA1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA1A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA1B0: 386AB95C  addi r3, r10, -0x46a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18084;
	// 826FA1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FA1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA1D4: 4BD6CC4D  bl 0x82466e20
	ctx.lr = 0x826FA1D8;
	sub_82466E20(ctx, base);
	// 826FA1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA1E8 size=108
    let mut pc: u32 = 0x826FA1E8;
    'dispatch: loop {
        match pc {
            0x826FA1E8 => {
    //   block [0x826FA1E8..0x826FA254)
	// 826FA1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA1F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA1FC: 38EB2EF0  addi r7, r11, 0x2ef0
	ctx.r[7].s64 = ctx.r[11].s64 + 12016;
	// 826FA200: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FA204: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826FA208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA20C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA218: 386AB98C  addi r3, r10, -0x4674
	ctx.r[3].s64 = ctx.r[10].s64 + -18036;
	// 826FA21C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA240: 4BD6CBE1  bl 0x82466e20
	ctx.lr = 0x826FA244;
	sub_82466E20(ctx, base);
	// 826FA244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA258 size=108
    let mut pc: u32 = 0x826FA258;
    'dispatch: loop {
        match pc {
            0x826FA258 => {
    //   block [0x826FA258..0x826FA2C4)
	// 826FA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA264: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA26C: 38EB2F08  addi r7, r11, 0x2f08
	ctx.r[7].s64 = ctx.r[11].s64 + 12040;
	// 826FA270: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FA274: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826FA278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA27C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA288: 386AB9BC  addi r3, r10, -0x4644
	ctx.r[3].s64 = ctx.r[10].s64 + -17988;
	// 826FA28C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA2A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA2AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA2B0: 4BD6CB71  bl 0x82466e20
	ctx.lr = 0x826FA2B4;
	sub_82466E20(ctx, base);
	// 826FA2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA2C8 size=116
    let mut pc: u32 = 0x826FA2C8;
    'dispatch: loop {
        match pc {
            0x826FA2C8 => {
    //   block [0x826FA2C8..0x826FA33C)
	// 826FA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA2D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA2D8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FA2DC: 390B2F68  addi r8, r11, 0x2f68
	ctx.r[8].s64 = ctx.r[11].s64 + 12136;
	// 826FA2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA2E4: 392AB3C0  addi r9, r10, -0x4c40
	ctx.r[9].s64 = ctx.r[10].s64 + -19520;
	// 826FA2E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA2EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826FA2F0: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FA2F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA2FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA30C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FA310: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826FA314: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FA318: 386BB9EC  addi r3, r11, -0x4614
	ctx.r[3].s64 = ctx.r[11].s64 + -17940;
	// 826FA31C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FA320: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA328: 4BD6CAF9  bl 0x82466e20
	ctx.lr = 0x826FA32C;
	sub_82466E20(ctx, base);
	// 826FA32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA340 size=108
    let mut pc: u32 = 0x826FA340;
    'dispatch: loop {
        match pc {
            0x826FA340 => {
    //   block [0x826FA340..0x826FA3AC)
	// 826FA340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA34C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA354: 38EB2F80  addi r7, r11, 0x2f80
	ctx.r[7].s64 = ctx.r[11].s64 + 12160;
	// 826FA358: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FA35C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826FA360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA370: 386ABA1C  addi r3, r10, -0x45e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17892;
	// 826FA374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA398: 4BD6CA89  bl 0x82466e20
	ctx.lr = 0x826FA39C;
	sub_82466E20(ctx, base);
	// 826FA39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA3B0 size=108
    let mut pc: u32 = 0x826FA3B0;
    'dispatch: loop {
        match pc {
            0x826FA3B0 => {
    //   block [0x826FA3B0..0x826FA41C)
	// 826FA3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA3BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA3C4: 38EB2FC8  addi r7, r11, 0x2fc8
	ctx.r[7].s64 = ctx.r[11].s64 + 12232;
	// 826FA3C8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826FA3CC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826FA3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA3D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA3E0: 386ABA4C  addi r3, r10, -0x45b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17844;
	// 826FA3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA408: 4BD6CA19  bl 0x82466e20
	ctx.lr = 0x826FA40C;
	sub_82466E20(ctx, base);
	// 826FA40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA420 size=108
    let mut pc: u32 = 0x826FA420;
    'dispatch: loop {
        match pc {
            0x826FA420 => {
    //   block [0x826FA420..0x826FA48C)
	// 826FA420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA42C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA434: 38EB3058  addi r7, r11, 0x3058
	ctx.r[7].s64 = ctx.r[11].s64 + 12376;
	// 826FA438: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826FA43C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826FA440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA450: 386ABA7C  addi r3, r10, -0x4584
	ctx.r[3].s64 = ctx.r[10].s64 + -17796;
	// 826FA454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA478: 4BD6C9A9  bl 0x82466e20
	ctx.lr = 0x826FA47C;
	sub_82466E20(ctx, base);
	// 826FA47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA490 size=100
    let mut pc: u32 = 0x826FA490;
    'dispatch: loop {
        match pc {
            0x826FA490 => {
    //   block [0x826FA490..0x826FA4F4)
	// 826FA490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA49C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA4A4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FA4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA4B0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826FA4B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA4C4: 386ABAAC  addi r3, r10, -0x4554
	ctx.r[3].s64 = ctx.r[10].s64 + -17748;
	// 826FA4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA4D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FA4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA4D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FA4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA4E0: 4BD6C941  bl 0x82466e20
	ctx.lr = 0x826FA4E4;
	sub_82466E20(ctx, base);
	// 826FA4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA4F8 size=112
    let mut pc: u32 = 0x826FA4F8;
    'dispatch: loop {
        match pc {
            0x826FA4F8 => {
    //   block [0x826FA4F8..0x826FA568)
	// 826FA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA508: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA50C: 38AABAAC  addi r5, r10, -0x4554
	ctx.r[5].s64 = ctx.r[10].s64 + -17748;
	// 826FA510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA514: 390B30E8  addi r8, r11, 0x30e8
	ctx.r[8].s64 = ctx.r[11].s64 + 12520;
	// 826FA518: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FA51C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826FA520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA530: 386ABADC  addi r3, r10, -0x4524
	ctx.r[3].s64 = ctx.r[10].s64 + -17700;
	// 826FA534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FA538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA554: 4BD6C8CD  bl 0x82466e20
	ctx.lr = 0x826FA558;
	sub_82466E20(ctx, base);
	// 826FA558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA568 size=108
    let mut pc: u32 = 0x826FA568;
    'dispatch: loop {
        match pc {
            0x826FA568 => {
    //   block [0x826FA568..0x826FA5D4)
	// 826FA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA574: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA57C: 38EB3148  addi r7, r11, 0x3148
	ctx.r[7].s64 = ctx.r[11].s64 + 12616;
	// 826FA580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FA584: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826FA588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA58C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA598: 386ABB0C  addi r3, r10, -0x44f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17652;
	// 826FA59C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA5A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA5B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA5B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA5BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA5C0: 4BD6C861  bl 0x82466e20
	ctx.lr = 0x826FA5C4;
	sub_82466E20(ctx, base);
	// 826FA5C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA5C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA5CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA5D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA5D8 size=108
    let mut pc: u32 = 0x826FA5D8;
    'dispatch: loop {
        match pc {
            0x826FA5D8 => {
    //   block [0x826FA5D8..0x826FA644)
	// 826FA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA5E4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA5EC: 38EB3178  addi r7, r11, 0x3178
	ctx.r[7].s64 = ctx.r[11].s64 + 12664;
	// 826FA5F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FA5F4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826FA5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA5FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA608: 386ABB3C  addi r3, r10, -0x44c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17604;
	// 826FA60C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA630: 4BD6C7F1  bl 0x82466e20
	ctx.lr = 0x826FA634;
	sub_82466E20(ctx, base);
	// 826FA634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA648 size=108
    let mut pc: u32 = 0x826FA648;
    'dispatch: loop {
        match pc {
            0x826FA648 => {
    //   block [0x826FA648..0x826FA6B4)
	// 826FA648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA654: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA65C: 38EB31D8  addi r7, r11, 0x31d8
	ctx.r[7].s64 = ctx.r[11].s64 + 12760;
	// 826FA660: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FA664: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826FA668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA66C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA678: 386ABB6C  addi r3, r10, -0x4494
	ctx.r[3].s64 = ctx.r[10].s64 + -17556;
	// 826FA67C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA6A0: 4BD6C781  bl 0x82466e20
	ctx.lr = 0x826FA6A4;
	sub_82466E20(ctx, base);
	// 826FA6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA6B8 size=112
    let mut pc: u32 = 0x826FA6B8;
    'dispatch: loop {
        match pc {
            0x826FA6B8 => {
    //   block [0x826FA6B8..0x826FA728)
	// 826FA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA6C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FA6C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA6CC: 392AB3F4  addi r9, r10, -0x4c0c
	ctx.r[9].s64 = ctx.r[10].s64 + -19468;
	// 826FA6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA6D4: 390B3240  addi r8, r11, 0x3240
	ctx.r[8].s64 = ctx.r[11].s64 + 12864;
	// 826FA6D8: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826FA6DC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826FA6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA6E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA6E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA6F0: 386ABB9C  addi r3, r10, -0x4464
	ctx.r[3].s64 = ctx.r[10].s64 + -17508;
	// 826FA6F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FA6F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FA6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA714: 4BD6C70D  bl 0x82466e20
	ctx.lr = 0x826FA718;
	sub_82466E20(ctx, base);
	// 826FA718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA728 size=108
    let mut pc: u32 = 0x826FA728;
    'dispatch: loop {
        match pc {
            0x826FA728 => {
    //   block [0x826FA728..0x826FA794)
	// 826FA728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA734: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA73C: 38EB3348  addi r7, r11, 0x3348
	ctx.r[7].s64 = ctx.r[11].s64 + 13128;
	// 826FA740: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826FA744: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826FA748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA74C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA758: 386ABBCC  addi r3, r10, -0x4434
	ctx.r[3].s64 = ctx.r[10].s64 + -17460;
	// 826FA75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA780: 4BD6C6A1  bl 0x82466e20
	ctx.lr = 0x826FA784;
	sub_82466E20(ctx, base);
	// 826FA784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA798 size=108
    let mut pc: u32 = 0x826FA798;
    'dispatch: loop {
        match pc {
            0x826FA798 => {
    //   block [0x826FA798..0x826FA804)
	// 826FA798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA7A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA7AC: 38EB3480  addi r7, r11, 0x3480
	ctx.r[7].s64 = ctx.r[11].s64 + 13440;
	// 826FA7B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FA7B4: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826FA7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA7BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA7C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA7C8: 386ABBFC  addi r3, r10, -0x4404
	ctx.r[3].s64 = ctx.r[10].s64 + -17412;
	// 826FA7CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA7F0: 4BD6C631  bl 0x82466e20
	ctx.lr = 0x826FA7F4;
	sub_82466E20(ctx, base);
	// 826FA7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA808 size=108
    let mut pc: u32 = 0x826FA808;
    'dispatch: loop {
        match pc {
            0x826FA808 => {
    //   block [0x826FA808..0x826FA874)
	// 826FA808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA814: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FA81C: 38EB3498  addi r7, r11, 0x3498
	ctx.r[7].s64 = ctx.r[11].s64 + 13464;
	// 826FA820: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FA824: 388A5948  addi r4, r10, 0x5948
	ctx.r[4].s64 = ctx.r[10].s64 + 22856;
	// 826FA828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA82C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA838: 386ABC2C  addi r3, r10, -0x43d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17364;
	// 826FA83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA860: 4BD6C5C1  bl 0x82466e20
	ctx.lr = 0x826FA864;
	sub_82466E20(ctx, base);
	// 826FA864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA878 size=108
    let mut pc: u32 = 0x826FA878;
    'dispatch: loop {
        match pc {
            0x826FA878 => {
    //   block [0x826FA878..0x826FA8E4)
	// 826FA878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA884: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FA88C: 38EB34B0  addi r7, r11, 0x34b0
	ctx.r[7].s64 = ctx.r[11].s64 + 13488;
	// 826FA890: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FA894: 388A595C  addi r4, r10, 0x595c
	ctx.r[4].s64 = ctx.r[10].s64 + 22876;
	// 826FA898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA89C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA8A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FA8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA8A8: 386ABC5C  addi r3, r10, -0x43a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17316;
	// 826FA8AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FA8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA8BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA8CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FA8D0: 4BD6C551  bl 0x82466e20
	ctx.lr = 0x826FA8D4;
	sub_82466E20(ctx, base);
	// 826FA8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA8E8 size=100
    let mut pc: u32 = 0x826FA8E8;
    'dispatch: loop {
        match pc {
            0x826FA8E8 => {
    //   block [0x826FA8E8..0x826FA94C)
	// 826FA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA8F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA8FC: 38AAC16C  addi r5, r10, -0x3e94
	ctx.r[5].s64 = ctx.r[10].s64 + -16020;
	// 826FA900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA908: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826FA90C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA91C: 386ABC8C  addi r3, r10, -0x4374
	ctx.r[3].s64 = ctx.r[10].s64 + -17268;
	// 826FA920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FA92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FA934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA938: 4BD6C4E9  bl 0x82466e20
	ctx.lr = 0x826FA93C;
	sub_82466E20(ctx, base);
	// 826FA93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA950 size=112
    let mut pc: u32 = 0x826FA950;
    'dispatch: loop {
        match pc {
            0x826FA950 => {
    //   block [0x826FA950..0x826FA9C0)
	// 826FA950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA95C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA960: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FA964: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FA968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA96C: 390B34E0  addi r8, r11, 0x34e0
	ctx.r[8].s64 = ctx.r[11].s64 + 13536;
	// 826FA970: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826FA974: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826FA978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA97C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FA984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FA988: 386ABCBC  addi r3, r10, -0x4344
	ctx.r[3].s64 = ctx.r[10].s64 + -17220;
	// 826FA98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FA990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FA994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FA9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA9AC: 4BD6C475  bl 0x82466e20
	ctx.lr = 0x826FA9B0;
	sub_82466E20(ctx, base);
	// 826FA9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FA9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FA9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FA9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FA9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FA9C0 size=100
    let mut pc: u32 = 0x826FA9C0;
    'dispatch: loop {
        match pc {
            0x826FA9C0 => {
    //   block [0x826FA9C0..0x826FAA24)
	// 826FA9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FA9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FA9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FA9CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FA9D4: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 826FA9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FA9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FA9E0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826FA9E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FA9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FA9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FA9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FA9F4: 386ABCEC  addi r3, r10, -0x4314
	ctx.r[3].s64 = ctx.r[10].s64 + -17172;
	// 826FA9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FA9FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAA00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FAA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAA08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FAA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAA10: 4BD6C411  bl 0x82466e20
	ctx.lr = 0x826FAA14;
	sub_82466E20(ctx, base);
	// 826FAA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAA28 size=108
    let mut pc: u32 = 0x826FAA28;
    'dispatch: loop {
        match pc {
            0x826FAA28 => {
    //   block [0x826FAA28..0x826FAA94)
	// 826FAA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAA34: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAA3C: 38EB3570  addi r7, r11, 0x3570
	ctx.r[7].s64 = ctx.r[11].s64 + 13680;
	// 826FAA40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FAA44: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826FAA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAA4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAA50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FAA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAA58: 386ABD1C  addi r3, r10, -0x42e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17124;
	// 826FAA5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FAA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FAA80: 4BD6C3A1  bl 0x82466e20
	ctx.lr = 0x826FAA84;
	sub_82466E20(ctx, base);
	// 826FAA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAA98 size=112
    let mut pc: u32 = 0x826FAA98;
    'dispatch: loop {
        match pc {
            0x826FAA98 => {
    //   block [0x826FAA98..0x826FAB08)
	// 826FAA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAAA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAAA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAAAC: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FAAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAAB4: 390B35B8  addi r8, r11, 0x35b8
	ctx.r[8].s64 = ctx.r[11].s64 + 13752;
	// 826FAAB8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826FAABC: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826FAAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAAC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAAC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAAD0: 386ABD4C  addi r3, r10, -0x42b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17076;
	// 826FAAD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAAF4: 4BD6C32D  bl 0x82466e20
	ctx.lr = 0x826FAAF8;
	sub_82466E20(ctx, base);
	// 826FAAF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAB08 size=108
    let mut pc: u32 = 0x826FAB08;
    'dispatch: loop {
        match pc {
            0x826FAB08 => {
    //   block [0x826FAB08..0x826FAB74)
	// 826FAB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAB14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAB1C: 38EB3630  addi r7, r11, 0x3630
	ctx.r[7].s64 = ctx.r[11].s64 + 13872;
	// 826FAB20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FAB24: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826FAB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAB2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FAB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAB38: 386ABD7C  addi r3, r10, -0x4284
	ctx.r[3].s64 = ctx.r[10].s64 + -17028;
	// 826FAB3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FAB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FAB60: 4BD6C2C1  bl 0x82466e20
	ctx.lr = 0x826FAB64;
	sub_82466E20(ctx, base);
	// 826FAB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FAB78 size=24
    let mut pc: u32 = 0x826FAB78;
    'dispatch: loop {
        match pc {
            0x826FAB78 => {
    //   block [0x826FAB78..0x826FAB90)
	// 826FAB78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAB7C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FAB80: 394A8D40  addi r10, r10, -0x72c0
	ctx.r[10].s64 = ctx.r[10].s64 + -29376;
	// 826FAB84: 816B323C  lwz r11, 0x323c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12860 as u32) ) } as u64;
	// 826FAB88: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826FAB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAB90 size=116
    let mut pc: u32 = 0x826FAB90;
    'dispatch: loop {
        match pc {
            0x826FAB90 => {
    //   block [0x826FAB90..0x826FAC04)
	// 826FAB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAB9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FABA0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FABA4: 392BB420  addi r9, r11, -0x4be0
	ctx.r[9].s64 = ctx.r[11].s64 + -19424;
	// 826FABA8: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FABAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FABB0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826FABB4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 826FABB8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FABBC: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826FABC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FABC4: 396B8D40  addi r11, r11, -0x72c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29376;
	// 826FABC8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826FABCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FABD0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826FABD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FABD8: 386ABDAC  addi r3, r10, -0x4254
	ctx.r[3].s64 = ctx.r[10].s64 + -16980;
	// 826FABDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FABE0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826FABE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FABE8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826FABEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FABF0: 4BD6C231  bl 0x82466e20
	ctx.lr = 0x826FABF4;
	sub_82466E20(ctx, base);
	// 826FABF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FABF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FABFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAC08 size=112
    let mut pc: u32 = 0x826FAC08;
    'dispatch: loop {
        match pc {
            0x826FAC08 => {
    //   block [0x826FAC08..0x826FAC78)
	// 826FAC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAC14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAC18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAC1C: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FAC20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FAC24: 390B3678  addi r8, r11, 0x3678
	ctx.r[8].s64 = ctx.r[11].s64 + 13944;
	// 826FAC28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FAC2C: 388A596C  addi r4, r10, 0x596c
	ctx.r[4].s64 = ctx.r[10].s64 + 22892;
	// 826FAC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAC34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAC40: 386ABDDC  addi r3, r10, -0x4224
	ctx.r[3].s64 = ctx.r[10].s64 + -16932;
	// 826FAC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAC64: 4BD6C1BD  bl 0x82466e20
	ctx.lr = 0x826FAC68;
	sub_82466E20(ctx, base);
	// 826FAC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAC78 size=112
    let mut pc: u32 = 0x826FAC78;
    'dispatch: loop {
        match pc {
            0x826FAC78 => {
    //   block [0x826FAC78..0x826FACE8)
	// 826FAC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAC84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAC88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAC8C: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FAC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAC94: 390B36A8  addi r8, r11, 0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13992;
	// 826FAC98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FAC9C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826FACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FACA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FACA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FACAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FACB0: 386ABE0C  addi r3, r10, -0x41f4
	ctx.r[3].s64 = ctx.r[10].s64 + -16884;
	// 826FACB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FACB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FACBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FACC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FACC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FACC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FACCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FACD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FACD4: 4BD6C14D  bl 0x82466e20
	ctx.lr = 0x826FACD8;
	sub_82466E20(ctx, base);
	// 826FACD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FACE8 size=100
    let mut pc: u32 = 0x826FACE8;
    'dispatch: loop {
        match pc {
            0x826FACE8 => {
    //   block [0x826FACE8..0x826FAD4C)
	// 826FACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FACF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FACF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FACFC: 38AAC16C  addi r5, r10, -0x3e94
	ctx.r[5].s64 = ctx.r[10].s64 + -16020;
	// 826FAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAD08: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826FAD0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAD10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAD18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAD1C: 386ABE3C  addi r3, r10, -0x41c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	// 826FAD20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAD28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FAD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAD30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FAD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAD38: 4BD6C0E9  bl 0x82466e20
	ctx.lr = 0x826FAD3C;
	sub_82466E20(ctx, base);
	// 826FAD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAD50 size=116
    let mut pc: u32 = 0x826FAD50;
    'dispatch: loop {
        match pc {
            0x826FAD50 => {
    //   block [0x826FAD50..0x826FADC4)
	// 826FAD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAD5C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FAD60: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826FAD64: 390A36C0  addi r8, r10, 0x36c0
	ctx.r[8].s64 = ctx.r[10].s64 + 14016;
	// 826FAD68: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAD6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FAD70: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FAD74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAD78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FAD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAD84: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826FAD88: 396BB468  addi r11, r11, -0x4b98
	ctx.r[11].s64 = ctx.r[11].s64 + -19352;
	// 826FAD8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAD90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAD94: 386ABE6C  addi r3, r10, -0x4194
	ctx.r[3].s64 = ctx.r[10].s64 + -16788;
	// 826FAD98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FAD9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FADA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FADA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FADA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FADAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FADB0: 4BD6C071  bl 0x82466e20
	ctx.lr = 0x826FADB4;
	sub_82466E20(ctx, base);
	// 826FADB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FADB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FADBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FADC8 size=112
    let mut pc: u32 = 0x826FADC8;
    'dispatch: loop {
        match pc {
            0x826FADC8 => {
    //   block [0x826FADC8..0x826FAE38)
	// 826FADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FADD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FADD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FADDC: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FADE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FADE4: 390B3858  addi r8, r11, 0x3858
	ctx.r[8].s64 = ctx.r[11].s64 + 14424;
	// 826FADE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FADEC: 388A5998  addi r4, r10, 0x5998
	ctx.r[4].s64 = ctx.r[10].s64 + 22936;
	// 826FADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FADF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FADF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FADFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAE00: 386ABE9C  addi r3, r10, -0x4164
	ctx.r[3].s64 = ctx.r[10].s64 + -16740;
	// 826FAE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAE24: 4BD6BFFD  bl 0x82466e20
	ctx.lr = 0x826FAE28;
	sub_82466E20(ctx, base);
	// 826FAE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAE38 size=112
    let mut pc: u32 = 0x826FAE38;
    'dispatch: loop {
        match pc {
            0x826FAE38 => {
    //   block [0x826FAE38..0x826FAEA8)
	// 826FAE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAE44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAE48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAE4C: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FAE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAE54: 390B3870  addi r8, r11, 0x3870
	ctx.r[8].s64 = ctx.r[11].s64 + 14448;
	// 826FAE58: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826FAE5C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826FAE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAE64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAE70: 386ABECC  addi r3, r10, -0x4134
	ctx.r[3].s64 = ctx.r[10].s64 + -16692;
	// 826FAE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAE94: 4BD6BF8D  bl 0x82466e20
	ctx.lr = 0x826FAE98;
	sub_82466E20(ctx, base);
	// 826FAE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAEA8 size=112
    let mut pc: u32 = 0x826FAEA8;
    'dispatch: loop {
        match pc {
            0x826FAEA8 => {
    //   block [0x826FAEA8..0x826FAF18)
	// 826FAEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAEB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAEB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAEBC: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FAEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAEC4: 390B3918  addi r8, r11, 0x3918
	ctx.r[8].s64 = ctx.r[11].s64 + 14616;
	// 826FAEC8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826FAECC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826FAED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAED4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAEE0: 386ABEFC  addi r3, r10, -0x4104
	ctx.r[3].s64 = ctx.r[10].s64 + -16644;
	// 826FAEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAF04: 4BD6BF1D  bl 0x82466e20
	ctx.lr = 0x826FAF08;
	sub_82466E20(ctx, base);
	// 826FAF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAF18 size=112
    let mut pc: u32 = 0x826FAF18;
    'dispatch: loop {
        match pc {
            0x826FAF18 => {
    //   block [0x826FAF18..0x826FAF88)
	// 826FAF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAF24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAF28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAF2C: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FAF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FAF34: 390B39F0  addi r8, r11, 0x39f0
	ctx.r[8].s64 = ctx.r[11].s64 + 14832;
	// 826FAF38: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826FAF3C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826FAF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAF44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAF48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAF50: 386ABF2C  addi r3, r10, -0x40d4
	ctx.r[3].s64 = ctx.r[10].s64 + -16596;
	// 826FAF54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAF5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAF74: 4BD6BEAD  bl 0x82466e20
	ctx.lr = 0x826FAF78;
	sub_82466E20(ctx, base);
	// 826FAF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FAF88 size=112
    let mut pc: u32 = 0x826FAF88;
    'dispatch: loop {
        match pc {
            0x826FAF88 => {
    //   block [0x826FAF88..0x826FAFF8)
	// 826FAF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FAF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FAF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FAF94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAF98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAF9C: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FAFA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FAFA4: 390B3AF8  addi r8, r11, 0x3af8
	ctx.r[8].s64 = ctx.r[11].s64 + 15096;
	// 826FAFA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FAFAC: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826FAFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FAFB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FAFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FAFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FAFC0: 386ABF5C  addi r3, r10, -0x40a4
	ctx.r[3].s64 = ctx.r[10].s64 + -16548;
	// 826FAFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FAFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FAFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FAFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FAFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FAFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FAFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FAFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FAFE4: 4BD6BE3D  bl 0x82466e20
	ctx.lr = 0x826FAFE8;
	sub_82466E20(ctx, base);
	// 826FAFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FAFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FAFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FAFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FAFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FAFF8 size=24
    let mut pc: u32 = 0x826FAFF8;
    'dispatch: loop {
        match pc {
            0x826FAFF8 => {
    //   block [0x826FAFF8..0x826FB010)
	// 826FAFF8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FAFFC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FB000: 394A8E60  addi r10, r10, -0x71a0
	ctx.r[10].s64 = ctx.r[10].s64 + -29088;
	// 826FB004: 816B3B28  lwz r11, 0x3b28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15144 as u32) ) } as u64;
	// 826FB008: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826FB00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB010 size=116
    let mut pc: u32 = 0x826FB010;
    'dispatch: loop {
        match pc {
            0x826FB010 => {
    //   block [0x826FB010..0x826FB084)
	// 826FB010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB01C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB020: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB024: 392BB4C4  addi r9, r11, -0x4b3c
	ctx.r[9].s64 = ctx.r[11].s64 + -19260;
	// 826FB028: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB02C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FB030: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826FB034: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826FB038: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FB03C: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826FB040: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB044: 396B8E60  addi r11, r11, -0x71a0
	ctx.r[11].s64 = ctx.r[11].s64 + -29088;
	// 826FB048: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826FB04C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB050: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826FB054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB058: 386ABF8C  addi r3, r10, -0x4074
	ctx.r[3].s64 = ctx.r[10].s64 + -16500;
	// 826FB05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FB060: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826FB064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB068: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826FB06C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FB070: 4BD6BDB1  bl 0x82466e20
	ctx.lr = 0x826FB074;
	sub_82466E20(ctx, base);
	// 826FB074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB088 size=116
    let mut pc: u32 = 0x826FB088;
    'dispatch: loop {
        match pc {
            0x826FB088 => {
    //   block [0x826FB088..0x826FB0FC)
	// 826FB088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB094: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FB098: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826FB09C: 390A3B2C  addi r8, r10, 0x3b2c
	ctx.r[8].s64 = ctx.r[10].s64 + 15148;
	// 826FB0A0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB0A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB0A8: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FB0AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB0B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FB0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB0BC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826FB0C0: 396BB534  addi r11, r11, -0x4acc
	ctx.r[11].s64 = ctx.r[11].s64 + -19148;
	// 826FB0C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB0C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB0CC: 386ABFBC  addi r3, r10, -0x4044
	ctx.r[3].s64 = ctx.r[10].s64 + -16452;
	// 826FB0D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FB0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB0D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FB0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB0E8: 4BD6BD39  bl 0x82466e20
	ctx.lr = 0x826FB0EC;
	sub_82466E20(ctx, base);
	// 826FB0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB100 size=108
    let mut pc: u32 = 0x826FB100;
    'dispatch: loop {
        match pc {
            0x826FB100 => {
    //   block [0x826FB100..0x826FB16C)
	// 826FB100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB10C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB114: 38EB3B60  addi r7, r11, 0x3b60
	ctx.r[7].s64 = ctx.r[11].s64 + 15200;
	// 826FB118: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826FB11C: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826FB120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB124: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB130: 386ABFEC  addi r3, r10, -0x4014
	ctx.r[3].s64 = ctx.r[10].s64 + -16404;
	// 826FB134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB158: 4BD6BCC9  bl 0x82466e20
	ctx.lr = 0x826FB15C;
	sub_82466E20(ctx, base);
	// 826FB15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FB170 size=24
    let mut pc: u32 = 0x826FB170;
    'dispatch: loop {
        match pc {
            0x826FB170 => {
    //   block [0x826FB170..0x826FB188)
	// 826FB170: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB174: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FB178: 394A8FF8  addi r10, r10, -0x7008
	ctx.r[10].s64 = ctx.r[10].s64 + -28680;
	// 826FB17C: 816B3B5C  lwz r11, 0x3b5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15196 as u32) ) } as u64;
	// 826FB180: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826FB184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB188 size=116
    let mut pc: u32 = 0x826FB188;
    'dispatch: loop {
        match pc {
            0x826FB188 => {
    //   block [0x826FB188..0x826FB1FC)
	// 826FB188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB194: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB198: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB19C: 392BB558  addi r9, r11, -0x4aa8
	ctx.r[9].s64 = ctx.r[11].s64 + -19112;
	// 826FB1A0: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB1A8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826FB1AC: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826FB1B0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FB1B4: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826FB1B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB1BC: 396B8FF8  addi r11, r11, -0x7008
	ctx.r[11].s64 = ctx.r[11].s64 + -28680;
	// 826FB1C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826FB1C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB1C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826FB1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB1D0: 386AC01C  addi r3, r10, -0x3fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -16356;
	// 826FB1D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FB1D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826FB1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB1E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826FB1E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FB1E8: 4BD6BC39  bl 0x82466e20
	ctx.lr = 0x826FB1EC;
	sub_82466E20(ctx, base);
	// 826FB1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB200 size=112
    let mut pc: u32 = 0x826FB200;
    'dispatch: loop {
        match pc {
            0x826FB200 => {
    //   block [0x826FB200..0x826FB270)
	// 826FB200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB20C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB210: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB214: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB21C: 390B3BF0  addi r8, r11, 0x3bf0
	ctx.r[8].s64 = ctx.r[11].s64 + 15344;
	// 826FB220: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FB224: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826FB228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB22C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB238: 386AC04C  addi r3, r10, -0x3fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -16308;
	// 826FB23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FB240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB25C: 4BD6BBC5  bl 0x82466e20
	ctx.lr = 0x826FB260;
	sub_82466E20(ctx, base);
	// 826FB260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB270 size=116
    let mut pc: u32 = 0x826FB270;
    'dispatch: loop {
        match pc {
            0x826FB270 => {
    //   block [0x826FB270..0x826FB2E4)
	// 826FB270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB27C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FB280: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FB284: 390A3C20  addi r8, r10, 0x3c20
	ctx.r[8].s64 = ctx.r[10].s64 + 15392;
	// 826FB288: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB28C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB290: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB294: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB298: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FB29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB2A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB2A4: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826FB2A8: 396BB5B0  addi r11, r11, -0x4a50
	ctx.r[11].s64 = ctx.r[11].s64 + -19024;
	// 826FB2AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB2B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB2B4: 386AC07C  addi r3, r10, -0x3f84
	ctx.r[3].s64 = ctx.r[10].s64 + -16260;
	// 826FB2B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FB2BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB2C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FB2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB2D0: 4BD6BB51  bl 0x82466e20
	ctx.lr = 0x826FB2D4;
	sub_82466E20(ctx, base);
	// 826FB2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB2E8 size=108
    let mut pc: u32 = 0x826FB2E8;
    'dispatch: loop {
        match pc {
            0x826FB2E8 => {
    //   block [0x826FB2E8..0x826FB354)
	// 826FB2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB2F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB2F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB2FC: 38EB3C98  addi r7, r11, 0x3c98
	ctx.r[7].s64 = ctx.r[11].s64 + 15512;
	// 826FB300: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826FB304: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826FB308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB30C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB318: 386AC0AC  addi r3, r10, -0x3f54
	ctx.r[3].s64 = ctx.r[10].s64 + -16212;
	// 826FB31C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB340: 4BD6BAE1  bl 0x82466e20
	ctx.lr = 0x826FB344;
	sub_82466E20(ctx, base);
	// 826FB344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB358 size=116
    let mut pc: u32 = 0x826FB358;
    'dispatch: loop {
        match pc {
            0x826FB358 => {
    //   block [0x826FB358..0x826FB3CC)
	// 826FB358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB364: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FB368: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826FB36C: 390A3DD0  addi r8, r10, 0x3dd0
	ctx.r[8].s64 = ctx.r[10].s64 + 15824;
	// 826FB370: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB374: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB378: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB37C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB380: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FB384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB38C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826FB390: 396BB5C8  addi r11, r11, -0x4a38
	ctx.r[11].s64 = ctx.r[11].s64 + -19000;
	// 826FB394: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB39C: 386AC0DC  addi r3, r10, -0x3f24
	ctx.r[3].s64 = ctx.r[10].s64 + -16164;
	// 826FB3A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FB3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB3A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FB3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB3B8: 4BD6BA69  bl 0x82466e20
	ctx.lr = 0x826FB3BC;
	sub_82466E20(ctx, base);
	// 826FB3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB3D0 size=112
    let mut pc: u32 = 0x826FB3D0;
    'dispatch: loop {
        match pc {
            0x826FB3D0 => {
    //   block [0x826FB3D0..0x826FB440)
	// 826FB3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB3DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB3E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB3E4: 38AABE3C  addi r5, r10, -0x41c4
	ctx.r[5].s64 = ctx.r[10].s64 + -16836;
	// 826FB3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB3EC: 390B3EC0  addi r8, r11, 0x3ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 16064;
	// 826FB3F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FB3F4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826FB3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB3FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB408: 386AC10C  addi r3, r10, -0x3ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -16116;
	// 826FB40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FB410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB42C: 4BD6B9F5  bl 0x82466e20
	ctx.lr = 0x826FB430;
	sub_82466E20(ctx, base);
	// 826FB430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB440 size=112
    let mut pc: u32 = 0x826FB440;
    'dispatch: loop {
        match pc {
            0x826FB440 => {
    //   block [0x826FB440..0x826FB4B0)
	// 826FB440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB44C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB450: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB454: 38AABEFC  addi r5, r10, -0x4104
	ctx.r[5].s64 = ctx.r[10].s64 + -16644;
	// 826FB458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB45C: 390B3ED8  addi r8, r11, 0x3ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 16088;
	// 826FB460: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826FB464: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826FB468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB46C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB478: 386AC13C  addi r3, r10, -0x3ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -16068;
	// 826FB47C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FB480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB49C: 4BD6B985  bl 0x82466e20
	ctx.lr = 0x826FB4A0;
	sub_82466E20(ctx, base);
	// 826FB4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB4B0 size=112
    let mut pc: u32 = 0x826FB4B0;
    'dispatch: loop {
        match pc {
            0x826FB4B0 => {
    //   block [0x826FB4B0..0x826FB520)
	// 826FB4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB4BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB4C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB4C4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FB4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB4CC: 390B3F50  addi r8, r11, 0x3f50
	ctx.r[8].s64 = ctx.r[11].s64 + 16208;
	// 826FB4D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FB4D4: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826FB4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB4DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB4E8: 386AC16C  addi r3, r10, -0x3e94
	ctx.r[3].s64 = ctx.r[10].s64 + -16020;
	// 826FB4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FB4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB50C: 4BD6B915  bl 0x82466e20
	ctx.lr = 0x826FB510;
	sub_82466E20(ctx, base);
	// 826FB510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB520 size=108
    let mut pc: u32 = 0x826FB520;
    'dispatch: loop {
        match pc {
            0x826FB520 => {
    //   block [0x826FB520..0x826FB58C)
	// 826FB520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB52C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB534: 38EB3F68  addi r7, r11, 0x3f68
	ctx.r[7].s64 = ctx.r[11].s64 + 16232;
	// 826FB538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FB53C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826FB540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB544: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB550: 386AC19C  addi r3, r10, -0x3e64
	ctx.r[3].s64 = ctx.r[10].s64 + -15972;
	// 826FB554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB578: 4BD6B8A9  bl 0x82466e20
	ctx.lr = 0x826FB57C;
	sub_82466E20(ctx, base);
	// 826FB57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB590 size=108
    let mut pc: u32 = 0x826FB590;
    'dispatch: loop {
        match pc {
            0x826FB590 => {
    //   block [0x826FB590..0x826FB5FC)
	// 826FB590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB59C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB5A4: 38EB3FB0  addi r7, r11, 0x3fb0
	ctx.r[7].s64 = ctx.r[11].s64 + 16304;
	// 826FB5A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FB5AC: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826FB5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB5B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB5B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB5C0: 386AC1CC  addi r3, r10, -0x3e34
	ctx.r[3].s64 = ctx.r[10].s64 + -15924;
	// 826FB5C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB5E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB5E8: 4BD6B839  bl 0x82466e20
	ctx.lr = 0x826FB5EC;
	sub_82466E20(ctx, base);
	// 826FB5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB600 size=116
    let mut pc: u32 = 0x826FB600;
    'dispatch: loop {
        match pc {
            0x826FB600 => {
    //   block [0x826FB600..0x826FB674)
	// 826FB600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB60C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FB610: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826FB614: 390A3FF8  addi r8, r10, 0x3ff8
	ctx.r[8].s64 = ctx.r[10].s64 + 16376;
	// 826FB618: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB61C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FB620: 38AABC8C  addi r5, r10, -0x4374
	ctx.r[5].s64 = ctx.r[10].s64 + -17268;
	// 826FB624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB628: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FB62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB634: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826FB638: 396BB600  addi r11, r11, -0x4a00
	ctx.r[11].s64 = ctx.r[11].s64 + -18944;
	// 826FB63C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB640: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB644: 386AC1FC  addi r3, r10, -0x3e04
	ctx.r[3].s64 = ctx.r[10].s64 + -15876;
	// 826FB648: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FB64C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB650: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FB654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB660: 4BD6B7C1  bl 0x82466e20
	ctx.lr = 0x826FB664;
	sub_82466E20(ctx, base);
	// 826FB664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB678 size=104
    let mut pc: u32 = 0x826FB678;
    'dispatch: loop {
        match pc {
            0x826FB678 => {
    //   block [0x826FB678..0x826FB6E0)
	// 826FB678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB684: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FB688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB68C: 392AB654  addi r9, r10, -0x49ac
	ctx.r[9].s64 = ctx.r[10].s64 + -18860;
	// 826FB690: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB698: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FB69C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FB6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB6AC: 388A5A40  addi r4, r10, 0x5a40
	ctx.r[4].s64 = ctx.r[10].s64 + 23104;
	// 826FB6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB6B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB6B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FB6BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB6C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FB6C4: 386AC22C  addi r3, r10, -0x3dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -15828;
	// 826FB6C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FB6CC: 4BD6B755  bl 0x82466e20
	ctx.lr = 0x826FB6D0;
	sub_82466E20(ctx, base);
	// 826FB6D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB6E0 size=108
    let mut pc: u32 = 0x826FB6E0;
    'dispatch: loop {
        match pc {
            0x826FB6E0 => {
    //   block [0x826FB6E0..0x826FB74C)
	// 826FB6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB6EC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB6F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FB6F4: 38EB4120  addi r7, r11, 0x4120
	ctx.r[7].s64 = ctx.r[11].s64 + 16672;
	// 826FB6F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FB6FC: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 826FB700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB704: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB708: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB710: 386AC25C  addi r3, r10, -0x3da4
	ctx.r[3].s64 = ctx.r[10].s64 + -15780;
	// 826FB714: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB72C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB738: 4BD6B6E9  bl 0x82466e20
	ctx.lr = 0x826FB73C;
	sub_82466E20(ctx, base);
	// 826FB73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FB750 size=24
    let mut pc: u32 = 0x826FB750;
    'dispatch: loop {
        match pc {
            0x826FB750 => {
    //   block [0x826FB750..0x826FB768)
	// 826FB750: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB754: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FB758: 394A9148  addi r10, r10, -0x6eb8
	ctx.r[10].s64 = ctx.r[10].s64 + -28344;
	// 826FB75C: 816B411C  lwz r11, 0x411c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16668 as u32) ) } as u64;
	// 826FB760: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826FB764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB768 size=116
    let mut pc: u32 = 0x826FB768;
    'dispatch: loop {
        match pc {
            0x826FB768 => {
    //   block [0x826FB768..0x826FB7DC)
	// 826FB768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB774: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FB778: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FB77C: 390B9148  addi r8, r11, -0x6eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -28344;
	// 826FB780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB784: 392AB688  addi r9, r10, -0x4978
	ctx.r[9].s64 = ctx.r[10].s64 + -18808;
	// 826FB788: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB78C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826FB790: 38AAC22C  addi r5, r10, -0x3dd4
	ctx.r[5].s64 = ctx.r[10].s64 + -15828;
	// 826FB794: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB79C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826FB7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB7AC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FB7B0: 388A5A5C  addi r4, r10, 0x5a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 23132;
	// 826FB7B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FB7B8: 386BC28C  addi r3, r11, -0x3d74
	ctx.r[3].s64 = ctx.r[11].s64 + -15732;
	// 826FB7BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FB7C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB7C8: 4BD6B659  bl 0x82466e20
	ctx.lr = 0x826FB7CC;
	sub_82466E20(ctx, base);
	// 826FB7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB7E0 size=108
    let mut pc: u32 = 0x826FB7E0;
    'dispatch: loop {
        match pc {
            0x826FB7E0 => {
    //   block [0x826FB7E0..0x826FB84C)
	// 826FB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB7EC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB7F4: 38EB4168  addi r7, r11, 0x4168
	ctx.r[7].s64 = ctx.r[11].s64 + 16744;
	// 826FB7F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FB7FC: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826FB800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB804: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB810: 386AC2BC  addi r3, r10, -0x3d44
	ctx.r[3].s64 = ctx.r[10].s64 + -15684;
	// 826FB814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB838: 4BD6B5E9  bl 0x82466e20
	ctx.lr = 0x826FB83C;
	sub_82466E20(ctx, base);
	// 826FB83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB850 size=108
    let mut pc: u32 = 0x826FB850;
    'dispatch: loop {
        match pc {
            0x826FB850 => {
    //   block [0x826FB850..0x826FB8BC)
	// 826FB850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB85C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB864: 38EB41C8  addi r7, r11, 0x41c8
	ctx.r[7].s64 = ctx.r[11].s64 + 16840;
	// 826FB868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FB86C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826FB870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB874: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB880: 386AC2EC  addi r3, r10, -0x3d14
	ctx.r[3].s64 = ctx.r[10].s64 + -15636;
	// 826FB884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB8A8: 4BD6B579  bl 0x82466e20
	ctx.lr = 0x826FB8AC;
	sub_82466E20(ctx, base);
	// 826FB8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB8C0 size=108
    let mut pc: u32 = 0x826FB8C0;
    'dispatch: loop {
        match pc {
            0x826FB8C0 => {
    //   block [0x826FB8C0..0x826FB92C)
	// 826FB8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB8CC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FB8D4: 38EB41F8  addi r7, r11, 0x41f8
	ctx.r[7].s64 = ctx.r[11].s64 + 16888;
	// 826FB8D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FB8DC: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826FB8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB8E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FB8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB8F0: 386AC31C  addi r3, r10, -0x3ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -15588;
	// 826FB8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FB8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB918: 4BD6B509  bl 0x82466e20
	ctx.lr = 0x826FB91C;
	sub_82466E20(ctx, base);
	// 826FB91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB930 size=96
    let mut pc: u32 = 0x826FB930;
    'dispatch: loop {
        match pc {
            0x826FB930 => {
    //   block [0x826FB930..0x826FB990)
	// 826FB930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB93C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FB940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB944: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826FB948: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB950: 386AC34C  addi r3, r10, -0x3cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -15540;
	// 826FB954: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB95C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FB960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FB974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FB978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FB97C: 4BD6B4A5  bl 0x82466e20
	ctx.lr = 0x826FB980;
	sub_82466E20(ctx, base);
	// 826FB980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FB990 size=112
    let mut pc: u32 = 0x826FB990;
    'dispatch: loop {
        match pc {
            0x826FB990 => {
    //   block [0x826FB990..0x826FBA00)
	// 826FB990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FB994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FB998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FB99C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB9A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FB9A4: 38AAC34C  addi r5, r10, -0x3cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -15540;
	// 826FB9A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FB9AC: 390B4210  addi r8, r11, 0x4210
	ctx.r[8].s64 = ctx.r[11].s64 + 16912;
	// 826FB9B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FB9B4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826FB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FB9BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FB9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FB9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FB9C8: 386AC37C  addi r3, r10, -0x3c84
	ctx.r[3].s64 = ctx.r[10].s64 + -15492;
	// 826FB9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FB9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FB9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FB9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FB9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FB9EC: 4BD6B435  bl 0x82466e20
	ctx.lr = 0x826FB9F0;
	sub_82466E20(ctx, base);
	// 826FB9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FB9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FB9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FB9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBA00 size=112
    let mut pc: u32 = 0x826FBA00;
    'dispatch: loop {
        match pc {
            0x826FBA00 => {
    //   block [0x826FBA00..0x826FBA70)
	// 826FBA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBA0C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FBA10: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBA14: 392AB6A4  addi r9, r10, -0x495c
	ctx.r[9].s64 = ctx.r[10].s64 + -18780;
	// 826FBA18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBA1C: 390B4248  addi r8, r11, 0x4248
	ctx.r[8].s64 = ctx.r[11].s64 + 16968;
	// 826FBA20: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826FBA24: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826FBA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBA2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FBA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBA38: 386AC3AC  addi r3, r10, -0x3c54
	ctx.r[3].s64 = ctx.r[10].s64 + -15444;
	// 826FBA3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FBA40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FBA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBA5C: 4BD6B3C5  bl 0x82466e20
	ctx.lr = 0x826FBA60;
	sub_82466E20(ctx, base);
	// 826FBA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBA70 size=108
    let mut pc: u32 = 0x826FBA70;
    'dispatch: loop {
        match pc {
            0x826FBA70 => {
    //   block [0x826FBA70..0x826FBADC)
	// 826FBA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBA7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBA80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBA84: 38EB42F0  addi r7, r11, 0x42f0
	ctx.r[7].s64 = ctx.r[11].s64 + 17136;
	// 826FBA88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FBA8C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826FBA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBA94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBA98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBAA0: 386AC3DC  addi r3, r10, -0x3c24
	ctx.r[3].s64 = ctx.r[10].s64 + -15396;
	// 826FBAA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBAC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBAC8: 4BD6B359  bl 0x82466e20
	ctx.lr = 0x826FBACC;
	sub_82466E20(ctx, base);
	// 826FBACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBAE0 size=108
    let mut pc: u32 = 0x826FBAE0;
    'dispatch: loop {
        match pc {
            0x826FBAE0 => {
    //   block [0x826FBAE0..0x826FBB4C)
	// 826FBAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBAEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBAF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBAF4: 38EB4320  addi r7, r11, 0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + 17184;
	// 826FBAF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FBAFC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826FBB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBB04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBB08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBB10: 386AC40C  addi r3, r10, -0x3bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -15348;
	// 826FBB14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBB38: 4BD6B2E9  bl 0x82466e20
	ctx.lr = 0x826FBB3C;
	sub_82466E20(ctx, base);
	// 826FBB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FBB50 size=28
    let mut pc: u32 = 0x826FBB50;
    'dispatch: loop {
        match pc {
            0x826FBB50 => {
    //   block [0x826FBB50..0x826FBB6C)
	// 826FBB50: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBB54: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FBB58: 394A9178  addi r10, r10, -0x6e88
	ctx.r[10].s64 = ctx.r[10].s64 + -28296;
	// 826FBB5C: 816B4244  lwz r11, 0x4244(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16964 as u32) ) } as u64;
	// 826FBB60: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826FBB64: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826FBB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBB70 size=112
    let mut pc: u32 = 0x826FBB70;
    'dispatch: loop {
        match pc {
            0x826FBB70 => {
    //   block [0x826FBB70..0x826FBBE0)
	// 826FBB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBB7C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FBB80: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FBB84: 392AB818  addi r9, r10, -0x47e8
	ctx.r[9].s64 = ctx.r[10].s64 + -18408;
	// 826FBB88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBB8C: 390B9178  addi r8, r11, -0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + -28296;
	// 826FBB90: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826FBB94: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826FBB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBB9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBBA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FBBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBBA8: 386AC43C  addi r3, r10, -0x3bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15300;
	// 826FBBAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FBBB0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826FBBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBBCC: 4BD6B255  bl 0x82466e20
	ctx.lr = 0x826FBBD0;
	sub_82466E20(ctx, base);
	// 826FBBD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBBD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBBD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBBE0 size=108
    let mut pc: u32 = 0x826FBBE0;
    'dispatch: loop {
        match pc {
            0x826FBBE0 => {
    //   block [0x826FBBE0..0x826FBC4C)
	// 826FBBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBBEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBBF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBBF4: 38EB4358  addi r7, r11, 0x4358
	ctx.r[7].s64 = ctx.r[11].s64 + 17240;
	// 826FBBF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FBBFC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826FBC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBC04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBC10: 386AC46C  addi r3, r10, -0x3b94
	ctx.r[3].s64 = ctx.r[10].s64 + -15252;
	// 826FBC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBC38: 4BD6B1E9  bl 0x82466e20
	ctx.lr = 0x826FBC3C;
	sub_82466E20(ctx, base);
	// 826FBC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBC50 size=108
    let mut pc: u32 = 0x826FBC50;
    'dispatch: loop {
        match pc {
            0x826FBC50 => {
    //   block [0x826FBC50..0x826FBCBC)
	// 826FBC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBC5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBC60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBC64: 38EB4388  addi r7, r11, 0x4388
	ctx.r[7].s64 = ctx.r[11].s64 + 17288;
	// 826FBC68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FBC6C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826FBC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBC74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBC78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBC80: 386AC49C  addi r3, r10, -0x3b64
	ctx.r[3].s64 = ctx.r[10].s64 + -15204;
	// 826FBC84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBCA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBCA8: 4BD6B179  bl 0x82466e20
	ctx.lr = 0x826FBCAC;
	sub_82466E20(ctx, base);
	// 826FBCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FBCC0 size=24
    let mut pc: u32 = 0x826FBCC0;
    'dispatch: loop {
        match pc {
            0x826FBCC0 => {
    //   block [0x826FBCC0..0x826FBCD8)
	// 826FBCC0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBCC4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FBCC8: 394A9238  addi r10, r10, -0x6dc8
	ctx.r[10].s64 = ctx.r[10].s64 + -28104;
	// 826FBCCC: 816B43A0  lwz r11, 0x43a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17312 as u32) ) } as u64;
	// 826FBCD0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826FBCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBCD8 size=112
    let mut pc: u32 = 0x826FBCD8;
    'dispatch: loop {
        match pc {
            0x826FBCD8 => {
    //   block [0x826FBCD8..0x826FBD48)
	// 826FBCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBCE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBCE4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FBCE8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FBCEC: 392AB86C  addi r9, r10, -0x4794
	ctx.r[9].s64 = ctx.r[10].s64 + -18324;
	// 826FBCF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBCF4: 390B9238  addi r8, r11, -0x6dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -28104;
	// 826FBCF8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826FBCFC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826FBD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBD04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBD08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FBD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBD10: 386AC4CC  addi r3, r10, -0x3b34
	ctx.r[3].s64 = ctx.r[10].s64 + -15156;
	// 826FBD14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FBD18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FBD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBD34: 4BD6B0ED  bl 0x82466e20
	ctx.lr = 0x826FBD38;
	sub_82466E20(ctx, base);
	// 826FBD38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBD48 size=108
    let mut pc: u32 = 0x826FBD48;
    'dispatch: loop {
        match pc {
            0x826FBD48 => {
    //   block [0x826FBD48..0x826FBDB4)
	// 826FBD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBD54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBD58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBD5C: 38EB43A4  addi r7, r11, 0x43a4
	ctx.r[7].s64 = ctx.r[11].s64 + 17316;
	// 826FBD60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FBD64: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826FBD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBD6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBD70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBD78: 386AC4FC  addi r3, r10, -0x3b04
	ctx.r[3].s64 = ctx.r[10].s64 + -15108;
	// 826FBD7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBD9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBDA0: 4BD6B081  bl 0x82466e20
	ctx.lr = 0x826FBDA4;
	sub_82466E20(ctx, base);
	// 826FBDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBDB8 size=108
    let mut pc: u32 = 0x826FBDB8;
    'dispatch: loop {
        match pc {
            0x826FBDB8 => {
    //   block [0x826FBDB8..0x826FBE24)
	// 826FBDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBDC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBDC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBDCC: 38EB43D4  addi r7, r11, 0x43d4
	ctx.r[7].s64 = ctx.r[11].s64 + 17364;
	// 826FBDD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FBDD4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826FBDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBDDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBDE8: 386AC52C  addi r3, r10, -0x3ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -15060;
	// 826FBDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBE10: 4BD6B011  bl 0x82466e20
	ctx.lr = 0x826FBE14;
	sub_82466E20(ctx, base);
	// 826FBE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBE28 size=112
    let mut pc: u32 = 0x826FBE28;
    'dispatch: loop {
        match pc {
            0x826FBE28 => {
    //   block [0x826FBE28..0x826FBE98)
	// 826FBE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBE34: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FBE38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBE3C: 392AB890  addi r9, r10, -0x4770
	ctx.r[9].s64 = ctx.r[10].s64 + -18288;
	// 826FBE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FBE44: 390B4408  addi r8, r11, 0x4408
	ctx.r[8].s64 = ctx.r[11].s64 + 17416;
	// 826FBE48: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826FBE4C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826FBE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBE54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FBE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBE60: 386AC55C  addi r3, r10, -0x3aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -15012;
	// 826FBE64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FBE68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FBE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBE7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBE84: 4BD6AF9D  bl 0x82466e20
	ctx.lr = 0x826FBE88;
	sub_82466E20(ctx, base);
	// 826FBE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBE98 size=108
    let mut pc: u32 = 0x826FBE98;
    'dispatch: loop {
        match pc {
            0x826FBE98 => {
    //   block [0x826FBE98..0x826FBF04)
	// 826FBE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBEA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBEA8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBEAC: 38EB4468  addi r7, r11, 0x4468
	ctx.r[7].s64 = ctx.r[11].s64 + 17512;
	// 826FBEB0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826FBEB4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826FBEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBEBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBEC8: 386AC58C  addi r3, r10, -0x3a74
	ctx.r[3].s64 = ctx.r[10].s64 + -14964;
	// 826FBECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBEF0: 4BD6AF31  bl 0x82466e20
	ctx.lr = 0x826FBEF4;
	sub_82466E20(ctx, base);
	// 826FBEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBF08 size=108
    let mut pc: u32 = 0x826FBF08;
    'dispatch: loop {
        match pc {
            0x826FBF08 => {
    //   block [0x826FBF08..0x826FBF74)
	// 826FBF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBF14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBF18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826FBF1C: 38EB4558  addi r7, r11, 0x4558
	ctx.r[7].s64 = ctx.r[11].s64 + 17752;
	// 826FBF20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FBF24: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826FBF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBF2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBF38: 386AC5BC  addi r3, r10, -0x3a44
	ctx.r[3].s64 = ctx.r[10].s64 + -14916;
	// 826FBF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBF60: 4BD6AEC1  bl 0x82466e20
	ctx.lr = 0x826FBF64;
	sub_82466E20(ctx, base);
	// 826FBF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FBF78 size=108
    let mut pc: u32 = 0x826FBF78;
    'dispatch: loop {
        match pc {
            0x826FBF78 => {
    //   block [0x826FBF78..0x826FBFE4)
	// 826FBF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FBF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FBF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FBF84: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FBF8C: 38EB4570  addi r7, r11, 0x4570
	ctx.r[7].s64 = ctx.r[11].s64 + 17776;
	// 826FBF90: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826FBF94: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826FBF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FBF9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FBFA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FBFA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FBFA8: 386AC5EC  addi r3, r10, -0x3a14
	ctx.r[3].s64 = ctx.r[10].s64 + -14868;
	// 826FBFAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FBFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FBFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FBFB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FBFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FBFC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FBFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FBFC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FBFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FBFD0: 4BD6AE51  bl 0x82466e20
	ctx.lr = 0x826FBFD4;
	sub_82466E20(ctx, base);
	// 826FBFD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FBFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FBFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FBFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FBFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FBFE8 size=24
    let mut pc: u32 = 0x826FBFE8;
    'dispatch: loop {
        match pc {
            0x826FBFE8 => {
    //   block [0x826FBFE8..0x826FC000)
	// 826FBFE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FBFEC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FBFF0: 394A92C8  addi r10, r10, -0x6d38
	ctx.r[10].s64 = ctx.r[10].s64 + -27960;
	// 826FBFF4: 816B4600  lwz r11, 0x4600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17920 as u32) ) } as u64;
	// 826FBFF8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826FBFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC000 size=108
    let mut pc: u32 = 0x826FC000;
    'dispatch: loop {
        match pc {
            0x826FC000 => {
    //   block [0x826FC000..0x826FC06C)
	// 826FC000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC00C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FC010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC014: 38EB92C8  addi r7, r11, -0x6d38
	ctx.r[7].s64 = ctx.r[11].s64 + -27960;
	// 826FC018: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC01C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826FC020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC024: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC030: 386AC61C  addi r3, r10, -0x39e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14820;
	// 826FC034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC058: 4BD6ADC9  bl 0x82466e20
	ctx.lr = 0x826FC05C;
	sub_82466E20(ctx, base);
	// 826FC05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FC070 size=24
    let mut pc: u32 = 0x826FC070;
    'dispatch: loop {
        match pc {
            0x826FC070 => {
    //   block [0x826FC070..0x826FC088)
	// 826FC070: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC074: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FC078: 394A92F8  addi r10, r10, -0x6d08
	ctx.r[10].s64 = ctx.r[10].s64 + -27912;
	// 826FC07C: 816B4600  lwz r11, 0x4600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17920 as u32) ) } as u64;
	// 826FC080: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826FC084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC088 size=108
    let mut pc: u32 = 0x826FC088;
    'dispatch: loop {
        match pc {
            0x826FC088 => {
    //   block [0x826FC088..0x826FC0F4)
	// 826FC088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC094: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FC098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC09C: 38EB92F8  addi r7, r11, -0x6d08
	ctx.r[7].s64 = ctx.r[11].s64 + -27912;
	// 826FC0A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC0A4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826FC0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC0AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC0B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC0B8: 386AC64C  addi r3, r10, -0x39b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14772;
	// 826FC0BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC0C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC0C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC0D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC0D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC0D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC0DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC0E0: 4BD6AD41  bl 0x82466e20
	ctx.lr = 0x826FC0E4;
	sub_82466E20(ctx, base);
	// 826FC0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC0F8 size=108
    let mut pc: u32 = 0x826FC0F8;
    'dispatch: loop {
        match pc {
            0x826FC0F8 => {
    //   block [0x826FC0F8..0x826FC164)
	// 826FC0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC104: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC10C: 38EB45E8  addi r7, r11, 0x45e8
	ctx.r[7].s64 = ctx.r[11].s64 + 17896;
	// 826FC110: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FC114: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826FC118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC11C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC120: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC128: 386AC67C  addi r3, r10, -0x3984
	ctx.r[3].s64 = ctx.r[10].s64 + -14724;
	// 826FC12C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC14C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC150: 4BD6ACD1  bl 0x82466e20
	ctx.lr = 0x826FC154;
	sub_82466E20(ctx, base);
	// 826FC154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FC168 size=24
    let mut pc: u32 = 0x826FC168;
    'dispatch: loop {
        match pc {
            0x826FC168 => {
    //   block [0x826FC168..0x826FC180)
	// 826FC168: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC16C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FC170: 394A9328  addi r10, r10, -0x6cd8
	ctx.r[10].s64 = ctx.r[10].s64 + -27864;
	// 826FC174: 816B4600  lwz r11, 0x4600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17920 as u32) ) } as u64;
	// 826FC178: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826FC17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC180 size=108
    let mut pc: u32 = 0x826FC180;
    'dispatch: loop {
        match pc {
            0x826FC180 => {
    //   block [0x826FC180..0x826FC1EC)
	// 826FC180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC18C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FC190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC194: 38EB9328  addi r7, r11, -0x6cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -27864;
	// 826FC198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC19C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826FC1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC1A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC1B0: 386AC6AC  addi r3, r10, -0x3954
	ctx.r[3].s64 = ctx.r[10].s64 + -14676;
	// 826FC1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC1D8: 4BD6AC49  bl 0x82466e20
	ctx.lr = 0x826FC1DC;
	sub_82466E20(ctx, base);
	// 826FC1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC1F0 size=112
    let mut pc: u32 = 0x826FC1F0;
    'dispatch: loop {
        match pc {
            0x826FC1F0 => {
    //   block [0x826FC1F0..0x826FC260)
	// 826FC1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC1FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FC200: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC204: 392AB8D4  addi r9, r10, -0x472c
	ctx.r[9].s64 = ctx.r[10].s64 + -18220;
	// 826FC208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC20C: 390B4604  addi r8, r11, 0x4604
	ctx.r[8].s64 = ctx.r[11].s64 + 17924;
	// 826FC210: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826FC214: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826FC218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC21C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FC224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC228: 386AC6DC  addi r3, r10, -0x3924
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	// 826FC22C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FC230: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FC234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC24C: 4BD6ABD5  bl 0x82466e20
	ctx.lr = 0x826FC250;
	sub_82466E20(ctx, base);
	// 826FC250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC260 size=108
    let mut pc: u32 = 0x826FC260;
    'dispatch: loop {
        match pc {
            0x826FC260 => {
    //   block [0x826FC260..0x826FC2CC)
	// 826FC260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC26C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC274: 38EB4634  addi r7, r11, 0x4634
	ctx.r[7].s64 = ctx.r[11].s64 + 17972;
	// 826FC278: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC27C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826FC280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC290: 386AC70C  addi r3, r10, -0x38f4
	ctx.r[3].s64 = ctx.r[10].s64 + -14580;
	// 826FC294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC2B8: 4BD6AB69  bl 0x82466e20
	ctx.lr = 0x826FC2BC;
	sub_82466E20(ctx, base);
	// 826FC2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC2D0 size=108
    let mut pc: u32 = 0x826FC2D0;
    'dispatch: loop {
        match pc {
            0x826FC2D0 => {
    //   block [0x826FC2D0..0x826FC33C)
	// 826FC2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC2DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC2E4: 38EB4664  addi r7, r11, 0x4664
	ctx.r[7].s64 = ctx.r[11].s64 + 18020;
	// 826FC2E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC2EC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826FC2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC2F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC2F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC300: 386AC73C  addi r3, r10, -0x38c4
	ctx.r[3].s64 = ctx.r[10].s64 + -14532;
	// 826FC304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC328: 4BD6AAF9  bl 0x82466e20
	ctx.lr = 0x826FC32C;
	sub_82466E20(ctx, base);
	// 826FC32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC340 size=112
    let mut pc: u32 = 0x826FC340;
    'dispatch: loop {
        match pc {
            0x826FC340 => {
    //   block [0x826FC340..0x826FC3B0)
	// 826FC340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC34C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC350: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC354: 38AAC79C  addi r5, r10, -0x3864
	ctx.r[5].s64 = ctx.r[10].s64 + -14436;
	// 826FC358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC35C: 390B4694  addi r8, r11, 0x4694
	ctx.r[8].s64 = ctx.r[11].s64 + 18068;
	// 826FC360: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FC364: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826FC368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC36C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FC374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC378: 386AC76C  addi r3, r10, -0x3894
	ctx.r[3].s64 = ctx.r[10].s64 + -14484;
	// 826FC37C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FC380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC39C: 4BD6AA85  bl 0x82466e20
	ctx.lr = 0x826FC3A0;
	sub_82466E20(ctx, base);
	// 826FC3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC3B0 size=108
    let mut pc: u32 = 0x826FC3B0;
    'dispatch: loop {
        match pc {
            0x826FC3B0 => {
    //   block [0x826FC3B0..0x826FC41C)
	// 826FC3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC3BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC3C4: 38EB46AC  addi r7, r11, 0x46ac
	ctx.r[7].s64 = ctx.r[11].s64 + 18092;
	// 826FC3C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC3CC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826FC3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC3D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC3E0: 386AC79C  addi r3, r10, -0x3864
	ctx.r[3].s64 = ctx.r[10].s64 + -14436;
	// 826FC3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC408: 4BD6AA19  bl 0x82466e20
	ctx.lr = 0x826FC40C;
	sub_82466E20(ctx, base);
	// 826FC40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC420 size=108
    let mut pc: u32 = 0x826FC420;
    'dispatch: loop {
        match pc {
            0x826FC420 => {
    //   block [0x826FC420..0x826FC48C)
	// 826FC420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC42C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC434: 38EB46DC  addi r7, r11, 0x46dc
	ctx.r[7].s64 = ctx.r[11].s64 + 18140;
	// 826FC438: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FC43C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826FC440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC450: 386AC7CC  addi r3, r10, -0x3834
	ctx.r[3].s64 = ctx.r[10].s64 + -14388;
	// 826FC454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC478: 4BD6A9A9  bl 0x82466e20
	ctx.lr = 0x826FC47C;
	sub_82466E20(ctx, base);
	// 826FC47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC490 size=108
    let mut pc: u32 = 0x826FC490;
    'dispatch: loop {
        match pc {
            0x826FC490 => {
    //   block [0x826FC490..0x826FC4FC)
	// 826FC490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC49C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC4A4: 38EB46F4  addi r7, r11, 0x46f4
	ctx.r[7].s64 = ctx.r[11].s64 + 18164;
	// 826FC4A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC4AC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826FC4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC4B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC4B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC4C0: 386AC7FC  addi r3, r10, -0x3804
	ctx.r[3].s64 = ctx.r[10].s64 + -14340;
	// 826FC4C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC4E8: 4BD6A939  bl 0x82466e20
	ctx.lr = 0x826FC4EC;
	sub_82466E20(ctx, base);
	// 826FC4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC500 size=108
    let mut pc: u32 = 0x826FC500;
    'dispatch: loop {
        match pc {
            0x826FC500 => {
    //   block [0x826FC500..0x826FC56C)
	// 826FC500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC50C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC514: 38EB4728  addi r7, r11, 0x4728
	ctx.r[7].s64 = ctx.r[11].s64 + 18216;
	// 826FC518: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826FC51C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826FC520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC530: 386AC82C  addi r3, r10, -0x37d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14292;
	// 826FC534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC558: 4BD6A8C9  bl 0x82466e20
	ctx.lr = 0x826FC55C;
	sub_82466E20(ctx, base);
	// 826FC55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC570 size=108
    let mut pc: u32 = 0x826FC570;
    'dispatch: loop {
        match pc {
            0x826FC570 => {
    //   block [0x826FC570..0x826FC5DC)
	// 826FC570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC57C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC584: 38EB47D0  addi r7, r11, 0x47d0
	ctx.r[7].s64 = ctx.r[11].s64 + 18384;
	// 826FC588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC58C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826FC590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC594: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC5A0: 386AC85C  addi r3, r10, -0x37a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14244;
	// 826FC5A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC5B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC5C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC5C8: 4BD6A859  bl 0x82466e20
	ctx.lr = 0x826FC5CC;
	sub_82466E20(ctx, base);
	// 826FC5CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC5E0 size=108
    let mut pc: u32 = 0x826FC5E0;
    'dispatch: loop {
        match pc {
            0x826FC5E0 => {
    //   block [0x826FC5E0..0x826FC64C)
	// 826FC5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC5EC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC5F4: 38EB4800  addi r7, r11, 0x4800
	ctx.r[7].s64 = ctx.r[11].s64 + 18432;
	// 826FC5F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FC5FC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826FC600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC604: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC60C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC610: 386AC88C  addi r3, r10, -0x3774
	ctx.r[3].s64 = ctx.r[10].s64 + -14196;
	// 826FC614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC638: 4BD6A7E9  bl 0x82466e20
	ctx.lr = 0x826FC63C;
	sub_82466E20(ctx, base);
	// 826FC63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC650 size=108
    let mut pc: u32 = 0x826FC650;
    'dispatch: loop {
        match pc {
            0x826FC650 => {
    //   block [0x826FC650..0x826FC6BC)
	// 826FC650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC65C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC664: 38EB4818  addi r7, r11, 0x4818
	ctx.r[7].s64 = ctx.r[11].s64 + 18456;
	// 826FC668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC66C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826FC670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC674: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC680: 386AC8BC  addi r3, r10, -0x3744
	ctx.r[3].s64 = ctx.r[10].s64 + -14148;
	// 826FC684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC6A8: 4BD6A779  bl 0x82466e20
	ctx.lr = 0x826FC6AC;
	sub_82466E20(ctx, base);
	// 826FC6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC6C0 size=108
    let mut pc: u32 = 0x826FC6C0;
    'dispatch: loop {
        match pc {
            0x826FC6C0 => {
    //   block [0x826FC6C0..0x826FC72C)
	// 826FC6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC6CC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC6D4: 38EB4848  addi r7, r11, 0x4848
	ctx.r[7].s64 = ctx.r[11].s64 + 18504;
	// 826FC6D8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826FC6DC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826FC6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC6E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC6E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC6F0: 386AC8EC  addi r3, r10, -0x3714
	ctx.r[3].s64 = ctx.r[10].s64 + -14100;
	// 826FC6F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC718: 4BD6A709  bl 0x82466e20
	ctx.lr = 0x826FC71C;
	sub_82466E20(ctx, base);
	// 826FC71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FC730 size=24
    let mut pc: u32 = 0x826FC730;
    'dispatch: loop {
        match pc {
            0x826FC730 => {
    //   block [0x826FC730..0x826FC748)
	// 826FC730: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC734: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FC738: 394A9358  addi r10, r10, -0x6ca8
	ctx.r[10].s64 = ctx.r[10].s64 + -27816;
	// 826FC73C: 816B4724  lwz r11, 0x4724(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18212 as u32) ) } as u64;
	// 826FC740: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826FC744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC748 size=112
    let mut pc: u32 = 0x826FC748;
    'dispatch: loop {
        match pc {
            0x826FC748 => {
    //   block [0x826FC748..0x826FC7B8)
	// 826FC748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC754: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FC758: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FC75C: 392AB900  addi r9, r10, -0x4700
	ctx.r[9].s64 = ctx.r[10].s64 + -18176;
	// 826FC760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC764: 390B9358  addi r8, r11, -0x6ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -27816;
	// 826FC768: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FC76C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826FC770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FC77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC780: 386AC91C  addi r3, r10, -0x36e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14052;
	// 826FC784: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FC788: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FC78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC7A4: 4BD6A67D  bl 0x82466e20
	ctx.lr = 0x826FC7A8;
	sub_82466E20(ctx, base);
	// 826FC7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC7B8 size=108
    let mut pc: u32 = 0x826FC7B8;
    'dispatch: loop {
        match pc {
            0x826FC7B8 => {
    //   block [0x826FC7B8..0x826FC824)
	// 826FC7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC7C4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC7C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC7CC: 38EB490C  addi r7, r11, 0x490c
	ctx.r[7].s64 = ctx.r[11].s64 + 18700;
	// 826FC7D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC7D4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826FC7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC7DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC7E8: 386AC94C  addi r3, r10, -0x36b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14004;
	// 826FC7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC810: 4BD6A611  bl 0x82466e20
	ctx.lr = 0x826FC814;
	sub_82466E20(ctx, base);
	// 826FC814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC828 size=112
    let mut pc: u32 = 0x826FC828;
    'dispatch: loop {
        match pc {
            0x826FC828 => {
    //   block [0x826FC828..0x826FC898)
	// 826FC828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC834: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FC838: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC83C: 392AB944  addi r9, r10, -0x46bc
	ctx.r[9].s64 = ctx.r[10].s64 + -18108;
	// 826FC840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC844: 390B4940  addi r8, r11, 0x4940
	ctx.r[8].s64 = ctx.r[11].s64 + 18752;
	// 826FC848: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826FC84C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826FC850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC854: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FC85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC860: 386AC97C  addi r3, r10, -0x3684
	ctx.r[3].s64 = ctx.r[10].s64 + -13956;
	// 826FC864: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FC868: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FC86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC884: 4BD6A59D  bl 0x82466e20
	ctx.lr = 0x826FC888;
	sub_82466E20(ctx, base);
	// 826FC888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FC898 size=24
    let mut pc: u32 = 0x826FC898;
    'dispatch: loop {
        match pc {
            0x826FC898 => {
    //   block [0x826FC898..0x826FC8B0)
	// 826FC898: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC89C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FC8A0: 394A93D0  addi r10, r10, -0x6c30
	ctx.r[10].s64 = ctx.r[10].s64 + -27696;
	// 826FC8A4: 816B493C  lwz r11, 0x493c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18748 as u32) ) } as u64;
	// 826FC8A8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826FC8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC8B0 size=112
    let mut pc: u32 = 0x826FC8B0;
    'dispatch: loop {
        match pc {
            0x826FC8B0 => {
    //   block [0x826FC8B0..0x826FC920)
	// 826FC8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC8BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FC8C0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FC8C4: 392AB980  addi r9, r10, -0x4680
	ctx.r[9].s64 = ctx.r[10].s64 + -18048;
	// 826FC8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC8CC: 390B93D0  addi r8, r11, -0x6c30
	ctx.r[8].s64 = ctx.r[11].s64 + -27696;
	// 826FC8D0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826FC8D4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826FC8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC8DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FC8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC8E8: 386AC9AC  addi r3, r10, -0x3654
	ctx.r[3].s64 = ctx.r[10].s64 + -13908;
	// 826FC8EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FC8F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FC8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC90C: 4BD6A515  bl 0x82466e20
	ctx.lr = 0x826FC910;
	sub_82466E20(ctx, base);
	// 826FC910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC920 size=108
    let mut pc: u32 = 0x826FC920;
    'dispatch: loop {
        match pc {
            0x826FC920 => {
    //   block [0x826FC920..0x826FC98C)
	// 826FC920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC92C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC934: 38EB4A00  addi r7, r11, 0x4a00
	ctx.r[7].s64 = ctx.r[11].s64 + 18944;
	// 826FC938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FC93C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826FC940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC950: 386AC9DC  addi r3, r10, -0x3624
	ctx.r[3].s64 = ctx.r[10].s64 + -13860;
	// 826FC954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC978: 4BD6A4A9  bl 0x82466e20
	ctx.lr = 0x826FC97C;
	sub_82466E20(ctx, base);
	// 826FC97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FC990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FC990 size=108
    let mut pc: u32 = 0x826FC990;
    'dispatch: loop {
        match pc {
            0x826FC990 => {
    //   block [0x826FC990..0x826FC9FC)
	// 826FC990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FC994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FC998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FC99C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FC9A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FC9A4: 38EB4A18  addi r7, r11, 0x4a18
	ctx.r[7].s64 = ctx.r[11].s64 + 18968;
	// 826FC9A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FC9AC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826FC9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FC9B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FC9B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FC9BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FC9C0: 386ACA0C  addi r3, r10, -0x35f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13812;
	// 826FC9C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FC9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FC9CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FC9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FC9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FC9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FC9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FC9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FC9E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FC9E8: 4BD6A439  bl 0x82466e20
	ctx.lr = 0x826FC9EC;
	sub_82466E20(ctx, base);
	// 826FC9EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FC9F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FC9F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FC9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826FCA00 size=24
    let mut pc: u32 = 0x826FCA00;
    'dispatch: loop {
        match pc {
            0x826FCA00 => {
    //   block [0x826FCA00..0x826FCA18)
	// 826FCA00: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCA04: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826FCA08: 394A9418  addi r10, r10, -0x6be8
	ctx.r[10].s64 = ctx.r[10].s64 + -27624;
	// 826FCA0C: 816B4A48  lwz r11, 0x4a48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19016 as u32) ) } as u64;
	// 826FCA10: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826FCA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCA18 size=112
    let mut pc: u32 = 0x826FCA18;
    'dispatch: loop {
        match pc {
            0x826FCA18 => {
    //   block [0x826FCA18..0x826FCA88)
	// 826FCA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCA24: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FCA28: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826FCA2C: 392AB9BC  addi r9, r10, -0x4644
	ctx.r[9].s64 = ctx.r[10].s64 + -17988;
	// 826FCA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCA34: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 826FCA38: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826FCA3C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826FCA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCA44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FCA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCA50: 386ACA3C  addi r3, r10, -0x35c4
	ctx.r[3].s64 = ctx.r[10].s64 + -13764;
	// 826FCA54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FCA58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FCA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCA6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCA74: 4BD6A3AD  bl 0x82466e20
	ctx.lr = 0x826FCA78;
	sub_82466E20(ctx, base);
	// 826FCA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCA88 size=108
    let mut pc: u32 = 0x826FCA88;
    'dispatch: loop {
        match pc {
            0x826FCA88 => {
    //   block [0x826FCA88..0x826FCAF4)
	// 826FCA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCA94: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCA9C: 38EB4A4C  addi r7, r11, 0x4a4c
	ctx.r[7].s64 = ctx.r[11].s64 + 19020;
	// 826FCAA0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FCAA4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826FCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCAAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCAB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCAB8: 386ACA6C  addi r3, r10, -0x3594
	ctx.r[3].s64 = ctx.r[10].s64 + -13716;
	// 826FCABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCAE0: 4BD6A341  bl 0x82466e20
	ctx.lr = 0x826FCAE4;
	sub_82466E20(ctx, base);
	// 826FCAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCAF8 size=108
    let mut pc: u32 = 0x826FCAF8;
    'dispatch: loop {
        match pc {
            0x826FCAF8 => {
    //   block [0x826FCAF8..0x826FCB64)
	// 826FCAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCB04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCB0C: 38EB4A68  addi r7, r11, 0x4a68
	ctx.r[7].s64 = ctx.r[11].s64 + 19048;
	// 826FCB10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FCB14: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826FCB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCB1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCB28: 386ACA9C  addi r3, r10, -0x3564
	ctx.r[3].s64 = ctx.r[10].s64 + -13668;
	// 826FCB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCB50: 4BD6A2D1  bl 0x82466e20
	ctx.lr = 0x826FCB54;
	sub_82466E20(ctx, base);
	// 826FCB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCB68 size=108
    let mut pc: u32 = 0x826FCB68;
    'dispatch: loop {
        match pc {
            0x826FCB68 => {
    //   block [0x826FCB68..0x826FCBD4)
	// 826FCB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCB74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCB7C: 38EB4AB0  addi r7, r11, 0x4ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 19120;
	// 826FCB80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826FCB84: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826FCB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCB8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCB98: 386ACACC  addi r3, r10, -0x3534
	ctx.r[3].s64 = ctx.r[10].s64 + -13620;
	// 826FCB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCBC0: 4BD6A261  bl 0x82466e20
	ctx.lr = 0x826FCBC4;
	sub_82466E20(ctx, base);
	// 826FCBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCBD8 size=108
    let mut pc: u32 = 0x826FCBD8;
    'dispatch: loop {
        match pc {
            0x826FCBD8 => {
    //   block [0x826FCBD8..0x826FCC44)
	// 826FCBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCBE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCBEC: 38EB4AE0  addi r7, r11, 0x4ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 19168;
	// 826FCBF0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826FCBF4: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826FCBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCBFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCC00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCC08: 386ACAFC  addi r3, r10, -0x3504
	ctx.r[3].s64 = ctx.r[10].s64 + -13572;
	// 826FCC0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCC2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCC30: 4BD6A1F1  bl 0x82466e20
	ctx.lr = 0x826FCC34;
	sub_82466E20(ctx, base);
	// 826FCC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCC48 size=108
    let mut pc: u32 = 0x826FCC48;
    'dispatch: loop {
        match pc {
            0x826FCC48 => {
    //   block [0x826FCC48..0x826FCCB4)
	// 826FCC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCC54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCC5C: 38EB4C00  addi r7, r11, 0x4c00
	ctx.r[7].s64 = ctx.r[11].s64 + 19456;
	// 826FCC60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826FCC64: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826FCC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCC6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCC70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCC78: 386ACB2C  addi r3, r10, -0x34d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	// 826FCC7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCCA0: 4BD6A181  bl 0x82466e20
	ctx.lr = 0x826FCCA4;
	sub_82466E20(ctx, base);
	// 826FCCA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCCB8 size=108
    let mut pc: u32 = 0x826FCCB8;
    'dispatch: loop {
        match pc {
            0x826FCCB8 => {
    //   block [0x826FCCB8..0x826FCD24)
	// 826FCCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCCC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCCCC: 38EB4C90  addi r7, r11, 0x4c90
	ctx.r[7].s64 = ctx.r[11].s64 + 19600;
	// 826FCCD0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826FCCD4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826FCCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCCDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCCE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCCE8: 386ACB5C  addi r3, r10, -0x34a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13476;
	// 826FCCEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCD10: 4BD6A111  bl 0x82466e20
	ctx.lr = 0x826FCD14;
	sub_82466E20(ctx, base);
	// 826FCD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCD28 size=108
    let mut pc: u32 = 0x826FCD28;
    'dispatch: loop {
        match pc {
            0x826FCD28 => {
    //   block [0x826FCD28..0x826FCD94)
	// 826FCD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCD34: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCD3C: 38EB4D50  addi r7, r11, 0x4d50
	ctx.r[7].s64 = ctx.r[11].s64 + 19792;
	// 826FCD40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826FCD44: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826FCD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCD4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCD50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCD58: 386ACB8C  addi r3, r10, -0x3474
	ctx.r[3].s64 = ctx.r[10].s64 + -13428;
	// 826FCD5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCD80: 4BD6A0A1  bl 0x82466e20
	ctx.lr = 0x826FCD84;
	sub_82466E20(ctx, base);
	// 826FCD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCD98 size=108
    let mut pc: u32 = 0x826FCD98;
    'dispatch: loop {
        match pc {
            0x826FCD98 => {
    //   block [0x826FCD98..0x826FCE04)
	// 826FCD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCDA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCDA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCDAC: 38EB4E28  addi r7, r11, 0x4e28
	ctx.r[7].s64 = ctx.r[11].s64 + 20008;
	// 826FCDB0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826FCDB4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826FCDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCDBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCDC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCDC8: 386ACBBC  addi r3, r10, -0x3444
	ctx.r[3].s64 = ctx.r[10].s64 + -13380;
	// 826FCDCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCDF0: 4BD6A031  bl 0x82466e20
	ctx.lr = 0x826FCDF4;
	sub_82466E20(ctx, base);
	// 826FCDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCE08 size=108
    let mut pc: u32 = 0x826FCE08;
    'dispatch: loop {
        match pc {
            0x826FCE08 => {
    //   block [0x826FCE08..0x826FCE74)
	// 826FCE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCE14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCE1C: 38EB4EE8  addi r7, r11, 0x4ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 20200;
	// 826FCE20: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826FCE24: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826FCE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCE2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCE38: 386ACBEC  addi r3, r10, -0x3414
	ctx.r[3].s64 = ctx.r[10].s64 + -13332;
	// 826FCE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCE60: 4BD69FC1  bl 0x82466e20
	ctx.lr = 0x826FCE64;
	sub_82466E20(ctx, base);
	// 826FCE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCE78 size=112
    let mut pc: u32 = 0x826FCE78;
    'dispatch: loop {
        match pc {
            0x826FCE78 => {
    //   block [0x826FCE78..0x826FCEE8)
	// 826FCE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCE84: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826FCE88: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826FCE8C: 38EA4F90  addi r7, r10, 0x4f90
	ctx.r[7].s64 = ctx.r[10].s64 + 20368;
	// 826FCE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCE94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826FCE98: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826FCE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCEA0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCEA4: 396BB9D0  addi r11, r11, -0x4630
	ctx.r[11].s64 = ctx.r[11].s64 + -17968;
	// 826FCEA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCEAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCEB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCEB4: 386ACC1C  addi r3, r10, -0x33e4
	ctx.r[3].s64 = ctx.r[10].s64 + -13284;
	// 826FCEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCEBC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826FCEC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCEC4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826FCEC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCECC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCED0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCED4: 4BD69F4D  bl 0x82466e20
	ctx.lr = 0x826FCED8;
	sub_82466E20(ctx, base);
	// 826FCED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCEE8 size=108
    let mut pc: u32 = 0x826FCEE8;
    'dispatch: loop {
        match pc {
            0x826FCEE8 => {
    //   block [0x826FCEE8..0x826FCF54)
	// 826FCEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCEF4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCEFC: 38EB50B0  addi r7, r11, 0x50b0
	ctx.r[7].s64 = ctx.r[11].s64 + 20656;
	// 826FCF00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FCF04: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826FCF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCF0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCF10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCF18: 386ACC4C  addi r3, r10, -0x33b4
	ctx.r[3].s64 = ctx.r[10].s64 + -13236;
	// 826FCF1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCF20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCF28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCF30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCF38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCF40: 4BD69EE1  bl 0x82466e20
	ctx.lr = 0x826FCF44;
	sub_82466E20(ctx, base);
	// 826FCF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCF58 size=108
    let mut pc: u32 = 0x826FCF58;
    'dispatch: loop {
        match pc {
            0x826FCF58 => {
    //   block [0x826FCF58..0x826FCFC4)
	// 826FCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCF64: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCF6C: 38EB5110  addi r7, r11, 0x5110
	ctx.r[7].s64 = ctx.r[11].s64 + 20752;
	// 826FCF70: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826FCF74: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826FCF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCF7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCF88: 386ACC7C  addi r3, r10, -0x3384
	ctx.r[3].s64 = ctx.r[10].s64 + -13188;
	// 826FCF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FCF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FCF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FCF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FCF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FCFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FCFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FCFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FCFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FCFB0: 4BD69E71  bl 0x82466e20
	ctx.lr = 0x826FCFB4;
	sub_82466E20(ctx, base);
	// 826FCFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FCFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FCFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FCFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FCFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FCFC8 size=108
    let mut pc: u32 = 0x826FCFC8;
    'dispatch: loop {
        match pc {
            0x826FCFC8 => {
    //   block [0x826FCFC8..0x826FD034)
	// 826FCFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FCFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FCFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FCFD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FCFD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FCFDC: 38EB5218  addi r7, r11, 0x5218
	ctx.r[7].s64 = ctx.r[11].s64 + 21016;
	// 826FCFE0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826FCFE4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826FCFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FCFEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FCFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FCFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FCFF8: 386ACCAC  addi r3, r10, -0x3354
	ctx.r[3].s64 = ctx.r[10].s64 + -13140;
	// 826FCFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD020: 4BD69E01  bl 0x82466e20
	ctx.lr = 0x826FD024;
	sub_82466E20(ctx, base);
	// 826FD024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD038 size=108
    let mut pc: u32 = 0x826FD038;
    'dispatch: loop {
        match pc {
            0x826FD038 => {
    //   block [0x826FD038..0x826FD0A4)
	// 826FD038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD044: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD04C: 38EB52F0  addi r7, r11, 0x52f0
	ctx.r[7].s64 = ctx.r[11].s64 + 21232;
	// 826FD050: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FD054: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826FD058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD05C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD068: 386ACCDC  addi r3, r10, -0x3324
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	// 826FD06C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD090: 4BD69D91  bl 0x82466e20
	ctx.lr = 0x826FD094;
	sub_82466E20(ctx, base);
	// 826FD094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD0A8 size=108
    let mut pc: u32 = 0x826FD0A8;
    'dispatch: loop {
        match pc {
            0x826FD0A8 => {
    //   block [0x826FD0A8..0x826FD114)
	// 826FD0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD0B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD0BC: 38EB5338  addi r7, r11, 0x5338
	ctx.r[7].s64 = ctx.r[11].s64 + 21304;
	// 826FD0C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD0C4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826FD0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD0CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD0D8: 386ACD0C  addi r3, r10, -0x32f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13044;
	// 826FD0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD100: 4BD69D21  bl 0x82466e20
	ctx.lr = 0x826FD104;
	sub_82466E20(ctx, base);
	// 826FD104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD118 size=108
    let mut pc: u32 = 0x826FD118;
    'dispatch: loop {
        match pc {
            0x826FD118 => {
    //   block [0x826FD118..0x826FD184)
	// 826FD118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD124: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD12C: 38EB5350  addi r7, r11, 0x5350
	ctx.r[7].s64 = ctx.r[11].s64 + 21328;
	// 826FD130: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826FD134: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826FD138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD13C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD148: 386ACD3C  addi r3, r10, -0x32c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12996;
	// 826FD14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD170: 4BD69CB1  bl 0x82466e20
	ctx.lr = 0x826FD174;
	sub_82466E20(ctx, base);
	// 826FD174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD188 size=108
    let mut pc: u32 = 0x826FD188;
    'dispatch: loop {
        match pc {
            0x826FD188 => {
    //   block [0x826FD188..0x826FD1F4)
	// 826FD188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD194: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD19C: 38EB5398  addi r7, r11, 0x5398
	ctx.r[7].s64 = ctx.r[11].s64 + 21400;
	// 826FD1A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD1A4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826FD1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD1AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD1B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD1B8: 386ACD6C  addi r3, r10, -0x3294
	ctx.r[3].s64 = ctx.r[10].s64 + -12948;
	// 826FD1BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD1DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD1E0: 4BD69C41  bl 0x82466e20
	ctx.lr = 0x826FD1E4;
	sub_82466E20(ctx, base);
	// 826FD1E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD1F8 size=112
    let mut pc: u32 = 0x826FD1F8;
    'dispatch: loop {
        match pc {
            0x826FD1F8 => {
    //   block [0x826FD1F8..0x826FD268)
	// 826FD1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD204: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD208: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD20C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FD210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD214: 390B53B0  addi r8, r11, 0x53b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21424;
	// 826FD218: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FD21C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826FD220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD224: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD230: 386ACD9C  addi r3, r10, -0x3264
	ctx.r[3].s64 = ctx.r[10].s64 + -12900;
	// 826FD234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD254: 4BD69BCD  bl 0x82466e20
	ctx.lr = 0x826FD258;
	sub_82466E20(ctx, base);
	// 826FD258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD268 size=108
    let mut pc: u32 = 0x826FD268;
    'dispatch: loop {
        match pc {
            0x826FD268 => {
    //   block [0x826FD268..0x826FD2D4)
	// 826FD268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD274: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD27C: 38EB53F8  addi r7, r11, 0x53f8
	ctx.r[7].s64 = ctx.r[11].s64 + 21496;
	// 826FD280: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FD284: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826FD288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD28C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD298: 386ACDCC  addi r3, r10, -0x3234
	ctx.r[3].s64 = ctx.r[10].s64 + -12852;
	// 826FD29C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD2BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD2C0: 4BD69B61  bl 0x82466e20
	ctx.lr = 0x826FD2C4;
	sub_82466E20(ctx, base);
	// 826FD2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD2D8 size=112
    let mut pc: u32 = 0x826FD2D8;
    'dispatch: loop {
        match pc {
            0x826FD2D8 => {
    //   block [0x826FD2D8..0x826FD348)
	// 826FD2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD2E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD2E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD2EC: 38AACDCC  addi r5, r10, -0x3234
	ctx.r[5].s64 = ctx.r[10].s64 + -12852;
	// 826FD2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD2F4: 390B5458  addi r8, r11, 0x5458
	ctx.r[8].s64 = ctx.r[11].s64 + 21592;
	// 826FD2F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FD2FC: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826FD300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD310: 386ACDFC  addi r3, r10, -0x3204
	ctx.r[3].s64 = ctx.r[10].s64 + -12804;
	// 826FD314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD334: 4BD69AED  bl 0x82466e20
	ctx.lr = 0x826FD338;
	sub_82466E20(ctx, base);
	// 826FD338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD348 size=96
    let mut pc: u32 = 0x826FD348;
    'dispatch: loop {
        match pc {
            0x826FD348 => {
    //   block [0x826FD348..0x826FD3A8)
	// 826FD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD354: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD35C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826FD360: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD368: 386ACE2C  addi r3, r10, -0x31d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12756;
	// 826FD36C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD374: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD394: 4BD69A8D  bl 0x82466e20
	ctx.lr = 0x826FD398;
	sub_82466E20(ctx, base);
	// 826FD398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD3A8 size=112
    let mut pc: u32 = 0x826FD3A8;
    'dispatch: loop {
        match pc {
            0x826FD3A8 => {
    //   block [0x826FD3A8..0x826FD418)
	// 826FD3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD3B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD3B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD3BC: 38AAEB3C  addi r5, r10, -0x14c4
	ctx.r[5].s64 = ctx.r[10].s64 + -5316;
	// 826FD3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD3C4: 390B54A0  addi r8, r11, 0x54a0
	ctx.r[8].s64 = ctx.r[11].s64 + 21664;
	// 826FD3C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826FD3CC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826FD3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD3D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD3E0: 386ACE5C  addi r3, r10, -0x31a4
	ctx.r[3].s64 = ctx.r[10].s64 + -12708;
	// 826FD3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD404: 4BD69A1D  bl 0x82466e20
	ctx.lr = 0x826FD408;
	sub_82466E20(ctx, base);
	// 826FD408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD418 size=96
    let mut pc: u32 = 0x826FD418;
    'dispatch: loop {
        match pc {
            0x826FD418 => {
    //   block [0x826FD418..0x826FD478)
	// 826FD418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD424: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD42C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826FD430: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD438: 386ACE8C  addi r3, r10, -0x3174
	ctx.r[3].s64 = ctx.r[10].s64 + -12660;
	// 826FD43C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD444: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD464: 4BD699BD  bl 0x82466e20
	ctx.lr = 0x826FD468;
	sub_82466E20(ctx, base);
	// 826FD468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD478 size=100
    let mut pc: u32 = 0x826FD478;
    'dispatch: loop {
        match pc {
            0x826FD478 => {
    //   block [0x826FD478..0x826FD4DC)
	// 826FD478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD48C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FD490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD498: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826FD49C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD4A4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FD4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD4AC: 386ACEBC  addi r3, r10, -0x3144
	ctx.r[3].s64 = ctx.r[10].s64 + -12612;
	// 826FD4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD4B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD4B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD4C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD4C8: 4BD69959  bl 0x82466e20
	ctx.lr = 0x826FD4CC;
	sub_82466E20(ctx, base);
	// 826FD4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD4E0 size=96
    let mut pc: u32 = 0x826FD4E0;
    'dispatch: loop {
        match pc {
            0x826FD4E0 => {
    //   block [0x826FD4E0..0x826FD540)
	// 826FD4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD4EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD4F4: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826FD4F8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD500: 386ACEEC  addi r3, r10, -0x3114
	ctx.r[3].s64 = ctx.r[10].s64 + -12564;
	// 826FD504: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD50C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD520: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD52C: 4BD698F5  bl 0x82466e20
	ctx.lr = 0x826FD530;
	sub_82466E20(ctx, base);
	// 826FD530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD540 size=112
    let mut pc: u32 = 0x826FD540;
    'dispatch: loop {
        match pc {
            0x826FD540 => {
    //   block [0x826FD540..0x826FD5B0)
	// 826FD540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD54C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD550: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD554: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 826FD558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD55C: 390B5500  addi r8, r11, 0x5500
	ctx.r[8].s64 = ctx.r[11].s64 + 21760;
	// 826FD560: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FD564: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826FD568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD56C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD578: 386ACF1C  addi r3, r10, -0x30e4
	ctx.r[3].s64 = ctx.r[10].s64 + -12516;
	// 826FD57C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD59C: 4BD69885  bl 0x82466e20
	ctx.lr = 0x826FD5A0;
	sub_82466E20(ctx, base);
	// 826FD5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD5B0 size=112
    let mut pc: u32 = 0x826FD5B0;
    'dispatch: loop {
        match pc {
            0x826FD5B0 => {
    //   block [0x826FD5B0..0x826FD620)
	// 826FD5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD5BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD5C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD5C4: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 826FD5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD5CC: 390B5530  addi r8, r11, 0x5530
	ctx.r[8].s64 = ctx.r[11].s64 + 21808;
	// 826FD5D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FD5D4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826FD5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD5DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD5E8: 386ACF4C  addi r3, r10, -0x30b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12468;
	// 826FD5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD60C: 4BD69815  bl 0x82466e20
	ctx.lr = 0x826FD610;
	sub_82466E20(ctx, base);
	// 826FD610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD620 size=100
    let mut pc: u32 = 0x826FD620;
    'dispatch: loop {
        match pc {
            0x826FD620 => {
    //   block [0x826FD620..0x826FD684)
	// 826FD620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD62C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD634: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 826FD638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD640: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826FD644: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD654: 386ACF7C  addi r3, r10, -0x3084
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	// 826FD658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD65C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD660: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD670: 4BD697B1  bl 0x82466e20
	ctx.lr = 0x826FD674;
	sub_82466E20(ctx, base);
	// 826FD674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD688 size=96
    let mut pc: u32 = 0x826FD688;
    'dispatch: loop {
        match pc {
            0x826FD688 => {
    //   block [0x826FD688..0x826FD6E8)
	// 826FD688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD694: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD69C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826FD6A0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD6A8: 386ACFAC  addi r3, r10, -0x3054
	ctx.r[3].s64 = ctx.r[10].s64 + -12372;
	// 826FD6AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD6B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD6B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD6B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD6C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD6C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD6D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD6D4: 4BD6974D  bl 0x82466e20
	ctx.lr = 0x826FD6D8;
	sub_82466E20(ctx, base);
	// 826FD6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD6E8 size=112
    let mut pc: u32 = 0x826FD6E8;
    'dispatch: loop {
        match pc {
            0x826FD6E8 => {
    //   block [0x826FD6E8..0x826FD758)
	// 826FD6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD6F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD6F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD6FC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 826FD700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD704: 390B5548  addi r8, r11, 0x5548
	ctx.r[8].s64 = ctx.r[11].s64 + 21832;
	// 826FD708: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FD70C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826FD710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD720: 386ACFDC  addi r3, r10, -0x3024
	ctx.r[3].s64 = ctx.r[10].s64 + -12324;
	// 826FD724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD744: 4BD696DD  bl 0x82466e20
	ctx.lr = 0x826FD748;
	sub_82466E20(ctx, base);
	// 826FD748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD758 size=96
    let mut pc: u32 = 0x826FD758;
    'dispatch: loop {
        match pc {
            0x826FD758 => {
    //   block [0x826FD758..0x826FD7B8)
	// 826FD758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD764: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD76C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826FD770: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD778: 386AD00C  addi r3, r10, -0x2ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -12276;
	// 826FD77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD784: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FD788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826FD79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD7A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826FD7A4: 4BD6967D  bl 0x82466e20
	ctx.lr = 0x826FD7A8;
	sub_82466E20(ctx, base);
	// 826FD7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD7B8 size=112
    let mut pc: u32 = 0x826FD7B8;
    'dispatch: loop {
        match pc {
            0x826FD7B8 => {
    //   block [0x826FD7B8..0x826FD828)
	// 826FD7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD7C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD7C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD7CC: 38AAD00C  addi r5, r10, -0x2ff4
	ctx.r[5].s64 = ctx.r[10].s64 + -12276;
	// 826FD7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD7D4: 390B5560  addi r8, r11, 0x5560
	ctx.r[8].s64 = ctx.r[11].s64 + 21856;
	// 826FD7D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FD7DC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826FD7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD7E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD7F0: 386AD03C  addi r3, r10, -0x2fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -12228;
	// 826FD7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD814: 4BD6960D  bl 0x82466e20
	ctx.lr = 0x826FD818;
	sub_82466E20(ctx, base);
	// 826FD818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD828 size=108
    let mut pc: u32 = 0x826FD828;
    'dispatch: loop {
        match pc {
            0x826FD828 => {
    //   block [0x826FD828..0x826FD894)
	// 826FD828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD834: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD83C: 38EB5578  addi r7, r11, 0x5578
	ctx.r[7].s64 = ctx.r[11].s64 + 21880;
	// 826FD840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826FD844: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826FD848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD84C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826FD854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD858: 386AD06C  addi r3, r10, -0x2f94
	ctx.r[3].s64 = ctx.r[10].s64 + -12180;
	// 826FD85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826FD860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826FD880: 4BD695A1  bl 0x82466e20
	ctx.lr = 0x826FD884;
	sub_82466E20(ctx, base);
	// 826FD884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD898 size=112
    let mut pc: u32 = 0x826FD898;
    'dispatch: loop {
        match pc {
            0x826FD898 => {
    //   block [0x826FD898..0x826FD908)
	// 826FD898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD8A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD8A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD8AC: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FD8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD8B4: 390B55D8  addi r8, r11, 0x55d8
	ctx.r[8].s64 = ctx.r[11].s64 + 21976;
	// 826FD8B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FD8BC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826FD8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD8C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD8D0: 386AD09C  addi r3, r10, -0x2f64
	ctx.r[3].s64 = ctx.r[10].s64 + -12132;
	// 826FD8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD8F4: 4BD6952D  bl 0x82466e20
	ctx.lr = 0x826FD8F8;
	sub_82466E20(ctx, base);
	// 826FD8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD908 size=112
    let mut pc: u32 = 0x826FD908;
    'dispatch: loop {
        match pc {
            0x826FD908 => {
    //   block [0x826FD908..0x826FD978)
	// 826FD908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD914: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD918: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD91C: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FD920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD924: 390B55F0  addi r8, r11, 0x55f0
	ctx.r[8].s64 = ctx.r[11].s64 + 22000;
	// 826FD928: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FD92C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826FD930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD934: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD940: 386AD0CC  addi r3, r10, -0x2f34
	ctx.r[3].s64 = ctx.r[10].s64 + -12084;
	// 826FD944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD964: 4BD694BD  bl 0x82466e20
	ctx.lr = 0x826FD968;
	sub_82466E20(ctx, base);
	// 826FD968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD978 size=112
    let mut pc: u32 = 0x826FD978;
    'dispatch: loop {
        match pc {
            0x826FD978 => {
    //   block [0x826FD978..0x826FD9E8)
	// 826FD978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD988: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD98C: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FD990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FD994: 390B5620  addi r8, r11, 0x5620
	ctx.r[8].s64 = ctx.r[11].s64 + 22048;
	// 826FD998: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FD99C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826FD9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FD9A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FD9A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FD9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FD9B0: 386AD0FC  addi r3, r10, -0x2f04
	ctx.r[3].s64 = ctx.r[10].s64 + -12036;
	// 826FD9B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FD9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FD9BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FD9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FD9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FD9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FD9CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FD9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FD9D4: 4BD6944D  bl 0x82466e20
	ctx.lr = 0x826FD9D8;
	sub_82466E20(ctx, base);
	// 826FD9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FD9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FD9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FD9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FD9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FD9E8 size=116
    let mut pc: u32 = 0x826FD9E8;
    'dispatch: loop {
        match pc {
            0x826FD9E8 => {
    //   block [0x826FD9E8..0x826FDA5C)
	// 826FD9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FD9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FD9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FD9F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FD9F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826FD9FC: 390B5638  addi r8, r11, 0x5638
	ctx.r[8].s64 = ctx.r[11].s64 + 22072;
	// 826FDA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDA04: 392ABA48  addi r9, r10, -0x45b8
	ctx.r[9].s64 = ctx.r[10].s64 + -17848;
	// 826FDA08: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDA0C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826FDA10: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FDA14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDA2C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826FDA30: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826FDA34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826FDA38: 386BD12C  addi r3, r11, -0x2ed4
	ctx.r[3].s64 = ctx.r[11].s64 + -11988;
	// 826FDA3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826FDA40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDA48: 4BD693D9  bl 0x82466e20
	ctx.lr = 0x826FDA4C;
	sub_82466E20(ctx, base);
	// 826FDA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDA60 size=112
    let mut pc: u32 = 0x826FDA60;
    'dispatch: loop {
        match pc {
            0x826FDA60 => {
    //   block [0x826FDA60..0x826FDAD0)
	// 826FDA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDA6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDA70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDA74: 38AACFDC  addi r5, r10, -0x3024
	ctx.r[5].s64 = ctx.r[10].s64 + -12324;
	// 826FDA78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDA7C: 390B5668  addi r8, r11, 0x5668
	ctx.r[8].s64 = ctx.r[11].s64 + 22120;
	// 826FDA80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FDA84: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826FDA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDA8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDA90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDA98: 386AD15C  addi r3, r10, -0x2ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -11940;
	// 826FDA9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDAAC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826FDAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDABC: 4BD69365  bl 0x82466e20
	ctx.lr = 0x826FDAC0;
	sub_82466E20(ctx, base);
	// 826FDAC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDAD0 size=112
    let mut pc: u32 = 0x826FDAD0;
    'dispatch: loop {
        match pc {
            0x826FDAD0 => {
    //   block [0x826FDAD0..0x826FDB40)
	// 826FDAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDAD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDAE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDAE4: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826FDAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDAEC: 390B5680  addi r8, r11, 0x5680
	ctx.r[8].s64 = ctx.r[11].s64 + 22144;
	// 826FDAF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FDAF4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826FDAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDAFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDB00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDB08: 386AD18C  addi r3, r10, -0x2e74
	ctx.r[3].s64 = ctx.r[10].s64 + -11892;
	// 826FDB0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDB2C: 4BD692F5  bl 0x82466e20
	ctx.lr = 0x826FDB30;
	sub_82466E20(ctx, base);
	// 826FDB30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDB40 size=112
    let mut pc: u32 = 0x826FDB40;
    'dispatch: loop {
        match pc {
            0x826FDB40 => {
    //   block [0x826FDB40..0x826FDBB0)
	// 826FDB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDB4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDB50: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDB54: 38AAD39C  addi r5, r10, -0x2c64
	ctx.r[5].s64 = ctx.r[10].s64 + -11364;
	// 826FDB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDB5C: 390B5698  addi r8, r11, 0x5698
	ctx.r[8].s64 = ctx.r[11].s64 + 22168;
	// 826FDB60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826FDB64: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826FDB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDB6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDB78: 386AD1BC  addi r3, r10, -0x2e44
	ctx.r[3].s64 = ctx.r[10].s64 + -11844;
	// 826FDB7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDB9C: 4BD69285  bl 0x82466e20
	ctx.lr = 0x826FDBA0;
	sub_82466E20(ctx, base);
	// 826FDBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDBB0 size=112
    let mut pc: u32 = 0x826FDBB0;
    'dispatch: loop {
        match pc {
            0x826FDBB0 => {
    //   block [0x826FDBB0..0x826FDC20)
	// 826FDBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDBBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDBC0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDBC4: 38AAD15C  addi r5, r10, -0x2ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -11940;
	// 826FDBC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDBCC: 390B56B0  addi r8, r11, 0x56b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22192;
	// 826FDBD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826FDBD4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826FDBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDBDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDBE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDBE8: 386AD1EC  addi r3, r10, -0x2e14
	ctx.r[3].s64 = ctx.r[10].s64 + -11796;
	// 826FDBEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDC0C: 4BD69215  bl 0x82466e20
	ctx.lr = 0x826FDC10;
	sub_82466E20(ctx, base);
	// 826FDC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826FDC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826FDC20 size=112
    let mut pc: u32 = 0x826FDC20;
    'dispatch: loop {
        match pc {
            0x826FDC20 => {
    //   block [0x826FDC20..0x826FDC90)
	// 826FDC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826FDC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826FDC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826FDC2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDC30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826FDC34: 38AAD18C  addi r5, r10, -0x2e74
	ctx.r[5].s64 = ctx.r[10].s64 + -11892;
	// 826FDC38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826FDC3C: 390B56F8  addi r8, r11, 0x56f8
	ctx.r[8].s64 = ctx.r[11].s64 + 22264;
	// 826FDC40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826FDC44: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826FDC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826FDC4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826FDC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826FDC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826FDC58: 386AD21C  addi r3, r10, -0x2de4
	ctx.r[3].s64 = ctx.r[10].s64 + -11748;
	// 826FDC5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826FDC60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826FDC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826FDC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826FDC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826FDC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826FDC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826FDC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826FDC7C: 4BD691A5  bl 0x82466e20
	ctx.lr = 0x826FDC80;
	sub_82466E20(ctx, base);
	// 826FDC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826FDC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826FDC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826FDC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


