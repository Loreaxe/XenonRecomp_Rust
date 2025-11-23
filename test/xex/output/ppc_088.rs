pub fn sub_8263F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F418 size=112
    let mut pc: u32 = 0x8263F418;
    'dispatch: loop {
        match pc {
            0x8263F418 => {
    //   block [0x8263F418..0x8263F488)
	// 8263F418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F428: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F42C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F434: 390B827C  addi r8, r11, -0x7d84
	ctx.r[8].s64 = ctx.r[11].s64 + -32132;
	// 8263F438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F43C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8263F440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F450: 386ACE2C  addi r3, r10, -0x31d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12756;
	// 8263F454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F474: 4BE279AD  bl 0x82466e20
	ctx.lr = 0x8263F478;
	sub_82466E20(ctx, base);
	// 8263F478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F488 size=112
    let mut pc: u32 = 0x8263F488;
    'dispatch: loop {
        match pc {
            0x8263F488 => {
    //   block [0x8263F488..0x8263F4F8)
	// 8263F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F494: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F498: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F49C: 392A6EA4  addi r9, r10, 0x6ea4
	ctx.r[9].s64 = ctx.r[10].s64 + 28324;
	// 8263F4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F4A4: 390B8298  addi r8, r11, -0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + -32104;
	// 8263F4A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263F4AC: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8263F4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F4B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F4C0: 386ACE5C  addi r3, r10, -0x31a4
	ctx.r[3].s64 = ctx.r[10].s64 + -12708;
	// 8263F4C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F4C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F4DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F4E4: 4BE2793D  bl 0x82466e20
	ctx.lr = 0x8263F4E8;
	sub_82466E20(ctx, base);
	// 8263F4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F4F8 size=112
    let mut pc: u32 = 0x8263F4F8;
    'dispatch: loop {
        match pc {
            0x8263F4F8 => {
    //   block [0x8263F4F8..0x8263F568)
	// 8263F4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F508: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F50C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F514: 390B8340  addi r8, r11, -0x7cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -31936;
	// 8263F518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F51C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8263F520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F524: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F530: 386ACE8C  addi r3, r10, -0x3174
	ctx.r[3].s64 = ctx.r[10].s64 + -12660;
	// 8263F534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F554: 4BE278CD  bl 0x82466e20
	ctx.lr = 0x8263F558;
	sub_82466E20(ctx, base);
	// 8263F558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F568 size=108
    let mut pc: u32 = 0x8263F568;
    'dispatch: loop {
        match pc {
            0x8263F568 => {
    //   block [0x8263F568..0x8263F5D4)
	// 8263F568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F574: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F57C: 38EB8358  addi r7, r11, -0x7ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -31912;
	// 8263F580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F584: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8263F588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F58C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F598: 386ACEBC  addi r3, r10, -0x3144
	ctx.r[3].s64 = ctx.r[10].s64 + -12612;
	// 8263F59C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F5A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F5B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F5B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F5BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F5C0: 4BE27861  bl 0x82466e20
	ctx.lr = 0x8263F5C4;
	sub_82466E20(ctx, base);
	// 8263F5C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F5C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F5CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F5D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F5D8 size=112
    let mut pc: u32 = 0x8263F5D8;
    'dispatch: loop {
        match pc {
            0x8263F5D8 => {
    //   block [0x8263F5D8..0x8263F648)
	// 8263F5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F5E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F5E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F5EC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F5F4: 390B8388  addi r8, r11, -0x7c78
	ctx.r[8].s64 = ctx.r[11].s64 + -31864;
	// 8263F5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F5FC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8263F600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F604: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F610: 386ACEEC  addi r3, r10, -0x3114
	ctx.r[3].s64 = ctx.r[10].s64 + -12564;
	// 8263F614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F634: 4BE277ED  bl 0x82466e20
	ctx.lr = 0x8263F638;
	sub_82466E20(ctx, base);
	// 8263F638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F648 size=112
    let mut pc: u32 = 0x8263F648;
    'dispatch: loop {
        match pc {
            0x8263F648 => {
    //   block [0x8263F648..0x8263F6B8)
	// 8263F648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F658: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F65C: 392A6ED8  addi r9, r10, 0x6ed8
	ctx.r[9].s64 = ctx.r[10].s64 + 28376;
	// 8263F660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F664: 390B83A8  addi r8, r11, -0x7c58
	ctx.r[8].s64 = ctx.r[11].s64 + -31832;
	// 8263F668: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263F66C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8263F670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F680: 386ACF1C  addi r3, r10, -0x30e4
	ctx.r[3].s64 = ctx.r[10].s64 + -12516;
	// 8263F684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F6A4: 4BE2777D  bl 0x82466e20
	ctx.lr = 0x8263F6A8;
	sub_82466E20(ctx, base);
	// 8263F6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F6B8 size=112
    let mut pc: u32 = 0x8263F6B8;
    'dispatch: loop {
        match pc {
            0x8263F6B8 => {
    //   block [0x8263F6B8..0x8263F728)
	// 8263F6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F6C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F6C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F6CC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F6D4: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8263F6D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263F6DC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8263F6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F6E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F6E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F6F0: 386ACF4C  addi r3, r10, -0x30b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12468;
	// 8263F6F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F714: 4BE2770D  bl 0x82466e20
	ctx.lr = 0x8263F718;
	sub_82466E20(ctx, base);
	// 8263F718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F728 size=112
    let mut pc: u32 = 0x8263F728;
    'dispatch: loop {
        match pc {
            0x8263F728 => {
    //   block [0x8263F728..0x8263F798)
	// 8263F728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F738: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F73C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F744: 390B8498  addi r8, r11, -0x7b68
	ctx.r[8].s64 = ctx.r[11].s64 + -31592;
	// 8263F748: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263F74C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8263F750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F754: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F760: 386ACF7C  addi r3, r10, -0x3084
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	// 8263F764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F784: 4BE2769D  bl 0x82466e20
	ctx.lr = 0x8263F788;
	sub_82466E20(ctx, base);
	// 8263F788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F798 size=100
    let mut pc: u32 = 0x8263F798;
    'dispatch: loop {
        match pc {
            0x8263F798 => {
    //   block [0x8263F798..0x8263F7FC)
	// 8263F798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F7A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F7AC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F7B8: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8263F7BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F7CC: 386ACFAC  addi r3, r10, -0x3054
	ctx.r[3].s64 = ctx.r[10].s64 + -12372;
	// 8263F7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F7D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263F7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F7E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263F7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F7E8: 4BE27639  bl 0x82466e20
	ctx.lr = 0x8263F7EC;
	sub_82466E20(ctx, base);
	// 8263F7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F800 size=112
    let mut pc: u32 = 0x8263F800;
    'dispatch: loop {
        match pc {
            0x8263F800 => {
    //   block [0x8263F800..0x8263F870)
	// 8263F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F80C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F810: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F814: 38AACC1C  addi r5, r10, -0x33e4
	ctx.r[5].s64 = ctx.r[10].s64 + -13284;
	// 8263F818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F81C: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 8263F820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263F824: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8263F828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F82C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F838: 386ACFDC  addi r3, r10, -0x3024
	ctx.r[3].s64 = ctx.r[10].s64 + -12324;
	// 8263F83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F85C: 4BE275C5  bl 0x82466e20
	ctx.lr = 0x8263F860;
	sub_82466E20(ctx, base);
	// 8263F860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F870 size=112
    let mut pc: u32 = 0x8263F870;
    'dispatch: loop {
        match pc {
            0x8263F870 => {
    //   block [0x8263F870..0x8263F8E0)
	// 8263F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F87C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F880: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F884: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263F888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F88C: 390B8588  addi r8, r11, -0x7a78
	ctx.r[8].s64 = ctx.r[11].s64 + -31352;
	// 8263F890: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F894: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8263F898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F89C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F8A8: 386AD00C  addi r3, r10, -0x2ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -12276;
	// 8263F8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F8CC: 4BE27555  bl 0x82466e20
	ctx.lr = 0x8263F8D0;
	sub_82466E20(ctx, base);
	// 8263F8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F8E0 size=108
    let mut pc: u32 = 0x8263F8E0;
    'dispatch: loop {
        match pc {
            0x8263F8E0 => {
    //   block [0x8263F8E0..0x8263F94C)
	// 8263F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F8EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F8F4: 38EB85A0  addi r7, r11, -0x7a60
	ctx.r[7].s64 = ctx.r[11].s64 + -31328;
	// 8263F8F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F8FC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8263F900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F910: 386AD03C  addi r3, r10, -0x2fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -12228;
	// 8263F914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F938: 4BE274E9  bl 0x82466e20
	ctx.lr = 0x8263F93C;
	sub_82466E20(ctx, base);
	// 8263F93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F950 size=112
    let mut pc: u32 = 0x8263F950;
    'dispatch: loop {
        match pc {
            0x8263F950 => {
    //   block [0x8263F950..0x8263F9C0)
	// 8263F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F95C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F960: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F964: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263F968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F96C: 390B85D0  addi r8, r11, -0x7a30
	ctx.r[8].s64 = ctx.r[11].s64 + -31280;
	// 8263F970: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263F974: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8263F978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F97C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F988: 386AD06C  addi r3, r10, -0x2f94
	ctx.r[3].s64 = ctx.r[10].s64 + -12180;
	// 8263F98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F9AC: 4BE27475  bl 0x82466e20
	ctx.lr = 0x8263F9B0;
	sub_82466E20(ctx, base);
	// 8263F9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F9C0 size=112
    let mut pc: u32 = 0x8263F9C0;
    'dispatch: loop {
        match pc {
            0x8263F9C0 => {
    //   block [0x8263F9C0..0x8263FA30)
	// 8263F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F9CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F9D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F9D4: 392A6F04  addi r9, r10, 0x6f04
	ctx.r[9].s64 = ctx.r[10].s64 + 28420;
	// 8263F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F9DC: 390B8660  addi r8, r11, -0x79a0
	ctx.r[8].s64 = ctx.r[11].s64 + -31136;
	// 8263F9E0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263F9E4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8263F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F9EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F9F8: 386AD09C  addi r3, r10, -0x2f64
	ctx.r[3].s64 = ctx.r[10].s64 + -12132;
	// 8263F9FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FA00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FA14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FA1C: 4BE27405  bl 0x82466e20
	ctx.lr = 0x8263FA20;
	sub_82466E20(ctx, base);
	// 8263FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FA30 size=112
    let mut pc: u32 = 0x8263FA30;
    'dispatch: loop {
        match pc {
            0x8263FA30 => {
    //   block [0x8263FA30..0x8263FAA0)
	// 8263FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FA3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FA40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FA44: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FA4C: 390B86A8  addi r8, r11, -0x7958
	ctx.r[8].s64 = ctx.r[11].s64 + -31064;
	// 8263FA50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263FA54: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8263FA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FA5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FA60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FA68: 386AD0CC  addi r3, r10, -0x2f34
	ctx.r[3].s64 = ctx.r[10].s64 + -12084;
	// 8263FA6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FA8C: 4BE27395  bl 0x82466e20
	ctx.lr = 0x8263FA90;
	sub_82466E20(ctx, base);
	// 8263FA90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FAA0 size=108
    let mut pc: u32 = 0x8263FAA0;
    'dispatch: loop {
        match pc {
            0x8263FAA0 => {
    //   block [0x8263FAA0..0x8263FB0C)
	// 8263FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FAAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FAB4: 38EB86C0  addi r7, r11, -0x7940
	ctx.r[7].s64 = ctx.r[11].s64 + -31040;
	// 8263FAB8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263FABC: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8263FAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FAC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FAC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FAD0: 386AD0FC  addi r3, r10, -0x2f04
	ctx.r[3].s64 = ctx.r[10].s64 + -12036;
	// 8263FAD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FAF8: 4BE27329  bl 0x82466e20
	ctx.lr = 0x8263FAFC;
	sub_82466E20(ctx, base);
	// 8263FAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FB10 size=116
    let mut pc: u32 = 0x8263FB10;
    'dispatch: loop {
        match pc {
            0x8263FB10 => {
    //   block [0x8263FB10..0x8263FB84)
	// 8263FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FB1C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FB20: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263FB24: 390A8750  addi r8, r10, -0x78b0
	ctx.r[8].s64 = ctx.r[10].s64 + -30896;
	// 8263FB28: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FB2C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263FB30: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263FB34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FB38: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FB44: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8263FB48: 396B6F18  addi r11, r11, 0x6f18
	ctx.r[11].s64 = ctx.r[11].s64 + 28440;
	// 8263FB4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FB50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FB54: 386AD12C  addi r3, r10, -0x2ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -11988;
	// 8263FB58: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263FB5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FB60: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263FB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FB70: 4BE272B1  bl 0x82466e20
	ctx.lr = 0x8263FB74;
	sub_82466E20(ctx, base);
	// 8263FB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FB88 size=108
    let mut pc: u32 = 0x8263FB88;
    'dispatch: loop {
        match pc {
            0x8263FB88 => {
    //   block [0x8263FB88..0x8263FBF4)
	// 8263FB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FB94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FB98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FB9C: 38EB8828  addi r7, r11, -0x77d8
	ctx.r[7].s64 = ctx.r[11].s64 + -30680;
	// 8263FBA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263FBA4: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8263FBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FBAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FBB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FBB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FBB8: 386AD15C  addi r3, r10, -0x2ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -11940;
	// 8263FBBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FBC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FBC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FBD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FBD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FBDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FBE0: 4BE27241  bl 0x82466e20
	ctx.lr = 0x8263FBE4;
	sub_82466E20(ctx, base);
	// 8263FBE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FBF8 size=112
    let mut pc: u32 = 0x8263FBF8;
    'dispatch: loop {
        match pc {
            0x8263FBF8 => {
    //   block [0x8263FBF8..0x8263FC68)
	// 8263FBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FC04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FC0C: 38AACFAC  addi r5, r10, -0x3054
	ctx.r[5].s64 = ctx.r[10].s64 + -12372;
	// 8263FC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FC14: 390B8870  addi r8, r11, -0x7790
	ctx.r[8].s64 = ctx.r[11].s64 + -30608;
	// 8263FC18: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263FC1C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8263FC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FC24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FC30: 386AD18C  addi r3, r10, -0x2e74
	ctx.r[3].s64 = ctx.r[10].s64 + -11892;
	// 8263FC34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FC54: 4BE271CD  bl 0x82466e20
	ctx.lr = 0x8263FC58;
	sub_82466E20(ctx, base);
	// 8263FC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FC68 size=112
    let mut pc: u32 = 0x8263FC68;
    'dispatch: loop {
        match pc {
            0x8263FC68 => {
    //   block [0x8263FC68..0x8263FCD8)
	// 8263FC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FC74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FC7C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FC84: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 8263FC88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263FC8C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8263FC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FC94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FCA0: 386AD1BC  addi r3, r10, -0x2e44
	ctx.r[3].s64 = ctx.r[10].s64 + -11844;
	// 8263FCA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FCC4: 4BE2715D  bl 0x82466e20
	ctx.lr = 0x8263FCC8;
	sub_82466E20(ctx, base);
	// 8263FCC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FCD8 size=108
    let mut pc: u32 = 0x8263FCD8;
    'dispatch: loop {
        match pc {
            0x8263FCD8 => {
    //   block [0x8263FCD8..0x8263FD44)
	// 8263FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FCE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FCE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FCE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FCEC: 38EB8918  addi r7, r11, -0x76e8
	ctx.r[7].s64 = ctx.r[11].s64 + -30440;
	// 8263FCF0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263FCF4: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8263FCF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FCFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FD00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FD08: 386AD1EC  addi r3, r10, -0x2e14
	ctx.r[3].s64 = ctx.r[10].s64 + -11796;
	// 8263FD0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FD30: 4BE270F1  bl 0x82466e20
	ctx.lr = 0x8263FD34;
	sub_82466E20(ctx, base);
	// 8263FD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FD48 size=108
    let mut pc: u32 = 0x8263FD48;
    'dispatch: loop {
        match pc {
            0x8263FD48 => {
    //   block [0x8263FD48..0x8263FDB4)
	// 8263FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FD54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FD58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FD5C: 38EB8978  addi r7, r11, -0x7688
	ctx.r[7].s64 = ctx.r[11].s64 + -30344;
	// 8263FD60: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263FD64: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8263FD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FD6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FD70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263FD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FD78: 386AD21C  addi r3, r10, -0x2de4
	ctx.r[3].s64 = ctx.r[10].s64 + -11748;
	// 8263FD7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263FD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FD9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263FDA0: 4BE27081  bl 0x82466e20
	ctx.lr = 0x8263FDA4;
	sub_82466E20(ctx, base);
	// 8263FDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FDB8 size=112
    let mut pc: u32 = 0x8263FDB8;
    'dispatch: loop {
        match pc {
            0x8263FDB8 => {
    //   block [0x8263FDB8..0x8263FE28)
	// 8263FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FDC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FDC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FDCC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263FDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FDD4: 390B89F0  addi r8, r11, -0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + -30224;
	// 8263FDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263FDDC: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8263FDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FDE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FDF0: 386AD24C  addi r3, r10, -0x2db4
	ctx.r[3].s64 = ctx.r[10].s64 + -11700;
	// 8263FDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FE14: 4BE2700D  bl 0x82466e20
	ctx.lr = 0x8263FE18;
	sub_82466E20(ctx, base);
	// 8263FE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263FE28 size=24
    let mut pc: u32 = 0x8263FE28;
    'dispatch: loop {
        match pc {
            0x8263FE28 => {
    //   block [0x8263FE28..0x8263FE40)
	// 8263FE28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FE2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FE30: 394ABEF0  addi r10, r10, -0x4110
	ctx.r[10].s64 = ctx.r[10].s64 + -16656;
	// 8263FE34: 816B8A38  lwz r11, -0x75c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30152 as u32) ) } as u64;
	// 8263FE38: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FE40 size=116
    let mut pc: u32 = 0x8263FE40;
    'dispatch: loop {
        match pc {
            0x8263FE40 => {
    //   block [0x8263FE40..0x8263FEB4)
	// 8263FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FE4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FE50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263FE54: 390BBEF0  addi r8, r11, -0x4110
	ctx.r[8].s64 = ctx.r[11].s64 + -16656;
	// 8263FE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FE5C: 392A6F7C  addi r9, r10, 0x6f7c
	ctx.r[9].s64 = ctx.r[10].s64 + 28540;
	// 8263FE60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FE64: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263FE68: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263FE6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FE74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FE84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263FE88: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8263FE8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FE90: 386BD27C  addi r3, r11, -0x2d84
	ctx.r[3].s64 = ctx.r[11].s64 + -11652;
	// 8263FE94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FE98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FEA0: 4BE26F81  bl 0x82466e20
	ctx.lr = 0x8263FEA4;
	sub_82466E20(ctx, base);
	// 8263FEA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FEB8 size=112
    let mut pc: u32 = 0x8263FEB8;
    'dispatch: loop {
        match pc {
            0x8263FEB8 => {
    //   block [0x8263FEB8..0x8263FF28)
	// 8263FEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FEC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FEC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FECC: 38AAD27C  addi r5, r10, -0x2d84
	ctx.r[5].s64 = ctx.r[10].s64 + -11652;
	// 8263FED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FED4: 390B8A3C  addi r8, r11, -0x75c4
	ctx.r[8].s64 = ctx.r[11].s64 + -30148;
	// 8263FED8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263FEDC: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8263FEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FEE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FEE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FEEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FEF0: 386AD2AC  addi r3, r10, -0x2d54
	ctx.r[3].s64 = ctx.r[10].s64 + -11604;
	// 8263FEF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263FF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FF14: 4BE26F0D  bl 0x82466e20
	ctx.lr = 0x8263FF18;
	sub_82466E20(ctx, base);
	// 8263FF18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263FF28 size=24
    let mut pc: u32 = 0x8263FF28;
    'dispatch: loop {
        match pc {
            0x8263FF28 => {
    //   block [0x8263FF28..0x8263FF40)
	// 8263FF28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FF2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263FF30: 394ABF08  addi r10, r10, -0x40f8
	ctx.r[10].s64 = ctx.r[10].s64 + -16632;
	// 8263FF34: 816B8A6C  lwz r11, -0x7594(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30100 as u32) ) } as u64;
	// 8263FF38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263FF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FF40 size=116
    let mut pc: u32 = 0x8263FF40;
    'dispatch: loop {
        match pc {
            0x8263FF40 => {
    //   block [0x8263FF40..0x8263FFB4)
	// 8263FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FF4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263FF54: 390BBF08  addi r8, r11, -0x40f8
	ctx.r[8].s64 = ctx.r[11].s64 + -16632;
	// 8263FF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FF5C: 392A6FB8  addi r9, r10, 0x6fb8
	ctx.r[9].s64 = ctx.r[10].s64 + 28600;
	// 8263FF60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FF64: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263FF68: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 8263FF6C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263FF74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263FF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263FF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263FF84: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263FF88: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8263FF8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263FF90: 386BD2DC  addi r3, r11, -0x2d24
	ctx.r[3].s64 = ctx.r[11].s64 + -11556;
	// 8263FF94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263FF98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263FFA0: 4BE26E81  bl 0x82466e20
	ctx.lr = 0x8263FFA4;
	sub_82466E20(ctx, base);
	// 8263FFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263FFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263FFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263FFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263FFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263FFB8 size=112
    let mut pc: u32 = 0x8263FFB8;
    'dispatch: loop {
        match pc {
            0x8263FFB8 => {
    //   block [0x8263FFB8..0x82640028)
	// 8263FFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263FFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263FFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263FFC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FFC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263FFCC: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 8263FFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263FFD4: 390B8A70  addi r8, r11, -0x7590
	ctx.r[8].s64 = ctx.r[11].s64 + -30096;
	// 8263FFD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263FFDC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8263FFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263FFE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263FFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263FFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263FFF0: 386AD30C  addi r3, r10, -0x2cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -11508;
	// 8263FFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263FFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263FFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264000C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640014: 4BE26E0D  bl 0x82466e20
	ctx.lr = 0x82640018;
	sub_82466E20(ctx, base);
	// 82640018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264001C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640028 size=112
    let mut pc: u32 = 0x82640028;
    'dispatch: loop {
        match pc {
            0x82640028 => {
    //   block [0x82640028..0x82640098)
	// 82640028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264002C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640038: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264003C: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 82640040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640044: 390B8AD0  addi r8, r11, -0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + -30000;
	// 82640048: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264004C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82640050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640054: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264005C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640060: 386AD33C  addi r3, r10, -0x2cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11460;
	// 82640064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264006C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264007C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640084: 4BE26D9D  bl 0x82466e20
	ctx.lr = 0x82640088;
	sub_82466E20(ctx, base);
	// 82640088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640098 size=112
    let mut pc: u32 = 0x82640098;
    'dispatch: loop {
        match pc {
            0x82640098 => {
    //   block [0x82640098..0x82640108)
	// 82640098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264009C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826400A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826400A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826400A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826400AC: 38AAD2AC  addi r5, r10, -0x2d54
	ctx.r[5].s64 = ctx.r[10].s64 + -11604;
	// 826400B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826400B4: 390B8B00  addi r8, r11, -0x7500
	ctx.r[8].s64 = ctx.r[11].s64 + -29952;
	// 826400B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826400BC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826400C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826400C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826400C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826400CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826400D0: 386AD36C  addi r3, r10, -0x2c94
	ctx.r[3].s64 = ctx.r[10].s64 + -11412;
	// 826400D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826400D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826400DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826400E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826400E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826400E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826400EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826400F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826400F4: 4BE26D2D  bl 0x82466e20
	ctx.lr = 0x826400F8;
	sub_82466E20(ctx, base);
	// 826400F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826400FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640108 size=108
    let mut pc: u32 = 0x82640108;
    'dispatch: loop {
        match pc {
            0x82640108 => {
    //   block [0x82640108..0x82640174)
	// 82640108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640114: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264011C: 38EB8B48  addi r7, r11, -0x74b8
	ctx.r[7].s64 = ctx.r[11].s64 + -29880;
	// 82640120: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82640124: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82640128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264012C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640138: 386AD39C  addi r3, r10, -0x2c64
	ctx.r[3].s64 = ctx.r[10].s64 + -11364;
	// 8264013C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264015C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640160: 4BE26CC1  bl 0x82466e20
	ctx.lr = 0x82640164;
	sub_82466E20(ctx, base);
	// 82640164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264016C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640178 size=112
    let mut pc: u32 = 0x82640178;
    'dispatch: loop {
        match pc {
            0x82640178 => {
    //   block [0x82640178..0x826401E8)
	// 82640178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264017C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640188: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264018C: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 82640190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640194: 390B8B78  addi r8, r11, -0x7488
	ctx.r[8].s64 = ctx.r[11].s64 + -29832;
	// 82640198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264019C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826401A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826401A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826401A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826401AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826401B0: 386AD3CC  addi r3, r10, -0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + -11316;
	// 826401B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826401B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826401BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826401C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826401C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826401C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826401CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826401D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826401D4: 4BE26C4D  bl 0x82466e20
	ctx.lr = 0x826401D8;
	sub_82466E20(ctx, base);
	// 826401D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826401DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826401E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826401E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826401E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826401E8 size=108
    let mut pc: u32 = 0x826401E8;
    'dispatch: loop {
        match pc {
            0x826401E8 => {
    //   block [0x826401E8..0x82640254)
	// 826401E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826401EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826401F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826401F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826401F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826401FC: 38EB8B98  addi r7, r11, -0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + -29800;
	// 82640200: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82640204: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82640208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264020C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640218: 386AD3FC  addi r3, r10, -0x2c04
	ctx.r[3].s64 = ctx.r[10].s64 + -11268;
	// 8264021C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264022C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264023C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640240: 4BE26BE1  bl 0x82466e20
	ctx.lr = 0x82640244;
	sub_82466E20(ctx, base);
	// 82640244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264024C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640258 size=108
    let mut pc: u32 = 0x82640258;
    'dispatch: loop {
        match pc {
            0x82640258 => {
    //   block [0x82640258..0x826402C4)
	// 82640258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640264: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264026C: 38EB8BE0  addi r7, r11, -0x7420
	ctx.r[7].s64 = ctx.r[11].s64 + -29728;
	// 82640270: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82640274: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82640278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264027C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640288: 386AD42C  addi r3, r10, -0x2bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -11220;
	// 8264028C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826402A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826402A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826402A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826402AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826402B0: 4BE26B71  bl 0x82466e20
	ctx.lr = 0x826402B4;
	sub_82466E20(ctx, base);
	// 826402B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826402B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826402BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826402C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826402C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826402C8 size=116
    let mut pc: u32 = 0x826402C8;
    'dispatch: loop {
        match pc {
            0x826402C8 => {
    //   block [0x826402C8..0x8264033C)
	// 826402C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826402CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826402D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826402D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826402D8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826402DC: 392B6FEC  addi r9, r11, 0x6fec
	ctx.r[9].s64 = ctx.r[11].s64 + 28652;
	// 826402E0: 38AAD8AC  addi r5, r10, -0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + -10068;
	// 826402E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826402E8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826402EC: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826402F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826402F4: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826402F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826402FC: 396B8C40  addi r11, r11, -0x73c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29632;
	// 82640300: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82640304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640308: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640310: 386AD45C  addi r3, r10, -0x2ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -11172;
	// 82640314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640318: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264031C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640320: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82640324: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640328: 4BE26AF9  bl 0x82466e20
	ctx.lr = 0x8264032C;
	sub_82466E20(ctx, base);
	// 8264032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640340 size=100
    let mut pc: u32 = 0x82640340;
    'dispatch: loop {
        match pc {
            0x82640340 => {
    //   block [0x82640340..0x826403A4)
	// 82640340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264034C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640354: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264035C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640360: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82640364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640374: 386AD48C  addi r3, r10, -0x2b74
	ctx.r[3].s64 = ctx.r[10].s64 + -11124;
	// 82640378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264037C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640380: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264038C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640390: 4BE26A91  bl 0x82466e20
	ctx.lr = 0x82640394;
	sub_82466E20(ctx, base);
	// 82640394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264039C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826403A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826403A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826403A8 size=100
    let mut pc: u32 = 0x826403A8;
    'dispatch: loop {
        match pc {
            0x826403A8 => {
    //   block [0x826403A8..0x8264040C)
	// 826403A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826403AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826403B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826403B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826403B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826403BC: 38AAD51C  addi r5, r10, -0x2ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -10980;
	// 826403C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826403C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826403C8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826403CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826403D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826403D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826403D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826403DC: 386AD4BC  addi r3, r10, -0x2b44
	ctx.r[3].s64 = ctx.r[10].s64 + -11076;
	// 826403E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826403E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826403E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826403EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826403F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826403F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826403F8: 4BE26A29  bl 0x82466e20
	ctx.lr = 0x826403FC;
	sub_82466E20(ctx, base);
	// 826403FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640410 size=100
    let mut pc: u32 = 0x82640410;
    'dispatch: loop {
        match pc {
            0x82640410 => {
    //   block [0x82640410..0x82640474)
	// 82640410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264041C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640424: 38AAD45C  addi r5, r10, -0x2ba4
	ctx.r[5].s64 = ctx.r[10].s64 + -11172;
	// 82640428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264042C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640430: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82640434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264043C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640444: 386AD4EC  addi r3, r10, -0x2b14
	ctx.r[3].s64 = ctx.r[10].s64 + -11028;
	// 82640448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264044C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264045C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640460: 4BE269C1  bl 0x82466e20
	ctx.lr = 0x82640464;
	sub_82466E20(ctx, base);
	// 82640464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264046C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640478 size=104
    let mut pc: u32 = 0x82640478;
    'dispatch: loop {
        match pc {
            0x82640478 => {
    //   block [0x82640478..0x826404E0)
	// 82640478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264047C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640484: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82640488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264048C: 392A7068  addi r9, r10, 0x7068
	ctx.r[9].s64 = ctx.r[10].s64 + 28776;
	// 82640490: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640498: 38AAD48C  addi r5, r10, -0x2b74
	ctx.r[5].s64 = ctx.r[10].s64 + -11124;
	// 8264049C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826404A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826404A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826404A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826404AC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826404B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826404B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826404B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826404BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826404C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826404C4: 386AD51C  addi r3, r10, -0x2ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -10980;
	// 826404C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826404CC: 4BE26955  bl 0x82466e20
	ctx.lr = 0x826404D0;
	sub_82466E20(ctx, base);
	// 826404D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826404D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826404D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826404DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826404E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826404E0 size=108
    let mut pc: u32 = 0x826404E0;
    'dispatch: loop {
        match pc {
            0x826404E0 => {
    //   block [0x826404E0..0x8264054C)
	// 826404E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826404E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826404E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826404EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826404F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826404F4: 38EB8DD8  addi r7, r11, -0x7228
	ctx.r[7].s64 = ctx.r[11].s64 + -29224;
	// 826404F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826404FC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82640500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264050C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640510: 386AD54C  addi r3, r10, -0x2ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -10932;
	// 82640514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264051C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264052C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640538: 4BE268E9  bl 0x82466e20
	ctx.lr = 0x8264053C;
	sub_82466E20(ctx, base);
	// 8264053C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640550 size=112
    let mut pc: u32 = 0x82640550;
    'dispatch: loop {
        match pc {
            0x82640550 => {
    //   block [0x82640550..0x826405C0)
	// 82640550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264055C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640560: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640564: 38AAD51C  addi r5, r10, -0x2ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -10980;
	// 82640568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264056C: 390B8E08  addi r8, r11, -0x71f8
	ctx.r[8].s64 = ctx.r[11].s64 + -29176;
	// 82640570: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82640574: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82640578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264057C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640588: 386AD57C  addi r3, r10, -0x2a84
	ctx.r[3].s64 = ctx.r[10].s64 + -10884;
	// 8264058C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264059C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826405A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826405A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826405A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826405AC: 4BE26875  bl 0x82466e20
	ctx.lr = 0x826405B0;
	sub_82466E20(ctx, base);
	// 826405B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826405B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826405B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826405BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826405C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826405C0 size=24
    let mut pc: u32 = 0x826405C0;
    'dispatch: loop {
        match pc {
            0x826405C0 => {
    //   block [0x826405C0..0x826405D8)
	// 826405C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826405C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826405C8: 394ABF80  addi r10, r10, -0x4080
	ctx.r[10].s64 = ctx.r[10].s64 + -16512;
	// 826405CC: 816B8EB0  lwz r11, -0x7150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29008 as u32) ) } as u64;
	// 826405D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826405D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826405D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826405D8 size=116
    let mut pc: u32 = 0x826405D8;
    'dispatch: loop {
        match pc {
            0x826405D8 => {
    //   block [0x826405D8..0x8264064C)
	// 826405D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826405DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826405E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826405E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826405E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826405EC: 390BBF80  addi r8, r11, -0x4080
	ctx.r[8].s64 = ctx.r[11].s64 + -16512;
	// 826405F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826405F4: 392A70D0  addi r9, r10, 0x70d0
	ctx.r[9].s64 = ctx.r[10].s64 + 28880;
	// 826405F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826405FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82640600: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640604: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264060C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264061C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82640620: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82640624: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640628: 386BD5AC  addi r3, r11, -0x2a54
	ctx.r[3].s64 = ctx.r[11].s64 + -10836;
	// 8264062C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640638: 4BE267E9  bl 0x82466e20
	ctx.lr = 0x8264063C;
	sub_82466E20(ctx, base);
	// 8264063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640650 size=100
    let mut pc: u32 = 0x82640650;
    'dispatch: loop {
        match pc {
            0x82640650 => {
    //   block [0x82640650..0x826406B4)
	// 82640650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264065C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640664: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264066C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640670: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82640674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264067C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640684: 386AD5DC  addi r3, r10, -0x2a24
	ctx.r[3].s64 = ctx.r[10].s64 + -10788;
	// 82640688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264068C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826406A0: 4BE26781  bl 0x82466e20
	ctx.lr = 0x826406A4;
	sub_82466E20(ctx, base);
	// 826406A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826406A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826406AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826406B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826406B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826406B8 size=100
    let mut pc: u32 = 0x826406B8;
    'dispatch: loop {
        match pc {
            0x826406B8 => {
    //   block [0x826406B8..0x8264071C)
	// 826406B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826406BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826406C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826406C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826406C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826406CC: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826406D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826406D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826406D8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826406DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826406E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826406E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826406E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826406EC: 386AD60C  addi r3, r10, -0x29f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10740;
	// 826406F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826406F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826406F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826406FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640708: 4BE26719  bl 0x82466e20
	ctx.lr = 0x8264070C;
	sub_82466E20(ctx, base);
	// 8264070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640720 size=112
    let mut pc: u32 = 0x82640720;
    'dispatch: loop {
        match pc {
            0x82640720 => {
    //   block [0x82640720..0x82640790)
	// 82640720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264072C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640730: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640734: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264073C: 390B8EB4  addi r8, r11, -0x714c
	ctx.r[8].s64 = ctx.r[11].s64 + -29004;
	// 82640740: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82640744: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82640748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264074C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640758: 386AD63C  addi r3, r10, -0x29c4
	ctx.r[3].s64 = ctx.r[10].s64 + -10692;
	// 8264075C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264076C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264077C: 4BE266A5  bl 0x82466e20
	ctx.lr = 0x82640780;
	sub_82466E20(ctx, base);
	// 82640780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264078C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640790 size=100
    let mut pc: u32 = 0x82640790;
    'dispatch: loop {
        match pc {
            0x82640790 => {
    //   block [0x82640790..0x826407F4)
	// 82640790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264079C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826407A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826407A4: 38AAD63C  addi r5, r10, -0x29c4
	ctx.r[5].s64 = ctx.r[10].s64 + -10692;
	// 826407A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826407AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826407B0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826407B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826407B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826407BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826407C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826407C4: 386AD66C  addi r3, r10, -0x2994
	ctx.r[3].s64 = ctx.r[10].s64 + -10644;
	// 826407C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826407CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826407D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826407D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826407D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826407DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826407E0: 4BE26641  bl 0x82466e20
	ctx.lr = 0x826407E4;
	sub_82466E20(ctx, base);
	// 826407E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826407E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826407EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826407F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826407F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826407F8 size=100
    let mut pc: u32 = 0x826407F8;
    'dispatch: loop {
        match pc {
            0x826407F8 => {
    //   block [0x826407F8..0x8264085C)
	// 826407F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826407FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264080C: 38AAD5AC  addi r5, r10, -0x2a54
	ctx.r[5].s64 = ctx.r[10].s64 + -10836;
	// 82640810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640818: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8264081C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264082C: 386AD69C  addi r3, r10, -0x2964
	ctx.r[3].s64 = ctx.r[10].s64 + -10596;
	// 82640830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640834: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640838: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640840: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640848: 4BE265D9  bl 0x82466e20
	ctx.lr = 0x8264084C;
	sub_82466E20(ctx, base);
	// 8264084C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640860 size=100
    let mut pc: u32 = 0x82640860;
    'dispatch: loop {
        match pc {
            0x82640860 => {
    //   block [0x82640860..0x826408C4)
	// 82640860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264086C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640874: 38AAD5DC  addi r5, r10, -0x2a24
	ctx.r[5].s64 = ctx.r[10].s64 + -10788;
	// 82640878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264087C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640880: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82640884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264088C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640894: 386AD6CC  addi r3, r10, -0x2934
	ctx.r[3].s64 = ctx.r[10].s64 + -10548;
	// 82640898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264089C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826408A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826408A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826408A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826408AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826408B0: 4BE26571  bl 0x82466e20
	ctx.lr = 0x826408B4;
	sub_82466E20(ctx, base);
	// 826408B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826408B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826408BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826408C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826408C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826408C8 size=100
    let mut pc: u32 = 0x826408C8;
    'dispatch: loop {
        match pc {
            0x826408C8 => {
    //   block [0x826408C8..0x8264092C)
	// 826408C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826408CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826408D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826408D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826408D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826408DC: 38AAD69C  addi r5, r10, -0x2964
	ctx.r[5].s64 = ctx.r[10].s64 + -10596;
	// 826408E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826408E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826408E8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826408EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826408F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826408F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826408F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826408FC: 386AD6FC  addi r3, r10, -0x2904
	ctx.r[3].s64 = ctx.r[10].s64 + -10500;
	// 82640900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264090C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640918: 4BE26509  bl 0x82466e20
	ctx.lr = 0x8264091C;
	sub_82466E20(ctx, base);
	// 8264091C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640930 size=100
    let mut pc: u32 = 0x82640930;
    'dispatch: loop {
        match pc {
            0x82640930 => {
    //   block [0x82640930..0x82640994)
	// 82640930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264093C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640944: 38AAD5DC  addi r5, r10, -0x2a24
	ctx.r[5].s64 = ctx.r[10].s64 + -10788;
	// 82640948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264094C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640950: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82640954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264095C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640964: 386AD72C  addi r3, r10, -0x28d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10452;
	// 82640968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264096C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82640974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264097C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640980: 4BE264A1  bl 0x82466e20
	ctx.lr = 0x82640984;
	sub_82466E20(ctx, base);
	// 82640984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264098C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640998 size=112
    let mut pc: u32 = 0x82640998;
    'dispatch: loop {
        match pc {
            0x82640998 => {
    //   block [0x82640998..0x82640A08)
	// 82640998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264099C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826409A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826409A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826409A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826409AC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 826409B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826409B4: 390B8EE4  addi r8, r11, -0x711c
	ctx.r[8].s64 = ctx.r[11].s64 + -28956;
	// 826409B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826409BC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826409C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826409C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826409C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826409CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826409D0: 386AD75C  addi r3, r10, -0x28a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10404;
	// 826409D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826409D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826409DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826409E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826409E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826409E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826409EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826409F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826409F4: 4BE2642D  bl 0x82466e20
	ctx.lr = 0x826409F8;
	sub_82466E20(ctx, base);
	// 826409F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826409FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640A08 size=112
    let mut pc: u32 = 0x82640A08;
    'dispatch: loop {
        match pc {
            0x82640A08 => {
    //   block [0x82640A08..0x82640A78)
	// 82640A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640A1C: 38AAD7EC  addi r5, r10, -0x2814
	ctx.r[5].s64 = ctx.r[10].s64 + -10260;
	// 82640A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640A24: 390B8F14  addi r8, r11, -0x70ec
	ctx.r[8].s64 = ctx.r[11].s64 + -28908;
	// 82640A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640A2C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82640A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640A40: 386AD78C  addi r3, r10, -0x2874
	ctx.r[3].s64 = ctx.r[10].s64 + -10356;
	// 82640A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640A64: 4BE263BD  bl 0x82466e20
	ctx.lr = 0x82640A68;
	sub_82466E20(ctx, base);
	// 82640A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640A78 size=112
    let mut pc: u32 = 0x82640A78;
    'dispatch: loop {
        match pc {
            0x82640A78 => {
    //   block [0x82640A78..0x82640AE8)
	// 82640A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640A8C: 38AAD8AC  addi r5, r10, -0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + -10068;
	// 82640A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640A94: 390B8F2C  addi r8, r11, -0x70d4
	ctx.r[8].s64 = ctx.r[11].s64 + -28884;
	// 82640A98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82640A9C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82640AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640AB0: 386AD7BC  addi r3, r10, -0x2844
	ctx.r[3].s64 = ctx.r[10].s64 + -10308;
	// 82640AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640AD4: 4BE2634D  bl 0x82466e20
	ctx.lr = 0x82640AD8;
	sub_82466E20(ctx, base);
	// 82640AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640AE8 size=112
    let mut pc: u32 = 0x82640AE8;
    'dispatch: loop {
        match pc {
            0x82640AE8 => {
    //   block [0x82640AE8..0x82640B58)
	// 82640AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640AFC: 38AAD7BC  addi r5, r10, -0x2844
	ctx.r[5].s64 = ctx.r[10].s64 + -10308;
	// 82640B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640B04: 390B8F5C  addi r8, r11, -0x70a4
	ctx.r[8].s64 = ctx.r[11].s64 + -28836;
	// 82640B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640B0C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82640B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640B20: 386AD7EC  addi r3, r10, -0x2814
	ctx.r[3].s64 = ctx.r[10].s64 + -10260;
	// 82640B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640B44: 4BE262DD  bl 0x82466e20
	ctx.lr = 0x82640B48;
	sub_82466E20(ctx, base);
	// 82640B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640B58 size=112
    let mut pc: u32 = 0x82640B58;
    'dispatch: loop {
        match pc {
            0x82640B58 => {
    //   block [0x82640B58..0x82640BC8)
	// 82640B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640B64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640B6C: 38AAD7EC  addi r5, r10, -0x2814
	ctx.r[5].s64 = ctx.r[10].s64 + -10260;
	// 82640B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640B74: 390B8F74  addi r8, r11, -0x708c
	ctx.r[8].s64 = ctx.r[11].s64 + -28812;
	// 82640B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640B7C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82640B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640B84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640B90: 386AD81C  addi r3, r10, -0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10212;
	// 82640B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640BB4: 4BE2626D  bl 0x82466e20
	ctx.lr = 0x82640BB8;
	sub_82466E20(ctx, base);
	// 82640BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640BC8 size=116
    let mut pc: u32 = 0x82640BC8;
    'dispatch: loop {
        match pc {
            0x82640BC8 => {
    //   block [0x82640BC8..0x82640C3C)
	// 82640BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640BD4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640BD8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82640BDC: 390A8F90  addi r8, r10, -0x7070
	ctx.r[8].s64 = ctx.r[10].s64 + -28784;
	// 82640BE0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640BE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82640BE8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640BEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640BF0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640BFC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82640C00: 396B70E4  addi r11, r11, 0x70e4
	ctx.r[11].s64 = ctx.r[11].s64 + 28900;
	// 82640C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640C08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640C0C: 386AD84C  addi r3, r10, -0x27b4
	ctx.r[3].s64 = ctx.r[10].s64 + -10164;
	// 82640C10: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82640C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640C18: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82640C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640C28: 4BE261F9  bl 0x82466e20
	ctx.lr = 0x82640C2C;
	sub_82466E20(ctx, base);
	// 82640C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82640C40 size=48
    let mut pc: u32 = 0x82640C40;
    'dispatch: loop {
        match pc {
            0x82640C40 => {
    //   block [0x82640C40..0x82640C70)
	// 82640C40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C44: 814B9040  lwz r10, -0x6fc0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28608 as u32) ) } as u64;
	// 82640C48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C4C: 396BC040  addi r11, r11, -0x3fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -16320;
	// 82640C50: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82640C54: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640C58: 814A903C  lwz r10, -0x6fc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28612 as u32) ) } as u64;
	// 82640C5C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82640C60: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82640C64: 814A9038  lwz r10, -0x6fc8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28616 as u32) ) } as u64;
	// 82640C68: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 82640C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640C70 size=116
    let mut pc: u32 = 0x82640C70;
    'dispatch: loop {
        match pc {
            0x82640C70 => {
    //   block [0x82640C70..0x82640CE4)
	// 82640C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82640C80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640C84: 392B71B8  addi r9, r11, 0x71b8
	ctx.r[9].s64 = ctx.r[11].s64 + 29112;
	// 82640C88: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640C90: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82640C94: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82640C98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640C9C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82640CA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640CA4: 396BC040  addi r11, r11, -0x3fc0
	ctx.r[11].s64 = ctx.r[11].s64 + -16320;
	// 82640CA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82640CAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640CB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82640CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640CB8: 386AD87C  addi r3, r10, -0x2784
	ctx.r[3].s64 = ctx.r[10].s64 + -10116;
	// 82640CBC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82640CC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82640CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640CC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82640CCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82640CD0: 4BE26151  bl 0x82466e20
	ctx.lr = 0x82640CD4;
	sub_82466E20(ctx, base);
	// 82640CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640CE8 size=116
    let mut pc: u32 = 0x82640CE8;
    'dispatch: loop {
        match pc {
            0x82640CE8 => {
    //   block [0x82640CE8..0x82640D5C)
	// 82640CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640CF4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640CF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82640CFC: 390B9048  addi r8, r11, -0x6fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -28600;
	// 82640D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640D04: 392A7334  addi r9, r10, 0x7334
	ctx.r[9].s64 = ctx.r[10].s64 + 29492;
	// 82640D08: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D0C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82640D10: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640D14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640D1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640D2C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82640D30: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82640D34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82640D38: 386BD8AC  addi r3, r11, -0x2754
	ctx.r[3].s64 = ctx.r[11].s64 + -10068;
	// 82640D3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82640D40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640D48: 4BE260D9  bl 0x82466e20
	ctx.lr = 0x82640D4C;
	sub_82466E20(ctx, base);
	// 82640D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640D60 size=112
    let mut pc: u32 = 0x82640D60;
    'dispatch: loop {
        match pc {
            0x82640D60 => {
    //   block [0x82640D60..0x82640DD0)
	// 82640D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640D74: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640D7C: 390B90D8  addi r8, r11, -0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + -28456;
	// 82640D80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640D84: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82640D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640D98: 386AD8DC  addi r3, r10, -0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + -10020;
	// 82640D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640DBC: 4BE26065  bl 0x82466e20
	ctx.lr = 0x82640DC0;
	sub_82466E20(ctx, base);
	// 82640DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640DD0 size=112
    let mut pc: u32 = 0x82640DD0;
    'dispatch: loop {
        match pc {
            0x82640DD0 => {
    //   block [0x82640DD0..0x82640E40)
	// 82640DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640DDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640DE0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640DE4: 38AAB8FC  addi r5, r10, -0x4704
	ctx.r[5].s64 = ctx.r[10].s64 + -18180;
	// 82640DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640DEC: 390B90F0  addi r8, r11, -0x6f10
	ctx.r[8].s64 = ctx.r[11].s64 + -28432;
	// 82640DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82640DF4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82640DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640DFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640E08: 386AD90C  addi r3, r10, -0x26f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9972;
	// 82640E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640E2C: 4BE25FF5  bl 0x82466e20
	ctx.lr = 0x82640E30;
	sub_82466E20(ctx, base);
	// 82640E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640E40 size=108
    let mut pc: u32 = 0x82640E40;
    'dispatch: loop {
        match pc {
            0x82640E40 => {
    //   block [0x82640E40..0x82640EAC)
	// 82640E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640E4C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640E54: 38EB9108  addi r7, r11, -0x6ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -28408;
	// 82640E58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82640E5C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82640E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640E64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640E68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640E70: 386AD93C  addi r3, r10, -0x26c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9924;
	// 82640E74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640E98: 4BE25F89  bl 0x82466e20
	ctx.lr = 0x82640E9C;
	sub_82466E20(ctx, base);
	// 82640E9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640EB0 size=112
    let mut pc: u32 = 0x82640EB0;
    'dispatch: loop {
        match pc {
            0x82640EB0 => {
    //   block [0x82640EB0..0x82640F20)
	// 82640EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640EC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640EC4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82640EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640ECC: 390B9120  addi r8, r11, -0x6ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -28384;
	// 82640ED0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82640ED4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82640ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82640EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640EE8: 386AD96C  addi r3, r10, -0x2694
	ctx.r[3].s64 = ctx.r[10].s64 + -9876;
	// 82640EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82640EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640F0C: 4BE25F15  bl 0x82466e20
	ctx.lr = 0x82640F10;
	sub_82466E20(ctx, base);
	// 82640F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640F20 size=108
    let mut pc: u32 = 0x82640F20;
    'dispatch: loop {
        match pc {
            0x82640F20 => {
    //   block [0x82640F20..0x82640F8C)
	// 82640F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640F2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640F34: 38EB9168  addi r7, r11, -0x6e98
	ctx.r[7].s64 = ctx.r[11].s64 + -28312;
	// 82640F38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82640F3C: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 82640F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640F50: 386AD99C  addi r3, r10, -0x2664
	ctx.r[3].s64 = ctx.r[10].s64 + -9828;
	// 82640F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640F78: 4BE25EA9  bl 0x82466e20
	ctx.lr = 0x82640F7C;
	sub_82466E20(ctx, base);
	// 82640F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82640F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82640F90 size=108
    let mut pc: u32 = 0x82640F90;
    'dispatch: loop {
        match pc {
            0x82640F90 => {
    //   block [0x82640F90..0x82640FFC)
	// 82640F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82640F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82640F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82640F9C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82640FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82640FA4: 38EB9198  addi r7, r11, -0x6e68
	ctx.r[7].s64 = ctx.r[11].s64 + -28264;
	// 82640FA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82640FAC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82640FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82640FB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82640FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82640FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82640FC0: 386AD9CC  addi r3, r10, -0x2634
	ctx.r[3].s64 = ctx.r[10].s64 + -9780;
	// 82640FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82640FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82640FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82640FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82640FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82640FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82640FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82640FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82640FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82640FE8: 4BE25E39  bl 0x82466e20
	ctx.lr = 0x82640FEC;
	sub_82466E20(ctx, base);
	// 82640FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82640FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82640FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82640FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641000 size=112
    let mut pc: u32 = 0x82641000;
    'dispatch: loop {
        match pc {
            0x82641000 => {
    //   block [0x82641000..0x82641070)
	// 82641000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264100C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641014: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264101C: 390B91B0  addi r8, r11, -0x6e50
	ctx.r[8].s64 = ctx.r[11].s64 + -28240;
	// 82641020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641024: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82641028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264102C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641038: 386AD9FC  addi r3, r10, -0x2604
	ctx.r[3].s64 = ctx.r[10].s64 + -9732;
	// 8264103C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264104C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264105C: 4BE25DC5  bl 0x82466e20
	ctx.lr = 0x82641060;
	sub_82466E20(ctx, base);
	// 82641060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264106C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641070 size=112
    let mut pc: u32 = 0x82641070;
    'dispatch: loop {
        match pc {
            0x82641070 => {
    //   block [0x82641070..0x826410E0)
	// 82641070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264107C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82641080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641084: 392A738C  addi r9, r10, 0x738c
	ctx.r[9].s64 = ctx.r[10].s64 + 29580;
	// 82641088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264108C: 390B91E8  addi r8, r11, -0x6e18
	ctx.r[8].s64 = ctx.r[11].s64 + -28184;
	// 82641090: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82641094: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82641098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264109C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826410A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826410A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826410A8: 386ADA2C  addi r3, r10, -0x25d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9684;
	// 826410AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826410B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826410B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826410B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826410BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826410C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826410C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826410C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826410CC: 4BE25D55  bl 0x82466e20
	ctx.lr = 0x826410D0;
	sub_82466E20(ctx, base);
	// 826410D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826410D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826410D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826410DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826410E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826410E0 size=116
    let mut pc: u32 = 0x826410E0;
    'dispatch: loop {
        match pc {
            0x826410E0 => {
    //   block [0x826410E0..0x82641154)
	// 826410E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826410E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826410E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826410EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826410F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826410F4: 390B9290  addi r8, r11, -0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + -28016;
	// 826410F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826410FC: 392A7360  addi r9, r10, 0x7360
	ctx.r[9].s64 = ctx.r[10].s64 + 29536;
	// 82641100: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641104: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82641108: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8264110C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641124: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82641128: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8264112C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82641130: 386BDA5C  addi r3, r11, -0x25a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9636;
	// 82641134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82641138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264113C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641140: 4BE25CE1  bl 0x82466e20
	ctx.lr = 0x82641144;
	sub_82466E20(ctx, base);
	// 82641144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264114C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641158 size=112
    let mut pc: u32 = 0x82641158;
    'dispatch: loop {
        match pc {
            0x82641158 => {
    //   block [0x82641158..0x826411C8)
	// 82641158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641164: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82641168: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264116C: 392A73B8  addi r9, r10, 0x73b8
	ctx.r[9].s64 = ctx.r[10].s64 + 29624;
	// 82641170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641174: 390B92B0  addi r8, r11, -0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + -27984;
	// 82641178: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264117C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82641180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641190: 386ADA8C  addi r3, r10, -0x2574
	ctx.r[3].s64 = ctx.r[10].s64 + -9588;
	// 82641194: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82641198: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264119C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826411A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826411A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826411A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826411AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826411B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826411B4: 4BE25C6D  bl 0x82466e20
	ctx.lr = 0x826411B8;
	sub_82466E20(ctx, base);
	// 826411B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826411BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826411C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826411C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826411C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826411C8 size=112
    let mut pc: u32 = 0x826411C8;
    'dispatch: loop {
        match pc {
            0x826411C8 => {
    //   block [0x826411C8..0x82641238)
	// 826411C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826411CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826411D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826411D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826411D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826411DC: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 826411E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826411E4: 390B9310  addi r8, r11, -0x6cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27888;
	// 826411E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826411EC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826411F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826411F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826411F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826411FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641200: 386ADABC  addi r3, r10, -0x2544
	ctx.r[3].s64 = ctx.r[10].s64 + -9540;
	// 82641204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264120C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264121C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641224: 4BE25BFD  bl 0x82466e20
	ctx.lr = 0x82641228;
	sub_82466E20(ctx, base);
	// 82641228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264122C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641238 size=112
    let mut pc: u32 = 0x82641238;
    'dispatch: loop {
        match pc {
            0x82641238 => {
    //   block [0x82641238..0x826412A8)
	// 82641238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264123C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641248: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264124C: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 82641250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641254: 390B9328  addi r8, r11, -0x6cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -27864;
	// 82641258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264125C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82641260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264126C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641270: 386ADAEC  addi r3, r10, -0x2514
	ctx.r[3].s64 = ctx.r[10].s64 + -9492;
	// 82641274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264127C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264128C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641294: 4BE25B8D  bl 0x82466e20
	ctx.lr = 0x82641298;
	sub_82466E20(ctx, base);
	// 82641298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264129C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826412A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826412A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826412A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826412A8 size=112
    let mut pc: u32 = 0x826412A8;
    'dispatch: loop {
        match pc {
            0x826412A8 => {
    //   block [0x826412A8..0x82641318)
	// 826412A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826412AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826412B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826412B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826412B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826412BC: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 826412C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826412C4: 390B9370  addi r8, r11, -0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + -27792;
	// 826412C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826412CC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826412D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826412D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826412D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826412DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826412E0: 386ADB1C  addi r3, r10, -0x24e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9444;
	// 826412E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826412E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826412EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826412F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826412F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826412F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826412FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641304: 4BE25B1D  bl 0x82466e20
	ctx.lr = 0x82641308;
	sub_82466E20(ctx, base);
	// 82641308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264130C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641318 size=112
    let mut pc: u32 = 0x82641318;
    'dispatch: loop {
        match pc {
            0x82641318 => {
    //   block [0x82641318..0x82641388)
	// 82641318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264131C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641328: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264132C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641334: 390B93D0  addi r8, r11, -0x6c30
	ctx.r[8].s64 = ctx.r[11].s64 + -27696;
	// 82641338: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264133C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82641340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641344: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264134C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641350: 386ADB4C  addi r3, r10, -0x24b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9396;
	// 82641354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264135C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264136C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641374: 4BE25AAD  bl 0x82466e20
	ctx.lr = 0x82641378;
	sub_82466E20(ctx, base);
	// 82641378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264137C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641388 size=112
    let mut pc: u32 = 0x82641388;
    'dispatch: loop {
        match pc {
            0x82641388 => {
    //   block [0x82641388..0x826413F8)
	// 82641388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264138C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641398: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264139C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 826413A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826413A4: 390B9430  addi r8, r11, -0x6bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -27600;
	// 826413A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826413AC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826413B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826413B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826413B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826413BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826413C0: 386ADB7C  addi r3, r10, -0x2484
	ctx.r[3].s64 = ctx.r[10].s64 + -9348;
	// 826413C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826413C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826413CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826413D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826413D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826413D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826413DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826413E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826413E4: 4BE25A3D  bl 0x82466e20
	ctx.lr = 0x826413E8;
	sub_82466E20(ctx, base);
	// 826413E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826413EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826413F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826413F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826413F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826413F8 size=112
    let mut pc: u32 = 0x826413F8;
    'dispatch: loop {
        match pc {
            0x826413F8 => {
    //   block [0x826413F8..0x82641468)
	// 826413F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826413FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641408: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264140C: 38AACAFC  addi r5, r10, -0x3504
	ctx.r[5].s64 = ctx.r[10].s64 + -13572;
	// 82641410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641414: 390B9490  addi r8, r11, -0x6b70
	ctx.r[8].s64 = ctx.r[11].s64 + -27504;
	// 82641418: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8264141C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82641420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264142C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641430: 386ADBAC  addi r3, r10, -0x2454
	ctx.r[3].s64 = ctx.r[10].s64 + -9300;
	// 82641434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264144C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641454: 4BE259CD  bl 0x82466e20
	ctx.lr = 0x82641458;
	sub_82466E20(ctx, base);
	// 82641458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264145C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641468 size=112
    let mut pc: u32 = 0x82641468;
    'dispatch: loop {
        match pc {
            0x82641468 => {
    //   block [0x82641468..0x826414D8)
	// 82641468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264146C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641474: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82641478: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8264147C: 38EA9550  addi r7, r10, -0x6ab0
	ctx.r[7].s64 = ctx.r[10].s64 + -27312;
	// 82641480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641484: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82641488: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8264148C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641490: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641494: 396B73D0  addi r11, r11, 0x73d0
	ctx.r[11].s64 = ctx.r[11].s64 + 29648;
	// 82641498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264149C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826414A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826414A4: 386ADBDC  addi r3, r10, -0x2424
	ctx.r[3].s64 = ctx.r[10].s64 + -9252;
	// 826414A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826414AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826414B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826414B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826414B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826414BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826414C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826414C4: 4BE2595D  bl 0x82466e20
	ctx.lr = 0x826414C8;
	sub_82466E20(ctx, base);
	// 826414C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826414CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826414D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826414D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826414D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826414D8 size=112
    let mut pc: u32 = 0x826414D8;
    'dispatch: loop {
        match pc {
            0x826414D8 => {
    //   block [0x826414D8..0x82641548)
	// 826414D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826414DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826414E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826414E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826414E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826414EC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826414F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826414F4: 390B9718  addi r8, r11, -0x68e8
	ctx.r[8].s64 = ctx.r[11].s64 + -26856;
	// 826414F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826414FC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82641500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264150C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641510: 386ADC0C  addi r3, r10, -0x23f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9204;
	// 82641514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641524: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82641528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641534: 4BE258ED  bl 0x82466e20
	ctx.lr = 0x82641538;
	sub_82466E20(ctx, base);
	// 82641538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264153C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641548 size=112
    let mut pc: u32 = 0x82641548;
    'dispatch: loop {
        match pc {
            0x82641548 => {
    //   block [0x82641548..0x826415B8)
	// 82641548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641558: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264155C: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 82641560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641564: 390B9730  addi r8, r11, -0x68d0
	ctx.r[8].s64 = ctx.r[11].s64 + -26832;
	// 82641568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264156C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82641570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641580: 386ADC3C  addi r3, r10, -0x23c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9156;
	// 82641584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641594: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82641598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826415A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826415A4: 4BE2587D  bl 0x82466e20
	ctx.lr = 0x826415A8;
	sub_82466E20(ctx, base);
	// 826415A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826415AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826415B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826415B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826415B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826415B8 size=112
    let mut pc: u32 = 0x826415B8;
    'dispatch: loop {
        match pc {
            0x826415B8 => {
    //   block [0x826415B8..0x82641628)
	// 826415B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826415BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826415C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826415C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826415C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826415CC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826415D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826415D4: 390B9748  addi r8, r11, -0x68b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26808;
	// 826415D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826415DC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826415E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826415E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826415E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826415EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826415F0: 386ADC6C  addi r3, r10, -0x2394
	ctx.r[3].s64 = ctx.r[10].s64 + -9108;
	// 826415F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826415F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826415FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264160C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641614: 4BE2580D  bl 0x82466e20
	ctx.lr = 0x82641618;
	sub_82466E20(ctx, base);
	// 82641618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264161C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641628 size=108
    let mut pc: u32 = 0x82641628;
    'dispatch: loop {
        match pc {
            0x82641628 => {
    //   block [0x82641628..0x82641694)
	// 82641628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641634: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264163C: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 82641640: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641644: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82641648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264164C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641658: 386ADC9C  addi r3, r10, -0x2364
	ctx.r[3].s64 = ctx.r[10].s64 + -9060;
	// 8264165C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264167C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641680: 4BE257A1  bl 0x82466e20
	ctx.lr = 0x82641684;
	sub_82466E20(ctx, base);
	// 82641684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264168C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641698 size=112
    let mut pc: u32 = 0x82641698;
    'dispatch: loop {
        match pc {
            0x82641698 => {
    //   block [0x82641698..0x82641708)
	// 82641698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826416A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826416A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826416A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826416AC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 826416B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826416B4: 390B97A8  addi r8, r11, -0x6858
	ctx.r[8].s64 = ctx.r[11].s64 + -26712;
	// 826416B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826416BC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826416C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826416C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826416C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826416CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826416D0: 386ADCCC  addi r3, r10, -0x2334
	ctx.r[3].s64 = ctx.r[10].s64 + -9012;
	// 826416D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826416D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826416DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826416E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826416E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826416E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826416EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826416F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826416F4: 4BE2572D  bl 0x82466e20
	ctx.lr = 0x826416F8;
	sub_82466E20(ctx, base);
	// 826416F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826416FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641708 size=108
    let mut pc: u32 = 0x82641708;
    'dispatch: loop {
        match pc {
            0x82641708 => {
    //   block [0x82641708..0x82641774)
	// 82641708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641714: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264171C: 38EB97C0  addi r7, r11, -0x6840
	ctx.r[7].s64 = ctx.r[11].s64 + -26688;
	// 82641720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641724: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82641728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264172C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641738: 386ADCFC  addi r3, r10, -0x2304
	ctx.r[3].s64 = ctx.r[10].s64 + -8964;
	// 8264173C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264175C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641760: 4BE256C1  bl 0x82466e20
	ctx.lr = 0x82641764;
	sub_82466E20(ctx, base);
	// 82641764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264176C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641778 size=108
    let mut pc: u32 = 0x82641778;
    'dispatch: loop {
        match pc {
            0x82641778 => {
    //   block [0x82641778..0x826417E4)
	// 82641778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641784: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264178C: 38EB97F0  addi r7, r11, -0x6810
	ctx.r[7].s64 = ctx.r[11].s64 + -26640;
	// 82641790: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82641794: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82641798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264179C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826417A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826417A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826417A8: 386ADD2C  addi r3, r10, -0x22d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8916;
	// 826417AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826417B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826417B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826417B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826417BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826417C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826417C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826417C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826417CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826417D0: 4BE25651  bl 0x82466e20
	ctx.lr = 0x826417D4;
	sub_82466E20(ctx, base);
	// 826417D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826417D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826417DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826417E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826417E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826417E8 size=112
    let mut pc: u32 = 0x826417E8;
    'dispatch: loop {
        match pc {
            0x826417E8 => {
    //   block [0x826417E8..0x82641858)
	// 826417E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826417EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826417F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826417F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826417F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826417FC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641804: 390B9838  addi r8, r11, -0x67c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26568;
	// 82641808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264180C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82641810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641820: 386ADD5C  addi r3, r10, -0x22a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8868;
	// 82641824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264182C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264183C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641844: 4BE255DD  bl 0x82466e20
	ctx.lr = 0x82641848;
	sub_82466E20(ctx, base);
	// 82641848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264184C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641858 size=112
    let mut pc: u32 = 0x82641858;
    'dispatch: loop {
        match pc {
            0x82641858 => {
    //   block [0x82641858..0x826418C8)
	// 82641858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264185C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264186C: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641874: 390B9880  addi r8, r11, -0x6780
	ctx.r[8].s64 = ctx.r[11].s64 + -26496;
	// 82641878: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264187C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82641880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264188C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641890: 386ADD8C  addi r3, r10, -0x2274
	ctx.r[3].s64 = ctx.r[10].s64 + -8820;
	// 82641894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826418A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826418A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826418A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826418AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826418B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826418B4: 4BE2556D  bl 0x82466e20
	ctx.lr = 0x826418B8;
	sub_82466E20(ctx, base);
	// 826418B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826418BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826418C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826418C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826418C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826418C8 size=108
    let mut pc: u32 = 0x826418C8;
    'dispatch: loop {
        match pc {
            0x826418C8 => {
    //   block [0x826418C8..0x82641934)
	// 826418C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826418CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826418D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826418D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826418D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826418DC: 38EB9910  addi r7, r11, -0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26352;
	// 826418E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826418E4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826418E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826418EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826418F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826418F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826418F8: 386ADDBC  addi r3, r10, -0x2244
	ctx.r[3].s64 = ctx.r[10].s64 + -8772;
	// 826418FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264191C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641920: 4BE25501  bl 0x82466e20
	ctx.lr = 0x82641924;
	sub_82466E20(ctx, base);
	// 82641924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641938 size=108
    let mut pc: u32 = 0x82641938;
    'dispatch: loop {
        match pc {
            0x82641938 => {
    //   block [0x82641938..0x826419A4)
	// 82641938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641944: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264194C: 38EB9958  addi r7, r11, -0x66a8
	ctx.r[7].s64 = ctx.r[11].s64 + -26280;
	// 82641950: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641954: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82641958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264195C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641968: 386ADDEC  addi r3, r10, -0x2214
	ctx.r[3].s64 = ctx.r[10].s64 + -8724;
	// 8264196C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264197C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264198C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641990: 4BE25491  bl 0x82466e20
	ctx.lr = 0x82641994;
	sub_82466E20(ctx, base);
	// 82641994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264199C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826419A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826419A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826419A8 size=108
    let mut pc: u32 = 0x826419A8;
    'dispatch: loop {
        match pc {
            0x826419A8 => {
    //   block [0x826419A8..0x82641A14)
	// 826419A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826419AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826419B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826419B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826419B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826419BC: 38EB9988  addi r7, r11, -0x6678
	ctx.r[7].s64 = ctx.r[11].s64 + -26232;
	// 826419C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826419C4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826419C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826419CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826419D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826419D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826419D8: 386ADE1C  addi r3, r10, -0x21e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8676;
	// 826419DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826419E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826419E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826419E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826419EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826419F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826419F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826419F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826419FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641A00: 4BE25421  bl 0x82466e20
	ctx.lr = 0x82641A04;
	sub_82466E20(ctx, base);
	// 82641A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641A18 size=112
    let mut pc: u32 = 0x82641A18;
    'dispatch: loop {
        match pc {
            0x82641A18 => {
    //   block [0x82641A18..0x82641A88)
	// 82641A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641A24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641A2C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641A34: 390B99B8  addi r8, r11, -0x6648
	ctx.r[8].s64 = ctx.r[11].s64 + -26184;
	// 82641A38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641A3C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82641A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641A44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641A50: 386ADE4C  addi r3, r10, -0x21b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8628;
	// 82641A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641A74: 4BE253AD  bl 0x82466e20
	ctx.lr = 0x82641A78;
	sub_82466E20(ctx, base);
	// 82641A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641A88 size=112
    let mut pc: u32 = 0x82641A88;
    'dispatch: loop {
        match pc {
            0x82641A88 => {
    //   block [0x82641A88..0x82641AF8)
	// 82641A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641A94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641A98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641A9C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641AA4: 390B99E8  addi r8, r11, -0x6618
	ctx.r[8].s64 = ctx.r[11].s64 + -26136;
	// 82641AA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641AAC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82641AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641AC0: 386ADE7C  addi r3, r10, -0x2184
	ctx.r[3].s64 = ctx.r[10].s64 + -8580;
	// 82641AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641AE4: 4BE2533D  bl 0x82466e20
	ctx.lr = 0x82641AE8;
	sub_82466E20(ctx, base);
	// 82641AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641AF8 size=112
    let mut pc: u32 = 0x82641AF8;
    'dispatch: loop {
        match pc {
            0x82641AF8 => {
    //   block [0x82641AF8..0x82641B68)
	// 82641AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641B0C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641B14: 390B9A00  addi r8, r11, -0x6600
	ctx.r[8].s64 = ctx.r[11].s64 + -26112;
	// 82641B18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641B1C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82641B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641B30: 386ADEAC  addi r3, r10, -0x2154
	ctx.r[3].s64 = ctx.r[10].s64 + -8532;
	// 82641B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641B54: 4BE252CD  bl 0x82466e20
	ctx.lr = 0x82641B58;
	sub_82466E20(ctx, base);
	// 82641B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641B68 size=108
    let mut pc: u32 = 0x82641B68;
    'dispatch: loop {
        match pc {
            0x82641B68 => {
    //   block [0x82641B68..0x82641BD4)
	// 82641B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641B74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641B7C: 38EB9A18  addi r7, r11, -0x65e8
	ctx.r[7].s64 = ctx.r[11].s64 + -26088;
	// 82641B80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82641B84: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82641B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641B98: 386ADEDC  addi r3, r10, -0x2124
	ctx.r[3].s64 = ctx.r[10].s64 + -8484;
	// 82641B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641BC0: 4BE25261  bl 0x82466e20
	ctx.lr = 0x82641BC4;
	sub_82466E20(ctx, base);
	// 82641BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641BD8 size=112
    let mut pc: u32 = 0x82641BD8;
    'dispatch: loop {
        match pc {
            0x82641BD8 => {
    //   block [0x82641BD8..0x82641C48)
	// 82641BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641BE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641BE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641BEC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641BF4: 390B9A48  addi r8, r11, -0x65b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26040;
	// 82641BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82641BFC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82641C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641C04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641C10: 386ADF0C  addi r3, r10, -0x20f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8436;
	// 82641C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641C34: 4BE251ED  bl 0x82466e20
	ctx.lr = 0x82641C38;
	sub_82466E20(ctx, base);
	// 82641C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641C48 size=108
    let mut pc: u32 = 0x82641C48;
    'dispatch: loop {
        match pc {
            0x82641C48 => {
    //   block [0x82641C48..0x82641CB4)
	// 82641C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641C54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641C5C: 38EB9A60  addi r7, r11, -0x65a0
	ctx.r[7].s64 = ctx.r[11].s64 + -26016;
	// 82641C60: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82641C64: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82641C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641C6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641C78: 386ADF3C  addi r3, r10, -0x20c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8388;
	// 82641C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641CA0: 4BE25181  bl 0x82466e20
	ctx.lr = 0x82641CA4;
	sub_82466E20(ctx, base);
	// 82641CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641CB8 size=112
    let mut pc: u32 = 0x82641CB8;
    'dispatch: loop {
        match pc {
            0x82641CB8 => {
    //   block [0x82641CB8..0x82641D28)
	// 82641CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641CC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641CC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641CCC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641CD4: 390B9B50  addi r8, r11, -0x64b0
	ctx.r[8].s64 = ctx.r[11].s64 + -25776;
	// 82641CD8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82641CDC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82641CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641CF0: 386ADF6C  addi r3, r10, -0x2094
	ctx.r[3].s64 = ctx.r[10].s64 + -8340;
	// 82641CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641D14: 4BE2510D  bl 0x82466e20
	ctx.lr = 0x82641D18;
	sub_82466E20(ctx, base);
	// 82641D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641D28 size=108
    let mut pc: u32 = 0x82641D28;
    'dispatch: loop {
        match pc {
            0x82641D28 => {
    //   block [0x82641D28..0x82641D94)
	// 82641D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641D34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641D38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641D3C: 38EB9D00  addi r7, r11, -0x6300
	ctx.r[7].s64 = ctx.r[11].s64 + -25344;
	// 82641D40: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82641D44: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82641D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641D4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641D50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641D58: 386ADF9C  addi r3, r10, -0x2064
	ctx.r[3].s64 = ctx.r[10].s64 + -8292;
	// 82641D5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641D80: 4BE250A1  bl 0x82466e20
	ctx.lr = 0x82641D84;
	sub_82466E20(ctx, base);
	// 82641D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641D98 size=112
    let mut pc: u32 = 0x82641D98;
    'dispatch: loop {
        match pc {
            0x82641D98 => {
    //   block [0x82641D98..0x82641E08)
	// 82641D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641DA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641DA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641DAC: 38AACB2C  addi r5, r10, -0x34d4
	ctx.r[5].s64 = ctx.r[10].s64 + -13524;
	// 82641DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641DB4: 390B9E98  addi r8, r11, -0x6168
	ctx.r[8].s64 = ctx.r[11].s64 + -24936;
	// 82641DB8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 82641DBC: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82641DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641DD0: 386ADFCC  addi r3, r10, -0x2034
	ctx.r[3].s64 = ctx.r[10].s64 + -8244;
	// 82641DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641DF4: 4BE2502D  bl 0x82466e20
	ctx.lr = 0x82641DF8;
	sub_82466E20(ctx, base);
	// 82641DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641E08 size=100
    let mut pc: u32 = 0x82641E08;
    'dispatch: loop {
        match pc {
            0x82641E08 => {
    //   block [0x82641E08..0x82641E6C)
	// 82641E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641E14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641E1C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641E28: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82641E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641E3C: 386ADFFC  addi r3, r10, -0x2004
	ctx.r[3].s64 = ctx.r[10].s64 + -8196;
	// 82641E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641E44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641E48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82641E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641E50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82641E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641E58: 4BE24FC9  bl 0x82466e20
	ctx.lr = 0x82641E5C;
	sub_82466E20(ctx, base);
	// 82641E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641E70 size=112
    let mut pc: u32 = 0x82641E70;
    'dispatch: loop {
        match pc {
            0x82641E70 => {
    //   block [0x82641E70..0x82641EE0)
	// 82641E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641E7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641E80: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641E84: 38AADFFC  addi r5, r10, -0x2004
	ctx.r[5].s64 = ctx.r[10].s64 + -8196;
	// 82641E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641E8C: 390BA0F0  addi r8, r11, -0x5f10
	ctx.r[8].s64 = ctx.r[11].s64 + -24336;
	// 82641E90: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82641E94: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82641E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641E9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641EA8: 386AE02C  addi r3, r10, -0x1fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -8148;
	// 82641EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641ECC: 4BE24F55  bl 0x82466e20
	ctx.lr = 0x82641ED0;
	sub_82466E20(ctx, base);
	// 82641ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641EE0 size=100
    let mut pc: u32 = 0x82641EE0;
    'dispatch: loop {
        match pc {
            0x82641EE0 => {
    //   block [0x82641EE0..0x82641F44)
	// 82641EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641EF4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82641EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641F00: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82641F04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641F14: 386AE05C  addi r3, r10, -0x1fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -8100;
	// 82641F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641F20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82641F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82641F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641F30: 4BE24EF1  bl 0x82466e20
	ctx.lr = 0x82641F34;
	sub_82466E20(ctx, base);
	// 82641F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641F48 size=108
    let mut pc: u32 = 0x82641F48;
    'dispatch: loop {
        match pc {
            0x82641F48 => {
    //   block [0x82641F48..0x82641FB4)
	// 82641F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641F54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641F5C: 38EBA168  addi r7, r11, -0x5e98
	ctx.r[7].s64 = ctx.r[11].s64 + -24216;
	// 82641F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82641F64: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82641F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641F6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82641F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82641F78: 386AE08C  addi r3, r10, -0x1f74
	ctx.r[3].s64 = ctx.r[10].s64 + -8052;
	// 82641F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82641F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82641F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82641F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82641F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82641F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82641F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82641FA0: 4BE24E81  bl 0x82466e20
	ctx.lr = 0x82641FA4;
	sub_82466E20(ctx, base);
	// 82641FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82641FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82641FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82641FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82641FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82641FB8 size=112
    let mut pc: u32 = 0x82641FB8;
    'dispatch: loop {
        match pc {
            0x82641FB8 => {
    //   block [0x82641FB8..0x82642028)
	// 82641FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82641FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82641FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82641FC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641FC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82641FCC: 38AAE05C  addi r5, r10, -0x1fa4
	ctx.r[5].s64 = ctx.r[10].s64 + -8100;
	// 82641FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82641FD4: 390BA1B0  addi r8, r11, -0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + -24144;
	// 82641FD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82641FDC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82641FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82641FE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82641FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82641FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82641FF0: 386AE0BC  addi r3, r10, -0x1f44
	ctx.r[3].s64 = ctx.r[10].s64 + -8004;
	// 82641FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82641FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82641FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642014: 4BE24E0D  bl 0x82466e20
	ctx.lr = 0x82642018;
	sub_82466E20(ctx, base);
	// 82642018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642028 size=100
    let mut pc: u32 = 0x82642028;
    'dispatch: loop {
        match pc {
            0x82642028 => {
    //   block [0x82642028..0x8264208C)
	// 82642028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642034: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264203C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642048: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8264204C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264205C: 386AE0EC  addi r3, r10, -0x1f14
	ctx.r[3].s64 = ctx.r[10].s64 + -7956;
	// 82642060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642064: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642068: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642070: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642078: 4BE24DA9  bl 0x82466e20
	ctx.lr = 0x8264207C;
	sub_82466E20(ctx, base);
	// 8264207C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642090 size=100
    let mut pc: u32 = 0x82642090;
    'dispatch: loop {
        match pc {
            0x82642090 => {
    //   block [0x82642090..0x826420F4)
	// 82642090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264209C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826420A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826420A4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826420A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826420AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826420B0: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826420B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826420B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826420BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826420C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826420C4: 386AE11C  addi r3, r10, -0x1ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -7908;
	// 826420C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826420CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826420D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826420D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826420D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826420DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826420E0: 4BE24D41  bl 0x82466e20
	ctx.lr = 0x826420E4;
	sub_82466E20(ctx, base);
	// 826420E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826420E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826420EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826420F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826420F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826420F8 size=112
    let mut pc: u32 = 0x826420F8;
    'dispatch: loop {
        match pc {
            0x826420F8 => {
    //   block [0x826420F8..0x82642168)
	// 826420F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826420FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642108: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264210C: 38AAE0EC  addi r5, r10, -0x1f14
	ctx.r[5].s64 = ctx.r[10].s64 + -7956;
	// 82642110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642114: 390BA1E0  addi r8, r11, -0x5e20
	ctx.r[8].s64 = ctx.r[11].s64 + -24096;
	// 82642118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264211C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82642120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264212C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642130: 386AE14C  addi r3, r10, -0x1eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7860;
	// 82642134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264213C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264214C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642154: 4BE24CCD  bl 0x82466e20
	ctx.lr = 0x82642158;
	sub_82466E20(ctx, base);
	// 82642158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642168 size=112
    let mut pc: u32 = 0x82642168;
    'dispatch: loop {
        match pc {
            0x82642168 => {
    //   block [0x82642168..0x826421D8)
	// 82642168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642178: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264217C: 38AAE11C  addi r5, r10, -0x1ee4
	ctx.r[5].s64 = ctx.r[10].s64 + -7908;
	// 82642180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642184: 390BA240  addi r8, r11, -0x5dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -24000;
	// 82642188: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264218C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82642190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264219C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826421A0: 386AE17C  addi r3, r10, -0x1e84
	ctx.r[3].s64 = ctx.r[10].s64 + -7812;
	// 826421A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826421A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826421AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826421B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826421B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826421B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826421BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826421C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826421C4: 4BE24C5D  bl 0x82466e20
	ctx.lr = 0x826421C8;
	sub_82466E20(ctx, base);
	// 826421C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826421CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826421D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826421D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826421D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826421D8 size=100
    let mut pc: u32 = 0x826421D8;
    'dispatch: loop {
        match pc {
            0x826421D8 => {
    //   block [0x826421D8..0x8264223C)
	// 826421D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826421DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826421E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826421E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826421E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826421EC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826421F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826421F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826421F8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826421FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264220C: 386AE1AC  addi r3, r10, -0x1e54
	ctx.r[3].s64 = ctx.r[10].s64 + -7764;
	// 82642210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642218: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264221C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642220: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642228: 4BE24BF9  bl 0x82466e20
	ctx.lr = 0x8264222C;
	sub_82466E20(ctx, base);
	// 8264222C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642240 size=112
    let mut pc: u32 = 0x82642240;
    'dispatch: loop {
        match pc {
            0x82642240 => {
    //   block [0x82642240..0x826422B0)
	// 82642240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264224C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642250: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642254: 38AAE1AC  addi r5, r10, -0x1e54
	ctx.r[5].s64 = ctx.r[10].s64 + -7764;
	// 82642258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264225C: 390BA2A0  addi r8, r11, -0x5d60
	ctx.r[8].s64 = ctx.r[11].s64 + -23904;
	// 82642260: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82642264: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82642268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264226C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642278: 386AE1DC  addi r3, r10, -0x1e24
	ctx.r[3].s64 = ctx.r[10].s64 + -7716;
	// 8264227C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264228C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264229C: 4BE24B85  bl 0x82466e20
	ctx.lr = 0x826422A0;
	sub_82466E20(ctx, base);
	// 826422A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826422A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826422A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826422AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826422B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826422B0 size=108
    let mut pc: u32 = 0x826422B0;
    'dispatch: loop {
        match pc {
            0x826422B0 => {
    //   block [0x826422B0..0x8264231C)
	// 826422B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826422B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826422B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826422BC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826422C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826422C4: 38EBA390  addi r7, r11, -0x5c70
	ctx.r[7].s64 = ctx.r[11].s64 + -23664;
	// 826422C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826422CC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826422D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826422D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826422D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826422DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826422E0: 386AE20C  addi r3, r10, -0x1df4
	ctx.r[3].s64 = ctx.r[10].s64 + -7668;
	// 826422E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826422E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826422EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826422F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826422F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826422F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826422FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642308: 4BE24B19  bl 0x82466e20
	ctx.lr = 0x8264230C;
	sub_82466E20(ctx, base);
	// 8264230C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642320 size=108
    let mut pc: u32 = 0x82642320;
    'dispatch: loop {
        match pc {
            0x82642320 => {
    //   block [0x82642320..0x8264238C)
	// 82642320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264232C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642334: 38EBA480  addi r7, r11, -0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + -23424;
	// 82642338: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264233C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82642340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642344: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264234C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642350: 386AE23C  addi r3, r10, -0x1dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -7620;
	// 82642354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264235C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264236C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642378: 4BE24AA9  bl 0x82466e20
	ctx.lr = 0x8264237C;
	sub_82466E20(ctx, base);
	// 8264237C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642390 size=108
    let mut pc: u32 = 0x82642390;
    'dispatch: loop {
        match pc {
            0x82642390 => {
    //   block [0x82642390..0x826423FC)
	// 82642390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264239C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826423A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826423A4: 38EBA4C8  addi r7, r11, -0x5b38
	ctx.r[7].s64 = ctx.r[11].s64 + -23352;
	// 826423A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826423AC: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826423B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826423B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826423B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826423BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826423C0: 386AE26C  addi r3, r10, -0x1d94
	ctx.r[3].s64 = ctx.r[10].s64 + -7572;
	// 826423C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826423C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826423CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826423D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826423D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826423D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826423DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826423E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826423E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826423E8: 4BE24A39  bl 0x82466e20
	ctx.lr = 0x826423EC;
	sub_82466E20(ctx, base);
	// 826423EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826423F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826423F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826423F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642400 size=108
    let mut pc: u32 = 0x82642400;
    'dispatch: loop {
        match pc {
            0x82642400 => {
    //   block [0x82642400..0x8264246C)
	// 82642400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264240C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642414: 38EBA5A0  addi r7, r11, -0x5a60
	ctx.r[7].s64 = ctx.r[11].s64 + -23136;
	// 82642418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264241C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82642420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264242C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642430: 386AE29C  addi r3, r10, -0x1d64
	ctx.r[3].s64 = ctx.r[10].s64 + -7524;
	// 82642434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264243C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264244C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642458: 4BE249C9  bl 0x82466e20
	ctx.lr = 0x8264245C;
	sub_82466E20(ctx, base);
	// 8264245C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642470 size=100
    let mut pc: u32 = 0x82642470;
    'dispatch: loop {
        match pc {
            0x82642470 => {
    //   block [0x82642470..0x826424D4)
	// 82642470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264247C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642484: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264248C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642490: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82642494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264249C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826424A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826424A4: 386AE2CC  addi r3, r10, -0x1d34
	ctx.r[3].s64 = ctx.r[10].s64 + -7476;
	// 826424A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826424AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826424B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826424B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826424B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826424BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826424C0: 4BE24961  bl 0x82466e20
	ctx.lr = 0x826424C4;
	sub_82466E20(ctx, base);
	// 826424C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826424C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826424CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826424D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826424D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826424D8 size=112
    let mut pc: u32 = 0x826424D8;
    'dispatch: loop {
        match pc {
            0x826424D8 => {
    //   block [0x826424D8..0x82642548)
	// 826424D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826424DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826424E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826424E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826424E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826424EC: 38AAE2CC  addi r5, r10, -0x1d34
	ctx.r[5].s64 = ctx.r[10].s64 + -7476;
	// 826424F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826424F4: 390BA5B8  addi r8, r11, -0x5a48
	ctx.r[8].s64 = ctx.r[11].s64 + -23112;
	// 826424F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826424FC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82642500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264250C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642510: 386AE2FC  addi r3, r10, -0x1d04
	ctx.r[3].s64 = ctx.r[10].s64 + -7428;
	// 82642514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264251C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264252C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642534: 4BE248ED  bl 0x82466e20
	ctx.lr = 0x82642538;
	sub_82466E20(ctx, base);
	// 82642538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642548 size=108
    let mut pc: u32 = 0x82642548;
    'dispatch: loop {
        match pc {
            0x82642548 => {
    //   block [0x82642548..0x826425B4)
	// 82642548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642554: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264255C: 38EBA600  addi r7, r11, -0x5a00
	ctx.r[7].s64 = ctx.r[11].s64 + -23040;
	// 82642560: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82642564: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82642568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264256C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642578: 386AE32C  addi r3, r10, -0x1cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -7380;
	// 8264257C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264258C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264259C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826425A0: 4BE24881  bl 0x82466e20
	ctx.lr = 0x826425A4;
	sub_82466E20(ctx, base);
	// 826425A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826425A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826425AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826425B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826425B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826425B8 size=112
    let mut pc: u32 = 0x826425B8;
    'dispatch: loop {
        match pc {
            0x826425B8 => {
    //   block [0x826425B8..0x82642628)
	// 826425B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826425BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826425C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826425C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826425C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826425CC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826425D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826425D4: 390BA648  addi r8, r11, -0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22968;
	// 826425D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826425DC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826425E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826425E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826425E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826425EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826425F0: 386AE35C  addi r3, r10, -0x1ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -7332;
	// 826425F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826425F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826425FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264260C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642614: 4BE2480D  bl 0x82466e20
	ctx.lr = 0x82642618;
	sub_82466E20(ctx, base);
	// 82642618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264261C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642628 size=108
    let mut pc: u32 = 0x82642628;
    'dispatch: loop {
        match pc {
            0x82642628 => {
    //   block [0x82642628..0x82642694)
	// 82642628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264262C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642634: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264263C: 38EBA660  addi r7, r11, -0x59a0
	ctx.r[7].s64 = ctx.r[11].s64 + -22944;
	// 82642640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82642644: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82642648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264264C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642658: 386AE38C  addi r3, r10, -0x1c74
	ctx.r[3].s64 = ctx.r[10].s64 + -7284;
	// 8264265C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264266C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264267C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642680: 4BE247A1  bl 0x82466e20
	ctx.lr = 0x82642684;
	sub_82466E20(ctx, base);
	// 82642684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264268C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642698 size=112
    let mut pc: u32 = 0x82642698;
    'dispatch: loop {
        match pc {
            0x82642698 => {
    //   block [0x82642698..0x82642708)
	// 82642698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264269C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826426A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826426A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826426A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826426AC: 38AAE35C  addi r5, r10, -0x1ca4
	ctx.r[5].s64 = ctx.r[10].s64 + -7332;
	// 826426B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826426B4: 390BA6A8  addi r8, r11, -0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + -22872;
	// 826426B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826426BC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826426C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826426C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826426C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826426CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826426D0: 386AE3BC  addi r3, r10, -0x1c44
	ctx.r[3].s64 = ctx.r[10].s64 + -7236;
	// 826426D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826426D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826426DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826426E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826426E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826426E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826426EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826426F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826426F4: 4BE2472D  bl 0x82466e20
	ctx.lr = 0x826426F8;
	sub_82466E20(ctx, base);
	// 826426F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826426FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642708 size=100
    let mut pc: u32 = 0x82642708;
    'dispatch: loop {
        match pc {
            0x82642708 => {
    //   block [0x82642708..0x8264276C)
	// 82642708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642714: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264271C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642728: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8264272C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264273C: 386AE3EC  addi r3, r10, -0x1c14
	ctx.r[3].s64 = ctx.r[10].s64 + -7188;
	// 82642740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264274C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82642754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642758: 4BE246C9  bl 0x82466e20
	ctx.lr = 0x8264275C;
	sub_82466E20(ctx, base);
	// 8264275C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642770 size=112
    let mut pc: u32 = 0x82642770;
    'dispatch: loop {
        match pc {
            0x82642770 => {
    //   block [0x82642770..0x826427E0)
	// 82642770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264277C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642780: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642784: 38AAE3EC  addi r5, r10, -0x1c14
	ctx.r[5].s64 = ctx.r[10].s64 + -7188;
	// 82642788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264278C: 390BA6C0  addi r8, r11, -0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + -22848;
	// 82642790: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82642794: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82642798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264279C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826427A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826427A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826427A8: 386AE41C  addi r3, r10, -0x1be4
	ctx.r[3].s64 = ctx.r[10].s64 + -7140;
	// 826427AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826427B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826427B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826427B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826427BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826427C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826427C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826427C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826427CC: 4BE24655  bl 0x82466e20
	ctx.lr = 0x826427D0;
	sub_82466E20(ctx, base);
	// 826427D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826427D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826427D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826427DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826427E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826427E0 size=108
    let mut pc: u32 = 0x826427E0;
    'dispatch: loop {
        match pc {
            0x826427E0 => {
    //   block [0x826427E0..0x8264284C)
	// 826427E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826427E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826427E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826427EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826427F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826427F4: 38EBA768  addi r7, r11, -0x5898
	ctx.r[7].s64 = ctx.r[11].s64 + -22680;
	// 826427F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826427FC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82642800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264280C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642810: 386AE44C  addi r3, r10, -0x1bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7092;
	// 82642814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264281C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264282C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642838: 4BE245E9  bl 0x82466e20
	ctx.lr = 0x8264283C;
	sub_82466E20(ctx, base);
	// 8264283C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642850 size=112
    let mut pc: u32 = 0x82642850;
    'dispatch: loop {
        match pc {
            0x82642850 => {
    //   block [0x82642850..0x826428C0)
	// 82642850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264285C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642860: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642864: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264286C: 390BA798  addi r8, r11, -0x5868
	ctx.r[8].s64 = ctx.r[11].s64 + -22632;
	// 82642870: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82642874: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82642878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264287C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642888: 386AE47C  addi r3, r10, -0x1b84
	ctx.r[3].s64 = ctx.r[10].s64 + -7044;
	// 8264288C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264289C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826428A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826428A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826428A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826428AC: 4BE24575  bl 0x82466e20
	ctx.lr = 0x826428B0;
	sub_82466E20(ctx, base);
	// 826428B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826428B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826428B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826428BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826428C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826428C0 size=112
    let mut pc: u32 = 0x826428C0;
    'dispatch: loop {
        match pc {
            0x826428C0 => {
    //   block [0x826428C0..0x82642930)
	// 826428C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826428C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826428C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826428CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826428D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826428D4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 826428D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826428DC: 390BA7E0  addi r8, r11, -0x5820
	ctx.r[8].s64 = ctx.r[11].s64 + -22560;
	// 826428E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826428E4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826428E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826428EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826428F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826428F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826428F8: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 826428FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264291C: 4BE24505  bl 0x82466e20
	ctx.lr = 0x82642920;
	sub_82466E20(ctx, base);
	// 82642920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642930 size=100
    let mut pc: u32 = 0x82642930;
    'dispatch: loop {
        match pc {
            0x82642930 => {
    //   block [0x82642930..0x82642994)
	// 82642930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264293C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642944: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264294C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642950: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82642954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264295C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642964: 386AE4DC  addi r3, r10, -0x1b24
	ctx.r[3].s64 = ctx.r[10].s64 + -6948;
	// 82642968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264296C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82642974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264297C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642980: 4BE244A1  bl 0x82466e20
	ctx.lr = 0x82642984;
	sub_82466E20(ctx, base);
	// 82642984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264298C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642998 size=112
    let mut pc: u32 = 0x82642998;
    'dispatch: loop {
        match pc {
            0x82642998 => {
    //   block [0x82642998..0x82642A08)
	// 82642998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264299C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826429A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826429A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826429A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826429AC: 38AAE4DC  addi r5, r10, -0x1b24
	ctx.r[5].s64 = ctx.r[10].s64 + -6948;
	// 826429B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826429B4: 390BA828  addi r8, r11, -0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + -22488;
	// 826429B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826429BC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826429C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826429C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826429C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826429CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826429D0: 386AE50C  addi r3, r10, -0x1af4
	ctx.r[3].s64 = ctx.r[10].s64 + -6900;
	// 826429D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826429D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826429DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826429E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826429E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826429E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826429EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826429F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826429F4: 4BE2442D  bl 0x82466e20
	ctx.lr = 0x826429F8;
	sub_82466E20(ctx, base);
	// 826429F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826429FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642A08 size=112
    let mut pc: u32 = 0x82642A08;
    'dispatch: loop {
        match pc {
            0x82642A08 => {
    //   block [0x82642A08..0x82642A78)
	// 82642A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642A14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642A1C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642A24: 390BA870  addi r8, r11, -0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + -22416;
	// 82642A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642A2C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82642A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642A34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642A40: 386AE53C  addi r3, r10, -0x1ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -6852;
	// 82642A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642A64: 4BE243BD  bl 0x82466e20
	ctx.lr = 0x82642A68;
	sub_82466E20(ctx, base);
	// 82642A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642A78 size=112
    let mut pc: u32 = 0x82642A78;
    'dispatch: loop {
        match pc {
            0x82642A78 => {
    //   block [0x82642A78..0x82642AE8)
	// 82642A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642A84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642A88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642A8C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82642A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642A94: 390BA888  addi r8, r11, -0x5778
	ctx.r[8].s64 = ctx.r[11].s64 + -22392;
	// 82642A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642A9C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82642AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642AB0: 386AE56C  addi r3, r10, -0x1a94
	ctx.r[3].s64 = ctx.r[10].s64 + -6804;
	// 82642AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642AC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82642AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642AD4: 4BE2434D  bl 0x82466e20
	ctx.lr = 0x82642AD8;
	sub_82466E20(ctx, base);
	// 82642AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642AE8 size=112
    let mut pc: u32 = 0x82642AE8;
    'dispatch: loop {
        match pc {
            0x82642AE8 => {
    //   block [0x82642AE8..0x82642B58)
	// 82642AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642AF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642AF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642AFC: 38AAE53C  addi r5, r10, -0x1ac4
	ctx.r[5].s64 = ctx.r[10].s64 + -6852;
	// 82642B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642B04: 390BA8A0  addi r8, r11, -0x5760
	ctx.r[8].s64 = ctx.r[11].s64 + -22368;
	// 82642B08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82642B0C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82642B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642B20: 386AE59C  addi r3, r10, -0x1a64
	ctx.r[3].s64 = ctx.r[10].s64 + -6756;
	// 82642B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642B44: 4BE242DD  bl 0x82466e20
	ctx.lr = 0x82642B48;
	sub_82466E20(ctx, base);
	// 82642B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642B58 size=72
    let mut pc: u32 = 0x82642B58;
    'dispatch: loop {
        match pc {
            0x82642B58 => {
    //   block [0x82642B58..0x82642BA0)
	// 82642B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642B64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82642B68: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82642B6C: 38CB5D10  addi r6, r11, 0x5d10
	ctx.r[6].s64 = ctx.r[11].s64 + 23824;
	// 82642B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82642B74: 388B7428  addi r4, r11, 0x7428
	ctx.r[4].s64 = ctx.r[11].s64 + 29736;
	// 82642B78: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82642B7C: 386BE5CC  addi r3, r11, -0x1a34
	ctx.r[3].s64 = ctx.r[11].s64 + -6708;
	// 82642B80: 4BE38F09  bl 0x8247ba88
	ctx.lr = 0x82642B84;
	sub_8247BA88(ctx, base);
	// 82642B84: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82642B88: 386BCD98  addi r3, r11, -0x3268
	ctx.r[3].s64 = ctx.r[11].s64 + -12904;
	// 82642B8C: 4BEEFFAD  bl 0x82532b38
	ctx.lr = 0x82642B90;
	sub_82532B38(ctx, base);
	// 82642B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82642B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642BA0 size=108
    let mut pc: u32 = 0x82642BA0;
    'dispatch: loop {
        match pc {
            0x82642BA0 => {
    //   block [0x82642BA0..0x82642C0C)
	// 82642BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642BAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642BB4: 38EBC460  addi r7, r11, -0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + -15264;
	// 82642BB8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82642BBC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82642BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642BC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642BD0: 386AE5E4  addi r3, r10, -0x1a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -6684;
	// 82642BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642BF8: 4BE24229  bl 0x82466e20
	ctx.lr = 0x82642BFC;
	sub_82466E20(ctx, base);
	// 82642BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642C10 size=108
    let mut pc: u32 = 0x82642C10;
    'dispatch: loop {
        match pc {
            0x82642C10 => {
    //   block [0x82642C10..0x82642C7C)
	// 82642C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642C1C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642C24: 38EBC4D8  addi r7, r11, -0x3b28
	ctx.r[7].s64 = ctx.r[11].s64 + -15144;
	// 82642C28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642C2C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82642C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642C34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642C40: 386AE614  addi r3, r10, -0x19ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6636;
	// 82642C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642C68: 4BE241B9  bl 0x82466e20
	ctx.lr = 0x82642C6C;
	sub_82466E20(ctx, base);
	// 82642C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642C80 size=108
    let mut pc: u32 = 0x82642C80;
    'dispatch: loop {
        match pc {
            0x82642C80 => {
    //   block [0x82642C80..0x82642CEC)
	// 82642C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642C8C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642C94: 38EBC508  addi r7, r11, -0x3af8
	ctx.r[7].s64 = ctx.r[11].s64 + -15096;
	// 82642C98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642C9C: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82642CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642CA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642CB0: 386AE644  addi r3, r10, -0x19bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6588;
	// 82642CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642CD8: 4BE24149  bl 0x82466e20
	ctx.lr = 0x82642CDC;
	sub_82466E20(ctx, base);
	// 82642CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642CF0 size=108
    let mut pc: u32 = 0x82642CF0;
    'dispatch: loop {
        match pc {
            0x82642CF0 => {
    //   block [0x82642CF0..0x82642D5C)
	// 82642CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642CFC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642D04: 38EBC538  addi r7, r11, -0x3ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -15048;
	// 82642D08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82642D0C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82642D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642D14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642D20: 386AE674  addi r3, r10, -0x198c
	ctx.r[3].s64 = ctx.r[10].s64 + -6540;
	// 82642D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642D48: 4BE240D9  bl 0x82466e20
	ctx.lr = 0x82642D4C;
	sub_82466E20(ctx, base);
	// 82642D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642D60 size=112
    let mut pc: u32 = 0x82642D60;
    'dispatch: loop {
        match pc {
            0x82642D60 => {
    //   block [0x82642D60..0x82642DD0)
	// 82642D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642D6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642D74: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 82642D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642D7C: 390BC568  addi r8, r11, -0x3a98
	ctx.r[8].s64 = ctx.r[11].s64 + -15000;
	// 82642D80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82642D84: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82642D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642D8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642D98: 386AE6A4  addi r3, r10, -0x195c
	ctx.r[3].s64 = ctx.r[10].s64 + -6492;
	// 82642D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642DBC: 4BE24065  bl 0x82466e20
	ctx.lr = 0x82642DC0;
	sub_82466E20(ctx, base);
	// 82642DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642DD0 size=108
    let mut pc: u32 = 0x82642DD0;
    'dispatch: loop {
        match pc {
            0x82642DD0 => {
    //   block [0x82642DD0..0x82642E3C)
	// 82642DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642DDC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642DE4: 38EBC5C8  addi r7, r11, -0x3a38
	ctx.r[7].s64 = ctx.r[11].s64 + -14904;
	// 82642DE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82642DEC: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82642DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642DF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642E00: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 82642E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642E28: 4BE23FF9  bl 0x82466e20
	ctx.lr = 0x82642E2C;
	sub_82466E20(ctx, base);
	// 82642E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642E40 size=112
    let mut pc: u32 = 0x82642E40;
    'dispatch: loop {
        match pc {
            0x82642E40 => {
    //   block [0x82642E40..0x82642EB0)
	// 82642E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642E4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642E50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642E54: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642E5C: 390BC628  addi r8, r11, -0x39d8
	ctx.r[8].s64 = ctx.r[11].s64 + -14808;
	// 82642E60: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82642E64: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82642E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642E6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642E78: 386AE704  addi r3, r10, -0x18fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6396;
	// 82642E7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642E9C: 4BE23F85  bl 0x82466e20
	ctx.lr = 0x82642EA0;
	sub_82466E20(ctx, base);
	// 82642EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642EB0 size=112
    let mut pc: u32 = 0x82642EB0;
    'dispatch: loop {
        match pc {
            0x82642EB0 => {
    //   block [0x82642EB0..0x82642F20)
	// 82642EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642EBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642EC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642EC4: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642ECC: 390BC6B8  addi r8, r11, -0x3948
	ctx.r[8].s64 = ctx.r[11].s64 + -14664;
	// 82642ED0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82642ED4: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82642ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642EDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642EE8: 386AE734  addi r3, r10, -0x18cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6348;
	// 82642EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642F0C: 4BE23F15  bl 0x82466e20
	ctx.lr = 0x82642F10;
	sub_82466E20(ctx, base);
	// 82642F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642F20 size=108
    let mut pc: u32 = 0x82642F20;
    'dispatch: loop {
        match pc {
            0x82642F20 => {
    //   block [0x82642F20..0x82642F8C)
	// 82642F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642F2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642F34: 38EBC6D0  addi r7, r11, -0x3930
	ctx.r[7].s64 = ctx.r[11].s64 + -14640;
	// 82642F38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82642F3C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82642F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642F44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82642F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642F50: 386AE764  addi r3, r10, -0x189c
	ctx.r[3].s64 = ctx.r[10].s64 + -6300;
	// 82642F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82642F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82642F78: 4BE23EA9  bl 0x82466e20
	ctx.lr = 0x82642F7C;
	sub_82466E20(ctx, base);
	// 82642F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82642F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82642F90 size=112
    let mut pc: u32 = 0x82642F90;
    'dispatch: loop {
        match pc {
            0x82642F90 => {
    //   block [0x82642F90..0x82643000)
	// 82642F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82642F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82642F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82642F9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642FA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82642FA4: 38AAE6A4  addi r5, r10, -0x195c
	ctx.r[5].s64 = ctx.r[10].s64 + -6492;
	// 82642FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82642FAC: 390BC730  addi r8, r11, -0x38d0
	ctx.r[8].s64 = ctx.r[11].s64 + -14544;
	// 82642FB0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82642FB4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82642FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82642FBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82642FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82642FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82642FC8: 386AE794  addi r3, r10, -0x186c
	ctx.r[3].s64 = ctx.r[10].s64 + -6252;
	// 82642FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82642FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82642FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82642FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82642FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82642FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82642FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82642FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82642FEC: 4BE23E35  bl 0x82466e20
	ctx.lr = 0x82642FF0;
	sub_82466E20(ctx, base);
	// 82642FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82642FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82642FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82642FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643000 size=108
    let mut pc: u32 = 0x82643000;
    'dispatch: loop {
        match pc {
            0x82643000 => {
    //   block [0x82643000..0x8264306C)
	// 82643000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264300C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643014: 38EBC7D8  addi r7, r11, -0x3828
	ctx.r[7].s64 = ctx.r[11].s64 + -14376;
	// 82643018: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264301C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82643020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264302C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643030: 386AE7C4  addi r3, r10, -0x183c
	ctx.r[3].s64 = ctx.r[10].s64 + -6204;
	// 82643034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264303C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264304C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643058: 4BE23DC9  bl 0x82466e20
	ctx.lr = 0x8264305C;
	sub_82466E20(ctx, base);
	// 8264305C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643070 size=108
    let mut pc: u32 = 0x82643070;
    'dispatch: loop {
        match pc {
            0x82643070 => {
    //   block [0x82643070..0x826430DC)
	// 82643070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264307C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643084: 38EBC7F0  addi r7, r11, -0x3810
	ctx.r[7].s64 = ctx.r[11].s64 + -14352;
	// 82643088: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264308C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82643090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264309C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826430A0: 386AE7F4  addi r3, r10, -0x180c
	ctx.r[3].s64 = ctx.r[10].s64 + -6156;
	// 826430A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826430A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826430AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826430B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826430B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826430B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826430BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826430C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826430C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826430C8: 4BE23D59  bl 0x82466e20
	ctx.lr = 0x826430CC;
	sub_82466E20(ctx, base);
	// 826430CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826430D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826430D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826430D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826430E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826430E0 size=112
    let mut pc: u32 = 0x826430E0;
    'dispatch: loop {
        match pc {
            0x826430E0 => {
    //   block [0x826430E0..0x82643150)
	// 826430E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826430E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826430E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826430EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826430F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826430F4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826430F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826430FC: 390BC850  addi r8, r11, -0x37b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14256;
	// 82643100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82643104: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82643108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264310C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82643114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643118: 386AE824  addi r3, r10, -0x17dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6108;
	// 8264311C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82643120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264312C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264313C: 4BE23CE5  bl 0x82466e20
	ctx.lr = 0x82643140;
	sub_82466E20(ctx, base);
	// 82643140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643150 size=108
    let mut pc: u32 = 0x82643150;
    'dispatch: loop {
        match pc {
            0x82643150 => {
    //   block [0x82643150..0x826431BC)
	// 82643150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264315C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643164: 38EBC868  addi r7, r11, -0x3798
	ctx.r[7].s64 = ctx.r[11].s64 + -14232;
	// 82643168: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264316C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82643170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264317C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643180: 386AE854  addi r3, r10, -0x17ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6060;
	// 82643184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264318C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264319C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826431A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826431A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826431A8: 4BE23C79  bl 0x82466e20
	ctx.lr = 0x826431AC;
	sub_82466E20(ctx, base);
	// 826431AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826431B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826431B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826431B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826431C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826431C0 size=108
    let mut pc: u32 = 0x826431C0;
    'dispatch: loop {
        match pc {
            0x826431C0 => {
    //   block [0x826431C0..0x8264322C)
	// 826431C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826431C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826431C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826431CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826431D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826431D4: 38EBC8B0  addi r7, r11, -0x3750
	ctx.r[7].s64 = ctx.r[11].s64 + -14160;
	// 826431D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826431DC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826431E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826431E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826431E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826431EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826431F0: 386AE884  addi r3, r10, -0x177c
	ctx.r[3].s64 = ctx.r[10].s64 + -6012;
	// 826431F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826431F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826431FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264320C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643218: 4BE23C09  bl 0x82466e20
	ctx.lr = 0x8264321C;
	sub_82466E20(ctx, base);
	// 8264321C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643230 size=108
    let mut pc: u32 = 0x82643230;
    'dispatch: loop {
        match pc {
            0x82643230 => {
    //   block [0x82643230..0x8264329C)
	// 82643230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82643234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264323C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643244: 38EBC940  addi r7, r11, -0x36c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14016;
	// 82643248: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264324C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82643250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264325C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643260: 386AE8B4  addi r3, r10, -0x174c
	ctx.r[3].s64 = ctx.r[10].s64 + -5964;
	// 82643264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264326C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264327C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643288: 4BE23B99  bl 0x82466e20
	ctx.lr = 0x8264328C;
	sub_82466E20(ctx, base);
	// 8264328C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826432A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826432A0 size=100
    let mut pc: u32 = 0x826432A0;
    'dispatch: loop {
        match pc {
            0x826432A0 => {
    //   block [0x826432A0..0x82643304)
	// 826432A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826432A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826432A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826432AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826432B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826432B4: 38AAEA04  addi r5, r10, -0x15fc
	ctx.r[5].s64 = ctx.r[10].s64 + -5628;
	// 826432B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826432BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826432C0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826432C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826432C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826432CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826432D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826432D4: 386AE8E4  addi r3, r10, -0x171c
	ctx.r[3].s64 = ctx.r[10].s64 + -5916;
	// 826432D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826432DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826432E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826432E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826432E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826432EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826432F0: 4BE23B31  bl 0x82466e20
	ctx.lr = 0x826432F4;
	sub_82466E20(ctx, base);
	// 826432F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826432F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826432FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643308 size=112
    let mut pc: u32 = 0x82643308;
    'dispatch: loop {
        match pc {
            0x82643308 => {
    //   block [0x82643308..0x82643378)
	// 82643308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264330C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643314: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643318: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264331C: 38AAE8E4  addi r5, r10, -0x171c
	ctx.r[5].s64 = ctx.r[10].s64 + -5916;
	// 82643320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82643324: 390BC9D0  addi r8, r11, -0x3630
	ctx.r[8].s64 = ctx.r[11].s64 + -13872;
	// 82643328: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264332C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82643330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82643334: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264333C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643340: 386AE914  addi r3, r10, -0x16ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5868;
	// 82643344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82643348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264334C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82643354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264335C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82643364: 4BE23ABD  bl 0x82466e20
	ctx.lr = 0x82643368;
	sub_82466E20(ctx, base);
	// 82643368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264336C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82643370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82643378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82643378 size=108
    let mut pc: u32 = 0x82643378;
    'dispatch: loop {
        match pc {
            0x82643378 => {
    //   block [0x82643378..0x826433E4)
	// 82643378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82643380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82643384: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82643388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264338C: 38EBCA30  addi r7, r11, -0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + -13776;
	// 82643390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82643394: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82643398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264339C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826433A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826433A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826433A8: 386AE944  addi r3, r10, -0x16bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5820;
	// 826433AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826433B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826433B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826433B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826433BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826433C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826433C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826433C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826433CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826433D0: 4BE23A51  bl 0x82466e20
	ctx.lr = 0x826433D4;
	sub_82466E20(ctx, base);
	// 826433D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826433D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826433DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826433E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826433E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826433E8 size=108
    let mut pc: u32 = 0x826433E8;
    'dispatch: loop {
        match pc {
            0x826433E8 => {
    //   block [0x826433E8..0x82643454)
	// 826433E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826433EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826433F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826433F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826433F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826433FC: 38EBCA60  addi r7, r11, -0x35a0
	ctx.r[7].s64 = ctx.r[11].s64 + -13728;
	// 82643400: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82643404: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82643408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264340C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82643410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82643414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82643418: 386AE974  addi r3, r10, -0x168c
	ctx.r[3].s64 = ctx.r[10].s64 + -5772;
	// 8264341C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82643420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82643424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82643428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264342C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82643430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82643434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82643438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264343C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82643440: 4BE239E1  bl 0x82466e20
	ctx.lr = 0x82643444;
	sub_82466E20(ctx, base);
	// 82643444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82643448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82643450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


