pub fn sub_82609348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609348 size=108
    let mut pc: u32 = 0x82609348;
    'dispatch: loop {
        match pc {
            0x82609348 => {
    //   block [0x82609348..0x826093B4)
	// 82609348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260934C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609354: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260935C: 38EBE828  addi r7, r11, -0x17d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6104;
	// 82609360: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82609364: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82609368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260936C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609378: 386A6104  addi r3, r10, 0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + 24836;
	// 8260937C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260939C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826093A0: 4BE5DA81  bl 0x82466e20
	ctx.lr = 0x826093A4;
	sub_82466E20(ctx, base);
	// 826093A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826093A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826093AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826093B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826093B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826093B8 size=108
    let mut pc: u32 = 0x826093B8;
    'dispatch: loop {
        match pc {
            0x826093B8 => {
    //   block [0x826093B8..0x82609424)
	// 826093B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826093BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826093C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826093C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826093C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826093CC: 38EBE8B8  addi r7, r11, -0x1748
	ctx.r[7].s64 = ctx.r[11].s64 + -5960;
	// 826093D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826093D4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826093D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826093DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826093E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826093E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826093E8: 386A6134  addi r3, r10, 0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + 24884;
	// 826093EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826093F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826093F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826093F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826093FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260940C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609410: 4BE5DA11  bl 0x82466e20
	ctx.lr = 0x82609414;
	sub_82466E20(ctx, base);
	// 82609414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260941C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609428 size=108
    let mut pc: u32 = 0x82609428;
    'dispatch: loop {
        match pc {
            0x82609428 => {
    //   block [0x82609428..0x82609494)
	// 82609428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260942C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609434: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260943C: 38EBE978  addi r7, r11, -0x1688
	ctx.r[7].s64 = ctx.r[11].s64 + -5768;
	// 82609440: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82609444: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82609448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260944C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609458: 386A6164  addi r3, r10, 0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + 24932;
	// 8260945C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260946C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260947C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609480: 4BE5D9A1  bl 0x82466e20
	ctx.lr = 0x82609484;
	sub_82466E20(ctx, base);
	// 82609484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609498 size=108
    let mut pc: u32 = 0x82609498;
    'dispatch: loop {
        match pc {
            0x82609498 => {
    //   block [0x82609498..0x82609504)
	// 82609498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826094A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826094A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826094A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826094AC: 38EBEA50  addi r7, r11, -0x15b0
	ctx.r[7].s64 = ctx.r[11].s64 + -5552;
	// 826094B0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826094B4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826094B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826094BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826094C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826094C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826094C8: 386A6194  addi r3, r10, 0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + 24980;
	// 826094CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826094D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826094D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826094D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826094DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826094E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826094E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826094E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826094EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826094F0: 4BE5D931  bl 0x82466e20
	ctx.lr = 0x826094F4;
	sub_82466E20(ctx, base);
	// 826094F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826094F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826094FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609508 size=108
    let mut pc: u32 = 0x82609508;
    'dispatch: loop {
        match pc {
            0x82609508 => {
    //   block [0x82609508..0x82609574)
	// 82609508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260950C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609514: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260951C: 38EBEB10  addi r7, r11, -0x14f0
	ctx.r[7].s64 = ctx.r[11].s64 + -5360;
	// 82609520: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82609524: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82609528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260952C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609538: 386A61C4  addi r3, r10, 0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25028;
	// 8260953C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260954C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260955C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609560: 4BE5D8C1  bl 0x82466e20
	ctx.lr = 0x82609564;
	sub_82466E20(ctx, base);
	// 82609564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609578 size=112
    let mut pc: u32 = 0x82609578;
    'dispatch: loop {
        match pc {
            0x82609578 => {
    //   block [0x82609578..0x826095E8)
	// 82609578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609584: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82609588: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8260958C: 38EAEBB8  addi r7, r10, -0x1448
	ctx.r[7].s64 = ctx.r[10].s64 + -5192;
	// 82609590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609594: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82609598: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8260959C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826095A0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826095A4: 396BD810  addi r11, r11, -0x27f0
	ctx.r[11].s64 = ctx.r[11].s64 + -10224;
	// 826095A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826095AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826095B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826095B4: 386A61F4  addi r3, r10, 0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25076;
	// 826095B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826095BC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826095C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826095C4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826095C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826095CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826095D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826095D4: 4BE5D84D  bl 0x82466e20
	ctx.lr = 0x826095D8;
	sub_82466E20(ctx, base);
	// 826095D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826095DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826095E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826095E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826095E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826095E8 size=108
    let mut pc: u32 = 0x826095E8;
    'dispatch: loop {
        match pc {
            0x826095E8 => {
    //   block [0x826095E8..0x82609654)
	// 826095E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826095EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826095F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826095F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826095F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826095FC: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 82609600: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609604: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82609608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260960C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609618: 386A6224  addi r3, r10, 0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + 25124;
	// 8260961C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260962C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260963C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609640: 4BE5D7E1  bl 0x82466e20
	ctx.lr = 0x82609644;
	sub_82466E20(ctx, base);
	// 82609644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609658 size=108
    let mut pc: u32 = 0x82609658;
    'dispatch: loop {
        match pc {
            0x82609658 => {
    //   block [0x82609658..0x826096C4)
	// 82609658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260965C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609664: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260966C: 38EBED38  addi r7, r11, -0x12c8
	ctx.r[7].s64 = ctx.r[11].s64 + -4808;
	// 82609670: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82609674: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82609678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260967C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609688: 386A6254  addi r3, r10, 0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + 25172;
	// 8260968C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260969C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826096A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826096A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826096A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826096AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826096B0: 4BE5D771  bl 0x82466e20
	ctx.lr = 0x826096B4;
	sub_82466E20(ctx, base);
	// 826096B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826096B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826096BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826096C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826096C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826096C8 size=108
    let mut pc: u32 = 0x826096C8;
    'dispatch: loop {
        match pc {
            0x826096C8 => {
    //   block [0x826096C8..0x82609734)
	// 826096C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826096CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826096D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826096D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826096D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826096DC: 38EBEE40  addi r7, r11, -0x11c0
	ctx.r[7].s64 = ctx.r[11].s64 + -4544;
	// 826096E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826096E4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826096E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826096EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826096F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826096F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826096F8: 386A6284  addi r3, r10, 0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + 25220;
	// 826096FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260970C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260971C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609720: 4BE5D701  bl 0x82466e20
	ctx.lr = 0x82609724;
	sub_82466E20(ctx, base);
	// 82609724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260972C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609738 size=108
    let mut pc: u32 = 0x82609738;
    'dispatch: loop {
        match pc {
            0x82609738 => {
    //   block [0x82609738..0x826097A4)
	// 82609738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260973C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609744: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260974C: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 82609750: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82609754: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82609758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260975C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609768: 386A62B4  addi r3, r10, 0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25268;
	// 8260976C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260977C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260978C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609790: 4BE5D691  bl 0x82466e20
	ctx.lr = 0x82609794;
	sub_82466E20(ctx, base);
	// 82609794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260979C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826097A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826097A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826097A8 size=108
    let mut pc: u32 = 0x826097A8;
    'dispatch: loop {
        match pc {
            0x826097A8 => {
    //   block [0x826097A8..0x82609814)
	// 826097A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826097AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826097B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826097B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826097B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826097BC: 38EBEF60  addi r7, r11, -0x10a0
	ctx.r[7].s64 = ctx.r[11].s64 + -4256;
	// 826097C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826097C4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826097C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826097CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826097D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826097D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826097D8: 386A62E4  addi r3, r10, 0x62e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25316;
	// 826097DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826097E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826097E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826097E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826097EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826097F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826097F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826097F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826097FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609800: 4BE5D621  bl 0x82466e20
	ctx.lr = 0x82609804;
	sub_82466E20(ctx, base);
	// 82609804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260980C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609818 size=108
    let mut pc: u32 = 0x82609818;
    'dispatch: loop {
        match pc {
            0x82609818 => {
    //   block [0x82609818..0x82609884)
	// 82609818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609824: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260982C: 38EBEF78  addi r7, r11, -0x1088
	ctx.r[7].s64 = ctx.r[11].s64 + -4232;
	// 82609830: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82609834: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82609838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260983C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609848: 386A6314  addi r3, r10, 0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + 25364;
	// 8260984C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260985C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260986C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609870: 4BE5D5B1  bl 0x82466e20
	ctx.lr = 0x82609874;
	sub_82466E20(ctx, base);
	// 82609874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609888 size=108
    let mut pc: u32 = 0x82609888;
    'dispatch: loop {
        match pc {
            0x82609888 => {
    //   block [0x82609888..0x826098F4)
	// 82609888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609894: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260989C: 38EBEFC0  addi r7, r11, -0x1040
	ctx.r[7].s64 = ctx.r[11].s64 + -4160;
	// 826098A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826098A4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826098A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826098AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826098B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826098B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826098B8: 386A6344  addi r3, r10, 0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + 25412;
	// 826098BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826098C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826098C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826098C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826098CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826098D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826098D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826098D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826098DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826098E0: 4BE5D541  bl 0x82466e20
	ctx.lr = 0x826098E4;
	sub_82466E20(ctx, base);
	// 826098E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826098E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826098EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826098F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826098F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826098F8 size=112
    let mut pc: u32 = 0x826098F8;
    'dispatch: loop {
        match pc {
            0x826098F8 => {
    //   block [0x826098F8..0x82609968)
	// 826098F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826098FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260990C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609914: 390BEFD8  addi r8, r11, -0x1028
	ctx.r[8].s64 = ctx.r[11].s64 + -4136;
	// 82609918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260991C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82609920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260992C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609930: 386A6374  addi r3, r10, 0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + 25460;
	// 82609934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260993C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260994C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609954: 4BE5D4CD  bl 0x82466e20
	ctx.lr = 0x82609958;
	sub_82466E20(ctx, base);
	// 82609958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260995C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609968 size=108
    let mut pc: u32 = 0x82609968;
    'dispatch: loop {
        match pc {
            0x82609968 => {
    //   block [0x82609968..0x826099D4)
	// 82609968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260996C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260997C: 38EBF020  addi r7, r11, -0xfe0
	ctx.r[7].s64 = ctx.r[11].s64 + -4064;
	// 82609980: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609984: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82609988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260998C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609998: 386A63A4  addi r3, r10, 0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + 25508;
	// 8260999C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826099A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826099A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826099A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826099AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826099B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826099B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826099B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826099BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826099C0: 4BE5D461  bl 0x82466e20
	ctx.lr = 0x826099C4;
	sub_82466E20(ctx, base);
	// 826099C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826099C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826099CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826099D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826099D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826099D8 size=116
    let mut pc: u32 = 0x826099D8;
    'dispatch: loop {
        match pc {
            0x826099D8 => {
    //   block [0x826099D8..0x82609A4C)
	// 826099D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826099DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826099E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826099E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826099E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826099EC: 390BF080  addi r8, r11, -0xf80
	ctx.r[8].s64 = ctx.r[11].s64 + -3968;
	// 826099F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826099F4: 392AD888  addi r9, r10, -0x2778
	ctx.r[9].s64 = ctx.r[10].s64 + -10104;
	// 826099F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826099FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82609A00: 38AA63A4  addi r5, r10, 0x63a4
	ctx.r[5].s64 = ctx.r[10].s64 + 25508;
	// 82609A04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609A0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609A1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82609A20: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82609A24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82609A28: 386B63D4  addi r3, r11, 0x63d4
	ctx.r[3].s64 = ctx.r[11].s64 + 25556;
	// 82609A2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609A38: 4BE5D3E9  bl 0x82466e20
	ctx.lr = 0x82609A3C;
	sub_82466E20(ctx, base);
	// 82609A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609A50 size=96
    let mut pc: u32 = 0x82609A50;
    'dispatch: loop {
        match pc {
            0x82609A50 => {
    //   block [0x82609A50..0x82609AB0)
	// 82609A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609A5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609A64: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82609A68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609A70: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 82609A74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609A7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609A90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609A98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609A9C: 4BE5D385  bl 0x82466e20
	ctx.lr = 0x82609AA0;
	sub_82466E20(ctx, base);
	// 82609AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609AB0 size=112
    let mut pc: u32 = 0x82609AB0;
    'dispatch: loop {
        match pc {
            0x82609AB0 => {
    //   block [0x82609AB0..0x82609B20)
	// 82609AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82609AC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609AC4: 38AA8414  addi r5, r10, -0x7bec
	ctx.r[5].s64 = ctx.r[10].s64 + -31724;
	// 82609AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609ACC: 390BF0F8  addi r8, r11, -0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + -3848;
	// 82609AD0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82609AD4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82609AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609AE8: 386A6434  addi r3, r10, 0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + 25652;
	// 82609AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609B0C: 4BE5D315  bl 0x82466e20
	ctx.lr = 0x82609B10;
	sub_82466E20(ctx, base);
	// 82609B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609B20 size=96
    let mut pc: u32 = 0x82609B20;
    'dispatch: loop {
        match pc {
            0x82609B20 => {
    //   block [0x82609B20..0x82609B80)
	// 82609B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609B2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609B34: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82609B38: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609B40: 386A6464  addi r3, r10, 0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + 25700;
	// 82609B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609B4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609B6C: 4BE5D2B5  bl 0x82466e20
	ctx.lr = 0x82609B70;
	sub_82466E20(ctx, base);
	// 82609B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82609B80 size=24
    let mut pc: u32 = 0x82609B80;
    'dispatch: loop {
        match pc {
            0x82609B80 => {
    //   block [0x82609B80..0x82609B98)
	// 82609B80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609B84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82609B88: 394A3710  addi r10, r10, 0x3710
	ctx.r[10].s64 = ctx.r[10].s64 + 14096;
	// 82609B8C: 816BF158  lwz r11, -0xea8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3752 as u32) ) } as u64;
	// 82609B90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82609B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609B98 size=116
    let mut pc: u32 = 0x82609B98;
    'dispatch: loop {
        match pc {
            0x82609B98 => {
    //   block [0x82609B98..0x82609C0C)
	// 82609B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609BA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82609BAC: 390B3710  addi r8, r11, 0x3710
	ctx.r[8].s64 = ctx.r[11].s64 + 14096;
	// 82609BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609BB4: 392AD8C4  addi r9, r10, -0x273c
	ctx.r[9].s64 = ctx.r[10].s64 + -10044;
	// 82609BB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609BBC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82609BC0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609BC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609BD4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609BDC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82609BE0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82609BE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82609BE8: 386B6494  addi r3, r11, 0x6494
	ctx.r[3].s64 = ctx.r[11].s64 + 25748;
	// 82609BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609BF8: 4BE5D229  bl 0x82466e20
	ctx.lr = 0x82609BFC;
	sub_82466E20(ctx, base);
	// 82609BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609C10 size=104
    let mut pc: u32 = 0x82609C10;
    'dispatch: loop {
        match pc {
            0x82609C10 => {
    //   block [0x82609C10..0x82609C78)
	// 82609C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82609C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609C24: 392AD8F0  addi r9, r10, -0x2710
	ctx.r[9].s64 = ctx.r[10].s64 + -10000;
	// 82609C28: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609C30: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609C34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609C44: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82609C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609C4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609C5C: 386A64C4  addi r3, r10, 0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25796;
	// 82609C60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609C64: 4BE5D1BD  bl 0x82466e20
	ctx.lr = 0x82609C68;
	sub_82466E20(ctx, base);
	// 82609C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609C78 size=96
    let mut pc: u32 = 0x82609C78;
    'dispatch: loop {
        match pc {
            0x82609C78 => {
    //   block [0x82609C78..0x82609CD8)
	// 82609C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609C8C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82609C90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609C98: 386A64F4  addi r3, r10, 0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25844;
	// 82609C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609CA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609CB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609CC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609CC4: 4BE5D15D  bl 0x82466e20
	ctx.lr = 0x82609CC8;
	sub_82466E20(ctx, base);
	// 82609CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609CD8 size=100
    let mut pc: u32 = 0x82609CD8;
    'dispatch: loop {
        match pc {
            0x82609CD8 => {
    //   block [0x82609CD8..0x82609D3C)
	// 82609CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609CE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609CEC: 38AA64C4  addi r5, r10, 0x64c4
	ctx.r[5].s64 = ctx.r[10].s64 + 25796;
	// 82609CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609CF8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82609CFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609D0C: 386A6524  addi r3, r10, 0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + 25892;
	// 82609D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609D28: 4BE5D0F9  bl 0x82466e20
	ctx.lr = 0x82609D2C;
	sub_82466E20(ctx, base);
	// 82609D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609D40 size=112
    let mut pc: u32 = 0x82609D40;
    'dispatch: loop {
        match pc {
            0x82609D40 => {
    //   block [0x82609D40..0x82609DB0)
	// 82609D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609D4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609D54: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609D5C: 390BF160  addi r8, r11, -0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + -3744;
	// 82609D60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82609D64: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82609D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609D78: 386A6554  addi r3, r10, 0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + 25940;
	// 82609D7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609D80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609D9C: 4BE5D085  bl 0x82466e20
	ctx.lr = 0x82609DA0;
	sub_82466E20(ctx, base);
	// 82609DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609DB0 size=112
    let mut pc: u32 = 0x82609DB0;
    'dispatch: loop {
        match pc {
            0x82609DB0 => {
    //   block [0x82609DB0..0x82609E20)
	// 82609DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609DC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609DC4: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609DCC: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 82609DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82609DD4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82609DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609DDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609DE8: 386A6584  addi r3, r10, 0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + 25988;
	// 82609DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609E0C: 4BE5D015  bl 0x82466e20
	ctx.lr = 0x82609E10;
	sub_82466E20(ctx, base);
	// 82609E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609E20 size=100
    let mut pc: u32 = 0x82609E20;
    'dispatch: loop {
        match pc {
            0x82609E20 => {
    //   block [0x82609E20..0x82609E84)
	// 82609E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609E34: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609E40: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82609E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609E54: 386A65B4  addi r3, r10, 0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26036;
	// 82609E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609E70: 4BE5CFB1  bl 0x82466e20
	ctx.lr = 0x82609E74;
	sub_82466E20(ctx, base);
	// 82609E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609E88 size=96
    let mut pc: u32 = 0x82609E88;
    'dispatch: loop {
        match pc {
            0x82609E88 => {
    //   block [0x82609E88..0x82609EE8)
	// 82609E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609E94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609E9C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82609EA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609EA8: 386A65E4  addi r3, r10, 0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26084;
	// 82609EAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609EB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609EC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609ED0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609ED4: 4BE5CF4D  bl 0x82466e20
	ctx.lr = 0x82609ED8;
	sub_82466E20(ctx, base);
	// 82609ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609EE8 size=112
    let mut pc: u32 = 0x82609EE8;
    'dispatch: loop {
        match pc {
            0x82609EE8 => {
    //   block [0x82609EE8..0x82609F58)
	// 82609EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609EF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609EF8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609EFC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609F04: 390BF1C0  addi r8, r11, -0xe40
	ctx.r[8].s64 = ctx.r[11].s64 + -3648;
	// 82609F08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82609F0C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82609F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609F14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609F20: 386A6614  addi r3, r10, 0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + 26132;
	// 82609F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609F44: 4BE5CEDD  bl 0x82466e20
	ctx.lr = 0x82609F48;
	sub_82466E20(ctx, base);
	// 82609F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609F58 size=96
    let mut pc: u32 = 0x82609F58;
    'dispatch: loop {
        match pc {
            0x82609F58 => {
    //   block [0x82609F58..0x82609FB8)
	// 82609F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609F64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609F6C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82609F70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609F78: 386A6644  addi r3, r10, 0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + 26180;
	// 82609F7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609F84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609F98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609FA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609FA4: 4BE5CE7D  bl 0x82466e20
	ctx.lr = 0x82609FA8;
	sub_82466E20(ctx, base);
	// 82609FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609FB8 size=112
    let mut pc: u32 = 0x82609FB8;
    'dispatch: loop {
        match pc {
            0x82609FB8 => {
    //   block [0x82609FB8..0x8260A028)
	// 82609FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609FC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609FC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609FCC: 38AA6644  addi r5, r10, 0x6644
	ctx.r[5].s64 = ctx.r[10].s64 + 26180;
	// 82609FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609FD4: 390BF1F0  addi r8, r11, -0xe10
	ctx.r[8].s64 = ctx.r[11].s64 + -3600;
	// 82609FD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82609FDC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 82609FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609FE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609FF0: 386A6674  addi r3, r10, 0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + 26228;
	// 82609FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A014: 4BE5CE0D  bl 0x82466e20
	ctx.lr = 0x8260A018;
	sub_82466E20(ctx, base);
	// 8260A018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A028 size=108
    let mut pc: u32 = 0x8260A028;
    'dispatch: loop {
        match pc {
            0x8260A028 => {
    //   block [0x8260A028..0x8260A094)
	// 8260A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A034: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A03C: 38EBF208  addi r7, r11, -0xdf8
	ctx.r[7].s64 = ctx.r[11].s64 + -3576;
	// 8260A040: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260A044: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8260A048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A04C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A058: 386A66A4  addi r3, r10, 0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26276;
	// 8260A05C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A07C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A080: 4BE5CDA1  bl 0x82466e20
	ctx.lr = 0x8260A084;
	sub_82466E20(ctx, base);
	// 8260A084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A098 size=112
    let mut pc: u32 = 0x8260A098;
    'dispatch: loop {
        match pc {
            0x8260A098 => {
    //   block [0x8260A098..0x8260A108)
	// 8260A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A0A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A0AC: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A0B4: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 8260A0B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260A0BC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8260A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A0C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A0D0: 386A66D4  addi r3, r10, 0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26324;
	// 8260A0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A0F4: 4BE5CD2D  bl 0x82466e20
	ctx.lr = 0x8260A0F8;
	sub_82466E20(ctx, base);
	// 8260A0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A108 size=112
    let mut pc: u32 = 0x8260A108;
    'dispatch: loop {
        match pc {
            0x8260A108 => {
    //   block [0x8260A108..0x8260A178)
	// 8260A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A114: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A11C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A124: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 8260A128: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260A12C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8260A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A140: 386A6704  addi r3, r10, 0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + 26372;
	// 8260A144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A164: 4BE5CCBD  bl 0x82466e20
	ctx.lr = 0x8260A168;
	sub_82466E20(ctx, base);
	// 8260A168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A178 size=100
    let mut pc: u32 = 0x8260A178;
    'dispatch: loop {
        match pc {
            0x8260A178 => {
    //   block [0x8260A178..0x8260A1DC)
	// 8260A178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A184: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A18C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A198: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8260A19C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A1AC: 386A6734  addi r3, r10, 0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + 26420;
	// 8260A1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A1B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A1C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A1C8: 4BE5CC59  bl 0x82466e20
	ctx.lr = 0x8260A1CC;
	sub_82466E20(ctx, base);
	// 8260A1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A1E0 size=116
    let mut pc: u32 = 0x8260A1E0;
    'dispatch: loop {
        match pc {
            0x8260A1E0 => {
    //   block [0x8260A1E0..0x8260A254)
	// 8260A1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A1EC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260A1F4: 390BF2B4  addi r8, r11, -0xd4c
	ctx.r[8].s64 = ctx.r[11].s64 + -3404;
	// 8260A1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A1FC: 392AD91C  addi r9, r10, -0x26e4
	ctx.r[9].s64 = ctx.r[10].s64 + -9956;
	// 8260A200: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A204: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8260A208: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A20C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A224: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260A228: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8260A22C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A230: 386B6764  addi r3, r11, 0x6764
	ctx.r[3].s64 = ctx.r[11].s64 + 26468;
	// 8260A234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A240: 4BE5CBE1  bl 0x82466e20
	ctx.lr = 0x8260A244;
	sub_82466E20(ctx, base);
	// 8260A244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A258 size=112
    let mut pc: u32 = 0x8260A258;
    'dispatch: loop {
        match pc {
            0x8260A258 => {
    //   block [0x8260A258..0x8260A2C8)
	// 8260A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A26C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A274: 390BF2E4  addi r8, r11, -0xd1c
	ctx.r[8].s64 = ctx.r[11].s64 + -3356;
	// 8260A278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260A27C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8260A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A290: 386A6794  addi r3, r10, 0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + 26516;
	// 8260A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A2A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A2B4: 4BE5CB6D  bl 0x82466e20
	ctx.lr = 0x8260A2B8;
	sub_82466E20(ctx, base);
	// 8260A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A2C8 size=116
    let mut pc: u32 = 0x8260A2C8;
    'dispatch: loop {
        match pc {
            0x8260A2C8 => {
    //   block [0x8260A2C8..0x8260A33C)
	// 8260A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A2D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A2D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260A2DC: 390BF300  addi r8, r11, -0xd00
	ctx.r[8].s64 = ctx.r[11].s64 + -3328;
	// 8260A2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A2E4: 392AD948  addi r9, r10, -0x26b8
	ctx.r[9].s64 = ctx.r[10].s64 + -9912;
	// 8260A2E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A2EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260A2F0: 38AA6DF4  addi r5, r10, 0x6df4
	ctx.r[5].s64 = ctx.r[10].s64 + 28148;
	// 8260A2F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A2FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A30C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260A310: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8260A314: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A318: 386B67C4  addi r3, r11, 0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + 26564;
	// 8260A31C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A320: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A328: 4BE5CAF9  bl 0x82466e20
	ctx.lr = 0x8260A32C;
	sub_82466E20(ctx, base);
	// 8260A32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A340 size=112
    let mut pc: u32 = 0x8260A340;
    'dispatch: loop {
        match pc {
            0x8260A340 => {
    //   block [0x8260A340..0x8260A3B0)
	// 8260A340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A34C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A350: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A354: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A35C: 390BF318  addi r8, r11, -0xce8
	ctx.r[8].s64 = ctx.r[11].s64 + -3304;
	// 8260A360: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260A364: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8260A368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A36C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A378: 386A67F4  addi r3, r10, 0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26612;
	// 8260A37C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A38C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260A390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A39C: 4BE5CA85  bl 0x82466e20
	ctx.lr = 0x8260A3A0;
	sub_82466E20(ctx, base);
	// 8260A3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A3B0 size=112
    let mut pc: u32 = 0x8260A3B0;
    'dispatch: loop {
        match pc {
            0x8260A3B0 => {
    //   block [0x8260A3B0..0x8260A420)
	// 8260A3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A3BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A3C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A3C4: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A3CC: 390BF390  addi r8, r11, -0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + -3184;
	// 8260A3D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A3D4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8260A3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A3E8: 386A6824  addi r3, r10, 0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + 26660;
	// 8260A3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A40C: 4BE5CA15  bl 0x82466e20
	ctx.lr = 0x8260A410;
	sub_82466E20(ctx, base);
	// 8260A410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A420 size=112
    let mut pc: u32 = 0x8260A420;
    'dispatch: loop {
        match pc {
            0x8260A420 => {
    //   block [0x8260A420..0x8260A490)
	// 8260A420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A430: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A434: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A43C: 390BF3D8  addi r8, r11, -0xc28
	ctx.r[8].s64 = ctx.r[11].s64 + -3112;
	// 8260A440: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A444: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8260A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A44C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A458: 386A6854  addi r3, r10, 0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + 26708;
	// 8260A45C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A47C: 4BE5C9A5  bl 0x82466e20
	ctx.lr = 0x8260A480;
	sub_82466E20(ctx, base);
	// 8260A480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A490 size=112
    let mut pc: u32 = 0x8260A490;
    'dispatch: loop {
        match pc {
            0x8260A490 => {
    //   block [0x8260A490..0x8260A500)
	// 8260A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A49C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A4A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A4A4: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A4AC: 390BF420  addi r8, r11, -0xbe0
	ctx.r[8].s64 = ctx.r[11].s64 + -3040;
	// 8260A4B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A4B4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8260A4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A4BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A4C8: 386A6884  addi r3, r10, 0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + 26756;
	// 8260A4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A4EC: 4BE5C935  bl 0x82466e20
	ctx.lr = 0x8260A4F0;
	sub_82466E20(ctx, base);
	// 8260A4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A500 size=108
    let mut pc: u32 = 0x8260A500;
    'dispatch: loop {
        match pc {
            0x8260A500 => {
    //   block [0x8260A500..0x8260A56C)
	// 8260A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A50C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A514: 38EBF468  addi r7, r11, -0xb98
	ctx.r[7].s64 = ctx.r[11].s64 + -2968;
	// 8260A518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260A51C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8260A520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A530: 386A68B4  addi r3, r10, 0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26804;
	// 8260A534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A558: 4BE5C8C9  bl 0x82466e20
	ctx.lr = 0x8260A55C;
	sub_82466E20(ctx, base);
	// 8260A55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A570 size=112
    let mut pc: u32 = 0x8260A570;
    'dispatch: loop {
        match pc {
            0x8260A570 => {
    //   block [0x8260A570..0x8260A5E0)
	// 8260A570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A580: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A584: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A58C: 390BF4B0  addi r8, r11, -0xb50
	ctx.r[8].s64 = ctx.r[11].s64 + -2896;
	// 8260A590: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260A594: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8260A598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A59C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A5A8: 386A68E4  addi r3, r10, 0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26852;
	// 8260A5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A5CC: 4BE5C855  bl 0x82466e20
	ctx.lr = 0x8260A5D0;
	sub_82466E20(ctx, base);
	// 8260A5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A5E0 size=116
    let mut pc: u32 = 0x8260A5E0;
    'dispatch: loop {
        match pc {
            0x8260A5E0 => {
    //   block [0x8260A5E0..0x8260A654)
	// 8260A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A5EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A5F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A5F4: 392BD984  addi r9, r11, -0x267c
	ctx.r[9].s64 = ctx.r[11].s64 + -9852;
	// 8260A5F8: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A5FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A600: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260A604: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8260A608: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A60C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8260A610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A614: 396BF530  addi r11, r11, -0xad0
	ctx.r[11].s64 = ctx.r[11].s64 + -2768;
	// 8260A618: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260A61C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A620: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260A624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A628: 386A6914  addi r3, r10, 0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + 26900;
	// 8260A62C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A630: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A638: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260A63C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A640: 4BE5C7E1  bl 0x82466e20
	ctx.lr = 0x8260A644;
	sub_82466E20(ctx, base);
	// 8260A644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A658 size=36
    let mut pc: u32 = 0x8260A658;
    'dispatch: loop {
        match pc {
            0x8260A658 => {
    //   block [0x8260A658..0x8260A67C)
	// 8260A658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A65C: 814BF5C4  lwz r10, -0xa3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2620 as u32) ) } as u64;
	// 8260A660: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A664: 396B3740  addi r11, r11, 0x3740
	ctx.r[11].s64 = ctx.r[11].s64 + 14144;
	// 8260A668: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8260A66C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A670: 814AF52C  lwz r10, -0xad4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2772 as u32) ) } as u64;
	// 8260A674: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8260A678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A680 size=108
    let mut pc: u32 = 0x8260A680;
    'dispatch: loop {
        match pc {
            0x8260A680 => {
    //   block [0x8260A680..0x8260A6EC)
	// 8260A680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A68C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A694: 38EB3740  addi r7, r11, 0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + 14144;
	// 8260A698: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8260A69C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8260A6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A6B0: 386A6944  addi r3, r10, 0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + 26948;
	// 8260A6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A6D8: 4BE5C749  bl 0x82466e20
	ctx.lr = 0x8260A6DC;
	sub_82466E20(ctx, base);
	// 8260A6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A6F0 size=24
    let mut pc: u32 = 0x8260A6F0;
    'dispatch: loop {
        match pc {
            0x8260A6F0 => {
    //   block [0x8260A6F0..0x8260A708)
	// 8260A6F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A6F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A6F8: 394A37E8  addi r10, r10, 0x37e8
	ctx.r[10].s64 = ctx.r[10].s64 + 14312;
	// 8260A6FC: 816BF52C  lwz r11, -0xad4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2772 as u32) ) } as u64;
	// 8260A700: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260A704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A708 size=116
    let mut pc: u32 = 0x8260A708;
    'dispatch: loop {
        match pc {
            0x8260A708 => {
    //   block [0x8260A708..0x8260A77C)
	// 8260A708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A714: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A718: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260A71C: 390A37E8  addi r8, r10, 0x37e8
	ctx.r[8].s64 = ctx.r[10].s64 + 14312;
	// 8260A720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A724: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A728: 38AA6944  addi r5, r10, 0x6944
	ctx.r[5].s64 = ctx.r[10].s64 + 26948;
	// 8260A72C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A730: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A73C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8260A740: 396BDA40  addi r11, r11, -0x25c0
	ctx.r[11].s64 = ctx.r[11].s64 + -9664;
	// 8260A744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A74C: 386A6974  addi r3, r10, 0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + 26996;
	// 8260A750: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260A754: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A758: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260A75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A768: 4BE5C6B9  bl 0x82466e20
	ctx.lr = 0x8260A76C;
	sub_82466E20(ctx, base);
	// 8260A76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A780 size=112
    let mut pc: u32 = 0x8260A780;
    'dispatch: loop {
        match pc {
            0x8260A780 => {
    //   block [0x8260A780..0x8260A7F0)
	// 8260A780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A78C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A790: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A794: 38AA6944  addi r5, r10, 0x6944
	ctx.r[5].s64 = ctx.r[10].s64 + 26948;
	// 8260A798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A79C: 390BF5C8  addi r8, r11, -0xa38
	ctx.r[8].s64 = ctx.r[11].s64 + -2616;
	// 8260A7A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260A7A4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8260A7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A7AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A7B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A7B8: 386A69A4  addi r3, r10, 0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + 27044;
	// 8260A7BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A7DC: 4BE5C645  bl 0x82466e20
	ctx.lr = 0x8260A7E0;
	sub_82466E20(ctx, base);
	// 8260A7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A7F0 size=24
    let mut pc: u32 = 0x8260A7F0;
    'dispatch: loop {
        match pc {
            0x8260A7F0 => {
    //   block [0x8260A7F0..0x8260A808)
	// 8260A7F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A7F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A7F8: 394A38A8  addi r10, r10, 0x38a8
	ctx.r[10].s64 = ctx.r[10].s64 + 14504;
	// 8260A7FC: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260A800: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260A804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A808 size=116
    let mut pc: u32 = 0x8260A808;
    'dispatch: loop {
        match pc {
            0x8260A808 => {
    //   block [0x8260A808..0x8260A87C)
	// 8260A808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A818: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A81C: 392BDA04  addi r9, r11, -0x25fc
	ctx.r[9].s64 = ctx.r[11].s64 + -9724;
	// 8260A820: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A824: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A828: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8260A82C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260A830: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A834: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8260A838: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A83C: 396B38A8  addi r11, r11, 0x38a8
	ctx.r[11].s64 = ctx.r[11].s64 + 14504;
	// 8260A840: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260A844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A848: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260A84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A850: 386A69D4  addi r3, r10, 0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + 27092;
	// 8260A854: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260A858: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260A85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A860: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260A864: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A868: 4BE5C5B9  bl 0x82466e20
	ctx.lr = 0x8260A86C;
	sub_82466E20(ctx, base);
	// 8260A86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A880 size=100
    let mut pc: u32 = 0x8260A880;
    'dispatch: loop {
        match pc {
            0x8260A880 => {
    //   block [0x8260A880..0x8260A8E4)
	// 8260A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A88C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A894: 38AA6B24  addi r5, r10, 0x6b24
	ctx.r[5].s64 = ctx.r[10].s64 + 27428;
	// 8260A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A8A0: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8260A8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A8B4: 386A6A04  addi r3, r10, 0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + 27140;
	// 8260A8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A8BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A8C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A8C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A8D0: 4BE5C551  bl 0x82466e20
	ctx.lr = 0x8260A8D4;
	sub_82466E20(ctx, base);
	// 8260A8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A8E8 size=100
    let mut pc: u32 = 0x8260A8E8;
    'dispatch: loop {
        match pc {
            0x8260A8E8 => {
    //   block [0x8260A8E8..0x8260A94C)
	// 8260A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A8F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A8FC: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A908: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8260A90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A91C: 386A6A34  addi r3, r10, 0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + 27188;
	// 8260A920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A938: 4BE5C4E9  bl 0x82466e20
	ctx.lr = 0x8260A93C;
	sub_82466E20(ctx, base);
	// 8260A93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A950 size=108
    let mut pc: u32 = 0x8260A950;
    'dispatch: loop {
        match pc {
            0x8260A950 => {
    //   block [0x8260A950..0x8260A9BC)
	// 8260A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A95C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A964: 38EBF628  addi r7, r11, -0x9d8
	ctx.r[7].s64 = ctx.r[11].s64 + -2520;
	// 8260A968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260A96C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8260A970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A974: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A980: 386A6A64  addi r3, r10, 0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + 27236;
	// 8260A984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A9A8: 4BE5C479  bl 0x82466e20
	ctx.lr = 0x8260A9AC;
	sub_82466E20(ctx, base);
	// 8260A9AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A9C0 size=112
    let mut pc: u32 = 0x8260A9C0;
    'dispatch: loop {
        match pc {
            0x8260A9C0 => {
    //   block [0x8260A9C0..0x8260AA30)
	// 8260A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A9D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A9D4: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A9DC: 390BF688  addi r8, r11, -0x978
	ctx.r[8].s64 = ctx.r[11].s64 + -2424;
	// 8260A9E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A9E4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8260A9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A9F8: 386A6A94  addi r3, r10, 0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + 27284;
	// 8260A9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AA1C: 4BE5C405  bl 0x82466e20
	ctx.lr = 0x8260AA20;
	sub_82466E20(ctx, base);
	// 8260AA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AA30 size=108
    let mut pc: u32 = 0x8260AA30;
    'dispatch: loop {
        match pc {
            0x8260AA30 => {
    //   block [0x8260AA30..0x8260AA9C)
	// 8260AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AA3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AA44: 38EBF6D0  addi r7, r11, -0x930
	ctx.r[7].s64 = ctx.r[11].s64 + -2352;
	// 8260AA48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260AA4C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8260AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AA54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AA60: 386A6AC4  addi r3, r10, 0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 27332;
	// 8260AA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AA88: 4BE5C399  bl 0x82466e20
	ctx.lr = 0x8260AA8C;
	sub_82466E20(ctx, base);
	// 8260AA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AAA0 size=28
    let mut pc: u32 = 0x8260AAA0;
    'dispatch: loop {
        match pc {
            0x8260AAA0 => {
    //   block [0x8260AAA0..0x8260AABC)
	// 8260AAA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AAA4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AAA8: 394A3950  addi r10, r10, 0x3950
	ctx.r[10].s64 = ctx.r[10].s64 + 14672;
	// 8260AAAC: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 8260AAB0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260AAB4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AAC0 size=112
    let mut pc: u32 = 0x8260AAC0;
    'dispatch: loop {
        match pc {
            0x8260AAC0 => {
    //   block [0x8260AAC0..0x8260AB30)
	// 8260AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AACC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AAD0: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8260AAD4: 38EA3950  addi r7, r10, 0x3950
	ctx.r[7].s64 = ctx.r[10].s64 + 14672;
	// 8260AAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AADC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AAE0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8260AAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AAE8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AAEC: 396BDAEC  addi r11, r11, -0x2514
	ctx.r[11].s64 = ctx.r[11].s64 + -9492;
	// 8260AAF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AAF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AAF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AAFC: 386A6AF4  addi r3, r10, 0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + 27380;
	// 8260AB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AB04: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260AB08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AB0C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260AB10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AB14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AB18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AB1C: 4BE5C305  bl 0x82466e20
	ctx.lr = 0x8260AB20;
	sub_82466E20(ctx, base);
	// 8260AB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AB30 size=24
    let mut pc: u32 = 0x8260AB30;
    'dispatch: loop {
        match pc {
            0x8260AB30 => {
    //   block [0x8260AB30..0x8260AB48)
	// 8260AB30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AB34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AB38: 394A3AA0  addi r10, r10, 0x3aa0
	ctx.r[10].s64 = ctx.r[10].s64 + 15008;
	// 8260AB3C: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260AB40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AB48 size=116
    let mut pc: u32 = 0x8260AB48;
    'dispatch: loop {
        match pc {
            0x8260AB48 => {
    //   block [0x8260AB48..0x8260ABBC)
	// 8260AB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AB54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AB58: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AB5C: 392BDAC4  addi r9, r11, -0x253c
	ctx.r[9].s64 = ctx.r[11].s64 + -9532;
	// 8260AB60: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260AB64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AB68: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8260AB6C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260AB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AB74: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8260AB78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AB7C: 396B3AA0  addi r11, r11, 0x3aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 15008;
	// 8260AB80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260AB84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AB88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260AB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AB90: 386A6B24  addi r3, r10, 0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + 27428;
	// 8260AB94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8260AB98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260AB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ABA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260ABA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260ABA8: 4BE5C279  bl 0x82466e20
	ctx.lr = 0x8260ABAC;
	sub_82466E20(ctx, base);
	// 8260ABAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ABB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ABB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ABB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ABC0 size=112
    let mut pc: u32 = 0x8260ABC0;
    'dispatch: loop {
        match pc {
            0x8260ABC0 => {
    //   block [0x8260ABC0..0x8260AC30)
	// 8260ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ABC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ABCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ABD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ABD4: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260ABD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ABDC: 390BF6F0  addi r8, r11, -0x910
	ctx.r[8].s64 = ctx.r[11].s64 + -2320;
	// 8260ABE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ABE4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8260ABE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ABEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ABF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ABF8: 386A6B54  addi r3, r10, 0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + 27476;
	// 8260ABFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AC1C: 4BE5C205  bl 0x82466e20
	ctx.lr = 0x8260AC20;
	sub_82466E20(ctx, base);
	// 8260AC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AC30 size=24
    let mut pc: u32 = 0x8260AC30;
    'dispatch: loop {
        match pc {
            0x8260AC30 => {
    //   block [0x8260AC30..0x8260AC48)
	// 8260AC30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AC34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AC38: 394A3B48  addi r10, r10, 0x3b48
	ctx.r[10].s64 = ctx.r[10].s64 + 15176;
	// 8260AC3C: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260AC40: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8260AC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AC48 size=116
    let mut pc: u32 = 0x8260AC48;
    'dispatch: loop {
        match pc {
            0x8260AC48 => {
    //   block [0x8260AC48..0x8260ACBC)
	// 8260AC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AC54: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AC58: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8260AC5C: 390A3B48  addi r8, r10, 0x3b48
	ctx.r[8].s64 = ctx.r[10].s64 + 15176;
	// 8260AC60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AC64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AC68: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260AC6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AC70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260AC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AC7C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8260AC80: 396BDB48  addi r11, r11, -0x24b8
	ctx.r[11].s64 = ctx.r[11].s64 + -9400;
	// 8260AC84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AC88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260AC8C: 386A6B84  addi r3, r10, 0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + 27524;
	// 8260AC90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260AC94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AC98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ACA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ACA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ACA8: 4BE5C179  bl 0x82466e20
	ctx.lr = 0x8260ACAC;
	sub_82466E20(ctx, base);
	// 8260ACAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ACB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ACB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ACB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ACC0 size=112
    let mut pc: u32 = 0x8260ACC0;
    'dispatch: loop {
        match pc {
            0x8260ACC0 => {
    //   block [0x8260ACC0..0x8260AD30)
	// 8260ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ACCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ACD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ACD4: 38AA8504  addi r5, r10, -0x7afc
	ctx.r[5].s64 = ctx.r[10].s64 + -31484;
	// 8260ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ACDC: 390BF720  addi r8, r11, -0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + -2272;
	// 8260ACE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ACE4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8260ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ACEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ACF8: 386A6BB4  addi r3, r10, 0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + 27572;
	// 8260ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AD1C: 4BE5C105  bl 0x82466e20
	ctx.lr = 0x8260AD20;
	sub_82466E20(ctx, base);
	// 8260AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AD30 size=108
    let mut pc: u32 = 0x8260AD30;
    'dispatch: loop {
        match pc {
            0x8260AD30 => {
    //   block [0x8260AD30..0x8260AD9C)
	// 8260AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AD3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AD44: 38EBF750  addi r7, r11, -0x8b0
	ctx.r[7].s64 = ctx.r[11].s64 + -2224;
	// 8260AD48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260AD4C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8260AD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AD54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AD60: 386A6BE4  addi r3, r10, 0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + 27620;
	// 8260AD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AD88: 4BE5C099  bl 0x82466e20
	ctx.lr = 0x8260AD8C;
	sub_82466E20(ctx, base);
	// 8260AD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ADA0 size=112
    let mut pc: u32 = 0x8260ADA0;
    'dispatch: loop {
        match pc {
            0x8260ADA0 => {
    //   block [0x8260ADA0..0x8260AE10)
	// 8260ADA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ADA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ADA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ADAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ADB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ADB4: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ADBC: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 8260ADC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ADC4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8260ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ADCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ADD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ADD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ADD8: 386A6C14  addi r3, r10, 0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + 27668;
	// 8260ADDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ADE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ADEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ADF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ADFC: 4BE5C025  bl 0x82466e20
	ctx.lr = 0x8260AE00;
	sub_82466E20(ctx, base);
	// 8260AE00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AE04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AE08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AE10 size=112
    let mut pc: u32 = 0x8260AE10;
    'dispatch: loop {
        match pc {
            0x8260AE10 => {
    //   block [0x8260AE10..0x8260AE80)
	// 8260AE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AE1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AE20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AE24: 38AA6DF4  addi r5, r10, 0x6df4
	ctx.r[5].s64 = ctx.r[10].s64 + 28148;
	// 8260AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AE2C: 390BF7B0  addi r8, r11, -0x850
	ctx.r[8].s64 = ctx.r[11].s64 + -2128;
	// 8260AE30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260AE34: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8260AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AE3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AE40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AE48: 386A6C44  addi r3, r10, 0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + 27716;
	// 8260AE4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AE6C: 4BE5BFB5  bl 0x82466e20
	ctx.lr = 0x8260AE70;
	sub_82466E20(ctx, base);
	// 8260AE70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AE80 size=108
    let mut pc: u32 = 0x8260AE80;
    'dispatch: loop {
        match pc {
            0x8260AE80 => {
    //   block [0x8260AE80..0x8260AEEC)
	// 8260AE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AE8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AE94: 38EBF7E0  addi r7, r11, -0x820
	ctx.r[7].s64 = ctx.r[11].s64 + -2080;
	// 8260AE98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260AE9C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8260AEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AEA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AEB0: 386A6C74  addi r3, r10, 0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + 27764;
	// 8260AEB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AED8: 4BE5BF49  bl 0x82466e20
	ctx.lr = 0x8260AEDC;
	sub_82466E20(ctx, base);
	// 8260AEDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AEF0 size=112
    let mut pc: u32 = 0x8260AEF0;
    'dispatch: loop {
        match pc {
            0x8260AEF0 => {
    //   block [0x8260AEF0..0x8260AF60)
	// 8260AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AEFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AF04: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AF0C: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 8260AF10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260AF14: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8260AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AF1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AF28: 386A6CA4  addi r3, r10, 0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 27812;
	// 8260AF2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AF4C: 4BE5BED5  bl 0x82466e20
	ctx.lr = 0x8260AF50;
	sub_82466E20(ctx, base);
	// 8260AF50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AF60 size=100
    let mut pc: u32 = 0x8260AF60;
    'dispatch: loop {
        match pc {
            0x8260AF60 => {
    //   block [0x8260AF60..0x8260AFC4)
	// 8260AF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AF6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AF74: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260AF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AF80: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8260AF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AF94: 386A6CD4  addi r3, r10, 0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27860;
	// 8260AF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AF9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AFA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260AFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AFA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260AFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AFB0: 4BE5BE71  bl 0x82466e20
	ctx.lr = 0x8260AFB4;
	sub_82466E20(ctx, base);
	// 8260AFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AFC8 size=112
    let mut pc: u32 = 0x8260AFC8;
    'dispatch: loop {
        match pc {
            0x8260AFC8 => {
    //   block [0x8260AFC8..0x8260B038)
	// 8260AFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AFD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AFD8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AFDC: 38AA6A34  addi r5, r10, 0x6a34
	ctx.r[5].s64 = ctx.r[10].s64 + 27188;
	// 8260AFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AFE4: 390BF888  addi r8, r11, -0x778
	ctx.r[8].s64 = ctx.r[11].s64 + -1912;
	// 8260AFE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260AFEC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8260AFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AFF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B000: 386A6D04  addi r3, r10, 0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + 27908;
	// 8260B004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B024: 4BE5BDFD  bl 0x82466e20
	ctx.lr = 0x8260B028;
	sub_82466E20(ctx, base);
	// 8260B028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B038 size=112
    let mut pc: u32 = 0x8260B038;
    'dispatch: loop {
        match pc {
            0x8260B038 => {
    //   block [0x8260B038..0x8260B0A8)
	// 8260B038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B044: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B048: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B04C: 38AA6A34  addi r5, r10, 0x6a34
	ctx.r[5].s64 = ctx.r[10].s64 + 27188;
	// 8260B050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B054: 390BF8D0  addi r8, r11, -0x730
	ctx.r[8].s64 = ctx.r[11].s64 + -1840;
	// 8260B058: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260B05C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8260B060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B070: 386A6D34  addi r3, r10, 0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + 27956;
	// 8260B074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B094: 4BE5BD8D  bl 0x82466e20
	ctx.lr = 0x8260B098;
	sub_82466E20(ctx, base);
	// 8260B098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B0A8 size=108
    let mut pc: u32 = 0x8260B0A8;
    'dispatch: loop {
        match pc {
            0x8260B0A8 => {
    //   block [0x8260B0A8..0x8260B114)
	// 8260B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B0B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B0BC: 38EBF978  addi r7, r11, -0x688
	ctx.r[7].s64 = ctx.r[11].s64 + -1672;
	// 8260B0C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260B0C4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8260B0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B0CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260B0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B0D8: 386A6D64  addi r3, r10, 0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + 28004;
	// 8260B0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260B0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B100: 4BE5BD21  bl 0x82466e20
	ctx.lr = 0x8260B104;
	sub_82466E20(ctx, base);
	// 8260B104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B118 size=24
    let mut pc: u32 = 0x8260B118;
    'dispatch: loop {
        match pc {
            0x8260B118 => {
    //   block [0x8260B118..0x8260B130)
	// 8260B118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B11C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B120: 394A3C80  addi r10, r10, 0x3c80
	ctx.r[10].s64 = ctx.r[10].s64 + 15488;
	// 8260B124: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260B128: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260B12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B130 size=116
    let mut pc: u32 = 0x8260B130;
    'dispatch: loop {
        match pc {
            0x8260B130 => {
    //   block [0x8260B130..0x8260B1A4)
	// 8260B130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B13C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B140: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260B144: 390A3C80  addi r8, r10, 0x3c80
	ctx.r[8].s64 = ctx.r[10].s64 + 15488;
	// 8260B148: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B14C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260B150: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260B154: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B158: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B164: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8260B168: 396BDB80  addi r11, r11, -0x2480
	ctx.r[11].s64 = ctx.r[11].s64 + -9344;
	// 8260B16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B174: 386A6D94  addi r3, r10, 0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + 28052;
	// 8260B178: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260B17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B180: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260B184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B190: 4BE5BC91  bl 0x82466e20
	ctx.lr = 0x8260B194;
	sub_82466E20(ctx, base);
	// 8260B194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B1A8 size=100
    let mut pc: u32 = 0x8260B1A8;
    'dispatch: loop {
        match pc {
            0x8260B1A8 => {
    //   block [0x8260B1A8..0x8260B20C)
	// 8260B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B1B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B1BC: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260B1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B1C8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8260B1CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B1DC: 386A6DC4  addi r3, r10, 0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28100;
	// 8260B1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B1E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B1F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B1F8: 4BE5BC29  bl 0x82466e20
	ctx.lr = 0x8260B1FC;
	sub_82466E20(ctx, base);
	// 8260B1FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B210 size=100
    let mut pc: u32 = 0x8260B210;
    'dispatch: loop {
        match pc {
            0x8260B210 => {
    //   block [0x8260B210..0x8260B274)
	// 8260B210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B21C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B224: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260B228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B230: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8260B234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B244: 386A6DF4  addi r3, r10, 0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + 28148;
	// 8260B248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B24C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B260: 4BE5BBC1  bl 0x82466e20
	ctx.lr = 0x8260B264;
	sub_82466E20(ctx, base);
	// 8260B264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B278 size=112
    let mut pc: u32 = 0x8260B278;
    'dispatch: loop {
        match pc {
            0x8260B278 => {
    //   block [0x8260B278..0x8260B2E8)
	// 8260B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B288: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B28C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B294: 390BF9D8  addi r8, r11, -0x628
	ctx.r[8].s64 = ctx.r[11].s64 + -1576;
	// 8260B298: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260B29C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8260B2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B2B0: 386A6E24  addi r3, r10, 0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + 28196;
	// 8260B2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B2D4: 4BE5BB4D  bl 0x82466e20
	ctx.lr = 0x8260B2D8;
	sub_82466E20(ctx, base);
	// 8260B2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B2E8 size=112
    let mut pc: u32 = 0x8260B2E8;
    'dispatch: loop {
        match pc {
            0x8260B2E8 => {
    //   block [0x8260B2E8..0x8260B358)
	// 8260B2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B2F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B2F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B2FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B304: 390BFA68  addi r8, r11, -0x598
	ctx.r[8].s64 = ctx.r[11].s64 + -1432;
	// 8260B308: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260B30C: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8260B310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B314: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B320: 386A6E54  addi r3, r10, 0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + 28244;
	// 8260B324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B344: 4BE5BADD  bl 0x82466e20
	ctx.lr = 0x8260B348;
	sub_82466E20(ctx, base);
	// 8260B348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B358 size=112
    let mut pc: u32 = 0x8260B358;
    'dispatch: loop {
        match pc {
            0x8260B358 => {
    //   block [0x8260B358..0x8260B3C8)
	// 8260B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B364: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B368: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B36C: 38AA69D4  addi r5, r10, 0x69d4
	ctx.r[5].s64 = ctx.r[10].s64 + 27092;
	// 8260B370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B374: 390BFAC8  addi r8, r11, -0x538
	ctx.r[8].s64 = ctx.r[11].s64 + -1336;
	// 8260B378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B37C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8260B380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B390: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 8260B394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B3B4: 4BE5BA6D  bl 0x82466e20
	ctx.lr = 0x8260B3B8;
	sub_82466E20(ctx, base);
	// 8260B3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B3C8 size=112
    let mut pc: u32 = 0x8260B3C8;
    'dispatch: loop {
        match pc {
            0x8260B3C8 => {
    //   block [0x8260B3C8..0x8260B438)
	// 8260B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B3D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B3D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B3DC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B3E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B3E4: 390BFAF8  addi r8, r11, -0x508
	ctx.r[8].s64 = ctx.r[11].s64 + -1288;
	// 8260B3E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260B3EC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8260B3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B3F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B400: 386A6EB4  addi r3, r10, 0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 28340;
	// 8260B404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B424: 4BE5B9FD  bl 0x82466e20
	ctx.lr = 0x8260B428;
	sub_82466E20(ctx, base);
	// 8260B428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B438 size=112
    let mut pc: u32 = 0x8260B438;
    'dispatch: loop {
        match pc {
            0x8260B438 => {
    //   block [0x8260B438..0x8260B4A8)
	// 8260B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B444: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B448: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B44C: 38AA6B24  addi r5, r10, 0x6b24
	ctx.r[5].s64 = ctx.r[10].s64 + 27428;
	// 8260B450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B454: 390BFB88  addi r8, r11, -0x478
	ctx.r[8].s64 = ctx.r[11].s64 + -1144;
	// 8260B458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260B45C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8260B460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B470: 386A6EE4  addi r3, r10, 0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 28388;
	// 8260B474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B47C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B494: 4BE5B98D  bl 0x82466e20
	ctx.lr = 0x8260B498;
	sub_82466E20(ctx, base);
	// 8260B498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B4A8 size=112
    let mut pc: u32 = 0x8260B4A8;
    'dispatch: loop {
        match pc {
            0x8260B4A8 => {
    //   block [0x8260B4A8..0x8260B518)
	// 8260B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B4B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B4B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B4BC: 38AA6D34  addi r5, r10, 0x6d34
	ctx.r[5].s64 = ctx.r[10].s64 + 27956;
	// 8260B4C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B4C4: 390BFBA0  addi r8, r11, -0x460
	ctx.r[8].s64 = ctx.r[11].s64 + -1120;
	// 8260B4C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B4CC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8260B4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B4D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B4D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B4E0: 386A6F14  addi r3, r10, 0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + 28436;
	// 8260B4E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B4E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B4F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B4F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B4FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B504: 4BE5B91D  bl 0x82466e20
	ctx.lr = 0x8260B508;
	sub_82466E20(ctx, base);
	// 8260B508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B518 size=112
    let mut pc: u32 = 0x8260B518;
    'dispatch: loop {
        match pc {
            0x8260B518 => {
    //   block [0x8260B518..0x8260B588)
	// 8260B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B528: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B52C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260B530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B534: 390BFBD0  addi r8, r11, -0x430
	ctx.r[8].s64 = ctx.r[11].s64 + -1072;
	// 8260B538: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260B53C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8260B540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B550: 386A6F44  addi r3, r10, 0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + 28484;
	// 8260B554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B574: 4BE5B8AD  bl 0x82466e20
	ctx.lr = 0x8260B578;
	sub_82466E20(ctx, base);
	// 8260B578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B57C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B588 size=24
    let mut pc: u32 = 0x8260B588;
    'dispatch: loop {
        match pc {
            0x8260B588 => {
    //   block [0x8260B588..0x8260B5A0)
	// 8260B588: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B58C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B590: 394A3CF8  addi r10, r10, 0x3cf8
	ctx.r[10].s64 = ctx.r[10].s64 + 15608;
	// 8260B594: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260B598: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260B59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B5A0 size=116
    let mut pc: u32 = 0x8260B5A0;
    'dispatch: loop {
        match pc {
            0x8260B5A0 => {
    //   block [0x8260B5A0..0x8260B614)
	// 8260B5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B5AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B5B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260B5B4: 390A3CF8  addi r8, r10, 0x3cf8
	ctx.r[8].s64 = ctx.r[10].s64 + 15608;
	// 8260B5B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B5BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260B5C0: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260B5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B5D4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8260B5D8: 396BDB98  addi r11, r11, -0x2468
	ctx.r[11].s64 = ctx.r[11].s64 + -9320;
	// 8260B5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B5E4: 386A6F74  addi r3, r10, 0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + 28532;
	// 8260B5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260B5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260B5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B600: 4BE5B821  bl 0x82466e20
	ctx.lr = 0x8260B604;
	sub_82466E20(ctx, base);
	// 8260B604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B618 size=112
    let mut pc: u32 = 0x8260B618;
    'dispatch: loop {
        match pc {
            0x8260B618 => {
    //   block [0x8260B618..0x8260B688)
	// 8260B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B624: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B628: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B62C: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260B630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B634: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 8260B638: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B63C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8260B640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B650: 386A6FA4  addi r3, r10, 0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 28580;
	// 8260B654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B674: 4BE5B7AD  bl 0x82466e20
	ctx.lr = 0x8260B678;
	sub_82466E20(ctx, base);
	// 8260B678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B688 size=112
    let mut pc: u32 = 0x8260B688;
    'dispatch: loop {
        match pc {
            0x8260B688 => {
    //   block [0x8260B688..0x8260B6F8)
	// 8260B688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B698: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B69C: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260B6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B6A4: 390BFC48  addi r8, r11, -0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + -952;
	// 8260B6A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B6AC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8260B6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B6C0: 386A6FD4  addi r3, r10, 0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 28628;
	// 8260B6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B6E4: 4BE5B73D  bl 0x82466e20
	ctx.lr = 0x8260B6E8;
	sub_82466E20(ctx, base);
	// 8260B6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B6F8 size=100
    let mut pc: u32 = 0x8260B6F8;
    'dispatch: loop {
        match pc {
            0x8260B6F8 => {
    //   block [0x8260B6F8..0x8260B75C)
	// 8260B6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B704: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260B708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B70C: 392ADC08  addi r9, r10, -0x23f8
	ctx.r[9].s64 = ctx.r[10].s64 + -9208;
	// 8260B710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B718: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8260B71C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B72C: 386A7004  addi r3, r10, 0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + 28676;
	// 8260B730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B734: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260B738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B748: 4BE5B6D9  bl 0x82466e20
	ctx.lr = 0x8260B74C;
	sub_82466E20(ctx, base);
	// 8260B74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B760 size=24
    let mut pc: u32 = 0x8260B760;
    'dispatch: loop {
        match pc {
            0x8260B760 => {
    //   block [0x8260B760..0x8260B778)
	// 8260B760: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B764: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B768: 394A3DA0  addi r10, r10, 0x3da0
	ctx.r[10].s64 = ctx.r[10].s64 + 15776;
	// 8260B76C: 816BFC84  lwz r11, -0x37c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-892 as u32) ) } as u64;
	// 8260B770: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260B774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B778 size=112
    let mut pc: u32 = 0x8260B778;
    'dispatch: loop {
        match pc {
            0x8260B778 => {
    //   block [0x8260B778..0x8260B7E8)
	// 8260B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B784: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260B788: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B78C: 392ADD48  addi r9, r10, -0x22b8
	ctx.r[9].s64 = ctx.r[10].s64 + -8888;
	// 8260B790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B794: 390B3DA0  addi r8, r11, 0x3da0
	ctx.r[8].s64 = ctx.r[11].s64 + 15776;
	// 8260B798: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260B79C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8260B7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B7B0: 386A7034  addi r3, r10, 0x7034
	ctx.r[3].s64 = ctx.r[10].s64 + 28724;
	// 8260B7B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B7B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8260B7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B7D4: 4BE5B64D  bl 0x82466e20
	ctx.lr = 0x8260B7D8;
	sub_82466E20(ctx, base);
	// 8260B7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B7E8 size=112
    let mut pc: u32 = 0x8260B7E8;
    'dispatch: loop {
        match pc {
            0x8260B7E8 => {
    //   block [0x8260B7E8..0x8260B858)
	// 8260B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B7F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B7F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B7FC: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B804: 390BFC8C  addi r8, r11, -0x374
	ctx.r[8].s64 = ctx.r[11].s64 + -884;
	// 8260B808: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B80C: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8260B810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B814: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B820: 386A7064  addi r3, r10, 0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + 28772;
	// 8260B824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B844: 4BE5B5DD  bl 0x82466e20
	ctx.lr = 0x8260B848;
	sub_82466E20(ctx, base);
	// 8260B848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B858 size=108
    let mut pc: u32 = 0x8260B858;
    'dispatch: loop {
        match pc {
            0x8260B858 => {
    //   block [0x8260B858..0x8260B8C4)
	// 8260B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B864: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B86C: 38EBFCBC  addi r7, r11, -0x344
	ctx.r[7].s64 = ctx.r[11].s64 + -836;
	// 8260B870: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260B874: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8260B878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260B884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B888: 386A7094  addi r3, r10, 0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + 28820;
	// 8260B88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260B890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B8B0: 4BE5B571  bl 0x82466e20
	ctx.lr = 0x8260B8B4;
	sub_82466E20(ctx, base);
	// 8260B8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B8C8 size=100
    let mut pc: u32 = 0x8260B8C8;
    'dispatch: loop {
        match pc {
            0x8260B8C8 => {
    //   block [0x8260B8C8..0x8260B92C)
	// 8260B8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B8D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B8DC: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B8E8: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8260B8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B8FC: 386A70C4  addi r3, r10, 0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + 28868;
	// 8260B900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B918: 4BE5B509  bl 0x82466e20
	ctx.lr = 0x8260B91C;
	sub_82466E20(ctx, base);
	// 8260B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B930 size=112
    let mut pc: u32 = 0x8260B930;
    'dispatch: loop {
        match pc {
            0x8260B930 => {
    //   block [0x8260B930..0x8260B9A0)
	// 8260B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B93C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B940: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B944: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B94C: 390BFCD4  addi r8, r11, -0x32c
	ctx.r[8].s64 = ctx.r[11].s64 + -812;
	// 8260B950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260B954: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8260B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B95C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B968: 386A70F4  addi r3, r10, 0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + 28916;
	// 8260B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B98C: 4BE5B495  bl 0x82466e20
	ctx.lr = 0x8260B990;
	sub_82466E20(ctx, base);
	// 8260B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B9A0 size=112
    let mut pc: u32 = 0x8260B9A0;
    'dispatch: loop {
        match pc {
            0x8260B9A0 => {
    //   block [0x8260B9A0..0x8260BA10)
	// 8260B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B9AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B9B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B9B4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B9BC: 390BFCEC  addi r8, r11, -0x314
	ctx.r[8].s64 = ctx.r[11].s64 + -788;
	// 8260B9C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B9C4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8260B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B9D8: 386A7124  addi r3, r10, 0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + 28964;
	// 8260B9DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B9FC: 4BE5B425  bl 0x82466e20
	ctx.lr = 0x8260BA00;
	sub_82466E20(ctx, base);
	// 8260BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BA10 size=112
    let mut pc: u32 = 0x8260BA10;
    'dispatch: loop {
        match pc {
            0x8260BA10 => {
    //   block [0x8260BA10..0x8260BA80)
	// 8260BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BA1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BA24: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BA2C: 390BFD1C  addi r8, r11, -0x2e4
	ctx.r[8].s64 = ctx.r[11].s64 + -740;
	// 8260BA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BA34: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8260BA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BA3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BA48: 386A7154  addi r3, r10, 0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + 29012;
	// 8260BA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BA6C: 4BE5B3B5  bl 0x82466e20
	ctx.lr = 0x8260BA70;
	sub_82466E20(ctx, base);
	// 8260BA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BA80 size=112
    let mut pc: u32 = 0x8260BA80;
    'dispatch: loop {
        match pc {
            0x8260BA80 => {
    //   block [0x8260BA80..0x8260BAF0)
	// 8260BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BA8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BA94: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BA9C: 390BFD4C  addi r8, r11, -0x2b4
	ctx.r[8].s64 = ctx.r[11].s64 + -692;
	// 8260BAA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BAA4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8260BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BAAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BAB8: 386A7184  addi r3, r10, 0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + 29060;
	// 8260BABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BADC: 4BE5B345  bl 0x82466e20
	ctx.lr = 0x8260BAE0;
	sub_82466E20(ctx, base);
	// 8260BAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BAF0 size=112
    let mut pc: u32 = 0x8260BAF0;
    'dispatch: loop {
        match pc {
            0x8260BAF0 => {
    //   block [0x8260BAF0..0x8260BB60)
	// 8260BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BAFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BB04: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BB0C: 390BFD7C  addi r8, r11, -0x284
	ctx.r[8].s64 = ctx.r[11].s64 + -644;
	// 8260BB10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BB14: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8260BB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BB28: 386A71B4  addi r3, r10, 0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29108;
	// 8260BB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BB4C: 4BE5B2D5  bl 0x82466e20
	ctx.lr = 0x8260BB50;
	sub_82466E20(ctx, base);
	// 8260BB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BB60 size=112
    let mut pc: u32 = 0x8260BB60;
    'dispatch: loop {
        match pc {
            0x8260BB60 => {
    //   block [0x8260BB60..0x8260BBD0)
	// 8260BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BB6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BB74: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BB7C: 390BFD94  addi r8, r11, -0x26c
	ctx.r[8].s64 = ctx.r[11].s64 + -620;
	// 8260BB80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BB84: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8260BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BB98: 386A71E4  addi r3, r10, 0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29156;
	// 8260BB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BBBC: 4BE5B265  bl 0x82466e20
	ctx.lr = 0x8260BBC0;
	sub_82466E20(ctx, base);
	// 8260BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BBD0 size=112
    let mut pc: u32 = 0x8260BBD0;
    'dispatch: loop {
        match pc {
            0x8260BBD0 => {
    //   block [0x8260BBD0..0x8260BC40)
	// 8260BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BBDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BBE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BBE4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BBEC: 390BFDB0  addi r8, r11, -0x250
	ctx.r[8].s64 = ctx.r[11].s64 + -592;
	// 8260BBF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260BBF4: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8260BBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BBFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BC08: 386A7214  addi r3, r10, 0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + 29204;
	// 8260BC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BC2C: 4BE5B1F5  bl 0x82466e20
	ctx.lr = 0x8260BC30;
	sub_82466E20(ctx, base);
	// 8260BC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BC40 size=112
    let mut pc: u32 = 0x8260BC40;
    'dispatch: loop {
        match pc {
            0x8260BC40 => {
    //   block [0x8260BC40..0x8260BCB0)
	// 8260BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BC4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BC54: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BC5C: 390BFDF8  addi r8, r11, -0x208
	ctx.r[8].s64 = ctx.r[11].s64 + -520;
	// 8260BC60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260BC64: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8260BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BC6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BC78: 386A7244  addi r3, r10, 0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + 29252;
	// 8260BC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BC9C: 4BE5B185  bl 0x82466e20
	ctx.lr = 0x8260BCA0;
	sub_82466E20(ctx, base);
	// 8260BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BCB0 size=112
    let mut pc: u32 = 0x8260BCB0;
    'dispatch: loop {
        match pc {
            0x8260BCB0 => {
    //   block [0x8260BCB0..0x8260BD20)
	// 8260BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BCBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BCC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BCC4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BCCC: 390BFE40  addi r8, r11, -0x1c0
	ctx.r[8].s64 = ctx.r[11].s64 + -448;
	// 8260BCD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260BCD4: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8260BCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BCE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BCE8: 386A7274  addi r3, r10, 0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + 29300;
	// 8260BCEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BD0C: 4BE5B115  bl 0x82466e20
	ctx.lr = 0x8260BD10;
	sub_82466E20(ctx, base);
	// 8260BD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BD20 size=112
    let mut pc: u32 = 0x8260BD20;
    'dispatch: loop {
        match pc {
            0x8260BD20 => {
    //   block [0x8260BD20..0x8260BD90)
	// 8260BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BD2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BD30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BD34: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BD3C: 390BFE58  addi r8, r11, -0x1a8
	ctx.r[8].s64 = ctx.r[11].s64 + -424;
	// 8260BD40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BD44: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8260BD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BD4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BD58: 386A72A4  addi r3, r10, 0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29348;
	// 8260BD5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BD7C: 4BE5B0A5  bl 0x82466e20
	ctx.lr = 0x8260BD80;
	sub_82466E20(ctx, base);
	// 8260BD80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BD90 size=116
    let mut pc: u32 = 0x8260BD90;
    'dispatch: loop {
        match pc {
            0x8260BD90 => {
    //   block [0x8260BD90..0x8260BE04)
	// 8260BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BD9C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BDA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260BDA4: 390AFE88  addi r8, r10, -0x178
	ctx.r[8].s64 = ctx.r[10].s64 + -376;
	// 8260BDA8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BDAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BDB0: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BDB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BDB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BDC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BDC4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8260BDC8: 396BDD70  addi r11, r11, -0x2290
	ctx.r[11].s64 = ctx.r[11].s64 + -8848;
	// 8260BDCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BDD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BDD4: 386A72D4  addi r3, r10, 0x72d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29396;
	// 8260BDD8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260BDDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BDE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260BDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BDF0: 4BE5B031  bl 0x82466e20
	ctx.lr = 0x8260BDF4;
	sub_82466E20(ctx, base);
	// 8260BDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BE08 size=116
    let mut pc: u32 = 0x8260BE08;
    'dispatch: loop {
        match pc {
            0x8260BE08 => {
    //   block [0x8260BE08..0x8260BE7C)
	// 8260BE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BE14: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BE18: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260BE1C: 390AFF00  addi r8, r10, -0x100
	ctx.r[8].s64 = ctx.r[10].s64 + -256;
	// 8260BE20: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BE24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BE28: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BE2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BE30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260BE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BE3C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8260BE40: 396BDD88  addi r11, r11, -0x2278
	ctx.r[11].s64 = ctx.r[11].s64 + -8824;
	// 8260BE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BE48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BE4C: 386A7304  addi r3, r10, 0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + 29444;
	// 8260BE50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260BE54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BE58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260BE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BE68: 4BE5AFB9  bl 0x82466e20
	ctx.lr = 0x8260BE6C;
	sub_82466E20(ctx, base);
	// 8260BE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260BE80 size=24
    let mut pc: u32 = 0x8260BE80;
    'dispatch: loop {
        match pc {
            0x8260BE80 => {
    //   block [0x8260BE80..0x8260BE98)
	// 8260BE80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BE84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260BE88: 394A3DB8  addi r10, r10, 0x3db8
	ctx.r[10].s64 = ctx.r[10].s64 + 15800;
	// 8260BE8C: 816BFDAC  lwz r11, -0x254(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-596 as u32) ) } as u64;
	// 8260BE90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BE98 size=116
    let mut pc: u32 = 0x8260BE98;
    'dispatch: loop {
        match pc {
            0x8260BE98 => {
    //   block [0x8260BE98..0x8260BF0C)
	// 8260BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BEA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260BEA8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BEAC: 392BDDB4  addi r9, r11, -0x224c
	ctx.r[9].s64 = ctx.r[11].s64 + -8780;
	// 8260BEB0: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BEB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BEB8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260BEBC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260BEC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BEC4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8260BEC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BECC: 396B3DB8  addi r11, r11, 0x3db8
	ctx.r[11].s64 = ctx.r[11].s64 + 15800;
	// 8260BED0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260BED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BED8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260BEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BEE0: 386A7334  addi r3, r10, 0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + 29492;
	// 8260BEE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260BEE8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260BEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BEF0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260BEF4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260BEF8: 4BE5AF29  bl 0x82466e20
	ctx.lr = 0x8260BEFC;
	sub_82466E20(ctx, base);
	// 8260BEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BF10 size=112
    let mut pc: u32 = 0x8260BF10;
    'dispatch: loop {
        match pc {
            0x8260BF10 => {
    //   block [0x8260BF10..0x8260BF80)
	// 8260BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BF1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BF24: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BF2C: 390BFF90  addi r8, r11, -0x70
	ctx.r[8].s64 = ctx.r[11].s64 + -112;
	// 8260BF30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260BF34: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8260BF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BF48: 386A7364  addi r3, r10, 0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + 29540;
	// 8260BF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BF6C: 4BE5AEB5  bl 0x82466e20
	ctx.lr = 0x8260BF70;
	sub_82466E20(ctx, base);
	// 8260BF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BF80 size=112
    let mut pc: u32 = 0x8260BF80;
    'dispatch: loop {
        match pc {
            0x8260BF80 => {
    //   block [0x8260BF80..0x8260BFF0)
	// 8260BF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BF8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BF90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BF94: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BF9C: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 8260BFA0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260BFA4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8260BFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BFAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BFB8: 386A7394  addi r3, r10, 0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + 29588;
	// 8260BFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BFDC: 4BE5AE45  bl 0x82466e20
	ctx.lr = 0x8260BFE0;
	sub_82466E20(ctx, base);
	// 8260BFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BFF0 size=112
    let mut pc: u32 = 0x8260BFF0;
    'dispatch: loop {
        match pc {
            0x8260BFF0 => {
    //   block [0x8260BFF0..0x8260C060)
	// 8260BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BFFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C000: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C004: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C00C: 390B0098  addi r8, r11, 0x98
	ctx.r[8].s64 = ctx.r[11].s64 + 152;
	// 8260C010: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260C014: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8260C018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C028: 386A73C4  addi r3, r10, 0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29636;
	// 8260C02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C04C: 4BE5ADD5  bl 0x82466e20
	ctx.lr = 0x8260C050;
	sub_82466E20(ctx, base);
	// 8260C050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C060 size=112
    let mut pc: u32 = 0x8260C060;
    'dispatch: loop {
        match pc {
            0x8260C060 => {
    //   block [0x8260C060..0x8260C0D0)
	// 8260C060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C06C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C070: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C074: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C07C: 390B0110  addi r8, r11, 0x110
	ctx.r[8].s64 = ctx.r[11].s64 + 272;
	// 8260C080: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260C084: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8260C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C098: 386A73F4  addi r3, r10, 0x73f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29684;
	// 8260C09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C0BC: 4BE5AD65  bl 0x82466e20
	ctx.lr = 0x8260C0C0;
	sub_82466E20(ctx, base);
	// 8260C0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C0D0 size=112
    let mut pc: u32 = 0x8260C0D0;
    'dispatch: loop {
        match pc {
            0x8260C0D0 => {
    //   block [0x8260C0D0..0x8260C140)
	// 8260C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C0DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C0E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C0E4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C0EC: 390B0158  addi r8, r11, 0x158
	ctx.r[8].s64 = ctx.r[11].s64 + 344;
	// 8260C0F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260C0F4: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8260C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C108: 386A7424  addi r3, r10, 0x7424
	ctx.r[3].s64 = ctx.r[10].s64 + 29732;
	// 8260C10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C12C: 4BE5ACF5  bl 0x82466e20
	ctx.lr = 0x8260C130;
	sub_82466E20(ctx, base);
	// 8260C130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C140 size=112
    let mut pc: u32 = 0x8260C140;
    'dispatch: loop {
        match pc {
            0x8260C140 => {
    //   block [0x8260C140..0x8260C1B0)
	// 8260C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C150: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C154: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C15C: 390B01E8  addi r8, r11, 0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + 488;
	// 8260C160: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C164: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8260C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C178: 386A7454  addi r3, r10, 0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + 29780;
	// 8260C17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C19C: 4BE5AC85  bl 0x82466e20
	ctx.lr = 0x8260C1A0;
	sub_82466E20(ctx, base);
	// 8260C1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C1B0 size=112
    let mut pc: u32 = 0x8260C1B0;
    'dispatch: loop {
        match pc {
            0x8260C1B0 => {
    //   block [0x8260C1B0..0x8260C220)
	// 8260C1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C1BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C1C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C1C4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C1CC: 390B0248  addi r8, r11, 0x248
	ctx.r[8].s64 = ctx.r[11].s64 + 584;
	// 8260C1D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C1D4: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8260C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C1E8: 386A7484  addi r3, r10, 0x7484
	ctx.r[3].s64 = ctx.r[10].s64 + 29828;
	// 8260C1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C20C: 4BE5AC15  bl 0x82466e20
	ctx.lr = 0x8260C210;
	sub_82466E20(ctx, base);
	// 8260C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C220 size=112
    let mut pc: u32 = 0x8260C220;
    'dispatch: loop {
        match pc {
            0x8260C220 => {
    //   block [0x8260C220..0x8260C290)
	// 8260C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C22C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C230: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C234: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C23C: 390B02A8  addi r8, r11, 0x2a8
	ctx.r[8].s64 = ctx.r[11].s64 + 680;
	// 8260C240: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C244: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8260C248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C24C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C258: 386A74B4  addi r3, r10, 0x74b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29876;
	// 8260C25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C27C: 4BE5ABA5  bl 0x82466e20
	ctx.lr = 0x8260C280;
	sub_82466E20(ctx, base);
	// 8260C280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C290 size=112
    let mut pc: u32 = 0x8260C290;
    'dispatch: loop {
        match pc {
            0x8260C290 => {
    //   block [0x8260C290..0x8260C300)
	// 8260C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C29C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C2A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C2A4: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C2AC: 390B02D8  addi r8, r11, 0x2d8
	ctx.r[8].s64 = ctx.r[11].s64 + 728;
	// 8260C2B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C2B4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8260C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C2C8: 386A74E4  addi r3, r10, 0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29924;
	// 8260C2CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C2EC: 4BE5AB35  bl 0x82466e20
	ctx.lr = 0x8260C2F0;
	sub_82466E20(ctx, base);
	// 8260C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C300 size=100
    let mut pc: u32 = 0x8260C300;
    'dispatch: loop {
        match pc {
            0x8260C300 => {
    //   block [0x8260C300..0x8260C364)
	// 8260C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C30C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C314: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C320: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8260C324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C334: 386A7514  addi r3, r10, 0x7514
	ctx.r[3].s64 = ctx.r[10].s64 + 29972;
	// 8260C338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C33C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C350: 4BE5AAD1  bl 0x82466e20
	ctx.lr = 0x8260C354;
	sub_82466E20(ctx, base);
	// 8260C354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C368 size=112
    let mut pc: u32 = 0x8260C368;
    'dispatch: loop {
        match pc {
            0x8260C368 => {
    //   block [0x8260C368..0x8260C3D8)
	// 8260C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C37C: 38AA7484  addi r5, r10, 0x7484
	ctx.r[5].s64 = ctx.r[10].s64 + 29828;
	// 8260C380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C384: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 8260C388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C38C: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8260C390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C394: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C3A0: 386A7544  addi r3, r10, 0x7544
	ctx.r[3].s64 = ctx.r[10].s64 + 30020;
	// 8260C3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C3C4: 4BE5AA5D  bl 0x82466e20
	ctx.lr = 0x8260C3C8;
	sub_82466E20(ctx, base);
	// 8260C3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C3D8 size=112
    let mut pc: u32 = 0x8260C3D8;
    'dispatch: loop {
        match pc {
            0x8260C3D8 => {
    //   block [0x8260C3D8..0x8260C448)
	// 8260C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C3E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C3EC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C3F4: 390B0320  addi r8, r11, 0x320
	ctx.r[8].s64 = ctx.r[11].s64 + 800;
	// 8260C3F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260C3FC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8260C400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C410: 386A7574  addi r3, r10, 0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + 30068;
	// 8260C414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C434: 4BE5A9ED  bl 0x82466e20
	ctx.lr = 0x8260C438;
	sub_82466E20(ctx, base);
	// 8260C438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C448 size=112
    let mut pc: u32 = 0x8260C448;
    'dispatch: loop {
        match pc {
            0x8260C448 => {
    //   block [0x8260C448..0x8260C4B8)
	// 8260C448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C458: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C45C: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C464: 390B0380  addi r8, r11, 0x380
	ctx.r[8].s64 = ctx.r[11].s64 + 896;
	// 8260C468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C46C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8260C470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C480: 386A75A4  addi r3, r10, 0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30116;
	// 8260C484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C4A4: 4BE5A97D  bl 0x82466e20
	ctx.lr = 0x8260C4A8;
	sub_82466E20(ctx, base);
	// 8260C4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C4B8 size=112
    let mut pc: u32 = 0x8260C4B8;
    'dispatch: loop {
        match pc {
            0x8260C4B8 => {
    //   block [0x8260C4B8..0x8260C528)
	// 8260C4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C4CC: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C4D4: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8260C4D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260C4DC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8260C4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C4F0: 386A75D4  addi r3, r10, 0x75d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30164;
	// 8260C4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C514: 4BE5A90D  bl 0x82466e20
	ctx.lr = 0x8260C518;
	sub_82466E20(ctx, base);
	// 8260C518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C528 size=112
    let mut pc: u32 = 0x8260C528;
    'dispatch: loop {
        match pc {
            0x8260C528 => {
    //   block [0x8260C528..0x8260C598)
	// 8260C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C538: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C53C: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260C540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C544: 390B03C8  addi r8, r11, 0x3c8
	ctx.r[8].s64 = ctx.r[11].s64 + 968;
	// 8260C548: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C54C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8260C550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C560: 386A7604  addi r3, r10, 0x7604
	ctx.r[3].s64 = ctx.r[10].s64 + 30212;
	// 8260C564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C584: 4BE5A89D  bl 0x82466e20
	ctx.lr = 0x8260C588;
	sub_82466E20(ctx, base);
	// 8260C588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260C598 size=24
    let mut pc: u32 = 0x8260C598;
    'dispatch: loop {
        match pc {
            0x8260C598 => {
    //   block [0x8260C598..0x8260C5B0)
	// 8260C598: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C59C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260C5A0: 394A3E60  addi r10, r10, 0x3e60
	ctx.r[10].s64 = ctx.r[10].s64 + 15968;
	// 8260C5A4: 816B03E0  lwz r11, 0x3e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(992 as u32) ) } as u64;
	// 8260C5A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260C5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C5B0 size=112
    let mut pc: u32 = 0x8260C5B0;
    'dispatch: loop {
        match pc {
            0x8260C5B0 => {
    //   block [0x8260C5B0..0x8260C620)
	// 8260C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C5BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C5C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C5C4: 392ADE10  addi r9, r10, -0x21f0
	ctx.r[9].s64 = ctx.r[10].s64 + -8688;
	// 8260C5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C5CC: 390B3E60  addi r8, r11, 0x3e60
	ctx.r[8].s64 = ctx.r[11].s64 + 15968;
	// 8260C5D0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260C5D4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8260C5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C5E8: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 8260C5EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C60C: 4BE5A815  bl 0x82466e20
	ctx.lr = 0x8260C610;
	sub_82466E20(ctx, base);
	// 8260C610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C620 size=108
    let mut pc: u32 = 0x8260C620;
    'dispatch: loop {
        match pc {
            0x8260C620 => {
    //   block [0x8260C620..0x8260C68C)
	// 8260C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C62C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C634: 38EB03E4  addi r7, r11, 0x3e4
	ctx.r[7].s64 = ctx.r[11].s64 + 996;
	// 8260C638: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260C63C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8260C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C650: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 8260C654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C678: 4BE5A7A9  bl 0x82466e20
	ctx.lr = 0x8260C67C;
	sub_82466E20(ctx, base);
	// 8260C67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C690 size=108
    let mut pc: u32 = 0x8260C690;
    'dispatch: loop {
        match pc {
            0x8260C690 => {
    //   block [0x8260C690..0x8260C6FC)
	// 8260C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C69C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C6A4: 38EB0400  addi r7, r11, 0x400
	ctx.r[7].s64 = ctx.r[11].s64 + 1024;
	// 8260C6A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260C6AC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8260C6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C6C0: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 8260C6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C6E8: 4BE5A739  bl 0x82466e20
	ctx.lr = 0x8260C6EC;
	sub_82466E20(ctx, base);
	// 8260C6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C700 size=116
    let mut pc: u32 = 0x8260C700;
    'dispatch: loop {
        match pc {
            0x8260C700 => {
    //   block [0x8260C700..0x8260C774)
	// 8260C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C70C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C714: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 8260C718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C71C: 392ADEC8  addi r9, r10, -0x2138
	ctx.r[9].s64 = ctx.r[10].s64 + -8504;
	// 8260C720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C724: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260C728: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C72C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C734: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C744: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260C748: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8260C74C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C750: 386B76C4  addi r3, r11, 0x76c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30404;
	// 8260C754: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C758: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C760: 4BE5A6C1  bl 0x82466e20
	ctx.lr = 0x8260C764;
	sub_82466E20(ctx, base);
	// 8260C764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260C778 size=24
    let mut pc: u32 = 0x8260C778;
    'dispatch: loop {
        match pc {
            0x8260C778 => {
    //   block [0x8260C778..0x8260C790)
	// 8260C778: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C77C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260C780: 394A3EA8  addi r10, r10, 0x3ea8
	ctx.r[10].s64 = ctx.r[10].s64 + 16040;
	// 8260C784: 816B0460  lwz r11, 0x460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1120 as u32) ) } as u64;
	// 8260C788: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C790 size=116
    let mut pc: u32 = 0x8260C790;
    'dispatch: loop {
        match pc {
            0x8260C790 => {
    //   block [0x8260C790..0x8260C804)
	// 8260C790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C79C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C7A4: 390B3EA8  addi r8, r11, 0x3ea8
	ctx.r[8].s64 = ctx.r[11].s64 + 16040;
	// 8260C7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C7AC: 392ADF24  addi r9, r10, -0x20dc
	ctx.r[9].s64 = ctx.r[10].s64 + -8412;
	// 8260C7B0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C7B4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8260C7B8: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260C7BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C7C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C7D4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260C7D8: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8260C7DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C7E0: 386B76F4  addi r3, r11, 0x76f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30452;
	// 8260C7E4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260C7E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C7F0: 4BE5A631  bl 0x82466e20
	ctx.lr = 0x8260C7F4;
	sub_82466E20(ctx, base);
	// 8260C7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C808 size=108
    let mut pc: u32 = 0x8260C808;
    'dispatch: loop {
        match pc {
            0x8260C808 => {
    //   block [0x8260C808..0x8260C874)
	// 8260C808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C814: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C81C: 38EB046C  addi r7, r11, 0x46c
	ctx.r[7].s64 = ctx.r[11].s64 + 1132;
	// 8260C820: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260C824: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8260C828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260C834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C838: 386A7724  addi r3, r10, 0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + 30500;
	// 8260C83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260C840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C860: 4BE5A5C1  bl 0x82466e20
	ctx.lr = 0x8260C864;
	sub_82466E20(ctx, base);
	// 8260C864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C878 size=112
    let mut pc: u32 = 0x8260C878;
    'dispatch: loop {
        match pc {
            0x8260C878 => {
    //   block [0x8260C878..0x8260C8E8)
	// 8260C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C888: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C88C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260C890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C894: 390B049C  addi r8, r11, 0x49c
	ctx.r[8].s64 = ctx.r[11].s64 + 1180;
	// 8260C898: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C89C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8260C8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C8A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C8B0: 386A7754  addi r3, r10, 0x7754
	ctx.r[3].s64 = ctx.r[10].s64 + 30548;
	// 8260C8B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C8D4: 4BE5A54D  bl 0x82466e20
	ctx.lr = 0x8260C8D8;
	sub_82466E20(ctx, base);
	// 8260C8D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C8E8 size=112
    let mut pc: u32 = 0x8260C8E8;
    'dispatch: loop {
        match pc {
            0x8260C8E8 => {
    //   block [0x8260C8E8..0x8260C958)
	// 8260C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C8F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C8F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C8FC: 392ADF68  addi r9, r10, -0x2098
	ctx.r[9].s64 = ctx.r[10].s64 + -8344;
	// 8260C900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C904: 390B04B8  addi r8, r11, 0x4b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1208;
	// 8260C908: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260C90C: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8260C910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C920: 386A7784  addi r3, r10, 0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + 30596;
	// 8260C924: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260C928: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260C92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260C940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C944: 4BE5A4DD  bl 0x82466e20
	ctx.lr = 0x8260C948;
	sub_82466E20(ctx, base);
	// 8260C948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C958 size=112
    let mut pc: u32 = 0x8260C958;
    'dispatch: loop {
        match pc {
            0x8260C958 => {
    //   block [0x8260C958..0x8260C9C8)
	// 8260C958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C964: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C968: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C96C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260C970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C974: 390B0500  addi r8, r11, 0x500
	ctx.r[8].s64 = ctx.r[11].s64 + 1280;
	// 8260C978: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260C97C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8260C980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260C990: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 8260C994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260C998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260C99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260C9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260C9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260C9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260C9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260C9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260C9B4: 4BE5A46D  bl 0x82466e20
	ctx.lr = 0x8260C9B8;
	sub_82466E20(ctx, base);
	// 8260C9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260C9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260C9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260C9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260C9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260C9C8 size=112
    let mut pc: u32 = 0x8260C9C8;
    'dispatch: loop {
        match pc {
            0x8260C9C8 => {
    //   block [0x8260C9C8..0x8260CA38)
	// 8260C9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260C9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260C9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260C9D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260C9D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260C9DC: 392ADF94  addi r9, r10, -0x206c
	ctx.r[9].s64 = ctx.r[10].s64 + -8300;
	// 8260C9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260C9E4: 390B0520  addi r8, r11, 0x520
	ctx.r[8].s64 = ctx.r[11].s64 + 1312;
	// 8260C9E8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260C9EC: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8260C9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260C9F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260C9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260C9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CA00: 386A77E4  addi r3, r10, 0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30692;
	// 8260CA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CA08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CA24: 4BE5A3FD  bl 0x82466e20
	ctx.lr = 0x8260CA28;
	sub_82466E20(ctx, base);
	// 8260CA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CA38 size=112
    let mut pc: u32 = 0x8260CA38;
    'dispatch: loop {
        match pc {
            0x8260CA38 => {
    //   block [0x8260CA38..0x8260CAA8)
	// 8260CA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CA44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CA48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CA4C: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CA50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CA54: 390B05B0  addi r8, r11, 0x5b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1456;
	// 8260CA58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CA5C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8260CA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CA64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CA68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CA70: 386A7814  addi r3, r10, 0x7814
	ctx.r[3].s64 = ctx.r[10].s64 + 30740;
	// 8260CA74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CA94: 4BE5A38D  bl 0x82466e20
	ctx.lr = 0x8260CA98;
	sub_82466E20(ctx, base);
	// 8260CA98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CAA8 size=112
    let mut pc: u32 = 0x8260CAA8;
    'dispatch: loop {
        match pc {
            0x8260CAA8 => {
    //   block [0x8260CAA8..0x8260CB18)
	// 8260CAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CAB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CABC: 38AA7874  addi r5, r10, 0x7874
	ctx.r[5].s64 = ctx.r[10].s64 + 30836;
	// 8260CAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CAC4: 390B05C8  addi r8, r11, 0x5c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1480;
	// 8260CAC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260CACC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8260CAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CAD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CAE0: 386A7844  addi r3, r10, 0x7844
	ctx.r[3].s64 = ctx.r[10].s64 + 30788;
	// 8260CAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CB04: 4BE5A31D  bl 0x82466e20
	ctx.lr = 0x8260CB08;
	sub_82466E20(ctx, base);
	// 8260CB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CB18 size=100
    let mut pc: u32 = 0x8260CB18;
    'dispatch: loop {
        match pc {
            0x8260CB18 => {
    //   block [0x8260CB18..0x8260CB7C)
	// 8260CB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CB24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CB2C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260CB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CB38: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8260CB3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CB4C: 386A7874  addi r3, r10, 0x7874
	ctx.r[3].s64 = ctx.r[10].s64 + 30836;
	// 8260CB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CB58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260CB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CB60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260CB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CB68: 4BE5A2B9  bl 0x82466e20
	ctx.lr = 0x8260CB6C;
	sub_82466E20(ctx, base);
	// 8260CB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260CB80 size=24
    let mut pc: u32 = 0x8260CB80;
    'dispatch: loop {
        match pc {
            0x8260CB80 => {
    //   block [0x8260CB80..0x8260CB98)
	// 8260CB80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CB84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260CB88: 394A3F80  addi r10, r10, 0x3f80
	ctx.r[10].s64 = ctx.r[10].s64 + 16256;
	// 8260CB8C: 816B051C  lwz r11, 0x51c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1308 as u32) ) } as u64;
	// 8260CB90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CB98 size=116
    let mut pc: u32 = 0x8260CB98;
    'dispatch: loop {
        match pc {
            0x8260CB98 => {
    //   block [0x8260CB98..0x8260CC0C)
	// 8260CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CBA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CBA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CBAC: 390B3F80  addi r8, r11, 0x3f80
	ctx.r[8].s64 = ctx.r[11].s64 + 16256;
	// 8260CBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CBB4: 392ADFD0  addi r9, r10, -0x2030
	ctx.r[9].s64 = ctx.r[10].s64 + -8240;
	// 8260CBB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CBBC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260CBC0: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CBC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CBC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CBCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CBD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CBD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CBDC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260CBE0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8260CBE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CBE8: 386B78A4  addi r3, r11, 0x78a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30884;
	// 8260CBEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CBF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CBF8: 4BE5A229  bl 0x82466e20
	ctx.lr = 0x8260CBFC;
	sub_82466E20(ctx, base);
	// 8260CBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CC10 size=108
    let mut pc: u32 = 0x8260CC10;
    'dispatch: loop {
        match pc {
            0x8260CC10 => {
    //   block [0x8260CC10..0x8260CC7C)
	// 8260CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CC1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CC24: 38EB0640  addi r7, r11, 0x640
	ctx.r[7].s64 = ctx.r[11].s64 + 1600;
	// 8260CC28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260CC2C: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8260CC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260CC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CC40: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 8260CC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260CC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CC68: 4BE5A1B9  bl 0x82466e20
	ctx.lr = 0x8260CC6C;
	sub_82466E20(ctx, base);
	// 8260CC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CC80 size=112
    let mut pc: u32 = 0x8260CC80;
    'dispatch: loop {
        match pc {
            0x8260CC80 => {
    //   block [0x8260CC80..0x8260CCF0)
	// 8260CC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CC90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CC94: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CC98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CC9C: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 8260CCA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CCA4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8260CCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CCAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CCB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CCB8: 386A7904  addi r3, r10, 0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + 30980;
	// 8260CCBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CCC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CCDC: 4BE5A145  bl 0x82466e20
	ctx.lr = 0x8260CCE0;
	sub_82466E20(ctx, base);
	// 8260CCE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CCF0 size=112
    let mut pc: u32 = 0x8260CCF0;
    'dispatch: loop {
        match pc {
            0x8260CCF0 => {
    //   block [0x8260CCF0..0x8260CD60)
	// 8260CCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CCFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CD00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CD04: 392ADFF4  addi r9, r10, -0x200c
	ctx.r[9].s64 = ctx.r[10].s64 + -8204;
	// 8260CD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CD0C: 390B0690  addi r8, r11, 0x690
	ctx.r[8].s64 = ctx.r[11].s64 + 1680;
	// 8260CD10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260CD14: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8260CD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CD1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CD28: 386A7934  addi r3, r10, 0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + 31028;
	// 8260CD2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CD30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CD4C: 4BE5A0D5  bl 0x82466e20
	ctx.lr = 0x8260CD50;
	sub_82466E20(ctx, base);
	// 8260CD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CD60 size=112
    let mut pc: u32 = 0x8260CD60;
    'dispatch: loop {
        match pc {
            0x8260CD60 => {
    //   block [0x8260CD60..0x8260CDD0)
	// 8260CD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CD6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CD74: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CD78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CD7C: 390B0738  addi r8, r11, 0x738
	ctx.r[8].s64 = ctx.r[11].s64 + 1848;
	// 8260CD80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CD84: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8260CD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CD8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CD90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CD98: 386A7964  addi r3, r10, 0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + 31076;
	// 8260CD9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CDA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CDBC: 4BE5A065  bl 0x82466e20
	ctx.lr = 0x8260CDC0;
	sub_82466E20(ctx, base);
	// 8260CDC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CDD0 size=108
    let mut pc: u32 = 0x8260CDD0;
    'dispatch: loop {
        match pc {
            0x8260CDD0 => {
    //   block [0x8260CDD0..0x8260CE3C)
	// 8260CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CDDC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CDE4: 38EB0750  addi r7, r11, 0x750
	ctx.r[7].s64 = ctx.r[11].s64 + 1872;
	// 8260CDE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260CDEC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8260CDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260CDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CE00: 386A7994  addi r3, r10, 0x7994
	ctx.r[3].s64 = ctx.r[10].s64 + 31124;
	// 8260CE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260CE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CE28: 4BE59FF9  bl 0x82466e20
	ctx.lr = 0x8260CE2C;
	sub_82466E20(ctx, base);
	// 8260CE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CE40 size=112
    let mut pc: u32 = 0x8260CE40;
    'dispatch: loop {
        match pc {
            0x8260CE40 => {
    //   block [0x8260CE40..0x8260CEB0)
	// 8260CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CE4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CE50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CE54: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CE5C: 390B0780  addi r8, r11, 0x780
	ctx.r[8].s64 = ctx.r[11].s64 + 1920;
	// 8260CE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260CE64: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8260CE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CE6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CE78: 386A79C4  addi r3, r10, 0x79c4
	ctx.r[3].s64 = ctx.r[10].s64 + 31172;
	// 8260CE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CE9C: 4BE59F85  bl 0x82466e20
	ctx.lr = 0x8260CEA0;
	sub_82466E20(ctx, base);
	// 8260CEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CEB0 size=112
    let mut pc: u32 = 0x8260CEB0;
    'dispatch: loop {
        match pc {
            0x8260CEB0 => {
    //   block [0x8260CEB0..0x8260CF20)
	// 8260CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CEBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260CEC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CEC4: 392AE028  addi r9, r10, -0x1fd8
	ctx.r[9].s64 = ctx.r[10].s64 + -8152;
	// 8260CEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CECC: 390B0798  addi r8, r11, 0x798
	ctx.r[8].s64 = ctx.r[11].s64 + 1944;
	// 8260CED0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260CED4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8260CED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CEDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CEE8: 386A79F4  addi r3, r10, 0x79f4
	ctx.r[3].s64 = ctx.r[10].s64 + 31220;
	// 8260CEEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260CEF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260CEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260CF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CF0C: 4BE59F15  bl 0x82466e20
	ctx.lr = 0x8260CF10;
	sub_82466E20(ctx, base);
	// 8260CF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CF20 size=112
    let mut pc: u32 = 0x8260CF20;
    'dispatch: loop {
        match pc {
            0x8260CF20 => {
    //   block [0x8260CF20..0x8260CF90)
	// 8260CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CF30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CF34: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CF3C: 390B0840  addi r8, r11, 0x840
	ctx.r[8].s64 = ctx.r[11].s64 + 2112;
	// 8260CF40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260CF44: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8260CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CF4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CF58: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 8260CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CF7C: 4BE59EA5  bl 0x82466e20
	ctx.lr = 0x8260CF80;
	sub_82466E20(ctx, base);
	// 8260CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260CF90 size=112
    let mut pc: u32 = 0x8260CF90;
    'dispatch: loop {
        match pc {
            0x8260CF90 => {
    //   block [0x8260CF90..0x8260D000)
	// 8260CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260CF9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CFA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260CFA4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260CFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260CFAC: 390B0888  addi r8, r11, 0x888
	ctx.r[8].s64 = ctx.r[11].s64 + 2184;
	// 8260CFB0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260CFB4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8260CFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260CFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260CFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260CFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260CFC8: 386A7A54  addi r3, r10, 0x7a54
	ctx.r[3].s64 = ctx.r[10].s64 + 31316;
	// 8260CFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260CFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260CFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260CFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260CFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260CFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260CFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260CFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260CFEC: 4BE59E35  bl 0x82466e20
	ctx.lr = 0x8260CFF0;
	sub_82466E20(ctx, base);
	// 8260CFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260CFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260CFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260CFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D000 size=100
    let mut pc: u32 = 0x8260D000;
    'dispatch: loop {
        match pc {
            0x8260D000 => {
    //   block [0x8260D000..0x8260D064)
	// 8260D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D00C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D014: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D020: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8260D024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D034: 386A7A84  addi r3, r10, 0x7a84
	ctx.r[3].s64 = ctx.r[10].s64 + 31364;
	// 8260D038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260D044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260D04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D050: 4BE59DD1  bl 0x82466e20
	ctx.lr = 0x8260D054;
	sub_82466E20(ctx, base);
	// 8260D054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D068 size=112
    let mut pc: u32 = 0x8260D068;
    'dispatch: loop {
        match pc {
            0x8260D068 => {
    //   block [0x8260D068..0x8260D0D8)
	// 8260D068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D074: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D078: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D07C: 38AA76F4  addi r5, r10, 0x76f4
	ctx.r[5].s64 = ctx.r[10].s64 + 30452;
	// 8260D080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D084: 390B0948  addi r8, r11, 0x948
	ctx.r[8].s64 = ctx.r[11].s64 + 2376;
	// 8260D088: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D08C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8260D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D0A0: 386A7AB4  addi r3, r10, 0x7ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 31412;
	// 8260D0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D0C4: 4BE59D5D  bl 0x82466e20
	ctx.lr = 0x8260D0C8;
	sub_82466E20(ctx, base);
	// 8260D0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D0D8 size=112
    let mut pc: u32 = 0x8260D0D8;
    'dispatch: loop {
        match pc {
            0x8260D0D8 => {
    //   block [0x8260D0D8..0x8260D148)
	// 8260D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D0E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D0E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D0EC: 38AA7574  addi r5, r10, 0x7574
	ctx.r[5].s64 = ctx.r[10].s64 + 30068;
	// 8260D0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D0F4: 390B0978  addi r8, r11, 0x978
	ctx.r[8].s64 = ctx.r[11].s64 + 2424;
	// 8260D0F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260D0FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8260D100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D110: 386A7AE4  addi r3, r10, 0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 31460;
	// 8260D114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D134: 4BE59CED  bl 0x82466e20
	ctx.lr = 0x8260D138;
	sub_82466E20(ctx, base);
	// 8260D138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D148 size=108
    let mut pc: u32 = 0x8260D148;
    'dispatch: loop {
        match pc {
            0x8260D148 => {
    //   block [0x8260D148..0x8260D1B4)
	// 8260D148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D154: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D15C: 38EB0990  addi r7, r11, 0x990
	ctx.r[7].s64 = ctx.r[11].s64 + 2448;
	// 8260D160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260D164: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8260D168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D178: 386A7B14  addi r3, r10, 0x7b14
	ctx.r[3].s64 = ctx.r[10].s64 + 31508;
	// 8260D17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D1A0: 4BE59C81  bl 0x82466e20
	ctx.lr = 0x8260D1A4;
	sub_82466E20(ctx, base);
	// 8260D1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D1B8 size=112
    let mut pc: u32 = 0x8260D1B8;
    'dispatch: loop {
        match pc {
            0x8260D1B8 => {
    //   block [0x8260D1B8..0x8260D228)
	// 8260D1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D1C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D1C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D1CC: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D1D4: 390B09C0  addi r8, r11, 0x9c0
	ctx.r[8].s64 = ctx.r[11].s64 + 2496;
	// 8260D1D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260D1DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8260D1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D1E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D1F0: 386A7B44  addi r3, r10, 0x7b44
	ctx.r[3].s64 = ctx.r[10].s64 + 31556;
	// 8260D1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D214: 4BE59C0D  bl 0x82466e20
	ctx.lr = 0x8260D218;
	sub_82466E20(ctx, base);
	// 8260D218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D228 size=112
    let mut pc: u32 = 0x8260D228;
    'dispatch: loop {
        match pc {
            0x8260D228 => {
    //   block [0x8260D228..0x8260D298)
	// 8260D228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D234: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D238: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D23C: 392AE054  addi r9, r10, -0x1fac
	ctx.r[9].s64 = ctx.r[10].s64 + -8108;
	// 8260D240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D244: 390B0A58  addi r8, r11, 0xa58
	ctx.r[8].s64 = ctx.r[11].s64 + 2648;
	// 8260D248: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260D24C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8260D250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D260: 386A7B74  addi r3, r10, 0x7b74
	ctx.r[3].s64 = ctx.r[10].s64 + 31604;
	// 8260D264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D268: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D284: 4BE59B9D  bl 0x82466e20
	ctx.lr = 0x8260D288;
	sub_82466E20(ctx, base);
	// 8260D288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D298 size=112
    let mut pc: u32 = 0x8260D298;
    'dispatch: loop {
        match pc {
            0x8260D298 => {
    //   block [0x8260D298..0x8260D308)
	// 8260D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D2A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D2AC: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D2B4: 390B0AA0  addi r8, r11, 0xaa0
	ctx.r[8].s64 = ctx.r[11].s64 + 2720;
	// 8260D2B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260D2BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8260D2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D2C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D2D0: 386A7BA4  addi r3, r10, 0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 31652;
	// 8260D2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D2F4: 4BE59B2D  bl 0x82466e20
	ctx.lr = 0x8260D2F8;
	sub_82466E20(ctx, base);
	// 8260D2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D308 size=108
    let mut pc: u32 = 0x8260D308;
    'dispatch: loop {
        match pc {
            0x8260D308 => {
    //   block [0x8260D308..0x8260D374)
	// 8260D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D314: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D31C: 38EB0AB8  addi r7, r11, 0xab8
	ctx.r[7].s64 = ctx.r[11].s64 + 2744;
	// 8260D320: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260D324: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8260D328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D32C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D338: 386A7BD4  addi r3, r10, 0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31700;
	// 8260D33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D360: 4BE59AC1  bl 0x82466e20
	ctx.lr = 0x8260D364;
	sub_82466E20(ctx, base);
	// 8260D364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D378 size=116
    let mut pc: u32 = 0x8260D378;
    'dispatch: loop {
        match pc {
            0x8260D378 => {
    //   block [0x8260D378..0x8260D3EC)
	// 8260D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D384: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D388: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8260D38C: 390A0B48  addi r8, r10, 0xb48
	ctx.r[8].s64 = ctx.r[10].s64 + 2888;
	// 8260D390: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260D398: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D39C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D3A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D3A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D3AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8260D3B0: 396BE068  addi r11, r11, -0x1f98
	ctx.r[11].s64 = ctx.r[11].s64 + -8088;
	// 8260D3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D3B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D3BC: 386A7C04  addi r3, r10, 0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + 31748;
	// 8260D3C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260D3C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D3C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260D3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D3D8: 4BE59A49  bl 0x82466e20
	ctx.lr = 0x8260D3DC;
	sub_82466E20(ctx, base);
	// 8260D3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D3F0 size=108
    let mut pc: u32 = 0x8260D3F0;
    'dispatch: loop {
        match pc {
            0x8260D3F0 => {
    //   block [0x8260D3F0..0x8260D45C)
	// 8260D3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D3FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D404: 38EB0C20  addi r7, r11, 0xc20
	ctx.r[7].s64 = ctx.r[11].s64 + 3104;
	// 8260D408: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260D40C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8260D410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D420: 386A7C34  addi r3, r10, 0x7c34
	ctx.r[3].s64 = ctx.r[10].s64 + 31796;
	// 8260D424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D448: 4BE599D9  bl 0x82466e20
	ctx.lr = 0x8260D44C;
	sub_82466E20(ctx, base);
	// 8260D44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D460 size=112
    let mut pc: u32 = 0x8260D460;
    'dispatch: loop {
        match pc {
            0x8260D460 => {
    //   block [0x8260D460..0x8260D4D0)
	// 8260D460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D46C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D470: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D474: 38AA7A84  addi r5, r10, 0x7a84
	ctx.r[5].s64 = ctx.r[10].s64 + 31364;
	// 8260D478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D47C: 390B0C68  addi r8, r11, 0xc68
	ctx.r[8].s64 = ctx.r[11].s64 + 3176;
	// 8260D480: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260D484: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8260D488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D48C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D498: 386A7C64  addi r3, r10, 0x7c64
	ctx.r[3].s64 = ctx.r[10].s64 + 31844;
	// 8260D49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D4BC: 4BE59965  bl 0x82466e20
	ctx.lr = 0x8260D4C0;
	sub_82466E20(ctx, base);
	// 8260D4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D4D0 size=112
    let mut pc: u32 = 0x8260D4D0;
    'dispatch: loop {
        match pc {
            0x8260D4D0 => {
    //   block [0x8260D4D0..0x8260D540)
	// 8260D4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D4DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D4E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D4E4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D4EC: 390B0CE0  addi r8, r11, 0xce0
	ctx.r[8].s64 = ctx.r[11].s64 + 3296;
	// 8260D4F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D4F4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8260D4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D4FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D508: 386A7C94  addi r3, r10, 0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + 31892;
	// 8260D50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D52C: 4BE598F5  bl 0x82466e20
	ctx.lr = 0x8260D530;
	sub_82466E20(ctx, base);
	// 8260D530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D540 size=108
    let mut pc: u32 = 0x8260D540;
    'dispatch: loop {
        match pc {
            0x8260D540 => {
    //   block [0x8260D540..0x8260D5AC)
	// 8260D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D54C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D554: 38EB0D10  addi r7, r11, 0xd10
	ctx.r[7].s64 = ctx.r[11].s64 + 3344;
	// 8260D558: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260D55C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8260D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D570: 386A7CC4  addi r3, r10, 0x7cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31940;
	// 8260D574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D598: 4BE59889  bl 0x82466e20
	ctx.lr = 0x8260D59C;
	sub_82466E20(ctx, base);
	// 8260D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D5B0 size=108
    let mut pc: u32 = 0x8260D5B0;
    'dispatch: loop {
        match pc {
            0x8260D5B0 => {
    //   block [0x8260D5B0..0x8260D61C)
	// 8260D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D5BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D5C4: 38EB0D70  addi r7, r11, 0xd70
	ctx.r[7].s64 = ctx.r[11].s64 + 3440;
	// 8260D5C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8260D5CC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8260D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D5D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D5D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D5E0: 386A7CF4  addi r3, r10, 0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31988;
	// 8260D5E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D608: 4BE59819  bl 0x82466e20
	ctx.lr = 0x8260D60C;
	sub_82466E20(ctx, base);
	// 8260D60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D620 size=112
    let mut pc: u32 = 0x8260D620;
    'dispatch: loop {
        match pc {
            0x8260D620 => {
    //   block [0x8260D620..0x8260D690)
	// 8260D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D62C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D630: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D634: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D63C: 390B0DE8  addi r8, r11, 0xde8
	ctx.r[8].s64 = ctx.r[11].s64 + 3560;
	// 8260D640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260D644: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8260D648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D64C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D658: 386A7D24  addi r3, r10, 0x7d24
	ctx.r[3].s64 = ctx.r[10].s64 + 32036;
	// 8260D65C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D67C: 4BE597A5  bl 0x82466e20
	ctx.lr = 0x8260D680;
	sub_82466E20(ctx, base);
	// 8260D680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260D690 size=24
    let mut pc: u32 = 0x8260D690;
    'dispatch: loop {
        match pc {
            0x8260D690 => {
    //   block [0x8260D690..0x8260D6A8)
	// 8260D690: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D694: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D698: 394A3FF8  addi r10, r10, 0x3ff8
	ctx.r[10].s64 = ctx.r[10].s64 + 16376;
	// 8260D69C: 816B0A54  lwz r11, 0xa54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2644 as u32) ) } as u64;
	// 8260D6A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260D6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D6A8 size=116
    let mut pc: u32 = 0x8260D6A8;
    'dispatch: loop {
        match pc {
            0x8260D6A8 => {
    //   block [0x8260D6A8..0x8260D71C)
	// 8260D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D6B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D6BC: 390B3FF8  addi r8, r11, 0x3ff8
	ctx.r[8].s64 = ctx.r[11].s64 + 16376;
	// 8260D6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D6C4: 392AE0CC  addi r9, r10, -0x1f34
	ctx.r[9].s64 = ctx.r[10].s64 + -7988;
	// 8260D6C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D6CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260D6D0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260D6D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D6DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D6EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260D6F0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8260D6F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D6F8: 386B7D54  addi r3, r11, 0x7d54
	ctx.r[3].s64 = ctx.r[11].s64 + 32084;
	// 8260D6FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D700: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D708: 4BE59719  bl 0x82466e20
	ctx.lr = 0x8260D70C;
	sub_82466E20(ctx, base);
	// 8260D70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D720 size=112
    let mut pc: u32 = 0x8260D720;
    'dispatch: loop {
        match pc {
            0x8260D720 => {
    //   block [0x8260D720..0x8260D790)
	// 8260D720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D72C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D730: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D734: 38AA7D54  addi r5, r10, 0x7d54
	ctx.r[5].s64 = ctx.r[10].s64 + 32084;
	// 8260D738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D73C: 390B0E30  addi r8, r11, 0xe30
	ctx.r[8].s64 = ctx.r[11].s64 + 3632;
	// 8260D740: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D744: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8260D748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D758: 386A7D84  addi r3, r10, 0x7d84
	ctx.r[3].s64 = ctx.r[10].s64 + 32132;
	// 8260D75C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D77C: 4BE596A5  bl 0x82466e20
	ctx.lr = 0x8260D780;
	sub_82466E20(ctx, base);
	// 8260D780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260D790 size=24
    let mut pc: u32 = 0x8260D790;
    'dispatch: loop {
        match pc {
            0x8260D790 => {
    //   block [0x8260D790..0x8260D7A8)
	// 8260D790: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D794: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260D798: 394A4010  addi r10, r10, 0x4010
	ctx.r[10].s64 = ctx.r[10].s64 + 16400;
	// 8260D79C: 816B0E60  lwz r11, 0xe60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3680 as u32) ) } as u64;
	// 8260D7A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260D7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D7A8 size=116
    let mut pc: u32 = 0x8260D7A8;
    'dispatch: loop {
        match pc {
            0x8260D7A8 => {
    //   block [0x8260D7A8..0x8260D81C)
	// 8260D7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D7B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D7B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260D7BC: 390B4010  addi r8, r11, 0x4010
	ctx.r[8].s64 = ctx.r[11].s64 + 16400;
	// 8260D7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D7C4: 392AE108  addi r9, r10, -0x1ef8
	ctx.r[9].s64 = ctx.r[10].s64 + -7928;
	// 8260D7C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D7CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260D7D0: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D7D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D7DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D7EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260D7F0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8260D7F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260D7F8: 386B7DB4  addi r3, r11, 0x7db4
	ctx.r[3].s64 = ctx.r[11].s64 + 32180;
	// 8260D7FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260D800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D808: 4BE59619  bl 0x82466e20
	ctx.lr = 0x8260D80C;
	sub_82466E20(ctx, base);
	// 8260D80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D820 size=112
    let mut pc: u32 = 0x8260D820;
    'dispatch: loop {
        match pc {
            0x8260D820 => {
    //   block [0x8260D820..0x8260D890)
	// 8260D820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D830: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D834: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D83C: 390B0E68  addi r8, r11, 0xe68
	ctx.r[8].s64 = ctx.r[11].s64 + 3688;
	// 8260D840: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260D844: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8260D848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D858: 386A7DE4  addi r3, r10, 0x7de4
	ctx.r[3].s64 = ctx.r[10].s64 + 32228;
	// 8260D85C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D87C: 4BE595A5  bl 0x82466e20
	ctx.lr = 0x8260D880;
	sub_82466E20(ctx, base);
	// 8260D880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D890 size=112
    let mut pc: u32 = 0x8260D890;
    'dispatch: loop {
        match pc {
            0x8260D890 => {
    //   block [0x8260D890..0x8260D900)
	// 8260D890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D89C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D8A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D8A4: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D8AC: 390B0EC8  addi r8, r11, 0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + 3784;
	// 8260D8B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260D8B4: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8260D8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D8BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D8C8: 386A7E14  addi r3, r10, 0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + 32276;
	// 8260D8CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D8EC: 4BE59535  bl 0x82466e20
	ctx.lr = 0x8260D8F0;
	sub_82466E20(ctx, base);
	// 8260D8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D900 size=112
    let mut pc: u32 = 0x8260D900;
    'dispatch: loop {
        match pc {
            0x8260D900 => {
    //   block [0x8260D900..0x8260D970)
	// 8260D900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D910: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D914: 38AA7D84  addi r5, r10, 0x7d84
	ctx.r[5].s64 = ctx.r[10].s64 + 32132;
	// 8260D918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D91C: 390B0EF8  addi r8, r11, 0xef8
	ctx.r[8].s64 = ctx.r[11].s64 + 3832;
	// 8260D920: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260D924: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8260D928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260D934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D938: 386A7E44  addi r3, r10, 0x7e44
	ctx.r[3].s64 = ctx.r[10].s64 + 32324;
	// 8260D93C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260D940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D94C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D95C: 4BE594C5  bl 0x82466e20
	ctx.lr = 0x8260D960;
	sub_82466E20(ctx, base);
	// 8260D960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D970 size=108
    let mut pc: u32 = 0x8260D970;
    'dispatch: loop {
        match pc {
            0x8260D970 => {
    //   block [0x8260D970..0x8260D9DC)
	// 8260D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D97C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D984: 38EB0F40  addi r7, r11, 0xf40
	ctx.r[7].s64 = ctx.r[11].s64 + 3904;
	// 8260D988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260D98C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8260D990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260D994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260D99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260D9A0: 386A7E74  addi r3, r10, 0x7e74
	ctx.r[3].s64 = ctx.r[10].s64 + 32372;
	// 8260D9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260D9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260D9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260D9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260D9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260D9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260D9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260D9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260D9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260D9C8: 4BE59459  bl 0x82466e20
	ctx.lr = 0x8260D9CC;
	sub_82466E20(ctx, base);
	// 8260D9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260D9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260D9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260D9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260D9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260D9E0 size=112
    let mut pc: u32 = 0x8260D9E0;
    'dispatch: loop {
        match pc {
            0x8260D9E0 => {
    //   block [0x8260D9E0..0x8260DA50)
	// 8260D9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260D9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260D9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260D9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260D9F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260D9F4: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260D9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260D9FC: 390B0F70  addi r8, r11, 0xf70
	ctx.r[8].s64 = ctx.r[11].s64 + 3952;
	// 8260DA00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260DA04: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8260DA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DA0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DA18: 386A7EA4  addi r3, r10, 0x7ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 32420;
	// 8260DA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DA3C: 4BE593E5  bl 0x82466e20
	ctx.lr = 0x8260DA40;
	sub_82466E20(ctx, base);
	// 8260DA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DA50 size=108
    let mut pc: u32 = 0x8260DA50;
    'dispatch: loop {
        match pc {
            0x8260DA50 => {
    //   block [0x8260DA50..0x8260DABC)
	// 8260DA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DA5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DA64: 38EB0F88  addi r7, r11, 0xf88
	ctx.r[7].s64 = ctx.r[11].s64 + 3976;
	// 8260DA68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260DA6C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 8260DA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DA80: 386A7ED4  addi r3, r10, 0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 32468;
	// 8260DA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DAA8: 4BE59379  bl 0x82466e20
	ctx.lr = 0x8260DAAC;
	sub_82466E20(ctx, base);
	// 8260DAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260DAC0 size=24
    let mut pc: u32 = 0x8260DAC0;
    'dispatch: loop {
        match pc {
            0x8260DAC0 => {
    //   block [0x8260DAC0..0x8260DAD8)
	// 8260DAC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DAC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260DAC8: 394A4088  addi r10, r10, 0x4088
	ctx.r[10].s64 = ctx.r[10].s64 + 16520;
	// 8260DACC: 816B0E64  lwz r11, 0xe64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3684 as u32) ) } as u64;
	// 8260DAD0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8260DAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DAD8 size=108
    let mut pc: u32 = 0x8260DAD8;
    'dispatch: loop {
        match pc {
            0x8260DAD8 => {
    //   block [0x8260DAD8..0x8260DB44)
	// 8260DAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DAE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DAEC: 38EB4088  addi r7, r11, 0x4088
	ctx.r[7].s64 = ctx.r[11].s64 + 16520;
	// 8260DAF0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260DAF4: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 8260DAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DAFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DB00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DB08: 386A7F04  addi r3, r10, 0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + 32516;
	// 8260DB0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DB30: 4BE592F1  bl 0x82466e20
	ctx.lr = 0x8260DB34;
	sub_82466E20(ctx, base);
	// 8260DB34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DB48 size=116
    let mut pc: u32 = 0x8260DB48;
    'dispatch: loop {
        match pc {
            0x8260DB48 => {
    //   block [0x8260DB48..0x8260DBBC)
	// 8260DB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DB54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260DB58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DB5C: 392BE13C  addi r9, r11, -0x1ec4
	ctx.r[9].s64 = ctx.r[11].s64 + -7876;
	// 8260DB60: 38AA8384  addi r5, r10, -0x7c7c
	ctx.r[5].s64 = ctx.r[10].s64 + -31868;
	// 8260DB64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DB68: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260DB6C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8260DB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DB74: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8260DB78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DB7C: 396B0FD0  addi r11, r11, 0xfd0
	ctx.r[11].s64 = ctx.r[11].s64 + 4048;
	// 8260DB80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260DB84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DB88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260DB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DB90: 386A7F34  addi r3, r10, 0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + 32564;
	// 8260DB94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DB98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260DB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DBA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260DBA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DBA8: 4BE59279  bl 0x82466e20
	ctx.lr = 0x8260DBAC;
	sub_82466E20(ctx, base);
	// 8260DBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DBC0 size=100
    let mut pc: u32 = 0x8260DBC0;
    'dispatch: loop {
        match pc {
            0x8260DBC0 => {
    //   block [0x8260DBC0..0x8260DC24)
	// 8260DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DBD4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260DBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DBE0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8260DBE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DBF4: 386A7F64  addi r3, r10, 0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + 32612;
	// 8260DBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DC10: 4BE59211  bl 0x82466e20
	ctx.lr = 0x8260DC14;
	sub_82466E20(ctx, base);
	// 8260DC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DC28 size=100
    let mut pc: u32 = 0x8260DC28;
    'dispatch: loop {
        match pc {
            0x8260DC28 => {
    //   block [0x8260DC28..0x8260DC8C)
	// 8260DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DC3C: 38AA7FF4  addi r5, r10, 0x7ff4
	ctx.r[5].s64 = ctx.r[10].s64 + 32756;
	// 8260DC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DC48: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8260DC4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DC5C: 386A7F94  addi r3, r10, 0x7f94
	ctx.r[3].s64 = ctx.r[10].s64 + 32660;
	// 8260DC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DC78: 4BE591A9  bl 0x82466e20
	ctx.lr = 0x8260DC7C;
	sub_82466E20(ctx, base);
	// 8260DC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DC90 size=100
    let mut pc: u32 = 0x8260DC90;
    'dispatch: loop {
        match pc {
            0x8260DC90 => {
    //   block [0x8260DC90..0x8260DCF4)
	// 8260DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DC9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DCA4: 38AA7F34  addi r5, r10, 0x7f34
	ctx.r[5].s64 = ctx.r[10].s64 + 32564;
	// 8260DCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DCB0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8260DCB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DCC4: 386A7FC4  addi r3, r10, 0x7fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32708;
	// 8260DCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DCE0: 4BE59141  bl 0x82466e20
	ctx.lr = 0x8260DCE4;
	sub_82466E20(ctx, base);
	// 8260DCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DCF8 size=104
    let mut pc: u32 = 0x8260DCF8;
    'dispatch: loop {
        match pc {
            0x8260DCF8 => {
    //   block [0x8260DCF8..0x8260DD60)
	// 8260DCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DD04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260DD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DD0C: 392AE1B8  addi r9, r10, -0x1e48
	ctx.r[9].s64 = ctx.r[10].s64 + -7752;
	// 8260DD10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DD18: 38AA7F64  addi r5, r10, 0x7f64
	ctx.r[5].s64 = ctx.r[10].s64 + 32612;
	// 8260DD1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DD2C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8260DD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DD34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DD44: 386A7FF4  addi r3, r10, 0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 32756;
	// 8260DD48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DD4C: 4BE590D5  bl 0x82466e20
	ctx.lr = 0x8260DD50;
	sub_82466E20(ctx, base);
	// 8260DD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DD60 size=108
    let mut pc: u32 = 0x8260DD60;
    'dispatch: loop {
        match pc {
            0x8260DD60 => {
    //   block [0x8260DD60..0x8260DDCC)
	// 8260DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DD6C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DD74: 38EB116C  addi r7, r11, 0x116c
	ctx.r[7].s64 = ctx.r[11].s64 + 4460;
	// 8260DD78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260DD7C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8260DD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260DD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DD90: 386A8024  addi r3, r10, -0x7fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -32732;
	// 8260DD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260DD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260DDB8: 4BE59069  bl 0x82466e20
	ctx.lr = 0x8260DDBC;
	sub_82466E20(ctx, base);
	// 8260DDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DDD0 size=112
    let mut pc: u32 = 0x8260DDD0;
    'dispatch: loop {
        match pc {
            0x8260DDD0 => {
    //   block [0x8260DDD0..0x8260DE40)
	// 8260DDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DDE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DDE4: 38AA7FF4  addi r5, r10, 0x7ff4
	ctx.r[5].s64 = ctx.r[10].s64 + 32756;
	// 8260DDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DDEC: 390B11A0  addi r8, r11, 0x11a0
	ctx.r[8].s64 = ctx.r[11].s64 + 4512;
	// 8260DDF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260DDF4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8260DDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DDFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DE00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DE08: 386A8054  addi r3, r10, -0x7fac
	ctx.r[3].s64 = ctx.r[10].s64 + -32684;
	// 8260DE0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DE2C: 4BE58FF5  bl 0x82466e20
	ctx.lr = 0x8260DE30;
	sub_82466E20(ctx, base);
	// 8260DE30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260DE40 size=24
    let mut pc: u32 = 0x8260DE40;
    'dispatch: loop {
        match pc {
            0x8260DE40 => {
    //   block [0x8260DE40..0x8260DE58)
	// 8260DE40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DE44: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260DE48: 394A40E8  addi r10, r10, 0x40e8
	ctx.r[10].s64 = ctx.r[10].s64 + 16616;
	// 8260DE4C: 816B119C  lwz r11, 0x119c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4508 as u32) ) } as u64;
	// 8260DE50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DE58 size=116
    let mut pc: u32 = 0x8260DE58;
    'dispatch: loop {
        match pc {
            0x8260DE58 => {
    //   block [0x8260DE58..0x8260DECC)
	// 8260DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DE64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DE68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260DE6C: 390B40E8  addi r8, r11, 0x40e8
	ctx.r[8].s64 = ctx.r[11].s64 + 16616;
	// 8260DE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DE74: 392AE220  addi r9, r10, -0x1de0
	ctx.r[9].s64 = ctx.r[10].s64 + -7648;
	// 8260DE78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260DE7C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260DE80: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260DE84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DE8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DE9C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260DEA0: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8260DEA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260DEA8: 386B8084  addi r3, r11, -0x7f7c
	ctx.r[3].s64 = ctx.r[11].s64 + -32636;
	// 8260DEAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260DEB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DEB8: 4BE58F69  bl 0x82466e20
	ctx.lr = 0x8260DEBC;
	sub_82466E20(ctx, base);
	// 8260DEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DED0 size=100
    let mut pc: u32 = 0x8260DED0;
    'dispatch: loop {
        match pc {
            0x8260DED0 => {
    //   block [0x8260DED0..0x8260DF34)
	// 8260DED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DEDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DEE4: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260DEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DEF0: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8260DEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DF04: 386A80B4  addi r3, r10, -0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32588;
	// 8260DF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DF0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DF10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DF14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DF18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DF20: 4BE58F01  bl 0x82466e20
	ctx.lr = 0x8260DF24;
	sub_82466E20(ctx, base);
	// 8260DF24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DF38 size=100
    let mut pc: u32 = 0x8260DF38;
    'dispatch: loop {
        match pc {
            0x8260DF38 => {
    //   block [0x8260DF38..0x8260DF9C)
	// 8260DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DF4C: 38AA8114  addi r5, r10, -0x7eec
	ctx.r[5].s64 = ctx.r[10].s64 + -32492;
	// 8260DF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DF58: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8260DF5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DF6C: 386A80E4  addi r3, r10, -0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32540;
	// 8260DF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DF74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DF78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260DF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DF80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260DF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DF88: 4BE58E99  bl 0x82466e20
	ctx.lr = 0x8260DF8C;
	sub_82466E20(ctx, base);
	// 8260DF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260DF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260DF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260DF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260DFA0 size=112
    let mut pc: u32 = 0x8260DFA0;
    'dispatch: loop {
        match pc {
            0x8260DFA0 => {
    //   block [0x8260DFA0..0x8260E010)
	// 8260DFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260DFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260DFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260DFAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DFB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260DFB4: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260DFB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260DFBC: 390B1248  addi r8, r11, 0x1248
	ctx.r[8].s64 = ctx.r[11].s64 + 4680;
	// 8260DFC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260DFC4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8260DFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260DFCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260DFD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260DFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260DFD8: 386A8114  addi r3, r10, -0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + -32492;
	// 8260DFDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260DFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260DFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260DFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260DFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260DFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260DFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260DFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260DFFC: 4BE58E25  bl 0x82466e20
	ctx.lr = 0x8260E000;
	sub_82466E20(ctx, base);
	// 8260E000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E010 size=100
    let mut pc: u32 = 0x8260E010;
    'dispatch: loop {
        match pc {
            0x8260E010 => {
    //   block [0x8260E010..0x8260E074)
	// 8260E010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E01C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E024: 38AA8114  addi r5, r10, -0x7eec
	ctx.r[5].s64 = ctx.r[10].s64 + -32492;
	// 8260E028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E030: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8260E034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E044: 386A8144  addi r3, r10, -0x7ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -32444;
	// 8260E048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E04C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E060: 4BE58DC1  bl 0x82466e20
	ctx.lr = 0x8260E064;
	sub_82466E20(ctx, base);
	// 8260E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E078 size=100
    let mut pc: u32 = 0x8260E078;
    'dispatch: loop {
        match pc {
            0x8260E078 => {
    //   block [0x8260E078..0x8260E0DC)
	// 8260E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E08C: 38AA8084  addi r5, r10, -0x7f7c
	ctx.r[5].s64 = ctx.r[10].s64 + -32636;
	// 8260E090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E098: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8260E09C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E0AC: 386A8174  addi r3, r10, -0x7e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32396;
	// 8260E0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E0B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E0B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E0C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E0C8: 4BE58D59  bl 0x82466e20
	ctx.lr = 0x8260E0CC;
	sub_82466E20(ctx, base);
	// 8260E0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E0E0 size=100
    let mut pc: u32 = 0x8260E0E0;
    'dispatch: loop {
        match pc {
            0x8260E0E0 => {
    //   block [0x8260E0E0..0x8260E144)
	// 8260E0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E0F4: 38AA80B4  addi r5, r10, -0x7f4c
	ctx.r[5].s64 = ctx.r[10].s64 + -32588;
	// 8260E0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E100: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8260E104: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E114: 386A81A4  addi r3, r10, -0x7e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32348;
	// 8260E118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E11C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E120: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E130: 4BE58CF1  bl 0x82466e20
	ctx.lr = 0x8260E134;
	sub_82466E20(ctx, base);
	// 8260E134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E148 size=100
    let mut pc: u32 = 0x8260E148;
    'dispatch: loop {
        match pc {
            0x8260E148 => {
    //   block [0x8260E148..0x8260E1AC)
	// 8260E148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E15C: 38AA8174  addi r5, r10, -0x7e8c
	ctx.r[5].s64 = ctx.r[10].s64 + -32396;
	// 8260E160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E168: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8260E16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E17C: 386A81D4  addi r3, r10, -0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32300;
	// 8260E180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E188: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E190: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E198: 4BE58C89  bl 0x82466e20
	ctx.lr = 0x8260E19C;
	sub_82466E20(ctx, base);
	// 8260E19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E1B0 size=100
    let mut pc: u32 = 0x8260E1B0;
    'dispatch: loop {
        match pc {
            0x8260E1B0 => {
    //   block [0x8260E1B0..0x8260E214)
	// 8260E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E1BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E1C4: 38AA80B4  addi r5, r10, -0x7f4c
	ctx.r[5].s64 = ctx.r[10].s64 + -32588;
	// 8260E1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E1D0: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8260E1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E1E4: 386A8204  addi r3, r10, -0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -32252;
	// 8260E1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E1F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E1F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E200: 4BE58C21  bl 0x82466e20
	ctx.lr = 0x8260E204;
	sub_82466E20(ctx, base);
	// 8260E204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E218 size=112
    let mut pc: u32 = 0x8260E218;
    'dispatch: loop {
        match pc {
            0x8260E218 => {
    //   block [0x8260E218..0x8260E288)
	// 8260E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E228: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E22C: 38AA8294  addi r5, r10, -0x7d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -32108;
	// 8260E230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E234: 390B1278  addi r8, r11, 0x1278
	ctx.r[8].s64 = ctx.r[11].s64 + 4728;
	// 8260E238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E23C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8260E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E250: 386A8234  addi r3, r10, -0x7dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -32204;
	// 8260E254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E274: 4BE58BAD  bl 0x82466e20
	ctx.lr = 0x8260E278;
	sub_82466E20(ctx, base);
	// 8260E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E288 size=112
    let mut pc: u32 = 0x8260E288;
    'dispatch: loop {
        match pc {
            0x8260E288 => {
    //   block [0x8260E288..0x8260E2F8)
	// 8260E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E298: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E29C: 38AA82C4  addi r5, r10, -0x7d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32060;
	// 8260E2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E2A4: 390B12A8  addi r8, r11, 0x12a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4776;
	// 8260E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E2AC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8260E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E2B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E2C0: 386A8264  addi r3, r10, -0x7d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32156;
	// 8260E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E2E4: 4BE58B3D  bl 0x82466e20
	ctx.lr = 0x8260E2E8;
	sub_82466E20(ctx, base);
	// 8260E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E2F8 size=112
    let mut pc: u32 = 0x8260E2F8;
    'dispatch: loop {
        match pc {
            0x8260E2F8 => {
    //   block [0x8260E2F8..0x8260E368)
	// 8260E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E30C: 38AA8384  addi r5, r10, -0x7c7c
	ctx.r[5].s64 = ctx.r[10].s64 + -31868;
	// 8260E310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E314: 390B12C0  addi r8, r11, 0x12c0
	ctx.r[8].s64 = ctx.r[11].s64 + 4800;
	// 8260E318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E31C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8260E320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E330: 386A8294  addi r3, r10, -0x7d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32108;
	// 8260E334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E354: 4BE58ACD  bl 0x82466e20
	ctx.lr = 0x8260E358;
	sub_82466E20(ctx, base);
	// 8260E358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E368 size=112
    let mut pc: u32 = 0x8260E368;
    'dispatch: loop {
        match pc {
            0x8260E368 => {
    //   block [0x8260E368..0x8260E3D8)
	// 8260E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E37C: 38AA8294  addi r5, r10, -0x7d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -32108;
	// 8260E380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E384: 390B12F0  addi r8, r11, 0x12f0
	ctx.r[8].s64 = ctx.r[11].s64 + 4848;
	// 8260E388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E38C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8260E390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E3A0: 386A82C4  addi r3, r10, -0x7d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32060;
	// 8260E3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E3C4: 4BE58A5D  bl 0x82466e20
	ctx.lr = 0x8260E3C8;
	sub_82466E20(ctx, base);
	// 8260E3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E3D8 size=112
    let mut pc: u32 = 0x8260E3D8;
    'dispatch: loop {
        match pc {
            0x8260E3D8 => {
    //   block [0x8260E3D8..0x8260E448)
	// 8260E3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E3E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E3EC: 38AA82C4  addi r5, r10, -0x7d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -32060;
	// 8260E3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E3F4: 390B1308  addi r8, r11, 0x1308
	ctx.r[8].s64 = ctx.r[11].s64 + 4872;
	// 8260E3F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E3FC: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8260E400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E410: 386A82F4  addi r3, r10, -0x7d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32012;
	// 8260E414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E434: 4BE589ED  bl 0x82466e20
	ctx.lr = 0x8260E438;
	sub_82466E20(ctx, base);
	// 8260E438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E448 size=116
    let mut pc: u32 = 0x8260E448;
    'dispatch: loop {
        match pc {
            0x8260E448 => {
    //   block [0x8260E448..0x8260E4BC)
	// 8260E448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E454: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E458: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260E45C: 390A1320  addi r8, r10, 0x1320
	ctx.r[8].s64 = ctx.r[10].s64 + 4896;
	// 8260E460: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E464: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260E468: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E46C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E470: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E47C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8260E480: 396BE234  addi r11, r11, -0x1dcc
	ctx.r[11].s64 = ctx.r[11].s64 + -7628;
	// 8260E484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E48C: 386A8324  addi r3, r10, -0x7cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -31964;
	// 8260E490: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260E494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E498: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260E49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E4A8: 4BE58979  bl 0x82466e20
	ctx.lr = 0x8260E4AC;
	sub_82466E20(ctx, base);
	// 8260E4AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E4B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E4B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260E4C0 size=48
    let mut pc: u32 = 0x8260E4C0;
    'dispatch: loop {
        match pc {
            0x8260E4C0 => {
    //   block [0x8260E4C0..0x8260E4F0)
	// 8260E4C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E4C4: 814B13D4  lwz r10, 0x13d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5076 as u32) ) } as u64;
	// 8260E4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E4CC: 396B41A8  addi r11, r11, 0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16808;
	// 8260E4D0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8260E4D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E4D8: 814A13D0  lwz r10, 0x13d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5072 as u32) ) } as u64;
	// 8260E4DC: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8260E4E0: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260E4E4: 814A13CC  lwz r10, 0x13cc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5068 as u32) ) } as u64;
	// 8260E4E8: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 8260E4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E4F0 size=116
    let mut pc: u32 = 0x8260E4F0;
    'dispatch: loop {
        match pc {
            0x8260E4F0 => {
    //   block [0x8260E4F0..0x8260E564)
	// 8260E4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E4FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260E500: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E504: 392BE308  addi r9, r11, -0x1cf8
	ctx.r[9].s64 = ctx.r[11].s64 + -7416;
	// 8260E508: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E50C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E510: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8260E514: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 8260E518: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E51C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8260E520: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E524: 396B41A8  addi r11, r11, 0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16808;
	// 8260E528: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260E52C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E530: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260E534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E538: 386A8354  addi r3, r10, -0x7cac
	ctx.r[3].s64 = ctx.r[10].s64 + -31916;
	// 8260E53C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8260E540: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260E544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E548: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260E54C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E550: 4BE588D1  bl 0x82466e20
	ctx.lr = 0x8260E554;
	sub_82466E20(ctx, base);
	// 8260E554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E568 size=116
    let mut pc: u32 = 0x8260E568;
    'dispatch: loop {
        match pc {
            0x8260E568 => {
    //   block [0x8260E568..0x8260E5DC)
	// 8260E568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E574: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E57C: 390B13E0  addi r8, r11, 0x13e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5088;
	// 8260E580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E584: 392AE480  addi r9, r10, -0x1b80
	ctx.r[9].s64 = ctx.r[10].s64 + -7040;
	// 8260E588: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E58C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8260E590: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E594: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E59C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E5AC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260E5B0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8260E5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E5B8: 386B8384  addi r3, r11, -0x7c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -31868;
	// 8260E5BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260E5C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E5C8: 4BE58859  bl 0x82466e20
	ctx.lr = 0x8260E5CC;
	sub_82466E20(ctx, base);
	// 8260E5CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E5D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E5D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E5E0 size=112
    let mut pc: u32 = 0x8260E5E0;
    'dispatch: loop {
        match pc {
            0x8260E5E0 => {
    //   block [0x8260E5E0..0x8260E650)
	// 8260E5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E5EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E5F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E5F4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E5FC: 390B1470  addi r8, r11, 0x1470
	ctx.r[8].s64 = ctx.r[11].s64 + 5232;
	// 8260E600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E604: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8260E608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E60C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E618: 386A83B4  addi r3, r10, -0x7c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31820;
	// 8260E61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E63C: 4BE587E5  bl 0x82466e20
	ctx.lr = 0x8260E640;
	sub_82466E20(ctx, base);
	// 8260E640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E650 size=112
    let mut pc: u32 = 0x8260E650;
    'dispatch: loop {
        match pc {
            0x8260E650 => {
    //   block [0x8260E650..0x8260E6C0)
	// 8260E650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E660: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E664: 38AA63D4  addi r5, r10, 0x63d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25556;
	// 8260E668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E66C: 390B1488  addi r8, r11, 0x1488
	ctx.r[8].s64 = ctx.r[11].s64 + 5256;
	// 8260E670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260E674: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8260E678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E67C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E688: 386A83E4  addi r3, r10, -0x7c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31772;
	// 8260E68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E6AC: 4BE58775  bl 0x82466e20
	ctx.lr = 0x8260E6B0;
	sub_82466E20(ctx, base);
	// 8260E6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E6C0 size=108
    let mut pc: u32 = 0x8260E6C0;
    'dispatch: loop {
        match pc {
            0x8260E6C0 => {
    //   block [0x8260E6C0..0x8260E72C)
	// 8260E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E6CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E6D4: 38EB14A0  addi r7, r11, 0x14a0
	ctx.r[7].s64 = ctx.r[11].s64 + 5280;
	// 8260E6D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E6DC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8260E6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E6E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E6E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E6F0: 386A8414  addi r3, r10, -0x7bec
	ctx.r[3].s64 = ctx.r[10].s64 + -31724;
	// 8260E6F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E718: 4BE58709  bl 0x82466e20
	ctx.lr = 0x8260E71C;
	sub_82466E20(ctx, base);
	// 8260E71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E730 size=112
    let mut pc: u32 = 0x8260E730;
    'dispatch: loop {
        match pc {
            0x8260E730 => {
    //   block [0x8260E730..0x8260E7A0)
	// 8260E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E73C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E740: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E744: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E74C: 390B14B8  addi r8, r11, 0x14b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5304;
	// 8260E750: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260E754: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8260E758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E768: 386A8444  addi r3, r10, -0x7bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31676;
	// 8260E76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E78C: 4BE58695  bl 0x82466e20
	ctx.lr = 0x8260E790;
	sub_82466E20(ctx, base);
	// 8260E790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E7A0 size=108
    let mut pc: u32 = 0x8260E7A0;
    'dispatch: loop {
        match pc {
            0x8260E7A0 => {
    //   block [0x8260E7A0..0x8260E80C)
	// 8260E7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E7AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E7B4: 38EB1500  addi r7, r11, 0x1500
	ctx.r[7].s64 = ctx.r[11].s64 + 5376;
	// 8260E7B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260E7BC: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8260E7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E7C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E7D0: 386A8474  addi r3, r10, -0x7b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31628;
	// 8260E7D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E7F8: 4BE58629  bl 0x82466e20
	ctx.lr = 0x8260E7FC;
	sub_82466E20(ctx, base);
	// 8260E7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E810 size=108
    let mut pc: u32 = 0x8260E810;
    'dispatch: loop {
        match pc {
            0x8260E810 => {
    //   block [0x8260E810..0x8260E87C)
	// 8260E810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E81C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E824: 38EB1530  addi r7, r11, 0x1530
	ctx.r[7].s64 = ctx.r[11].s64 + 5424;
	// 8260E828: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E82C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8260E830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260E83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E840: 386A84A4  addi r3, r10, -0x7b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31580;
	// 8260E844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260E848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E868: 4BE585B9  bl 0x82466e20
	ctx.lr = 0x8260E86C;
	sub_82466E20(ctx, base);
	// 8260E86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E880 size=112
    let mut pc: u32 = 0x8260E880;
    'dispatch: loop {
        match pc {
            0x8260E880 => {
    //   block [0x8260E880..0x8260E8F0)
	// 8260E880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E88C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E890: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E894: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260E898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E89C: 390B1548  addi r8, r11, 0x1548
	ctx.r[8].s64 = ctx.r[11].s64 + 5448;
	// 8260E8A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260E8A4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8260E8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E8AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E8B8: 386A84D4  addi r3, r10, -0x7b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31532;
	// 8260E8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260E8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E8DC: 4BE58545  bl 0x82466e20
	ctx.lr = 0x8260E8E0;
	sub_82466E20(ctx, base);
	// 8260E8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E8F0 size=96
    let mut pc: u32 = 0x8260E8F0;
    'dispatch: loop {
        match pc {
            0x8260E8F0 => {
    //   block [0x8260E8F0..0x8260E950)
	// 8260E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E8FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E904: 388A507C  addi r4, r10, 0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + 20604;
	// 8260E908: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260E910: 386A8504  addi r3, r10, -0x7afc
	ctx.r[3].s64 = ctx.r[10].s64 + -31484;
	// 8260E914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260E918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E91C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260E920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260E934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260E93C: 4BE584E5  bl 0x82466e20
	ctx.lr = 0x8260E940;
	sub_82466E20(ctx, base);
	// 8260E940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E950 size=112
    let mut pc: u32 = 0x8260E950;
    'dispatch: loop {
        match pc {
            0x8260E950 => {
    //   block [0x8260E950..0x8260E9C0)
	// 8260E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E95C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E960: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E964: 392AE4D8  addi r9, r10, -0x1b28
	ctx.r[9].s64 = ctx.r[10].s64 + -6952;
	// 8260E968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E96C: 390B1580  addi r8, r11, 0x1580
	ctx.r[8].s64 = ctx.r[11].s64 + 5504;
	// 8260E970: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260E974: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8260E978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E97C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260E980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260E988: 386A8534  addi r3, r10, -0x7acc
	ctx.r[3].s64 = ctx.r[10].s64 + -31436;
	// 8260E98C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260E990: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260E998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260E9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260E9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260E9AC: 4BE58475  bl 0x82466e20
	ctx.lr = 0x8260E9B0;
	sub_82466E20(ctx, base);
	// 8260E9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260E9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260E9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260E9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260E9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260E9C0 size=116
    let mut pc: u32 = 0x8260E9C0;
    'dispatch: loop {
        match pc {
            0x8260E9C0 => {
    //   block [0x8260E9C0..0x8260EA34)
	// 8260E9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260E9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260E9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260E9CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260E9D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260E9D4: 390B1628  addi r8, r11, 0x1628
	ctx.r[8].s64 = ctx.r[11].s64 + 5672;
	// 8260E9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260E9DC: 392AE4AC  addi r9, r10, -0x1b54
	ctx.r[9].s64 = ctx.r[10].s64 + -6996;
	// 8260E9E0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260E9E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260E9E8: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260E9EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260E9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260E9F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260E9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260E9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EA04: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8260EA08: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8260EA0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260EA10: 386B8564  addi r3, r11, -0x7a9c
	ctx.r[3].s64 = ctx.r[11].s64 + -31388;
	// 8260EA14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260EA18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EA20: 4BE58401  bl 0x82466e20
	ctx.lr = 0x8260EA24;
	sub_82466E20(ctx, base);
	// 8260EA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EA38 size=112
    let mut pc: u32 = 0x8260EA38;
    'dispatch: loop {
        match pc {
            0x8260EA38 => {
    //   block [0x8260EA38..0x8260EAA8)
	// 8260EA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EA44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260EA48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EA4C: 392AE504  addi r9, r10, -0x1afc
	ctx.r[9].s64 = ctx.r[10].s64 + -6908;
	// 8260EA50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EA54: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 8260EA58: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8260EA5C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8260EA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EA64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EA68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EA70: 386A8594  addi r3, r10, -0x7a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31340;
	// 8260EA74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260EA78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260EA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EA94: 4BE5838D  bl 0x82466e20
	ctx.lr = 0x8260EA98;
	sub_82466E20(ctx, base);
	// 8260EA98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EAA8 size=112
    let mut pc: u32 = 0x8260EAA8;
    'dispatch: loop {
        match pc {
            0x8260EAA8 => {
    //   block [0x8260EAA8..0x8260EB18)
	// 8260EAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EAB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EABC: 38AA76C4  addi r5, r10, 0x76c4
	ctx.r[5].s64 = ctx.r[10].s64 + 30404;
	// 8260EAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EAC4: 390B16A0  addi r8, r11, 0x16a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5792;
	// 8260EAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EACC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8260EAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EAD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EAE0: 386A85C4  addi r3, r10, -0x7a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31292;
	// 8260EAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EB04: 4BE5831D  bl 0x82466e20
	ctx.lr = 0x8260EB08;
	sub_82466E20(ctx, base);
	// 8260EB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EB18 size=112
    let mut pc: u32 = 0x8260EB18;
    'dispatch: loop {
        match pc {
            0x8260EB18 => {
    //   block [0x8260EB18..0x8260EB88)
	// 8260EB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EB24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EB28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EB2C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260EB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EB34: 390B16B8  addi r8, r11, 0x16b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5816;
	// 8260EB38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260EB3C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8260EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EB44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EB50: 386A85F4  addi r3, r10, -0x7a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31244;
	// 8260EB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EB74: 4BE582AD  bl 0x82466e20
	ctx.lr = 0x8260EB78;
	sub_82466E20(ctx, base);
	// 8260EB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EB88 size=112
    let mut pc: u32 = 0x8260EB88;
    'dispatch: loop {
        match pc {
            0x8260EB88 => {
    //   block [0x8260EB88..0x8260EBF8)
	// 8260EB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EB94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EB98: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EB9C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260EBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EBA4: 390B1700  addi r8, r11, 0x1700
	ctx.r[8].s64 = ctx.r[11].s64 + 5888;
	// 8260EBA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EBAC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8260EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EBB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EBC0: 386A8624  addi r3, r10, -0x79dc
	ctx.r[3].s64 = ctx.r[10].s64 + -31196;
	// 8260EBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EBE4: 4BE5823D  bl 0x82466e20
	ctx.lr = 0x8260EBE8;
	sub_82466E20(ctx, base);
	// 8260EBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EBF8 size=112
    let mut pc: u32 = 0x8260EBF8;
    'dispatch: loop {
        match pc {
            0x8260EBF8 => {
    //   block [0x8260EBF8..0x8260EC68)
	// 8260EBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EC04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EC08: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EC0C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260EC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EC14: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 8260EC18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EC1C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8260EC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EC24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EC30: 386A8654  addi r3, r10, -0x79ac
	ctx.r[3].s64 = ctx.r[10].s64 + -31148;
	// 8260EC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EC54: 4BE581CD  bl 0x82466e20
	ctx.lr = 0x8260EC58;
	sub_82466E20(ctx, base);
	// 8260EC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EC68 size=112
    let mut pc: u32 = 0x8260EC68;
    'dispatch: loop {
        match pc {
            0x8260EC68 => {
    //   block [0x8260EC68..0x8260ECD8)
	// 8260EC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EC74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EC78: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EC7C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260EC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EC84: 390B17C0  addi r8, r11, 0x17c0
	ctx.r[8].s64 = ctx.r[11].s64 + 6080;
	// 8260EC88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260EC8C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8260EC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ECA0: 386A8684  addi r3, r10, -0x797c
	ctx.r[3].s64 = ctx.r[10].s64 + -31100;
	// 8260ECA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ECA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ECAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ECB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ECB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ECBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ECC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ECC4: 4BE5815D  bl 0x82466e20
	ctx.lr = 0x8260ECC8;
	sub_82466E20(ctx, base);
	// 8260ECC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ECD8 size=112
    let mut pc: u32 = 0x8260ECD8;
    'dispatch: loop {
        match pc {
            0x8260ECD8 => {
    //   block [0x8260ECD8..0x8260ED48)
	// 8260ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ECE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ECE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ECE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ECEC: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 8260ECF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ECF4: 390B1820  addi r8, r11, 0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + 6176;
	// 8260ECF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260ECFC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8260ED00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ED04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ED08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ED0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ED10: 386A86B4  addi r3, r10, -0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + -31052;
	// 8260ED14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ED18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ED1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ED20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ED24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ED28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ED2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ED30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ED34: 4BE580ED  bl 0x82466e20
	ctx.lr = 0x8260ED38;
	sub_82466E20(ctx, base);
	// 8260ED38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ED48 size=112
    let mut pc: u32 = 0x8260ED48;
    'dispatch: loop {
        match pc {
            0x8260ED48 => {
    //   block [0x8260ED48..0x8260EDB8)
	// 8260ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ED54: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260ED58: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8260ED5C: 38EA18E0  addi r7, r10, 0x18e0
	ctx.r[7].s64 = ctx.r[10].s64 + 6368;
	// 8260ED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ED64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260ED68: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8260ED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ED70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260ED74: 396BE518  addi r11, r11, -0x1ae8
	ctx.r[11].s64 = ctx.r[11].s64 + -6888;
	// 8260ED78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260ED7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ED80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ED84: 386A86E4  addi r3, r10, -0x791c
	ctx.r[3].s64 = ctx.r[10].s64 + -31004;
	// 8260ED88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ED8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260ED90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ED94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260ED98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ED9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EDA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EDA4: 4BE5807D  bl 0x82466e20
	ctx.lr = 0x8260EDA8;
	sub_82466E20(ctx, base);
	// 8260EDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EDB8 size=112
    let mut pc: u32 = 0x8260EDB8;
    'dispatch: loop {
        match pc {
            0x8260EDB8 => {
    //   block [0x8260EDB8..0x8260EE28)
	// 8260EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EDC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EDC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EDCC: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EDD4: 390B1AA8  addi r8, r11, 0x1aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 6824;
	// 8260EDD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EDDC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8260EDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EDE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EDF0: 386A8714  addi r3, r10, -0x78ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30956;
	// 8260EDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EE04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EE14: 4BE5800D  bl 0x82466e20
	ctx.lr = 0x8260EE18;
	sub_82466E20(ctx, base);
	// 8260EE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EE28 size=112
    let mut pc: u32 = 0x8260EE28;
    'dispatch: loop {
        match pc {
            0x8260EE28 => {
    //   block [0x8260EE28..0x8260EE98)
	// 8260EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EE38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EE3C: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EE44: 390B1AC0  addi r8, r11, 0x1ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 6848;
	// 8260EE48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EE4C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8260EE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EE54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EE60: 386A8744  addi r3, r10, -0x78bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30908;
	// 8260EE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EE74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EE84: 4BE57F9D  bl 0x82466e20
	ctx.lr = 0x8260EE88;
	sub_82466E20(ctx, base);
	// 8260EE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EE98 size=112
    let mut pc: u32 = 0x8260EE98;
    'dispatch: loop {
        match pc {
            0x8260EE98 => {
    //   block [0x8260EE98..0x8260EF08)
	// 8260EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EEA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EEAC: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EEB4: 390B1AD8  addi r8, r11, 0x1ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 6872;
	// 8260EEB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260EEBC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8260EEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EED0: 386A8774  addi r3, r10, -0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + -30860;
	// 8260EED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EEF4: 4BE57F2D  bl 0x82466e20
	ctx.lr = 0x8260EEF8;
	sub_82466E20(ctx, base);
	// 8260EEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EF08 size=108
    let mut pc: u32 = 0x8260EF08;
    'dispatch: loop {
        match pc {
            0x8260EF08 => {
    //   block [0x8260EF08..0x8260EF74)
	// 8260EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EF14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EF1C: 38EB1B08  addi r7, r11, 0x1b08
	ctx.r[7].s64 = ctx.r[11].s64 + 6920;
	// 8260EF20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260EF24: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8260EF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260EF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EF38: 386A87A4  addi r3, r10, -0x785c
	ctx.r[3].s64 = ctx.r[10].s64 + -30812;
	// 8260EF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260EF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260EF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260EF60: 4BE57EC1  bl 0x82466e20
	ctx.lr = 0x8260EF64;
	sub_82466E20(ctx, base);
	// 8260EF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EF78 size=112
    let mut pc: u32 = 0x8260EF78;
    'dispatch: loop {
        match pc {
            0x8260EF78 => {
    //   block [0x8260EF78..0x8260EFE8)
	// 8260EF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260EF88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EF8C: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 8260EF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EF94: 390B1B38  addi r8, r11, 0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + 6968;
	// 8260EF98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260EF9C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8260EFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260EFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260EFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260EFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260EFB0: 386A87D4  addi r3, r10, -0x782c
	ctx.r[3].s64 = ctx.r[10].s64 + -30764;
	// 8260EFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260EFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260EFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260EFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260EFC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260EFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260EFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260EFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260EFD4: 4BE57E4D  bl 0x82466e20
	ctx.lr = 0x8260EFD8;
	sub_82466E20(ctx, base);
	// 8260EFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260EFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260EFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260EFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260EFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260EFE8 size=108
    let mut pc: u32 = 0x8260EFE8;
    'dispatch: loop {
        match pc {
            0x8260EFE8 => {
    //   block [0x8260EFE8..0x8260F054)
	// 8260EFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260EFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260EFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260EFF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260EFF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260EFFC: 38EB1B50  addi r7, r11, 0x1b50
	ctx.r[7].s64 = ctx.r[11].s64 + 6992;
	// 8260F000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F004: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8260F008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F018: 386A8804  addi r3, r10, -0x77fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30716;
	// 8260F01C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F03C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F040: 4BE57DE1  bl 0x82466e20
	ctx.lr = 0x8260F044;
	sub_82466E20(ctx, base);
	// 8260F044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F058 size=108
    let mut pc: u32 = 0x8260F058;
    'dispatch: loop {
        match pc {
            0x8260F058 => {
    //   block [0x8260F058..0x8260F0C4)
	// 8260F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F064: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F06C: 38EB1B80  addi r7, r11, 0x1b80
	ctx.r[7].s64 = ctx.r[11].s64 + 7040;
	// 8260F070: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F074: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8260F078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F07C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F088: 386A8834  addi r3, r10, -0x77cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30668;
	// 8260F08C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F0B0: 4BE57D71  bl 0x82466e20
	ctx.lr = 0x8260F0B4;
	sub_82466E20(ctx, base);
	// 8260F0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F0C8 size=112
    let mut pc: u32 = 0x8260F0C8;
    'dispatch: loop {
        match pc {
            0x8260F0C8 => {
    //   block [0x8260F0C8..0x8260F138)
	// 8260F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F0D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F0D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F0DC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F0E4: 390B1BC8  addi r8, r11, 0x1bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 7112;
	// 8260F0E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260F0EC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8260F0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F100: 386A8864  addi r3, r10, -0x779c
	ctx.r[3].s64 = ctx.r[10].s64 + -30620;
	// 8260F104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F124: 4BE57CFD  bl 0x82466e20
	ctx.lr = 0x8260F128;
	sub_82466E20(ctx, base);
	// 8260F128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F138 size=112
    let mut pc: u32 = 0x8260F138;
    'dispatch: loop {
        match pc {
            0x8260F138 => {
    //   block [0x8260F138..0x8260F1A8)
	// 8260F138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F148: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F14C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260F150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F154: 390B1C10  addi r8, r11, 0x1c10
	ctx.r[8].s64 = ctx.r[11].s64 + 7184;
	// 8260F158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260F15C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8260F160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F170: 386A8894  addi r3, r10, -0x776c
	ctx.r[3].s64 = ctx.r[10].s64 + -30572;
	// 8260F174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F194: 4BE57C8D  bl 0x82466e20
	ctx.lr = 0x8260F198;
	sub_82466E20(ctx, base);
	// 8260F198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F1A8 size=108
    let mut pc: u32 = 0x8260F1A8;
    'dispatch: loop {
        match pc {
            0x8260F1A8 => {
    //   block [0x8260F1A8..0x8260F214)
	// 8260F1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F1B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F1BC: 38EB1CA0  addi r7, r11, 0x1ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 7328;
	// 8260F1C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F1C4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8260F1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F1D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F1D8: 386A88C4  addi r3, r10, -0x773c
	ctx.r[3].s64 = ctx.r[10].s64 + -30524;
	// 8260F1DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F200: 4BE57C21  bl 0x82466e20
	ctx.lr = 0x8260F204;
	sub_82466E20(ctx, base);
	// 8260F204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F218 size=108
    let mut pc: u32 = 0x8260F218;
    'dispatch: loop {
        match pc {
            0x8260F218 => {
    //   block [0x8260F218..0x8260F284)
	// 8260F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F224: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F22C: 38EB1CE8  addi r7, r11, 0x1ce8
	ctx.r[7].s64 = ctx.r[11].s64 + 7400;
	// 8260F230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F234: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8260F238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F248: 386A88F4  addi r3, r10, -0x770c
	ctx.r[3].s64 = ctx.r[10].s64 + -30476;
	// 8260F24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F270: 4BE57BB1  bl 0x82466e20
	ctx.lr = 0x8260F274;
	sub_82466E20(ctx, base);
	// 8260F274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F288 size=108
    let mut pc: u32 = 0x8260F288;
    'dispatch: loop {
        match pc {
            0x8260F288 => {
    //   block [0x8260F288..0x8260F2F4)
	// 8260F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F294: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F29C: 38EB1D18  addi r7, r11, 0x1d18
	ctx.r[7].s64 = ctx.r[11].s64 + 7448;
	// 8260F2A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F2A4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8260F2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F2AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F2B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F2B8: 386A8924  addi r3, r10, -0x76dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30428;
	// 8260F2BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F2DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F2E0: 4BE57B41  bl 0x82466e20
	ctx.lr = 0x8260F2E4;
	sub_82466E20(ctx, base);
	// 8260F2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F2F8 size=112
    let mut pc: u32 = 0x8260F2F8;
    'dispatch: loop {
        match pc {
            0x8260F2F8 => {
    //   block [0x8260F2F8..0x8260F368)
	// 8260F2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F30C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F314: 390B1D48  addi r8, r11, 0x1d48
	ctx.r[8].s64 = ctx.r[11].s64 + 7496;
	// 8260F318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260F31C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8260F320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F330: 386A8954  addi r3, r10, -0x76ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30380;
	// 8260F334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F354: 4BE57ACD  bl 0x82466e20
	ctx.lr = 0x8260F358;
	sub_82466E20(ctx, base);
	// 8260F358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F368 size=112
    let mut pc: u32 = 0x8260F368;
    'dispatch: loop {
        match pc {
            0x8260F368 => {
    //   block [0x8260F368..0x8260F3D8)
	// 8260F368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F374: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F378: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F37C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F384: 390B1D78  addi r8, r11, 0x1d78
	ctx.r[8].s64 = ctx.r[11].s64 + 7544;
	// 8260F388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F38C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8260F390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F3A0: 386A8984  addi r3, r10, -0x767c
	ctx.r[3].s64 = ctx.r[10].s64 + -30332;
	// 8260F3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F3C4: 4BE57A5D  bl 0x82466e20
	ctx.lr = 0x8260F3C8;
	sub_82466E20(ctx, base);
	// 8260F3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F3D8 size=112
    let mut pc: u32 = 0x8260F3D8;
    'dispatch: loop {
        match pc {
            0x8260F3D8 => {
    //   block [0x8260F3D8..0x8260F448)
	// 8260F3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F3E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F3E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F3EC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F3F4: 390B1D90  addi r8, r11, 0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + 7568;
	// 8260F3F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F3FC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8260F400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F410: 386A89B4  addi r3, r10, -0x764c
	ctx.r[3].s64 = ctx.r[10].s64 + -30284;
	// 8260F414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F434: 4BE579ED  bl 0x82466e20
	ctx.lr = 0x8260F438;
	sub_82466E20(ctx, base);
	// 8260F438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F448 size=108
    let mut pc: u32 = 0x8260F448;
    'dispatch: loop {
        match pc {
            0x8260F448 => {
    //   block [0x8260F448..0x8260F4B4)
	// 8260F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F454: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F45C: 38EB1DA8  addi r7, r11, 0x1da8
	ctx.r[7].s64 = ctx.r[11].s64 + 7592;
	// 8260F460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260F464: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8260F468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F46C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F478: 386A89E4  addi r3, r10, -0x761c
	ctx.r[3].s64 = ctx.r[10].s64 + -30236;
	// 8260F47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F4A0: 4BE57981  bl 0x82466e20
	ctx.lr = 0x8260F4A4;
	sub_82466E20(ctx, base);
	// 8260F4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F4B8 size=112
    let mut pc: u32 = 0x8260F4B8;
    'dispatch: loop {
        match pc {
            0x8260F4B8 => {
    //   block [0x8260F4B8..0x8260F528)
	// 8260F4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F4C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F4CC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F4D4: 390B1DD8  addi r8, r11, 0x1dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 7640;
	// 8260F4D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260F4DC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8260F4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F4F0: 386A8A14  addi r3, r10, -0x75ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30188;
	// 8260F4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F514: 4BE5790D  bl 0x82466e20
	ctx.lr = 0x8260F518;
	sub_82466E20(ctx, base);
	// 8260F518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F528 size=108
    let mut pc: u32 = 0x8260F528;
    'dispatch: loop {
        match pc {
            0x8260F528 => {
    //   block [0x8260F528..0x8260F594)
	// 8260F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F534: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F53C: 38EB1DF0  addi r7, r11, 0x1df0
	ctx.r[7].s64 = ctx.r[11].s64 + 7664;
	// 8260F540: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8260F544: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8260F548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F558: 386A8A44  addi r3, r10, -0x75bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30140;
	// 8260F55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F580: 4BE578A1  bl 0x82466e20
	ctx.lr = 0x8260F584;
	sub_82466E20(ctx, base);
	// 8260F584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F598 size=112
    let mut pc: u32 = 0x8260F598;
    'dispatch: loop {
        match pc {
            0x8260F598 => {
    //   block [0x8260F598..0x8260F608)
	// 8260F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F5A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F5AC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F5B4: 390B1EE0  addi r8, r11, 0x1ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 7904;
	// 8260F5B8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8260F5BC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8260F5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F5D0: 386A8A74  addi r3, r10, -0x758c
	ctx.r[3].s64 = ctx.r[10].s64 + -30092;
	// 8260F5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F5F4: 4BE5782D  bl 0x82466e20
	ctx.lr = 0x8260F5F8;
	sub_82466E20(ctx, base);
	// 8260F5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F608 size=108
    let mut pc: u32 = 0x8260F608;
    'dispatch: loop {
        match pc {
            0x8260F608 => {
    //   block [0x8260F608..0x8260F674)
	// 8260F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F614: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F61C: 38EB2090  addi r7, r11, 0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + 8336;
	// 8260F620: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8260F624: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8260F628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F62C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F638: 386A8AA4  addi r3, r10, -0x755c
	ctx.r[3].s64 = ctx.r[10].s64 + -30044;
	// 8260F63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F660: 4BE577C1  bl 0x82466e20
	ctx.lr = 0x8260F664;
	sub_82466E20(ctx, base);
	// 8260F664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F678 size=112
    let mut pc: u32 = 0x8260F678;
    'dispatch: loop {
        match pc {
            0x8260F678 => {
    //   block [0x8260F678..0x8260F6E8)
	// 8260F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F688: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F68C: 38AA7604  addi r5, r10, 0x7604
	ctx.r[5].s64 = ctx.r[10].s64 + 30212;
	// 8260F690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F694: 390B2228  addi r8, r11, 0x2228
	ctx.r[8].s64 = ctx.r[11].s64 + 8744;
	// 8260F698: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8260F69C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8260F6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F6B0: 386A8AD4  addi r3, r10, -0x752c
	ctx.r[3].s64 = ctx.r[10].s64 + -29996;
	// 8260F6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F6D4: 4BE5774D  bl 0x82466e20
	ctx.lr = 0x8260F6D8;
	sub_82466E20(ctx, base);
	// 8260F6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F6E8 size=100
    let mut pc: u32 = 0x8260F6E8;
    'dispatch: loop {
        match pc {
            0x8260F6E8 => {
    //   block [0x8260F6E8..0x8260F74C)
	// 8260F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F6F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F6FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F708: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8260F70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F71C: 386A8B04  addi r3, r10, -0x74fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29948;
	// 8260F720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F738: 4BE576E9  bl 0x82466e20
	ctx.lr = 0x8260F73C;
	sub_82466E20(ctx, base);
	// 8260F73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F750 size=112
    let mut pc: u32 = 0x8260F750;
    'dispatch: loop {
        match pc {
            0x8260F750 => {
    //   block [0x8260F750..0x8260F7C0)
	// 8260F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F760: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F764: 38AA8B04  addi r5, r10, -0x74fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29948;
	// 8260F768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F76C: 390B2480  addi r8, r11, 0x2480
	ctx.r[8].s64 = ctx.r[11].s64 + 9344;
	// 8260F770: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260F774: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8260F778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F77C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F788: 386A8B34  addi r3, r10, -0x74cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29900;
	// 8260F78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F7AC: 4BE57675  bl 0x82466e20
	ctx.lr = 0x8260F7B0;
	sub_82466E20(ctx, base);
	// 8260F7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F7C0 size=100
    let mut pc: u32 = 0x8260F7C0;
    'dispatch: loop {
        match pc {
            0x8260F7C0 => {
    //   block [0x8260F7C0..0x8260F824)
	// 8260F7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F7CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F7D4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F7E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8260F7E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F7F4: 386A8B64  addi r3, r10, -0x749c
	ctx.r[3].s64 = ctx.r[10].s64 + -29852;
	// 8260F7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F7FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F810: 4BE57611  bl 0x82466e20
	ctx.lr = 0x8260F814;
	sub_82466E20(ctx, base);
	// 8260F814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F828 size=108
    let mut pc: u32 = 0x8260F828;
    'dispatch: loop {
        match pc {
            0x8260F828 => {
    //   block [0x8260F828..0x8260F894)
	// 8260F828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F834: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F83C: 38EB24F8  addi r7, r11, 0x24f8
	ctx.r[7].s64 = ctx.r[11].s64 + 9464;
	// 8260F840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260F844: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8260F848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F84C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260F854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F858: 386A8B94  addi r3, r10, -0x746c
	ctx.r[3].s64 = ctx.r[10].s64 + -29804;
	// 8260F85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260F860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260F880: 4BE575A1  bl 0x82466e20
	ctx.lr = 0x8260F884;
	sub_82466E20(ctx, base);
	// 8260F884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F898 size=112
    let mut pc: u32 = 0x8260F898;
    'dispatch: loop {
        match pc {
            0x8260F898 => {
    //   block [0x8260F898..0x8260F908)
	// 8260F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F8A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F8A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F8AC: 38AA8B64  addi r5, r10, -0x749c
	ctx.r[5].s64 = ctx.r[10].s64 + -29852;
	// 8260F8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F8B4: 390B2540  addi r8, r11, 0x2540
	ctx.r[8].s64 = ctx.r[11].s64 + 9536;
	// 8260F8B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260F8BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8260F8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F8C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260F8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F8D0: 386A8BC4  addi r3, r10, -0x743c
	ctx.r[3].s64 = ctx.r[10].s64 + -29756;
	// 8260F8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260F8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F8F4: 4BE5752D  bl 0x82466e20
	ctx.lr = 0x8260F8F8;
	sub_82466E20(ctx, base);
	// 8260F8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F908 size=100
    let mut pc: u32 = 0x8260F908;
    'dispatch: loop {
        match pc {
            0x8260F908 => {
    //   block [0x8260F908..0x8260F96C)
	// 8260F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F914: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F91C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F928: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8260F92C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F93C: 386A8BF4  addi r3, r10, -0x740c
	ctx.r[3].s64 = ctx.r[10].s64 + -29708;
	// 8260F940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F958: 4BE574C9  bl 0x82466e20
	ctx.lr = 0x8260F95C;
	sub_82466E20(ctx, base);
	// 8260F95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F970 size=100
    let mut pc: u32 = 0x8260F970;
    'dispatch: loop {
        match pc {
            0x8260F970 => {
    //   block [0x8260F970..0x8260F9D4)
	// 8260F970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F97C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260F980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260F984: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260F988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260F990: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8260F994: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260F9A4: 386A8C24  addi r3, r10, -0x73dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29660;
	// 8260F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260F9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260F9B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260F9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260F9B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260F9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260F9C0: 4BE57461  bl 0x82466e20
	ctx.lr = 0x8260F9C4;
	sub_82466E20(ctx, base);
	// 8260F9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260F9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260F9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260F9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


