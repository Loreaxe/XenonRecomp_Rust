pub fn sub_82663740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663740 size=112
    let mut pc: u32 = 0x82663740;
    'dispatch: loop {
        match pc {
            0x82663740 => {
    //   block [0x82663740..0x826637B0)
	// 82663740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266374C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663750: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663754: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266375C: 390B5D88  addi r8, r11, 0x5d88
	ctx.r[8].s64 = ctx.r[11].s64 + 23944;
	// 82663760: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82663764: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82663768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266376C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663778: 386AC2E0  addi r3, r10, -0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + -15648;
	// 8266377C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82663780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266378C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266379C: 4BE03685  bl 0x82466e20
	ctx.lr = 0x826637A0;
	sub_82466E20(ctx, base);
	// 826637A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826637A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826637A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826637AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826637B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826637B0 size=108
    let mut pc: u32 = 0x826637B0;
    'dispatch: loop {
        match pc {
            0x826637B0 => {
    //   block [0x826637B0..0x8266381C)
	// 826637B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826637B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826637B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826637BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826637C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826637C4: 38EB5DB8  addi r7, r11, 0x5db8
	ctx.r[7].s64 = ctx.r[11].s64 + 23992;
	// 826637C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826637CC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826637D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826637D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826637D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826637DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826637E0: 386AC310  addi r3, r10, -0x3cf0
	ctx.r[3].s64 = ctx.r[10].s64 + -15600;
	// 826637E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826637E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826637EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826637F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826637F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826637F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826637FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663808: 4BE03619  bl 0x82466e20
	ctx.lr = 0x8266380C;
	sub_82466E20(ctx, base);
	// 8266380C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663820 size=108
    let mut pc: u32 = 0x82663820;
    'dispatch: loop {
        match pc {
            0x82663820 => {
    //   block [0x82663820..0x8266388C)
	// 82663820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266382C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663834: 38EB5DE8  addi r7, r11, 0x5de8
	ctx.r[7].s64 = ctx.r[11].s64 + 24040;
	// 82663838: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266383C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82663840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266384C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663850: 386AC340  addi r3, r10, -0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15552;
	// 82663854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266385C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266386C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663878: 4BE035A9  bl 0x82466e20
	ctx.lr = 0x8266387C;
	sub_82466E20(ctx, base);
	// 8266387C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663890 size=108
    let mut pc: u32 = 0x82663890;
    'dispatch: loop {
        match pc {
            0x82663890 => {
    //   block [0x82663890..0x826638FC)
	// 82663890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266389C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826638A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826638A4: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 826638A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826638AC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826638B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826638B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826638B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826638BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826638C0: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 826638C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826638C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826638CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826638D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826638D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826638D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826638DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826638E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826638E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826638E8: 4BE03539  bl 0x82466e20
	ctx.lr = 0x826638EC;
	sub_82466E20(ctx, base);
	// 826638EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826638F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826638F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826638F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663900 size=108
    let mut pc: u32 = 0x82663900;
    'dispatch: loop {
        match pc {
            0x82663900 => {
    //   block [0x82663900..0x8266396C)
	// 82663900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266390C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663914: 38EB5E78  addi r7, r11, 0x5e78
	ctx.r[7].s64 = ctx.r[11].s64 + 24184;
	// 82663918: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8266391C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82663920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266392C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663930: 386AC3A0  addi r3, r10, -0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	// 82663934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266393C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663958: 4BE034C9  bl 0x82466e20
	ctx.lr = 0x8266395C;
	sub_82466E20(ctx, base);
	// 8266395C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663970 size=108
    let mut pc: u32 = 0x82663970;
    'dispatch: loop {
        match pc {
            0x82663970 => {
    //   block [0x82663970..0x826639DC)
	// 82663970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266397C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663984: 38EB5F98  addi r7, r11, 0x5f98
	ctx.r[7].s64 = ctx.r[11].s64 + 24472;
	// 82663988: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266398C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82663990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266399C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826639A0: 386AC3D0  addi r3, r10, -0x3c30
	ctx.r[3].s64 = ctx.r[10].s64 + -15408;
	// 826639A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826639A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826639AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826639B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826639B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826639B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826639BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826639C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826639C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826639C8: 4BE03459  bl 0x82466e20
	ctx.lr = 0x826639CC;
	sub_82466E20(ctx, base);
	// 826639CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826639D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826639D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826639D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826639E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826639E0 size=108
    let mut pc: u32 = 0x826639E0;
    'dispatch: loop {
        match pc {
            0x826639E0 => {
    //   block [0x826639E0..0x82663A4C)
	// 826639E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826639E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826639E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826639EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826639F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826639F4: 38EB5FB0  addi r7, r11, 0x5fb0
	ctx.r[7].s64 = ctx.r[11].s64 + 24496;
	// 826639F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826639FC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82663A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663A04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663A10: 386AC400  addi r3, r10, -0x3c00
	ctx.r[3].s64 = ctx.r[10].s64 + -15360;
	// 82663A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663A38: 4BE033E9  bl 0x82466e20
	ctx.lr = 0x82663A3C;
	sub_82466E20(ctx, base);
	// 82663A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663A50 size=108
    let mut pc: u32 = 0x82663A50;
    'dispatch: loop {
        match pc {
            0x82663A50 => {
    //   block [0x82663A50..0x82663ABC)
	// 82663A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663A5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663A64: 38EB5FC8  addi r7, r11, 0x5fc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24520;
	// 82663A68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663A6C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82663A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663A74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663A80: 386AC430  addi r3, r10, -0x3bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -15312;
	// 82663A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663AA8: 4BE03379  bl 0x82466e20
	ctx.lr = 0x82663AAC;
	sub_82466E20(ctx, base);
	// 82663AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663AC0 size=108
    let mut pc: u32 = 0x82663AC0;
    'dispatch: loop {
        match pc {
            0x82663AC0 => {
    //   block [0x82663AC0..0x82663B2C)
	// 82663AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663ACC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663AD4: 38EB5FE0  addi r7, r11, 0x5fe0
	ctx.r[7].s64 = ctx.r[11].s64 + 24544;
	// 82663AD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663ADC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82663AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663AE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663AF0: 386AC460  addi r3, r10, -0x3ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -15264;
	// 82663AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663B18: 4BE03309  bl 0x82466e20
	ctx.lr = 0x82663B1C;
	sub_82466E20(ctx, base);
	// 82663B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663B30 size=108
    let mut pc: u32 = 0x82663B30;
    'dispatch: loop {
        match pc {
            0x82663B30 => {
    //   block [0x82663B30..0x82663B9C)
	// 82663B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663B3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663B44: 38EB5FF8  addi r7, r11, 0x5ff8
	ctx.r[7].s64 = ctx.r[11].s64 + 24568;
	// 82663B48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663B4C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82663B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663B60: 386AC490  addi r3, r10, -0x3b70
	ctx.r[3].s64 = ctx.r[10].s64 + -15216;
	// 82663B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663B88: 4BE03299  bl 0x82466e20
	ctx.lr = 0x82663B8C;
	sub_82466E20(ctx, base);
	// 82663B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663BA0 size=108
    let mut pc: u32 = 0x82663BA0;
    'dispatch: loop {
        match pc {
            0x82663BA0 => {
    //   block [0x82663BA0..0x82663C0C)
	// 82663BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663BAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663BB4: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 82663BB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663BBC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82663BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663BD0: 386AC4C0  addi r3, r10, -0x3b40
	ctx.r[3].s64 = ctx.r[10].s64 + -15168;
	// 82663BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663BF8: 4BE03229  bl 0x82466e20
	ctx.lr = 0x82663BFC;
	sub_82466E20(ctx, base);
	// 82663BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663C10 size=108
    let mut pc: u32 = 0x82663C10;
    'dispatch: loop {
        match pc {
            0x82663C10 => {
    //   block [0x82663C10..0x82663C7C)
	// 82663C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663C1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663C24: 38EB6028  addi r7, r11, 0x6028
	ctx.r[7].s64 = ctx.r[11].s64 + 24616;
	// 82663C28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82663C2C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82663C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663C40: 386AC4F0  addi r3, r10, -0x3b10
	ctx.r[3].s64 = ctx.r[10].s64 + -15120;
	// 82663C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663C68: 4BE031B9  bl 0x82466e20
	ctx.lr = 0x82663C6C;
	sub_82466E20(ctx, base);
	// 82663C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663C80 size=108
    let mut pc: u32 = 0x82663C80;
    'dispatch: loop {
        match pc {
            0x82663C80 => {
    //   block [0x82663C80..0x82663CEC)
	// 82663C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663C8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663C94: 38EB60B8  addi r7, r11, 0x60b8
	ctx.r[7].s64 = ctx.r[11].s64 + 24760;
	// 82663C98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82663C9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82663CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663CB0: 386AC520  addi r3, r10, -0x3ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -15072;
	// 82663CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663CD8: 4BE03149  bl 0x82466e20
	ctx.lr = 0x82663CDC;
	sub_82466E20(ctx, base);
	// 82663CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663CF0 size=108
    let mut pc: u32 = 0x82663CF0;
    'dispatch: loop {
        match pc {
            0x82663CF0 => {
    //   block [0x82663CF0..0x82663D5C)
	// 82663CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663CFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663D04: 38EB6178  addi r7, r11, 0x6178
	ctx.r[7].s64 = ctx.r[11].s64 + 24952;
	// 82663D08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82663D0C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82663D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663D20: 386AC550  addi r3, r10, -0x3ab0
	ctx.r[3].s64 = ctx.r[10].s64 + -15024;
	// 82663D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663D48: 4BE030D9  bl 0x82466e20
	ctx.lr = 0x82663D4C;
	sub_82466E20(ctx, base);
	// 82663D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663D60 size=108
    let mut pc: u32 = 0x82663D60;
    'dispatch: loop {
        match pc {
            0x82663D60 => {
    //   block [0x82663D60..0x82663DCC)
	// 82663D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663D6C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663D74: 38EB6250  addi r7, r11, 0x6250
	ctx.r[7].s64 = ctx.r[11].s64 + 25168;
	// 82663D78: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82663D7C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82663D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663D84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663D88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663D90: 386AC580  addi r3, r10, -0x3a80
	ctx.r[3].s64 = ctx.r[10].s64 + -14976;
	// 82663D94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663DB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663DB8: 4BE03069  bl 0x82466e20
	ctx.lr = 0x82663DBC;
	sub_82466E20(ctx, base);
	// 82663DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663DD0 size=108
    let mut pc: u32 = 0x82663DD0;
    'dispatch: loop {
        match pc {
            0x82663DD0 => {
    //   block [0x82663DD0..0x82663E3C)
	// 82663DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663DDC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663DE4: 38EB6310  addi r7, r11, 0x6310
	ctx.r[7].s64 = ctx.r[11].s64 + 25360;
	// 82663DE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82663DEC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82663DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663E00: 386AC5B0  addi r3, r10, -0x3a50
	ctx.r[3].s64 = ctx.r[10].s64 + -14928;
	// 82663E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663E28: 4BE02FF9  bl 0x82466e20
	ctx.lr = 0x82663E2C;
	sub_82466E20(ctx, base);
	// 82663E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663E40 size=112
    let mut pc: u32 = 0x82663E40;
    'dispatch: loop {
        match pc {
            0x82663E40 => {
    //   block [0x82663E40..0x82663EB0)
	// 82663E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663E4C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82663E50: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82663E54: 38EA63B8  addi r7, r10, 0x63b8
	ctx.r[7].s64 = ctx.r[10].s64 + 25528;
	// 82663E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663E5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82663E60: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82663E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663E68: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663E6C: 396BF160  addi r11, r11, -0xea0
	ctx.r[11].s64 = ctx.r[11].s64 + -3744;
	// 82663E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663E78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663E7C: 386AC5E0  addi r3, r10, -0x3a20
	ctx.r[3].s64 = ctx.r[10].s64 + -14880;
	// 82663E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663E84: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82663E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663E8C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82663E90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663E94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663E98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663E9C: 4BE02F85  bl 0x82466e20
	ctx.lr = 0x82663EA0;
	sub_82466E20(ctx, base);
	// 82663EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663EB0 size=108
    let mut pc: u32 = 0x82663EB0;
    'dispatch: loop {
        match pc {
            0x82663EB0 => {
    //   block [0x82663EB0..0x82663F1C)
	// 82663EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663EBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663EC4: 38EB64D8  addi r7, r11, 0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + 25816;
	// 82663EC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82663ECC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82663ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663ED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663EE0: 386AC610  addi r3, r10, -0x39f0
	ctx.r[3].s64 = ctx.r[10].s64 + -14832;
	// 82663EE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663F04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663F08: 4BE02F19  bl 0x82466e20
	ctx.lr = 0x82663F0C;
	sub_82466E20(ctx, base);
	// 82663F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663F20 size=108
    let mut pc: u32 = 0x82663F20;
    'dispatch: loop {
        match pc {
            0x82663F20 => {
    //   block [0x82663F20..0x82663F8C)
	// 82663F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663F2C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663F34: 38EB6538  addi r7, r11, 0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + 25912;
	// 82663F38: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82663F3C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82663F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663F44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663F50: 386AC640  addi r3, r10, -0x39c0
	ctx.r[3].s64 = ctx.r[10].s64 + -14784;
	// 82663F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663F78: 4BE02EA9  bl 0x82466e20
	ctx.lr = 0x82663F7C;
	sub_82466E20(ctx, base);
	// 82663F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663F90 size=108
    let mut pc: u32 = 0x82663F90;
    'dispatch: loop {
        match pc {
            0x82663F90 => {
    //   block [0x82663F90..0x82663FFC)
	// 82663F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663F9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663FA4: 38EB6640  addi r7, r11, 0x6640
	ctx.r[7].s64 = ctx.r[11].s64 + 26176;
	// 82663FA8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82663FAC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82663FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663FB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663FC0: 386AC670  addi r3, r10, -0x3990
	ctx.r[3].s64 = ctx.r[10].s64 + -14736;
	// 82663FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663FE8: 4BE02E39  bl 0x82466e20
	ctx.lr = 0x82663FEC;
	sub_82466E20(ctx, base);
	// 82663FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664000 size=108
    let mut pc: u32 = 0x82664000;
    'dispatch: loop {
        match pc {
            0x82664000 => {
    //   block [0x82664000..0x8266406C)
	// 82664000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266400C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664014: 38EB6718  addi r7, r11, 0x6718
	ctx.r[7].s64 = ctx.r[11].s64 + 26392;
	// 82664018: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266401C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82664020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664030: 386AC6A0  addi r3, r10, -0x3960
	ctx.r[3].s64 = ctx.r[10].s64 + -14688;
	// 82664034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266403C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266404C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664058: 4BE02DC9  bl 0x82466e20
	ctx.lr = 0x8266405C;
	sub_82466E20(ctx, base);
	// 8266405C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664070 size=108
    let mut pc: u32 = 0x82664070;
    'dispatch: loop {
        match pc {
            0x82664070 => {
    //   block [0x82664070..0x826640DC)
	// 82664070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266407C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664084: 38EB6760  addi r7, r11, 0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + 26464;
	// 82664088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266408C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82664090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826640A0: 386AC6D0  addi r3, r10, -0x3930
	ctx.r[3].s64 = ctx.r[10].s64 + -14640;
	// 826640A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826640A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826640AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826640B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826640B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826640B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826640BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826640C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826640C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826640C8: 4BE02D59  bl 0x82466e20
	ctx.lr = 0x826640CC;
	sub_82466E20(ctx, base);
	// 826640CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826640D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826640D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826640D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826640E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826640E0 size=112
    let mut pc: u32 = 0x826640E0;
    'dispatch: loop {
        match pc {
            0x826640E0 => {
    //   block [0x826640E0..0x82664150)
	// 826640E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826640E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826640E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826640EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826640F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826640F4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826640F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826640FC: 390B6778  addi r8, r11, 0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + 26488;
	// 82664100: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82664104: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82664108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266410C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664118: 386AC700  addi r3, r10, -0x3900
	ctx.r[3].s64 = ctx.r[10].s64 + -14592;
	// 8266411C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266412C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266413C: 4BE02CE5  bl 0x82466e20
	ctx.lr = 0x82664140;
	sub_82466E20(ctx, base);
	// 82664140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266414C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664150 size=112
    let mut pc: u32 = 0x82664150;
    'dispatch: loop {
        match pc {
            0x82664150 => {
    //   block [0x82664150..0x826641C0)
	// 82664150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266415C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664160: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664164: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266416C: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 82664170: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82664174: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82664178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266417C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664188: 386AC730  addi r3, r10, -0x38d0
	ctx.r[3].s64 = ctx.r[10].s64 + -14544;
	// 8266418C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266419C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826641A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826641A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826641A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826641AC: 4BE02C75  bl 0x82466e20
	ctx.lr = 0x826641B0;
	sub_82466E20(ctx, base);
	// 826641B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826641B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826641B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826641BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826641C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826641C0 size=108
    let mut pc: u32 = 0x826641C0;
    'dispatch: loop {
        match pc {
            0x826641C0 => {
    //   block [0x826641C0..0x8266422C)
	// 826641C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826641C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826641C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826641CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826641D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826641D4: 38EB6820  addi r7, r11, 0x6820
	ctx.r[7].s64 = ctx.r[11].s64 + 26656;
	// 826641D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826641DC: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826641E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826641E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826641E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826641EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826641F0: 386AC760  addi r3, r10, -0x38a0
	ctx.r[3].s64 = ctx.r[10].s64 + -14496;
	// 826641F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826641F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826641FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266420C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664218: 4BE02C09  bl 0x82466e20
	ctx.lr = 0x8266421C;
	sub_82466E20(ctx, base);
	// 8266421C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82664230 size=24
    let mut pc: u32 = 0x82664230;
    'dispatch: loop {
        match pc {
            0x82664230 => {
    //   block [0x82664230..0x82664248)
	// 82664230: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664234: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82664238: 394ABF60  addi r10, r10, -0x40a0
	ctx.r[10].s64 = ctx.r[10].s64 + -16544;
	// 8266423C: 816B6D60  lwz r11, 0x6d60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28000 as u32) ) } as u64;
	// 82664240: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82664244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664248 size=112
    let mut pc: u32 = 0x82664248;
    'dispatch: loop {
        match pc {
            0x82664248 => {
    //   block [0x82664248..0x826642B8)
	// 82664248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266424C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664254: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664258: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266425C: 38AAC970  addi r5, r10, -0x3690
	ctx.r[5].s64 = ctx.r[10].s64 + -13968;
	// 82664260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664264: 390BBF60  addi r8, r11, -0x40a0
	ctx.r[8].s64 = ctx.r[11].s64 + -16544;
	// 82664268: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8266426C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82664270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664274: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266427C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664280: 386AC790  addi r3, r10, -0x3870
	ctx.r[3].s64 = ctx.r[10].s64 + -14448;
	// 82664284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266428C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266429C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826642A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826642A4: 4BE02B7D  bl 0x82466e20
	ctx.lr = 0x826642A8;
	sub_82466E20(ctx, base);
	// 826642A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826642AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826642B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826642B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826642B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826642B8 size=108
    let mut pc: u32 = 0x826642B8;
    'dispatch: loop {
        match pc {
            0x826642B8 => {
    //   block [0x826642B8..0x82664324)
	// 826642B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826642BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826642C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826642C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826642C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826642CC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 826642D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826642D4: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826642D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826642DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826642E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826642E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826642E8: 386AC7C0  addi r3, r10, -0x3840
	ctx.r[3].s64 = ctx.r[10].s64 + -14400;
	// 826642EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826642F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826642F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826642F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826642FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266430C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664310: 4BE02B11  bl 0x82466e20
	ctx.lr = 0x82664314;
	sub_82466E20(ctx, base);
	// 82664314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266431C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664328 size=112
    let mut pc: u32 = 0x82664328;
    'dispatch: loop {
        match pc {
            0x82664328 => {
    //   block [0x82664328..0x82664398)
	// 82664328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266432C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664334: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664338: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266433C: 38AAC970  addi r5, r10, -0x3690
	ctx.r[5].s64 = ctx.r[10].s64 + -13968;
	// 82664340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664344: 390B6898  addi r8, r11, 0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + 26776;
	// 82664348: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8266434C: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82664350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266435C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664360: 386AC7F0  addi r3, r10, -0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	// 82664364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266436C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266437C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664384: 4BE02A9D  bl 0x82466e20
	ctx.lr = 0x82664388;
	sub_82466E20(ctx, base);
	// 82664388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266438C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664398 size=108
    let mut pc: u32 = 0x82664398;
    'dispatch: loop {
        match pc {
            0x82664398 => {
    //   block [0x82664398..0x82664404)
	// 82664398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826643A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826643A4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826643A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826643AC: 38EB6958  addi r7, r11, 0x6958
	ctx.r[7].s64 = ctx.r[11].s64 + 26968;
	// 826643B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826643B4: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826643B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826643BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826643C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826643C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826643C8: 386AC820  addi r3, r10, -0x37e0
	ctx.r[3].s64 = ctx.r[10].s64 + -14304;
	// 826643CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826643D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826643D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826643D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826643DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826643E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826643E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826643E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826643EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826643F0: 4BE02A31  bl 0x82466e20
	ctx.lr = 0x826643F4;
	sub_82466E20(ctx, base);
	// 826643F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826643F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826643FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664408 size=108
    let mut pc: u32 = 0x82664408;
    'dispatch: loop {
        match pc {
            0x82664408 => {
    //   block [0x82664408..0x82664474)
	// 82664408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664414: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266441C: 38EB69D0  addi r7, r11, 0x69d0
	ctx.r[7].s64 = ctx.r[11].s64 + 27088;
	// 82664420: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664424: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82664428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266442C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664438: 386AC850  addi r3, r10, -0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + -14256;
	// 8266443C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266444C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266445C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664460: 4BE029C1  bl 0x82466e20
	ctx.lr = 0x82664464;
	sub_82466E20(ctx, base);
	// 82664464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266446C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664478 size=108
    let mut pc: u32 = 0x82664478;
    'dispatch: loop {
        match pc {
            0x82664478 => {
    //   block [0x82664478..0x826644E4)
	// 82664478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664484: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266448C: 38EB6A18  addi r7, r11, 0x6a18
	ctx.r[7].s64 = ctx.r[11].s64 + 27160;
	// 82664490: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664494: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82664498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266449C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826644A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826644A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826644A8: 386AC880  addi r3, r10, -0x3780
	ctx.r[3].s64 = ctx.r[10].s64 + -14208;
	// 826644AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826644B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826644B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826644B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826644BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826644C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826644C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826644C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826644CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826644D0: 4BE02951  bl 0x82466e20
	ctx.lr = 0x826644D4;
	sub_82466E20(ctx, base);
	// 826644D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826644D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826644DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826644E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826644E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826644E8 size=112
    let mut pc: u32 = 0x826644E8;
    'dispatch: loop {
        match pc {
            0x826644E8 => {
    //   block [0x826644E8..0x82664558)
	// 826644E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826644EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826644F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826644F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826644F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826644FC: 38AAC880  addi r5, r10, -0x3780
	ctx.r[5].s64 = ctx.r[10].s64 + -14208;
	// 82664500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664504: 390B6A60  addi r8, r11, 0x6a60
	ctx.r[8].s64 = ctx.r[11].s64 + 27232;
	// 82664508: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266450C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82664510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266451C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664520: 386AC8B0  addi r3, r10, -0x3750
	ctx.r[3].s64 = ctx.r[10].s64 + -14160;
	// 82664524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266452C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266453C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664544: 4BE028DD  bl 0x82466e20
	ctx.lr = 0x82664548;
	sub_82466E20(ctx, base);
	// 82664548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266454C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664558 size=108
    let mut pc: u32 = 0x82664558;
    'dispatch: loop {
        match pc {
            0x82664558 => {
    //   block [0x82664558..0x826645C4)
	// 82664558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664564: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266456C: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82664570: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664574: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82664578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266457C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664580: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664588: 386AC8E0  addi r3, r10, -0x3720
	ctx.r[3].s64 = ctx.r[10].s64 + -14112;
	// 8266458C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266459C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826645A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826645A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826645A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826645AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826645B0: 4BE02871  bl 0x82466e20
	ctx.lr = 0x826645B4;
	sub_82466E20(ctx, base);
	// 826645B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826645B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826645BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826645C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826645C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826645C8 size=108
    let mut pc: u32 = 0x826645C8;
    'dispatch: loop {
        match pc {
            0x826645C8 => {
    //   block [0x826645C8..0x82664634)
	// 826645C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826645CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826645D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826645D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826645D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826645DC: 38EB6B20  addi r7, r11, 0x6b20
	ctx.r[7].s64 = ctx.r[11].s64 + 27424;
	// 826645E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826645E4: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826645E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826645EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826645F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826645F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826645F8: 386AC910  addi r3, r10, -0x36f0
	ctx.r[3].s64 = ctx.r[10].s64 + -14064;
	// 826645FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266460C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266461C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664620: 4BE02801  bl 0x82466e20
	ctx.lr = 0x82664624;
	sub_82466E20(ctx, base);
	// 82664624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266462C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664638 size=108
    let mut pc: u32 = 0x82664638;
    'dispatch: loop {
        match pc {
            0x82664638 => {
    //   block [0x82664638..0x826646A4)
	// 82664638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266463C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664644: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266464C: 38EB6BE0  addi r7, r11, 0x6be0
	ctx.r[7].s64 = ctx.r[11].s64 + 27616;
	// 82664650: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82664654: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 82664658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266465C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664660: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664668: 386AC940  addi r3, r10, -0x36c0
	ctx.r[3].s64 = ctx.r[10].s64 + -14016;
	// 8266466C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266467C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266468C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664690: 4BE02791  bl 0x82466e20
	ctx.lr = 0x82664694;
	sub_82466E20(ctx, base);
	// 82664694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266469C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826646A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826646A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826646A8 size=112
    let mut pc: u32 = 0x826646A8;
    'dispatch: loop {
        match pc {
            0x826646A8 => {
    //   block [0x826646A8..0x82664718)
	// 826646A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826646AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826646B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826646B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826646B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826646BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826646C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826646C4: 390B6D68  addi r8, r11, 0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + 28008;
	// 826646C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826646CC: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826646D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826646D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826646D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826646DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826646E0: 386AC970  addi r3, r10, -0x3690
	ctx.r[3].s64 = ctx.r[10].s64 + -13968;
	// 826646E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826646E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826646EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826646F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826646F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826646F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826646FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664704: 4BE0271D  bl 0x82466e20
	ctx.lr = 0x82664708;
	sub_82466E20(ctx, base);
	// 82664708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266470C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664718 size=108
    let mut pc: u32 = 0x82664718;
    'dispatch: loop {
        match pc {
            0x82664718 => {
    //   block [0x82664718..0x82664784)
	// 82664718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266471C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664724: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266472C: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 82664730: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82664734: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 82664738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266473C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664740: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664748: 386AC9A0  addi r3, r10, -0x3660
	ctx.r[3].s64 = ctx.r[10].s64 + -13920;
	// 8266474C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266475C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266476C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664770: 4BE026B1  bl 0x82466e20
	ctx.lr = 0x82664774;
	sub_82466E20(ctx, base);
	// 82664774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266477C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664788 size=112
    let mut pc: u32 = 0x82664788;
    'dispatch: loop {
        match pc {
            0x82664788 => {
    //   block [0x82664788..0x826647F8)
	// 82664788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664798: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266479C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826647A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826647A4: 390B6E40  addi r8, r11, 0x6e40
	ctx.r[8].s64 = ctx.r[11].s64 + 28224;
	// 826647A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826647AC: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826647B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826647B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826647B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826647BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826647C0: 386AC9D0  addi r3, r10, -0x3630
	ctx.r[3].s64 = ctx.r[10].s64 + -13872;
	// 826647C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826647C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826647CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826647D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826647D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826647D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826647DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826647E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826647E4: 4BE0263D  bl 0x82466e20
	ctx.lr = 0x826647E8;
	sub_82466E20(ctx, base);
	// 826647E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826647EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826647F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826647F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826647F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826647F8 size=108
    let mut pc: u32 = 0x826647F8;
    'dispatch: loop {
        match pc {
            0x826647F8 => {
    //   block [0x826647F8..0x82664864)
	// 826647F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826647FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664804: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266480C: 38EB6E88  addi r7, r11, 0x6e88
	ctx.r[7].s64 = ctx.r[11].s64 + 28296;
	// 82664810: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664814: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 82664818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266481C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664828: 386ACA00  addi r3, r10, -0x3600
	ctx.r[3].s64 = ctx.r[10].s64 + -13824;
	// 8266482C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266483C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266484C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664850: 4BE025D1  bl 0x82466e20
	ctx.lr = 0x82664854;
	sub_82466E20(ctx, base);
	// 82664854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266485C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664868 size=108
    let mut pc: u32 = 0x82664868;
    'dispatch: loop {
        match pc {
            0x82664868 => {
    //   block [0x82664868..0x826648D4)
	// 82664868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664874: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266487C: 38EB6EE8  addi r7, r11, 0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 28392;
	// 82664880: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664884: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82664888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266488C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664898: 386ACA30  addi r3, r10, -0x35d0
	ctx.r[3].s64 = ctx.r[10].s64 + -13776;
	// 8266489C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826648A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826648A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826648A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826648AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826648B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826648B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826648B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826648BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826648C0: 4BE02561  bl 0x82466e20
	ctx.lr = 0x826648C4;
	sub_82466E20(ctx, base);
	// 826648C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826648C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826648CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826648D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826648D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826648D8 size=108
    let mut pc: u32 = 0x826648D8;
    'dispatch: loop {
        match pc {
            0x826648D8 => {
    //   block [0x826648D8..0x82664944)
	// 826648D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826648DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826648E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826648E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826648E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826648EC: 38EB6F30  addi r7, r11, 0x6f30
	ctx.r[7].s64 = ctx.r[11].s64 + 28464;
	// 826648F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826648F4: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826648F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826648FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664908: 386ACA60  addi r3, r10, -0x35a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13728;
	// 8266490C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266491C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266492C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664930: 4BE024F1  bl 0x82466e20
	ctx.lr = 0x82664934;
	sub_82466E20(ctx, base);
	// 82664934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266493C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664948 size=108
    let mut pc: u32 = 0x82664948;
    'dispatch: loop {
        match pc {
            0x82664948 => {
    //   block [0x82664948..0x826649B4)
	// 82664948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664954: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266495C: 38EB6FF0  addi r7, r11, 0x6ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 28656;
	// 82664960: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82664964: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82664968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266496C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664978: 386ACA90  addi r3, r10, -0x3570
	ctx.r[3].s64 = ctx.r[10].s64 + -13680;
	// 8266497C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266498C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266499C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826649A0: 4BE02481  bl 0x82466e20
	ctx.lr = 0x826649A4;
	sub_82466E20(ctx, base);
	// 826649A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826649A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826649AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826649B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826649B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826649B8 size=108
    let mut pc: u32 = 0x826649B8;
    'dispatch: loop {
        match pc {
            0x826649B8 => {
    //   block [0x826649B8..0x82664A24)
	// 826649B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826649BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826649C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826649C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826649C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826649CC: 38EB7080  addi r7, r11, 0x7080
	ctx.r[7].s64 = ctx.r[11].s64 + 28800;
	// 826649D0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826649D4: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 826649D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826649DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826649E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826649E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826649E8: 386ACAC0  addi r3, r10, -0x3540
	ctx.r[3].s64 = ctx.r[10].s64 + -13632;
	// 826649EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826649F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826649F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826649F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826649FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664A10: 4BE02411  bl 0x82466e20
	ctx.lr = 0x82664A14;
	sub_82466E20(ctx, base);
	// 82664A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664A28 size=108
    let mut pc: u32 = 0x82664A28;
    'dispatch: loop {
        match pc {
            0x82664A28 => {
    //   block [0x82664A28..0x82664A94)
	// 82664A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664A34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664A3C: 38EB71B8  addi r7, r11, 0x71b8
	ctx.r[7].s64 = ctx.r[11].s64 + 29112;
	// 82664A40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664A44: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82664A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664A4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664A58: 386ACAF0  addi r3, r10, -0x3510
	ctx.r[3].s64 = ctx.r[10].s64 + -13584;
	// 82664A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664A80: 4BE023A1  bl 0x82466e20
	ctx.lr = 0x82664A84;
	sub_82466E20(ctx, base);
	// 82664A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664A98 size=116
    let mut pc: u32 = 0x82664A98;
    'dispatch: loop {
        match pc {
            0x82664A98 => {
    //   block [0x82664A98..0x82664B0C)
	// 82664A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664AA4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664AA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664AAC: 390B7218  addi r8, r11, 0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + 29208;
	// 82664AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664AB4: 392AF214  addi r9, r10, -0xdec
	ctx.r[9].s64 = ctx.r[10].s64 + -3564;
	// 82664AB8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664ABC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82664AC0: 38AACAF0  addi r5, r10, -0x3510
	ctx.r[5].s64 = ctx.r[10].s64 + -13584;
	// 82664AC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664ACC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664ADC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82664AE0: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82664AE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82664AE8: 386BCB20  addi r3, r11, -0x34e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13536;
	// 82664AEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664AF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664AF8: 4BE02329  bl 0x82466e20
	ctx.lr = 0x82664AFC;
	sub_82466E20(ctx, base);
	// 82664AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664B10 size=96
    let mut pc: u32 = 0x82664B10;
    'dispatch: loop {
        match pc {
            0x82664B10 => {
    //   block [0x82664B10..0x82664B70)
	// 82664B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664B1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664B24: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82664B28: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664B30: 386ACB50  addi r3, r10, -0x34b0
	ctx.r[3].s64 = ctx.r[10].s64 + -13488;
	// 82664B34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664B3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664B50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664B5C: 4BE022C5  bl 0x82466e20
	ctx.lr = 0x82664B60;
	sub_82466E20(ctx, base);
	// 82664B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664B70 size=112
    let mut pc: u32 = 0x82664B70;
    'dispatch: loop {
        match pc {
            0x82664B70 => {
    //   block [0x82664B70..0x82664BE0)
	// 82664B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664B80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664B84: 38AAECE0  addi r5, r10, -0x1320
	ctx.r[5].s64 = ctx.r[10].s64 + -4896;
	// 82664B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664B8C: 390B7290  addi r8, r11, 0x7290
	ctx.r[8].s64 = ctx.r[11].s64 + 29328;
	// 82664B90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82664B94: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82664B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664B9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664BA8: 386ACB80  addi r3, r10, -0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + -13440;
	// 82664BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664BCC: 4BE02255  bl 0x82466e20
	ctx.lr = 0x82664BD0;
	sub_82466E20(ctx, base);
	// 82664BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664BE0 size=96
    let mut pc: u32 = 0x82664BE0;
    'dispatch: loop {
        match pc {
            0x82664BE0 => {
    //   block [0x82664BE0..0x82664C40)
	// 82664BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664BEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664BF4: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82664BF8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664C00: 386ACBB0  addi r3, r10, -0x3450
	ctx.r[3].s64 = ctx.r[10].s64 + -13392;
	// 82664C04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664C0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664C20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664C28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664C2C: 4BE021F5  bl 0x82466e20
	ctx.lr = 0x82664C30;
	sub_82466E20(ctx, base);
	// 82664C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82664C40 size=24
    let mut pc: u32 = 0x82664C40;
    'dispatch: loop {
        match pc {
            0x82664C40 => {
    //   block [0x82664C40..0x82664C58)
	// 82664C40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664C44: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82664C48: 394AC020  addi r10, r10, -0x3fe0
	ctx.r[10].s64 = ctx.r[10].s64 + -16352;
	// 82664C4C: 816B72F0  lwz r11, 0x72f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29424 as u32) ) } as u64;
	// 82664C50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82664C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664C58 size=116
    let mut pc: u32 = 0x82664C58;
    'dispatch: loop {
        match pc {
            0x82664C58 => {
    //   block [0x82664C58..0x82664CCC)
	// 82664C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664C64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82664C68: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664C6C: 390BC020  addi r8, r11, -0x3fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -16352;
	// 82664C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664C74: 392AF250  addi r9, r10, -0xdb0
	ctx.r[9].s64 = ctx.r[10].s64 + -3504;
	// 82664C78: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664C7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82664C80: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664C84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664C94: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664C9C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82664CA0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82664CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82664CA8: 386BCBE0  addi r3, r11, -0x3420
	ctx.r[3].s64 = ctx.r[11].s64 + -13344;
	// 82664CAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664CB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664CB8: 4BE02169  bl 0x82466e20
	ctx.lr = 0x82664CBC;
	sub_82466E20(ctx, base);
	// 82664CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664CD0 size=104
    let mut pc: u32 = 0x82664CD0;
    'dispatch: loop {
        match pc {
            0x82664CD0 => {
    //   block [0x82664CD0..0x82664D38)
	// 82664CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664CDC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664CE4: 392AF27C  addi r9, r10, -0xd84
	ctx.r[9].s64 = ctx.r[10].s64 + -3460;
	// 82664CE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664CF0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664CF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664D04: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82664D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664D0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664D10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664D18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664D1C: 386ACC10  addi r3, r10, -0x33f0
	ctx.r[3].s64 = ctx.r[10].s64 + -13296;
	// 82664D20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664D24: 4BE020FD  bl 0x82466e20
	ctx.lr = 0x82664D28;
	sub_82466E20(ctx, base);
	// 82664D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664D38 size=96
    let mut pc: u32 = 0x82664D38;
    'dispatch: loop {
        match pc {
            0x82664D38 => {
    //   block [0x82664D38..0x82664D98)
	// 82664D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664D44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664D4C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82664D50: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664D58: 386ACC40  addi r3, r10, -0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	// 82664D5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664D78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664D80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664D84: 4BE0209D  bl 0x82466e20
	ctx.lr = 0x82664D88;
	sub_82466E20(ctx, base);
	// 82664D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664D98 size=100
    let mut pc: u32 = 0x82664D98;
    'dispatch: loop {
        match pc {
            0x82664D98 => {
    //   block [0x82664D98..0x82664DFC)
	// 82664D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664DAC: 38AACC10  addi r5, r10, -0x33f0
	ctx.r[5].s64 = ctx.r[10].s64 + -13296;
	// 82664DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664DB8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82664DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664DCC: 386ACC70  addi r3, r10, -0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + -13200;
	// 82664DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664DD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664DE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664DE8: 4BE02039  bl 0x82466e20
	ctx.lr = 0x82664DEC;
	sub_82466E20(ctx, base);
	// 82664DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664E00 size=112
    let mut pc: u32 = 0x82664E00;
    'dispatch: loop {
        match pc {
            0x82664E00 => {
    //   block [0x82664E00..0x82664E70)
	// 82664E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664E0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E10: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664E14: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664E1C: 390B72F8  addi r8, r11, 0x72f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29432;
	// 82664E20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82664E24: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82664E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664E2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664E38: 386ACCA0  addi r3, r10, -0x3360
	ctx.r[3].s64 = ctx.r[10].s64 + -13152;
	// 82664E3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664E5C: 4BE01FC5  bl 0x82466e20
	ctx.lr = 0x82664E60;
	sub_82466E20(ctx, base);
	// 82664E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664E70 size=112
    let mut pc: u32 = 0x82664E70;
    'dispatch: loop {
        match pc {
            0x82664E70 => {
    //   block [0x82664E70..0x82664EE0)
	// 82664E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664E7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664E84: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664E8C: 390B7340  addi r8, r11, 0x7340
	ctx.r[8].s64 = ctx.r[11].s64 + 29504;
	// 82664E90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82664E94: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82664E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664E9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664EA8: 386ACCD0  addi r3, r10, -0x3330
	ctx.r[3].s64 = ctx.r[10].s64 + -13104;
	// 82664EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664ECC: 4BE01F55  bl 0x82466e20
	ctx.lr = 0x82664ED0;
	sub_82466E20(ctx, base);
	// 82664ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664EE0 size=100
    let mut pc: u32 = 0x82664EE0;
    'dispatch: loop {
        match pc {
            0x82664EE0 => {
    //   block [0x82664EE0..0x82664F44)
	// 82664EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664EEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664EF4: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664F00: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82664F04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664F14: 386ACD00  addi r3, r10, -0x3300
	ctx.r[3].s64 = ctx.r[10].s64 + -13056;
	// 82664F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664F20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664F30: 4BE01EF1  bl 0x82466e20
	ctx.lr = 0x82664F34;
	sub_82466E20(ctx, base);
	// 82664F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664F48 size=96
    let mut pc: u32 = 0x82664F48;
    'dispatch: loop {
        match pc {
            0x82664F48 => {
    //   block [0x82664F48..0x82664FA8)
	// 82664F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664F54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664F5C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82664F60: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664F68: 386ACD30  addi r3, r10, -0x32d0
	ctx.r[3].s64 = ctx.r[10].s64 + -13008;
	// 82664F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664F74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664F88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664F90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664F94: 4BE01E8D  bl 0x82466e20
	ctx.lr = 0x82664F98;
	sub_82466E20(ctx, base);
	// 82664F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664FA8 size=112
    let mut pc: u32 = 0x82664FA8;
    'dispatch: loop {
        match pc {
            0x82664FA8 => {
    //   block [0x82664FA8..0x82665018)
	// 82664FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664FB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664FB8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664FBC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664FC4: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 82664FC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82664FCC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82664FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664FE0: 386ACD60  addi r3, r10, -0x32a0
	ctx.r[3].s64 = ctx.r[10].s64 + -12960;
	// 82664FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665004: 4BE01E1D  bl 0x82466e20
	ctx.lr = 0x82665008;
	sub_82466E20(ctx, base);
	// 82665008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266500C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665018 size=96
    let mut pc: u32 = 0x82665018;
    'dispatch: loop {
        match pc {
            0x82665018 => {
    //   block [0x82665018..0x82665078)
	// 82665018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266501C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665024: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266502C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82665030: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665038: 386ACD90  addi r3, r10, -0x3270
	ctx.r[3].s64 = ctx.r[10].s64 + -12912;
	// 8266503C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665044: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266504C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665058: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266505C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665064: 4BE01DBD  bl 0x82466e20
	ctx.lr = 0x82665068;
	sub_82466E20(ctx, base);
	// 82665068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266506C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665078 size=112
    let mut pc: u32 = 0x82665078;
    'dispatch: loop {
        match pc {
            0x82665078 => {
    //   block [0x82665078..0x826650E8)
	// 82665078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266507C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665088: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266508C: 38AACD90  addi r5, r10, -0x3270
	ctx.r[5].s64 = ctx.r[10].s64 + -12912;
	// 82665090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665094: 390B7388  addi r8, r11, 0x7388
	ctx.r[8].s64 = ctx.r[11].s64 + 29576;
	// 82665098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266509C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826650A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826650A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826650A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826650AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826650B0: 386ACDC0  addi r3, r10, -0x3240
	ctx.r[3].s64 = ctx.r[10].s64 + -12864;
	// 826650B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826650B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826650BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826650C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826650C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826650C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826650CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826650D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826650D4: 4BE01D4D  bl 0x82466e20
	ctx.lr = 0x826650D8;
	sub_82466E20(ctx, base);
	// 826650D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826650DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826650E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826650E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826650E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826650E8 size=108
    let mut pc: u32 = 0x826650E8;
    'dispatch: loop {
        match pc {
            0x826650E8 => {
    //   block [0x826650E8..0x82665154)
	// 826650E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826650EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826650F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826650F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826650F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826650FC: 38EB73A0  addi r7, r11, 0x73a0
	ctx.r[7].s64 = ctx.r[11].s64 + 29600;
	// 82665100: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665104: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82665108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266510C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665118: 386ACDF0  addi r3, r10, -0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + -12816;
	// 8266511C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266512C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266513C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665140: 4BE01CE1  bl 0x82466e20
	ctx.lr = 0x82665144;
	sub_82466E20(ctx, base);
	// 82665144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266514C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665158 size=112
    let mut pc: u32 = 0x82665158;
    'dispatch: loop {
        match pc {
            0x82665158 => {
    //   block [0x82665158..0x826651C8)
	// 82665158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266515C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266516C: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665174: 390B7400  addi r8, r11, 0x7400
	ctx.r[8].s64 = ctx.r[11].s64 + 29696;
	// 82665178: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266517C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82665180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266518C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665190: 386ACE20  addi r3, r10, -0x31e0
	ctx.r[3].s64 = ctx.r[10].s64 + -12768;
	// 82665194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266519C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826651A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826651A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826651A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826651AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826651B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826651B4: 4BE01C6D  bl 0x82466e20
	ctx.lr = 0x826651B8;
	sub_82466E20(ctx, base);
	// 826651B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826651BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826651C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826651C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826651C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826651C8 size=112
    let mut pc: u32 = 0x826651C8;
    'dispatch: loop {
        match pc {
            0x826651C8 => {
    //   block [0x826651C8..0x82665238)
	// 826651C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826651CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826651D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826651D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826651D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826651DC: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826651E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826651E4: 390B7418  addi r8, r11, 0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + 29720;
	// 826651E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826651EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826651F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826651F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826651F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826651FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665200: 386ACE50  addi r3, r10, -0x31b0
	ctx.r[3].s64 = ctx.r[10].s64 + -12720;
	// 82665204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266520C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266521C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665224: 4BE01BFD  bl 0x82466e20
	ctx.lr = 0x82665228;
	sub_82466E20(ctx, base);
	// 82665228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266522C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665238 size=100
    let mut pc: u32 = 0x82665238;
    'dispatch: loop {
        match pc {
            0x82665238 => {
    //   block [0x82665238..0x8266529C)
	// 82665238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266524C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665258: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8266525C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266526C: 386ACE80  addi r3, r10, -0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + -12672;
	// 82665270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665278: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266527C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665288: 4BE01B99  bl 0x82466e20
	ctx.lr = 0x8266528C;
	sub_82466E20(ctx, base);
	// 8266528C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826652A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826652A0 size=116
    let mut pc: u32 = 0x826652A0;
    'dispatch: loop {
        match pc {
            0x826652A0 => {
    //   block [0x826652A0..0x82665314)
	// 826652A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826652A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826652A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826652AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826652B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826652B4: 390B744C  addi r8, r11, 0x744c
	ctx.r[8].s64 = ctx.r[11].s64 + 29772;
	// 826652B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826652BC: 392AF2A8  addi r9, r10, -0xd58
	ctx.r[9].s64 = ctx.r[10].s64 + -3416;
	// 826652C0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826652C4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826652C8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826652CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826652D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826652D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826652D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826652DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826652E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826652E4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826652E8: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826652EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826652F0: 386BCEB0  addi r3, r11, -0x3150
	ctx.r[3].s64 = ctx.r[11].s64 + -12624;
	// 826652F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826652F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826652FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665300: 4BE01B21  bl 0x82466e20
	ctx.lr = 0x82665304;
	sub_82466E20(ctx, base);
	// 82665304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665318 size=112
    let mut pc: u32 = 0x82665318;
    'dispatch: loop {
        match pc {
            0x82665318 => {
    //   block [0x82665318..0x82665388)
	// 82665318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665328: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266532C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665334: 390B747C  addi r8, r11, 0x747c
	ctx.r[8].s64 = ctx.r[11].s64 + 29820;
	// 82665338: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266533C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82665340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266534C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665350: 386ACEE0  addi r3, r10, -0x3120
	ctx.r[3].s64 = ctx.r[10].s64 + -12576;
	// 82665354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266535C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665364: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266536C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665374: 4BE01AAD  bl 0x82466e20
	ctx.lr = 0x82665378;
	sub_82466E20(ctx, base);
	// 82665378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266537C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665388 size=116
    let mut pc: u32 = 0x82665388;
    'dispatch: loop {
        match pc {
            0x82665388 => {
    //   block [0x82665388..0x826653FC)
	// 82665388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665394: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665398: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266539C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 826653A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826653A4: 392AF2D4  addi r9, r10, -0xd2c
	ctx.r[9].s64 = ctx.r[10].s64 + -3372;
	// 826653A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826653AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826653B0: 38AAD570  addi r5, r10, -0x2a90
	ctx.r[5].s64 = ctx.r[10].s64 + -10896;
	// 826653B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826653B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826653BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826653C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826653C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826653C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826653CC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826653D0: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826653D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826653D8: 386BCF10  addi r3, r11, -0x30f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12528;
	// 826653DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826653E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826653E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826653E8: 4BE01A39  bl 0x82466e20
	ctx.lr = 0x826653EC;
	sub_82466E20(ctx, base);
	// 826653EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826653F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826653F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826653F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665400 size=112
    let mut pc: u32 = 0x82665400;
    'dispatch: loop {
        match pc {
            0x82665400 => {
    //   block [0x82665400..0x82665470)
	// 82665400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266540C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665410: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665414: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266541C: 390B74B0  addi r8, r11, 0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29872;
	// 82665420: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665424: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82665428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266542C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665438: 386ACF40  addi r3, r10, -0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + -12480;
	// 8266543C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266544C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266545C: 4BE019C5  bl 0x82466e20
	ctx.lr = 0x82665460;
	sub_82466E20(ctx, base);
	// 82665460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665470 size=112
    let mut pc: u32 = 0x82665470;
    'dispatch: loop {
        match pc {
            0x82665470 => {
    //   block [0x82665470..0x826654E0)
	// 82665470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266547C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665480: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665484: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266548C: 390B7528  addi r8, r11, 0x7528
	ctx.r[8].s64 = ctx.r[11].s64 + 29992;
	// 82665490: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665494: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82665498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266549C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826654A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826654A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826654A8: 386ACF70  addi r3, r10, -0x3090
	ctx.r[3].s64 = ctx.r[10].s64 + -12432;
	// 826654AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826654B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826654B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826654B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826654BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826654C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826654C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826654C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826654CC: 4BE01955  bl 0x82466e20
	ctx.lr = 0x826654D0;
	sub_82466E20(ctx, base);
	// 826654D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826654D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826654D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826654DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826654E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826654E0 size=112
    let mut pc: u32 = 0x826654E0;
    'dispatch: loop {
        match pc {
            0x826654E0 => {
    //   block [0x826654E0..0x82665550)
	// 826654E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826654E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826654E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826654EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826654F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826654F4: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826654F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826654FC: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82665500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665504: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82665508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266550C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665518: 386ACFA0  addi r3, r10, -0x3060
	ctx.r[3].s64 = ctx.r[10].s64 + -12384;
	// 8266551C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266552C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266553C: 4BE018E5  bl 0x82466e20
	ctx.lr = 0x82665540;
	sub_82466E20(ctx, base);
	// 82665540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266554C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665550 size=112
    let mut pc: u32 = 0x82665550;
    'dispatch: loop {
        match pc {
            0x82665550 => {
    //   block [0x82665550..0x826655C0)
	// 82665550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266555C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665560: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665564: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266556C: 390B75B8  addi r8, r11, 0x75b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30136;
	// 82665570: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665574: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82665578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266557C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665588: 386ACFD0  addi r3, r10, -0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + -12336;
	// 8266558C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266559C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826655A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826655A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826655A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826655AC: 4BE01875  bl 0x82466e20
	ctx.lr = 0x826655B0;
	sub_82466E20(ctx, base);
	// 826655B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826655B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826655B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826655BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826655C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826655C0 size=108
    let mut pc: u32 = 0x826655C0;
    'dispatch: loop {
        match pc {
            0x826655C0 => {
    //   block [0x826655C0..0x8266562C)
	// 826655C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826655C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826655C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826655CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826655D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826655D4: 38EB7600  addi r7, r11, 0x7600
	ctx.r[7].s64 = ctx.r[11].s64 + 30208;
	// 826655D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826655DC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826655E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826655E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826655E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826655EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826655F0: 386AD000  addi r3, r10, -0x3000
	ctx.r[3].s64 = ctx.r[10].s64 + -12288;
	// 826655F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826655F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826655FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266560C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665618: 4BE01809  bl 0x82466e20
	ctx.lr = 0x8266561C;
	sub_82466E20(ctx, base);
	// 8266561C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665630 size=112
    let mut pc: u32 = 0x82665630;
    'dispatch: loop {
        match pc {
            0x82665630 => {
    //   block [0x82665630..0x826656A0)
	// 82665630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266563C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665640: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665644: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266564C: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 82665650: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665654: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82665658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266565C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665668: 386AD030  addi r3, r10, -0x2fd0
	ctx.r[3].s64 = ctx.r[10].s64 + -12240;
	// 8266566C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266567C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266568C: 4BE01795  bl 0x82466e20
	ctx.lr = 0x82665690;
	sub_82466E20(ctx, base);
	// 82665690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266569C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826656A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826656A0 size=116
    let mut pc: u32 = 0x826656A0;
    'dispatch: loop {
        match pc {
            0x826656A0 => {
    //   block [0x826656A0..0x82665714)
	// 826656A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826656A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826656A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826656AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826656B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826656B4: 392BF310  addi r9, r11, -0xcf0
	ctx.r[9].s64 = ctx.r[11].s64 + -3312;
	// 826656B8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826656BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826656C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826656C4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826656C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826656CC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826656D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826656D4: 396B76C8  addi r11, r11, 0x76c8
	ctx.r[11].s64 = ctx.r[11].s64 + 30408;
	// 826656D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826656DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826656E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826656E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826656E8: 386AD060  addi r3, r10, -0x2fa0
	ctx.r[3].s64 = ctx.r[10].s64 + -12192;
	// 826656EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826656F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826656F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826656F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826656FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665700: 4BE01721  bl 0x82466e20
	ctx.lr = 0x82665704;
	sub_82466E20(ctx, base);
	// 82665704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266570C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665718 size=36
    let mut pc: u32 = 0x82665718;
    'dispatch: loop {
        match pc {
            0x82665718 => {
    //   block [0x82665718..0x8266573C)
	// 82665718: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266571C: 814B775C  lwz r10, 0x775c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30556 as u32) ) } as u64;
	// 82665720: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665724: 396BC050  addi r11, r11, -0x3fb0
	ctx.r[11].s64 = ctx.r[11].s64 + -16304;
	// 82665728: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8266572C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82665730: 814A76C4  lwz r10, 0x76c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30404 as u32) ) } as u64;
	// 82665734: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82665738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665740 size=108
    let mut pc: u32 = 0x82665740;
    'dispatch: loop {
        match pc {
            0x82665740 => {
    //   block [0x82665740..0x826657AC)
	// 82665740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266574C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665754: 38EBC050  addi r7, r11, -0x3fb0
	ctx.r[7].s64 = ctx.r[11].s64 + -16304;
	// 82665758: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8266575C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82665760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266576C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665770: 386AD090  addi r3, r10, -0x2f70
	ctx.r[3].s64 = ctx.r[10].s64 + -12144;
	// 82665774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266577C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266578C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665798: 4BE01689  bl 0x82466e20
	ctx.lr = 0x8266579C;
	sub_82466E20(ctx, base);
	// 8266579C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826657A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826657A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826657A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826657B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826657B0 size=24
    let mut pc: u32 = 0x826657B0;
    'dispatch: loop {
        match pc {
            0x826657B0 => {
    //   block [0x826657B0..0x826657C8)
	// 826657B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826657B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826657B8: 394AC0F8  addi r10, r10, -0x3f08
	ctx.r[10].s64 = ctx.r[10].s64 + -16136;
	// 826657BC: 816B76C4  lwz r11, 0x76c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30404 as u32) ) } as u64;
	// 826657C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826657C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826657C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826657C8 size=116
    let mut pc: u32 = 0x826657C8;
    'dispatch: loop {
        match pc {
            0x826657C8 => {
    //   block [0x826657C8..0x8266583C)
	// 826657C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826657CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826657D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826657D4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826657D8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826657DC: 390AC0F8  addi r8, r10, -0x3f08
	ctx.r[8].s64 = ctx.r[10].s64 + -16136;
	// 826657E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826657E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826657E8: 38AAD090  addi r5, r10, -0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + -12144;
	// 826657EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826657F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826657F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826657F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826657FC: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82665800: 396BF3CC  addi r11, r11, -0xc34
	ctx.r[11].s64 = ctx.r[11].s64 + -3124;
	// 82665804: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266580C: 386AD0C0  addi r3, r10, -0x2f40
	ctx.r[3].s64 = ctx.r[10].s64 + -12096;
	// 82665810: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665818: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665828: 4BE015F9  bl 0x82466e20
	ctx.lr = 0x8266582C;
	sub_82466E20(ctx, base);
	// 8266582C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665840 size=112
    let mut pc: u32 = 0x82665840;
    'dispatch: loop {
        match pc {
            0x82665840 => {
    //   block [0x82665840..0x826658B0)
	// 82665840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266584C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665850: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665854: 38AAD090  addi r5, r10, -0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + -12144;
	// 82665858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266585C: 390B7760  addi r8, r11, 0x7760
	ctx.r[8].s64 = ctx.r[11].s64 + 30560;
	// 82665860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665864: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82665868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266586C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665878: 386AD0F0  addi r3, r10, -0x2f10
	ctx.r[3].s64 = ctx.r[10].s64 + -12048;
	// 8266587C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266588C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266589C: 4BE01585  bl 0x82466e20
	ctx.lr = 0x826658A0;
	sub_82466E20(ctx, base);
	// 826658A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826658A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826658A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826658AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826658B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826658B0 size=24
    let mut pc: u32 = 0x826658B0;
    'dispatch: loop {
        match pc {
            0x826658B0 => {
    //   block [0x826658B0..0x826658C8)
	// 826658B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826658B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826658B8: 394AC1E8  addi r10, r10, -0x3e18
	ctx.r[10].s64 = ctx.r[10].s64 + -15896;
	// 826658BC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 826658C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826658C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826658C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826658C8 size=116
    let mut pc: u32 = 0x826658C8;
    'dispatch: loop {
        match pc {
            0x826658C8 => {
    //   block [0x826658C8..0x8266593C)
	// 826658C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826658CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826658D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826658D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826658D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826658DC: 392BF390  addi r9, r11, -0xc70
	ctx.r[9].s64 = ctx.r[11].s64 + -3184;
	// 826658E0: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 826658E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826658E8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826658EC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826658F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826658F4: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826658F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826658FC: 396BC1E8  addi r11, r11, -0x3e18
	ctx.r[11].s64 = ctx.r[11].s64 + -15896;
	// 82665900: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82665904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665908: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266590C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665910: 386AD120  addi r3, r10, -0x2ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -12000;
	// 82665914: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82665918: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266591C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665920: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82665924: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665928: 4BE014F9  bl 0x82466e20
	ctx.lr = 0x8266592C;
	sub_82466E20(ctx, base);
	// 8266592C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665940 size=100
    let mut pc: u32 = 0x82665940;
    'dispatch: loop {
        match pc {
            0x82665940 => {
    //   block [0x82665940..0x826659A4)
	// 82665940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266594C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665954: 38AAD270  addi r5, r10, -0x2d90
	ctx.r[5].s64 = ctx.r[10].s64 + -11664;
	// 82665958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665960: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82665964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665974: 386AD150  addi r3, r10, -0x2eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11952;
	// 82665978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266597C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82665984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266598C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665990: 4BE01491  bl 0x82466e20
	ctx.lr = 0x82665994;
	sub_82466E20(ctx, base);
	// 82665994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826659A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826659A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826659A8 size=100
    let mut pc: u32 = 0x826659A8;
    'dispatch: loop {
        match pc {
            0x826659A8 => {
    //   block [0x826659A8..0x82665A0C)
	// 826659A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826659AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826659B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826659B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826659B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826659BC: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826659C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826659C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826659C8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826659CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826659D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826659D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826659D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826659DC: 386AD180  addi r3, r10, -0x2e80
	ctx.r[3].s64 = ctx.r[10].s64 + -11904;
	// 826659E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826659E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826659E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826659EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826659F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826659F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826659F8: 4BE01429  bl 0x82466e20
	ctx.lr = 0x826659FC;
	sub_82466E20(ctx, base);
	// 826659FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665A10 size=108
    let mut pc: u32 = 0x82665A10;
    'dispatch: loop {
        match pc {
            0x82665A10 => {
    //   block [0x82665A10..0x82665A7C)
	// 82665A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665A1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665A24: 38EB77D8  addi r7, r11, 0x77d8
	ctx.r[7].s64 = ctx.r[11].s64 + 30680;
	// 82665A28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665A2C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82665A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665A40: 386AD1B0  addi r3, r10, -0x2e50
	ctx.r[3].s64 = ctx.r[10].s64 + -11856;
	// 82665A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665A68: 4BE013B9  bl 0x82466e20
	ctx.lr = 0x82665A6C;
	sub_82466E20(ctx, base);
	// 82665A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665A80 size=112
    let mut pc: u32 = 0x82665A80;
    'dispatch: loop {
        match pc {
            0x82665A80 => {
    //   block [0x82665A80..0x82665AF0)
	// 82665A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665A8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665A90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665A94: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665A9C: 390B7838  addi r8, r11, 0x7838
	ctx.r[8].s64 = ctx.r[11].s64 + 30776;
	// 82665AA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82665AA4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82665AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665AAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665AB8: 386AD1E0  addi r3, r10, -0x2e20
	ctx.r[3].s64 = ctx.r[10].s64 + -11808;
	// 82665ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665ADC: 4BE01345  bl 0x82466e20
	ctx.lr = 0x82665AE0;
	sub_82466E20(ctx, base);
	// 82665AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665AF0 size=108
    let mut pc: u32 = 0x82665AF0;
    'dispatch: loop {
        match pc {
            0x82665AF0 => {
    //   block [0x82665AF0..0x82665B5C)
	// 82665AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665AFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665B04: 38EB7898  addi r7, r11, 0x7898
	ctx.r[7].s64 = ctx.r[11].s64 + 30872;
	// 82665B08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665B0C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82665B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665B14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665B20: 386AD210  addi r3, r10, -0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + -11760;
	// 82665B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665B48: 4BE012D9  bl 0x82466e20
	ctx.lr = 0x82665B4C;
	sub_82466E20(ctx, base);
	// 82665B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665B60 size=28
    let mut pc: u32 = 0x82665B60;
    'dispatch: loop {
        match pc {
            0x82665B60 => {
    //   block [0x82665B60..0x82665B7C)
	// 82665B60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665B64: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665B68: 394AC2D8  addi r10, r10, -0x3d28
	ctx.r[10].s64 = ctx.r[10].s64 + -15656;
	// 82665B6C: 816B78B0  lwz r11, 0x78b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30896 as u32) ) } as u64;
	// 82665B70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82665B74: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82665B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665B80 size=112
    let mut pc: u32 = 0x82665B80;
    'dispatch: loop {
        match pc {
            0x82665B80 => {
    //   block [0x82665B80..0x82665BF0)
	// 82665B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665B8C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665B90: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82665B94: 38EAC2D8  addi r7, r10, -0x3d28
	ctx.r[7].s64 = ctx.r[10].s64 + -15656;
	// 82665B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665B9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665BA0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82665BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665BA8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665BAC: 396BF490  addi r11, r11, -0xb70
	ctx.r[11].s64 = ctx.r[11].s64 + -2928;
	// 82665BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665BB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665BB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665BBC: 386AD240  addi r3, r10, -0x2dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11712;
	// 82665BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665BC4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665BC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665BCC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82665BD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665BD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665BD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665BDC: 4BE01245  bl 0x82466e20
	ctx.lr = 0x82665BE0;
	sub_82466E20(ctx, base);
	// 82665BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665BF0 size=24
    let mut pc: u32 = 0x82665BF0;
    'dispatch: loop {
        match pc {
            0x82665BF0 => {
    //   block [0x82665BF0..0x82665C08)
	// 82665BF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665BF4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665BF8: 394AC440  addi r10, r10, -0x3bc0
	ctx.r[10].s64 = ctx.r[10].s64 + -15296;
	// 82665BFC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82665C00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82665C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665C08 size=116
    let mut pc: u32 = 0x82665C08;
    'dispatch: loop {
        match pc {
            0x82665C08 => {
    //   block [0x82665C08..0x82665C7C)
	// 82665C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665C14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665C18: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C1C: 392BF464  addi r9, r11, -0xb9c
	ctx.r[9].s64 = ctx.r[11].s64 + -2972;
	// 82665C20: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665C24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665C28: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 82665C2C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82665C30: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665C34: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82665C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665C3C: 396BC440  addi r11, r11, -0x3bc0
	ctx.r[11].s64 = ctx.r[11].s64 + -15296;
	// 82665C40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82665C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82665C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665C50: 386AD270  addi r3, r10, -0x2d90
	ctx.r[3].s64 = ctx.r[10].s64 + -11664;
	// 82665C54: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82665C58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82665C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665C60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82665C64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665C68: 4BE011B9  bl 0x82466e20
	ctx.lr = 0x82665C6C;
	sub_82466E20(ctx, base);
	// 82665C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665C80 size=112
    let mut pc: u32 = 0x82665C80;
    'dispatch: loop {
        match pc {
            0x82665C80 => {
    //   block [0x82665C80..0x82665CF0)
	// 82665C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665C8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665C94: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82665C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665C9C: 390B78B8  addi r8, r11, 0x78b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30904;
	// 82665CA0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82665CA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82665CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665CAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665CB8: 386AD2A0  addi r3, r10, -0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + -11616;
	// 82665CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665CDC: 4BE01145  bl 0x82466e20
	ctx.lr = 0x82665CE0;
	sub_82466E20(ctx, base);
	// 82665CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665CF0 size=24
    let mut pc: u32 = 0x82665CF0;
    'dispatch: loop {
        match pc {
            0x82665CF0 => {
    //   block [0x82665CF0..0x82665D08)
	// 82665CF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665CF4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665CF8: 394AC4E8  addi r10, r10, -0x3b18
	ctx.r[10].s64 = ctx.r[10].s64 + -15128;
	// 82665CFC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82665D00: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82665D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665D08 size=116
    let mut pc: u32 = 0x82665D08;
    'dispatch: loop {
        match pc {
            0x82665D08 => {
    //   block [0x82665D08..0x82665D7C)
	// 82665D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665D14: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665D18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82665D1C: 390AC4E8  addi r8, r10, -0x3b18
	ctx.r[8].s64 = ctx.r[10].s64 + -15128;
	// 82665D20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665D28: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82665D2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665D30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82665D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665D3C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 82665D40: 396BF4F0  addi r11, r11, -0xb10
	ctx.r[11].s64 = ctx.r[11].s64 + -2832;
	// 82665D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665D4C: 386AD2D0  addi r3, r10, -0x2d30
	ctx.r[3].s64 = ctx.r[10].s64 + -11568;
	// 82665D50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665D54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665D58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82665D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665D68: 4BE010B9  bl 0x82466e20
	ctx.lr = 0x82665D6C;
	sub_82466E20(ctx, base);
	// 82665D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665D80 size=112
    let mut pc: u32 = 0x82665D80;
    'dispatch: loop {
        match pc {
            0x82665D80 => {
    //   block [0x82665D80..0x82665DF0)
	// 82665D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665D8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665D94: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82665D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665D9C: 390B7948  addi r8, r11, 0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + 31048;
	// 82665DA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665DA4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82665DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665DB8: 386AD300  addi r3, r10, -0x2d00
	ctx.r[3].s64 = ctx.r[10].s64 + -11520;
	// 82665DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665DCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665DDC: 4BE01045  bl 0x82466e20
	ctx.lr = 0x82665DE0;
	sub_82466E20(ctx, base);
	// 82665DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665DF0 size=108
    let mut pc: u32 = 0x82665DF0;
    'dispatch: loop {
        match pc {
            0x82665DF0 => {
    //   block [0x82665DF0..0x82665E5C)
	// 82665DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665DFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665E04: 38EB7978  addi r7, r11, 0x7978
	ctx.r[7].s64 = ctx.r[11].s64 + 31096;
	// 82665E08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82665E0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82665E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665E20: 386AD330  addi r3, r10, -0x2cd0
	ctx.r[3].s64 = ctx.r[10].s64 + -11472;
	// 82665E24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665E48: 4BE00FD9  bl 0x82466e20
	ctx.lr = 0x82665E4C;
	sub_82466E20(ctx, base);
	// 82665E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665E60 size=112
    let mut pc: u32 = 0x82665E60;
    'dispatch: loop {
        match pc {
            0x82665E60 => {
    //   block [0x82665E60..0x82665ED0)
	// 82665E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665E6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665E74: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665E7C: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 82665E80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665E84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82665E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665E8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665E98: 386AD360  addi r3, r10, -0x2ca0
	ctx.r[3].s64 = ctx.r[10].s64 + -11424;
	// 82665E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665EBC: 4BE00F65  bl 0x82466e20
	ctx.lr = 0x82665EC0;
	sub_82466E20(ctx, base);
	// 82665EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665ED0 size=112
    let mut pc: u32 = 0x82665ED0;
    'dispatch: loop {
        match pc {
            0x82665ED0 => {
    //   block [0x82665ED0..0x82665F40)
	// 82665ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665EE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665EE4: 38AAD570  addi r5, r10, -0x2a90
	ctx.r[5].s64 = ctx.r[10].s64 + -10896;
	// 82665EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665EEC: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82665EF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665EF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82665EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665F08: 386AD390  addi r3, r10, -0x2c70
	ctx.r[3].s64 = ctx.r[10].s64 + -11376;
	// 82665F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665F2C: 4BE00EF5  bl 0x82466e20
	ctx.lr = 0x82665F30;
	sub_82466E20(ctx, base);
	// 82665F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665F40 size=108
    let mut pc: u32 = 0x82665F40;
    'dispatch: loop {
        match pc {
            0x82665F40 => {
    //   block [0x82665F40..0x82665FAC)
	// 82665F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665F4C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665F54: 38EB7A08  addi r7, r11, 0x7a08
	ctx.r[7].s64 = ctx.r[11].s64 + 31240;
	// 82665F58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82665F5C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82665F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665F64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665F70: 386AD3C0  addi r3, r10, -0x2c40
	ctx.r[3].s64 = ctx.r[10].s64 + -11328;
	// 82665F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665F98: 4BE00E89  bl 0x82466e20
	ctx.lr = 0x82665F9C;
	sub_82466E20(ctx, base);
	// 82665F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665FB0 size=108
    let mut pc: u32 = 0x82665FB0;
    'dispatch: loop {
        match pc {
            0x82665FB0 => {
    //   block [0x82665FB0..0x8266601C)
	// 82665FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665FBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665FC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82665FC4: 38EB7A50  addi r7, r11, 0x7a50
	ctx.r[7].s64 = ctx.r[11].s64 + 31312;
	// 82665FC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665FCC: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 82665FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665FE0: 386AD3F0  addi r3, r10, -0x2c10
	ctx.r[3].s64 = ctx.r[10].s64 + -11280;
	// 82665FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666008: 4BE00E19  bl 0x82466e20
	ctx.lr = 0x8266600C;
	sub_82466E20(ctx, base);
	// 8266600C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666020 size=112
    let mut pc: u32 = 0x82666020;
    'dispatch: loop {
        match pc {
            0x82666020 => {
    //   block [0x82666020..0x82666090)
	// 82666020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266602C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666030: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666034: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82666038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266603C: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 82666040: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666044: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82666048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266604C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666058: 386AD420  addi r3, r10, -0x2be0
	ctx.r[3].s64 = ctx.r[10].s64 + -11232;
	// 8266605C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266606C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266607C: 4BE00DA5  bl 0x82466e20
	ctx.lr = 0x82666080;
	sub_82466E20(ctx, base);
	// 82666080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266608C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666090 size=100
    let mut pc: u32 = 0x82666090;
    'dispatch: loop {
        match pc {
            0x82666090 => {
    //   block [0x82666090..0x826660F4)
	// 82666090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266609C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826660A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826660A4: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826660A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826660AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826660B0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826660B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826660B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826660BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826660C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826660C4: 386AD450  addi r3, r10, -0x2bb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11184;
	// 826660C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826660CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826660D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826660D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826660D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826660DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826660E0: 4BE00D41  bl 0x82466e20
	ctx.lr = 0x826660E4;
	sub_82466E20(ctx, base);
	// 826660E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826660E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826660EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826660F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826660F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826660F8 size=112
    let mut pc: u32 = 0x826660F8;
    'dispatch: loop {
        match pc {
            0x826660F8 => {
    //   block [0x826660F8..0x82666168)
	// 826660F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826660FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666108: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266610C: 38AAD180  addi r5, r10, -0x2e80
	ctx.r[5].s64 = ctx.r[10].s64 + -11904;
	// 82666110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666114: 390B7B10  addi r8, r11, 0x7b10
	ctx.r[8].s64 = ctx.r[11].s64 + 31504;
	// 82666118: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266611C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82666120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266612C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666130: 386AD480  addi r3, r10, -0x2b80
	ctx.r[3].s64 = ctx.r[10].s64 + -11136;
	// 82666134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266613C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266614C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666154: 4BE00CCD  bl 0x82466e20
	ctx.lr = 0x82666158;
	sub_82466E20(ctx, base);
	// 82666158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266615C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666168 size=112
    let mut pc: u32 = 0x82666168;
    'dispatch: loop {
        match pc {
            0x82666168 => {
    //   block [0x82666168..0x826661D8)
	// 82666168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666178: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266617C: 38AAD180  addi r5, r10, -0x2e80
	ctx.r[5].s64 = ctx.r[10].s64 + -11904;
	// 82666180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666184: 390B7B58  addi r8, r11, 0x7b58
	ctx.r[8].s64 = ctx.r[11].s64 + 31576;
	// 82666188: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266618C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82666190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266619C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826661A0: 386AD4B0  addi r3, r10, -0x2b50
	ctx.r[3].s64 = ctx.r[10].s64 + -11088;
	// 826661A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826661A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826661AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826661B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826661B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826661B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826661BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826661C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826661C4: 4BE00C5D  bl 0x82466e20
	ctx.lr = 0x826661C8;
	sub_82466E20(ctx, base);
	// 826661C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826661CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826661D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826661D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826661D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826661D8 size=108
    let mut pc: u32 = 0x826661D8;
    'dispatch: loop {
        match pc {
            0x826661D8 => {
    //   block [0x826661D8..0x82666244)
	// 826661D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826661DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826661E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826661E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826661E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826661EC: 38EB7C00  addi r7, r11, 0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + 31744;
	// 826661F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826661F4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826661F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826661FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82666204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666208: 386AD4E0  addi r3, r10, -0x2b20
	ctx.r[3].s64 = ctx.r[10].s64 + -11040;
	// 8266620C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82666210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266621C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266622C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666230: 4BE00BF1  bl 0x82466e20
	ctx.lr = 0x82666234;
	sub_82466E20(ctx, base);
	// 82666234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266623C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82666248 size=24
    let mut pc: u32 = 0x82666248;
    'dispatch: loop {
        match pc {
            0x82666248 => {
    //   block [0x82666248..0x82666260)
	// 82666248: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266624C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666250: 394AC620  addi r10, r10, -0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14816;
	// 82666254: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82666258: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266625C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666260 size=116
    let mut pc: u32 = 0x82666260;
    'dispatch: loop {
        match pc {
            0x82666260 => {
    //   block [0x82666260..0x826662D4)
	// 82666260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266626C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666270: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82666274: 390AC620  addi r8, r10, -0x39e0
	ctx.r[8].s64 = ctx.r[10].s64 + -14816;
	// 82666278: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266627C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666280: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82666284: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666288: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266628C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666294: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82666298: 396BF528  addi r11, r11, -0xad8
	ctx.r[11].s64 = ctx.r[11].s64 + -2776;
	// 8266629C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826662A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826662A4: 386AD510  addi r3, r10, -0x2af0
	ctx.r[3].s64 = ctx.r[10].s64 + -10992;
	// 826662A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826662AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826662B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826662B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826662B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826662BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826662C0: 4BE00B61  bl 0x82466e20
	ctx.lr = 0x826662C4;
	sub_82466E20(ctx, base);
	// 826662C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826662C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826662CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826662D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826662D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826662D8 size=112
    let mut pc: u32 = 0x826662D8;
    'dispatch: loop {
        match pc {
            0x826662D8 => {
    //   block [0x826662D8..0x82666348)
	// 826662D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826662DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826662E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826662E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826662E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826662EC: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826662F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826662F4: 390B7C60  addi r8, r11, 0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + 31840;
	// 826662F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826662FC: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82666300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266630C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666310: 386AD540  addi r3, r10, -0x2ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -10944;
	// 82666314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266631C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266632C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666334: 4BE00AED  bl 0x82466e20
	ctx.lr = 0x82666338;
	sub_82466E20(ctx, base);
	// 82666338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266633C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666348 size=100
    let mut pc: u32 = 0x82666348;
    'dispatch: loop {
        match pc {
            0x82666348 => {
    //   block [0x82666348..0x826663AC)
	// 82666348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266635C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82666360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666368: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8266636C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266637C: 386AD570  addi r3, r10, -0x2a90
	ctx.r[3].s64 = ctx.r[10].s64 + -10896;
	// 82666380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82666394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666398: 4BE00A89  bl 0x82466e20
	ctx.lr = 0x8266639C;
	sub_82466E20(ctx, base);
	// 8266639C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826663A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826663A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826663A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826663B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826663B0 size=112
    let mut pc: u32 = 0x826663B0;
    'dispatch: loop {
        match pc {
            0x826663B0 => {
    //   block [0x826663B0..0x82666420)
	// 826663B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826663B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826663B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826663BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826663C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826663C4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826663C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826663CC: 390B7C78  addi r8, r11, 0x7c78
	ctx.r[8].s64 = ctx.r[11].s64 + 31864;
	// 826663D0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826663D4: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826663D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826663DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826663E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826663E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826663E8: 386AD5A0  addi r3, r10, -0x2a60
	ctx.r[3].s64 = ctx.r[10].s64 + -10848;
	// 826663EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826663F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826663F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826663F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826663FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266640C: 4BE00A15  bl 0x82466e20
	ctx.lr = 0x82666410;
	sub_82466E20(ctx, base);
	// 82666410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666420 size=112
    let mut pc: u32 = 0x82666420;
    'dispatch: loop {
        match pc {
            0x82666420 => {
    //   block [0x82666420..0x82666490)
	// 82666420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266642C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666430: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666434: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82666438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266643C: 390B7D08  addi r8, r11, 0x7d08
	ctx.r[8].s64 = ctx.r[11].s64 + 32008;
	// 82666440: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666444: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82666448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266644C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666458: 386AD5D0  addi r3, r10, -0x2a30
	ctx.r[3].s64 = ctx.r[10].s64 + -10800;
	// 8266645C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266646C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266647C: 4BE009A5  bl 0x82466e20
	ctx.lr = 0x82666480;
	sub_82466E20(ctx, base);
	// 82666480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266648C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666490 size=112
    let mut pc: u32 = 0x82666490;
    'dispatch: loop {
        match pc {
            0x82666490 => {
    //   block [0x82666490..0x82666500)
	// 82666490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266649C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826664A0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826664A4: 38AAD120  addi r5, r10, -0x2ee0
	ctx.r[5].s64 = ctx.r[10].s64 + -12000;
	// 826664A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826664AC: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 826664B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826664B4: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826664B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826664BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826664C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826664C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826664C8: 386AD600  addi r3, r10, -0x2a00
	ctx.r[3].s64 = ctx.r[10].s64 + -10752;
	// 826664CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826664D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826664D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826664D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826664DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826664E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826664E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826664E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826664EC: 4BE00935  bl 0x82466e20
	ctx.lr = 0x826664F0;
	sub_82466E20(ctx, base);
	// 826664F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826664F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826664F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826664FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666500 size=112
    let mut pc: u32 = 0x82666500;
    'dispatch: loop {
        match pc {
            0x82666500 => {
    //   block [0x82666500..0x82666570)
	// 82666500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266650C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666510: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666514: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82666518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266651C: 390B7D98  addi r8, r11, 0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + 32152;
	// 82666520: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82666524: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82666528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266652C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666538: 386AD630  addi r3, r10, -0x29d0
	ctx.r[3].s64 = ctx.r[10].s64 + -10704;
	// 8266653C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266654C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266655C: 4BE008C5  bl 0x82466e20
	ctx.lr = 0x82666560;
	sub_82466E20(ctx, base);
	// 82666560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266656C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666570 size=112
    let mut pc: u32 = 0x82666570;
    'dispatch: loop {
        match pc {
            0x82666570 => {
    //   block [0x82666570..0x826665E0)
	// 82666570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266657C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666580: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666584: 38AAD270  addi r5, r10, -0x2d90
	ctx.r[5].s64 = ctx.r[10].s64 + -11664;
	// 82666588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266658C: 390B7E28  addi r8, r11, 0x7e28
	ctx.r[8].s64 = ctx.r[11].s64 + 32296;
	// 82666590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666594: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82666598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266659C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826665A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826665A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826665A8: 386AD660  addi r3, r10, -0x29a0
	ctx.r[3].s64 = ctx.r[10].s64 + -10656;
	// 826665AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826665B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826665B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826665B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826665BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826665C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826665C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826665C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826665CC: 4BE00855  bl 0x82466e20
	ctx.lr = 0x826665D0;
	sub_82466E20(ctx, base);
	// 826665D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826665D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826665D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826665DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826665E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826665E0 size=112
    let mut pc: u32 = 0x826665E0;
    'dispatch: loop {
        match pc {
            0x826665E0 => {
    //   block [0x826665E0..0x82666650)
	// 826665E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826665E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826665E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826665EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826665F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826665F4: 38AAD4B0  addi r5, r10, -0x2b50
	ctx.r[5].s64 = ctx.r[10].s64 + -11088;
	// 826665F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826665FC: 390B7E40  addi r8, r11, 0x7e40
	ctx.r[8].s64 = ctx.r[11].s64 + 32320;
	// 82666600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666604: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82666608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266660C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666618: 386AD690  addi r3, r10, -0x2970
	ctx.r[3].s64 = ctx.r[10].s64 + -10608;
	// 8266661C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266663C: 4BE007E5  bl 0x82466e20
	ctx.lr = 0x82666640;
	sub_82466E20(ctx, base);
	// 82666640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266664C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666650 size=112
    let mut pc: u32 = 0x82666650;
    'dispatch: loop {
        match pc {
            0x82666650 => {
    //   block [0x82666650..0x826666C0)
	// 82666650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266665C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666664: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82666668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266666C: 390B7E70  addi r8, r11, 0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + 32368;
	// 82666670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666674: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82666678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266667C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666688: 386AD6C0  addi r3, r10, -0x2940
	ctx.r[3].s64 = ctx.r[10].s64 + -10560;
	// 8266668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826666A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826666A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826666A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826666AC: 4BE00775  bl 0x82466e20
	ctx.lr = 0x826666B0;
	sub_82466E20(ctx, base);
	// 826666B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826666B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826666B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826666BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826666C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826666C0 size=24
    let mut pc: u32 = 0x826666C0;
    'dispatch: loop {
        match pc {
            0x826666C0 => {
    //   block [0x826666C0..0x826666D8)
	// 826666C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826666C4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826666C8: 394AC698  addi r10, r10, -0x3968
	ctx.r[10].s64 = ctx.r[10].s64 + -14696;
	// 826666CC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 826666D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826666D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826666D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826666D8 size=116
    let mut pc: u32 = 0x826666D8;
    'dispatch: loop {
        match pc {
            0x826666D8 => {
    //   block [0x826666D8..0x8266674C)
	// 826666D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826666DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826666E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826666E4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826666E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826666EC: 390AC698  addi r8, r10, -0x3968
	ctx.r[8].s64 = ctx.r[10].s64 + -14696;
	// 826666F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826666F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826666F8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826666FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666700: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266670C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82666710: 396BF540  addi r11, r11, -0xac0
	ctx.r[11].s64 = ctx.r[11].s64 + -2752;
	// 82666714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666718: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266671C: 386AD6F0  addi r3, r10, -0x2910
	ctx.r[3].s64 = ctx.r[10].s64 + -10512;
	// 82666720: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666728: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266672C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666738: 4BE006E9  bl 0x82466e20
	ctx.lr = 0x8266673C;
	sub_82466E20(ctx, base);
	// 8266673C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666750 size=112
    let mut pc: u32 = 0x82666750;
    'dispatch: loop {
        match pc {
            0x82666750 => {
    //   block [0x82666750..0x826667C0)
	// 82666750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266675C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666760: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666764: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82666768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266676C: 390B7EB8  addi r8, r11, 0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 32440;
	// 82666770: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666774: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82666778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266677C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666788: 386AD720  addi r3, r10, -0x28e0
	ctx.r[3].s64 = ctx.r[10].s64 + -10464;
	// 8266678C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266679C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826667A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826667A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826667A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826667AC: 4BE00675  bl 0x82466e20
	ctx.lr = 0x826667B0;
	sub_82466E20(ctx, base);
	// 826667B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826667B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826667B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826667BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826667C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826667C0 size=112
    let mut pc: u32 = 0x826667C0;
    'dispatch: loop {
        match pc {
            0x826667C0 => {
    //   block [0x826667C0..0x82666830)
	// 826667C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826667C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826667C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826667CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826667D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826667D4: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 826667D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826667DC: 390B7EE8  addi r8, r11, 0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 32488;
	// 826667E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826667E4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826667E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826667EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826667F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826667F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826667F8: 386AD750  addi r3, r10, -0x28b0
	ctx.r[3].s64 = ctx.r[10].s64 + -10416;
	// 826667FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266680C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266681C: 4BE00605  bl 0x82466e20
	ctx.lr = 0x82666820;
	sub_82466E20(ctx, base);
	// 82666820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666830 size=100
    let mut pc: u32 = 0x82666830;
    'dispatch: loop {
        match pc {
            0x82666830 => {
    //   block [0x82666830..0x82666894)
	// 82666830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266683C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82666840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666844: 392AF5B0  addi r9, r10, -0xa50
	ctx.r[9].s64 = ctx.r[10].s64 + -2640;
	// 82666848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266684C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666850: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 82666854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266685C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666864: 386AD780  addi r3, r10, -0x2880
	ctx.r[3].s64 = ctx.r[10].s64 + -10368;
	// 82666868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266686C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82666870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82666874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666880: 4BE005A1  bl 0x82466e20
	ctx.lr = 0x82666884;
	sub_82466E20(ctx, base);
	// 82666884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82666898 size=24
    let mut pc: u32 = 0x82666898;
    'dispatch: loop {
        match pc {
            0x82666898 => {
    //   block [0x82666898..0x826668B0)
	// 82666898: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266689C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826668A0: 394AC740  addi r10, r10, -0x38c0
	ctx.r[10].s64 = ctx.r[10].s64 + -14528;
	// 826668A4: 816B7F24  lwz r11, 0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32548 as u32) ) } as u64;
	// 826668A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826668AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826668B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826668B0 size=112
    let mut pc: u32 = 0x826668B0;
    'dispatch: loop {
        match pc {
            0x826668B0 => {
    //   block [0x826668B0..0x82666920)
	// 826668B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826668B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826668B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826668BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826668C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826668C4: 392AF6F0  addi r9, r10, -0x910
	ctx.r[9].s64 = ctx.r[10].s64 + -2320;
	// 826668C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826668CC: 390BC740  addi r8, r11, -0x38c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14528;
	// 826668D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826668D4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826668D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826668DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826668E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826668E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826668E8: 386AD7B0  addi r3, r10, -0x2850
	ctx.r[3].s64 = ctx.r[10].s64 + -10320;
	// 826668EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826668F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826668F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826668F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826668FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82666908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266690C: 4BE00515  bl 0x82466e20
	ctx.lr = 0x82666910;
	sub_82466E20(ctx, base);
	// 82666910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266691C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666920 size=112
    let mut pc: u32 = 0x82666920;
    'dispatch: loop {
        match pc {
            0x82666920 => {
    //   block [0x82666920..0x82666990)
	// 82666920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266692C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666930: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666934: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266693C: 390B7F2C  addi r8, r11, 0x7f2c
	ctx.r[8].s64 = ctx.r[11].s64 + 32556;
	// 82666940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666944: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 82666948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266694C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666958: 386AD7E0  addi r3, r10, -0x2820
	ctx.r[3].s64 = ctx.r[10].s64 + -10272;
	// 8266695C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266696C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266697C: 4BE004A5  bl 0x82466e20
	ctx.lr = 0x82666980;
	sub_82466E20(ctx, base);
	// 82666980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266698C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666990 size=108
    let mut pc: u32 = 0x82666990;
    'dispatch: loop {
        match pc {
            0x82666990 => {
    //   block [0x82666990..0x826669FC)
	// 82666990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266699C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826669A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826669A4: 38EB7F5C  addi r7, r11, 0x7f5c
	ctx.r[7].s64 = ctx.r[11].s64 + 32604;
	// 826669A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826669AC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826669B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826669B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826669B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826669BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826669C0: 386AD810  addi r3, r10, -0x27f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10224;
	// 826669C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826669C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826669CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826669D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826669D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826669D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826669DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826669E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826669E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826669E8: 4BE00439  bl 0x82466e20
	ctx.lr = 0x826669EC;
	sub_82466E20(ctx, base);
	// 826669EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826669F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826669F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826669F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666A00 size=112
    let mut pc: u32 = 0x82666A00;
    'dispatch: loop {
        match pc {
            0x82666A00 => {
    //   block [0x82666A00..0x82666A70)
	// 82666A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A10: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666A14: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82666A1C: 390B7F78  addi r8, r11, 0x7f78
	ctx.r[8].s64 = ctx.r[11].s64 + 32632;
	// 82666A20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82666A24: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 82666A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666A2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666A38: 386AD840  addi r3, r10, -0x27c0
	ctx.r[3].s64 = ctx.r[10].s64 + -10176;
	// 82666A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666A5C: 4BE003C5  bl 0x82466e20
	ctx.lr = 0x82666A60;
	sub_82466E20(ctx, base);
	// 82666A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666A70 size=100
    let mut pc: u32 = 0x82666A70;
    'dispatch: loop {
        match pc {
            0x82666A70 => {
    //   block [0x82666A70..0x82666AD4)
	// 82666A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666A7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666A84: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666A90: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 82666A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666AA4: 386AD870  addi r3, r10, -0x2790
	ctx.r[3].s64 = ctx.r[10].s64 + -10128;
	// 82666AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666AAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666AB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82666AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666AB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82666ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666AC0: 4BE00361  bl 0x82466e20
	ctx.lr = 0x82666AC4;
	sub_82466E20(ctx, base);
	// 82666AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666AD8 size=112
    let mut pc: u32 = 0x82666AD8;
    'dispatch: loop {
        match pc {
            0x82666AD8 => {
    //   block [0x82666AD8..0x82666B48)
	// 82666AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666AE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666AE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666AEC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666AF4: 390B7FD8  addi r8, r11, 0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32728;
	// 82666AF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666AFC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82666B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666B04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666B10: 386AD8A0  addi r3, r10, -0x2760
	ctx.r[3].s64 = ctx.r[10].s64 + -10080;
	// 82666B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666B34: 4BE002ED  bl 0x82466e20
	ctx.lr = 0x82666B38;
	sub_82466E20(ctx, base);
	// 82666B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666B48 size=112
    let mut pc: u32 = 0x82666B48;
    'dispatch: loop {
        match pc {
            0x82666B48 => {
    //   block [0x82666B48..0x82666BB8)
	// 82666B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82666B5C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666B64: 390B7FF0  addi r8, r11, 0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 32752;
	// 82666B68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666B6C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 82666B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666B74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666B80: 386AD8D0  addi r3, r10, -0x2730
	ctx.r[3].s64 = ctx.r[10].s64 + -10032;
	// 82666B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666BA4: 4BE0027D  bl 0x82466e20
	ctx.lr = 0x82666BA8;
	sub_82466E20(ctx, base);
	// 82666BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666BB8 size=112
    let mut pc: u32 = 0x82666BB8;
    'dispatch: loop {
        match pc {
            0x82666BB8 => {
    //   block [0x82666BB8..0x82666C28)
	// 82666BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666BC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666BCC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666BD4: 390B8020  addi r8, r11, -0x7fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -32736;
	// 82666BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666BDC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82666BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666BE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666BF0: 386AD900  addi r3, r10, -0x2700
	ctx.r[3].s64 = ctx.r[10].s64 + -9984;
	// 82666BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666C14: 4BE0020D  bl 0x82466e20
	ctx.lr = 0x82666C18;
	sub_82466E20(ctx, base);
	// 82666C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666C28 size=112
    let mut pc: u32 = 0x82666C28;
    'dispatch: loop {
        match pc {
            0x82666C28 => {
    //   block [0x82666C28..0x82666C98)
	// 82666C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666C38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666C3C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666C44: 390B8050  addi r8, r11, -0x7fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -32688;
	// 82666C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666C4C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82666C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666C54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666C60: 386AD930  addi r3, r10, -0x26d0
	ctx.r[3].s64 = ctx.r[10].s64 + -9936;
	// 82666C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666C84: 4BE0019D  bl 0x82466e20
	ctx.lr = 0x82666C88;
	sub_82466E20(ctx, base);
	// 82666C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666C98 size=112
    let mut pc: u32 = 0x82666C98;
    'dispatch: loop {
        match pc {
            0x82666C98 => {
    //   block [0x82666C98..0x82666D08)
	// 82666C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666CA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666CAC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666CB4: 390B8080  addi r8, r11, -0x7f80
	ctx.r[8].s64 = ctx.r[11].s64 + -32640;
	// 82666CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666CBC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82666CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666CC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666CD0: 386AD960  addi r3, r10, -0x26a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9888;
	// 82666CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666CF4: 4BE0012D  bl 0x82466e20
	ctx.lr = 0x82666CF8;
	sub_82466E20(ctx, base);
	// 82666CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666D08 size=112
    let mut pc: u32 = 0x82666D08;
    'dispatch: loop {
        match pc {
            0x82666D08 => {
    //   block [0x82666D08..0x82666D78)
	// 82666D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666D1C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666D24: 390B8098  addi r8, r11, -0x7f68
	ctx.r[8].s64 = ctx.r[11].s64 + -32616;
	// 82666D28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666D2C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82666D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666D40: 386AD990  addi r3, r10, -0x2670
	ctx.r[3].s64 = ctx.r[10].s64 + -9840;
	// 82666D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666D64: 4BE000BD  bl 0x82466e20
	ctx.lr = 0x82666D68;
	sub_82466E20(ctx, base);
	// 82666D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666D78 size=112
    let mut pc: u32 = 0x82666D78;
    'dispatch: loop {
        match pc {
            0x82666D78 => {
    //   block [0x82666D78..0x82666DE8)
	// 82666D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666D84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666D88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666D8C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666D94: 390B80B0  addi r8, r11, -0x7f50
	ctx.r[8].s64 = ctx.r[11].s64 + -32592;
	// 82666D98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666D9C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 82666DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666DB0: 386AD9C0  addi r3, r10, -0x2640
	ctx.r[3].s64 = ctx.r[10].s64 + -9792;
	// 82666DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666DD4: 4BE0004D  bl 0x82466e20
	ctx.lr = 0x82666DD8;
	sub_82466E20(ctx, base);
	// 82666DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666DE8 size=112
    let mut pc: u32 = 0x82666DE8;
    'dispatch: loop {
        match pc {
            0x82666DE8 => {
    //   block [0x82666DE8..0x82666E58)
	// 82666DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666DF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666DFC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666E04: 390B80F8  addi r8, r11, -0x7f08
	ctx.r[8].s64 = ctx.r[11].s64 + -32520;
	// 82666E08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82666E0C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82666E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666E20: 386AD9F0  addi r3, r10, -0x2610
	ctx.r[3].s64 = ctx.r[10].s64 + -9744;
	// 82666E24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666E44: 4BDFFFDD  bl 0x82466e20
	ctx.lr = 0x82666E48;
	sub_82466E20(ctx, base);
	// 82666E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666E58 size=112
    let mut pc: u32 = 0x82666E58;
    'dispatch: loop {
        match pc {
            0x82666E58 => {
    //   block [0x82666E58..0x82666EC8)
	// 82666E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666E64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666E6C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666E74: 390B8140  addi r8, r11, -0x7ec0
	ctx.r[8].s64 = ctx.r[11].s64 + -32448;
	// 82666E78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82666E7C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 82666E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666E90: 386ADA20  addi r3, r10, -0x25e0
	ctx.r[3].s64 = ctx.r[10].s64 + -9696;
	// 82666E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666EB4: 4BDFFF6D  bl 0x82466e20
	ctx.lr = 0x82666EB8;
	sub_82466E20(ctx, base);
	// 82666EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666EC8 size=112
    let mut pc: u32 = 0x82666EC8;
    'dispatch: loop {
        match pc {
            0x82666EC8 => {
    //   block [0x82666EC8..0x82666F38)
	// 82666EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666ED8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82666EDC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666EE4: 390B8158  addi r8, r11, -0x7ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -32424;
	// 82666EE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82666EEC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82666EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82666EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666F00: 386ADA50  addi r3, r10, -0x25b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9648;
	// 82666F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82666F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82666F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82666F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666F24: 4BDFFEFD  bl 0x82466e20
	ctx.lr = 0x82666F28;
	sub_82466E20(ctx, base);
	// 82666F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666F38 size=116
    let mut pc: u32 = 0x82666F38;
    'dispatch: loop {
        match pc {
            0x82666F38 => {
    //   block [0x82666F38..0x82666FAC)
	// 82666F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666F44: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666F48: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82666F4C: 390A8188  addi r8, r10, -0x7e78
	ctx.r[8].s64 = ctx.r[10].s64 + -32376;
	// 82666F50: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666F54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666F58: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666F60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666F6C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82666F70: 396BF718  addi r11, r11, -0x8e8
	ctx.r[11].s64 = ctx.r[11].s64 + -2280;
	// 82666F74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666F78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666F7C: 386ADA80  addi r3, r10, -0x2580
	ctx.r[3].s64 = ctx.r[10].s64 + -9600;
	// 82666F80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666F84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82666F88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82666F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82666F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82666F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82666F98: 4BDFFE89  bl 0x82466e20
	ctx.lr = 0x82666F9C;
	sub_82466E20(ctx, base);
	// 82666F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82666FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82666FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82666FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82666FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82666FB0 size=116
    let mut pc: u32 = 0x82666FB0;
    'dispatch: loop {
        match pc {
            0x82666FB0 => {
    //   block [0x82666FB0..0x82667024)
	// 82666FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82666FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82666FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82666FBC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82666FC0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82666FC4: 390A8200  addi r8, r10, -0x7e00
	ctx.r[8].s64 = ctx.r[10].s64 + -32256;
	// 82666FC8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666FCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82666FD0: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82666FD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82666FD8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82666FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82666FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82666FE4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 82666FE8: 396BF730  addi r11, r11, -0x8d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2256;
	// 82666FEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82666FF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82666FF4: 386ADAB0  addi r3, r10, -0x2550
	ctx.r[3].s64 = ctx.r[10].s64 + -9552;
	// 82666FF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82666FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667000: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82667004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266700C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667010: 4BDFFE11  bl 0x82466e20
	ctx.lr = 0x82667014;
	sub_82466E20(ctx, base);
	// 82667014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266701C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667028 size=24
    let mut pc: u32 = 0x82667028;
    'dispatch: loop {
        match pc {
            0x82667028 => {
    //   block [0x82667028..0x82667040)
	// 82667028: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266702C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667030: 394AC758  addi r10, r10, -0x38a8
	ctx.r[10].s64 = ctx.r[10].s64 + -14504;
	// 82667034: 816B7F74  lwz r11, 0x7f74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32628 as u32) ) } as u64;
	// 82667038: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266703C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667040 size=116
    let mut pc: u32 = 0x82667040;
    'dispatch: loop {
        match pc {
            0x82667040 => {
    //   block [0x82667040..0x826670B4)
	// 82667040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266704C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82667050: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667054: 392BF75C  addi r9, r11, -0x8a4
	ctx.r[9].s64 = ctx.r[11].s64 + -2212;
	// 82667058: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 8266705C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667060: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82667064: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82667068: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266706C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82667070: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667074: 396BC758  addi r11, r11, -0x38a8
	ctx.r[11].s64 = ctx.r[11].s64 + -14504;
	// 82667078: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8266707C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667080: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82667084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667088: 386ADAE0  addi r3, r10, -0x2520
	ctx.r[3].s64 = ctx.r[10].s64 + -9504;
	// 8266708C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667090: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82667094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667098: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266709C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826670A0: 4BDFFD81  bl 0x82466e20
	ctx.lr = 0x826670A4;
	sub_82466E20(ctx, base);
	// 826670A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826670A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826670AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826670B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826670B8 size=112
    let mut pc: u32 = 0x826670B8;
    'dispatch: loop {
        match pc {
            0x826670B8 => {
    //   block [0x826670B8..0x82667128)
	// 826670B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826670BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826670C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826670C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826670C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826670CC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 826670D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826670D4: 390B8290  addi r8, r11, -0x7d70
	ctx.r[8].s64 = ctx.r[11].s64 + -32112;
	// 826670D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826670DC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826670E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826670E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826670E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826670EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826670F0: 386ADB10  addi r3, r10, -0x24f0
	ctx.r[3].s64 = ctx.r[10].s64 + -9456;
	// 826670F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826670F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826670FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266710C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667114: 4BDFFD0D  bl 0x82466e20
	ctx.lr = 0x82667118;
	sub_82466E20(ctx, base);
	// 82667118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266711C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667128 size=112
    let mut pc: u32 = 0x82667128;
    'dispatch: loop {
        match pc {
            0x82667128 => {
    //   block [0x82667128..0x82667198)
	// 82667128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667138: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266713C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667144: 390B82F0  addi r8, r11, -0x7d10
	ctx.r[8].s64 = ctx.r[11].s64 + -32016;
	// 82667148: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266714C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82667150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266715C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667160: 386ADB40  addi r3, r10, -0x24c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9408;
	// 82667164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266716C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266717C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667184: 4BDFFC9D  bl 0x82466e20
	ctx.lr = 0x82667188;
	sub_82466E20(ctx, base);
	// 82667188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266718C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667198 size=112
    let mut pc: u32 = 0x82667198;
    'dispatch: loop {
        match pc {
            0x82667198 => {
    //   block [0x82667198..0x82667208)
	// 82667198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266719C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826671A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826671A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826671A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826671AC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 826671B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826671B4: 390B8398  addi r8, r11, -0x7c68
	ctx.r[8].s64 = ctx.r[11].s64 + -31848;
	// 826671B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826671BC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826671C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826671C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826671C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826671CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826671D0: 386ADB70  addi r3, r10, -0x2490
	ctx.r[3].s64 = ctx.r[10].s64 + -9360;
	// 826671D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826671D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826671DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826671E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826671E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826671E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826671EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826671F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826671F4: 4BDFFC2D  bl 0x82466e20
	ctx.lr = 0x826671F8;
	sub_82466E20(ctx, base);
	// 826671F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826671FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667208 size=112
    let mut pc: u32 = 0x82667208;
    'dispatch: loop {
        match pc {
            0x82667208 => {
    //   block [0x82667208..0x82667278)
	// 82667208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667218: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266721C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667224: 390B8410  addi r8, r11, -0x7bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -31728;
	// 82667228: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266722C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82667230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667234: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266723C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667240: 386ADBA0  addi r3, r10, -0x2460
	ctx.r[3].s64 = ctx.r[10].s64 + -9312;
	// 82667244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266724C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266725C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667264: 4BDFFBBD  bl 0x82466e20
	ctx.lr = 0x82667268;
	sub_82466E20(ctx, base);
	// 82667268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667278 size=112
    let mut pc: u32 = 0x82667278;
    'dispatch: loop {
        match pc {
            0x82667278 => {
    //   block [0x82667278..0x826672E8)
	// 82667278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667288: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266728C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667294: 390B8458  addi r8, r11, -0x7ba8
	ctx.r[8].s64 = ctx.r[11].s64 + -31656;
	// 82667298: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266729C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826672A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826672A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826672A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826672AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826672B0: 386ADBD0  addi r3, r10, -0x2430
	ctx.r[3].s64 = ctx.r[10].s64 + -9264;
	// 826672B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826672B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826672BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826672C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826672C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826672C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826672CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826672D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826672D4: 4BDFFB4D  bl 0x82466e20
	ctx.lr = 0x826672D8;
	sub_82466E20(ctx, base);
	// 826672D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826672DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826672E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826672E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826672E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826672E8 size=112
    let mut pc: u32 = 0x826672E8;
    'dispatch: loop {
        match pc {
            0x826672E8 => {
    //   block [0x826672E8..0x82667358)
	// 826672E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826672EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826672F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826672F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826672F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826672FC: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667304: 390B84E8  addi r8, r11, -0x7b18
	ctx.r[8].s64 = ctx.r[11].s64 + -31512;
	// 82667308: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266730C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82667310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667314: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266731C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667320: 386ADC00  addi r3, r10, -0x2400
	ctx.r[3].s64 = ctx.r[10].s64 + -9216;
	// 82667324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266732C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266733C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667344: 4BDFFADD  bl 0x82466e20
	ctx.lr = 0x82667348;
	sub_82466E20(ctx, base);
	// 82667348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266734C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667358 size=112
    let mut pc: u32 = 0x82667358;
    'dispatch: loop {
        match pc {
            0x82667358 => {
    //   block [0x82667358..0x826673C8)
	// 82667358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266735C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667368: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266736C: 38AAD7B0  addi r5, r10, -0x2850
	ctx.r[5].s64 = ctx.r[10].s64 + -10320;
	// 82667370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667374: 390B8548  addi r8, r11, -0x7ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -31416;
	// 82667378: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266737C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82667380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667384: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266738C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667390: 386ADC30  addi r3, r10, -0x23d0
	ctx.r[3].s64 = ctx.r[10].s64 + -9168;
	// 82667394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826673A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826673A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826673A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826673AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826673B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826673B4: 4BDFFA6D  bl 0x82466e20
	ctx.lr = 0x826673B8;
	sub_82466E20(ctx, base);
	// 826673B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826673BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826673C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826673C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826673C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826673C8 size=112
    let mut pc: u32 = 0x826673C8;
    'dispatch: loop {
        match pc {
            0x826673C8 => {
    //   block [0x826673C8..0x82667438)
	// 826673C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826673CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826673D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826673D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826673D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826673DC: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 826673E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826673E4: 390B85A8  addi r8, r11, -0x7a58
	ctx.r[8].s64 = ctx.r[11].s64 + -31320;
	// 826673E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826673EC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826673F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826673F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826673F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826673FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667400: 386ADC60  addi r3, r10, -0x23a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9120;
	// 82667404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266740C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266741C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667424: 4BDFF9FD  bl 0x82466e20
	ctx.lr = 0x82667428;
	sub_82466E20(ctx, base);
	// 82667428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266742C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667438 size=112
    let mut pc: u32 = 0x82667438;
    'dispatch: loop {
        match pc {
            0x82667438 => {
    //   block [0x82667438..0x826674A8)
	// 82667438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667448: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266744C: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 82667450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667454: 390B85D8  addi r8, r11, -0x7a28
	ctx.r[8].s64 = ctx.r[11].s64 + -31272;
	// 82667458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266745C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82667460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266746C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667470: 386ADC90  addi r3, r10, -0x2370
	ctx.r[3].s64 = ctx.r[10].s64 + -9072;
	// 82667474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266747C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266748C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667494: 4BDFF98D  bl 0x82466e20
	ctx.lr = 0x82667498;
	sub_82466E20(ctx, base);
	// 82667498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266749C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826674A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826674A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826674A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826674A8 size=100
    let mut pc: u32 = 0x826674A8;
    'dispatch: loop {
        match pc {
            0x826674A8 => {
    //   block [0x826674A8..0x8266750C)
	// 826674A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826674AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826674B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826674B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826674B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826674BC: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 826674C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826674C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826674C8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826674CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826674D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826674D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826674D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826674DC: 386ADCC0  addi r3, r10, -0x2340
	ctx.r[3].s64 = ctx.r[10].s64 + -9024;
	// 826674E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826674E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826674E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826674EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826674F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826674F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826674F8: 4BDFF929  bl 0x82466e20
	ctx.lr = 0x826674FC;
	sub_82466E20(ctx, base);
	// 826674FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667510 size=112
    let mut pc: u32 = 0x82667510;
    'dispatch: loop {
        match pc {
            0x82667510 => {
    //   block [0x82667510..0x82667580)
	// 82667510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266751C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667520: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667524: 38AADC30  addi r5, r10, -0x23d0
	ctx.r[5].s64 = ctx.r[10].s64 + -9168;
	// 82667528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266752C: 390B8620  addi r8, r11, -0x79e0
	ctx.r[8].s64 = ctx.r[11].s64 + -31200;
	// 82667530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667534: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 82667538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266753C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667548: 386ADCF0  addi r3, r10, -0x2310
	ctx.r[3].s64 = ctx.r[10].s64 + -8976;
	// 8266754C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266755C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266756C: 4BDFF8B5  bl 0x82466e20
	ctx.lr = 0x82667570;
	sub_82466E20(ctx, base);
	// 82667570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266757C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667580 size=108
    let mut pc: u32 = 0x82667580;
    'dispatch: loop {
        match pc {
            0x82667580 => {
    //   block [0x82667580..0x826675EC)
	// 82667580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266758C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667590: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82667594: 38EB8638  addi r7, r11, -0x79c8
	ctx.r[7].s64 = ctx.r[11].s64 + -31176;
	// 82667598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266759C: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826675A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826675A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826675A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826675AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826675B0: 386ADD20  addi r3, r10, -0x22e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8928;
	// 826675B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826675B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826675BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826675C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826675C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826675C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826675CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826675D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826675D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826675D8: 4BDFF849  bl 0x82466e20
	ctx.lr = 0x826675DC;
	sub_82466E20(ctx, base);
	// 826675DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826675E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826675E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826675E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826675F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826675F0 size=112
    let mut pc: u32 = 0x826675F0;
    'dispatch: loop {
        match pc {
            0x826675F0 => {
    //   block [0x826675F0..0x82667660)
	// 826675F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826675F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826675F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826675FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667600: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667604: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266760C: 390B8680  addi r8, r11, -0x7980
	ctx.r[8].s64 = ctx.r[11].s64 + -31104;
	// 82667610: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82667614: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82667618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266761C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667628: 386ADD50  addi r3, r10, -0x22b0
	ctx.r[3].s64 = ctx.r[10].s64 + -8880;
	// 8266762C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266763C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266764C: 4BDFF7D5  bl 0x82466e20
	ctx.lr = 0x82667650;
	sub_82466E20(ctx, base);
	// 82667650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667660 size=112
    let mut pc: u32 = 0x82667660;
    'dispatch: loop {
        match pc {
            0x82667660 => {
    //   block [0x82667660..0x826676D0)
	// 82667660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266766C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667670: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667674: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 82667678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266767C: 390B86E0  addi r8, r11, -0x7920
	ctx.r[8].s64 = ctx.r[11].s64 + -31008;
	// 82667680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667684: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82667688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266768C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667698: 386ADD80  addi r3, r10, -0x2280
	ctx.r[3].s64 = ctx.r[10].s64 + -8832;
	// 8266769C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826676A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826676A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826676A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826676AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826676B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826676B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826676B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826676BC: 4BDFF765  bl 0x82466e20
	ctx.lr = 0x826676C0;
	sub_82466E20(ctx, base);
	// 826676C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826676C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826676C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826676CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826676D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826676D0 size=112
    let mut pc: u32 = 0x826676D0;
    'dispatch: loop {
        match pc {
            0x826676D0 => {
    //   block [0x826676D0..0x82667740)
	// 826676D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826676D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826676D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826676DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826676E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826676E4: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 826676E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826676EC: 390B86F8  addi r8, r11, -0x7908
	ctx.r[8].s64 = ctx.r[11].s64 + -30984;
	// 826676F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826676F4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826676F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826676FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667708: 386ADDB0  addi r3, r10, -0x2250
	ctx.r[3].s64 = ctx.r[10].s64 + -8784;
	// 8266770C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266771C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266772C: 4BDFF6F5  bl 0x82466e20
	ctx.lr = 0x82667730;
	sub_82466E20(ctx, base);
	// 82667730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266773C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667740 size=112
    let mut pc: u32 = 0x82667740;
    'dispatch: loop {
        match pc {
            0x82667740 => {
    //   block [0x82667740..0x826677B0)
	// 82667740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266774C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667750: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667754: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 82667758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266775C: 390B8728  addi r8, r11, -0x78d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30936;
	// 82667760: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667764: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82667768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266776C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667778: 386ADDE0  addi r3, r10, -0x2220
	ctx.r[3].s64 = ctx.r[10].s64 + -8736;
	// 8266777C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266778C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266779C: 4BDFF685  bl 0x82466e20
	ctx.lr = 0x826677A0;
	sub_82466E20(ctx, base);
	// 826677A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826677A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826677A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826677AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826677B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826677B0 size=24
    let mut pc: u32 = 0x826677B0;
    'dispatch: loop {
        match pc {
            0x826677B0 => {
    //   block [0x826677B0..0x826677C8)
	// 826677B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826677B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826677B8: 394AC800  addi r10, r10, -0x3800
	ctx.r[10].s64 = ctx.r[10].s64 + -14336;
	// 826677BC: 816B8740  lwz r11, -0x78c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30912 as u32) ) } as u64;
	// 826677C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826677C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826677C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826677C8 size=112
    let mut pc: u32 = 0x826677C8;
    'dispatch: loop {
        match pc {
            0x826677C8 => {
    //   block [0x826677C8..0x82667838)
	// 826677C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826677CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826677D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826677D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826677D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826677DC: 392AF7B8  addi r9, r10, -0x848
	ctx.r[9].s64 = ctx.r[10].s64 + -2120;
	// 826677E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826677E4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 826677E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826677EC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826677F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826677F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826677F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826677FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667800: 386ADE10  addi r3, r10, -0x21f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8688;
	// 82667804: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667808: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266780C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266781C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667824: 4BDFF5FD  bl 0x82466e20
	ctx.lr = 0x82667828;
	sub_82466E20(ctx, base);
	// 82667828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266782C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667838 size=108
    let mut pc: u32 = 0x82667838;
    'dispatch: loop {
        match pc {
            0x82667838 => {
    //   block [0x82667838..0x826678A4)
	// 82667838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667844: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266784C: 38EB8744  addi r7, r11, -0x78bc
	ctx.r[7].s64 = ctx.r[11].s64 + -30908;
	// 82667850: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82667854: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82667858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266785C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667868: 386ADE40  addi r3, r10, -0x21c0
	ctx.r[3].s64 = ctx.r[10].s64 + -8640;
	// 8266786C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266787C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266788C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667890: 4BDFF591  bl 0x82466e20
	ctx.lr = 0x82667894;
	sub_82466E20(ctx, base);
	// 82667894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266789C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826678A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826678A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826678A8 size=108
    let mut pc: u32 = 0x826678A8;
    'dispatch: loop {
        match pc {
            0x826678A8 => {
    //   block [0x826678A8..0x82667914)
	// 826678A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826678AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826678B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826678B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826678B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826678BC: 38EB8760  addi r7, r11, -0x78a0
	ctx.r[7].s64 = ctx.r[11].s64 + -30880;
	// 826678C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826678C4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826678C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826678CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826678D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826678D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826678D8: 386ADE70  addi r3, r10, -0x2190
	ctx.r[3].s64 = ctx.r[10].s64 + -8592;
	// 826678DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826678E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826678E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826678E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826678EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826678F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826678F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826678F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826678FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667900: 4BDFF521  bl 0x82466e20
	ctx.lr = 0x82667904;
	sub_82466E20(ctx, base);
	// 82667904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266790C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667918 size=116
    let mut pc: u32 = 0x82667918;
    'dispatch: loop {
        match pc {
            0x82667918 => {
    //   block [0x82667918..0x8266798C)
	// 82667918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266791C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667924: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667928: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266792C: 390B87A8  addi r8, r11, -0x7858
	ctx.r[8].s64 = ctx.r[11].s64 + -30808;
	// 82667930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667934: 392AF870  addi r9, r10, -0x790
	ctx.r[9].s64 = ctx.r[10].s64 + -1936;
	// 82667938: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266793C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82667940: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667944: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266794C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266795C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82667960: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82667964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667968: 386BDEA0  addi r3, r11, -0x2160
	ctx.r[3].s64 = ctx.r[11].s64 + -8544;
	// 8266796C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667970: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667978: 4BDFF4A9  bl 0x82466e20
	ctx.lr = 0x8266797C;
	sub_82466E20(ctx, base);
	// 8266797C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667990 size=24
    let mut pc: u32 = 0x82667990;
    'dispatch: loop {
        match pc {
            0x82667990 => {
    //   block [0x82667990..0x826679A8)
	// 82667990: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667994: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667998: 394AC848  addi r10, r10, -0x37b8
	ctx.r[10].s64 = ctx.r[10].s64 + -14264;
	// 8266799C: 816B87C0  lwz r11, -0x7840(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30784 as u32) ) } as u64;
	// 826679A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826679A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826679A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826679A8 size=116
    let mut pc: u32 = 0x826679A8;
    'dispatch: loop {
        match pc {
            0x826679A8 => {
    //   block [0x826679A8..0x82667A1C)
	// 826679A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826679AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826679B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826679B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826679B8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826679BC: 390BC848  addi r8, r11, -0x37b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14264;
	// 826679C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826679C4: 392AF8E0  addi r9, r10, -0x720
	ctx.r[9].s64 = ctx.r[10].s64 + -1824;
	// 826679C8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826679CC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826679D0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826679D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826679D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826679DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826679E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826679E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826679E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826679EC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826679F0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826679F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826679F8: 386BDED0  addi r3, r11, -0x2130
	ctx.r[3].s64 = ctx.r[11].s64 + -8496;
	// 826679FC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82667A00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667A08: 4BDFF419  bl 0x82466e20
	ctx.lr = 0x82667A0C;
	sub_82466E20(ctx, base);
	// 82667A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667A20 size=108
    let mut pc: u32 = 0x82667A20;
    'dispatch: loop {
        match pc {
            0x82667A20 => {
    //   block [0x82667A20..0x82667A8C)
	// 82667A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667A2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667A34: 38EB87D0  addi r7, r11, -0x7830
	ctx.r[7].s64 = ctx.r[11].s64 + -30768;
	// 82667A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82667A3C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 82667A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667A44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667A50: 386ADF00  addi r3, r10, -0x2100
	ctx.r[3].s64 = ctx.r[10].s64 + -8448;
	// 82667A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667A78: 4BDFF3A9  bl 0x82466e20
	ctx.lr = 0x82667A7C;
	sub_82466E20(ctx, base);
	// 82667A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667A90 size=112
    let mut pc: u32 = 0x82667A90;
    'dispatch: loop {
        match pc {
            0x82667A90 => {
    //   block [0x82667A90..0x82667B00)
	// 82667A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667AA0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667AA4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667AAC: 390B8800  addi r8, r11, -0x7800
	ctx.r[8].s64 = ctx.r[11].s64 + -30720;
	// 82667AB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667AB4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82667AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667ABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667AC8: 386ADF30  addi r3, r10, -0x20d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8400;
	// 82667ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667AEC: 4BDFF335  bl 0x82466e20
	ctx.lr = 0x82667AF0;
	sub_82466E20(ctx, base);
	// 82667AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667B00 size=112
    let mut pc: u32 = 0x82667B00;
    'dispatch: loop {
        match pc {
            0x82667B00 => {
    //   block [0x82667B00..0x82667B70)
	// 82667B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667B0C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667B10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667B14: 392AF938  addi r9, r10, -0x6c8
	ctx.r[9].s64 = ctx.r[10].s64 + -1736;
	// 82667B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667B1C: 390B8820  addi r8, r11, -0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30688;
	// 82667B20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82667B24: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 82667B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667B38: 386ADF60  addi r3, r10, -0x20a0
	ctx.r[3].s64 = ctx.r[10].s64 + -8352;
	// 82667B3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667B40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667B5C: 4BDFF2C5  bl 0x82466e20
	ctx.lr = 0x82667B60;
	sub_82466E20(ctx, base);
	// 82667B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667B70 size=112
    let mut pc: u32 = 0x82667B70;
    'dispatch: loop {
        match pc {
            0x82667B70 => {
    //   block [0x82667B70..0x82667BE0)
	// 82667B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667B80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667B84: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667B8C: 390B8868  addi r8, r11, -0x7798
	ctx.r[8].s64 = ctx.r[11].s64 + -30616;
	// 82667B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667B94: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82667B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667B9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667BA8: 386ADF90  addi r3, r10, -0x2070
	ctx.r[3].s64 = ctx.r[10].s64 + -8304;
	// 82667BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667BCC: 4BDFF255  bl 0x82466e20
	ctx.lr = 0x82667BD0;
	sub_82466E20(ctx, base);
	// 82667BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667BE0 size=112
    let mut pc: u32 = 0x82667BE0;
    'dispatch: loop {
        match pc {
            0x82667BE0 => {
    //   block [0x82667BE0..0x82667C50)
	// 82667BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667BEC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667BF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667BF4: 392AF964  addi r9, r10, -0x69c
	ctx.r[9].s64 = ctx.r[10].s64 + -1692;
	// 82667BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667BFC: 390B8880  addi r8, r11, -0x7780
	ctx.r[8].s64 = ctx.r[11].s64 + -30592;
	// 82667C00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82667C04: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 82667C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667C18: 386ADFC0  addi r3, r10, -0x2040
	ctx.r[3].s64 = ctx.r[10].s64 + -8256;
	// 82667C1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667C20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667C3C: 4BDFF1E5  bl 0x82466e20
	ctx.lr = 0x82667C40;
	sub_82466E20(ctx, base);
	// 82667C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667C50 size=112
    let mut pc: u32 = 0x82667C50;
    'dispatch: loop {
        match pc {
            0x82667C50 => {
    //   block [0x82667C50..0x82667CC0)
	// 82667C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667C64: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667C6C: 390B8910  addi r8, r11, -0x76f0
	ctx.r[8].s64 = ctx.r[11].s64 + -30448;
	// 82667C70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667C74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82667C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667C88: 386ADFF0  addi r3, r10, -0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + -8208;
	// 82667C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667CAC: 4BDFF175  bl 0x82466e20
	ctx.lr = 0x82667CB0;
	sub_82466E20(ctx, base);
	// 82667CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667CC0 size=112
    let mut pc: u32 = 0x82667CC0;
    'dispatch: loop {
        match pc {
            0x82667CC0 => {
    //   block [0x82667CC0..0x82667D30)
	// 82667CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667CD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667CD4: 38AAE050  addi r5, r10, -0x1fb0
	ctx.r[5].s64 = ctx.r[10].s64 + -8112;
	// 82667CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667CDC: 390B8928  addi r8, r11, -0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30424;
	// 82667CE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82667CE4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82667CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667CF8: 386AE020  addi r3, r10, -0x1fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -8160;
	// 82667CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667D1C: 4BDFF105  bl 0x82466e20
	ctx.lr = 0x82667D20;
	sub_82466E20(ctx, base);
	// 82667D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667D30 size=100
    let mut pc: u32 = 0x82667D30;
    'dispatch: loop {
        match pc {
            0x82667D30 => {
    //   block [0x82667D30..0x82667D94)
	// 82667D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667D44: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82667D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667D50: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82667D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667D64: 386AE050  addi r3, r10, -0x1fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -8112;
	// 82667D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667D70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82667D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667D78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82667D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667D80: 4BDFF0A1  bl 0x82466e20
	ctx.lr = 0x82667D84;
	sub_82466E20(ctx, base);
	// 82667D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82667D98 size=24
    let mut pc: u32 = 0x82667D98;
    'dispatch: loop {
        match pc {
            0x82667D98 => {
    //   block [0x82667D98..0x82667DB0)
	// 82667D98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667D9C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82667DA0: 394AC920  addi r10, r10, -0x36e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14048;
	// 82667DA4: 816B89A0  lwz r11, -0x7660(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30304 as u32) ) } as u64;
	// 82667DA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82667DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667DB0 size=116
    let mut pc: u32 = 0x82667DB0;
    'dispatch: loop {
        match pc {
            0x82667DB0 => {
    //   block [0x82667DB0..0x82667E24)
	// 82667DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667DBC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667DC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667DC4: 390BC920  addi r8, r11, -0x36e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14048;
	// 82667DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667DCC: 392AF9A0  addi r9, r10, -0x660
	ctx.r[9].s64 = ctx.r[10].s64 + -1632;
	// 82667DD0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667DD4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82667DD8: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667DDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667DE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667DF4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82667DF8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82667DFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667E00: 386BE080  addi r3, r11, -0x1f80
	ctx.r[3].s64 = ctx.r[11].s64 + -8064;
	// 82667E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667E08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667E10: 4BDFF011  bl 0x82466e20
	ctx.lr = 0x82667E14;
	sub_82466E20(ctx, base);
	// 82667E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667E28 size=108
    let mut pc: u32 = 0x82667E28;
    'dispatch: loop {
        match pc {
            0x82667E28 => {
    //   block [0x82667E28..0x82667E94)
	// 82667E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667E34: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667E3C: 38EB89A4  addi r7, r11, -0x765c
	ctx.r[7].s64 = ctx.r[11].s64 + -30300;
	// 82667E40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82667E44: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 82667E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667E4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82667E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667E58: 386AE0B0  addi r3, r10, -0x1f50
	ctx.r[3].s64 = ctx.r[10].s64 + -8016;
	// 82667E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82667E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667E80: 4BDFEFA1  bl 0x82466e20
	ctx.lr = 0x82667E84;
	sub_82466E20(ctx, base);
	// 82667E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667E98 size=112
    let mut pc: u32 = 0x82667E98;
    'dispatch: loop {
        match pc {
            0x82667E98 => {
    //   block [0x82667E98..0x82667F08)
	// 82667E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667EA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667EAC: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667EB4: 390B89D4  addi r8, r11, -0x762c
	ctx.r[8].s64 = ctx.r[11].s64 + -30252;
	// 82667EB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667EBC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82667EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667ED0: 386AE0E0  addi r3, r10, -0x1f20
	ctx.r[3].s64 = ctx.r[10].s64 + -7968;
	// 82667ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667EF4: 4BDFEF2D  bl 0x82466e20
	ctx.lr = 0x82667EF8;
	sub_82466E20(ctx, base);
	// 82667EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667F08 size=112
    let mut pc: u32 = 0x82667F08;
    'dispatch: loop {
        match pc {
            0x82667F08 => {
    //   block [0x82667F08..0x82667F78)
	// 82667F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667F14: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667F18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667F1C: 392AF9C4  addi r9, r10, -0x63c
	ctx.r[9].s64 = ctx.r[10].s64 + -1596;
	// 82667F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667F24: 390B89F0  addi r8, r11, -0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + -30224;
	// 82667F28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82667F2C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 82667F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667F40: 386AE110  addi r3, r10, -0x1ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -7920;
	// 82667F44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82667F48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82667F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82667F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667F64: 4BDFEEBD  bl 0x82466e20
	ctx.lr = 0x82667F68;
	sub_82466E20(ctx, base);
	// 82667F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667F78 size=112
    let mut pc: u32 = 0x82667F78;
    'dispatch: loop {
        match pc {
            0x82667F78 => {
    //   block [0x82667F78..0x82667FE8)
	// 82667F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667F88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667F8C: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82667F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82667F94: 390B8A98  addi r8, r11, -0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + -30056;
	// 82667F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82667F9C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82667FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82667FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82667FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82667FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82667FB0: 386AE140  addi r3, r10, -0x1ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -7872;
	// 82667FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82667FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82667FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82667FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82667FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82667FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82667FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82667FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82667FD4: 4BDFEE4D  bl 0x82466e20
	ctx.lr = 0x82667FD8;
	sub_82466E20(ctx, base);
	// 82667FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82667FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82667FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82667FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82667FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82667FE8 size=112
    let mut pc: u32 = 0x82667FE8;
    'dispatch: loop {
        match pc {
            0x82667FE8 => {
    //   block [0x82667FE8..0x82668058)
	// 82667FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82667FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82667FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82667FF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82667FF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82667FFC: 392AFA1C  addi r9, r10, -0x5e4
	ctx.r[9].s64 = ctx.r[10].s64 + -1508;
	// 82668000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668004: 390B8AB8  addi r8, r11, -0x7548
	ctx.r[8].s64 = ctx.r[11].s64 + -30024;
	// 82668008: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8266800C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82668010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668020: 386AE170  addi r3, r10, -0x1e90
	ctx.r[3].s64 = ctx.r[10].s64 + -7824;
	// 82668024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266802C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266803C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668044: 4BDFEDDD  bl 0x82466e20
	ctx.lr = 0x82668048;
	sub_82466E20(ctx, base);
	// 82668048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668058 size=116
    let mut pc: u32 = 0x82668058;
    'dispatch: loop {
        match pc {
            0x82668058 => {
    //   block [0x82668058..0x826680CC)
	// 82668058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668064: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668068: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266806C: 390B8B60  addi r8, r11, -0x74a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29856;
	// 82668070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668074: 392AF9F0  addi r9, r10, -0x610
	ctx.r[9].s64 = ctx.r[10].s64 + -1552;
	// 82668078: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266807C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82668080: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668084: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266808C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266809C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826680A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826680A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826680A8: 386BE1A0  addi r3, r11, -0x1e60
	ctx.r[3].s64 = ctx.r[11].s64 + -7776;
	// 826680AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826680B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826680B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826680B8: 4BDFED69  bl 0x82466e20
	ctx.lr = 0x826680BC;
	sub_82466E20(ctx, base);
	// 826680BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826680C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826680C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826680C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826680D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826680D0 size=108
    let mut pc: u32 = 0x826680D0;
    'dispatch: loop {
        match pc {
            0x826680D0 => {
    //   block [0x826680D0..0x8266813C)
	// 826680D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826680D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826680D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826680DC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826680E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826680E4: 38EB8B78  addi r7, r11, -0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + -29832;
	// 826680E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826680EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826680F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826680F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826680F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826680FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668100: 386AE1D0  addi r3, r10, -0x1e30
	ctx.r[3].s64 = ctx.r[10].s64 + -7728;
	// 82668104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266810C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266811C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668128: 4BDFECF9  bl 0x82466e20
	ctx.lr = 0x8266812C;
	sub_82466E20(ctx, base);
	// 8266812C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668140 size=112
    let mut pc: u32 = 0x82668140;
    'dispatch: loop {
        match pc {
            0x82668140 => {
    //   block [0x82668140..0x826681B0)
	// 82668140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266814C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668150: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668154: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266815C: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 82668160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668164: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82668168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266816C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668178: 386AE200  addi r3, r10, -0x1e00
	ctx.r[3].s64 = ctx.r[10].s64 + -7680;
	// 8266817C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266819C: 4BDFEC85  bl 0x82466e20
	ctx.lr = 0x826681A0;
	sub_82466E20(ctx, base);
	// 826681A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826681A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826681A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826681AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826681B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826681B0 size=112
    let mut pc: u32 = 0x826681B0;
    'dispatch: loop {
        match pc {
            0x826681B0 => {
    //   block [0x826681B0..0x82668220)
	// 826681B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826681B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826681B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826681BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826681C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826681C4: 392AFA50  addi r9, r10, -0x5b0
	ctx.r[9].s64 = ctx.r[10].s64 + -1456;
	// 826681C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826681CC: 390B8BC8  addi r8, r11, -0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + -29752;
	// 826681D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826681D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826681D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826681DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826681E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826681E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826681E8: 386AE230  addi r3, r10, -0x1dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -7632;
	// 826681EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826681F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826681F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826681F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826681FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266820C: 4BDFEC15  bl 0x82466e20
	ctx.lr = 0x82668210;
	sub_82466E20(ctx, base);
	// 82668210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668220 size=112
    let mut pc: u32 = 0x82668220;
    'dispatch: loop {
        match pc {
            0x82668220 => {
    //   block [0x82668220..0x82668290)
	// 82668220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266822C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668230: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668234: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266823C: 390B8C70  addi r8, r11, -0x7390
	ctx.r[8].s64 = ctx.r[11].s64 + -29584;
	// 82668240: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668244: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82668248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266824C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668258: 386AE260  addi r3, r10, -0x1da0
	ctx.r[3].s64 = ctx.r[10].s64 + -7584;
	// 8266825C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266827C: 4BDFEBA5  bl 0x82466e20
	ctx.lr = 0x82668280;
	sub_82466E20(ctx, base);
	// 82668280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266828C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668290 size=112
    let mut pc: u32 = 0x82668290;
    'dispatch: loop {
        match pc {
            0x82668290 => {
    //   block [0x82668290..0x82668300)
	// 82668290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266829C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826682A0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826682A4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826682A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826682AC: 390B8CB8  addi r8, r11, -0x7348
	ctx.r[8].s64 = ctx.r[11].s64 + -29512;
	// 826682B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826682B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826682B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826682BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826682C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826682C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826682C8: 386AE290  addi r3, r10, -0x1d70
	ctx.r[3].s64 = ctx.r[10].s64 + -7536;
	// 826682CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826682D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826682D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826682D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826682DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826682E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826682E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826682E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826682EC: 4BDFEB35  bl 0x82466e20
	ctx.lr = 0x826682F0;
	sub_82466E20(ctx, base);
	// 826682F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826682F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826682F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826682FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668300 size=100
    let mut pc: u32 = 0x82668300;
    'dispatch: loop {
        match pc {
            0x82668300 => {
    //   block [0x82668300..0x82668364)
	// 82668300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266830C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668314: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668320: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82668324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668334: 386AE2C0  addi r3, r10, -0x1d40
	ctx.r[3].s64 = ctx.r[10].s64 + -7488;
	// 82668338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266833C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82668344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266834C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668350: 4BDFEAD1  bl 0x82466e20
	ctx.lr = 0x82668354;
	sub_82466E20(ctx, base);
	// 82668354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266835C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668368 size=112
    let mut pc: u32 = 0x82668368;
    'dispatch: loop {
        match pc {
            0x82668368 => {
    //   block [0x82668368..0x826683D8)
	// 82668368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668378: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266837C: 38AADED0  addi r5, r10, -0x2130
	ctx.r[5].s64 = ctx.r[10].s64 + -8496;
	// 82668380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668384: 390B8D78  addi r8, r11, -0x7288
	ctx.r[8].s64 = ctx.r[11].s64 + -29320;
	// 82668388: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266838C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82668390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266839C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826683A0: 386AE2F0  addi r3, r10, -0x1d10
	ctx.r[3].s64 = ctx.r[10].s64 + -7440;
	// 826683A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826683A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826683AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826683B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826683B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826683B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826683BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826683C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826683C4: 4BDFEA5D  bl 0x82466e20
	ctx.lr = 0x826683C8;
	sub_82466E20(ctx, base);
	// 826683C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826683CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826683D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826683D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826683D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826683D8 size=112
    let mut pc: u32 = 0x826683D8;
    'dispatch: loop {
        match pc {
            0x826683D8 => {
    //   block [0x826683D8..0x82668448)
	// 826683D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826683DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826683E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826683E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826683E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826683EC: 38AADD50  addi r5, r10, -0x22b0
	ctx.r[5].s64 = ctx.r[10].s64 + -8880;
	// 826683F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826683F4: 390B8DA8  addi r8, r11, -0x7258
	ctx.r[8].s64 = ctx.r[11].s64 + -29272;
	// 826683F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826683FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82668400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266840C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668410: 386AE320  addi r3, r10, -0x1ce0
	ctx.r[3].s64 = ctx.r[10].s64 + -7392;
	// 82668414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266842C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668434: 4BDFE9ED  bl 0x82466e20
	ctx.lr = 0x82668438;
	sub_82466E20(ctx, base);
	// 82668438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668448 size=108
    let mut pc: u32 = 0x82668448;
    'dispatch: loop {
        match pc {
            0x82668448 => {
    //   block [0x82668448..0x826684B4)
	// 82668448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668454: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266845C: 38EB8DC0  addi r7, r11, -0x7240
	ctx.r[7].s64 = ctx.r[11].s64 + -29248;
	// 82668460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82668464: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82668468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266846C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668478: 386AE350  addi r3, r10, -0x1cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -7344;
	// 8266847C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266849C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826684A0: 4BDFE981  bl 0x82466e20
	ctx.lr = 0x826684A4;
	sub_82466E20(ctx, base);
	// 826684A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826684A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826684AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826684B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826684B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826684B8 size=112
    let mut pc: u32 = 0x826684B8;
    'dispatch: loop {
        match pc {
            0x826684B8 => {
    //   block [0x826684B8..0x82668528)
	// 826684B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826684BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826684C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826684C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826684C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826684CC: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 826684D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826684D4: 390B8DF0  addi r8, r11, -0x7210
	ctx.r[8].s64 = ctx.r[11].s64 + -29200;
	// 826684D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826684DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826684E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826684E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826684E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826684EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826684F0: 386AE380  addi r3, r10, -0x1c80
	ctx.r[3].s64 = ctx.r[10].s64 + -7296;
	// 826684F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826684F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826684FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266850C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668514: 4BDFE90D  bl 0x82466e20
	ctx.lr = 0x82668518;
	sub_82466E20(ctx, base);
	// 82668518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266851C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668528 size=112
    let mut pc: u32 = 0x82668528;
    'dispatch: loop {
        match pc {
            0x82668528 => {
    //   block [0x82668528..0x82668598)
	// 82668528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266852C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668534: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668538: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266853C: 392AFA7C  addi r9, r10, -0x584
	ctx.r[9].s64 = ctx.r[10].s64 + -1412;
	// 82668540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668544: 390B8E80  addi r8, r11, -0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + -29056;
	// 82668548: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266854C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 82668550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266855C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668560: 386AE3B0  addi r3, r10, -0x1c50
	ctx.r[3].s64 = ctx.r[10].s64 + -7248;
	// 82668564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668568: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266856C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266857C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668584: 4BDFE89D  bl 0x82466e20
	ctx.lr = 0x82668588;
	sub_82466E20(ctx, base);
	// 82668588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266858C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668598 size=112
    let mut pc: u32 = 0x82668598;
    'dispatch: loop {
        match pc {
            0x82668598 => {
    //   block [0x82668598..0x82668608)
	// 82668598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826685A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826685A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826685A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826685AC: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826685B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826685B4: 390B8EC8  addi r8, r11, -0x7138
	ctx.r[8].s64 = ctx.r[11].s64 + -28984;
	// 826685B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826685BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826685C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826685C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826685C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826685CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826685D0: 386AE3E0  addi r3, r10, -0x1c20
	ctx.r[3].s64 = ctx.r[10].s64 + -7200;
	// 826685D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826685D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826685DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826685E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826685E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826685E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826685EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826685F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826685F4: 4BDFE82D  bl 0x82466e20
	ctx.lr = 0x826685F8;
	sub_82466E20(ctx, base);
	// 826685F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826685FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668608 size=108
    let mut pc: u32 = 0x82668608;
    'dispatch: loop {
        match pc {
            0x82668608 => {
    //   block [0x82668608..0x82668674)
	// 82668608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266861C: 38EB8EE0  addi r7, r11, -0x7120
	ctx.r[7].s64 = ctx.r[11].s64 + -28960;
	// 82668620: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82668624: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82668628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266862C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668638: 386AE410  addi r3, r10, -0x1bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -7152;
	// 8266863C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266864C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266865C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668660: 4BDFE7C1  bl 0x82466e20
	ctx.lr = 0x82668664;
	sub_82466E20(ctx, base);
	// 82668664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266866C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668678 size=116
    let mut pc: u32 = 0x82668678;
    'dispatch: loop {
        match pc {
            0x82668678 => {
    //   block [0x82668678..0x826686EC)
	// 82668678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266867C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668684: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668688: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8266868C: 390A8F70  addi r8, r10, -0x7090
	ctx.r[8].s64 = ctx.r[10].s64 + -28816;
	// 82668690: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668694: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82668698: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 8266869C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826686A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826686A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826686A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826686AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826686B0: 396BFA90  addi r11, r11, -0x570
	ctx.r[11].s64 = ctx.r[11].s64 + -1392;
	// 826686B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826686B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826686BC: 386AE440  addi r3, r10, -0x1bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -7104;
	// 826686C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826686C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826686C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826686CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826686D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826686D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826686D8: 4BDFE749  bl 0x82466e20
	ctx.lr = 0x826686DC;
	sub_82466E20(ctx, base);
	// 826686DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826686E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826686E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826686E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826686F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826686F0 size=112
    let mut pc: u32 = 0x826686F0;
    'dispatch: loop {
        match pc {
            0x826686F0 => {
    //   block [0x826686F0..0x82668760)
	// 826686F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826686F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826686F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826686FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668700: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668704: 392AFADC  addi r9, r10, -0x524
	ctx.r[9].s64 = ctx.r[10].s64 + -1316;
	// 82668708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266870C: 390B9050  addi r8, r11, -0x6fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -28592;
	// 82668710: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82668714: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82668718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266871C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668728: 386AE470  addi r3, r10, -0x1b90
	ctx.r[3].s64 = ctx.r[10].s64 + -7056;
	// 8266872C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668730: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266873C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266874C: 4BDFE6D5  bl 0x82466e20
	ctx.lr = 0x82668750;
	sub_82466E20(ctx, base);
	// 82668750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668760 size=112
    let mut pc: u32 = 0x82668760;
    'dispatch: loop {
        match pc {
            0x82668760 => {
    //   block [0x82668760..0x826687D0)
	// 82668760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266876C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668770: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668774: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266877C: 390B90B0  addi r8, r11, -0x6f50
	ctx.r[8].s64 = ctx.r[11].s64 + -28496;
	// 82668780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668784: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82668788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266878C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668798: 386AE4A0  addi r3, r10, -0x1b60
	ctx.r[3].s64 = ctx.r[10].s64 + -7008;
	// 8266879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826687A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826687A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826687A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826687AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826687B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826687B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826687B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826687BC: 4BDFE665  bl 0x82466e20
	ctx.lr = 0x826687C0;
	sub_82466E20(ctx, base);
	// 826687C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826687C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826687C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826687CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826687D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826687D0 size=108
    let mut pc: u32 = 0x826687D0;
    'dispatch: loop {
        match pc {
            0x826687D0 => {
    //   block [0x826687D0..0x8266883C)
	// 826687D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826687D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826687D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826687DC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826687E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826687E4: 38EB90C8  addi r7, r11, -0x6f38
	ctx.r[7].s64 = ctx.r[11].s64 + -28472;
	// 826687E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826687EC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826687F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826687F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826687F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826687FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668800: 386AE4D0  addi r3, r10, -0x1b30
	ctx.r[3].s64 = ctx.r[10].s64 + -6960;
	// 82668804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266880C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266881C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668828: 4BDFE5F9  bl 0x82466e20
	ctx.lr = 0x8266882C;
	sub_82466E20(ctx, base);
	// 8266882C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668840 size=112
    let mut pc: u32 = 0x82668840;
    'dispatch: loop {
        match pc {
            0x82668840 => {
    //   block [0x82668840..0x826688B0)
	// 82668840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266884C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668850: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668854: 38AAE2C0  addi r5, r10, -0x1d40
	ctx.r[5].s64 = ctx.r[10].s64 + -7488;
	// 82668858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266885C: 390B9110  addi r8, r11, -0x6ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -28400;
	// 82668860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82668864: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82668868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266886C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668878: 386AE500  addi r3, r10, -0x1b00
	ctx.r[3].s64 = ctx.r[10].s64 + -6912;
	// 8266887C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266888C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266889C: 4BDFE585  bl 0x82466e20
	ctx.lr = 0x826688A0;
	sub_82466E20(ctx, base);
	// 826688A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826688A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826688A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826688AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826688B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826688B0 size=112
    let mut pc: u32 = 0x826688B0;
    'dispatch: loop {
        match pc {
            0x826688B0 => {
    //   block [0x826688B0..0x82668920)
	// 826688B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826688B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826688B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826688BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826688C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826688C4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 826688C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826688CC: 390B9188  addi r8, r11, -0x6e78
	ctx.r[8].s64 = ctx.r[11].s64 + -28280;
	// 826688D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826688D4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826688D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826688DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826688E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826688E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826688E8: 386AE530  addi r3, r10, -0x1ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -6864;
	// 826688EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826688F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826688F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826688F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826688FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266890C: 4BDFE515  bl 0x82466e20
	ctx.lr = 0x82668910;
	sub_82466E20(ctx, base);
	// 82668910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266891C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668920 size=108
    let mut pc: u32 = 0x82668920;
    'dispatch: loop {
        match pc {
            0x82668920 => {
    //   block [0x82668920..0x8266898C)
	// 82668920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266892C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668934: 38EB91B8  addi r7, r11, -0x6e48
	ctx.r[7].s64 = ctx.r[11].s64 + -28232;
	// 82668938: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266893C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82668940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266894C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668950: 386AE560  addi r3, r10, -0x1aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -6816;
	// 82668954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266895C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266896C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668978: 4BDFE4A9  bl 0x82466e20
	ctx.lr = 0x8266897C;
	sub_82466E20(ctx, base);
	// 8266897C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668990 size=108
    let mut pc: u32 = 0x82668990;
    'dispatch: loop {
        match pc {
            0x82668990 => {
    //   block [0x82668990..0x826689FC)
	// 82668990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266899C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826689A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826689A4: 38EB9218  addi r7, r11, -0x6de8
	ctx.r[7].s64 = ctx.r[11].s64 + -28136;
	// 826689A8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826689AC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826689B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826689B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826689B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826689BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826689C0: 386AE590  addi r3, r10, -0x1a70
	ctx.r[3].s64 = ctx.r[10].s64 + -6768;
	// 826689C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826689C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826689CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826689D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826689D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826689D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826689DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826689E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826689E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826689E8: 4BDFE439  bl 0x82466e20
	ctx.lr = 0x826689EC;
	sub_82466E20(ctx, base);
	// 826689EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826689F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826689F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826689F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668A00 size=112
    let mut pc: u32 = 0x82668A00;
    'dispatch: loop {
        match pc {
            0x82668A00 => {
    //   block [0x82668A00..0x82668A70)
	// 82668A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668A10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A14: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668A1C: 390B9290  addi r8, r11, -0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + -28016;
	// 82668A20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668A24: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82668A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668A2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668A38: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 82668A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668A5C: 4BDFE3C5  bl 0x82466e20
	ctx.lr = 0x82668A60;
	sub_82466E20(ctx, base);
	// 82668A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82668A70 size=24
    let mut pc: u32 = 0x82668A70;
    'dispatch: loop {
        match pc {
            0x82668A70 => {
    //   block [0x82668A70..0x82668A88)
	// 82668A70: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A74: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668A78: 394AC998  addi r10, r10, -0x3668
	ctx.r[10].s64 = ctx.r[10].s64 + -13928;
	// 82668A7C: 816B904C  lwz r11, -0x6fb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28596 as u32) ) } as u64;
	// 82668A80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82668A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668A88 size=116
    let mut pc: u32 = 0x82668A88;
    'dispatch: loop {
        match pc {
            0x82668A88 => {
    //   block [0x82668A88..0x82668AFC)
	// 82668A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668A94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668A98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668A9C: 390BC998  addi r8, r11, -0x3668
	ctx.r[8].s64 = ctx.r[11].s64 + -13928;
	// 82668AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668AA4: 392AFB20  addi r9, r10, -0x4e0
	ctx.r[9].s64 = ctx.r[10].s64 + -1248;
	// 82668AA8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668AAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82668AB0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82668AB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668ABC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668ACC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82668AD0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82668AD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668AD8: 386BE5F0  addi r3, r11, -0x1a10
	ctx.r[3].s64 = ctx.r[11].s64 + -6672;
	// 82668ADC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668AE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668AE8: 4BDFE339  bl 0x82466e20
	ctx.lr = 0x82668AEC;
	sub_82466E20(ctx, base);
	// 82668AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668B00 size=112
    let mut pc: u32 = 0x82668B00;
    'dispatch: loop {
        match pc {
            0x82668B00 => {
    //   block [0x82668B00..0x82668B70)
	// 82668B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668B10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B14: 38AAE5F0  addi r5, r10, -0x1a10
	ctx.r[5].s64 = ctx.r[10].s64 + -6672;
	// 82668B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668B1C: 390B92D8  addi r8, r11, -0x6d28
	ctx.r[8].s64 = ctx.r[11].s64 + -27944;
	// 82668B20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82668B24: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82668B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668B38: 386AE620  addi r3, r10, -0x19e0
	ctx.r[3].s64 = ctx.r[10].s64 + -6624;
	// 82668B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668B5C: 4BDFE2C5  bl 0x82466e20
	ctx.lr = 0x82668B60;
	sub_82466E20(ctx, base);
	// 82668B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82668B70 size=24
    let mut pc: u32 = 0x82668B70;
    'dispatch: loop {
        match pc {
            0x82668B70 => {
    //   block [0x82668B70..0x82668B88)
	// 82668B70: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B74: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82668B78: 394AC9B0  addi r10, r10, -0x3650
	ctx.r[10].s64 = ctx.r[10].s64 + -13904;
	// 82668B7C: 816B9308  lwz r11, -0x6cf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27896 as u32) ) } as u64;
	// 82668B80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82668B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668B88 size=116
    let mut pc: u32 = 0x82668B88;
    'dispatch: loop {
        match pc {
            0x82668B88 => {
    //   block [0x82668B88..0x82668BFC)
	// 82668B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668B94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668B98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82668B9C: 390BC9B0  addi r8, r11, -0x3650
	ctx.r[8].s64 = ctx.r[11].s64 + -13904;
	// 82668BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668BA4: 392AFB5C  addi r9, r10, -0x4a4
	ctx.r[9].s64 = ctx.r[10].s64 + -1188;
	// 82668BA8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668BAC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82668BB0: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668BB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668BBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668BCC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82668BD0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 82668BD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82668BD8: 386BE650  addi r3, r11, -0x19b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6576;
	// 82668BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668BE8: 4BDFE239  bl 0x82466e20
	ctx.lr = 0x82668BEC;
	sub_82466E20(ctx, base);
	// 82668BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668C00 size=112
    let mut pc: u32 = 0x82668C00;
    'dispatch: loop {
        match pc {
            0x82668C00 => {
    //   block [0x82668C00..0x82668C70)
	// 82668C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668C14: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668C1C: 390B9310  addi r8, r11, -0x6cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27888;
	// 82668C20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82668C24: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82668C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668C2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668C38: 386AE680  addi r3, r10, -0x1980
	ctx.r[3].s64 = ctx.r[10].s64 + -6528;
	// 82668C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668C5C: 4BDFE1C5  bl 0x82466e20
	ctx.lr = 0x82668C60;
	sub_82466E20(ctx, base);
	// 82668C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668C70 size=112
    let mut pc: u32 = 0x82668C70;
    'dispatch: loop {
        match pc {
            0x82668C70 => {
    //   block [0x82668C70..0x82668CE0)
	// 82668C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668C80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668C84: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668C8C: 390B9370  addi r8, r11, -0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + -27792;
	// 82668C90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82668C94: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82668C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668C9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668CA8: 386AE6B0  addi r3, r10, -0x1950
	ctx.r[3].s64 = ctx.r[10].s64 + -6480;
	// 82668CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668CCC: 4BDFE155  bl 0x82466e20
	ctx.lr = 0x82668CD0;
	sub_82466E20(ctx, base);
	// 82668CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668CE0 size=112
    let mut pc: u32 = 0x82668CE0;
    'dispatch: loop {
        match pc {
            0x82668CE0 => {
    //   block [0x82668CE0..0x82668D50)
	// 82668CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668CF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668CF4: 38AAE620  addi r5, r10, -0x19e0
	ctx.r[5].s64 = ctx.r[10].s64 + -6624;
	// 82668CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668CFC: 390B93A0  addi r8, r11, -0x6c60
	ctx.r[8].s64 = ctx.r[11].s64 + -27744;
	// 82668D00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82668D04: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82668D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668D0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668D18: 386AE6E0  addi r3, r10, -0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + -6432;
	// 82668D1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668D3C: 4BDFE0E5  bl 0x82466e20
	ctx.lr = 0x82668D40;
	sub_82466E20(ctx, base);
	// 82668D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668D50 size=108
    let mut pc: u32 = 0x82668D50;
    'dispatch: loop {
        match pc {
            0x82668D50 => {
    //   block [0x82668D50..0x82668DBC)
	// 82668D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668D5C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668D64: 38EB93E8  addi r7, r11, -0x6c18
	ctx.r[7].s64 = ctx.r[11].s64 + -27672;
	// 82668D68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82668D6C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82668D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668D74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668D80: 386AE710  addi r3, r10, -0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6384;
	// 82668D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668DA8: 4BDFE079  bl 0x82466e20
	ctx.lr = 0x82668DAC;
	sub_82466E20(ctx, base);
	// 82668DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668DC0 size=112
    let mut pc: u32 = 0x82668DC0;
    'dispatch: loop {
        match pc {
            0x82668DC0 => {
    //   block [0x82668DC0..0x82668E30)
	// 82668DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668DCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668DD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668DD4: 38AADEA0  addi r5, r10, -0x2160
	ctx.r[5].s64 = ctx.r[10].s64 + -8544;
	// 82668DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668DDC: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 82668DE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82668DE4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82668DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668DEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82668DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668DF8: 386AE740  addi r3, r10, -0x18c0
	ctx.r[3].s64 = ctx.r[10].s64 + -6336;
	// 82668DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82668E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668E1C: 4BDFE005  bl 0x82466e20
	ctx.lr = 0x82668E20;
	sub_82466E20(ctx, base);
	// 82668E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668E30 size=108
    let mut pc: u32 = 0x82668E30;
    'dispatch: loop {
        match pc {
            0x82668E30 => {
    //   block [0x82668E30..0x82668E9C)
	// 82668E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668E3C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668E44: 38EB9430  addi r7, r11, -0x6bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -27600;
	// 82668E48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82668E4C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82668E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668E54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668E60: 386AE770  addi r3, r10, -0x1890
	ctx.r[3].s64 = ctx.r[10].s64 + -6288;
	// 82668E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668E88: 4BDFDF99  bl 0x82466e20
	ctx.lr = 0x82668E8C;
	sub_82466E20(ctx, base);
	// 82668E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668EA0 size=108
    let mut pc: u32 = 0x82668EA0;
    'dispatch: loop {
        match pc {
            0x82668EA0 => {
    //   block [0x82668EA0..0x82668F0C)
	// 82668EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668EAC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668EB4: 38EB9478  addi r7, r11, -0x6b88
	ctx.r[7].s64 = ctx.r[11].s64 + -27528;
	// 82668EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82668EBC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82668EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82668ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668ED0: 386AE7A0  addi r3, r10, -0x1860
	ctx.r[3].s64 = ctx.r[10].s64 + -6240;
	// 82668ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82668ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82668EF8: 4BDFDF29  bl 0x82466e20
	ctx.lr = 0x82668EFC;
	sub_82466E20(ctx, base);
	// 82668EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668F10 size=116
    let mut pc: u32 = 0x82668F10;
    'dispatch: loop {
        match pc {
            0x82668F10 => {
    //   block [0x82668F10..0x82668F84)
	// 82668F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668F1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82668F20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F24: 392BFB90  addi r9, r11, -0x470
	ctx.r[9].s64 = ctx.r[11].s64 + -1136;
	// 82668F28: 38AAEC20  addi r5, r10, -0x13e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5088;
	// 82668F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668F30: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82668F34: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 82668F38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82668F3C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82668F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668F44: 396B94D8  addi r11, r11, -0x6b28
	ctx.r[11].s64 = ctx.r[11].s64 + -27432;
	// 82668F48: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82668F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F50: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82668F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668F58: 386AE7D0  addi r3, r10, -0x1830
	ctx.r[3].s64 = ctx.r[10].s64 + -6192;
	// 82668F5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82668F60: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82668F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668F68: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82668F6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82668F70: 4BDFDEB1  bl 0x82466e20
	ctx.lr = 0x82668F74;
	sub_82466E20(ctx, base);
	// 82668F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668F88 size=100
    let mut pc: u32 = 0x82668F88;
    'dispatch: loop {
        match pc {
            0x82668F88 => {
    //   block [0x82668F88..0x82668FEC)
	// 82668F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668F94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82668F9C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82668FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82668FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82668FA8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82668FAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82668FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82668FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82668FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82668FBC: 386AE800  addi r3, r10, -0x1800
	ctx.r[3].s64 = ctx.r[10].s64 + -6144;
	// 82668FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82668FC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82668FC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82668FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82668FD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82668FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82668FD8: 4BDFDE49  bl 0x82466e20
	ctx.lr = 0x82668FDC;
	sub_82466E20(ctx, base);
	// 82668FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82668FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82668FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82668FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82668FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82668FF0 size=100
    let mut pc: u32 = 0x82668FF0;
    'dispatch: loop {
        match pc {
            0x82668FF0 => {
    //   block [0x82668FF0..0x82669054)
	// 82668FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82668FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82668FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82668FFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669004: 38AAE890  addi r5, r10, -0x1770
	ctx.r[5].s64 = ctx.r[10].s64 + -6000;
	// 82669008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266900C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669010: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82669014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266901C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669024: 386AE830  addi r3, r10, -0x17d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6096;
	// 82669028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266902C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266903C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669040: 4BDFDDE1  bl 0x82466e20
	ctx.lr = 0x82669044;
	sub_82466E20(ctx, base);
	// 82669044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669058 size=100
    let mut pc: u32 = 0x82669058;
    'dispatch: loop {
        match pc {
            0x82669058 => {
    //   block [0x82669058..0x826690BC)
	// 82669058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266906C: 38AAE7D0  addi r5, r10, -0x1830
	ctx.r[5].s64 = ctx.r[10].s64 + -6192;
	// 82669070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669078: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8266907C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266908C: 386AE860  addi r3, r10, -0x17a0
	ctx.r[3].s64 = ctx.r[10].s64 + -6048;
	// 82669090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669098: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266909C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826690A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826690A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826690A8: 4BDFDD79  bl 0x82466e20
	ctx.lr = 0x826690AC;
	sub_82466E20(ctx, base);
	// 826690AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826690B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826690B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826690B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826690C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826690C0 size=104
    let mut pc: u32 = 0x826690C0;
    'dispatch: loop {
        match pc {
            0x826690C0 => {
    //   block [0x826690C0..0x82669128)
	// 826690C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826690C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826690C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826690CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826690D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826690D4: 392AFC14  addi r9, r10, -0x3ec
	ctx.r[9].s64 = ctx.r[10].s64 + -1004;
	// 826690D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826690DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826690E0: 38AAE800  addi r5, r10, -0x1800
	ctx.r[5].s64 = ctx.r[10].s64 + -6144;
	// 826690E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826690E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826690EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826690F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826690F4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826690F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826690FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669100: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669108: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266910C: 386AE890  addi r3, r10, -0x1770
	ctx.r[3].s64 = ctx.r[10].s64 + -6000;
	// 82669110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669114: 4BDFDD0D  bl 0x82466e20
	ctx.lr = 0x82669118;
	sub_82466E20(ctx, base);
	// 82669118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669128 size=108
    let mut pc: u32 = 0x82669128;
    'dispatch: loop {
        match pc {
            0x82669128 => {
    //   block [0x82669128..0x82669194)
	// 82669128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669134: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266913C: 38EB968C  addi r7, r11, -0x6974
	ctx.r[7].s64 = ctx.r[11].s64 + -26996;
	// 82669140: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82669144: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82669148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266914C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669158: 386AE8C0  addi r3, r10, -0x1740
	ctx.r[3].s64 = ctx.r[10].s64 + -5952;
	// 8266915C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266917C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669180: 4BDFDCA1  bl 0x82466e20
	ctx.lr = 0x82669184;
	sub_82466E20(ctx, base);
	// 82669184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669198 size=112
    let mut pc: u32 = 0x82669198;
    'dispatch: loop {
        match pc {
            0x82669198 => {
    //   block [0x82669198..0x82669208)
	// 82669198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826691A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826691A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826691A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826691AC: 38AAE890  addi r5, r10, -0x1770
	ctx.r[5].s64 = ctx.r[10].s64 + -6000;
	// 826691B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826691B4: 390B96C0  addi r8, r11, -0x6940
	ctx.r[8].s64 = ctx.r[11].s64 + -26944;
	// 826691B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826691BC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826691C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826691C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826691C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826691CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826691D0: 386AE8F0  addi r3, r10, -0x1710
	ctx.r[3].s64 = ctx.r[10].s64 + -5904;
	// 826691D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826691D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826691DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826691E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826691E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826691E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826691EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826691F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826691F4: 4BDFDC2D  bl 0x82466e20
	ctx.lr = 0x826691F8;
	sub_82466E20(ctx, base);
	// 826691F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826691FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82669208 size=24
    let mut pc: u32 = 0x82669208;
    'dispatch: loop {
        match pc {
            0x82669208 => {
    //   block [0x82669208..0x82669220)
	// 82669208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266920C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669210: 394ACA28  addi r10, r10, -0x35d8
	ctx.r[10].s64 = ctx.r[10].s64 + -13784;
	// 82669214: 816B96BC  lwz r11, -0x6944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26948 as u32) ) } as u64;
	// 82669218: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266921C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669220 size=116
    let mut pc: u32 = 0x82669220;
    'dispatch: loop {
        match pc {
            0x82669220 => {
    //   block [0x82669220..0x82669294)
	// 82669220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266922C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669230: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82669234: 390BCA28  addi r8, r11, -0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + -13784;
	// 82669238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266923C: 392AFC78  addi r9, r10, -0x388
	ctx.r[9].s64 = ctx.r[10].s64 + -904;
	// 82669240: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669244: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82669248: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266924C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669254: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266925C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669264: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82669268: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8266926C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82669270: 386BE920  addi r3, r11, -0x16e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5856;
	// 82669274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669278: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266927C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669280: 4BDFDBA1  bl 0x82466e20
	ctx.lr = 0x82669284;
	sub_82466E20(ctx, base);
	// 82669284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669298 size=100
    let mut pc: u32 = 0x82669298;
    'dispatch: loop {
        match pc {
            0x82669298 => {
    //   block [0x82669298..0x826692FC)
	// 82669298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266929C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826692A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826692A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826692A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826692AC: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 826692B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826692B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826692B8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826692BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826692C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826692C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826692C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826692CC: 386AE950  addi r3, r10, -0x16b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5808;
	// 826692D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826692D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826692D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826692DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826692E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826692E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826692E8: 4BDFDB39  bl 0x82466e20
	ctx.lr = 0x826692EC;
	sub_82466E20(ctx, base);
	// 826692EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826692F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826692F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826692F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669300 size=100
    let mut pc: u32 = 0x82669300;
    'dispatch: loop {
        match pc {
            0x82669300 => {
    //   block [0x82669300..0x82669364)
	// 82669300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266930C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669314: 38AAE9B0  addi r5, r10, -0x1650
	ctx.r[5].s64 = ctx.r[10].s64 + -5712;
	// 82669318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266931C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669320: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82669324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266932C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669334: 386AE980  addi r3, r10, -0x1680
	ctx.r[3].s64 = ctx.r[10].s64 + -5760;
	// 82669338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266933C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669340: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266934C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669350: 4BDFDAD1  bl 0x82466e20
	ctx.lr = 0x82669354;
	sub_82466E20(ctx, base);
	// 82669354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266935C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669368 size=112
    let mut pc: u32 = 0x82669368;
    'dispatch: loop {
        match pc {
            0x82669368 => {
    //   block [0x82669368..0x826693D8)
	// 82669368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669378: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266937C: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 82669380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669384: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 82669388: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266938C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82669390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266939C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826693A0: 386AE9B0  addi r3, r10, -0x1650
	ctx.r[3].s64 = ctx.r[10].s64 + -5712;
	// 826693A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826693A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826693AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826693B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826693B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826693B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826693BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826693C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826693C4: 4BDFDA5D  bl 0x82466e20
	ctx.lr = 0x826693C8;
	sub_82466E20(ctx, base);
	// 826693C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826693CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826693D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826693D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826693D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826693D8 size=100
    let mut pc: u32 = 0x826693D8;
    'dispatch: loop {
        match pc {
            0x826693D8 => {
    //   block [0x826693D8..0x8266943C)
	// 826693D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826693DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826693E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826693E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826693E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826693EC: 38AAE9B0  addi r5, r10, -0x1650
	ctx.r[5].s64 = ctx.r[10].s64 + -5712;
	// 826693F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826693F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826693F8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826693FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266940C: 386AE9E0  addi r3, r10, -0x1620
	ctx.r[3].s64 = ctx.r[10].s64 + -5664;
	// 82669410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82669424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669428: 4BDFD9F9  bl 0x82466e20
	ctx.lr = 0x8266942C;
	sub_82466E20(ctx, base);
	// 8266942C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669440 size=100
    let mut pc: u32 = 0x82669440;
    'dispatch: loop {
        match pc {
            0x82669440 => {
    //   block [0x82669440..0x826694A4)
	// 82669440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266944C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669454: 38AAE920  addi r5, r10, -0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5856;
	// 82669458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266945C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669460: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82669464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669474: 386AEA10  addi r3, r10, -0x15f0
	ctx.r[3].s64 = ctx.r[10].s64 + -5616;
	// 82669478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266947C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266948C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669490: 4BDFD991  bl 0x82466e20
	ctx.lr = 0x82669494;
	sub_82466E20(ctx, base);
	// 82669494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266949C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826694A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826694A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826694A8 size=100
    let mut pc: u32 = 0x826694A8;
    'dispatch: loop {
        match pc {
            0x826694A8 => {
    //   block [0x826694A8..0x8266950C)
	// 826694A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826694AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826694B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826694B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826694B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826694BC: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 826694C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826694C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826694C8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826694CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826694D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826694D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826694D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826694DC: 386AEA40  addi r3, r10, -0x15c0
	ctx.r[3].s64 = ctx.r[10].s64 + -5568;
	// 826694E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826694E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826694E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826694EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826694F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826694F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826694F8: 4BDFD929  bl 0x82466e20
	ctx.lr = 0x826694FC;
	sub_82466E20(ctx, base);
	// 826694FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669510 size=100
    let mut pc: u32 = 0x82669510;
    'dispatch: loop {
        match pc {
            0x82669510 => {
    //   block [0x82669510..0x82669574)
	// 82669510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266951C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669524: 38AAEA10  addi r5, r10, -0x15f0
	ctx.r[5].s64 = ctx.r[10].s64 + -5616;
	// 82669528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669530: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82669534: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266953C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669544: 386AEA70  addi r3, r10, -0x1590
	ctx.r[3].s64 = ctx.r[10].s64 + -5520;
	// 82669548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266954C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82669554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266955C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669560: 4BDFD8C1  bl 0x82466e20
	ctx.lr = 0x82669564;
	sub_82466E20(ctx, base);
	// 82669564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669578 size=100
    let mut pc: u32 = 0x82669578;
    'dispatch: loop {
        match pc {
            0x82669578 => {
    //   block [0x82669578..0x826695DC)
	// 82669578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266958C: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 82669590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669598: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8266959C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826695A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826695A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826695A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826695AC: 386AEAA0  addi r3, r10, -0x1560
	ctx.r[3].s64 = ctx.r[10].s64 + -5472;
	// 826695B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826695B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826695B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826695BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826695C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826695C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826695C8: 4BDFD859  bl 0x82466e20
	ctx.lr = 0x826695CC;
	sub_82466E20(ctx, base);
	// 826695CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826695D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826695D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826695D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826695E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826695E0 size=112
    let mut pc: u32 = 0x826695E0;
    'dispatch: loop {
        match pc {
            0x826695E0 => {
    //   block [0x826695E0..0x82669650)
	// 826695E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826695E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826695E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826695EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826695F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826695F4: 38AAEB30  addi r5, r10, -0x14d0
	ctx.r[5].s64 = ctx.r[10].s64 + -5328;
	// 826695F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826695FC: 390B9798  addi r8, r11, -0x6868
	ctx.r[8].s64 = ctx.r[11].s64 + -26728;
	// 82669600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82669604: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82669608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266960C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669618: 386AEAD0  addi r3, r10, -0x1530
	ctx.r[3].s64 = ctx.r[10].s64 + -5424;
	// 8266961C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266962C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266963C: 4BDFD7E5  bl 0x82466e20
	ctx.lr = 0x82669640;
	sub_82466E20(ctx, base);
	// 82669640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669650 size=112
    let mut pc: u32 = 0x82669650;
    'dispatch: loop {
        match pc {
            0x82669650 => {
    //   block [0x82669650..0x826696C0)
	// 82669650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266965C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669660: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669664: 38AAEB60  addi r5, r10, -0x14a0
	ctx.r[5].s64 = ctx.r[10].s64 + -5280;
	// 82669668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266966C: 390B97C8  addi r8, r11, -0x6838
	ctx.r[8].s64 = ctx.r[11].s64 + -26680;
	// 82669670: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669674: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82669678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266967C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669688: 386AEB00  addi r3, r10, -0x1500
	ctx.r[3].s64 = ctx.r[10].s64 + -5376;
	// 8266968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826696A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826696A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826696A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826696AC: 4BDFD775  bl 0x82466e20
	ctx.lr = 0x826696B0;
	sub_82466E20(ctx, base);
	// 826696B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826696B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826696B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826696BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826696C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826696C0 size=112
    let mut pc: u32 = 0x826696C0;
    'dispatch: loop {
        match pc {
            0x826696C0 => {
    //   block [0x826696C0..0x82669730)
	// 826696C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826696C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826696C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826696CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826696D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826696D4: 38AAEC20  addi r5, r10, -0x13e0
	ctx.r[5].s64 = ctx.r[10].s64 + -5088;
	// 826696D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826696DC: 390B97E0  addi r8, r11, -0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + -26656;
	// 826696E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826696E4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826696E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826696EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826696F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826696F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826696F8: 386AEB30  addi r3, r10, -0x14d0
	ctx.r[3].s64 = ctx.r[10].s64 + -5328;
	// 826696FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266970C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266971C: 4BDFD705  bl 0x82466e20
	ctx.lr = 0x82669720;
	sub_82466E20(ctx, base);
	// 82669720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669730 size=112
    let mut pc: u32 = 0x82669730;
    'dispatch: loop {
        match pc {
            0x82669730 => {
    //   block [0x82669730..0x826697A0)
	// 82669730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266973C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669740: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669744: 38AAEB30  addi r5, r10, -0x14d0
	ctx.r[5].s64 = ctx.r[10].s64 + -5328;
	// 82669748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266974C: 390B9810  addi r8, r11, -0x67f0
	ctx.r[8].s64 = ctx.r[11].s64 + -26608;
	// 82669750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669754: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82669758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266975C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669768: 386AEB60  addi r3, r10, -0x14a0
	ctx.r[3].s64 = ctx.r[10].s64 + -5280;
	// 8266976C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266977C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266978C: 4BDFD695  bl 0x82466e20
	ctx.lr = 0x82669790;
	sub_82466E20(ctx, base);
	// 82669790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266979C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826697A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826697A0 size=112
    let mut pc: u32 = 0x826697A0;
    'dispatch: loop {
        match pc {
            0x826697A0 => {
    //   block [0x826697A0..0x82669810)
	// 826697A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826697A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826697A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826697AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826697B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826697B4: 38AAEB60  addi r5, r10, -0x14a0
	ctx.r[5].s64 = ctx.r[10].s64 + -5280;
	// 826697B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826697BC: 390B9828  addi r8, r11, -0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + -26584;
	// 826697C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826697C4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826697C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826697CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826697D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826697D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826697D8: 386AEB90  addi r3, r10, -0x1470
	ctx.r[3].s64 = ctx.r[10].s64 + -5232;
	// 826697DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826697E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826697E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826697E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826697EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826697F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826697F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826697F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826697FC: 4BDFD625  bl 0x82466e20
	ctx.lr = 0x82669800;
	sub_82466E20(ctx, base);
	// 82669800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266980C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669810 size=116
    let mut pc: u32 = 0x82669810;
    'dispatch: loop {
        match pc {
            0x82669810 => {
    //   block [0x82669810..0x82669884)
	// 82669810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266981C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669820: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82669824: 390A9840  addi r8, r10, -0x67c0
	ctx.r[8].s64 = ctx.r[10].s64 + -26560;
	// 82669828: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266982C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82669830: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669834: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669838: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669844: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82669848: 396BFC8C  addi r11, r11, -0x374
	ctx.r[11].s64 = ctx.r[11].s64 + -884;
	// 8266984C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669854: 386AEBC0  addi r3, r10, -0x1440
	ctx.r[3].s64 = ctx.r[10].s64 + -5184;
	// 82669858: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266985C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669860: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82669864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266986C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669870: 4BDFD5B1  bl 0x82466e20
	ctx.lr = 0x82669874;
	sub_82466E20(ctx, base);
	// 82669874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82669888 size=48
    let mut pc: u32 = 0x82669888;
    'dispatch: loop {
        match pc {
            0x82669888 => {
    //   block [0x82669888..0x826698B8)
	// 82669888: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266988C: 814B98F4  lwz r10, -0x670c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26380 as u32) ) } as u64;
	// 82669890: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669894: 396BCAE8  addi r11, r11, -0x3518
	ctx.r[11].s64 = ctx.r[11].s64 + -13592;
	// 82669898: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8266989C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826698A0: 814A98F0  lwz r10, -0x6710(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26384 as u32) ) } as u64;
	// 826698A4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826698A8: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826698AC: 814A98EC  lwz r10, -0x6714(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26388 as u32) ) } as u64;
	// 826698B0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 826698B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826698B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826698B8 size=116
    let mut pc: u32 = 0x826698B8;
    'dispatch: loop {
        match pc {
            0x826698B8 => {
    //   block [0x826698B8..0x8266992C)
	// 826698B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826698BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826698C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826698C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826698C8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826698CC: 392BFD60  addi r9, r11, -0x2a0
	ctx.r[9].s64 = ctx.r[11].s64 + -672;
	// 826698D0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826698D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826698D8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826698DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 826698E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826698E4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826698E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826698EC: 396BCAE8  addi r11, r11, -0x3518
	ctx.r[11].s64 = ctx.r[11].s64 + -13592;
	// 826698F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826698F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826698F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826698FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669900: 386AEBF0  addi r3, r10, -0x1410
	ctx.r[3].s64 = ctx.r[10].s64 + -5136;
	// 82669904: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82669908: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266990C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669910: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82669914: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82669918: 4BDFD509  bl 0x82466e20
	ctx.lr = 0x8266991C;
	sub_82466E20(ctx, base);
	// 8266991C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669930 size=116
    let mut pc: u32 = 0x82669930;
    'dispatch: loop {
        match pc {
            0x82669930 => {
    //   block [0x82669930..0x826699A4)
	// 82669930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266993C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669940: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82669944: 390B9900  addi r8, r11, -0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + -26368;
	// 82669948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266994C: 392AFF00  addi r9, r10, -0x100
	ctx.r[9].s64 = ctx.r[10].s64 + -256;
	// 82669950: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669954: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82669958: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266995C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266996C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669974: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82669978: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8266997C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82669980: 386BEC20  addi r3, r11, -0x13e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5088;
	// 82669984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82669988: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669990: 4BDFD491  bl 0x82466e20
	ctx.lr = 0x82669994;
	sub_82466E20(ctx, base);
	// 82669994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266999C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826699A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826699A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826699A8 size=112
    let mut pc: u32 = 0x826699A8;
    'dispatch: loop {
        match pc {
            0x826699A8 => {
    //   block [0x826699A8..0x82669A18)
	// 826699A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826699AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826699B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826699B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826699B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826699BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826699C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826699C4: 390B9990  addi r8, r11, -0x6670
	ctx.r[8].s64 = ctx.r[11].s64 + -26224;
	// 826699C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826699CC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826699D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826699D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826699D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826699DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826699E0: 386AEC50  addi r3, r10, -0x13b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5040;
	// 826699E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826699E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826699EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826699F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826699F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826699F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826699FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669A04: 4BDFD41D  bl 0x82466e20
	ctx.lr = 0x82669A08;
	sub_82466E20(ctx, base);
	// 82669A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669A18 size=108
    let mut pc: u32 = 0x82669A18;
    'dispatch: loop {
        match pc {
            0x82669A18 => {
    //   block [0x82669A18..0x82669A84)
	// 82669A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669A24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82669A2C: 38EB99A8  addi r7, r11, -0x6658
	ctx.r[7].s64 = ctx.r[11].s64 + -26200;
	// 82669A30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82669A34: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 82669A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669A3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669A48: 386AEC80  addi r3, r10, -0x1380
	ctx.r[3].s64 = ctx.r[10].s64 + -4992;
	// 82669A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669A70: 4BDFD3B1  bl 0x82466e20
	ctx.lr = 0x82669A74;
	sub_82466E20(ctx, base);
	// 82669A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669A88 size=112
    let mut pc: u32 = 0x82669A88;
    'dispatch: loop {
        match pc {
            0x82669A88 => {
    //   block [0x82669A88..0x82669AF8)
	// 82669A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669A98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669A9C: 38AACB20  addi r5, r10, -0x34e0
	ctx.r[5].s64 = ctx.r[10].s64 + -13536;
	// 82669AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669AA4: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 82669AA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669AAC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82669AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669AB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669AC0: 386AECB0  addi r3, r10, -0x1350
	ctx.r[3].s64 = ctx.r[10].s64 + -4944;
	// 82669AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669AE4: 4BDFD33D  bl 0x82466e20
	ctx.lr = 0x82669AE8;
	sub_82466E20(ctx, base);
	// 82669AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669AF8 size=108
    let mut pc: u32 = 0x82669AF8;
    'dispatch: loop {
        match pc {
            0x82669AF8 => {
    //   block [0x82669AF8..0x82669B64)
	// 82669AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669B04: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669B0C: 38EB9A38  addi r7, r11, -0x65c8
	ctx.r[7].s64 = ctx.r[11].s64 + -26056;
	// 82669B10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82669B14: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82669B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669B1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669B28: 386AECE0  addi r3, r10, -0x1320
	ctx.r[3].s64 = ctx.r[10].s64 + -4896;
	// 82669B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669B50: 4BDFD2D1  bl 0x82466e20
	ctx.lr = 0x82669B54;
	sub_82466E20(ctx, base);
	// 82669B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669B68 size=112
    let mut pc: u32 = 0x82669B68;
    'dispatch: loop {
        match pc {
            0x82669B68 => {
    //   block [0x82669B68..0x82669BD8)
	// 82669B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669B74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B78: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669B7C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669B84: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 82669B88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82669B8C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82669B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669B94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669BA0: 386AED10  addi r3, r10, -0x12f0
	ctx.r[3].s64 = ctx.r[10].s64 + -4848;
	// 82669BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669BC4: 4BDFD25D  bl 0x82466e20
	ctx.lr = 0x82669BC8;
	sub_82466E20(ctx, base);
	// 82669BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669BD8 size=108
    let mut pc: u32 = 0x82669BD8;
    'dispatch: loop {
        match pc {
            0x82669BD8 => {
    //   block [0x82669BD8..0x82669C44)
	// 82669BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669BE4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669BEC: 38EB9A98  addi r7, r11, -0x6568
	ctx.r[7].s64 = ctx.r[11].s64 + -25960;
	// 82669BF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82669BF4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 82669BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669BFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669C08: 386AED40  addi r3, r10, -0x12c0
	ctx.r[3].s64 = ctx.r[10].s64 + -4800;
	// 82669C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669C30: 4BDFD1F1  bl 0x82466e20
	ctx.lr = 0x82669C34;
	sub_82466E20(ctx, base);
	// 82669C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669C48 size=108
    let mut pc: u32 = 0x82669C48;
    'dispatch: loop {
        match pc {
            0x82669C48 => {
    //   block [0x82669C48..0x82669CB4)
	// 82669C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669C54: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669C5C: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 82669C60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82669C64: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82669C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669C78: 386AED70  addi r3, r10, -0x1290
	ctx.r[3].s64 = ctx.r[10].s64 + -4752;
	// 82669C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669CA0: 4BDFD181  bl 0x82466e20
	ctx.lr = 0x82669CA4;
	sub_82466E20(ctx, base);
	// 82669CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669CB8 size=112
    let mut pc: u32 = 0x82669CB8;
    'dispatch: loop {
        match pc {
            0x82669CB8 => {
    //   block [0x82669CB8..0x82669D28)
	// 82669CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669CC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669CC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669CCC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82669CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669CD4: 390B9AE0  addi r8, r11, -0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + -25888;
	// 82669CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82669CDC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82669CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669CE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669CF0: 386AEDA0  addi r3, r10, -0x1260
	ctx.r[3].s64 = ctx.r[10].s64 + -4704;
	// 82669CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669D14: 4BDFD10D  bl 0x82466e20
	ctx.lr = 0x82669D18;
	sub_82466E20(ctx, base);
	// 82669D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669D28 size=112
    let mut pc: u32 = 0x82669D28;
    'dispatch: loop {
        match pc {
            0x82669D28 => {
    //   block [0x82669D28..0x82669D98)
	// 82669D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669D38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669D3C: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669D44: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 82669D48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82669D4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82669D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669D60: 386AEDD0  addi r3, r10, -0x1230
	ctx.r[3].s64 = ctx.r[10].s64 + -4656;
	// 82669D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669D84: 4BDFD09D  bl 0x82466e20
	ctx.lr = 0x82669D88;
	sub_82466E20(ctx, base);
	// 82669D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669D98 size=112
    let mut pc: u32 = 0x82669D98;
    'dispatch: loop {
        match pc {
            0x82669D98 => {
    //   block [0x82669D98..0x82669E08)
	// 82669D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669DA8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669DAC: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669DB4: 390B9B58  addi r8, r11, -0x64a8
	ctx.r[8].s64 = ctx.r[11].s64 + -25768;
	// 82669DB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669DBC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82669DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669DC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669DD0: 386AEE00  addi r3, r10, -0x1200
	ctx.r[3].s64 = ctx.r[10].s64 + -4608;
	// 82669DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669DF4: 4BDFD02D  bl 0x82466e20
	ctx.lr = 0x82669DF8;
	sub_82466E20(ctx, base);
	// 82669DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


