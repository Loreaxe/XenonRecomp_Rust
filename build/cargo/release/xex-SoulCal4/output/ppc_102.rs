pub fn sub_82643458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643458 size=108
    let mut pc: u32 = 0x82643458;
    'dispatch: loop {
        match pc {
            0x82643458 => {
    //   block [0x82643458..0x826434C4)
	// 82643458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643464: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264346C: 38EBCAA8  addi r7, r11, -0x3558
	ctx.r[7].s64 = ctx.r[11].s64 + -13656;
	// 82643470: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82643474: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82643478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264347C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643488: 386AE9A4  addi r3, r10, -0x165c
	ctx.r[3].s64 = ctx.r[10].s64 + -5724;
	// 8264348C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264349C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826434A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826434A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826434A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826434AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826434B0: 4BE23971  bl 0x82466e20
	ctx.lr = 0x826434B4;
	sub_82466E20(ctx, base);
	// 826434B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826434B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826434BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826434C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826434C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826434C8 size=96
    let mut pc: u32 = 0x826434C8;
    'dispatch: loop {
        match pc {
            0x826434C8 => {
    //   block [0x826434C8..0x82643528)
	// 826434C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826434CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826434D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826434D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826434D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826434DC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826434E0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826434E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826434E8: 386AE9D4  addi r3, r10, -0x162c
	ctx.r[3].s64 = ctx.r[10].s64 + -5676;
	// 826434EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826434F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826434F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826434F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826434FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643508: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264350C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643510: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82643514: 4BE2390D  bl 0x82466e20
	ctx.lr = 0x82643518;
	sub_82466E20(ctx, base);
	// 82643518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264351C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643528 size=112
    let mut pc: u32 = 0x82643528;
    'dispatch: loop {
        match pc {
            0x82643528 => {
    //   block [0x82643528..0x82643598)
	// 82643528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264352C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643538: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264353C: 38AAE9D4  addi r5, r10, -0x162c
	ctx.r[5].s64 = ctx.r[10].s64 + -5676;
	// 82643540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82643544: 390BCAF0  addi r8, r11, -0x3510
	ctx.r[8].s64 = ctx.r[11].s64 + -13584;
	// 82643548: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264354C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82643550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264355C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643560: 386AEA04  addi r3, r10, -0x15fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5628;
	// 82643564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82643568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264356C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264357C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643584: 4BE2389D  bl 0x82466e20
	ctx.lr = 0x82643588;
	sub_82466E20(ctx, base);
	// 82643588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643598 size=108
    let mut pc: u32 = 0x82643598;
    'dispatch: loop {
        match pc {
            0x82643598 => {
    //   block [0x82643598..0x82643604)
	// 82643598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826435A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826435A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826435A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826435AC: 38EBCB20  addi r7, r11, -0x34e0
	ctx.r[7].s64 = ctx.r[11].s64 + -13536;
	// 826435B0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826435B4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826435B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826435BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826435C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826435C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826435C8: 386AEA34  addi r3, r10, -0x15cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5580;
	// 826435CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826435D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826435D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826435D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826435DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826435E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826435E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826435E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826435EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826435F0: 4BE23831  bl 0x82466e20
	ctx.lr = 0x826435F4;
	sub_82466E20(ctx, base);
	// 826435F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826435F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826435FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643608 size=108
    let mut pc: u32 = 0x82643608;
    'dispatch: loop {
        match pc {
            0x82643608 => {
    //   block [0x82643608..0x82643674)
	// 82643608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264360C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643614: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643618: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264361C: 38EBCBC8  addi r7, r11, -0x3438
	ctx.r[7].s64 = ctx.r[11].s64 + -13368;
	// 82643620: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643624: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82643628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264362C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643638: 386AEA64  addi r3, r10, -0x159c
	ctx.r[3].s64 = ctx.r[10].s64 + -5532;
	// 8264363C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264364C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264365C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643660: 4BE237C1  bl 0x82466e20
	ctx.lr = 0x82643664;
	sub_82466E20(ctx, base);
	// 82643664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264366C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643678 size=108
    let mut pc: u32 = 0x82643678;
    'dispatch: loop {
        match pc {
            0x82643678 => {
    //   block [0x82643678..0x826436E4)
	// 82643678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264367C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643684: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643688: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264368C: 38EBCBF8  addi r7, r11, -0x3408
	ctx.r[7].s64 = ctx.r[11].s64 + -13320;
	// 82643690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643694: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82643698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264369C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826436A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826436A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826436A8: 386AEA94  addi r3, r10, -0x156c
	ctx.r[3].s64 = ctx.r[10].s64 + -5484;
	// 826436AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826436B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826436B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826436B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826436BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826436C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826436C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826436C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826436CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826436D0: 4BE23751  bl 0x82466e20
	ctx.lr = 0x826436D4;
	sub_82466E20(ctx, base);
	// 826436D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826436D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826436DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826436E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826436E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826436E8 size=28
    let mut pc: u32 = 0x826436E8;
    'dispatch: loop {
        match pc {
            0x826436E8 => {
    //   block [0x826436E8..0x82643704)
	// 826436E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826436EC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826436F0: 394A09A0  addi r10, r10, 0x9a0
	ctx.r[10].s64 = ctx.r[10].s64 + 2464;
	// 826436F4: 816BCC28  lwz r11, -0x33d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13272 as u32) ) } as u64;
	// 826436F8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826436FC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82643700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643708 size=112
    let mut pc: u32 = 0x82643708;
    'dispatch: loop {
        match pc {
            0x82643708 => {
    //   block [0x82643708..0x82643778)
	// 82643708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643714: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82643718: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264371C: 392A85B0  addi r9, r10, -0x7a50
	ctx.r[9].s64 = ctx.r[10].s64 + -31312;
	// 82643720: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82643724: 390B09A0  addi r8, r11, 0x9a0
	ctx.r[8].s64 = ctx.r[11].s64 + 2464;
	// 82643728: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264372C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82643730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643740: 386AEAC4  addi r3, r10, -0x153c
	ctx.r[3].s64 = ctx.r[10].s64 + -5436;
	// 82643744: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82643748: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8264374C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264375C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643764: 4BE236BD  bl 0x82466e20
	ctx.lr = 0x82643768;
	sub_82466E20(ctx, base);
	// 82643768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264376C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643778 size=108
    let mut pc: u32 = 0x82643778;
    'dispatch: loop {
        match pc {
            0x82643778 => {
    //   block [0x82643778..0x826437E4)
	// 82643778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264377C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643784: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643788: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264378C: 38EBCC34  addi r7, r11, -0x33cc
	ctx.r[7].s64 = ctx.r[11].s64 + -13260;
	// 82643790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643794: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82643798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264379C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826437A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826437A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826437A8: 386AEAF4  addi r3, r10, -0x150c
	ctx.r[3].s64 = ctx.r[10].s64 + -5388;
	// 826437AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826437B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826437B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826437B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826437BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826437C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826437C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826437C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826437CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826437D0: 4BE23651  bl 0x82466e20
	ctx.lr = 0x826437D4;
	sub_82466E20(ctx, base);
	// 826437D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826437D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826437DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826437E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826437E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826437E8 size=108
    let mut pc: u32 = 0x826437E8;
    'dispatch: loop {
        match pc {
            0x826437E8 => {
    //   block [0x826437E8..0x82643854)
	// 826437E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826437EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826437F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826437F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826437F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826437FC: 38EBCC64  addi r7, r11, -0x339c
	ctx.r[7].s64 = ctx.r[11].s64 + -13212;
	// 82643800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643804: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82643808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264380C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643818: 386AEB24  addi r3, r10, -0x14dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5340;
	// 8264381C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264382C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264383C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643840: 4BE235E1  bl 0x82466e20
	ctx.lr = 0x82643844;
	sub_82466E20(ctx, base);
	// 82643844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264384C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643858 size=112
    let mut pc: u32 = 0x82643858;
    'dispatch: loop {
        match pc {
            0x82643858 => {
    //   block [0x82643858..0x826438C8)
	// 82643858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643864: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82643868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264386C: 392A85FC  addi r9, r10, -0x7a04
	ctx.r[9].s64 = ctx.r[10].s64 + -31236;
	// 82643870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643874: 390BCC98  addi r8, r11, -0x3368
	ctx.r[8].s64 = ctx.r[11].s64 + -13160;
	// 82643878: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264387C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82643880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264388C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643890: 386AEB54  addi r3, r10, -0x14ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5292;
	// 82643894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82643898: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264389C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826438A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826438A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826438A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826438AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826438B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826438B4: 4BE2356D  bl 0x82466e20
	ctx.lr = 0x826438B8;
	sub_82466E20(ctx, base);
	// 826438B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826438BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826438C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826438C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826438C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826438C8 size=108
    let mut pc: u32 = 0x826438C8;
    'dispatch: loop {
        match pc {
            0x826438C8 => {
    //   block [0x826438C8..0x82643934)
	// 826438C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826438CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826438D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826438D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826438D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826438DC: 38EBCCF8  addi r7, r11, -0x3308
	ctx.r[7].s64 = ctx.r[11].s64 + -13064;
	// 826438E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826438E4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826438E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826438EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826438F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826438F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826438F8: 386AEB84  addi r3, r10, -0x147c
	ctx.r[3].s64 = ctx.r[10].s64 + -5244;
	// 826438FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264390C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264391C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643920: 4BE23501  bl 0x82466e20
	ctx.lr = 0x82643924;
	sub_82466E20(ctx, base);
	// 82643924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264392C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643938 size=108
    let mut pc: u32 = 0x82643938;
    'dispatch: loop {
        match pc {
            0x82643938 => {
    //   block [0x82643938..0x826439A4)
	// 82643938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264393C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643944: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643948: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264394C: 38EBCDB8  addi r7, r11, -0x3248
	ctx.r[7].s64 = ctx.r[11].s64 + -12872;
	// 82643950: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82643954: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82643958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264395C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643968: 386AEBB4  addi r3, r10, -0x144c
	ctx.r[3].s64 = ctx.r[10].s64 + -5196;
	// 8264396C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264397C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264398C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643990: 4BE23491  bl 0x82466e20
	ctx.lr = 0x82643994;
	sub_82466E20(ctx, base);
	// 82643994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264399C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826439A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826439A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826439A8 size=108
    let mut pc: u32 = 0x826439A8;
    'dispatch: loop {
        match pc {
            0x826439A8 => {
    //   block [0x826439A8..0x82643A14)
	// 826439A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826439AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826439B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826439B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826439B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826439BC: 38EBCDD0  addi r7, r11, -0x3230
	ctx.r[7].s64 = ctx.r[11].s64 + -12848;
	// 826439C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826439C4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826439C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826439CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826439D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826439D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826439D8: 386AEBE4  addi r3, r10, -0x141c
	ctx.r[3].s64 = ctx.r[10].s64 + -5148;
	// 826439DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826439E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826439E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826439E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826439EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826439F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826439F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826439F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826439FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643A00: 4BE23421  bl 0x82466e20
	ctx.lr = 0x82643A04;
	sub_82466E20(ctx, base);
	// 82643A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643A18 size=108
    let mut pc: u32 = 0x82643A18;
    'dispatch: loop {
        match pc {
            0x82643A18 => {
    //   block [0x82643A18..0x82643A84)
	// 82643A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643A24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643A2C: 38EBCE48  addi r7, r11, -0x31b8
	ctx.r[7].s64 = ctx.r[11].s64 + -12728;
	// 82643A30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82643A34: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82643A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643A48: 386AEC14  addi r3, r10, -0x13ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5100;
	// 82643A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643A70: 4BE233B1  bl 0x82466e20
	ctx.lr = 0x82643A74;
	sub_82466E20(ctx, base);
	// 82643A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643A88 size=108
    let mut pc: u32 = 0x82643A88;
    'dispatch: loop {
        match pc {
            0x82643A88 => {
    //   block [0x82643A88..0x82643AF4)
	// 82643A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643A94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643A9C: 38EBCED8  addi r7, r11, -0x3128
	ctx.r[7].s64 = ctx.r[11].s64 + -12584;
	// 82643AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643AA4: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82643AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643AAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643AB8: 386AEC44  addi r3, r10, -0x13bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5052;
	// 82643ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643AE0: 4BE23341  bl 0x82466e20
	ctx.lr = 0x82643AE4;
	sub_82466E20(ctx, base);
	// 82643AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643AF8 size=108
    let mut pc: u32 = 0x82643AF8;
    'dispatch: loop {
        match pc {
            0x82643AF8 => {
    //   block [0x82643AF8..0x82643B64)
	// 82643AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643B04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643B0C: 38EBCF08  addi r7, r11, -0x30f8
	ctx.r[7].s64 = ctx.r[11].s64 + -12536;
	// 82643B10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82643B14: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82643B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643B1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643B28: 386AEC74  addi r3, r10, -0x138c
	ctx.r[3].s64 = ctx.r[10].s64 + -5004;
	// 82643B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643B50: 4BE232D1  bl 0x82466e20
	ctx.lr = 0x82643B54;
	sub_82466E20(ctx, base);
	// 82643B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82643B68 size=24
    let mut pc: u32 = 0x82643B68;
    'dispatch: loop {
        match pc {
            0x82643B68 => {
    //   block [0x82643B68..0x82643B80)
	// 82643B68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643B6C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82643B70: 394A0A60  addi r10, r10, 0xa60
	ctx.r[10].s64 = ctx.r[10].s64 + 2656;
	// 82643B74: 816BCFB0  lwz r11, -0x3050(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12368 as u32) ) } as u64;
	// 82643B78: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82643B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643B80 size=112
    let mut pc: u32 = 0x82643B80;
    'dispatch: loop {
        match pc {
            0x82643B80 => {
    //   block [0x82643B80..0x82643BF0)
	// 82643B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643B8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82643B90: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643B94: 392A8628  addi r9, r10, -0x79d8
	ctx.r[9].s64 = ctx.r[10].s64 + -31192;
	// 82643B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643B9C: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 82643BA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82643BA4: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82643BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82643BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643BB8: 386AECA4  addi r3, r10, -0x135c
	ctx.r[3].s64 = ctx.r[10].s64 + -4956;
	// 82643BBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82643BC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82643BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643BDC: 4BE23245  bl 0x82466e20
	ctx.lr = 0x82643BE0;
	sub_82466E20(ctx, base);
	// 82643BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643BF0 size=108
    let mut pc: u32 = 0x82643BF0;
    'dispatch: loop {
        match pc {
            0x82643BF0 => {
    //   block [0x82643BF0..0x82643C5C)
	// 82643BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643BFC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643C04: 38EBCFB8  addi r7, r11, -0x3048
	ctx.r[7].s64 = ctx.r[11].s64 + -12360;
	// 82643C08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643C0C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82643C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643C14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643C20: 386AECD4  addi r3, r10, -0x132c
	ctx.r[3].s64 = ctx.r[10].s64 + -4908;
	// 82643C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643C48: 4BE231D9  bl 0x82466e20
	ctx.lr = 0x82643C4C;
	sub_82466E20(ctx, base);
	// 82643C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643C60 size=112
    let mut pc: u32 = 0x82643C60;
    'dispatch: loop {
        match pc {
            0x82643C60 => {
    //   block [0x82643C60..0x82643CD0)
	// 82643C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643C6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82643C70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643C74: 392A866C  addi r9, r10, -0x7994
	ctx.r[9].s64 = ctx.r[10].s64 + -31124;
	// 82643C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643C7C: 390BCFE8  addi r8, r11, -0x3018
	ctx.r[8].s64 = ctx.r[11].s64 + -12312;
	// 82643C80: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82643C84: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82643C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643C8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82643C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643C98: 386AED04  addi r3, r10, -0x12fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4860;
	// 82643C9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82643CA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82643CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643CBC: 4BE23165  bl 0x82466e20
	ctx.lr = 0x82643CC0;
	sub_82466E20(ctx, base);
	// 82643CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643CD0 size=108
    let mut pc: u32 = 0x82643CD0;
    'dispatch: loop {
        match pc {
            0x82643CD0 => {
    //   block [0x82643CD0..0x82643D3C)
	// 82643CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643CDC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643CE4: 38EBD090  addi r7, r11, -0x2f70
	ctx.r[7].s64 = ctx.r[11].s64 + -12144;
	// 82643CE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82643CEC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82643CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643CF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643D00: 386AED34  addi r3, r10, -0x12cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4812;
	// 82643D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643D28: 4BE230F9  bl 0x82466e20
	ctx.lr = 0x82643D2C;
	sub_82466E20(ctx, base);
	// 82643D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643D40 size=108
    let mut pc: u32 = 0x82643D40;
    'dispatch: loop {
        match pc {
            0x82643D40 => {
    //   block [0x82643D40..0x82643DAC)
	// 82643D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643D4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643D54: 38EBD0A8  addi r7, r11, -0x2f58
	ctx.r[7].s64 = ctx.r[11].s64 + -12120;
	// 82643D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643D5C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82643D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643D64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643D70: 386AED64  addi r3, r10, -0x129c
	ctx.r[3].s64 = ctx.r[10].s64 + -4764;
	// 82643D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643D98: 4BE23089  bl 0x82466e20
	ctx.lr = 0x82643D9C;
	sub_82466E20(ctx, base);
	// 82643D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82643DB0 size=24
    let mut pc: u32 = 0x82643DB0;
    'dispatch: loop {
        match pc {
            0x82643DB0 => {
    //   block [0x82643DB0..0x82643DC8)
	// 82643DB0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643DB4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82643DB8: 394A0AD8  addi r10, r10, 0xad8
	ctx.r[10].s64 = ctx.r[10].s64 + 2776;
	// 82643DBC: 816BD0D8  lwz r11, -0x2f28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12072 as u32) ) } as u64;
	// 82643DC0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82643DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643DC8 size=112
    let mut pc: u32 = 0x82643DC8;
    'dispatch: loop {
        match pc {
            0x82643DC8 => {
    //   block [0x82643DC8..0x82643E38)
	// 82643DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643DD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82643DD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643DDC: 392A86A8  addi r9, r10, -0x7958
	ctx.r[9].s64 = ctx.r[10].s64 + -31064;
	// 82643DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643DE4: 390B0AD8  addi r8, r11, 0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + 2776;
	// 82643DE8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82643DEC: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82643DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82643DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643E00: 386AED94  addi r3, r10, -0x126c
	ctx.r[3].s64 = ctx.r[10].s64 + -4716;
	// 82643E04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82643E08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82643E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643E24: 4BE22FFD  bl 0x82466e20
	ctx.lr = 0x82643E28;
	sub_82466E20(ctx, base);
	// 82643E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643E38 size=108
    let mut pc: u32 = 0x82643E38;
    'dispatch: loop {
        match pc {
            0x82643E38 => {
    //   block [0x82643E38..0x82643EA4)
	// 82643E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643E44: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643E4C: 38EBD0DC  addi r7, r11, -0x2f24
	ctx.r[7].s64 = ctx.r[11].s64 + -12068;
	// 82643E50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82643E54: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82643E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643E5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643E68: 386AEDC4  addi r3, r10, -0x123c
	ctx.r[3].s64 = ctx.r[10].s64 + -4668;
	// 82643E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643E90: 4BE22F91  bl 0x82466e20
	ctx.lr = 0x82643E94;
	sub_82466E20(ctx, base);
	// 82643E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643EA8 size=108
    let mut pc: u32 = 0x82643EA8;
    'dispatch: loop {
        match pc {
            0x82643EA8 => {
    //   block [0x82643EA8..0x82643F14)
	// 82643EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643EB4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643EBC: 38EBD0F8  addi r7, r11, -0x2f08
	ctx.r[7].s64 = ctx.r[11].s64 + -12040;
	// 82643EC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82643EC4: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82643EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643ED8: 386AEDF4  addi r3, r10, -0x120c
	ctx.r[3].s64 = ctx.r[10].s64 + -4620;
	// 82643EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643F00: 4BE22F21  bl 0x82466e20
	ctx.lr = 0x82643F04;
	sub_82466E20(ctx, base);
	// 82643F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643F18 size=108
    let mut pc: u32 = 0x82643F18;
    'dispatch: loop {
        match pc {
            0x82643F18 => {
    //   block [0x82643F18..0x82643F84)
	// 82643F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643F24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643F2C: 38EBD140  addi r7, r11, -0x2ec0
	ctx.r[7].s64 = ctx.r[11].s64 + -11968;
	// 82643F30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643F34: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82643F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643F48: 386AEE24  addi r3, r10, -0x11dc
	ctx.r[3].s64 = ctx.r[10].s64 + -4572;
	// 82643F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643F70: 4BE22EB1  bl 0x82466e20
	ctx.lr = 0x82643F74;
	sub_82466E20(ctx, base);
	// 82643F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643F88 size=108
    let mut pc: u32 = 0x82643F88;
    'dispatch: loop {
        match pc {
            0x82643F88 => {
    //   block [0x82643F88..0x82643FF4)
	// 82643F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643F94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643F9C: 38EBD170  addi r7, r11, -0x2e90
	ctx.r[7].s64 = ctx.r[11].s64 + -11920;
	// 82643FA0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82643FA4: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82643FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643FAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643FB8: 386AEE54  addi r3, r10, -0x11ac
	ctx.r[3].s64 = ctx.r[10].s64 + -4524;
	// 82643FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643FE0: 4BE22E41  bl 0x82466e20
	ctx.lr = 0x82643FE4;
	sub_82466E20(ctx, base);
	// 82643FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643FF8 size=108
    let mut pc: u32 = 0x82643FF8;
    'dispatch: loop {
        match pc {
            0x82643FF8 => {
    //   block [0x82643FF8..0x82644064)
	// 82643FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644004: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264400C: 38EBD290  addi r7, r11, -0x2d70
	ctx.r[7].s64 = ctx.r[11].s64 + -11632;
	// 82644010: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82644014: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82644018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264401C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644028: 386AEE84  addi r3, r10, -0x117c
	ctx.r[3].s64 = ctx.r[10].s64 + -4476;
	// 8264402C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264403C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264404C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644050: 4BE22DD1  bl 0x82466e20
	ctx.lr = 0x82644054;
	sub_82466E20(ctx, base);
	// 82644054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264405C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644068 size=108
    let mut pc: u32 = 0x82644068;
    'dispatch: loop {
        match pc {
            0x82644068 => {
    //   block [0x82644068..0x826440D4)
	// 82644068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644074: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264407C: 38EBD320  addi r7, r11, -0x2ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -11488;
	// 82644080: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82644084: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82644088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264408C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644098: 386AEEB4  addi r3, r10, -0x114c
	ctx.r[3].s64 = ctx.r[10].s64 + -4428;
	// 8264409C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826440A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826440A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826440A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826440AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826440B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826440B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826440B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826440BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826440C0: 4BE22D61  bl 0x82466e20
	ctx.lr = 0x826440C4;
	sub_82466E20(ctx, base);
	// 826440C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826440C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826440CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826440D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826440D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826440D8 size=108
    let mut pc: u32 = 0x826440D8;
    'dispatch: loop {
        match pc {
            0x826440D8 => {
    //   block [0x826440D8..0x82644144)
	// 826440D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826440DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826440E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826440E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826440E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826440EC: 38EBD3E0  addi r7, r11, -0x2c20
	ctx.r[7].s64 = ctx.r[11].s64 + -11296;
	// 826440F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826440F4: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826440F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826440FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644108: 386AEEE4  addi r3, r10, -0x111c
	ctx.r[3].s64 = ctx.r[10].s64 + -4380;
	// 8264410C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264411C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264412C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644130: 4BE22CF1  bl 0x82466e20
	ctx.lr = 0x82644134;
	sub_82466E20(ctx, base);
	// 82644134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264413C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644148 size=108
    let mut pc: u32 = 0x82644148;
    'dispatch: loop {
        match pc {
            0x82644148 => {
    //   block [0x82644148..0x826441B4)
	// 82644148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644154: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264415C: 38EBD4B8  addi r7, r11, -0x2b48
	ctx.r[7].s64 = ctx.r[11].s64 + -11080;
	// 82644160: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82644164: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82644168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264416C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644178: 386AEF14  addi r3, r10, -0x10ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4332;
	// 8264417C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264419C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826441A0: 4BE22C81  bl 0x82466e20
	ctx.lr = 0x826441A4;
	sub_82466E20(ctx, base);
	// 826441A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826441A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826441AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826441B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826441B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826441B8 size=108
    let mut pc: u32 = 0x826441B8;
    'dispatch: loop {
        match pc {
            0x826441B8 => {
    //   block [0x826441B8..0x82644224)
	// 826441B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826441BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826441C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826441C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826441C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826441CC: 38EBD578  addi r7, r11, -0x2a88
	ctx.r[7].s64 = ctx.r[11].s64 + -10888;
	// 826441D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826441D4: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826441D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826441DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826441E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826441E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826441E8: 386AEF44  addi r3, r10, -0x10bc
	ctx.r[3].s64 = ctx.r[10].s64 + -4284;
	// 826441EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826441F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826441F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826441F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826441FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264420C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644210: 4BE22C11  bl 0x82466e20
	ctx.lr = 0x82644214;
	sub_82466E20(ctx, base);
	// 82644214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264421C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644228 size=108
    let mut pc: u32 = 0x82644228;
    'dispatch: loop {
        match pc {
            0x82644228 => {
    //   block [0x82644228..0x82644294)
	// 82644228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264422C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644234: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264423C: 38EBD620  addi r7, r11, -0x29e0
	ctx.r[7].s64 = ctx.r[11].s64 + -10720;
	// 82644240: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82644244: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82644248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264424C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644258: 386AEF74  addi r3, r10, -0x108c
	ctx.r[3].s64 = ctx.r[10].s64 + -4236;
	// 8264425C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264426C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264427C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644280: 4BE22BA1  bl 0x82466e20
	ctx.lr = 0x82644284;
	sub_82466E20(ctx, base);
	// 82644284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264428C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644298 size=108
    let mut pc: u32 = 0x82644298;
    'dispatch: loop {
        match pc {
            0x82644298 => {
    //   block [0x82644298..0x82644304)
	// 82644298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826442A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826442A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826442A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826442AC: 38EBD728  addi r7, r11, -0x28d8
	ctx.r[7].s64 = ctx.r[11].s64 + -10456;
	// 826442B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826442B4: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826442B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826442BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826442C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826442C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826442C8: 386AEFA4  addi r3, r10, -0x105c
	ctx.r[3].s64 = ctx.r[10].s64 + -4188;
	// 826442CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826442D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826442D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826442D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826442DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826442E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826442E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826442E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826442EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826442F0: 4BE22B31  bl 0x82466e20
	ctx.lr = 0x826442F4;
	sub_82466E20(ctx, base);
	// 826442F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826442F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826442FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644308 size=108
    let mut pc: u32 = 0x82644308;
    'dispatch: loop {
        match pc {
            0x82644308 => {
    //   block [0x82644308..0x82644374)
	// 82644308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264430C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644314: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264431C: 38EBD788  addi r7, r11, -0x2878
	ctx.r[7].s64 = ctx.r[11].s64 + -10360;
	// 82644320: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82644324: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82644328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264432C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644338: 386AEFD4  addi r3, r10, -0x102c
	ctx.r[3].s64 = ctx.r[10].s64 + -4140;
	// 8264433C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264434C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264435C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644360: 4BE22AC1  bl 0x82466e20
	ctx.lr = 0x82644364;
	sub_82466E20(ctx, base);
	// 82644364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264436C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644378 size=108
    let mut pc: u32 = 0x82644378;
    'dispatch: loop {
        match pc {
            0x82644378 => {
    //   block [0x82644378..0x826443E4)
	// 82644378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644384: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264438C: 38EBD878  addi r7, r11, -0x2788
	ctx.r[7].s64 = ctx.r[11].s64 + -10120;
	// 82644390: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82644394: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82644398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264439C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826443A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826443A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826443A8: 386AF004  addi r3, r10, -0xffc
	ctx.r[3].s64 = ctx.r[10].s64 + -4092;
	// 826443AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826443B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826443B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826443B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826443BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826443C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826443C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826443C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826443CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826443D0: 4BE22A51  bl 0x82466e20
	ctx.lr = 0x826443D4;
	sub_82466E20(ctx, base);
	// 826443D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826443D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826443DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826443E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826443E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826443E8 size=108
    let mut pc: u32 = 0x826443E8;
    'dispatch: loop {
        match pc {
            0x826443E8 => {
    //   block [0x826443E8..0x82644454)
	// 826443E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826443EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826443F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826443F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826443F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826443FC: 38EBD950  addi r7, r11, -0x26b0
	ctx.r[7].s64 = ctx.r[11].s64 + -9904;
	// 82644400: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82644404: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82644408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264440C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644418: 386AF034  addi r3, r10, -0xfcc
	ctx.r[3].s64 = ctx.r[10].s64 + -4044;
	// 8264441C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264442C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264443C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644440: 4BE229E1  bl 0x82466e20
	ctx.lr = 0x82644444;
	sub_82466E20(ctx, base);
	// 82644444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264444C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644458 size=108
    let mut pc: u32 = 0x82644458;
    'dispatch: loop {
        match pc {
            0x82644458 => {
    //   block [0x82644458..0x826444C4)
	// 82644458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644464: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264446C: 38EBD980  addi r7, r11, -0x2680
	ctx.r[7].s64 = ctx.r[11].s64 + -9856;
	// 82644470: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82644474: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82644478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264447C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644488: 386AF064  addi r3, r10, -0xf9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3996;
	// 8264448C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264449C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826444A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826444A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826444A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826444AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826444B0: 4BE22971  bl 0x82466e20
	ctx.lr = 0x826444B4;
	sub_82466E20(ctx, base);
	// 826444B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826444B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826444BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826444C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826444C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826444C8 size=112
    let mut pc: u32 = 0x826444C8;
    'dispatch: loop {
        match pc {
            0x826444C8 => {
    //   block [0x826444C8..0x82644538)
	// 826444C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826444CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826444D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826444D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826444D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826444DC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826444E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826444E4: 390BD998  addi r8, r11, -0x2668
	ctx.r[8].s64 = ctx.r[11].s64 + -9832;
	// 826444E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826444EC: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826444F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826444F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826444F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826444FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644500: 386AF094  addi r3, r10, -0xf6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3948;
	// 82644504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264450C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264451C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644524: 4BE228FD  bl 0x82466e20
	ctx.lr = 0x82644528;
	sub_82466E20(ctx, base);
	// 82644528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264452C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644538 size=108
    let mut pc: u32 = 0x82644538;
    'dispatch: loop {
        match pc {
            0x82644538 => {
    //   block [0x82644538..0x826445A4)
	// 82644538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644544: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264454C: 38EBD9E0  addi r7, r11, -0x2620
	ctx.r[7].s64 = ctx.r[11].s64 + -9760;
	// 82644550: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82644554: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82644558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264455C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644568: 386AF0C4  addi r3, r10, -0xf3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3900;
	// 8264456C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264457C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264458C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644590: 4BE22891  bl 0x82466e20
	ctx.lr = 0x82644594;
	sub_82466E20(ctx, base);
	// 82644594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264459C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826445A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826445A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826445A8 size=112
    let mut pc: u32 = 0x826445A8;
    'dispatch: loop {
        match pc {
            0x826445A8 => {
    //   block [0x826445A8..0x82644618)
	// 826445A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826445AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826445B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826445B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826445B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826445BC: 38AAF0C4  addi r5, r10, -0xf3c
	ctx.r[5].s64 = ctx.r[10].s64 + -3900;
	// 826445C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826445C4: 390BDA40  addi r8, r11, -0x25c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9664;
	// 826445C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826445CC: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826445D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826445D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826445D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826445DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826445E0: 386AF0F4  addi r3, r10, -0xf0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3852;
	// 826445E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826445E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826445EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826445F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826445F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826445F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826445FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644604: 4BE2281D  bl 0x82466e20
	ctx.lr = 0x82644608;
	sub_82466E20(ctx, base);
	// 82644608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264460C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644618 size=96
    let mut pc: u32 = 0x82644618;
    'dispatch: loop {
        match pc {
            0x82644618 => {
    //   block [0x82644618..0x82644678)
	// 82644618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264461C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264462C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82644630: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644638: 386AF124  addi r3, r10, -0xedc
	ctx.r[3].s64 = ctx.r[10].s64 + -3804;
	// 8264463C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644644: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82644648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264464C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264465C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82644664: 4BE227BD  bl 0x82466e20
	ctx.lr = 0x82644668;
	sub_82466E20(ctx, base);
	// 82644668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264466C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644678 size=112
    let mut pc: u32 = 0x82644678;
    'dispatch: loop {
        match pc {
            0x82644678 => {
    //   block [0x82644678..0x826446E8)
	// 82644678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644684: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264468C: 38AA06E4  addi r5, r10, 0x6e4
	ctx.r[5].s64 = ctx.r[10].s64 + 1764;
	// 82644690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644694: 390BDA88  addi r8, r11, -0x2578
	ctx.r[8].s64 = ctx.r[11].s64 + -9592;
	// 82644698: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264469C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826446A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826446A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826446A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826446AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826446B0: 386AF154  addi r3, r10, -0xeac
	ctx.r[3].s64 = ctx.r[10].s64 + -3756;
	// 826446B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826446B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826446BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826446C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826446C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826446C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826446CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826446D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826446D4: 4BE2274D  bl 0x82466e20
	ctx.lr = 0x826446D8;
	sub_82466E20(ctx, base);
	// 826446D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826446DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826446E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826446E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826446E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826446E8 size=96
    let mut pc: u32 = 0x826446E8;
    'dispatch: loop {
        match pc {
            0x826446E8 => {
    //   block [0x826446E8..0x82644748)
	// 826446E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826446EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826446F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826446F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826446F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826446FC: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82644700: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644708: 386AF184  addi r3, r10, -0xe7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3708;
	// 8264470C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644714: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82644718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264472C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82644734: 4BE226ED  bl 0x82466e20
	ctx.lr = 0x82644738;
	sub_82466E20(ctx, base);
	// 82644738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264473C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644748 size=100
    let mut pc: u32 = 0x82644748;
    'dispatch: loop {
        match pc {
            0x82644748 => {
    //   block [0x82644748..0x826447AC)
	// 82644748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264475C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82644760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644768: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8264476C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644774: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82644778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264477C: 386AF1B4  addi r3, r10, -0xe4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3660;
	// 82644780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644788: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644790: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82644794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644798: 4BE22689  bl 0x82466e20
	ctx.lr = 0x8264479C;
	sub_82466E20(ctx, base);
	// 8264479C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826447A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826447A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826447A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826447B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826447B0 size=96
    let mut pc: u32 = 0x826447B0;
    'dispatch: loop {
        match pc {
            0x826447B0 => {
    //   block [0x826447B0..0x82644810)
	// 826447B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826447B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826447B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826447BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826447C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826447C4: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826447C8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826447CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826447D0: 386AF1E4  addi r3, r10, -0xe1c
	ctx.r[3].s64 = ctx.r[10].s64 + -3612;
	// 826447D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826447D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826447DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826447E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826447E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826447E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826447EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826447F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826447F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826447F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826447FC: 4BE22625  bl 0x82466e20
	ctx.lr = 0x82644800;
	sub_82466E20(ctx, base);
	// 82644800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264480C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644810 size=112
    let mut pc: u32 = 0x82644810;
    'dispatch: loop {
        match pc {
            0x82644810 => {
    //   block [0x82644810..0x82644880)
	// 82644810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264481C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644820: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644824: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82644828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264482C: 390BDAE8  addi r8, r11, -0x2518
	ctx.r[8].s64 = ctx.r[11].s64 + -9496;
	// 82644830: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82644834: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82644838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264483C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644848: 386AF214  addi r3, r10, -0xdec
	ctx.r[3].s64 = ctx.r[10].s64 + -3564;
	// 8264484C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264485C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264486C: 4BE225B5  bl 0x82466e20
	ctx.lr = 0x82644870;
	sub_82466E20(ctx, base);
	// 82644870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264487C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644880 size=112
    let mut pc: u32 = 0x82644880;
    'dispatch: loop {
        match pc {
            0x82644880 => {
    //   block [0x82644880..0x826448F0)
	// 82644880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264488C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644890: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644894: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82644898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264489C: 390BDB18  addi r8, r11, -0x24e8
	ctx.r[8].s64 = ctx.r[11].s64 + -9448;
	// 826448A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826448A4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826448A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826448AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826448B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826448B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826448B8: 386AF244  addi r3, r10, -0xdbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3516;
	// 826448BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826448C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826448C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826448C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826448CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826448D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826448D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826448D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826448DC: 4BE22545  bl 0x82466e20
	ctx.lr = 0x826448E0;
	sub_82466E20(ctx, base);
	// 826448E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826448E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826448E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826448EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826448F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826448F0 size=100
    let mut pc: u32 = 0x826448F0;
    'dispatch: loop {
        match pc {
            0x826448F0 => {
    //   block [0x826448F0..0x82644954)
	// 826448F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826448F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826448F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826448FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644904: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82644908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264490C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644910: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82644914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264491C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644924: 386AF274  addi r3, r10, -0xd8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3468;
	// 82644928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264492C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82644934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264493C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644940: 4BE224E1  bl 0x82466e20
	ctx.lr = 0x82644944;
	sub_82466E20(ctx, base);
	// 82644944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264494C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644958 size=96
    let mut pc: u32 = 0x82644958;
    'dispatch: loop {
        match pc {
            0x82644958 => {
    //   block [0x82644958..0x826449B8)
	// 82644958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264495C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264496C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82644970: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644978: 386AF2A4  addi r3, r10, -0xd5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3420;
	// 8264497C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82644988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264498C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644998: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264499C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826449A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826449A4: 4BE2247D  bl 0x82466e20
	ctx.lr = 0x826449A8;
	sub_82466E20(ctx, base);
	// 826449A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826449AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826449B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826449B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826449B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826449B8 size=112
    let mut pc: u32 = 0x826449B8;
    'dispatch: loop {
        match pc {
            0x826449B8 => {
    //   block [0x826449B8..0x82644A28)
	// 826449B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826449BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826449C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826449C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826449C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826449CC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826449D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826449D4: 390BDB30  addi r8, r11, -0x24d0
	ctx.r[8].s64 = ctx.r[11].s64 + -9424;
	// 826449D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826449DC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826449E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826449E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826449E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826449EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826449F0: 386AF2D4  addi r3, r10, -0xd2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3372;
	// 826449F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826449F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826449FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644A14: 4BE2240D  bl 0x82466e20
	ctx.lr = 0x82644A18;
	sub_82466E20(ctx, base);
	// 82644A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644A28 size=108
    let mut pc: u32 = 0x82644A28;
    'dispatch: loop {
        match pc {
            0x82644A28 => {
    //   block [0x82644A28..0x82644A94)
	// 82644A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644A34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644A3C: 38EBDB48  addi r7, r11, -0x24b8
	ctx.r[7].s64 = ctx.r[11].s64 + -9400;
	// 82644A40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82644A44: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82644A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644A4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644A58: 386AF304  addi r3, r10, -0xcfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3324;
	// 82644A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644A80: 4BE223A1  bl 0x82466e20
	ctx.lr = 0x82644A84;
	sub_82466E20(ctx, base);
	// 82644A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644A98 size=112
    let mut pc: u32 = 0x82644A98;
    'dispatch: loop {
        match pc {
            0x82644A98 => {
    //   block [0x82644A98..0x82644B08)
	// 82644A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644AA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644AAC: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82644AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644AB4: 390BDBA8  addi r8, r11, -0x2458
	ctx.r[8].s64 = ctx.r[11].s64 + -9304;
	// 82644AB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82644ABC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82644AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644AC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644AD0: 386AF334  addi r3, r10, -0xccc
	ctx.r[3].s64 = ctx.r[10].s64 + -3276;
	// 82644AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644AF4: 4BE2232D  bl 0x82466e20
	ctx.lr = 0x82644AF8;
	sub_82466E20(ctx, base);
	// 82644AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644B08 size=112
    let mut pc: u32 = 0x82644B08;
    'dispatch: loop {
        match pc {
            0x82644B08 => {
    //   block [0x82644B08..0x82644B78)
	// 82644B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644B18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644B1C: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 82644B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644B24: 390BDBC0  addi r8, r11, -0x2440
	ctx.r[8].s64 = ctx.r[11].s64 + -9280;
	// 82644B28: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82644B2C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82644B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644B34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644B40: 386AF364  addi r3, r10, -0xc9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3228;
	// 82644B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644B64: 4BE222BD  bl 0x82466e20
	ctx.lr = 0x82644B68;
	sub_82466E20(ctx, base);
	// 82644B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644B78 size=112
    let mut pc: u32 = 0x82644B78;
    'dispatch: loop {
        match pc {
            0x82644B78 => {
    //   block [0x82644B78..0x82644BE8)
	// 82644B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644B84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644B88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644B8C: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 82644B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644B94: 390BDBF0  addi r8, r11, -0x2410
	ctx.r[8].s64 = ctx.r[11].s64 + -9232;
	// 82644B98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82644B9C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82644BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644BA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644BB0: 386AF394  addi r3, r10, -0xc6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3180;
	// 82644BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644BD4: 4BE2224D  bl 0x82466e20
	ctx.lr = 0x82644BD8;
	sub_82466E20(ctx, base);
	// 82644BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644BE8 size=112
    let mut pc: u32 = 0x82644BE8;
    'dispatch: loop {
        match pc {
            0x82644BE8 => {
    //   block [0x82644BE8..0x82644C58)
	// 82644BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644BF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644BF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644BFC: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82644C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644C04: 390BDC08  addi r8, r11, -0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9208;
	// 82644C08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82644C0C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82644C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644C14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644C20: 386AF3C4  addi r3, r10, -0xc3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3132;
	// 82644C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644C44: 4BE221DD  bl 0x82466e20
	ctx.lr = 0x82644C48;
	sub_82466E20(ctx, base);
	// 82644C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644C58 size=112
    let mut pc: u32 = 0x82644C58;
    'dispatch: loop {
        match pc {
            0x82644C58 => {
    //   block [0x82644C58..0x82644CC8)
	// 82644C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644C68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644C6C: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 82644C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644C74: 390BDC38  addi r8, r11, -0x23c8
	ctx.r[8].s64 = ctx.r[11].s64 + -9160;
	// 82644C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82644C7C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82644C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644C84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644C90: 386AF3F4  addi r3, r10, -0xc0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3084;
	// 82644C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644CB4: 4BE2216D  bl 0x82466e20
	ctx.lr = 0x82644CB8;
	sub_82466E20(ctx, base);
	// 82644CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644CC8 size=112
    let mut pc: u32 = 0x82644CC8;
    'dispatch: loop {
        match pc {
            0x82644CC8 => {
    //   block [0x82644CC8..0x82644D38)
	// 82644CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644CD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644CDC: 38AAF8D4  addi r5, r10, -0x72c
	ctx.r[5].s64 = ctx.r[10].s64 + -1836;
	// 82644CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644CE4: 390BDC50  addi r8, r11, -0x23b0
	ctx.r[8].s64 = ctx.r[11].s64 + -9136;
	// 82644CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82644CEC: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82644CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644CF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644D00: 386AF424  addi r3, r10, -0xbdc
	ctx.r[3].s64 = ctx.r[10].s64 + -3036;
	// 82644D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644D24: 4BE220FD  bl 0x82466e20
	ctx.lr = 0x82644D28;
	sub_82466E20(ctx, base);
	// 82644D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644D38 size=112
    let mut pc: u32 = 0x82644D38;
    'dispatch: loop {
        match pc {
            0x82644D38 => {
    //   block [0x82644D38..0x82644DA8)
	// 82644D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644D44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644D48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644D4C: 38AAF634  addi r5, r10, -0x9cc
	ctx.r[5].s64 = ctx.r[10].s64 + -2508;
	// 82644D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644D54: 390BDC68  addi r8, r11, -0x2398
	ctx.r[8].s64 = ctx.r[11].s64 + -9112;
	// 82644D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82644D5C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82644D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644D64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644D70: 386AF454  addi r3, r10, -0xbac
	ctx.r[3].s64 = ctx.r[10].s64 + -2988;
	// 82644D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644D94: 4BE2208D  bl 0x82466e20
	ctx.lr = 0x82644D98;
	sub_82466E20(ctx, base);
	// 82644D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644DA8 size=112
    let mut pc: u32 = 0x82644DA8;
    'dispatch: loop {
        match pc {
            0x82644DA8 => {
    //   block [0x82644DA8..0x82644E18)
	// 82644DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644DB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644DBC: 38AAF3F4  addi r5, r10, -0xc0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3084;
	// 82644DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644DC4: 390BDC80  addi r8, r11, -0x2380
	ctx.r[8].s64 = ctx.r[11].s64 + -9088;
	// 82644DC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82644DCC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82644DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644DD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644DE0: 386AF484  addi r3, r10, -0xb7c
	ctx.r[3].s64 = ctx.r[10].s64 + -2940;
	// 82644DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644E04: 4BE2201D  bl 0x82466e20
	ctx.lr = 0x82644E08;
	sub_82466E20(ctx, base);
	// 82644E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644E18 size=112
    let mut pc: u32 = 0x82644E18;
    'dispatch: loop {
        match pc {
            0x82644E18 => {
    //   block [0x82644E18..0x82644E88)
	// 82644E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644E24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644E28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644E2C: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82644E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644E34: 390BDCC8  addi r8, r11, -0x2338
	ctx.r[8].s64 = ctx.r[11].s64 + -9016;
	// 82644E38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82644E3C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82644E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644E44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644E50: 386AF4B4  addi r3, r10, -0xb4c
	ctx.r[3].s64 = ctx.r[10].s64 + -2892;
	// 82644E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644E74: 4BE21FAD  bl 0x82466e20
	ctx.lr = 0x82644E78;
	sub_82466E20(ctx, base);
	// 82644E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644E88 size=112
    let mut pc: u32 = 0x82644E88;
    'dispatch: loop {
        match pc {
            0x82644E88 => {
    //   block [0x82644E88..0x82644EF8)
	// 82644E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644E94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644E98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644E9C: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82644EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644EA4: 390BDCF8  addi r8, r11, -0x2308
	ctx.r[8].s64 = ctx.r[11].s64 + -8968;
	// 82644EA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82644EAC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82644EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644EB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644EC0: 386AF4E4  addi r3, r10, -0xb1c
	ctx.r[3].s64 = ctx.r[10].s64 + -2844;
	// 82644EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644EE4: 4BE21F3D  bl 0x82466e20
	ctx.lr = 0x82644EE8;
	sub_82466E20(ctx, base);
	// 82644EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644EF8 size=108
    let mut pc: u32 = 0x82644EF8;
    'dispatch: loop {
        match pc {
            0x82644EF8 => {
    //   block [0x82644EF8..0x82644F64)
	// 82644EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644F04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644F0C: 38EBDD28  addi r7, r11, -0x22d8
	ctx.r[7].s64 = ctx.r[11].s64 + -8920;
	// 82644F10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82644F14: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82644F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644F1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82644F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644F28: 386AF514  addi r3, r10, -0xaec
	ctx.r[3].s64 = ctx.r[10].s64 + -2796;
	// 82644F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82644F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82644F50: 4BE21ED1  bl 0x82466e20
	ctx.lr = 0x82644F54;
	sub_82466E20(ctx, base);
	// 82644F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644F68 size=112
    let mut pc: u32 = 0x82644F68;
    'dispatch: loop {
        match pc {
            0x82644F68 => {
    //   block [0x82644F68..0x82644FD8)
	// 82644F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644F74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644F78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644F7C: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82644F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82644F84: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 82644F88: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82644F8C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82644F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644F94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82644F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82644FA0: 386AF544  addi r3, r10, -0xabc
	ctx.r[3].s64 = ctx.r[10].s64 + -2748;
	// 82644FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82644FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82644FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82644FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82644FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82644FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82644FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82644FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82644FC4: 4BE21E5D  bl 0x82466e20
	ctx.lr = 0x82644FC8;
	sub_82466E20(ctx, base);
	// 82644FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82644FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82644FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82644FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82644FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82644FD8 size=116
    let mut pc: u32 = 0x82644FD8;
    'dispatch: loop {
        match pc {
            0x82644FD8 => {
    //   block [0x82644FD8..0x8264504C)
	// 82644FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82644FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82644FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82644FE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82644FE8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82644FEC: 390BDDE8  addi r8, r11, -0x2218
	ctx.r[8].s64 = ctx.r[11].s64 + -8728;
	// 82644FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82644FF4: 392A86E4  addi r9, r10, -0x791c
	ctx.r[9].s64 = ctx.r[10].s64 + -31004;
	// 82644FF8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82644FFC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82645000: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82645004: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264500C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264501C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82645020: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82645024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82645028: 386BF574  addi r3, r11, -0xa8c
	ctx.r[3].s64 = ctx.r[11].s64 + -2700;
	// 8264502C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82645030: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645038: 4BE21DE9  bl 0x82466e20
	ctx.lr = 0x8264503C;
	sub_82466E20(ctx, base);
	// 8264503C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645050 size=100
    let mut pc: u32 = 0x82645050;
    'dispatch: loop {
        match pc {
            0x82645050 => {
    //   block [0x82645050..0x826450B4)
	// 82645050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264505C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645064: 38AAF6C4  addi r5, r10, -0x93c
	ctx.r[5].s64 = ctx.r[10].s64 + -2364;
	// 82645068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645070: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82645074: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264507C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645084: 386AF5A4  addi r3, r10, -0xa5c
	ctx.r[3].s64 = ctx.r[10].s64 + -2652;
	// 82645088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264508C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645090: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82645094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645098: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264509C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826450A0: 4BE21D81  bl 0x82466e20
	ctx.lr = 0x826450A4;
	sub_82466E20(ctx, base);
	// 826450A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826450A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826450AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826450B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826450B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826450B8 size=100
    let mut pc: u32 = 0x826450B8;
    'dispatch: loop {
        match pc {
            0x826450B8 => {
    //   block [0x826450B8..0x8264511C)
	// 826450B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826450BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826450C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826450C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826450C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826450CC: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 826450D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826450D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826450D8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826450DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826450E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826450E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826450E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826450EC: 386AF5D4  addi r3, r10, -0xa2c
	ctx.r[3].s64 = ctx.r[10].s64 + -2604;
	// 826450F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826450F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826450F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826450FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645100: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82645104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645108: 4BE21D19  bl 0x82466e20
	ctx.lr = 0x8264510C;
	sub_82466E20(ctx, base);
	// 8264510C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645120 size=108
    let mut pc: u32 = 0x82645120;
    'dispatch: loop {
        match pc {
            0x82645120 => {
    //   block [0x82645120..0x8264518C)
	// 82645120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264512C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645134: 38EBDE60  addi r7, r11, -0x21a0
	ctx.r[7].s64 = ctx.r[11].s64 + -8608;
	// 82645138: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264513C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82645140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645150: 386AF604  addi r3, r10, -0x9fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2556;
	// 82645154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645178: 4BE21CA9  bl 0x82466e20
	ctx.lr = 0x8264517C;
	sub_82466E20(ctx, base);
	// 8264517C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645190 size=112
    let mut pc: u32 = 0x82645190;
    'dispatch: loop {
        match pc {
            0x82645190 => {
    //   block [0x82645190..0x82645200)
	// 82645190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264519C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826451A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826451A4: 38AAF3F4  addi r5, r10, -0xc0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3084;
	// 826451A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826451AC: 390BDE90  addi r8, r11, -0x2170
	ctx.r[8].s64 = ctx.r[11].s64 + -8560;
	// 826451B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826451B4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826451B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826451BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826451C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826451C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826451C8: 386AF634  addi r3, r10, -0x9cc
	ctx.r[3].s64 = ctx.r[10].s64 + -2508;
	// 826451CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826451D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826451D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826451D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826451DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826451E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826451E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826451E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826451EC: 4BE21C35  bl 0x82466e20
	ctx.lr = 0x826451F0;
	sub_82466E20(ctx, base);
	// 826451F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826451F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826451F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826451FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645200 size=108
    let mut pc: u32 = 0x82645200;
    'dispatch: loop {
        match pc {
            0x82645200 => {
    //   block [0x82645200..0x8264526C)
	// 82645200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264520C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645214: 38EBDEA8  addi r7, r11, -0x2158
	ctx.r[7].s64 = ctx.r[11].s64 + -8536;
	// 82645218: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264521C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82645220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264522C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645230: 386AF664  addi r3, r10, -0x99c
	ctx.r[3].s64 = ctx.r[10].s64 + -2460;
	// 82645234: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264523C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264524C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645258: 4BE21BC9  bl 0x82466e20
	ctx.lr = 0x8264525C;
	sub_82466E20(ctx, base);
	// 8264525C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82645270 size=28
    let mut pc: u32 = 0x82645270;
    'dispatch: loop {
        match pc {
            0x82645270 => {
    //   block [0x82645270..0x8264528C)
	// 82645270: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645274: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82645278: 394A0B50  addi r10, r10, 0xb50
	ctx.r[10].s64 = ctx.r[10].s64 + 2896;
	// 8264527C: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 82645280: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82645284: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82645288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645290 size=108
    let mut pc: u32 = 0x82645290;
    'dispatch: loop {
        match pc {
            0x82645290 => {
    //   block [0x82645290..0x826452FC)
	// 82645290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264529C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826452A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826452A4: 38EB0B50  addi r7, r11, 0xb50
	ctx.r[7].s64 = ctx.r[11].s64 + 2896;
	// 826452A8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826452AC: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826452B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826452B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826452B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826452BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826452C0: 386AF694  addi r3, r10, -0x96c
	ctx.r[3].s64 = ctx.r[10].s64 + -2412;
	// 826452C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826452C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826452CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826452D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826452D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826452D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826452DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826452E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826452E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826452E8: 4BE21B39  bl 0x82466e20
	ctx.lr = 0x826452EC;
	sub_82466E20(ctx, base);
	// 826452EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826452F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826452F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826452F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645300 size=116
    let mut pc: u32 = 0x82645300;
    'dispatch: loop {
        match pc {
            0x82645300 => {
    //   block [0x82645300..0x82645374)
	// 82645300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264530C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645310: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82645314: 390BDEC8  addi r8, r11, -0x2138
	ctx.r[8].s64 = ctx.r[11].s64 + -8504;
	// 82645318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264531C: 392A8738  addi r9, r10, -0x78c8
	ctx.r[9].s64 = ctx.r[10].s64 + -30920;
	// 82645320: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645324: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82645328: 38AAF3F4  addi r5, r10, -0xc0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3084;
	// 8264532C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645334: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264533C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645344: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82645348: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8264534C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82645350: 386BF6C4  addi r3, r11, -0x93c
	ctx.r[3].s64 = ctx.r[11].s64 + -2364;
	// 82645354: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82645358: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264535C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645360: 4BE21AC1  bl 0x82466e20
	ctx.lr = 0x82645364;
	sub_82466E20(ctx, base);
	// 82645364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645378 size=112
    let mut pc: u32 = 0x82645378;
    'dispatch: loop {
        match pc {
            0x82645378 => {
    //   block [0x82645378..0x826453E8)
	// 82645378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645388: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264538C: 38AAF394  addi r5, r10, -0xc6c
	ctx.r[5].s64 = ctx.r[10].s64 + -3180;
	// 82645390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645394: 390BDF28  addi r8, r11, -0x20d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8408;
	// 82645398: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264539C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826453A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826453A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826453A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826453AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826453B0: 386AF6F4  addi r3, r10, -0x90c
	ctx.r[3].s64 = ctx.r[10].s64 + -2316;
	// 826453B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826453B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826453BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826453C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826453C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826453C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826453CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826453D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826453D4: 4BE21A4D  bl 0x82466e20
	ctx.lr = 0x826453D8;
	sub_82466E20(ctx, base);
	// 826453D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826453DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826453E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826453E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826453E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826453E8 size=108
    let mut pc: u32 = 0x826453E8;
    'dispatch: loop {
        match pc {
            0x826453E8 => {
    //   block [0x826453E8..0x82645454)
	// 826453E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826453EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826453F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826453F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826453F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826453FC: 38EBDF40  addi r7, r11, -0x20c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8384;
	// 82645400: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82645404: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82645408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264540C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645418: 386AF724  addi r3, r10, -0x8dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2268;
	// 8264541C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264542C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264543C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645440: 4BE219E1  bl 0x82466e20
	ctx.lr = 0x82645444;
	sub_82466E20(ctx, base);
	// 82645444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264544C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645458 size=112
    let mut pc: u32 = 0x82645458;
    'dispatch: loop {
        match pc {
            0x82645458 => {
    //   block [0x82645458..0x826454C8)
	// 82645458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264545C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645468: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264546C: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 82645470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645474: 390BDF70  addi r8, r11, -0x2090
	ctx.r[8].s64 = ctx.r[11].s64 + -8336;
	// 82645478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264547C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82645480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645490: 386AF754  addi r3, r10, -0x8ac
	ctx.r[3].s64 = ctx.r[10].s64 + -2220;
	// 82645494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264549C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826454A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826454A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826454A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826454AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826454B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826454B4: 4BE2196D  bl 0x82466e20
	ctx.lr = 0x826454B8;
	sub_82466E20(ctx, base);
	// 826454B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826454BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826454C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826454C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826454C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826454C8 size=116
    let mut pc: u32 = 0x826454C8;
    'dispatch: loop {
        match pc {
            0x826454C8 => {
    //   block [0x826454C8..0x8264553C)
	// 826454C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826454CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826454D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826454D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826454D8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826454DC: 390BDFA4  addi r8, r11, -0x205c
	ctx.r[8].s64 = ctx.r[11].s64 + -8284;
	// 826454E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826454E4: 392A8768  addi r9, r10, -0x7898
	ctx.r[9].s64 = ctx.r[10].s64 + -30872;
	// 826454E8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826454EC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826454F0: 38AAF8D4  addi r5, r10, -0x72c
	ctx.r[5].s64 = ctx.r[10].s64 + -1836;
	// 826454F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826454F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826454FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264550C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82645510: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82645514: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82645518: 386BF784  addi r3, r11, -0x87c
	ctx.r[3].s64 = ctx.r[11].s64 + -2172;
	// 8264551C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82645520: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645528: 4BE218F9  bl 0x82466e20
	ctx.lr = 0x8264552C;
	sub_82466E20(ctx, base);
	// 8264552C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645540 size=100
    let mut pc: u32 = 0x82645540;
    'dispatch: loop {
        match pc {
            0x82645540 => {
    //   block [0x82645540..0x826455A4)
	// 82645540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264554C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645554: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 82645558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264555C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645560: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82645564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264556C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645574: 386AF7B4  addi r3, r10, -0x84c
	ctx.r[3].s64 = ctx.r[10].s64 + -2124;
	// 82645578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264557C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645580: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82645584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645588: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264558C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645590: 4BE21891  bl 0x82466e20
	ctx.lr = 0x82645594;
	sub_82466E20(ctx, base);
	// 82645594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264559C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826455A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826455A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826455A8 size=112
    let mut pc: u32 = 0x826455A8;
    'dispatch: loop {
        match pc {
            0x826455A8 => {
    //   block [0x826455A8..0x82645618)
	// 826455A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826455AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826455B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826455B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826455B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826455BC: 38AAF5D4  addi r5, r10, -0xa2c
	ctx.r[5].s64 = ctx.r[10].s64 + -2604;
	// 826455C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826455C4: 390BDFD8  addi r8, r11, -0x2028
	ctx.r[8].s64 = ctx.r[11].s64 + -8232;
	// 826455C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826455CC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826455D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826455D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826455D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826455DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826455E0: 386AF7E4  addi r3, r10, -0x81c
	ctx.r[3].s64 = ctx.r[10].s64 + -2076;
	// 826455E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826455E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826455EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826455F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826455F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826455F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826455FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645604: 4BE2181D  bl 0x82466e20
	ctx.lr = 0x82645608;
	sub_82466E20(ctx, base);
	// 82645608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264560C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645618 size=112
    let mut pc: u32 = 0x82645618;
    'dispatch: loop {
        match pc {
            0x82645618 => {
    //   block [0x82645618..0x82645688)
	// 82645618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264561C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645624: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645628: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264562C: 38AAF5D4  addi r5, r10, -0xa2c
	ctx.r[5].s64 = ctx.r[10].s64 + -2604;
	// 82645630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645634: 390BE020  addi r8, r11, -0x1fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -8160;
	// 82645638: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264563C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82645640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645644: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264564C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645650: 386AF814  addi r3, r10, -0x7ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2028;
	// 82645654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264565C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264566C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645674: 4BE217AD  bl 0x82466e20
	ctx.lr = 0x82645678;
	sub_82466E20(ctx, base);
	// 82645678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264567C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645688 size=108
    let mut pc: u32 = 0x82645688;
    'dispatch: loop {
        match pc {
            0x82645688 => {
    //   block [0x82645688..0x826456F4)
	// 82645688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264568C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645694: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264569C: 38EBE0C8  addi r7, r11, -0x1f38
	ctx.r[7].s64 = ctx.r[11].s64 + -7992;
	// 826456A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826456A4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826456A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826456AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826456B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826456B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826456B8: 386AF844  addi r3, r10, -0x7bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1980;
	// 826456BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826456C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826456C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826456C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826456CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826456D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826456D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826456D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826456DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826456E0: 4BE21741  bl 0x82466e20
	ctx.lr = 0x826456E4;
	sub_82466E20(ctx, base);
	// 826456E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826456E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826456EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826456F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826456F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826456F8 size=112
    let mut pc: u32 = 0x826456F8;
    'dispatch: loop {
        match pc {
            0x826456F8 => {
    //   block [0x826456F8..0x82645768)
	// 826456F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826456FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645708: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264570C: 38AAF3F4  addi r5, r10, -0xc0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3084;
	// 82645710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645714: 390BE110  addi r8, r11, -0x1ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -7920;
	// 82645718: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264571C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82645720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264572C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645730: 386AF874  addi r3, r10, -0x78c
	ctx.r[3].s64 = ctx.r[10].s64 + -1932;
	// 82645734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264573C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264574C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645754: 4BE216CD  bl 0x82466e20
	ctx.lr = 0x82645758;
	sub_82466E20(ctx, base);
	// 82645758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264575C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645768 size=100
    let mut pc: u32 = 0x82645768;
    'dispatch: loop {
        match pc {
            0x82645768 => {
    //   block [0x82645768..0x826457CC)
	// 82645768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264576C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645774: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264577C: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82645780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645788: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8264578C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264579C: 386AF8A4  addi r3, r10, -0x75c
	ctx.r[3].s64 = ctx.r[10].s64 + -1884;
	// 826457A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826457A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826457A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826457AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826457B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826457B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826457B8: 4BE21669  bl 0x82466e20
	ctx.lr = 0x826457BC;
	sub_82466E20(ctx, base);
	// 826457BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826457C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826457C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826457C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826457D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826457D0 size=100
    let mut pc: u32 = 0x826457D0;
    'dispatch: loop {
        match pc {
            0x826457D0 => {
    //   block [0x826457D0..0x82645834)
	// 826457D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826457D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826457D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826457DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826457E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826457E4: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 826457E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826457EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826457F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826457F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826457F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826457FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645804: 386AF8D4  addi r3, r10, -0x72c
	ctx.r[3].s64 = ctx.r[10].s64 + -1836;
	// 82645808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264580C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82645814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264581C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645820: 4BE21601  bl 0x82466e20
	ctx.lr = 0x82645824;
	sub_82466E20(ctx, base);
	// 82645824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264582C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645838 size=108
    let mut pc: u32 = 0x82645838;
    'dispatch: loop {
        match pc {
            0x82645838 => {
    //   block [0x82645838..0x826458A4)
	// 82645838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645844: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264584C: 38EBE170  addi r7, r11, -0x1e90
	ctx.r[7].s64 = ctx.r[11].s64 + -7824;
	// 82645850: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82645854: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82645858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264585C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645868: 386AF904  addi r3, r10, -0x6fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1788;
	// 8264586C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264587C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264588C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645890: 4BE21591  bl 0x82466e20
	ctx.lr = 0x82645894;
	sub_82466E20(ctx, base);
	// 82645894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264589C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826458A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826458A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826458A8 size=112
    let mut pc: u32 = 0x826458A8;
    'dispatch: loop {
        match pc {
            0x826458A8 => {
    //   block [0x826458A8..0x82645918)
	// 826458A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826458AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826458B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826458B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826458B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826458BC: 38AAF6C4  addi r5, r10, -0x93c
	ctx.r[5].s64 = ctx.r[10].s64 + -2364;
	// 826458C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826458C4: 390BE200  addi r8, r11, -0x1e00
	ctx.r[8].s64 = ctx.r[11].s64 + -7680;
	// 826458C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826458CC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826458D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826458D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826458D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826458DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826458E0: 386AF934  addi r3, r10, -0x6cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1740;
	// 826458E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826458E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826458EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826458F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826458F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826458F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826458FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645904: 4BE2151D  bl 0x82466e20
	ctx.lr = 0x82645908;
	sub_82466E20(ctx, base);
	// 82645908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264590C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645918 size=112
    let mut pc: u32 = 0x82645918;
    'dispatch: loop {
        match pc {
            0x82645918 => {
    //   block [0x82645918..0x82645988)
	// 82645918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264591C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645928: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264592C: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82645930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645934: 390BE218  addi r8, r11, -0x1de8
	ctx.r[8].s64 = ctx.r[11].s64 + -7656;
	// 82645938: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264593C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82645940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264594C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645950: 386AF964  addi r3, r10, -0x69c
	ctx.r[3].s64 = ctx.r[10].s64 + -1692;
	// 82645954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264596C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645974: 4BE214AD  bl 0x82466e20
	ctx.lr = 0x82645978;
	sub_82466E20(ctx, base);
	// 82645978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264597C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645988 size=112
    let mut pc: u32 = 0x82645988;
    'dispatch: loop {
        match pc {
            0x82645988 => {
    //   block [0x82645988..0x826459F8)
	// 82645988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645994: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645998: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264599C: 38AAF2D4  addi r5, r10, -0xd2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3372;
	// 826459A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826459A4: 390BE248  addi r8, r11, -0x1db8
	ctx.r[8].s64 = ctx.r[11].s64 + -7608;
	// 826459A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826459AC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826459B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826459B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826459B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826459BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826459C0: 386AF994  addi r3, r10, -0x66c
	ctx.r[3].s64 = ctx.r[10].s64 + -1644;
	// 826459C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826459C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826459CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826459D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826459D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826459D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826459DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826459E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826459E4: 4BE2143D  bl 0x82466e20
	ctx.lr = 0x826459E8;
	sub_82466E20(ctx, base);
	// 826459E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826459EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826459F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826459F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826459F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826459F8 size=112
    let mut pc: u32 = 0x826459F8;
    'dispatch: loop {
        match pc {
            0x826459F8 => {
    //   block [0x826459F8..0x82645A68)
	// 826459F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826459FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645A04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645A08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645A0C: 38AAF424  addi r5, r10, -0xbdc
	ctx.r[5].s64 = ctx.r[10].s64 + -3036;
	// 82645A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645A14: 390BE290  addi r8, r11, -0x1d70
	ctx.r[8].s64 = ctx.r[11].s64 + -7536;
	// 82645A18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82645A1C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82645A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645A24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645A30: 386AF9C4  addi r3, r10, -0x63c
	ctx.r[3].s64 = ctx.r[10].s64 + -1596;
	// 82645A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645A54: 4BE213CD  bl 0x82466e20
	ctx.lr = 0x82645A58;
	sub_82466E20(ctx, base);
	// 82645A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645A68 size=108
    let mut pc: u32 = 0x82645A68;
    'dispatch: loop {
        match pc {
            0x82645A68 => {
    //   block [0x82645A68..0x82645AD4)
	// 82645A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645A74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645A78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82645A7C: 38EBE2D8  addi r7, r11, -0x1d28
	ctx.r[7].s64 = ctx.r[11].s64 + -7464;
	// 82645A80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82645A84: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 82645A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645A8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645A98: 386AF9F4  addi r3, r10, -0x60c
	ctx.r[3].s64 = ctx.r[10].s64 + -1548;
	// 82645A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645AC0: 4BE21361  bl 0x82466e20
	ctx.lr = 0x82645AC4;
	sub_82466E20(ctx, base);
	// 82645AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645AD8 size=112
    let mut pc: u32 = 0x82645AD8;
    'dispatch: loop {
        match pc {
            0x82645AD8 => {
    //   block [0x82645AD8..0x82645B48)
	// 82645AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645AE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645AE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645AEC: 38AAF394  addi r5, r10, -0xc6c
	ctx.r[5].s64 = ctx.r[10].s64 + -3180;
	// 82645AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645AF4: 390BE320  addi r8, r11, -0x1ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -7392;
	// 82645AF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82645AFC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82645B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645B10: 386AFA24  addi r3, r10, -0x5dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1500;
	// 82645B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645B34: 4BE212ED  bl 0x82466e20
	ctx.lr = 0x82645B38;
	sub_82466E20(ctx, base);
	// 82645B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645B48 size=112
    let mut pc: u32 = 0x82645B48;
    'dispatch: loop {
        match pc {
            0x82645B48 => {
    //   block [0x82645B48..0x82645BB8)
	// 82645B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645B54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645B58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645B5C: 38AAF3F4  addi r5, r10, -0xc0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3084;
	// 82645B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645B64: 390BE338  addi r8, r11, -0x1cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -7368;
	// 82645B68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82645B6C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82645B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645B74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645B80: 386AFA54  addi r3, r10, -0x5ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1452;
	// 82645B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645BA4: 4BE2127D  bl 0x82466e20
	ctx.lr = 0x82645BA8;
	sub_82466E20(ctx, base);
	// 82645BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645BB8 size=108
    let mut pc: u32 = 0x82645BB8;
    'dispatch: loop {
        match pc {
            0x82645BB8 => {
    //   block [0x82645BB8..0x82645C24)
	// 82645BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645BC4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645BCC: 38EBE368  addi r7, r11, -0x1c98
	ctx.r[7].s64 = ctx.r[11].s64 + -7320;
	// 82645BD0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82645BD4: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82645BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645BDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645BE8: 386AFA84  addi r3, r10, -0x57c
	ctx.r[3].s64 = ctx.r[10].s64 + -1404;
	// 82645BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645C10: 4BE21211  bl 0x82466e20
	ctx.lr = 0x82645C14;
	sub_82466E20(ctx, base);
	// 82645C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645C28 size=108
    let mut pc: u32 = 0x82645C28;
    'dispatch: loop {
        match pc {
            0x82645C28 => {
    //   block [0x82645C28..0x82645C94)
	// 82645C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645C34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645C3C: 38EBE458  addi r7, r11, -0x1ba8
	ctx.r[7].s64 = ctx.r[11].s64 + -7080;
	// 82645C40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82645C44: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82645C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645C4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645C58: 386AFAB4  addi r3, r10, -0x54c
	ctx.r[3].s64 = ctx.r[10].s64 + -1356;
	// 82645C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645C80: 4BE211A1  bl 0x82466e20
	ctx.lr = 0x82645C84;
	sub_82466E20(ctx, base);
	// 82645C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645C98 size=108
    let mut pc: u32 = 0x82645C98;
    'dispatch: loop {
        match pc {
            0x82645C98 => {
    //   block [0x82645C98..0x82645D04)
	// 82645C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645CA4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645CAC: 38EBE4A0  addi r7, r11, -0x1b60
	ctx.r[7].s64 = ctx.r[11].s64 + -7008;
	// 82645CB0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82645CB4: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82645CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645CBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645CC8: 386AFAE4  addi r3, r10, -0x51c
	ctx.r[3].s64 = ctx.r[10].s64 + -1308;
	// 82645CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645CF0: 4BE21131  bl 0x82466e20
	ctx.lr = 0x82645CF4;
	sub_82466E20(ctx, base);
	// 82645CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645D08 size=108
    let mut pc: u32 = 0x82645D08;
    'dispatch: loop {
        match pc {
            0x82645D08 => {
    //   block [0x82645D08..0x82645D74)
	// 82645D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645D14: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645D1C: 38EBE548  addi r7, r11, -0x1ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -6840;
	// 82645D20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82645D24: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82645D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645D2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645D38: 386AFB14  addi r3, r10, -0x4ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1260;
	// 82645D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645D60: 4BE210C1  bl 0x82466e20
	ctx.lr = 0x82645D64;
	sub_82466E20(ctx, base);
	// 82645D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645D78 size=112
    let mut pc: u32 = 0x82645D78;
    'dispatch: loop {
        match pc {
            0x82645D78 => {
    //   block [0x82645D78..0x82645DE8)
	// 82645D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645D88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645D8C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82645D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645D94: 390BE560  addi r8, r11, -0x1aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -6816;
	// 82645D98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82645D9C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82645DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645DA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645DB0: 386AFB44  addi r3, r10, -0x4bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1212;
	// 82645DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645DD4: 4BE2104D  bl 0x82466e20
	ctx.lr = 0x82645DD8;
	sub_82466E20(ctx, base);
	// 82645DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645DE8 size=112
    let mut pc: u32 = 0x82645DE8;
    'dispatch: loop {
        match pc {
            0x82645DE8 => {
    //   block [0x82645DE8..0x82645E58)
	// 82645DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645DF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645DFC: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645E04: 390BE5C0  addi r8, r11, -0x1a40
	ctx.r[8].s64 = ctx.r[11].s64 + -6720;
	// 82645E08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82645E0C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82645E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645E14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645E20: 386AFB74  addi r3, r10, -0x48c
	ctx.r[3].s64 = ctx.r[10].s64 + -1164;
	// 82645E24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645E44: 4BE20FDD  bl 0x82466e20
	ctx.lr = 0x82645E48;
	sub_82466E20(ctx, base);
	// 82645E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645E58 size=112
    let mut pc: u32 = 0x82645E58;
    'dispatch: loop {
        match pc {
            0x82645E58 => {
    //   block [0x82645E58..0x82645EC8)
	// 82645E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645E6C: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645E74: 390BE5D8  addi r8, r11, -0x1a28
	ctx.r[8].s64 = ctx.r[11].s64 + -6696;
	// 82645E78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82645E7C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82645E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645E90: 386AFBA4  addi r3, r10, -0x45c
	ctx.r[3].s64 = ctx.r[10].s64 + -1116;
	// 82645E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645EB4: 4BE20F6D  bl 0x82466e20
	ctx.lr = 0x82645EB8;
	sub_82466E20(ctx, base);
	// 82645EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645EC8 size=112
    let mut pc: u32 = 0x82645EC8;
    'dispatch: loop {
        match pc {
            0x82645EC8 => {
    //   block [0x82645EC8..0x82645F38)
	// 82645EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645ED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645ED8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645EDC: 38AAFB44  addi r5, r10, -0x4bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1212;
	// 82645EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645EE4: 390BE608  addi r8, r11, -0x19f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6648;
	// 82645EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82645EEC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82645EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645EF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645F00: 386AFBD4  addi r3, r10, -0x42c
	ctx.r[3].s64 = ctx.r[10].s64 + -1068;
	// 82645F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82645F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645F24: 4BE20EFD  bl 0x82466e20
	ctx.lr = 0x82645F28;
	sub_82466E20(ctx, base);
	// 82645F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82645F38 size=24
    let mut pc: u32 = 0x82645F38;
    'dispatch: loop {
        match pc {
            0x82645F38 => {
    //   block [0x82645F38..0x82645F50)
	// 82645F38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645F3C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82645F40: 394A0C88  addi r10, r10, 0xc88
	ctx.r[10].s64 = ctx.r[10].s64 + 3208;
	// 82645F44: 816BDFD4  lwz r11, -0x202c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8236 as u32) ) } as u64;
	// 82645F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82645F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645F50 size=112
    let mut pc: u32 = 0x82645F50;
    'dispatch: loop {
        match pc {
            0x82645F50 => {
    //   block [0x82645F50..0x82645FC0)
	// 82645F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645F5C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82645F60: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645F64: 392A87A4  addi r9, r10, -0x785c
	ctx.r[9].s64 = ctx.r[10].s64 + -30812;
	// 82645F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645F6C: 390B0C88  addi r8, r11, 0xc88
	ctx.r[8].s64 = ctx.r[11].s64 + 3208;
	// 82645F70: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82645F74: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82645F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645F7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82645F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82645F88: 386AFC04  addi r3, r10, -0x3fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1020;
	// 82645F8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82645F90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82645F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82645F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82645F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82645FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82645FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82645FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82645FAC: 4BE20E75  bl 0x82466e20
	ctx.lr = 0x82645FB0;
	sub_82466E20(ctx, base);
	// 82645FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82645FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82645FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82645FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82645FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82645FC0 size=108
    let mut pc: u32 = 0x82645FC0;
    'dispatch: loop {
        match pc {
            0x82645FC0 => {
    //   block [0x82645FC0..0x8264602C)
	// 82645FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82645FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82645FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82645FCC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82645FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82645FD4: 38EBE620  addi r7, r11, -0x19e0
	ctx.r[7].s64 = ctx.r[11].s64 + -6624;
	// 82645FD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82645FDC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82645FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82645FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82645FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82645FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82645FF0: 386AFC34  addi r3, r10, -0x3cc
	ctx.r[3].s64 = ctx.r[10].s64 + -972;
	// 82645FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82645FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82645FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264600C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646018: 4BE20E09  bl 0x82466e20
	ctx.lr = 0x8264601C;
	sub_82466E20(ctx, base);
	// 8264601C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646030 size=108
    let mut pc: u32 = 0x82646030;
    'dispatch: loop {
        match pc {
            0x82646030 => {
    //   block [0x82646030..0x8264609C)
	// 82646030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264603C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646044: 38EBE638  addi r7, r11, -0x19c8
	ctx.r[7].s64 = ctx.r[11].s64 + -6600;
	// 82646048: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264604C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82646050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264605C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646060: 386AFC64  addi r3, r10, -0x39c
	ctx.r[3].s64 = ctx.r[10].s64 + -924;
	// 82646064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264606C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264607C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646088: 4BE20D99  bl 0x82466e20
	ctx.lr = 0x8264608C;
	sub_82466E20(ctx, base);
	// 8264608C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826460A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826460A0 size=112
    let mut pc: u32 = 0x826460A0;
    'dispatch: loop {
        match pc {
            0x826460A0 => {
    //   block [0x826460A0..0x82646110)
	// 826460A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826460A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826460A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826460AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826460B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826460B4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826460B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826460BC: 390BE684  addi r8, r11, -0x197c
	ctx.r[8].s64 = ctx.r[11].s64 + -6524;
	// 826460C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826460C4: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826460C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826460CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826460D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826460D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826460D8: 386AFC94  addi r3, r10, -0x36c
	ctx.r[3].s64 = ctx.r[10].s64 + -876;
	// 826460DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826460E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826460E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826460E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826460EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826460F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826460F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826460F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826460FC: 4BE20D25  bl 0x82466e20
	ctx.lr = 0x82646100;
	sub_82466E20(ctx, base);
	// 82646100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264610C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646110 size=108
    let mut pc: u32 = 0x82646110;
    'dispatch: loop {
        match pc {
            0x82646110 => {
    //   block [0x82646110..0x8264617C)
	// 82646110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264611C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646124: 38EBE6A0  addi r7, r11, -0x1960
	ctx.r[7].s64 = ctx.r[11].s64 + -6496;
	// 82646128: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264612C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82646130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264613C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646140: 386AFCC4  addi r3, r10, -0x33c
	ctx.r[3].s64 = ctx.r[10].s64 + -828;
	// 82646144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264614C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264615C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646168: 4BE20CB9  bl 0x82466e20
	ctx.lr = 0x8264616C;
	sub_82466E20(ctx, base);
	// 8264616C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82646180 size=24
    let mut pc: u32 = 0x82646180;
    'dispatch: loop {
        match pc {
            0x82646180 => {
    //   block [0x82646180..0x82646198)
	// 82646180: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646184: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82646188: 394A0CD0  addi r10, r10, 0xcd0
	ctx.r[10].s64 = ctx.r[10].s64 + 3280;
	// 8264618C: 816BE69C  lwz r11, -0x1964(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6500 as u32) ) } as u64;
	// 82646190: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82646194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646198 size=116
    let mut pc: u32 = 0x82646198;
    'dispatch: loop {
        match pc {
            0x82646198 => {
    //   block [0x82646198..0x8264620C)
	// 82646198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264619C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826461A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826461A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826461A8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826461AC: 390B0CD0  addi r8, r11, 0xcd0
	ctx.r[8].s64 = ctx.r[11].s64 + 3280;
	// 826461B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826461B4: 392A886C  addi r9, r10, -0x7794
	ctx.r[9].s64 = ctx.r[10].s64 + -30612;
	// 826461B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826461BC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826461C0: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826461C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826461C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826461CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826461D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826461D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826461D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826461DC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826461E0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826461E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826461E8: 386BFCF4  addi r3, r11, -0x30c
	ctx.r[3].s64 = ctx.r[11].s64 + -780;
	// 826461EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826461F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826461F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826461F8: 4BE20C29  bl 0x82466e20
	ctx.lr = 0x826461FC;
	sub_82466E20(ctx, base);
	// 826461FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646210 size=112
    let mut pc: u32 = 0x82646210;
    'dispatch: loop {
        match pc {
            0x82646210 => {
    //   block [0x82646210..0x82646280)
	// 82646210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264621C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646220: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646224: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264622C: 390BE700  addi r8, r11, -0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + -6400;
	// 82646230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82646234: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82646238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264623C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646248: 386AFD24  addi r3, r10, -0x2dc
	ctx.r[3].s64 = ctx.r[10].s64 + -732;
	// 8264624C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264625C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264626C: 4BE20BB5  bl 0x82466e20
	ctx.lr = 0x82646270;
	sub_82466E20(ctx, base);
	// 82646270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264627C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646280 size=108
    let mut pc: u32 = 0x82646280;
    'dispatch: loop {
        match pc {
            0x82646280 => {
    //   block [0x82646280..0x826462EC)
	// 82646280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264628C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646294: 38EBE730  addi r7, r11, -0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6352;
	// 82646298: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264629C: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826462A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826462A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826462A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826462AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826462B0: 386AFD54  addi r3, r10, -0x2ac
	ctx.r[3].s64 = ctx.r[10].s64 + -684;
	// 826462B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826462B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826462BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826462C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826462C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826462C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826462CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826462D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826462D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826462D8: 4BE20B49  bl 0x82466e20
	ctx.lr = 0x826462DC;
	sub_82466E20(ctx, base);
	// 826462DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826462E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826462E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826462E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826462F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826462F0 size=108
    let mut pc: u32 = 0x826462F0;
    'dispatch: loop {
        match pc {
            0x826462F0 => {
    //   block [0x826462F0..0x8264635C)
	// 826462F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826462F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826462F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826462FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646304: 38EBE778  addi r7, r11, -0x1888
	ctx.r[7].s64 = ctx.r[11].s64 + -6280;
	// 82646308: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264630C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82646310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646314: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264631C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646320: 386AFD84  addi r3, r10, -0x27c
	ctx.r[3].s64 = ctx.r[10].s64 + -636;
	// 82646324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264632C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264633C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646348: 4BE20AD9  bl 0x82466e20
	ctx.lr = 0x8264634C;
	sub_82466E20(ctx, base);
	// 8264634C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646360 size=112
    let mut pc: u32 = 0x82646360;
    'dispatch: loop {
        match pc {
            0x82646360 => {
    //   block [0x82646360..0x826463D0)
	// 82646360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264636C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646370: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646374: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264637C: 390BE7A8  addi r8, r11, -0x1858
	ctx.r[8].s64 = ctx.r[11].s64 + -6232;
	// 82646380: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82646384: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82646388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264638C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646398: 386AFDB4  addi r3, r10, -0x24c
	ctx.r[3].s64 = ctx.r[10].s64 + -588;
	// 8264639C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826463A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826463A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826463A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826463AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826463B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826463B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826463B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826463BC: 4BE20A65  bl 0x82466e20
	ctx.lr = 0x826463C0;
	sub_82466E20(ctx, base);
	// 826463C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826463C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826463C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826463CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826463D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826463D0 size=108
    let mut pc: u32 = 0x826463D0;
    'dispatch: loop {
        match pc {
            0x826463D0 => {
    //   block [0x826463D0..0x8264643C)
	// 826463D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826463D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826463D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826463DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826463E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826463E4: 38EBE7D8  addi r7, r11, -0x1828
	ctx.r[7].s64 = ctx.r[11].s64 + -6184;
	// 826463E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826463EC: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826463F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826463F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826463F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826463FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646400: 386AFDE4  addi r3, r10, -0x21c
	ctx.r[3].s64 = ctx.r[10].s64 + -540;
	// 82646404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264640C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264641C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646428: 4BE209F9  bl 0x82466e20
	ctx.lr = 0x8264642C;
	sub_82466E20(ctx, base);
	// 8264642C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646440 size=108
    let mut pc: u32 = 0x82646440;
    'dispatch: loop {
        match pc {
            0x82646440 => {
    //   block [0x82646440..0x826464AC)
	// 82646440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264644C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646454: 38EBE838  addi r7, r11, -0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + -6088;
	// 82646458: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264645C: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82646460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264646C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646470: 386AFE14  addi r3, r10, -0x1ec
	ctx.r[3].s64 = ctx.r[10].s64 + -492;
	// 82646474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264647C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264648C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646498: 4BE20989  bl 0x82466e20
	ctx.lr = 0x8264649C;
	sub_82466E20(ctx, base);
	// 8264649C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826464A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826464A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826464A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826464B0 size=112
    let mut pc: u32 = 0x826464B0;
    'dispatch: loop {
        match pc {
            0x826464B0 => {
    //   block [0x826464B0..0x82646520)
	// 826464B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826464B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826464B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826464BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826464C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826464C4: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826464C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826464CC: 390BE880  addi r8, r11, -0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + -6016;
	// 826464D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826464D4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826464D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826464DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826464E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826464E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826464E8: 386AFE44  addi r3, r10, -0x1bc
	ctx.r[3].s64 = ctx.r[10].s64 + -444;
	// 826464EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826464F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826464F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826464F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826464FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264650C: 4BE20915  bl 0x82466e20
	ctx.lr = 0x82646510;
	sub_82466E20(ctx, base);
	// 82646510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264651C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646520 size=112
    let mut pc: u32 = 0x82646520;
    'dispatch: loop {
        match pc {
            0x82646520 => {
    //   block [0x82646520..0x82646590)
	// 82646520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264652C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646530: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646534: 38AAFEA4  addi r5, r10, -0x15c
	ctx.r[5].s64 = ctx.r[10].s64 + -348;
	// 82646538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264653C: 390BE8F8  addi r8, r11, -0x1708
	ctx.r[8].s64 = ctx.r[11].s64 + -5896;
	// 82646540: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82646544: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82646548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264654C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646558: 386AFE74  addi r3, r10, -0x18c
	ctx.r[3].s64 = ctx.r[10].s64 + -396;
	// 8264655C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264656C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264657C: 4BE208A5  bl 0x82466e20
	ctx.lr = 0x82646580;
	sub_82466E20(ctx, base);
	// 82646580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264658C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646590 size=100
    let mut pc: u32 = 0x82646590;
    'dispatch: loop {
        match pc {
            0x82646590 => {
    //   block [0x82646590..0x826465F4)
	// 82646590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264659C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826465A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826465A4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826465A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826465AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826465B0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826465B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826465B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826465BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826465C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826465C4: 386AFEA4  addi r3, r10, -0x15c
	ctx.r[3].s64 = ctx.r[10].s64 + -348;
	// 826465C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826465CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826465D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826465D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826465D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826465DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826465E0: 4BE20841  bl 0x82466e20
	ctx.lr = 0x826465E4;
	sub_82466E20(ctx, base);
	// 826465E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826465E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826465EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826465F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826465F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826465F8 size=24
    let mut pc: u32 = 0x826465F8;
    'dispatch: loop {
        match pc {
            0x826465F8 => {
    //   block [0x826465F8..0x82646610)
	// 826465F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826465FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82646600: 394A0D90  addi r10, r10, 0xd90
	ctx.r[10].s64 = ctx.r[10].s64 + 3472;
	// 82646604: 816BE970  lwz r11, -0x1690(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5776 as u32) ) } as u64;
	// 82646608: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8264660C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646610 size=116
    let mut pc: u32 = 0x82646610;
    'dispatch: loop {
        match pc {
            0x82646610 => {
    //   block [0x82646610..0x82646684)
	// 82646610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264661C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646620: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82646624: 390B0D90  addi r8, r11, 0xd90
	ctx.r[8].s64 = ctx.r[11].s64 + 3472;
	// 82646628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264662C: 392A88A8  addi r9, r10, -0x7758
	ctx.r[9].s64 = ctx.r[10].s64 + -30552;
	// 82646630: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646634: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82646638: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 8264663C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646644: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264664C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646654: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82646658: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8264665C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82646660: 386BFED4  addi r3, r11, -0x12c
	ctx.r[3].s64 = ctx.r[11].s64 + -300;
	// 82646664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82646668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264666C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646670: 4BE207B1  bl 0x82466e20
	ctx.lr = 0x82646674;
	sub_82466E20(ctx, base);
	// 82646674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264667C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646688 size=112
    let mut pc: u32 = 0x82646688;
    'dispatch: loop {
        match pc {
            0x82646688 => {
    //   block [0x82646688..0x826466F8)
	// 82646688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646698: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264669C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826466A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826466A4: 390BE978  addi r8, r11, -0x1688
	ctx.r[8].s64 = ctx.r[11].s64 + -5768;
	// 826466A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826466AC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826466B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826466B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826466B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826466BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826466C0: 386AFF04  addi r3, r10, -0xfc
	ctx.r[3].s64 = ctx.r[10].s64 + -252;
	// 826466C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826466C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826466CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826466D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826466D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826466D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826466DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826466E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826466E4: 4BE2073D  bl 0x82466e20
	ctx.lr = 0x826466E8;
	sub_82466E20(ctx, base);
	// 826466E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826466EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826466F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826466F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826466F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826466F8 size=112
    let mut pc: u32 = 0x826466F8;
    'dispatch: loop {
        match pc {
            0x826466F8 => {
    //   block [0x826466F8..0x82646768)
	// 826466F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826466FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646708: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264670C: 38AAFE44  addi r5, r10, -0x1bc
	ctx.r[5].s64 = ctx.r[10].s64 + -444;
	// 82646710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82646714: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 82646718: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264671C: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82646720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264672C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646730: 386AFF34  addi r3, r10, -0xcc
	ctx.r[3].s64 = ctx.r[10].s64 + -204;
	// 82646734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264673C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264674C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646754: 4BE206CD  bl 0x82466e20
	ctx.lr = 0x82646758;
	sub_82466E20(ctx, base);
	// 82646758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646768 size=108
    let mut pc: u32 = 0x82646768;
    'dispatch: loop {
        match pc {
            0x82646768 => {
    //   block [0x82646768..0x826467D4)
	// 82646768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264676C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646774: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264677C: 38EBEA20  addi r7, r11, -0x15e0
	ctx.r[7].s64 = ctx.r[11].s64 + -5600;
	// 82646780: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82646784: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82646788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264678C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646798: 386AFF64  addi r3, r10, -0x9c
	ctx.r[3].s64 = ctx.r[10].s64 + -156;
	// 8264679C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826467A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826467A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826467A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826467AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826467B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826467B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826467B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826467BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826467C0: 4BE20661  bl 0x82466e20
	ctx.lr = 0x826467C4;
	sub_82466E20(ctx, base);
	// 826467C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826467C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826467CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826467D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826467D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826467D8 size=108
    let mut pc: u32 = 0x826467D8;
    'dispatch: loop {
        match pc {
            0x826467D8 => {
    //   block [0x826467D8..0x82646844)
	// 826467D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826467DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826467E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826467E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826467E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826467EC: 38EBEA68  addi r7, r11, -0x1598
	ctx.r[7].s64 = ctx.r[11].s64 + -5528;
	// 826467F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826467F4: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 826467F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826467FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646808: 386AFF94  addi r3, r10, -0x6c
	ctx.r[3].s64 = ctx.r[10].s64 + -108;
	// 8264680C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264681C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264682C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646830: 4BE205F1  bl 0x82466e20
	ctx.lr = 0x82646834;
	sub_82466E20(ctx, base);
	// 82646834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646848 size=112
    let mut pc: u32 = 0x82646848;
    'dispatch: loop {
        match pc {
            0x82646848 => {
    //   block [0x82646848..0x826468B8)
	// 82646848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646854: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646858: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264685C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646864: 390BEAB0  addi r8, r11, -0x1550
	ctx.r[8].s64 = ctx.r[11].s64 + -5456;
	// 82646868: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264686C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82646870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646874: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264687C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646880: 386AFFC4  addi r3, r10, -0x3c
	ctx.r[3].s64 = ctx.r[10].s64 + -60;
	// 82646884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264688C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264689C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826468A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826468A4: 4BE2057D  bl 0x82466e20
	ctx.lr = 0x826468A8;
	sub_82466E20(ctx, base);
	// 826468A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826468AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826468B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826468B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826468B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826468B8 size=112
    let mut pc: u32 = 0x826468B8;
    'dispatch: loop {
        match pc {
            0x826468B8 => {
    //   block [0x826468B8..0x82646928)
	// 826468B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826468BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826468C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826468C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826468C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826468CC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 826468D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826468D4: 390BEB58  addi r8, r11, -0x14a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5288;
	// 826468D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826468DC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826468E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826468E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826468E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826468EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826468F0: 386AFFF4  addi r3, r10, -0xc
	ctx.r[3].s64 = ctx.r[10].s64 + -12;
	// 826468F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826468F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826468FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264690C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646914: 4BE2050D  bl 0x82466e20
	ctx.lr = 0x82646918;
	sub_82466E20(ctx, base);
	// 82646918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646928 size=108
    let mut pc: u32 = 0x82646928;
    'dispatch: loop {
        match pc {
            0x82646928 => {
    //   block [0x82646928..0x82646994)
	// 82646928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646934: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264693C: 38EBEBA0  addi r7, r11, -0x1460
	ctx.r[7].s64 = ctx.r[11].s64 + -5216;
	// 82646940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82646944: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82646948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264694C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646958: 386A0024  addi r3, r10, 0x24
	ctx.r[3].s64 = ctx.r[10].s64 + 36;
	// 8264695C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646980: 4BE204A1  bl 0x82466e20
	ctx.lr = 0x82646984;
	sub_82466E20(ctx, base);
	// 82646984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264698C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646998 size=108
    let mut pc: u32 = 0x82646998;
    'dispatch: loop {
        match pc {
            0x82646998 => {
    //   block [0x82646998..0x82646A04)
	// 82646998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826469A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826469A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826469A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826469AC: 38EBEBD0  addi r7, r11, -0x1430
	ctx.r[7].s64 = ctx.r[11].s64 + -5168;
	// 826469B0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826469B4: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826469B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826469BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826469C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826469C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826469C8: 386A0054  addi r3, r10, 0x54
	ctx.r[3].s64 = ctx.r[10].s64 + 84;
	// 826469CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826469D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826469D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826469D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826469DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826469E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826469E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826469E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826469EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826469F0: 4BE20431  bl 0x82466e20
	ctx.lr = 0x826469F4;
	sub_82466E20(ctx, base);
	// 826469F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826469F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826469FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646A08 size=112
    let mut pc: u32 = 0x82646A08;
    'dispatch: loop {
        match pc {
            0x82646A08 => {
    //   block [0x82646A08..0x82646A78)
	// 82646A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646A1C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646A24: 390BEC60  addi r8, r11, -0x13a0
	ctx.r[8].s64 = ctx.r[11].s64 + -5024;
	// 82646A28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82646A2C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82646A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646A40: 386A0084  addi r3, r10, 0x84
	ctx.r[3].s64 = ctx.r[10].s64 + 132;
	// 82646A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646A64: 4BE203BD  bl 0x82466e20
	ctx.lr = 0x82646A68;
	sub_82466E20(ctx, base);
	// 82646A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646A78 size=112
    let mut pc: u32 = 0x82646A78;
    'dispatch: loop {
        match pc {
            0x82646A78 => {
    //   block [0x82646A78..0x82646AE8)
	// 82646A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646A8C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646A94: 390BECF0  addi r8, r11, -0x1310
	ctx.r[8].s64 = ctx.r[11].s64 + -4880;
	// 82646A98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82646A9C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82646AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646AB0: 386A00B4  addi r3, r10, 0xb4
	ctx.r[3].s64 = ctx.r[10].s64 + 180;
	// 82646AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646AD4: 4BE2034D  bl 0x82466e20
	ctx.lr = 0x82646AD8;
	sub_82466E20(ctx, base);
	// 82646AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646AE8 size=112
    let mut pc: u32 = 0x82646AE8;
    'dispatch: loop {
        match pc {
            0x82646AE8 => {
    //   block [0x82646AE8..0x82646B58)
	// 82646AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646AFC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646B04: 390BED98  addi r8, r11, -0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + -4712;
	// 82646B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82646B0C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82646B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646B20: 386A00E4  addi r3, r10, 0xe4
	ctx.r[3].s64 = ctx.r[10].s64 + 228;
	// 82646B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646B44: 4BE202DD  bl 0x82466e20
	ctx.lr = 0x82646B48;
	sub_82466E20(ctx, base);
	// 82646B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646B58 size=108
    let mut pc: u32 = 0x82646B58;
    'dispatch: loop {
        match pc {
            0x82646B58 => {
    //   block [0x82646B58..0x82646BC4)
	// 82646B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646B64: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646B6C: 38EBEDB0  addi r7, r11, -0x1250
	ctx.r[7].s64 = ctx.r[11].s64 + -4688;
	// 82646B70: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82646B74: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82646B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82646B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646B88: 386A0114  addi r3, r10, 0x114
	ctx.r[3].s64 = ctx.r[10].s64 + 276;
	// 82646B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82646B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82646BB0: 4BE20271  bl 0x82466e20
	ctx.lr = 0x82646BB4;
	sub_82466E20(ctx, base);
	// 82646BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646BC8 size=112
    let mut pc: u32 = 0x82646BC8;
    'dispatch: loop {
        match pc {
            0x82646BC8 => {
    //   block [0x82646BC8..0x82646C38)
	// 82646BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646BD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646BD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646BDC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82646BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646BE4: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 82646BE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82646BEC: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82646BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646BF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646C00: 386A0144  addi r3, r10, 0x144
	ctx.r[3].s64 = ctx.r[10].s64 + 324;
	// 82646C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646C24: 4BE201FD  bl 0x82466e20
	ctx.lr = 0x82646C28;
	sub_82466E20(ctx, base);
	// 82646C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646C38 size=100
    let mut pc: u32 = 0x82646C38;
    'dispatch: loop {
        match pc {
            0x82646C38 => {
    //   block [0x82646C38..0x82646C9C)
	// 82646C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646C4C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82646C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646C58: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82646C5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646C6C: 386A0174  addi r3, r10, 0x174
	ctx.r[3].s64 = ctx.r[10].s64 + 372;
	// 82646C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646C74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646C88: 4BE20199  bl 0x82466e20
	ctx.lr = 0x82646C8C;
	sub_82466E20(ctx, base);
	// 82646C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646CA0 size=112
    let mut pc: u32 = 0x82646CA0;
    'dispatch: loop {
        match pc {
            0x82646CA0 => {
    //   block [0x82646CA0..0x82646D10)
	// 82646CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646CAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646CB0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646CB4: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646CBC: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 82646CC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82646CC4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82646CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646CCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646CD8: 386A01A4  addi r3, r10, 0x1a4
	ctx.r[3].s64 = ctx.r[10].s64 + 420;
	// 82646CDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646CFC: 4BE20125  bl 0x82466e20
	ctx.lr = 0x82646D00;
	sub_82466E20(ctx, base);
	// 82646D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646D10 size=112
    let mut pc: u32 = 0x82646D10;
    'dispatch: loop {
        match pc {
            0x82646D10 => {
    //   block [0x82646D10..0x82646D80)
	// 82646D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646D1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646D24: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646D2C: 390BEEE8  addi r8, r11, -0x1118
	ctx.r[8].s64 = ctx.r[11].s64 + -4376;
	// 82646D30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82646D34: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82646D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646D48: 386A01D4  addi r3, r10, 0x1d4
	ctx.r[3].s64 = ctx.r[10].s64 + 468;
	// 82646D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646D6C: 4BE200B5  bl 0x82466e20
	ctx.lr = 0x82646D70;
	sub_82466E20(ctx, base);
	// 82646D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646D80 size=112
    let mut pc: u32 = 0x82646D80;
    'dispatch: loop {
        match pc {
            0x82646D80 => {
    //   block [0x82646D80..0x82646DF0)
	// 82646D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646D90: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646D94: 38AA0174  addi r5, r10, 0x174
	ctx.r[5].s64 = ctx.r[10].s64 + 372;
	// 82646D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646D9C: 390BEF48  addi r8, r11, -0x10b8
	ctx.r[8].s64 = ctx.r[11].s64 + -4280;
	// 82646DA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82646DA4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82646DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646DAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646DB8: 386A0204  addi r3, r10, 0x204
	ctx.r[3].s64 = ctx.r[10].s64 + 516;
	// 82646DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646DDC: 4BE20045  bl 0x82466e20
	ctx.lr = 0x82646DE0;
	sub_82466E20(ctx, base);
	// 82646DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646DF0 size=112
    let mut pc: u32 = 0x82646DF0;
    'dispatch: loop {
        match pc {
            0x82646DF0 => {
    //   block [0x82646DF0..0x82646E60)
	// 82646DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E00: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82646E04: 38AA0654  addi r5, r10, 0x654
	ctx.r[5].s64 = ctx.r[10].s64 + 1620;
	// 82646E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646E0C: 390BEFA8  addi r8, r11, -0x1058
	ctx.r[8].s64 = ctx.r[11].s64 + -4184;
	// 82646E10: 39200011  li r9, 0x11
	ctx.r[9].s64 = 17;
	// 82646E14: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82646E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646E1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82646E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646E28: 386A0234  addi r3, r10, 0x234
	ctx.r[3].s64 = ctx.r[10].s64 + 564;
	// 82646E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82646E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646E4C: 4BE1FFD5  bl 0x82466e20
	ctx.lr = 0x82646E50;
	sub_82466E20(ctx, base);
	// 82646E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646E60 size=100
    let mut pc: u32 = 0x82646E60;
    'dispatch: loop {
        match pc {
            0x82646E60 => {
    //   block [0x82646E60..0x82646EC4)
	// 82646E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646E74: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82646E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646E80: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82646E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646E94: 386A0264  addi r3, r10, 0x264
	ctx.r[3].s64 = ctx.r[10].s64 + 612;
	// 82646E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646EB0: 4BE1FF71  bl 0x82466e20
	ctx.lr = 0x82646EB4;
	sub_82466E20(ctx, base);
	// 82646EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646EC8 size=100
    let mut pc: u32 = 0x82646EC8;
    'dispatch: loop {
        match pc {
            0x82646EC8 => {
    //   block [0x82646EC8..0x82646F2C)
	// 82646EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646ED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646EDC: 38AA02F4  addi r5, r10, 0x2f4
	ctx.r[5].s64 = ctx.r[10].s64 + 756;
	// 82646EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646EE8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82646EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646EFC: 386A0294  addi r3, r10, 0x294
	ctx.r[3].s64 = ctx.r[10].s64 + 660;
	// 82646F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646F04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646F08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646F10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646F18: 4BE1FF09  bl 0x82466e20
	ctx.lr = 0x82646F1C;
	sub_82466E20(ctx, base);
	// 82646F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646F30 size=100
    let mut pc: u32 = 0x82646F30;
    'dispatch: loop {
        match pc {
            0x82646F30 => {
    //   block [0x82646F30..0x82646F94)
	// 82646F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646F3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646F44: 38AA0234  addi r5, r10, 0x234
	ctx.r[5].s64 = ctx.r[10].s64 + 564;
	// 82646F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82646F50: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82646F54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646F64: 386A02C4  addi r3, r10, 0x2c4
	ctx.r[3].s64 = ctx.r[10].s64 + 708;
	// 82646F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82646F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646F80: 4BE1FEA1  bl 0x82466e20
	ctx.lr = 0x82646F84;
	sub_82466E20(ctx, base);
	// 82646F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82646F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82646F98 size=104
    let mut pc: u32 = 0x82646F98;
    'dispatch: loop {
        match pc {
            0x82646F98 => {
    //   block [0x82646F98..0x82647000)
	// 82646F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82646F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82646FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82646FA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82646FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82646FAC: 392A88DC  addi r9, r10, -0x7724
	ctx.r[9].s64 = ctx.r[10].s64 + -30500;
	// 82646FB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82646FB8: 38AA0264  addi r5, r10, 0x264
	ctx.r[5].s64 = ctx.r[10].s64 + 612;
	// 82646FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82646FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82646FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82646FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82646FCC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82646FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82646FD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82646FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82646FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82646FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82646FE4: 386A02F4  addi r3, r10, 0x2f4
	ctx.r[3].s64 = ctx.r[10].s64 + 756;
	// 82646FE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82646FEC: 4BE1FE35  bl 0x82466e20
	ctx.lr = 0x82646FF0;
	sub_82466E20(ctx, base);
	// 82646FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82646FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82646FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82646FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647000 size=108
    let mut pc: u32 = 0x82647000;
    'dispatch: loop {
        match pc {
            0x82647000 => {
    //   block [0x82647000..0x8264706C)
	// 82647000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264700C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647014: 38EBF140  addi r7, r11, -0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + -3776;
	// 82647018: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264701C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82647020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264702C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647030: 386A0324  addi r3, r10, 0x324
	ctx.r[3].s64 = ctx.r[10].s64 + 804;
	// 82647034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264703C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647058: 4BE1FDC9  bl 0x82466e20
	ctx.lr = 0x8264705C;
	sub_82466E20(ctx, base);
	// 8264705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647070 size=112
    let mut pc: u32 = 0x82647070;
    'dispatch: loop {
        match pc {
            0x82647070 => {
    //   block [0x82647070..0x826470E0)
	// 82647070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264707C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647084: 38AA02F4  addi r5, r10, 0x2f4
	ctx.r[5].s64 = ctx.r[10].s64 + 756;
	// 82647088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264708C: 390BF170  addi r8, r11, -0xe90
	ctx.r[8].s64 = ctx.r[11].s64 + -3728;
	// 82647090: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82647094: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82647098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264709C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826470A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826470A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826470A8: 386A0354  addi r3, r10, 0x354
	ctx.r[3].s64 = ctx.r[10].s64 + 852;
	// 826470AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826470B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826470B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826470B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826470BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826470C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826470C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826470C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826470CC: 4BE1FD55  bl 0x82466e20
	ctx.lr = 0x826470D0;
	sub_82466E20(ctx, base);
	// 826470D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826470D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826470D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826470DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826470E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826470E0 size=116
    let mut pc: u32 = 0x826470E0;
    'dispatch: loop {
        match pc {
            0x826470E0 => {
    //   block [0x826470E0..0x82647154)
	// 826470E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826470E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826470E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826470EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826470F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826470F4: 390BF21C  addi r8, r11, -0xde4
	ctx.r[8].s64 = ctx.r[11].s64 + -3556;
	// 826470F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826470FC: 392A8938  addi r9, r10, -0x76c8
	ctx.r[9].s64 = ctx.r[10].s64 + -30408;
	// 82647100: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647104: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82647108: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 8264710C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264711C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647124: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82647128: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8264712C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82647130: 386B0384  addi r3, r11, 0x384
	ctx.r[3].s64 = ctx.r[11].s64 + 900;
	// 82647134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82647138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647140: 4BE1FCE1  bl 0x82466e20
	ctx.lr = 0x82647144;
	sub_82466E20(ctx, base);
	// 82647144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647158 size=112
    let mut pc: u32 = 0x82647158;
    'dispatch: loop {
        match pc {
            0x82647158 => {
    //   block [0x82647158..0x826471C8)
	// 82647158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647168: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264716C: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647174: 390BF234  addi r8, r11, -0xdcc
	ctx.r[8].s64 = ctx.r[11].s64 + -3532;
	// 82647178: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264717C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82647180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647190: 386A03B4  addi r3, r10, 0x3b4
	ctx.r[3].s64 = ctx.r[10].s64 + 948;
	// 82647194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826471A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826471A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826471A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826471AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826471B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826471B4: 4BE1FC6D  bl 0x82466e20
	ctx.lr = 0x826471B8;
	sub_82466E20(ctx, base);
	// 826471B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826471BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826471C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826471C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826471C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826471C8 size=100
    let mut pc: u32 = 0x826471C8;
    'dispatch: loop {
        match pc {
            0x826471C8 => {
    //   block [0x826471C8..0x8264722C)
	// 826471C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826471CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826471D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826471D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826471D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826471DC: 38AA0414  addi r5, r10, 0x414
	ctx.r[5].s64 = ctx.r[10].s64 + 1044;
	// 826471E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826471E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826471E8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826471EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826471F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826471F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826471F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826471FC: 386A03E4  addi r3, r10, 0x3e4
	ctx.r[3].s64 = ctx.r[10].s64 + 996;
	// 82647200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264720C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82647214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647218: 4BE1FC09  bl 0x82466e20
	ctx.lr = 0x8264721C;
	sub_82466E20(ctx, base);
	// 8264721C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647230 size=112
    let mut pc: u32 = 0x82647230;
    'dispatch: loop {
        match pc {
            0x82647230 => {
    //   block [0x82647230..0x826472A0)
	// 82647230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264723C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647240: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647244: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264724C: 390BF24C  addi r8, r11, -0xdb4
	ctx.r[8].s64 = ctx.r[11].s64 + -3508;
	// 82647250: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82647254: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82647258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264725C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647268: 386A0414  addi r3, r10, 0x414
	ctx.r[3].s64 = ctx.r[10].s64 + 1044;
	// 8264726C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264727C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264728C: 4BE1FB95  bl 0x82466e20
	ctx.lr = 0x82647290;
	sub_82466E20(ctx, base);
	// 82647290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264729C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826472A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826472A0 size=112
    let mut pc: u32 = 0x826472A0;
    'dispatch: loop {
        match pc {
            0x826472A0 => {
    //   block [0x826472A0..0x82647310)
	// 826472A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826472A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826472A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826472AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826472B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826472B4: 38AA0384  addi r5, r10, 0x384
	ctx.r[5].s64 = ctx.r[10].s64 + 900;
	// 826472B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826472BC: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 826472C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826472C4: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 826472C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826472CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826472D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826472D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826472D8: 386A0444  addi r3, r10, 0x444
	ctx.r[3].s64 = ctx.r[10].s64 + 1092;
	// 826472DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826472E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826472E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826472E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826472EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826472F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826472F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826472F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826472FC: 4BE1FB25  bl 0x82466e20
	ctx.lr = 0x82647300;
	sub_82466E20(ctx, base);
	// 82647300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647310 size=100
    let mut pc: u32 = 0x82647310;
    'dispatch: loop {
        match pc {
            0x82647310 => {
    //   block [0x82647310..0x82647374)
	// 82647310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264731C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647324: 38AA0444  addi r5, r10, 0x444
	ctx.r[5].s64 = ctx.r[10].s64 + 1092;
	// 82647328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264732C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647330: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82647334: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264733C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647344: 386A0474  addi r3, r10, 0x474
	ctx.r[3].s64 = ctx.r[10].s64 + 1140;
	// 82647348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264734C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82647354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264735C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647360: 4BE1FAC1  bl 0x82466e20
	ctx.lr = 0x82647364;
	sub_82466E20(ctx, base);
	// 82647364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647378 size=100
    let mut pc: u32 = 0x82647378;
    'dispatch: loop {
        match pc {
            0x82647378 => {
    //   block [0x82647378..0x826473DC)
	// 82647378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264738C: 38AA03B4  addi r5, r10, 0x3b4
	ctx.r[5].s64 = ctx.r[10].s64 + 948;
	// 82647390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647398: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8264739C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826473A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826473A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826473A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826473AC: 386A04A4  addi r3, r10, 0x4a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1188;
	// 826473B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826473B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826473B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826473BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826473C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826473C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826473C8: 4BE1FA59  bl 0x82466e20
	ctx.lr = 0x826473CC;
	sub_82466E20(ctx, base);
	// 826473CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826473D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826473D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826473D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826473E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826473E0 size=100
    let mut pc: u32 = 0x826473E0;
    'dispatch: loop {
        match pc {
            0x826473E0 => {
    //   block [0x826473E0..0x82647444)
	// 826473E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826473E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826473E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826473EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826473F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826473F4: 38AA0474  addi r5, r10, 0x474
	ctx.r[5].s64 = ctx.r[10].s64 + 1140;
	// 826473F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826473FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647400: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82647404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264740C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647414: 386A04D4  addi r3, r10, 0x4d4
	ctx.r[3].s64 = ctx.r[10].s64 + 1236;
	// 82647418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264741C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82647424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647430: 4BE1F9F1  bl 0x82466e20
	ctx.lr = 0x82647434;
	sub_82466E20(ctx, base);
	// 82647434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647448 size=112
    let mut pc: u32 = 0x82647448;
    'dispatch: loop {
        match pc {
            0x82647448 => {
    //   block [0x82647448..0x826474B8)
	// 82647448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264745C: 38AA0564  addi r5, r10, 0x564
	ctx.r[5].s64 = ctx.r[10].s64 + 1380;
	// 82647460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647464: 390BF328  addi r8, r11, -0xcd8
	ctx.r[8].s64 = ctx.r[11].s64 + -3288;
	// 82647468: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264746C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82647470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264747C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647480: 386A0504  addi r3, r10, 0x504
	ctx.r[3].s64 = ctx.r[10].s64 + 1284;
	// 82647484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264748C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264749C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826474A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826474A4: 4BE1F97D  bl 0x82466e20
	ctx.lr = 0x826474A8;
	sub_82466E20(ctx, base);
	// 826474A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826474AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826474B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826474B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826474B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826474B8 size=112
    let mut pc: u32 = 0x826474B8;
    'dispatch: loop {
        match pc {
            0x826474B8 => {
    //   block [0x826474B8..0x82647528)
	// 826474B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826474BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826474C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826474C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826474C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826474CC: 38AA0594  addi r5, r10, 0x594
	ctx.r[5].s64 = ctx.r[10].s64 + 1428;
	// 826474D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826474D4: 390BF358  addi r8, r11, -0xca8
	ctx.r[8].s64 = ctx.r[11].s64 + -3240;
	// 826474D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826474DC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826474E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826474E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826474E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826474EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826474F0: 386A0534  addi r3, r10, 0x534
	ctx.r[3].s64 = ctx.r[10].s64 + 1332;
	// 826474F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826474F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826474FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264750C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647514: 4BE1F90D  bl 0x82466e20
	ctx.lr = 0x82647518;
	sub_82466E20(ctx, base);
	// 82647518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264751C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647528 size=112
    let mut pc: u32 = 0x82647528;
    'dispatch: loop {
        match pc {
            0x82647528 => {
    //   block [0x82647528..0x82647598)
	// 82647528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647538: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264753C: 38AA0654  addi r5, r10, 0x654
	ctx.r[5].s64 = ctx.r[10].s64 + 1620;
	// 82647540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647544: 390BF370  addi r8, r11, -0xc90
	ctx.r[8].s64 = ctx.r[11].s64 + -3216;
	// 82647548: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264754C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82647550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264755C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647560: 386A0564  addi r3, r10, 0x564
	ctx.r[3].s64 = ctx.r[10].s64 + 1380;
	// 82647564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264756C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264757C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647584: 4BE1F89D  bl 0x82466e20
	ctx.lr = 0x82647588;
	sub_82466E20(ctx, base);
	// 82647588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264758C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647598 size=112
    let mut pc: u32 = 0x82647598;
    'dispatch: loop {
        match pc {
            0x82647598 => {
    //   block [0x82647598..0x82647608)
	// 82647598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826475A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826475A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826475A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826475AC: 38AA0564  addi r5, r10, 0x564
	ctx.r[5].s64 = ctx.r[10].s64 + 1380;
	// 826475B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826475B4: 390BF3A0  addi r8, r11, -0xc60
	ctx.r[8].s64 = ctx.r[11].s64 + -3168;
	// 826475B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826475BC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826475C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826475C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826475C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826475CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826475D0: 386A0594  addi r3, r10, 0x594
	ctx.r[3].s64 = ctx.r[10].s64 + 1428;
	// 826475D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826475D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826475DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826475E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826475E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826475E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826475EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826475F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826475F4: 4BE1F82D  bl 0x82466e20
	ctx.lr = 0x826475F8;
	sub_82466E20(ctx, base);
	// 826475F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826475FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647608 size=112
    let mut pc: u32 = 0x82647608;
    'dispatch: loop {
        match pc {
            0x82647608 => {
    //   block [0x82647608..0x82647678)
	// 82647608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647618: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264761C: 38AA0594  addi r5, r10, 0x594
	ctx.r[5].s64 = ctx.r[10].s64 + 1428;
	// 82647620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647624: 390BF3B8  addi r8, r11, -0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + -3144;
	// 82647628: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264762C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82647630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647634: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264763C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647640: 386A05C4  addi r3, r10, 0x5c4
	ctx.r[3].s64 = ctx.r[10].s64 + 1476;
	// 82647644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264764C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264765C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647664: 4BE1F7BD  bl 0x82466e20
	ctx.lr = 0x82647668;
	sub_82466E20(ctx, base);
	// 82647668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264766C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647678 size=112
    let mut pc: u32 = 0x82647678;
    'dispatch: loop {
        match pc {
            0x82647678 => {
    //   block [0x82647678..0x826476E8)
	// 82647678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647684: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264768C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647694: 390BF3D0  addi r8, r11, -0xc30
	ctx.r[8].s64 = ctx.r[11].s64 + -3120;
	// 82647698: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264769C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826476A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826476A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826476A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826476AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826476B0: 386A05F4  addi r3, r10, 0x5f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1524;
	// 826476B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826476B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826476BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826476C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826476C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826476C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826476CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826476D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826476D4: 4BE1F74D  bl 0x82466e20
	ctx.lr = 0x826476D8;
	sub_82466E20(ctx, base);
	// 826476D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826476DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826476E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826476E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826476E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826476E8 size=48
    let mut pc: u32 = 0x826476E8;
    'dispatch: loop {
        match pc {
            0x826476E8 => {
    //   block [0x826476E8..0x82647718)
	// 826476E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826476EC: 814BF468  lwz r10, -0xb98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) } as u64;
	// 826476F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826476F4: 396B0DF0  addi r11, r11, 0xdf0
	ctx.r[11].s64 = ctx.r[11].s64 + 3568;
	// 826476F8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826476FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82647700: 814AF464  lwz r10, -0xb9c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2972 as u32) ) } as u64;
	// 82647704: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82647708: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264770C: 814AF460  lwz r10, -0xba0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2976 as u32) ) } as u64;
	// 82647710: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 82647714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647718 size=116
    let mut pc: u32 = 0x82647718;
    'dispatch: loop {
        match pc {
            0x82647718 => {
    //   block [0x82647718..0x8264778C)
	// 82647718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264771C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647724: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82647728: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264772C: 392B8A38  addi r9, r11, -0x75c8
	ctx.r[9].s64 = ctx.r[11].s64 + -30152;
	// 82647730: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647734: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647738: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8264773C: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82647740: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647744: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82647748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264774C: 396B0DF0  addi r11, r11, 0xdf0
	ctx.r[11].s64 = ctx.r[11].s64 + 3568;
	// 82647750: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82647754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647758: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264775C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647760: 386A0624  addi r3, r10, 0x624
	ctx.r[3].s64 = ctx.r[10].s64 + 1572;
	// 82647764: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82647768: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264776C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647770: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82647774: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82647778: 4BE1F6A9  bl 0x82466e20
	ctx.lr = 0x8264777C;
	sub_82466E20(ctx, base);
	// 8264777C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647790 size=116
    let mut pc: u32 = 0x82647790;
    'dispatch: loop {
        match pc {
            0x82647790 => {
    //   block [0x82647790..0x82647804)
	// 82647790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264779C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826477A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826477A4: 390BF470  addi r8, r11, -0xb90
	ctx.r[8].s64 = ctx.r[11].s64 + -2960;
	// 826477A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826477AC: 392A894C  addi r9, r10, -0x76b4
	ctx.r[9].s64 = ctx.r[10].s64 + -30388;
	// 826477B0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826477B4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826477B8: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826477BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826477C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826477C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826477C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826477CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826477D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826477D4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826477D8: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826477DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826477E0: 386B0654  addi r3, r11, 0x654
	ctx.r[3].s64 = ctx.r[11].s64 + 1620;
	// 826477E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826477E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826477EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826477F0: 4BE1F631  bl 0x82466e20
	ctx.lr = 0x826477F4;
	sub_82466E20(ctx, base);
	// 826477F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826477F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826477FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647808 size=112
    let mut pc: u32 = 0x82647808;
    'dispatch: loop {
        match pc {
            0x82647808 => {
    //   block [0x82647808..0x82647878)
	// 82647808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264780C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647818: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264781C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647824: 390BF4E8  addi r8, r11, -0xb18
	ctx.r[8].s64 = ctx.r[11].s64 + -2840;
	// 82647828: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264782C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82647830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264783C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647840: 386A0684  addi r3, r10, 0x684
	ctx.r[3].s64 = ctx.r[10].s64 + 1668;
	// 82647844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264784C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264785C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647864: 4BE1F5BD  bl 0x82466e20
	ctx.lr = 0x82647868;
	sub_82466E20(ctx, base);
	// 82647868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264786C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647878 size=112
    let mut pc: u32 = 0x82647878;
    'dispatch: loop {
        match pc {
            0x82647878 => {
    //   block [0x82647878..0x826478E8)
	// 82647878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264787C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647888: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264788C: 38AAF0F4  addi r5, r10, -0xf0c
	ctx.r[5].s64 = ctx.r[10].s64 + -3852;
	// 82647890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647894: 390BF500  addi r8, r11, -0xb00
	ctx.r[8].s64 = ctx.r[11].s64 + -2816;
	// 82647898: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264789C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826478A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826478A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826478A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826478AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826478B0: 386A06B4  addi r3, r10, 0x6b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1716;
	// 826478B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826478B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826478BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826478C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826478C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826478C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826478CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826478D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826478D4: 4BE1F54D  bl 0x82466e20
	ctx.lr = 0x826478D8;
	sub_82466E20(ctx, base);
	// 826478D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826478DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826478E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826478E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826478E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826478E8 size=108
    let mut pc: u32 = 0x826478E8;
    'dispatch: loop {
        match pc {
            0x826478E8 => {
    //   block [0x826478E8..0x82647954)
	// 826478E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826478EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826478F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826478F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826478F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826478FC: 38EBF518  addi r7, r11, -0xae8
	ctx.r[7].s64 = ctx.r[11].s64 + -2792;
	// 82647900: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82647904: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82647908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264790C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647918: 386A06E4  addi r3, r10, 0x6e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1764;
	// 8264791C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264792C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264793C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647940: 4BE1F4E1  bl 0x82466e20
	ctx.lr = 0x82647944;
	sub_82466E20(ctx, base);
	// 82647944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264794C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647958 size=112
    let mut pc: u32 = 0x82647958;
    'dispatch: loop {
        match pc {
            0x82647958 => {
    //   block [0x82647958..0x826479C8)
	// 82647958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647968: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264796C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647974: 390BF530  addi r8, r11, -0xad0
	ctx.r[8].s64 = ctx.r[11].s64 + -2768;
	// 82647978: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264797C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82647980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264798C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647990: 386A0714  addi r3, r10, 0x714
	ctx.r[3].s64 = ctx.r[10].s64 + 1812;
	// 82647994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264799C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826479A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826479A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826479A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826479AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826479B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826479B4: 4BE1F46D  bl 0x82466e20
	ctx.lr = 0x826479B8;
	sub_82466E20(ctx, base);
	// 826479B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826479BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826479C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826479C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826479C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826479C8 size=108
    let mut pc: u32 = 0x826479C8;
    'dispatch: loop {
        match pc {
            0x826479C8 => {
    //   block [0x826479C8..0x82647A34)
	// 826479C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826479CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826479D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826479D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826479D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826479DC: 38EBF578  addi r7, r11, -0xa88
	ctx.r[7].s64 = ctx.r[11].s64 + -2696;
	// 826479E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826479E4: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826479E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826479EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826479F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826479F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826479F8: 386A0744  addi r3, r10, 0x744
	ctx.r[3].s64 = ctx.r[10].s64 + 1860;
	// 826479FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647A20: 4BE1F401  bl 0x82466e20
	ctx.lr = 0x82647A24;
	sub_82466E20(ctx, base);
	// 82647A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647A38 size=112
    let mut pc: u32 = 0x82647A38;
    'dispatch: loop {
        match pc {
            0x82647A38 => {
    //   block [0x82647A38..0x82647AA8)
	// 82647A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647A48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647A4C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82647A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647A54: 390BF590  addi r8, r11, -0xa70
	ctx.r[8].s64 = ctx.r[11].s64 + -2672;
	// 82647A58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82647A5C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82647A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647A64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647A70: 386A0774  addi r3, r10, 0x774
	ctx.r[3].s64 = ctx.r[10].s64 + 1908;
	// 82647A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647A94: 4BE1F38D  bl 0x82466e20
	ctx.lr = 0x82647A98;
	sub_82466E20(ctx, base);
	// 82647A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647AA8 size=112
    let mut pc: u32 = 0x82647AA8;
    'dispatch: loop {
        match pc {
            0x82647AA8 => {
    //   block [0x82647AA8..0x82647B18)
	// 82647AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647AB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647ABC: 38AA0834  addi r5, r10, 0x834
	ctx.r[5].s64 = ctx.r[10].s64 + 2100;
	// 82647AC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647AC4: 390BF5C0  addi r8, r11, -0xa40
	ctx.r[8].s64 = ctx.r[11].s64 + -2624;
	// 82647AC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82647ACC: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82647AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647AD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647AE0: 386A07A4  addi r3, r10, 0x7a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1956;
	// 82647AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647B04: 4BE1F31D  bl 0x82466e20
	ctx.lr = 0x82647B08;
	sub_82466E20(ctx, base);
	// 82647B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647B18 size=108
    let mut pc: u32 = 0x82647B18;
    'dispatch: loop {
        match pc {
            0x82647B18 => {
    //   block [0x82647B18..0x82647B84)
	// 82647B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647B24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647B2C: 38EBF638  addi r7, r11, -0x9c8
	ctx.r[7].s64 = ctx.r[11].s64 + -2504;
	// 82647B30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82647B34: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82647B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647B40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647B48: 386A07D4  addi r3, r10, 0x7d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2004;
	// 82647B4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647B70: 4BE1F2B1  bl 0x82466e20
	ctx.lr = 0x82647B74;
	sub_82466E20(ctx, base);
	// 82647B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647B88 size=108
    let mut pc: u32 = 0x82647B88;
    'dispatch: loop {
        match pc {
            0x82647B88 => {
    //   block [0x82647B88..0x82647BF4)
	// 82647B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647B94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82647B9C: 38EBF680  addi r7, r11, -0x980
	ctx.r[7].s64 = ctx.r[11].s64 + -2432;
	// 82647BA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82647BA4: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82647BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647BAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647BB8: 386A0804  addi r3, r10, 0x804
	ctx.r[3].s64 = ctx.r[10].s64 + 2052;
	// 82647BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647BE0: 4BE1F241  bl 0x82466e20
	ctx.lr = 0x82647BE4;
	sub_82466E20(ctx, base);
	// 82647BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647BF8 size=112
    let mut pc: u32 = 0x82647BF8;
    'dispatch: loop {
        match pc {
            0x82647BF8 => {
    //   block [0x82647BF8..0x82647C68)
	// 82647BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647C0C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82647C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647C14: 390BF6C8  addi r8, r11, -0x938
	ctx.r[8].s64 = ctx.r[11].s64 + -2360;
	// 82647C18: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82647C1C: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82647C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647C24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647C30: 386A0834  addi r3, r10, 0x834
	ctx.r[3].s64 = ctx.r[10].s64 + 2100;
	// 82647C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647C54: 4BE1F1CD  bl 0x82466e20
	ctx.lr = 0x82647C58;
	sub_82466E20(ctx, base);
	// 82647C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647C68 size=112
    let mut pc: u32 = 0x82647C68;
    'dispatch: loop {
        match pc {
            0x82647C68 => {
    //   block [0x82647C68..0x82647CD8)
	// 82647C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647C7C: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647C84: 390BF7A0  addi r8, r11, -0x860
	ctx.r[8].s64 = ctx.r[11].s64 + -2144;
	// 82647C88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82647C8C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82647C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647CA0: 386A0864  addi r3, r10, 0x864
	ctx.r[3].s64 = ctx.r[10].s64 + 2148;
	// 82647CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647CC4: 4BE1F15D  bl 0x82466e20
	ctx.lr = 0x82647CC8;
	sub_82466E20(ctx, base);
	// 82647CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647CD8 size=112
    let mut pc: u32 = 0x82647CD8;
    'dispatch: loop {
        match pc {
            0x82647CD8 => {
    //   block [0x82647CD8..0x82647D48)
	// 82647CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647CE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647CEC: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647CF4: 390BF7E8  addi r8, r11, -0x818
	ctx.r[8].s64 = ctx.r[11].s64 + -2072;
	// 82647CF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647CFC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82647D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647D04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647D10: 386A0894  addi r3, r10, 0x894
	ctx.r[3].s64 = ctx.r[10].s64 + 2196;
	// 82647D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647D34: 4BE1F0ED  bl 0x82466e20
	ctx.lr = 0x82647D38;
	sub_82466E20(ctx, base);
	// 82647D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647D48 size=112
    let mut pc: u32 = 0x82647D48;
    'dispatch: loop {
        match pc {
            0x82647D48 => {
    //   block [0x82647D48..0x82647DB8)
	// 82647D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647D5C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82647D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647D64: 390BF848  addi r8, r11, -0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + -1976;
	// 82647D68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647D6C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82647D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647D74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647D80: 386A08C4  addi r3, r10, 0x8c4
	ctx.r[3].s64 = ctx.r[10].s64 + 2244;
	// 82647D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647DA4: 4BE1F07D  bl 0x82466e20
	ctx.lr = 0x82647DA8;
	sub_82466E20(ctx, base);
	// 82647DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647DB8 size=112
    let mut pc: u32 = 0x82647DB8;
    'dispatch: loop {
        match pc {
            0x82647DB8 => {
    //   block [0x82647DB8..0x82647E28)
	// 82647DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647DC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647DCC: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82647DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647DD4: 390BF8A8  addi r8, r11, -0x758
	ctx.r[8].s64 = ctx.r[11].s64 + -1880;
	// 82647DD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82647DDC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82647DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647DE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647DF0: 386A08F4  addi r3, r10, 0x8f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2292;
	// 82647DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647E14: 4BE1F00D  bl 0x82466e20
	ctx.lr = 0x82647E18;
	sub_82466E20(ctx, base);
	// 82647E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647E28 size=112
    let mut pc: u32 = 0x82647E28;
    'dispatch: loop {
        match pc {
            0x82647E28 => {
    //   block [0x82647E28..0x82647E98)
	// 82647E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647E34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647E38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647E3C: 38AAFBA4  addi r5, r10, -0x45c
	ctx.r[5].s64 = ctx.r[10].s64 + -1116;
	// 82647E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647E44: 390BF908  addi r8, r11, -0x6f8
	ctx.r[8].s64 = ctx.r[11].s64 + -1784;
	// 82647E48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82647E4C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82647E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647E60: 386A0924  addi r3, r10, 0x924
	ctx.r[3].s64 = ctx.r[10].s64 + 2340;
	// 82647E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647E84: 4BE1EF9D  bl 0x82466e20
	ctx.lr = 0x82647E88;
	sub_82466E20(ctx, base);
	// 82647E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647E98 size=112
    let mut pc: u32 = 0x82647E98;
    'dispatch: loop {
        match pc {
            0x82647E98 => {
    //   block [0x82647E98..0x82647F08)
	// 82647E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647EA4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82647EA8: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82647EAC: 38EAF9C8  addi r7, r10, -0x638
	ctx.r[7].s64 = ctx.r[10].s64 + -1592;
	// 82647EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647EB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82647EB8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82647EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647EC0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82647EC4: 396B8B10  addi r11, r11, -0x74f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29936;
	// 82647EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82647ECC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647ED4: 386A0954  addi r3, r10, 0x954
	ctx.r[3].s64 = ctx.r[10].s64 + 2388;
	// 82647ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647EDC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82647EE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647EE4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82647EE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647EEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647EF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82647EF4: 4BE1EF2D  bl 0x82466e20
	ctx.lr = 0x82647EF8;
	sub_82466E20(ctx, base);
	// 82647EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647F08 size=112
    let mut pc: u32 = 0x82647F08;
    'dispatch: loop {
        match pc {
            0x82647F08 => {
    //   block [0x82647F08..0x82647F78)
	// 82647F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647F1C: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82647F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647F24: 390BFB60  addi r8, r11, -0x4a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1184;
	// 82647F28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82647F2C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82647F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647F40: 386A0984  addi r3, r10, 0x984
	ctx.r[3].s64 = ctx.r[10].s64 + 2436;
	// 82647F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82647F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647F64: 4BE1EEBD  bl 0x82466e20
	ctx.lr = 0x82647F68;
	sub_82466E20(ctx, base);
	// 82647F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647F78 size=112
    let mut pc: u32 = 0x82647F78;
    'dispatch: loop {
        match pc {
            0x82647F78 => {
    //   block [0x82647F78..0x82647FE8)
	// 82647F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647F88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647F8C: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82647F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82647F94: 390BFB78  addi r8, r11, -0x488
	ctx.r[8].s64 = ctx.r[11].s64 + -1160;
	// 82647F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82647F9C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82647FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82647FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82647FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82647FB0: 386A09B4  addi r3, r10, 0x9b4
	ctx.r[3].s64 = ctx.r[10].s64 + 2484;
	// 82647FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82647FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82647FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82647FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82647FC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82647FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82647FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82647FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82647FD4: 4BE1EE4D  bl 0x82466e20
	ctx.lr = 0x82647FD8;
	sub_82466E20(ctx, base);
	// 82647FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82647FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82647FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82647FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82647FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82647FE8 size=112
    let mut pc: u32 = 0x82647FE8;
    'dispatch: loop {
        match pc {
            0x82647FE8 => {
    //   block [0x82647FE8..0x82648058)
	// 82647FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82647FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82647FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82647FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82647FF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82647FFC: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 82648000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648004: 390BFB90  addi r8, r11, -0x470
	ctx.r[8].s64 = ctx.r[11].s64 + -1136;
	// 82648008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264800C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82648010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648020: 386A09E4  addi r3, r10, 0x9e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2532;
	// 82648024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264802C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264803C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648044: 4BE1EDDD  bl 0x82466e20
	ctx.lr = 0x82648048;
	sub_82466E20(ctx, base);
	// 82648048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648058 size=108
    let mut pc: u32 = 0x82648058;
    'dispatch: loop {
        match pc {
            0x82648058 => {
    //   block [0x82648058..0x826480C4)
	// 82648058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648064: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264806C: 38EBFBC0  addi r7, r11, -0x440
	ctx.r[7].s64 = ctx.r[11].s64 + -1088;
	// 82648070: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648074: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82648078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264807C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648088: 386A0A14  addi r3, r10, 0xa14
	ctx.r[3].s64 = ctx.r[10].s64 + 2580;
	// 8264808C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264809C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826480A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826480A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826480A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826480AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826480B0: 4BE1ED71  bl 0x82466e20
	ctx.lr = 0x826480B4;
	sub_82466E20(ctx, base);
	// 826480B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826480B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826480BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826480C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826480C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826480C8 size=112
    let mut pc: u32 = 0x826480C8;
    'dispatch: loop {
        match pc {
            0x826480C8 => {
    //   block [0x826480C8..0x82648138)
	// 826480C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826480CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826480D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826480D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826480D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826480DC: 38AAF1B4  addi r5, r10, -0xe4c
	ctx.r[5].s64 = ctx.r[10].s64 + -3660;
	// 826480E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826480E4: 390BFBF0  addi r8, r11, -0x410
	ctx.r[8].s64 = ctx.r[11].s64 + -1040;
	// 826480E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826480EC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826480F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826480F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826480F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826480FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648100: 386A0A44  addi r3, r10, 0xa44
	ctx.r[3].s64 = ctx.r[10].s64 + 2628;
	// 82648104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82648118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264811C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648124: 4BE1ECFD  bl 0x82466e20
	ctx.lr = 0x82648128;
	sub_82466E20(ctx, base);
	// 82648128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264812C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648138 size=112
    let mut pc: u32 = 0x82648138;
    'dispatch: loop {
        match pc {
            0x82648138 => {
    //   block [0x82648138..0x826481A8)
	// 82648138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264813C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648148: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264814C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82648150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648154: 390BFC08  addi r8, r11, -0x3f8
	ctx.r[8].s64 = ctx.r[11].s64 + -1016;
	// 82648158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264815C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82648160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264816C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648170: 386A0A74  addi r3, r10, 0xa74
	ctx.r[3].s64 = ctx.r[10].s64 + 2676;
	// 82648174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264817C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264818C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648194: 4BE1EC8D  bl 0x82466e20
	ctx.lr = 0x82648198;
	sub_82466E20(ctx, base);
	// 82648198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264819C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826481A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826481A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826481A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826481A8 size=112
    let mut pc: u32 = 0x826481A8;
    'dispatch: loop {
        match pc {
            0x826481A8 => {
    //   block [0x826481A8..0x82648218)
	// 826481A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826481AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826481B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826481B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826481B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826481BC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826481C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826481C4: 390BFC98  addi r8, r11, -0x368
	ctx.r[8].s64 = ctx.r[11].s64 + -872;
	// 826481C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826481CC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826481D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826481D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826481D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826481DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826481E0: 386A0AA4  addi r3, r10, 0xaa4
	ctx.r[3].s64 = ctx.r[10].s64 + 2724;
	// 826481E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826481E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826481EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826481F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826481F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826481F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826481FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648204: 4BE1EC1D  bl 0x82466e20
	ctx.lr = 0x82648208;
	sub_82466E20(ctx, base);
	// 82648208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264820C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648218 size=112
    let mut pc: u32 = 0x82648218;
    'dispatch: loop {
        match pc {
            0x82648218 => {
    //   block [0x82648218..0x82648288)
	// 82648218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264821C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648228: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264822C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648234: 390BFCC8  addi r8, r11, -0x338
	ctx.r[8].s64 = ctx.r[11].s64 + -824;
	// 82648238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264823C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82648240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264824C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648250: 386A0AD4  addi r3, r10, 0xad4
	ctx.r[3].s64 = ctx.r[10].s64 + 2772;
	// 82648254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264825C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264826C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648274: 4BE1EBAD  bl 0x82466e20
	ctx.lr = 0x82648278;
	sub_82466E20(ctx, base);
	// 82648278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264827C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648288 size=112
    let mut pc: u32 = 0x82648288;
    'dispatch: loop {
        match pc {
            0x82648288 => {
    //   block [0x82648288..0x826482F8)
	// 82648288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264828C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648294: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648298: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264829C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826482A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826482A4: 390BFCE0  addi r8, r11, -0x320
	ctx.r[8].s64 = ctx.r[11].s64 + -800;
	// 826482A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826482AC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826482B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826482B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826482B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826482BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826482C0: 386A0B04  addi r3, r10, 0xb04
	ctx.r[3].s64 = ctx.r[10].s64 + 2820;
	// 826482C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826482C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826482CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826482D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826482D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826482D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826482DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826482E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826482E4: 4BE1EB3D  bl 0x82466e20
	ctx.lr = 0x826482E8;
	sub_82466E20(ctx, base);
	// 826482E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826482EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826482F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826482F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826482F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826482F8 size=108
    let mut pc: u32 = 0x826482F8;
    'dispatch: loop {
        match pc {
            0x826482F8 => {
    //   block [0x826482F8..0x82648364)
	// 826482F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826482FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648304: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264830C: 38EBFCF8  addi r7, r11, -0x308
	ctx.r[7].s64 = ctx.r[11].s64 + -776;
	// 82648310: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648314: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82648318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264831C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648328: 386A0B34  addi r3, r10, 0xb34
	ctx.r[3].s64 = ctx.r[10].s64 + 2868;
	// 8264832C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264833C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264834C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648350: 4BE1EAD1  bl 0x82466e20
	ctx.lr = 0x82648354;
	sub_82466E20(ctx, base);
	// 82648354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264835C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648368 size=112
    let mut pc: u32 = 0x82648368;
    'dispatch: loop {
        match pc {
            0x82648368 => {
    //   block [0x82648368..0x826483D8)
	// 82648368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648378: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264837C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648384: 390BFD28  addi r8, r11, -0x2d8
	ctx.r[8].s64 = ctx.r[11].s64 + -728;
	// 82648388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264838C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82648390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264839C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826483A0: 386A0B64  addi r3, r10, 0xb64
	ctx.r[3].s64 = ctx.r[10].s64 + 2916;
	// 826483A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826483A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826483AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826483B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826483B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826483B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826483BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826483C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826483C4: 4BE1EA5D  bl 0x82466e20
	ctx.lr = 0x826483C8;
	sub_82466E20(ctx, base);
	// 826483C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826483CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826483D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826483D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826483D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826483D8 size=108
    let mut pc: u32 = 0x826483D8;
    'dispatch: loop {
        match pc {
            0x826483D8 => {
    //   block [0x826483D8..0x82648444)
	// 826483D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826483DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826483E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826483E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826483E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826483EC: 38EBFD40  addi r7, r11, -0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + -704;
	// 826483F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826483F4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826483F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826483FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648408: 386A0B94  addi r3, r10, 0xb94
	ctx.r[3].s64 = ctx.r[10].s64 + 2964;
	// 8264840C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264841C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264842C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648430: 4BE1E9F1  bl 0x82466e20
	ctx.lr = 0x82648434;
	sub_82466E20(ctx, base);
	// 82648434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264843C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648448 size=112
    let mut pc: u32 = 0x82648448;
    'dispatch: loop {
        match pc {
            0x82648448 => {
    //   block [0x82648448..0x826484B8)
	// 82648448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264845C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648464: 390BFE18  addi r8, r11, -0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + -488;
	// 82648468: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8264846C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82648470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264847C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648480: 386A0BC4  addi r3, r10, 0xbc4
	ctx.r[3].s64 = ctx.r[10].s64 + 3012;
	// 82648484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264848C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264849C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826484A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826484A4: 4BE1E97D  bl 0x82466e20
	ctx.lr = 0x826484A8;
	sub_82466E20(ctx, base);
	// 826484A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826484AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826484B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826484B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826484B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826484B8 size=108
    let mut pc: u32 = 0x826484B8;
    'dispatch: loop {
        match pc {
            0x826484B8 => {
    //   block [0x826484B8..0x82648524)
	// 826484B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826484BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826484C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826484C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826484C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826484CC: 38EBFFC8  addi r7, r11, -0x38
	ctx.r[7].s64 = ctx.r[11].s64 + -56;
	// 826484D0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826484D4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826484D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826484DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826484E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826484E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826484E8: 386A0BF4  addi r3, r10, 0xbf4
	ctx.r[3].s64 = ctx.r[10].s64 + 3060;
	// 826484EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826484F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826484F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826484F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826484FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264850C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648510: 4BE1E911  bl 0x82466e20
	ctx.lr = 0x82648514;
	sub_82466E20(ctx, base);
	// 82648514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264851C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648528 size=112
    let mut pc: u32 = 0x82648528;
    'dispatch: loop {
        match pc {
            0x82648528 => {
    //   block [0x82648528..0x82648598)
	// 82648528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264852C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648534: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648538: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264853C: 38AAFBD4  addi r5, r10, -0x42c
	ctx.r[5].s64 = ctx.r[10].s64 + -1068;
	// 82648540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648544: 390B0160  addi r8, r11, 0x160
	ctx.r[8].s64 = ctx.r[11].s64 + 352;
	// 82648548: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8264854C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82648550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264855C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648560: 386A0C24  addi r3, r10, 0xc24
	ctx.r[3].s64 = ctx.r[10].s64 + 3108;
	// 82648564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264856C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264857C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648584: 4BE1E89D  bl 0x82466e20
	ctx.lr = 0x82648588;
	sub_82466E20(ctx, base);
	// 82648588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264858C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648598 size=100
    let mut pc: u32 = 0x82648598;
    'dispatch: loop {
        match pc {
            0x82648598 => {
    //   block [0x82648598..0x826485FC)
	// 82648598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826485A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826485A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826485A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826485AC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826485B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826485B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826485B8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826485BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826485C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826485C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826485C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826485CC: 386A0C54  addi r3, r10, 0xc54
	ctx.r[3].s64 = ctx.r[10].s64 + 3156;
	// 826485D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826485D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826485D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826485DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826485E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826485E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826485E8: 4BE1E839  bl 0x82466e20
	ctx.lr = 0x826485EC;
	sub_82466E20(ctx, base);
	// 826485EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826485F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826485F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826485F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648600 size=112
    let mut pc: u32 = 0x82648600;
    'dispatch: loop {
        match pc {
            0x82648600 => {
    //   block [0x82648600..0x82648670)
	// 82648600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264860C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648610: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648614: 38AA0C54  addi r5, r10, 0xc54
	ctx.r[5].s64 = ctx.r[10].s64 + 3156;
	// 82648618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264861C: 390B03D0  addi r8, r11, 0x3d0
	ctx.r[8].s64 = ctx.r[11].s64 + 976;
	// 82648620: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82648624: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82648628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264862C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648638: 386A0C84  addi r3, r10, 0xc84
	ctx.r[3].s64 = ctx.r[10].s64 + 3204;
	// 8264863C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264864C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264865C: 4BE1E7C5  bl 0x82466e20
	ctx.lr = 0x82648660;
	sub_82466E20(ctx, base);
	// 82648660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264866C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648670 size=100
    let mut pc: u32 = 0x82648670;
    'dispatch: loop {
        match pc {
            0x82648670 => {
    //   block [0x82648670..0x826486D4)
	// 82648670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264867C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648684: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264868C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648690: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82648694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264869C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826486A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826486A4: 386A0CB4  addi r3, r10, 0xcb4
	ctx.r[3].s64 = ctx.r[10].s64 + 3252;
	// 826486A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826486AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826486B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826486B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826486B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826486BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826486C0: 4BE1E761  bl 0x82466e20
	ctx.lr = 0x826486C4;
	sub_82466E20(ctx, base);
	// 826486C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826486C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826486CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826486D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826486D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826486D8 size=108
    let mut pc: u32 = 0x826486D8;
    'dispatch: loop {
        match pc {
            0x826486D8 => {
    //   block [0x826486D8..0x82648744)
	// 826486D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826486DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826486E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826486E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826486E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826486EC: 38EB0448  addi r7, r11, 0x448
	ctx.r[7].s64 = ctx.r[11].s64 + 1096;
	// 826486F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826486F4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826486F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826486FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648708: 386A0CE4  addi r3, r10, 0xce4
	ctx.r[3].s64 = ctx.r[10].s64 + 3300;
	// 8264870C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264872C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648730: 4BE1E6F1  bl 0x82466e20
	ctx.lr = 0x82648734;
	sub_82466E20(ctx, base);
	// 82648734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264873C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648748 size=112
    let mut pc: u32 = 0x82648748;
    'dispatch: loop {
        match pc {
            0x82648748 => {
    //   block [0x82648748..0x826487B8)
	// 82648748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264874C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648758: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264875C: 38AA0CB4  addi r5, r10, 0xcb4
	ctx.r[5].s64 = ctx.r[10].s64 + 3252;
	// 82648760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648764: 390B0490  addi r8, r11, 0x490
	ctx.r[8].s64 = ctx.r[11].s64 + 1168;
	// 82648768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264876C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82648770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648774: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264877C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648780: 386A0D14  addi r3, r10, 0xd14
	ctx.r[3].s64 = ctx.r[10].s64 + 3348;
	// 82648784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264878C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264879C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826487A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826487A4: 4BE1E67D  bl 0x82466e20
	ctx.lr = 0x826487A8;
	sub_82466E20(ctx, base);
	// 826487A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826487AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826487B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826487B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826487B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826487B8 size=100
    let mut pc: u32 = 0x826487B8;
    'dispatch: loop {
        match pc {
            0x826487B8 => {
    //   block [0x826487B8..0x8264881C)
	// 826487B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826487BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826487C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826487C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826487C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826487CC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826487D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826487D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826487D8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826487DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826487E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826487E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826487E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826487EC: 386A0D44  addi r3, r10, 0xd44
	ctx.r[3].s64 = ctx.r[10].s64 + 3396;
	// 826487F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826487F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826487F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826487FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648808: 4BE1E619  bl 0x82466e20
	ctx.lr = 0x8264880C;
	sub_82466E20(ctx, base);
	// 8264880C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648820 size=100
    let mut pc: u32 = 0x82648820;
    'dispatch: loop {
        match pc {
            0x82648820 => {
    //   block [0x82648820..0x82648884)
	// 82648820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264882C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648834: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264883C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648840: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82648844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648854: 386A0D74  addi r3, r10, 0xd74
	ctx.r[3].s64 = ctx.r[10].s64 + 3444;
	// 82648858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264885C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648860: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648868: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264886C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648870: 4BE1E5B1  bl 0x82466e20
	ctx.lr = 0x82648874;
	sub_82466E20(ctx, base);
	// 82648874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264887C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648888 size=112
    let mut pc: u32 = 0x82648888;
    'dispatch: loop {
        match pc {
            0x82648888 => {
    //   block [0x82648888..0x826488F8)
	// 82648888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648898: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264889C: 38AA0D44  addi r5, r10, 0xd44
	ctx.r[5].s64 = ctx.r[10].s64 + 3396;
	// 826488A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826488A4: 390B04C0  addi r8, r11, 0x4c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1216;
	// 826488A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826488AC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826488B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826488B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826488B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826488BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826488C0: 386A0DA4  addi r3, r10, 0xda4
	ctx.r[3].s64 = ctx.r[10].s64 + 3492;
	// 826488C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826488C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826488CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826488D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826488D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826488D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826488DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826488E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826488E4: 4BE1E53D  bl 0x82466e20
	ctx.lr = 0x826488E8;
	sub_82466E20(ctx, base);
	// 826488E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826488EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826488F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826488F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826488F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826488F8 size=112
    let mut pc: u32 = 0x826488F8;
    'dispatch: loop {
        match pc {
            0x826488F8 => {
    //   block [0x826488F8..0x82648968)
	// 826488F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826488FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648908: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264890C: 38AA0D74  addi r5, r10, 0xd74
	ctx.r[5].s64 = ctx.r[10].s64 + 3444;
	// 82648910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648914: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 82648918: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264891C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82648920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264892C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648930: 386A0DD4  addi r3, r10, 0xdd4
	ctx.r[3].s64 = ctx.r[10].s64 + 3540;
	// 82648934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264893C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264894C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648954: 4BE1E4CD  bl 0x82466e20
	ctx.lr = 0x82648958;
	sub_82466E20(ctx, base);
	// 82648958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264895C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648968 size=100
    let mut pc: u32 = 0x82648968;
    'dispatch: loop {
        match pc {
            0x82648968 => {
    //   block [0x82648968..0x826489CC)
	// 82648968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264897C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648988: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8264898C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264899C: 386A0E04  addi r3, r10, 0xe04
	ctx.r[3].s64 = ctx.r[10].s64 + 3588;
	// 826489A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826489A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826489A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826489AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826489B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826489B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826489B8: 4BE1E469  bl 0x82466e20
	ctx.lr = 0x826489BC;
	sub_82466E20(ctx, base);
	// 826489BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826489C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826489C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826489C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826489D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826489D0 size=112
    let mut pc: u32 = 0x826489D0;
    'dispatch: loop {
        match pc {
            0x826489D0 => {
    //   block [0x826489D0..0x82648A40)
	// 826489D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826489D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826489D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826489DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826489E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826489E4: 38AA0E04  addi r5, r10, 0xe04
	ctx.r[5].s64 = ctx.r[10].s64 + 3588;
	// 826489E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826489EC: 390B0580  addi r8, r11, 0x580
	ctx.r[8].s64 = ctx.r[11].s64 + 1408;
	// 826489F0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826489F4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826489F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826489FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648A08: 386A0E34  addi r3, r10, 0xe34
	ctx.r[3].s64 = ctx.r[10].s64 + 3636;
	// 82648A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648A2C: 4BE1E3F5  bl 0x82466e20
	ctx.lr = 0x82648A30;
	sub_82466E20(ctx, base);
	// 82648A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648A40 size=100
    let mut pc: u32 = 0x82648A40;
    'dispatch: loop {
        match pc {
            0x82648A40 => {
    //   block [0x82648A40..0x82648AA4)
	// 82648A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648A4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648A54: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648A60: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82648A64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648A74: 386A0E64  addi r3, r10, 0xe64
	ctx.r[3].s64 = ctx.r[10].s64 + 3684;
	// 82648A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648A80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648A88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648A90: 4BE1E391  bl 0x82466e20
	ctx.lr = 0x82648A94;
	sub_82466E20(ctx, base);
	// 82648A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648AA8 size=112
    let mut pc: u32 = 0x82648AA8;
    'dispatch: loop {
        match pc {
            0x82648AA8 => {
    //   block [0x82648AA8..0x82648B18)
	// 82648AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648AB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648ABC: 38AA0E64  addi r5, r10, 0xe64
	ctx.r[5].s64 = ctx.r[10].s64 + 3684;
	// 82648AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648AC4: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 82648AC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648ACC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82648AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648AD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648AE0: 386A0E94  addi r3, r10, 0xe94
	ctx.r[3].s64 = ctx.r[10].s64 + 3732;
	// 82648AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648B04: 4BE1E31D  bl 0x82466e20
	ctx.lr = 0x82648B08;
	sub_82466E20(ctx, base);
	// 82648B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648B18 size=108
    let mut pc: u32 = 0x82648B18;
    'dispatch: loop {
        match pc {
            0x82648B18 => {
    //   block [0x82648B18..0x82648B84)
	// 82648B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648B24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648B2C: 38EB06B8  addi r7, r11, 0x6b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1720;
	// 82648B30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82648B34: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82648B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648B40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648B48: 386A0EC4  addi r3, r10, 0xec4
	ctx.r[3].s64 = ctx.r[10].s64 + 3780;
	// 82648B4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648B70: 4BE1E2B1  bl 0x82466e20
	ctx.lr = 0x82648B74;
	sub_82466E20(ctx, base);
	// 82648B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648B88 size=112
    let mut pc: u32 = 0x82648B88;
    'dispatch: loop {
        match pc {
            0x82648B88 => {
    //   block [0x82648B88..0x82648BF8)
	// 82648B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648B98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648B9C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648BA4: 390B0700  addi r8, r11, 0x700
	ctx.r[8].s64 = ctx.r[11].s64 + 1792;
	// 82648BA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648BAC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82648BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648BB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648BC0: 386A0EF4  addi r3, r10, 0xef4
	ctx.r[3].s64 = ctx.r[10].s64 + 3828;
	// 82648BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648BE4: 4BE1E23D  bl 0x82466e20
	ctx.lr = 0x82648BE8;
	sub_82466E20(ctx, base);
	// 82648BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648BF8 size=108
    let mut pc: u32 = 0x82648BF8;
    'dispatch: loop {
        match pc {
            0x82648BF8 => {
    //   block [0x82648BF8..0x82648C64)
	// 82648BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648C04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648C0C: 38EB0718  addi r7, r11, 0x718
	ctx.r[7].s64 = ctx.r[11].s64 + 1816;
	// 82648C10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82648C14: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82648C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648C1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648C28: 386A0F24  addi r3, r10, 0xf24
	ctx.r[3].s64 = ctx.r[10].s64 + 3876;
	// 82648C2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648C50: 4BE1E1D1  bl 0x82466e20
	ctx.lr = 0x82648C54;
	sub_82466E20(ctx, base);
	// 82648C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648C68 size=112
    let mut pc: u32 = 0x82648C68;
    'dispatch: loop {
        match pc {
            0x82648C68 => {
    //   block [0x82648C68..0x82648CD8)
	// 82648C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648C74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648C7C: 38AA0EF4  addi r5, r10, 0xef4
	ctx.r[5].s64 = ctx.r[10].s64 + 3828;
	// 82648C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648C84: 390B0760  addi r8, r11, 0x760
	ctx.r[8].s64 = ctx.r[11].s64 + 1888;
	// 82648C88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648C8C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82648C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648C94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648CA0: 386A0F54  addi r3, r10, 0xf54
	ctx.r[3].s64 = ctx.r[10].s64 + 3924;
	// 82648CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648CC4: 4BE1E15D  bl 0x82466e20
	ctx.lr = 0x82648CC8;
	sub_82466E20(ctx, base);
	// 82648CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648CD8 size=100
    let mut pc: u32 = 0x82648CD8;
    'dispatch: loop {
        match pc {
            0x82648CD8 => {
    //   block [0x82648CD8..0x82648D3C)
	// 82648CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648CEC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648CF8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82648CFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648D0C: 386A0F84  addi r3, r10, 0xf84
	ctx.r[3].s64 = ctx.r[10].s64 + 3972;
	// 82648D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648D28: 4BE1E0F9  bl 0x82466e20
	ctx.lr = 0x82648D2C;
	sub_82466E20(ctx, base);
	// 82648D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648D40 size=112
    let mut pc: u32 = 0x82648D40;
    'dispatch: loop {
        match pc {
            0x82648D40 => {
    //   block [0x82648D40..0x82648DB0)
	// 82648D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648D4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648D54: 38AA0F84  addi r5, r10, 0xf84
	ctx.r[5].s64 = ctx.r[10].s64 + 3972;
	// 82648D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648D5C: 390B0778  addi r8, r11, 0x778
	ctx.r[8].s64 = ctx.r[11].s64 + 1912;
	// 82648D60: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82648D64: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82648D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648D78: 386A0FB4  addi r3, r10, 0xfb4
	ctx.r[3].s64 = ctx.r[10].s64 + 4020;
	// 82648D7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648D80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648D9C: 4BE1E085  bl 0x82466e20
	ctx.lr = 0x82648DA0;
	sub_82466E20(ctx, base);
	// 82648DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648DB0 size=108
    let mut pc: u32 = 0x82648DB0;
    'dispatch: loop {
        match pc {
            0x82648DB0 => {
    //   block [0x82648DB0..0x82648E1C)
	// 82648DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648DBC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648DC4: 38EB0820  addi r7, r11, 0x820
	ctx.r[7].s64 = ctx.r[11].s64 + 2080;
	// 82648DC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82648DCC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82648DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648DD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648DD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82648DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648DE0: 386A0FE4  addi r3, r10, 0xfe4
	ctx.r[3].s64 = ctx.r[10].s64 + 4068;
	// 82648DE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82648DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82648E08: 4BE1E019  bl 0x82466e20
	ctx.lr = 0x82648E0C;
	sub_82466E20(ctx, base);
	// 82648E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648E20 size=112
    let mut pc: u32 = 0x82648E20;
    'dispatch: loop {
        match pc {
            0x82648E20 => {
    //   block [0x82648E20..0x82648E90)
	// 82648E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648E30: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648E34: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648E3C: 390B0850  addi r8, r11, 0x850
	ctx.r[8].s64 = ctx.r[11].s64 + 2128;
	// 82648E40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648E44: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82648E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648E4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648E58: 386A1014  addi r3, r10, 0x1014
	ctx.r[3].s64 = ctx.r[10].s64 + 4116;
	// 82648E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648E7C: 4BE1DFA5  bl 0x82466e20
	ctx.lr = 0x82648E80;
	sub_82466E20(ctx, base);
	// 82648E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648E90 size=112
    let mut pc: u32 = 0x82648E90;
    'dispatch: loop {
        match pc {
            0x82648E90 => {
    //   block [0x82648E90..0x82648F00)
	// 82648E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648E9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648EA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648EA4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648EAC: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 82648EB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648EB4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82648EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648EC8: 386A1044  addi r3, r10, 0x1044
	ctx.r[3].s64 = ctx.r[10].s64 + 4164;
	// 82648ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648EEC: 4BE1DF35  bl 0x82466e20
	ctx.lr = 0x82648EF0;
	sub_82466E20(ctx, base);
	// 82648EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648F00 size=100
    let mut pc: u32 = 0x82648F00;
    'dispatch: loop {
        match pc {
            0x82648F00 => {
    //   block [0x82648F00..0x82648F64)
	// 82648F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648F0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648F14: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648F20: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82648F24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648F34: 386A1074  addi r3, r10, 0x1074
	ctx.r[3].s64 = ctx.r[10].s64 + 4212;
	// 82648F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82648F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82648F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648F50: 4BE1DED1  bl 0x82466e20
	ctx.lr = 0x82648F54;
	sub_82466E20(ctx, base);
	// 82648F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648F68 size=112
    let mut pc: u32 = 0x82648F68;
    'dispatch: loop {
        match pc {
            0x82648F68 => {
    //   block [0x82648F68..0x82648FD8)
	// 82648F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648F74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648F7C: 38AA1074  addi r5, r10, 0x1074
	ctx.r[5].s64 = ctx.r[10].s64 + 4212;
	// 82648F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648F84: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 82648F88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82648F8C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82648F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82648F94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82648F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82648FA0: 386A10A4  addi r3, r10, 0x10a4
	ctx.r[3].s64 = ctx.r[10].s64 + 4260;
	// 82648FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82648FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82648FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82648FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82648FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82648FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82648FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82648FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82648FC4: 4BE1DE5D  bl 0x82466e20
	ctx.lr = 0x82648FC8;
	sub_82466E20(ctx, base);
	// 82648FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82648FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82648FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82648FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82648FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82648FD8 size=112
    let mut pc: u32 = 0x82648FD8;
    'dispatch: loop {
        match pc {
            0x82648FD8 => {
    //   block [0x82648FD8..0x82649048)
	// 82648FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82648FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82648FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82648FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82648FE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82648FEC: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82648FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82648FF4: 390B0928  addi r8, r11, 0x928
	ctx.r[8].s64 = ctx.r[11].s64 + 2344;
	// 82648FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82648FFC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82649000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649004: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649010: 386A10D4  addi r3, r10, 0x10d4
	ctx.r[3].s64 = ctx.r[10].s64 + 4308;
	// 82649014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264901C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264902C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649034: 4BE1DDED  bl 0x82466e20
	ctx.lr = 0x82649038;
	sub_82466E20(ctx, base);
	// 82649038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264903C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649048 size=112
    let mut pc: u32 = 0x82649048;
    'dispatch: loop {
        match pc {
            0x82649048 => {
    //   block [0x82649048..0x826490B8)
	// 82649048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264904C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649058: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264905C: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82649060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649064: 390B0940  addi r8, r11, 0x940
	ctx.r[8].s64 = ctx.r[11].s64 + 2368;
	// 82649068: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264906C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82649070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649074: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264907C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649080: 386A1104  addi r3, r10, 0x1104
	ctx.r[3].s64 = ctx.r[10].s64 + 4356;
	// 82649084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264908C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649094: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82649098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264909C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826490A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826490A4: 4BE1DD7D  bl 0x82466e20
	ctx.lr = 0x826490A8;
	sub_82466E20(ctx, base);
	// 826490A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826490AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826490B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826490B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826490B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826490B8 size=112
    let mut pc: u32 = 0x826490B8;
    'dispatch: loop {
        match pc {
            0x826490B8 => {
    //   block [0x826490B8..0x82649128)
	// 826490B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826490BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826490C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826490C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826490C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826490CC: 38AA10D4  addi r5, r10, 0x10d4
	ctx.r[5].s64 = ctx.r[10].s64 + 4308;
	// 826490D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826490D4: 390B0958  addi r8, r11, 0x958
	ctx.r[8].s64 = ctx.r[11].s64 + 2392;
	// 826490D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826490DC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826490E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826490E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826490E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826490EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826490F0: 386A1134  addi r3, r10, 0x1134
	ctx.r[3].s64 = ctx.r[10].s64 + 4404;
	// 826490F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826490F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826490FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649114: 4BE1DD0D  bl 0x82466e20
	ctx.lr = 0x82649118;
	sub_82466E20(ctx, base);
	// 82649118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649128 size=72
    let mut pc: u32 = 0x82649128;
    'dispatch: loop {
        match pc {
            0x82649128 => {
    //   block [0x82649128..0x82649170)
	// 82649128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649134: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649138: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8264913C: 38CB8448  addi r6, r11, -0x7bb8
	ctx.r[6].s64 = ctx.r[11].s64 + -31672;
	// 82649140: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649144: 388B8B68  addi r4, r11, -0x7498
	ctx.r[4].s64 = ctx.r[11].s64 + -29848;
	// 82649148: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264914C: 386B1164  addi r3, r11, 0x1164
	ctx.r[3].s64 = ctx.r[11].s64 + 4452;
	// 82649150: 4BE32939  bl 0x8247ba88
	ctx.lr = 0x82649154;
	sub_8247BA88(ctx, base);
	// 82649154: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82649158: 386BCDB0  addi r3, r11, -0x3250
	ctx.r[3].s64 = ctx.r[11].s64 + -12880;
	// 8264915C: 4BEE99DD  bl 0x82532b38
	ctx.lr = 0x82649160;
	sub_82532B38(ctx, base);
	// 82649160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82649164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264916C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649170 size=108
    let mut pc: u32 = 0x82649170;
    'dispatch: loop {
        match pc {
            0x82649170 => {
    //   block [0x82649170..0x826491DC)
	// 82649170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264917C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649184: 38EB10A8  addi r7, r11, 0x10a8
	ctx.r[7].s64 = ctx.r[11].s64 + 4264;
	// 82649188: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8264918C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82649190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264919C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826491A0: 386A1180  addi r3, r10, 0x1180
	ctx.r[3].s64 = ctx.r[10].s64 + 4480;
	// 826491A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826491A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826491AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826491B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826491B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826491B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826491BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826491C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826491C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826491C8: 4BE1DC59  bl 0x82466e20
	ctx.lr = 0x826491CC;
	sub_82466E20(ctx, base);
	// 826491CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826491D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826491D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826491D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826491E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826491E0 size=24
    let mut pc: u32 = 0x826491E0;
    'dispatch: loop {
        match pc {
            0x826491E0 => {
    //   block [0x826491E0..0x826491F8)
	// 826491E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826491E4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826491E8: 394A76C0  addi r10, r10, 0x76c0
	ctx.r[10].s64 = ctx.r[10].s64 + 30400;
	// 826491EC: 816B1120  lwz r11, 0x1120(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4384 as u32) ) } as u64;
	// 826491F0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826491F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826491F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826491F8 size=112
    let mut pc: u32 = 0x826491F8;
    'dispatch: loop {
        match pc {
            0x826491F8 => {
    //   block [0x826491F8..0x82649268)
	// 826491F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826491FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649204: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264920C: 392B9504  addi r9, r11, -0x6afc
	ctx.r[9].s64 = ctx.r[11].s64 + -27388;
	// 82649210: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82649214: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82649218: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264921C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82649220: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649224: 396B76C0  addi r11, r11, 0x76c0
	ctx.r[11].s64 = ctx.r[11].s64 + 30400;
	// 82649228: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264922C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649230: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82649234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649238: 386A11B0  addi r3, r10, 0x11b0
	ctx.r[3].s64 = ctx.r[10].s64 + 4528;
	// 8264923C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649240: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82649244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649248: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264924C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649254: 4BE1DBCD  bl 0x82466e20
	ctx.lr = 0x82649258;
	sub_82466E20(ctx, base);
	// 82649258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264925C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649268 size=108
    let mut pc: u32 = 0x82649268;
    'dispatch: loop {
        match pc {
            0x82649268 => {
    //   block [0x82649268..0x826492D4)
	// 82649268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649274: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264927C: 38EB1124  addi r7, r11, 0x1124
	ctx.r[7].s64 = ctx.r[11].s64 + 4388;
	// 82649280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82649284: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82649288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264928C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649298: 386A11E0  addi r3, r10, 0x11e0
	ctx.r[3].s64 = ctx.r[10].s64 + 4576;
	// 8264929C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826492A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826492A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826492A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826492AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826492B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826492B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826492B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826492BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826492C0: 4BE1DB61  bl 0x82466e20
	ctx.lr = 0x826492C4;
	sub_82466E20(ctx, base);
	// 826492C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826492C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826492CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826492D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826492D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826492D8 size=108
    let mut pc: u32 = 0x826492D8;
    'dispatch: loop {
        match pc {
            0x826492D8 => {
    //   block [0x826492D8..0x82649344)
	// 826492D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826492DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826492E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826492E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826492E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826492EC: 38EB1154  addi r7, r11, 0x1154
	ctx.r[7].s64 = ctx.r[11].s64 + 4436;
	// 826492F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826492F4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826492F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826492FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649308: 386A1210  addi r3, r10, 0x1210
	ctx.r[3].s64 = ctx.r[10].s64 + 4624;
	// 8264930C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264931C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264932C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649330: 4BE1DAF1  bl 0x82466e20
	ctx.lr = 0x82649334;
	sub_82466E20(ctx, base);
	// 82649334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264933C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82649348 size=24
    let mut pc: u32 = 0x82649348;
    'dispatch: loop {
        match pc {
            0x82649348 => {
    //   block [0x82649348..0x82649360)
	// 82649348: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264934C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649350: 394A7708  addi r10, r10, 0x7708
	ctx.r[10].s64 = ctx.r[10].s64 + 30472;
	// 82649354: 816B1184  lwz r11, 0x1184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4484 as u32) ) } as u64;
	// 82649358: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649360 size=116
    let mut pc: u32 = 0x82649360;
    'dispatch: loop {
        match pc {
            0x82649360 => {
    //   block [0x82649360..0x826493D4)
	// 82649360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264936C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649370: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649374: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 82649378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264937C: 392A9548  addi r9, r10, -0x6ab8
	ctx.r[9].s64 = ctx.r[10].s64 + -27320;
	// 82649380: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649384: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82649388: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264938C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649394: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264939C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826493A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826493A4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826493A8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826493AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826493B0: 386B1240  addi r3, r11, 0x1240
	ctx.r[3].s64 = ctx.r[11].s64 + 4672;
	// 826493B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826493B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826493BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826493C0: 4BE1DA61  bl 0x82466e20
	ctx.lr = 0x826493C4;
	sub_82466E20(ctx, base);
	// 826493C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826493C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826493CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826493D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826493D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826493D8 size=108
    let mut pc: u32 = 0x826493D8;
    'dispatch: loop {
        match pc {
            0x826493D8 => {
    //   block [0x826493D8..0x82649444)
	// 826493D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826493DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826493E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826493E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826493E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826493EC: 38EB1188  addi r7, r11, 0x1188
	ctx.r[7].s64 = ctx.r[11].s64 + 4488;
	// 826493F0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826493F4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826493F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826493FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649408: 386A1270  addi r3, r10, 0x1270
	ctx.r[3].s64 = ctx.r[10].s64 + 4720;
	// 8264940C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264942C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649430: 4BE1D9F1  bl 0x82466e20
	ctx.lr = 0x82649434;
	sub_82466E20(ctx, base);
	// 82649434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649448 size=112
    let mut pc: u32 = 0x82649448;
    'dispatch: loop {
        match pc {
            0x82649448 => {
    //   block [0x82649448..0x826494B8)
	// 82649448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264945C: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 82649460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649464: 390B1218  addi r8, r11, 0x1218
	ctx.r[8].s64 = ctx.r[11].s64 + 4632;
	// 82649468: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8264946C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82649470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264947C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649480: 386A12A0  addi r3, r10, 0x12a0
	ctx.r[3].s64 = ctx.r[10].s64 + 4768;
	// 82649484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264948C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264949C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826494A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826494A4: 4BE1D97D  bl 0x82466e20
	ctx.lr = 0x826494A8;
	sub_82466E20(ctx, base);
	// 826494A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826494AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826494B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826494B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826494B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826494B8 size=112
    let mut pc: u32 = 0x826494B8;
    'dispatch: loop {
        match pc {
            0x826494B8 => {
    //   block [0x826494B8..0x82649528)
	// 826494B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826494BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826494C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826494C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826494C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826494CC: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 826494D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826494D4: 390B1338  addi r8, r11, 0x1338
	ctx.r[8].s64 = ctx.r[11].s64 + 4920;
	// 826494D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826494DC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826494E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826494E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826494E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826494EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826494F0: 386A12D0  addi r3, r10, 0x12d0
	ctx.r[3].s64 = ctx.r[10].s64 + 4816;
	// 826494F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826494F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826494FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264950C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649514: 4BE1D90D  bl 0x82466e20
	ctx.lr = 0x82649518;
	sub_82466E20(ctx, base);
	// 82649518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649528 size=108
    let mut pc: u32 = 0x82649528;
    'dispatch: loop {
        match pc {
            0x82649528 => {
    //   block [0x82649528..0x82649594)
	// 82649528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649534: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264953C: 38EB1350  addi r7, r11, 0x1350
	ctx.r[7].s64 = ctx.r[11].s64 + 4944;
	// 82649540: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82649544: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82649548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264954C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649558: 386A1300  addi r3, r10, 0x1300
	ctx.r[3].s64 = ctx.r[10].s64 + 4864;
	// 8264955C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264956C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264957C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649580: 4BE1D8A1  bl 0x82466e20
	ctx.lr = 0x82649584;
	sub_82466E20(ctx, base);
	// 82649584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649598 size=112
    let mut pc: u32 = 0x82649598;
    'dispatch: loop {
        match pc {
            0x82649598 => {
    //   block [0x82649598..0x82649608)
	// 82649598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264959C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826495A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826495A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826495A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826495AC: 38AA1240  addi r5, r10, 0x1240
	ctx.r[5].s64 = ctx.r[10].s64 + 4672;
	// 826495B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826495B4: 390B13E0  addi r8, r11, 0x13e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5088;
	// 826495B8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826495BC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826495C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826495C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826495C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826495CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826495D0: 386A1330  addi r3, r10, 0x1330
	ctx.r[3].s64 = ctx.r[10].s64 + 4912;
	// 826495D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826495D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826495DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826495E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826495E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826495E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826495EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826495F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826495F4: 4BE1D82D  bl 0x82466e20
	ctx.lr = 0x826495F8;
	sub_82466E20(ctx, base);
	// 826495F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826495FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649608 size=108
    let mut pc: u32 = 0x82649608;
    'dispatch: loop {
        match pc {
            0x82649608 => {
    //   block [0x82649608..0x82649674)
	// 82649608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264960C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649614: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264961C: 38EB14D0  addi r7, r11, 0x14d0
	ctx.r[7].s64 = ctx.r[11].s64 + 5328;
	// 82649620: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82649624: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82649628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264962C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649638: 386A1360  addi r3, r10, 0x1360
	ctx.r[3].s64 = ctx.r[10].s64 + 4960;
	// 8264963C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264964C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264965C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649660: 4BE1D7C1  bl 0x82466e20
	ctx.lr = 0x82649664;
	sub_82466E20(ctx, base);
	// 82649664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264966C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649678 size=108
    let mut pc: u32 = 0x82649678;
    'dispatch: loop {
        match pc {
            0x82649678 => {
    //   block [0x82649678..0x826496E4)
	// 82649678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649684: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264968C: 38EB14E8  addi r7, r11, 0x14e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5352;
	// 82649690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649694: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82649698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264969C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826496A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826496A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826496A8: 386A1390  addi r3, r10, 0x1390
	ctx.r[3].s64 = ctx.r[10].s64 + 5008;
	// 826496AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826496B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826496B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826496B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826496BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826496C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826496C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826496C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826496CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826496D0: 4BE1D751  bl 0x82466e20
	ctx.lr = 0x826496D4;
	sub_82466E20(ctx, base);
	// 826496D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826496D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826496DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826496E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826496E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826496E8 size=116
    let mut pc: u32 = 0x826496E8;
    'dispatch: loop {
        match pc {
            0x826496E8 => {
    //   block [0x826496E8..0x8264975C)
	// 826496E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826496EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826496F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826496F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826496F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826496FC: 390B154C  addi r8, r11, 0x154c
	ctx.r[8].s64 = ctx.r[11].s64 + 5452;
	// 82649700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649704: 392A9574  addi r9, r10, -0x6a8c
	ctx.r[9].s64 = ctx.r[10].s64 + -27276;
	// 82649708: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264970C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82649710: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82649714: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264971C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264972C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82649730: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82649734: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649738: 386B13C0  addi r3, r11, 0x13c0
	ctx.r[3].s64 = ctx.r[11].s64 + 5056;
	// 8264973C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649740: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649748: 4BE1D6D9  bl 0x82466e20
	ctx.lr = 0x8264974C;
	sub_82466E20(ctx, base);
	// 8264974C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649760 size=108
    let mut pc: u32 = 0x82649760;
    'dispatch: loop {
        match pc {
            0x82649760 => {
    //   block [0x82649760..0x826497CC)
	// 82649760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264976C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649774: 38EB1568  addi r7, r11, 0x1568
	ctx.r[7].s64 = ctx.r[11].s64 + 5480;
	// 82649778: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264977C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82649780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264978C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649790: 386A13F0  addi r3, r10, 0x13f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5104;
	// 82649794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264979C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826497A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826497A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826497A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826497AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826497B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826497B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826497B8: 4BE1D669  bl 0x82466e20
	ctx.lr = 0x826497BC;
	sub_82466E20(ctx, base);
	// 826497BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826497C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826497C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826497C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826497D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826497D0 size=108
    let mut pc: u32 = 0x826497D0;
    'dispatch: loop {
        match pc {
            0x826497D0 => {
    //   block [0x826497D0..0x8264983C)
	// 826497D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826497D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826497D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826497DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826497E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826497E4: 38EB15B0  addi r7, r11, 0x15b0
	ctx.r[7].s64 = ctx.r[11].s64 + 5552;
	// 826497E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826497EC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826497F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826497F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826497F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826497FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649800: 386A1420  addi r3, r10, 0x1420
	ctx.r[3].s64 = ctx.r[10].s64 + 5152;
	// 82649804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264980C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264981C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649828: 4BE1D5F9  bl 0x82466e20
	ctx.lr = 0x8264982C;
	sub_82466E20(ctx, base);
	// 8264982C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649840 size=108
    let mut pc: u32 = 0x82649840;
    'dispatch: loop {
        match pc {
            0x82649840 => {
    //   block [0x82649840..0x826498AC)
	// 82649840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264984C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649854: 38EB1640  addi r7, r11, 0x1640
	ctx.r[7].s64 = ctx.r[11].s64 + 5696;
	// 82649858: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264985C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82649860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264986C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649870: 386A1450  addi r3, r10, 0x1450
	ctx.r[3].s64 = ctx.r[10].s64 + 5200;
	// 82649874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264987C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264988C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649898: 4BE1D589  bl 0x82466e20
	ctx.lr = 0x8264989C;
	sub_82466E20(ctx, base);
	// 8264989C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826498A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826498A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826498A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826498B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826498B0 size=100
    let mut pc: u32 = 0x826498B0;
    'dispatch: loop {
        match pc {
            0x826498B0 => {
    //   block [0x826498B0..0x82649914)
	// 826498B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826498B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826498B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826498BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826498C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826498C4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826498C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826498CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826498D0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826498D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826498D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826498DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826498E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826498E4: 386A1480  addi r3, r10, 0x1480
	ctx.r[3].s64 = ctx.r[10].s64 + 5248;
	// 826498E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826498EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826498F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826498F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826498F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826498FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649900: 4BE1D521  bl 0x82466e20
	ctx.lr = 0x82649904;
	sub_82466E20(ctx, base);
	// 82649904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649918 size=112
    let mut pc: u32 = 0x82649918;
    'dispatch: loop {
        match pc {
            0x82649918 => {
    //   block [0x82649918..0x82649988)
	// 82649918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264991C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649924: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649928: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264992C: 38AA1480  addi r5, r10, 0x1480
	ctx.r[5].s64 = ctx.r[10].s64 + 5248;
	// 82649930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649934: 390B16D0  addi r8, r11, 0x16d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5840;
	// 82649938: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264993C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82649940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264994C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649950: 386A14B0  addi r3, r10, 0x14b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5296;
	// 82649954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264995C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264996C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649974: 4BE1D4AD  bl 0x82466e20
	ctx.lr = 0x82649978;
	sub_82466E20(ctx, base);
	// 82649978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264997C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649988 size=108
    let mut pc: u32 = 0x82649988;
    'dispatch: loop {
        match pc {
            0x82649988 => {
    //   block [0x82649988..0x826499F4)
	// 82649988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649994: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264999C: 38EB1730  addi r7, r11, 0x1730
	ctx.r[7].s64 = ctx.r[11].s64 + 5936;
	// 826499A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826499A4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826499A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826499AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826499B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826499B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826499B8: 386A14E0  addi r3, r10, 0x14e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5344;
	// 826499BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826499C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826499C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826499C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826499CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826499D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826499D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826499D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826499DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826499E0: 4BE1D441  bl 0x82466e20
	ctx.lr = 0x826499E4;
	sub_82466E20(ctx, base);
	// 826499E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826499E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826499EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826499F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826499F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826499F8 size=108
    let mut pc: u32 = 0x826499F8;
    'dispatch: loop {
        match pc {
            0x826499F8 => {
    //   block [0x826499F8..0x82649A64)
	// 826499F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826499FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649A04: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649A0C: 38EB1760  addi r7, r11, 0x1760
	ctx.r[7].s64 = ctx.r[11].s64 + 5984;
	// 82649A10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649A14: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82649A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649A28: 386A1510  addi r3, r10, 0x1510
	ctx.r[3].s64 = ctx.r[10].s64 + 5392;
	// 82649A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649A50: 4BE1D3D1  bl 0x82466e20
	ctx.lr = 0x82649A54;
	sub_82466E20(ctx, base);
	// 82649A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649A68 size=108
    let mut pc: u32 = 0x82649A68;
    'dispatch: loop {
        match pc {
            0x82649A68 => {
    //   block [0x82649A68..0x82649AD4)
	// 82649A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649A74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649A7C: 38EB17C0  addi r7, r11, 0x17c0
	ctx.r[7].s64 = ctx.r[11].s64 + 6080;
	// 82649A80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82649A84: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82649A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649A8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649A98: 386A1540  addi r3, r10, 0x1540
	ctx.r[3].s64 = ctx.r[10].s64 + 5440;
	// 82649A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649AC0: 4BE1D361  bl 0x82466e20
	ctx.lr = 0x82649AC4;
	sub_82466E20(ctx, base);
	// 82649AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649AD8 size=108
    let mut pc: u32 = 0x82649AD8;
    'dispatch: loop {
        match pc {
            0x82649AD8 => {
    //   block [0x82649AD8..0x82649B44)
	// 82649AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649AE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649AEC: 38EB1820  addi r7, r11, 0x1820
	ctx.r[7].s64 = ctx.r[11].s64 + 6176;
	// 82649AF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82649AF4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82649AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649AFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649B08: 386A1570  addi r3, r10, 0x1570
	ctx.r[3].s64 = ctx.r[10].s64 + 5488;
	// 82649B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649B30: 4BE1D2F1  bl 0x82466e20
	ctx.lr = 0x82649B34;
	sub_82466E20(ctx, base);
	// 82649B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649B48 size=112
    let mut pc: u32 = 0x82649B48;
    'dispatch: loop {
        match pc {
            0x82649B48 => {
    //   block [0x82649B48..0x82649BB8)
	// 82649B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649B54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649B58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649B5C: 392A95A8  addi r9, r10, -0x6a58
	ctx.r[9].s64 = ctx.r[10].s64 + -27224;
	// 82649B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649B64: 390B1898  addi r8, r11, 0x1898
	ctx.r[8].s64 = ctx.r[11].s64 + 6296;
	// 82649B68: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82649B6C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82649B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649B74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649B80: 386A15A0  addi r3, r10, 0x15a0
	ctx.r[3].s64 = ctx.r[10].s64 + 5536;
	// 82649B84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649B88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649BA4: 4BE1D27D  bl 0x82466e20
	ctx.lr = 0x82649BA8;
	sub_82466E20(ctx, base);
	// 82649BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649BB8 size=112
    let mut pc: u32 = 0x82649BB8;
    'dispatch: loop {
        match pc {
            0x82649BB8 => {
    //   block [0x82649BB8..0x82649C28)
	// 82649BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649BC4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649BC8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82649BCC: 38EA19A0  addi r7, r10, 0x19a0
	ctx.r[7].s64 = ctx.r[10].s64 + 6560;
	// 82649BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649BD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649BD8: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82649BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649BE0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649BE4: 396B95BC  addi r11, r11, -0x6a44
	ctx.r[11].s64 = ctx.r[11].s64 + -27204;
	// 82649BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649BEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649BF4: 386A15D0  addi r3, r10, 0x15d0
	ctx.r[3].s64 = ctx.r[10].s64 + 5584;
	// 82649BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649BFC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649C04: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649C0C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649C14: 4BE1D20D  bl 0x82466e20
	ctx.lr = 0x82649C18;
	sub_82466E20(ctx, base);
	// 82649C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649C28 size=112
    let mut pc: u32 = 0x82649C28;
    'dispatch: loop {
        match pc {
            0x82649C28 => {
    //   block [0x82649C28..0x82649C98)
	// 82649C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649C34: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82649C38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649C3C: 392A9600  addi r9, r10, -0x6a00
	ctx.r[9].s64 = ctx.r[10].s64 + -27136;
	// 82649C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649C44: 390B1AAC  addi r8, r11, 0x1aac
	ctx.r[8].s64 = ctx.r[11].s64 + 6828;
	// 82649C48: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82649C4C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82649C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649C54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649C60: 386A1600  addi r3, r10, 0x1600
	ctx.r[3].s64 = ctx.r[10].s64 + 5632;
	// 82649C64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649C68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649C84: 4BE1D19D  bl 0x82466e20
	ctx.lr = 0x82649C88;
	sub_82466E20(ctx, base);
	// 82649C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649C98 size=100
    let mut pc: u32 = 0x82649C98;
    'dispatch: loop {
        match pc {
            0x82649C98 => {
    //   block [0x82649C98..0x82649CFC)
	// 82649C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649CA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649CAC: 38AA1BD0  addi r5, r10, 0x1bd0
	ctx.r[5].s64 = ctx.r[10].s64 + 7120;
	// 82649CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649CB8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82649CBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649CCC: 386A1630  addi r3, r10, 0x1630
	ctx.r[3].s64 = ctx.r[10].s64 + 5680;
	// 82649CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82649CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649CE8: 4BE1D139  bl 0x82466e20
	ctx.lr = 0x82649CEC;
	sub_82466E20(ctx, base);
	// 82649CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649D00 size=116
    let mut pc: u32 = 0x82649D00;
    'dispatch: loop {
        match pc {
            0x82649D00 => {
    //   block [0x82649D00..0x82649D74)
	// 82649D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649D0C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649D10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82649D14: 390A1AE0  addi r8, r10, 0x1ae0
	ctx.r[8].s64 = ctx.r[10].s64 + 6880;
	// 82649D18: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649D20: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649D28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649D34: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82649D38: 396B9614  addi r11, r11, -0x69ec
	ctx.r[11].s64 = ctx.r[11].s64 + -27116;
	// 82649D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649D44: 386A1660  addi r3, r10, 0x1660
	ctx.r[3].s64 = ctx.r[10].s64 + 5728;
	// 82649D48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649D50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649D60: 4BE1D0C1  bl 0x82466e20
	ctx.lr = 0x82649D64;
	sub_82466E20(ctx, base);
	// 82649D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


